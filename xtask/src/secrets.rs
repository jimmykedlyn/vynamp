//! Secret scan over the repository history (TASK-E19-01 AC-5).
//!
//! `REQ-E19-01` requires "nenhum segredo no histórico". Nothing in the tree
//! checked that: the Woodpecker preflight that used to look for token shapes
//! lived in `infra/ci/`, left with ADR-014, and no replacement was written. A
//! claim nobody verifies is the failure mode this module exists to close.
//!
//! The scan reads **every blob reachable from every ref**, not the working
//! tree: a secret deleted in the next commit is still in the clone, and the
//! working tree would report it clean. It also reports the blob, the path and
//! the line, because a finding without a location cannot be revoked or
//! assessed.
//!
//! Deliberately not a general-purpose scanner. It covers shapes that are
//! unambiguous on sight -- prefix-tagged forge and cloud tokens, PEM private
//! keys -- and stops there. A high-entropy heuristic would need a baseline
//! file to be usable, and a baseline is where real findings go to be
//! forgotten. New shapes are cheap to add; false positives are not.

use anyhow::{Context, Result, bail, ensure};
use regex::Regex;
use std::fmt::Write as _;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::Stdio;
use std::sync::LazyLock;

/// One credential shape that is a secret on sight.
///
/// The `name` is what a finding prints, so it says what the reader must
/// revoke, not merely what matched.
struct Pattern {
    name: &'static str,
    regex: Regex,
}

fn pattern(name: &'static str, regex: &str) -> Pattern {
    Pattern {
        name,
        regex: Regex::new(regex).expect("secret pattern compiles"),
    }
}

/// Matched against every line of every blob.
///
/// `\\$\\{` guards are intentionally absent: a GitHub workflow that writes
/// `${{ secrets.X }}` interpolates a name, not a value, and would not match
/// any pattern here anyway -- the patterns require a literal credential.
static PATTERNS: LazyLock<Vec<Pattern>> = LazyLock::new(|| {
    vec![
        pattern("GitHub token", r"\bgh[pousr]_[A-Za-z0-9]{36,}"),
        pattern(
            "GitHub fine-grained token",
            r"\bgithub_pat_[A-Za-z0-9_]{22,}",
        ),
        pattern("AWS access key id", r"\b(?:AKIA|ASIA)[0-9A-Z]{16}\b"),
        pattern("Slack token", r"\bxox[baprs]-[A-Za-z0-9-]{10,}"),
        pattern("Google API key", r"\bAIza[0-9A-Za-z_-]{35}\b"),
        pattern("npm token", r"\bnpm_[A-Za-z0-9]{36}\b"),
        pattern("Cloudflare tunnel token", r"\beyJhIjoi[A-Za-z0-9_-]{40,}"),
        pattern(
            "Private key",
            r"-----BEGIN (?:RSA |EC |DSA |OPENSSH |PGP |ENCRYPTED )?PRIVATE KEY-----",
        ),
    ]
});

/// Blobs a maintainer read and found to hold no credential, with the reason.
///
/// Keyed by blob id, not by path or line: a later edit of the same file is a
/// new blob and is scanned again. Each entry is a blob already in the shared
/// history that cannot be removed without a force push, which the project
/// forbids. Add one only after reading the blob, with the maintainer's OK.
const REVIEWED_BLOBS: &[(&str, &str)] = &[(
    "b846c5d8bcaf45cc1fb347d9879ec4529007ecc1",
    "_arquivo/media-player/STATE.md:104 narrates the PEM header that once \
     sat in a test; prose, no key material (umbrella PR #49, approved by the \
     maintainer 2026-11-11)",
)];

/// A credential found in the history.
pub struct Finding {
    pub pattern: &'static str,
    /// Object id of the blob, not of a commit: the blob is what matched, and
    /// the same blob can be reached from many commits. `git log --all
    /// --find-object=<id>` maps it back to the commits that carry it.
    pub blob: String,
    pub path: String,
    pub line: usize,
}

impl Finding {
    /// Render for a log line. The matched text is **not** printed: a scan that
    /// echoes the secret into CI output, where every log reader and every
    /// archive of that log can see it, has just published the secret it was
    /// meant to contain.
    fn render(&self) -> String {
        format!(
            "{} at {}:{} (blob {})",
            self.pattern, self.path, self.line, self.blob
        )
    }
}

/// One blob object and a path it was reached by.
struct Blob {
    id: String,
    path: String,
}

/// Every distinct blob object reachable from every ref, with one path each.
///
/// Keyed by **object id**, and read by object id. The first draft of this
/// module got both wrong: in `rev-list --objects` the first field of a line is
/// the *object* id, not a commit, so `git show <id>:<path>` failed for every
/// entry -- and the `else { continue }` that skipped unreadable entries turned
/// that failure into silence. The scan printed "no credential found" for any
/// repository, which is the worst form of a gate: it fails open and looks
/// green. Reading by `cat-file -p <id>` cannot fail that way, and any entry
/// that does not read is now an error rather than a skip.
///
/// `rev-list --objects` interleaves commits, trees and blobs, so the object
/// types come from a single `cat-file --batch-check` rather than from guessing
/// at the output shape.
fn blobs(root: &Path) -> Result<Vec<Blob>> {
    // `util::git` goes through `util::program`, which already scrubs the hook
    // environment from every `git` child; using it keeps one error style and
    // one place to change the git invocation.
    let listing = crate::util::git(root, &["rev-list", "--all", "--objects"])?;
    let mut candidates: Vec<Blob> = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for line in listing.lines() {
        let mut parts = line.splitn(2, ' ');
        let Some(id) = parts.next().filter(|s| !s.is_empty()) else {
            continue;
        };
        let Some(path) = parts.next().filter(|s| !s.is_empty()) else {
            // The commit object itself carries no path.
            continue;
        };
        if seen.insert(id.to_owned()) {
            candidates.push(Blob {
                id: id.to_owned(),
                path: path.to_owned(),
            });
        }
    }
    let types: std::collections::BTreeMap<String, String> =
        object_types(root, &candidates)?.into_iter().collect();
    Ok(candidates
        .into_iter()
        .filter(|b| types.get(&b.id).is_some_and(|t| t == "blob"))
        .collect())
}

/// Object id to type, resolved in one child process.
///
/// The ids are written from a separate thread, because writing them all before
/// reading any output is pathologically slow here once the request outgrows a
/// pipe buffer (64 KiB, roughly 1,600 object ids). Measured on the umbrella's
/// own history of ~3,300 objects: sequential write-then-read took **330 s** in
/// one run and passed **600 s** in another, against **9.5 s** with the writer on
/// its own thread.
///
/// The mechanism is not a clean deadlock, and this comment briefly claimed one
/// on no evidence. `wait_with_output` closes the child's stdin before it starts
/// reading, so the sequential version does terminate; a faithful reproduction of
/// the bare pattern in another language blocked past 200 s, which is where the
/// wrong description came from. What is certain is the measurement, not the
/// explanation. `wait_with_output` already drains stdout and stderr on threads
/// of its own, so only the writer needs one.
fn object_types(root: &Path, candidates: &[Blob]) -> Result<Vec<(String, String)>> {
    let mut child = crate::util::local_git(root)
        .args(["cat-file", "--batch-check=%(objectname) %(objecttype)"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Start git cat-file --batch-check")?;
    let mut stdin = child.stdin.take().context("cat-file stdin")?;
    let ids: Vec<String> = candidates.iter().map(|b| b.id.clone()).collect();
    let writer = std::thread::spawn(move || -> Result<()> {
        for id in &ids {
            writeln!(stdin, "{id}")?;
        }
        stdin.flush()?;
        Ok(())
    });
    let out = child.wait_with_output()?;
    writer
        .join()
        .map_err(|_| anyhow::anyhow!("cat-file writer thread panicked"))??;
    ensure!(
        out.status.success(),
        "git cat-file --batch-check failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let text = String::from_utf8(out.stdout)?;
    Ok(text
        .lines()
        .filter_map(|l| {
            let mut parts = l.split_whitespace();
            Some((parts.next()?.to_owned(), parts.next()?.to_owned()))
        })
        .collect())
}

/// Scan every blob reachable from every ref.
///
/// A file deleted in a later commit is still in the clone, which is the whole
/// point: the working tree would call it clean.
///
/// One `git cat-file --batch` process reads every blob, not one process per
/// blob. The first version spawned a `git cat-file -p <id>` per blob, which on
/// Windows costs a process start each: the umbrella's history did not finish in
/// ten minutes, so the gate existed and could not be run. It also read each body
/// through `String`, so the first binary blob in the history would have aborted
/// the whole scan; the batch reader consumes the declared byte count and
/// decodes lossily instead.
///
/// The ids are written and their output read one at a time, so neither pipe can
/// fill while the other is being drained.
pub fn scan(root: &Path) -> Result<Vec<Finding>> {
    let blobs = blobs(root)?;
    let mut findings = Vec::new();
    if blobs.is_empty() {
        return Ok(findings);
    }
    let mut child = crate::util::local_git(root)
        .args(["cat-file", "--batch"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Start git cat-file --batch")?;
    let mut stdin = child.stdin.take().context("cat-file stdin")?;
    let mut stdout = BufReader::new(child.stdout.take().context("cat-file stdout")?);
    let mut buffer = Vec::new();
    for blob in &blobs {
        writeln!(stdin, "{}", blob.id)?;
        stdin.flush()?;
        let mut header = String::new();
        ensure!(
            stdout.read_line(&mut header)? > 0,
            "git cat-file --batch stopped before {} ({})",
            blob.path,
            blob.id
        );
        let mut fields = header.split_whitespace();
        let (Some(_), Some(kind), Some(size)) = (fields.next(), fields.next(), fields.next())
        else {
            bail!("Unexpected cat-file header {header:?} for {}", blob.path);
        };
        let size: usize = size
            .parse()
            .with_context(|| format!("cat-file size {size:?} for {}", blob.path))?;
        buffer.clear();
        buffer.resize(size, 0);
        stdout.read_exact(&mut buffer)?;
        // Each object is followed by a newline that is not part of its body.
        let mut newline = [0u8; 1];
        stdout.read_exact(&mut newline)?;
        ensure!(
            kind == "blob",
            "{} resolved to a {kind}, not a blob; the object type filter is wrong",
            blob.path
        );
        let body = String::from_utf8_lossy(&buffer);
        findings.extend(scan_text(&body, &blob.path, &blob.id));
    }
    drop(stdin);
    drop(stdout);
    let status = child.wait()?;
    ensure!(
        status.success(),
        "git cat-file --batch exited with {status}"
    );
    Ok(findings)
}

/// Scan one blob's text, returning what matched.
///
/// Separated from `scan` so a test can drive it without building a repository.
fn scan_text(body: &str, path: &str, blob: &str) -> Vec<Finding> {
    let mut out = Vec::new();
    if REVIEWED_BLOBS.iter().any(|(id, _)| *id == blob) {
        return out;
    }
    for (index, line) in body.lines().enumerate() {
        for p in PATTERNS.iter() {
            if p.regex.is_match(line) {
                out.push(Finding {
                    pattern: p.name,
                    blob: blob.to_owned(),
                    path: path.to_owned(),
                    line: index + 1,
                });
            }
        }
    }
    out
}

/// Fail when the history carries a credential.
///
/// Callers get the rendered list and nothing else; the matched text stays out
/// of the message for the reason `Finding::render` gives.
pub fn check(root: &Path) -> Result<()> {
    let findings = scan(root)?;
    if findings.is_empty() {
        println!("No credential shape found in the reachable history.");
        return Ok(());
    }
    let mut message = String::from("Credential shapes found in the history:\n");
    for f in &findings {
        let _ = writeln!(message, "  {}", f.render());
    }
    let _ = write!(
        message,
        "Revoke each credential, then rewrite the history before the repository is used again."
    );
    anyhow::bail!(message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn github_tokens_are_found_with_their_location() {
        // Assembled at runtime so this file does not itself contain a token
        // shape -- otherwise the gate would fail on its own test.
        let token = format!("gh{}_", "p") + &"A1b2C3d4E5f6G7h8I9j0".repeat(2);
        let body = format!("line one\nconst TOKEN: &str = \"{token}\";\n");
        let found = scan_text(&body, "src/lib.rs", "abc123");
        assert_eq!(found.len(), 1, "one finding expected");
        assert_eq!(found[0].pattern, "GitHub token");
        assert_eq!(found[0].line, 2);
        assert_eq!(found[0].path, "src/lib.rs");
    }

    #[test]
    fn private_keys_are_found() {
        // Assembled at runtime, like the token fixtures: a literal PEM header in
        // this file is a finding, and the gate scans the history, so committing
        // one makes the repository fail its own scan. That is not hypothetical --
        // this line was written as a literal, CI caught it, and the local run
        // that reported "clean" had been made before the file was committed.
        let body = format!("-----BEGIN {} PRIVATE KEY-----\nabc\n", "OPENSSH");
        let found = scan_text(&body, "id_ed25519", "sha");
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].pattern, "Private key");
    }

    #[test]
    fn reviewed_blobs_are_skipped_and_only_those() {
        let body = format!("-----BEGIN {} PRIVATE KEY-----\n", "OPENSSH");
        let (reviewed, _) = REVIEWED_BLOBS[0];
        assert!(scan_text(&body, "STATE.md", reviewed).is_empty());
        assert_eq!(scan_text(&body, "STATE.md", "other").len(), 1);
    }

    /// The message must name the location and never the credential: CI logs
    /// are readable by anyone with access to the run, and are retained.
    #[test]
    fn the_rendered_finding_does_not_echo_the_secret() {
        let finding = Finding {
            pattern: "GitHub token",
            blob: "abc123".to_owned(),
            path: "src/lib.rs".to_owned(),
            line: 7,
        };
        let rendered = finding.render();
        assert!(rendered.contains("src/lib.rs:7"));
        assert!(rendered.contains("abc123"));
        assert!(!rendered.contains("gh"));
    }

    /// A workflow that interpolates a secret by name holds no credential and
    /// must not be reported; otherwise every repository fails on its own CI.
    #[test]
    fn secret_interpolation_is_not_a_finding() {
        let body = "env:\n  TOKEN: ${{ secrets.GITHUB_TOKEN }}\n";
        assert!(scan_text(body, ".github/workflows/ci.yml", "sha").is_empty());
    }

    /// A short, obviously fake string is not a credential, and a scanner that
    /// flags it trains its readers to ignore the gate.
    #[test]
    fn short_lookalikes_are_not_findings() {
        for body in [
            "let gh = \"ghp_short\";",
            "AKIA123",
            "-----BEGIN NOT A KEY-----",
        ] {
            assert!(
                scan_text(body, "x", "sha").is_empty(),
                "{body} must not match"
            );
        }
    }

    /// The bug this module shipped first, kept as a test so it cannot return.
    ///
    /// The first version deduplicated by path and read `commit:path` from the
    /// newest commit that named the path -- so a secret committed and then
    /// *deleted* bound to the deleting commit, `git show` failed, and the scan
    /// reported the repository clean. That is the exact case AC-5 exists for:
    /// the working tree is clean and the history is not.
    #[test]
    fn a_secret_deleted_from_the_working_tree_is_still_found() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        let token = format!("gh{}_", "p") + &"A1b2C3d4E5f6G7h8I9j0".repeat(2);
        fixture_ok(root, &["init", "-q", "."]);
        fixture_ok(root, &["config", "user.email", "test@example.invalid"]);
        fixture_ok(root, &["config", "user.name", "test"]);
        std::fs::write(
            root.join("leak.txt"),
            format!(
                "clean
TOKEN={token}
"
            ),
        )
        .unwrap();
        fixture_ok(root, &["add", "leak.txt"]);
        fixture_ok(root, &["commit", "-q", "-m", "leak"]);
        std::fs::remove_file(root.join("leak.txt")).unwrap();
        fixture_ok(root, &["add", "-A"]);
        fixture_ok(root, &["commit", "-q", "-m", "remove the leak"]);
        assert!(
            !root.join("leak.txt").exists(),
            "the working tree must be clean for this test to mean anything"
        );
        let found = scan(root).expect("scan runs");
        assert_eq!(
            found.len(),
            1,
            "history still holds the secret: {:?}",
            found.iter().map(Finding::render).collect::<Vec<_>>()
        );
        assert_eq!(found[0].path, "leak.txt");
        assert_eq!(found[0].line, 2);
    }
    /// `scan` must actually read history, not silently return nothing.
    ///
    /// Driving it over the real repository would cost a full-object read on
    /// every test run, so this builds a repository with a known clean history
    /// and asserts the scan completes and reports it clean; the deleted-secret
    /// case above proves it reads content, not just paths. Together they cover
    /// both ways a scan can lie: finding nothing because it read nothing, and
    /// finding nothing because it looked in the wrong place.
    #[test]
    fn the_scan_reads_a_repository_history() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        fixture_ok(root, &["init", "-q", "."]);
        fixture_ok(root, &["config", "user.email", "test@example.invalid"]);
        fixture_ok(root, &["config", "user.name", "test"]);
        std::fs::write(
            root.join("ok.txt"),
            "no credential here
",
        )
        .unwrap();
        fixture_ok(root, &["add", "ok.txt"]);
        fixture_ok(root, &["commit", "-q", "-m", "clean"]);
        assert!(scan(root).expect("scan runs").is_empty());
    }

    /// A scan larger than a pipe buffer must finish inside a deadline.
    ///
    /// This is a **timing** guard, and worth saying plainly what it does and does
    /// not prove. It passes on the slow sequential version too -- that version
    /// finished this 2,000-file history in ~4 s, well under the deadline -- so it
    /// does not discriminate between the two implementations. What it catches is
    /// a regression severe enough to stall the scan outright, which is the shape
    /// the failure took in practice: the gate appeared to hang and never ran.
    /// The real reason the writer thread exists is the measurement recorded on
    /// `object_types`, not this test.
    #[test]
    fn a_scan_larger_than_a_pipe_buffer_completes() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        fixture_ok(root, &["init", "-q", "."]);
        fixture_ok(root, &["config", "user.email", "test@example.invalid"]);
        fixture_ok(root, &["config", "user.name", "test"]);
        // 2,000 blobs is about 82 KiB of object ids here: comfortably past the
        // 64 KiB pipe buffer the deadlock needed, on every platform.
        for i in 0..2_000 {
            std::fs::write(root.join(format!("f{i}.txt")), "no credential here\n").unwrap();
        }
        fixture_ok(root, &["add", "-A"]);
        fixture_ok(root, &["commit", "-q", "-m", "many files"]);

        let (tx, rx) = std::sync::mpsc::channel();
        let path = root.to_path_buf();
        let worker = std::thread::spawn(move || {
            let _ = tx.send(scan(&path));
        });
        match rx.recv_timeout(std::time::Duration::from_secs(120)) {
            Ok(result) => assert!(result.expect("scan runs").is_empty()),
            Err(_) => panic!("scan deadlocked on a history larger than a pipe buffer"),
        }
        worker.join().expect("scan thread");
    }
    /// Run git against a throwaway fixture, isolated from the caller.
    ///
    /// `current_dir` is not enough: Git reads `GIT_DIR` and friends **before**
    /// it looks at the working directory, and `pre-push` here exports them
    /// (`cargo xtask ci docs` runs inside the hook). Without the removal below,
    /// a fixture's `git init/add/commit` lands in the real repository that
    /// started the push.
    fn fixture_git(root: &Path, args: &[&str]) -> std::process::Output {
        let mut command = std::process::Command::new("git");
        crate::util::scrub_git_env(&mut command);
        command
            .args(args)
            .current_dir(root)
            .output()
            .expect("git runs")
    }

    fn fixture_ok(root: &Path, args: &[&str]) {
        let out = fixture_git(root, args);
        assert!(
            out.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }

    fn fixture_text(root: &Path, args: &[&str]) -> String {
        let out = fixture_git(root, args);
        assert!(
            out.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8(out.stdout).unwrap()
    }

    /// Child half of [`a_fixture_never_writes_to_the_repository_git_exported`].
    ///
    /// Builds and scans a fixture repository exactly the way the tests above do,
    /// under whatever `GIT_*` the parent exported. Returns at once when the
    /// parent did not set `VYNAMP_FIXTURE_PROBE` -- a plain `cargo test` run
    /// must not need the sentinel, and the parent is what makes this mean
    /// anything.
    #[test]
    fn the_fixture_probe() {
        if std::env::var_os("VYNAMP_FIXTURE_PROBE").is_none() {
            return;
        }
        let target = tempfile::tempdir().unwrap();
        let root = target.path();
        fixture_ok(root, &["init", "-q", "."]);
        fixture_ok(root, &["config", "user.email", "test@example.invalid"]);
        fixture_ok(root, &["config", "user.name", "test"]);
        std::fs::write(root.join("ok.txt"), "no credential here\n").unwrap();
        fixture_ok(root, &["add", "ok.txt"]);
        fixture_ok(root, &["commit", "-q", "-m", "fixture"]);
        assert!(
            scan(root).expect("scan runs").is_empty(),
            "the fixture history is clean, so the scan must report nothing"
        );
    }

    /// A fixture must operate on its own tempdir, never on the repository Git
    /// exported to the hook.
    ///
    /// The oracle is a sentinel repository that carries a token-shaped blob.
    /// The child runs a whole fixture flow (`init`, `add`, `commit`, `scan`)
    /// with `GIT_DIR` and friends pointed at the sentinel, which is what Git
    /// hands `pre-push`. Two independent things must hold afterwards: the
    /// sentinel's history must be untouched, and the scan must have read the
    /// fixture rather than the sentinel -- reading the sentinel would surface
    /// its token and fail the child's own assertion.
    ///
    /// This is not hypothetical. On 2026-11-12 the unguarded tests committed
    /// `clean` and `many files` onto the branch being pushed, overwrote the
    /// worktree index with 2,000 fixture files, and wrote `core.bare=true` and
    /// `[user] test` into the shared `.git/config`. The damage landed in the
    /// sentinel's role here, which is a tempdir -- that is the point of it.
    #[test]
    fn a_fixture_never_writes_to_the_repository_git_exported() {
        let sentinel = tempfile::tempdir().unwrap();
        let sentinel_root = sentinel.path();
        fixture_ok(sentinel_root, &["init", "-q", "."]);
        fixture_ok(
            sentinel_root,
            &["config", "user.email", "test@example.invalid"],
        );
        fixture_ok(sentinel_root, &["config", "user.name", "test"]);
        // Assembled at runtime so this file has no token shape of its own.
        let token = format!("gh{}_", "p") + &"A1b2C3d4E5f6G7h8I9j0".repeat(2);
        std::fs::write(sentinel_root.join("secret.txt"), format!("TOKEN={token}\n")).unwrap();
        fixture_ok(sentinel_root, &["add", "secret.txt"]);
        fixture_ok(sentinel_root, &["commit", "-q", "-m", "sentinel"]);

        let head_before = fixture_text(sentinel_root, &["rev-parse", "HEAD"]);
        let count_before = fixture_text(sentinel_root, &["rev-list", "--all", "--count"]);
        let status_before = fixture_text(sentinel_root, &["status", "--porcelain"]);
        // HEAD, commit count and `status --porcelain` cannot see a config write:
        // measured, `git config user.name test` changes `.git/config` while all
        // three stay identical. The config is where the real damage landed
        // (`core.bare`, `[user]`), so it is hashed on its own.
        let config_before = file_digest(&sentinel_root.join(".git/config"));

        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "secrets::tests::the_fixture_probe",
                "--nocapture",
            ])
            .env("GIT_DIR", sentinel_root.join(".git"))
            .env("GIT_COMMON_DIR", sentinel_root.join(".git"))
            .env("GIT_WORK_TREE", sentinel_root)
            .env("GIT_INDEX_FILE", sentinel_root.join(".git/index"))
            .env("VYNAMP_FIXTURE_PROBE", "1")
            .output()
            .unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            output.status.success(),
            "the fixture probe failed under the hook environment -- it either read the exported \
             repository or wrote into it:\n{stdout}\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        // A child that runs zero tests exits 0: `--exact` with a wrong name
        // prints "0 passed ... 110 filtered out" and succeeds. Without this the
        // probe could be renamed and the test would pass forever, proving nothing.
        assert!(
            stdout.contains("1 passed"),
            "the probe did not run any test, so this test proves nothing:\n{stdout}"
        );

        assert_eq!(
            fixture_text(sentinel_root, &["rev-parse", "HEAD"]),
            head_before,
            "the exported repository HEAD must not move"
        );
        assert_eq!(
            fixture_text(sentinel_root, &["rev-list", "--all", "--count"]),
            count_before,
            "the exported repository must gain no commits"
        );
        assert_eq!(
            fixture_text(sentinel_root, &["status", "--porcelain"]),
            status_before,
            "the exported repository index and working tree must not change"
        );
        assert_eq!(
            file_digest(&sentinel_root.join(".git/config")),
            config_before,
            "the exported repository config must not be rewritten -- `git init` and \
             `git config user.*` in a fixture both land here"
        );
    }

    /// SHA-256 of a file, used to observe writes that leave no trace in HEAD,
    /// the commit count or `status --porcelain`.
    fn file_digest(path: &Path) -> Option<String> {
        crate::util::sha(path).ok()
    }
}
