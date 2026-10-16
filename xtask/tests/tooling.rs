use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};
use tempfile::TempDir;
struct Fixture {
    _temp: TempDir,
    root: PathBuf,
}
impl Fixture {
    fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("project with spaces");
        fs::create_dir(&root).unwrap();
        fs::write(root.join(".gitignore"), ".worktrees/\n.superpowers/\n").unwrap();
        fs::write(root.join("README.md"), "# Fixture\n").unwrap();
        fs::create_dir(root.join("tasks")).unwrap();
        fs::create_dir(root.join("config")).unwrap();
        fs::write(
            root.join("tasks/backlog.json"),
            json!({"tasks":[{"id":"TASK-E01-03"},{"id":"TASK-E02-03"}]}).to_string(),
        )
        .unwrap();
        fs::write(root.join("config/project.json"), json!({"application_ready":false,"release_enabled":false,"desktop_package":"vynamp-desktop","desktop_binary":"vynamp","product_version":"0.1.0"}).to_string()).unwrap();
        Self { _temp: temp, root }
    }
    fn command(&self, exe: &str, args: &[&str], cwd: &Path) -> Command {
        let mut command = Command::new(exe);
        // Fixtures operate on foreign repositories. Git exports these local
        // variables to hooks; changing cwd alone does not isolate their children.
        // Keep this list aligned with `git rev-parse --local-env-vars`.
        for name in [
            "GIT_ALTERNATE_OBJECT_DIRECTORIES",
            "GIT_CONFIG",
            "GIT_CONFIG_PARAMETERS",
            "GIT_CONFIG_COUNT",
            "GIT_OBJECT_DIRECTORY",
            "GIT_DIR",
            "GIT_WORK_TREE",
            "GIT_IMPLICIT_WORK_TREE",
            "GIT_GRAFT_FILE",
            "GIT_INDEX_FILE",
            "GIT_NO_REPLACE_OBJECTS",
            "GIT_REPLACE_REF_BASE",
            "GIT_PREFIX",
            "GIT_SHALLOW_FILE",
            "GIT_COMMON_DIR",
        ] {
            command.env_remove(name);
        }
        command
            .args(args)
            .current_dir(cwd)
            .env("GIT_AUTHOR_NAME", "Tooling Test")
            .env("GIT_AUTHOR_EMAIL", "test@example.invalid")
            .env("GIT_COMMITTER_NAME", "Tooling Test")
            .env("GIT_COMMITTER_EMAIL", "test@example.invalid")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", self.root.join("no-global-config"));
        command
    }
    fn cmd(&self, exe: &str, args: &[&str], cwd: &Path) -> Output {
        self.command(exe, args, cwd).output().unwrap()
    }
    fn run(&self, args: &[&str]) -> Output {
        let mut all = vec!["--root", self.root.to_str().unwrap()];
        all.extend(args);
        self.cmd(env!("CARGO_BIN_EXE_xtask"), &all, &self.root)
    }
    fn git(&self, args: &[&str]) -> Output {
        self.cmd("git", args, &self.root)
    }
    fn init(&self) {
        ok(self.run(&["repo", "init"]));
    }
    fn work(&self, task: &str, agent: &str, slug: &str, kind: &str) -> Output {
        self.run(&[
            "repo", "worktree", "--task", task, "--agent", agent, "--slug", slug, "--kind", kind,
        ])
    }
    fn path(&self, agent: &str, task: &str, slug: &str) -> PathBuf {
        self.root
            .join(format!(".worktrees/{agent}/{}-{slug}", task.to_lowercase()))
    }
}
fn ok(o: Output) {
    assert!(
        o.status.success(),
        "stdout={} stderr={}",
        String::from_utf8_lossy(&o.stdout),
        String::from_utf8_lossy(&o.stderr)
    );
}
#[test]
fn fixtures_are_isolated_from_hook_repository_environment() {
    let f = Fixture::new();
    f.init();
    let before = fs::read(f.root.join(".git/index")).unwrap();
    // Run an existing fixture test as a child, with the caller's repository
    // context just as Git exports it to hooks. Do not mutate this test process.
    let test_names = [
        "init_creates_main_develop_without_remote",
        #[cfg(unix)]
        "git_hooks_do_not_reuse_shared_outer_driver_cache",
    ];
    for test_name in test_names {
        ok(Command::new(std::env::current_exe().unwrap())
            .args(["--exact", test_name, "--nocapture"])
            .env("GIT_DIR", f.root.join(".git"))
            .env("GIT_COMMON_DIR", f.root.join(".git"))
            .env("GIT_WORK_TREE", &f.root)
            .env("GIT_INDEX_FILE", f.root.join(".git/index"))
            .output()
            .unwrap());
    }
    assert_eq!(fs::read(f.root.join(".git/index")).unwrap(), before);
    assert!(f.git(&["status", "--porcelain"]).stdout.is_empty());
}
#[test]
fn init_creates_main_develop_without_remote() {
    let f = Fixture::new();
    f.init();
    assert_eq!(String::from_utf8_lossy(&f.git(&["remote"]).stdout), "");
    let branches =
        String::from_utf8(f.git(&["branch", "--format=%(refname:short)"]).stdout).unwrap();
    assert!(branches.contains("main") && branches.contains("develop"));
}
#[test]
fn agents_have_independent_files_and_indexes() {
    let f = Fixture::new();
    f.init();
    ok(f.work("TASK-E01-03", "codex", "state", "feature"));
    ok(f.work("TASK-E02-03", "claude", "ui", "feature"));
    let a = f.path("codex", "TASK-E01-03", "state");
    let b = f.path("claude", "TASK-E02-03", "ui");
    fs::write(a.join("isolated.txt"), "private").unwrap();
    ok(f.cmd("git", &["add", "isolated.txt"], &a));
    assert!(!b.join("isolated.txt").exists());
    assert!(
        !String::from_utf8_lossy(&f.git(&["status", "--porcelain"]).stdout).contains("isolated")
    );
    assert!(
        !String::from_utf8_lossy(&f.cmd("git", &["status", "--porcelain"], &b).stdout)
            .contains("isolated")
    );
}
#[test]
fn task_has_only_one_owner() {
    let f = Fixture::new();
    f.init();
    ok(f.work("TASK-E01-03", "codex", "one", "feature"));
    assert!(
        !f.work("TASK-E01-03", "claude", "two", "feature")
            .status
            .success()
    );
}
#[test]
fn traversal_and_unknown_task_create_no_branch() {
    let f = Fixture::new();
    f.init();
    assert!(
        !f.work("TASK-E01-03", "codex", "../../escape", "feature")
            .status
            .success()
    );
    assert!(
        !f.work("TASK-E99-99", "codex", "unknown", "feature")
            .status
            .success()
    );
    assert_eq!(
        String::from_utf8_lossy(&f.git(&["branch", "--format=%(refname)"]).stdout)
            .lines()
            .count(),
        2
    );
}
#[test]
fn dirty_worktree_is_preserved() {
    let f = Fixture::new();
    f.init();
    ok(f.work("TASK-E01-03", "codex", "state", "feature"));
    let p = f.path("codex", "TASK-E01-03", "state");
    fs::write(p.join("precious"), "keep").unwrap();
    assert!(
        !f.run(&[
            "repo",
            "remove-worktree",
            "--task",
            "TASK-E01-03",
            "--agent",
            "codex"
        ])
        .status
        .success()
    );
    assert!(p.join("precious").exists());
}
#[test]
fn unmerged_worktree_is_preserved() {
    let f = Fixture::new();
    f.init();
    ok(f.work("TASK-E01-03", "codex", "state", "feature"));
    let p = f.path("codex", "TASK-E01-03", "state");
    fs::write(p.join("new"), "keep").unwrap();
    ok(f.cmd("git", &["add", "new"], &p));
    ok(f.cmd("git", &["commit", "-m", "feat: test"], &p));
    assert!(
        !f.run(&[
            "repo",
            "remove-worktree",
            "--task",
            "TASK-E01-03",
            "--agent",
            "codex"
        ])
        .status
        .success()
    );
    assert!(p.exists());
}
#[test]
fn hotfix_requires_both_main_and_develop() {
    let f = Fixture::new();
    f.init();
    ok(f.work("TASK-E01-03", "codex", "hot", "hotfix"));
    let p = f.path("codex", "TASK-E01-03", "hot");
    fs::write(p.join("hot"), "fix").unwrap();
    ok(f.cmd("git", &["add", "hot"], &p));
    ok(f.cmd("git", &["commit", "-m", "fix: urgent"], &p));
    ok(f.git(&["switch", "develop"]));
    ok(f.git(&[
        "merge",
        "--no-ff",
        "hotfix/task-e01-03-codex-hot",
        "-m",
        "merge test",
    ]));
    assert!(
        !f.run(&[
            "repo",
            "remove-worktree",
            "--task",
            "TASK-E01-03",
            "--agent",
            "codex"
        ])
        .status
        .success()
    );
    ok(f.git(&["switch", "main"]));
    ok(f.git(&[
        "merge",
        "--no-ff",
        "hotfix/task-e01-03-codex-hot",
        "-m",
        "merge stable",
    ]));
    ok(f.run(&[
        "repo",
        "remove-worktree",
        "--task",
        "TASK-E01-03",
        "--agent",
        "codex",
    ]));
    assert!(!p.exists());
}
#[test]
fn no_nested_worktrees() {
    let f = Fixture::new();
    f.init();
    ok(f.work("TASK-E01-03", "codex", "state", "feature"));
    let p = f.path("codex", "TASK-E01-03", "state");
    let o = f.cmd(
        env!("CARGO_BIN_EXE_xtask"),
        &[
            "--root",
            p.to_str().unwrap(),
            "repo",
            "worktree",
            "--task",
            "TASK-E02-03",
            "--agent",
            "claude",
            "--slug",
            "nested",
        ],
        &p,
    );
    assert!(!o.status.success());
}
#[test]
fn release_cannot_publish_unimplemented_player() {
    let f = Fixture::new();
    let o = f.run(&["release", "preflight", "--version", "0.1.0"]);
    assert!(!o.status.success());
    assert!(String::from_utf8_lossy(&o.stderr).contains("disabled"));
}
#[test]
fn xtask_workspace_does_not_activate_player_ci() {
    let f = Fixture::new();
    fs::write(
        f.root.join("Cargo.toml"),
        "[workspace]\nmembers=['xtask']\n",
    )
    .unwrap();
    let o = f.run(&["ci", "detect"]);
    assert!(o.status.success());
    let v: Value = serde_json::from_slice(&o.stdout).unwrap();
    assert_eq!(v["core"], "false");
    assert_eq!(v["android"], "false");
}
#[test]
fn commit_on_main_is_blocked_after_bootstrap() {
    let f = Fixture::new();
    f.init();
    assert!(!f.run(&["guard-branch"]).status.success());
}
#[test]
fn desktop_detection_parses_a_real_manifest() {
    let f = Fixture::new();
    fs::create_dir_all(f.root.join("apps/desktop/src")).unwrap();
    fs::write(
        f.root.join("apps/desktop/Cargo.toml"),
        "[package]\nname='vynamp-desktop'\nversion='0.1.0'\nedition='2024'\n[[bin]]\nname='vynamp'\npath='src/main.rs'\n",
    )
    .unwrap();
    fs::write(f.root.join("apps/desktop/src/main.rs"), "fn main() {}\n").unwrap();
    fs::write(
        f.root.join("Cargo.toml"),
        "[workspace]\nmembers=['apps/desktop']\nresolver='3'\n",
    )
    .unwrap();
    ok(f.cmd("cargo", &["generate-lockfile", "--offline"], &f.root));
    let o = f.run(&["ci", "detect"]);
    ok(Output {
        status: o.status,
        stdout: o.stdout.clone(),
        stderr: o.stderr.clone(),
    });
    let v: Value = serde_json::from_slice(&o.stdout).unwrap();
    assert_eq!(v["core"], "true");
    fs::write(
        f.root.join("apps/desktop/Cargo.toml"),
        "[package]\nname='xtask'\n",
    )
    .unwrap();
    let o = f.run(&["ci", "detect"]);
    let v: Value = serde_json::from_slice(&o.stdout).unwrap();
    assert_eq!(v["core"], "false");
}
#[test]
fn windows_autocrlf_keeps_release_sources_identical() {
    let f = Fixture::new();
    let package = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    fs::write(
        f.root.join(".gitattributes"),
        fs::read(package.join(".gitattributes")).unwrap(),
    )
    .unwrap();
    fs::create_dir_all(f.root.join("xtask/src")).unwrap();
    fs::write(
        f.root.join("xtask/src/main.rs"),
        "fn main() {\n    println!(\"fixture\");\n}\n",
    )
    .unwrap();
    fs::write(
        f.root.join("Cargo.toml"),
        "[workspace]\nmembers=['xtask']\n",
    )
    .unwrap();
    f.init();
    let clone = f._temp.path().join("windows clone");
    ok(f.cmd(
        "git",
        &[
            "clone",
            "-c",
            "core.autocrlf=true",
            f.root.to_str().unwrap(),
            clone.to_str().unwrap(),
        ],
        f._temp.path(),
    ));
    for rel in ["xtask/src/main.rs", "Cargo.toml", "config/project.json"] {
        assert_eq!(
            fs::read(f.root.join(rel)).unwrap(),
            fs::read(clone.join(rel)).unwrap(),
            "Checkout changed {rel}"
        );
    }
}

#[test]
fn excluded_desktop_package_does_not_activate_core_ci() {
    let f = Fixture::new();
    fs::create_dir_all(f.root.join("apps/desktop/src")).unwrap();
    fs::create_dir_all(f.root.join("xtask/src")).unwrap();
    fs::write(
        f.root.join("apps/desktop/Cargo.toml"),
        "[package]\nname='vynamp-desktop'\nversion='0.1.0'\nedition='2024'\n",
    )
    .unwrap();
    fs::write(f.root.join("apps/desktop/src/main.rs"), "fn main() {}\n").unwrap();
    fs::write(
        f.root.join("xtask/Cargo.toml"),
        "[package]\nname='xtask'\nversion='0.1.0'\nedition='2024'\n",
    )
    .unwrap();
    fs::write(f.root.join("xtask/src/main.rs"), "fn main() {}\n").unwrap();
    fs::write(
        f.root.join("Cargo.toml"),
        "[workspace]\nmembers=['xtask']\nexclude=['apps/desktop']\nresolver='3'\n",
    )
    .unwrap();
    ok(f.cmd("cargo", &["generate-lockfile", "--offline"], &f.root));
    let o = f.run(&["ci", "detect"]);
    ok(Output {
        status: o.status,
        stdout: o.stdout.clone(),
        stderr: o.stderr.clone(),
    });
    let v: Value = serde_json::from_slice(&o.stdout).unwrap();
    assert_eq!(v["core"], "false");
}
#[test]
fn vynamp_zip_uses_new_root_and_contains_approved_branding() {
    let f = Fixture::new();
    fs::create_dir_all(f.root.join("assets/branding")).unwrap();
    fs::write(
        f.root.join("assets/branding/approved-c-board.png"),
        b"fixture, not a real image",
    )
    .unwrap();
    fs::create_dir_all(f.root.join("bootstrap")).unwrap();
    for name in [
        "INICIAR.ps1",
        "AMBIENTE.ps1",
        "ABRIR-CODEX.ps1",
        "ABRIR-CLAUDE.ps1",
        "PROMPT-CODEX.txt",
        "PROMPT-CLAUDE.txt",
        "LEIA-ME.txt",
    ] {
        fs::copy(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../bootstrap")
                .join(name),
            f.root.join("bootstrap").join(name),
        )
        .unwrap();
    }
    let out = f._temp.path().join("vynamp.zip");
    ok(f.run(&["bundle-spec", "--output", out.to_str().unwrap()]));
    let mut archive = zip::ZipArchive::new(fs::File::open(&out).unwrap()).unwrap();
    assert!(
        archive
            .by_name("vynamp-media-player/pacote/assets/branding/approved-c-board.png")
            .is_ok()
    );
    assert!(archive.by_name("vynamp-media-player/INICIAR.ps1").is_ok());
    assert!(
        archive
            .by_name("vynamp-media-player/ABRIR-CLAUDE.ps1")
            .is_ok()
    );
    assert!(
        archive
            .by_name("auralis-mediaplayer/config/project.json")
            .is_err()
    );
}
#[test]
fn renamed_product_honors_legacy_task_claims() {
    let f = Fixture::new();
    f.init();
    let claim = f.root.join(".git/auralis/claims/TASK-E01-03");
    fs::create_dir_all(&claim).unwrap();
    fs::write(
        claim.join("owner.json"),
        json!({"agent":"claude","task":"TASK-E01-03"}).to_string(),
    )
    .unwrap();
    let result = f.work("TASK-E01-03", "codex", "new", "feature");
    assert!(!result.status.success());
    assert!(claim.join("owner.json").is_file());
}
#[test]
fn release_blocks_pending_native_branding_exports() {
    let f = Fixture::new();
    fs::write(f.root.join("config/project.json"),json!({"application_ready":true,"release_enabled":true,"branding_exports_ready":false,"product_version":"0.1.0"}).to_string()).unwrap();
    let result = f.run(&["release", "preflight", "--version", "0.1.0"]);
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("branding"));
}

fn bootstrap_fixture() -> Fixture {
    let f = Fixture::new();
    fs::write(
        f.root.join("tasks/backlog.json"),
        json!({"tasks":[{"id":"TASK-E15-01"}]}).to_string(),
    )
    .unwrap();
    f.init();
    f
}
#[test]
fn prepare_agents_isolates_bootstrap_and_reviewer_and_can_resume() {
    let f = bootstrap_fixture();
    ok(f.run(&["repo", "prepare-agents"]));
    let code = f.root.join(".worktrees/codex/task-e15-01-bootstrap");
    let review = f.root.join(".worktrees/claude/review-bootstrap");
    assert_eq!(
        String::from_utf8_lossy(&f.cmd("git", &["branch", "--show-current"], &code).stdout).trim(),
        "feature/task-e15-01-codex-bootstrap"
    );
    assert!(
        String::from_utf8_lossy(&f.cmd("git", &["branch", "--show-current"], &review).stdout)
            .trim()
            .is_empty()
    );
    fs::write(code.join("pending.txt"), "preserve").unwrap();
    ok(f.run(&["repo", "prepare-agents"]));
    assert_eq!(
        fs::read_to_string(code.join("pending.txt")).unwrap(),
        "preserve"
    );
    assert!(
        f.root
            .join(".git/auralis/claims/TASK-E15-01/owner.json")
            .is_file()
    );
}
#[test]
fn prepare_agents_reuses_import_branch_when_develop_is_absent() {
    let f = bootstrap_fixture();
    ok(f.git(&["branch", "-D", "develop"]));
    ok(f.git(&["switch", "-c", "chore/task-e15-01-codex-project-bootstrap"]));
    ok(f.run(&["repo", "prepare-agents"]));
    let code = f.root.join(".worktrees/codex/task-e15-01-bootstrap");
    assert_eq!(
        String::from_utf8_lossy(&f.cmd("git", &["branch", "--show-current"], &code).stdout).trim(),
        "chore/task-e15-01-codex-project-bootstrap"
    );
    assert!(
        String::from_utf8_lossy(&f.git(&["branch", "--show-current"]).stdout)
            .trim()
            .is_empty()
    );
}
#[test]
fn prepare_agents_preserves_dirty_import_checkout() {
    let f = bootstrap_fixture();
    ok(f.git(&["switch", "-c", "chore/task-e15-01-codex-project-bootstrap"]));
    fs::write(f.root.join("README.md"), "pending changes").unwrap();
    assert!(!f.run(&["repo", "prepare-agents"]).status.success());
    assert_eq!(
        fs::read_to_string(f.root.join("README.md")).unwrap(),
        "pending changes"
    );
    assert!(
        !f.root
            .join(".worktrees/codex/task-e15-01-bootstrap")
            .exists()
    );
}

#[test]
fn prepare_agents_rejects_plain_reviewer_folder() {
    let f = bootstrap_fixture();
    ok(f.git(&["switch", "-c", "chore/task-e15-01-codex-project-bootstrap"]));
    let review = f.root.join(".worktrees/claude/review-bootstrap");
    fs::create_dir_all(&review).unwrap();
    assert!(!f.run(&["repo", "prepare-agents"]).status.success());
    assert!(!review.join(".git").exists());
    assert!(review.is_dir());
}

#[test]
fn prepare_agents_adopts_numbered_import_branch() {
    let f = bootstrap_fixture();
    ok(f.git(&["branch", "-D", "develop"]));
    ok(f.git(&[
        "switch",
        "-c",
        "chore/task-e15-01-codex-project-bootstrap-2",
    ]));
    ok(f.run(&["repo", "prepare-agents"]));
    let code = f.root.join(".worktrees/codex/task-e15-01-bootstrap");
    assert_eq!(
        String::from_utf8_lossy(&f.cmd("git", &["branch", "--show-current"], &code).stdout).trim(),
        "chore/task-e15-01-codex-project-bootstrap-2"
    );
}

#[cfg(unix)]
#[test]
fn git_hooks_do_not_reuse_shared_outer_driver_cache() {
    use std::os::unix::fs::PermissionsExt;
    let f = Fixture::new();
    f.init();
    let hooks = f.root.join(".githooks");
    fs::create_dir_all(&hooks).unwrap();
    for name in ["pre-commit", "pre-push"] {
        let to = hooks.join(name);
        fs::copy(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../.githooks")
                .join(name),
            &to,
        )
        .unwrap();
        fs::set_permissions(to, fs::Permissions::from_mode(0o755)).unwrap();
    }
    let bin = f.root.join("fake-bin");
    fs::create_dir(&bin).unwrap();
    let cargo = bin.join("cargo");
    fs::write(
        &cargo,
        "#!/bin/sh\nprintf '%s\\n' \"$CARGO_TARGET_DIR\" >> \"$VYNAMP_HOOK_LOG\"\nexit 0\n",
    )
    .unwrap();
    fs::set_permissions(&cargo, fs::Permissions::from_mode(0o755)).unwrap();
    ok(f.git(&["switch", "-c", "fix/task-e01-03-codex-hooktest"]));
    fs::write(f.root.join("README.md"), "# changed\n").unwrap();
    ok(f.git(&["add", "README.md"]));
    let bare = f._temp.path().join("remote.git");
    ok(f.cmd(
        "git",
        &["init", "--bare", bare.to_str().unwrap()],
        f._temp.path(),
    ));
    let mut paths = vec![bin];
    paths.extend(std::env::split_paths(&std::env::var_os("PATH").unwrap()));
    let path = std::env::join_paths(paths).unwrap();
    let log = f.root.join("hook-target.log");
    for args in [
        vec!["commit", "-m", "fix: hook fixture"],
        vec!["push", bare.to_str().unwrap(), "HEAD:main"],
    ] {
        let output = f
            .command("git", &args, &f.root)
            .env("PATH", &path)
            .env("CARGO_TARGET_DIR", f.root.join("shared-driver"))
            .env("VYNAMP_HOOK_LOG", &log)
            .output()
            .unwrap();
        ok(output);
    }
    let output = fs::read_to_string(log).unwrap();
    assert_eq!(output.lines().count(), 2);
    // Git resolves the worktree root physically (for example /var -> /private/var
    // on macOS), while tempfile may preserve an alias in its original path.
    let expected = f.root.canonicalize().unwrap().join("target/xtask-hooks");
    for value in output.lines() {
        assert_eq!(Path::new(value), expected);
    }
}

#[test]
fn prepare_agents_rejects_claim_without_codex_index() {
    let f = bootstrap_fixture();
    let code = f.root.join(".worktrees/codex/task-e15-01-bootstrap");
    fs::create_dir_all(&code).unwrap();
    let claim = f.root.join(".git/auralis/claims/TASK-E15-01");
    fs::create_dir_all(&claim).unwrap();
    fs::write(
        claim.join("owner.json"),
        json!({"task":"TASK-E15-01","agent":"codex","path":code,"branch":"main"}).to_string(),
    )
    .unwrap();
    assert!(!f.run(&["repo", "prepare-agents"]).status.success());
    assert!(claim.join("owner.json").is_file());
}

#[cfg(unix)]
#[test]
fn worktree_rejects_external_agent_alias_before_claim() {
    let f = Fixture::new();
    f.init();
    let outside = tempfile::tempdir().unwrap();
    fs::create_dir(f.root.join(".worktrees")).unwrap();
    std::os::unix::fs::symlink(outside.path(), f.root.join(".worktrees/codex")).unwrap();
    assert!(
        !f.work("TASK-E01-03", "codex", "escape", "feature")
            .status
            .success()
    );
    assert!(!f.root.join(".git/auralis/claims/TASK-E01-03").exists());
    assert_eq!(fs::read_dir(outside.path()).unwrap().count(), 0);
}
