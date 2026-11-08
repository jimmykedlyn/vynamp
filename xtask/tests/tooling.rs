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
    fn claim(&self, task: &str) -> PathBuf {
        self.root.join(".git/auralis/claims").join(task)
    }
    fn remove(&self, task: &str, agent: &str) -> Output {
        self.run(&["repo", "remove-worktree", "--task", task, "--agent", agent])
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
    ok(f.work("TASK-E01-03", "claude", "state", "feature"));
    ok(f.work("TASK-E02-03", "claude", "ui", "feature"));
    let a = f.path("claude", "TASK-E01-03", "state");
    let b = f.path("claude", "TASK-E02-03", "ui");
    let index = |path: &Path| {
        let o = f.cmd("git", &["rev-parse", "--git-path", "index"], path);
        assert!(o.status.success());
        path.join(String::from_utf8(o.stdout).unwrap().trim())
            .canonicalize()
            .unwrap()
    };
    assert_ne!(index(&a), index(&b));
    assert_ne!(index(&a), index(&f.root));
    let root_index = fs::read(index(&f.root)).unwrap();
    let b_index = fs::read(index(&b)).unwrap();
    fs::write(a.join("isolated.txt"), "private").unwrap();
    ok(f.cmd("git", &["add", "isolated.txt"], &a));
    assert_eq!(fs::read(index(&b)).unwrap(), b_index);
    fs::write(b.join("review.txt"), "reviewer").unwrap();
    ok(f.cmd("git", &["add", "review.txt"], &b));
    assert_eq!(fs::read(index(&f.root)).unwrap(), root_index);
    assert_eq!(
        String::from_utf8_lossy(
            &f.cmd("git", &["diff", "--cached", "--name-only"], &a)
                .stdout
        )
        .trim(),
        "isolated.txt"
    );
    assert_eq!(
        String::from_utf8_lossy(
            &f.cmd("git", &["diff", "--cached", "--name-only"], &b)
                .stdout
        )
        .trim(),
        "review.txt"
    );
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
    ok(f.work("TASK-E01-03", "claude", "one", "feature"));
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
        !f.work("TASK-E01-03", "claude", "../../escape", "feature")
            .status
            .success()
    );
    assert!(
        !f.work("TASK-E99-99", "claude", "unknown", "feature")
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
    for state in ["untracked", "modified", "staged"] {
        let f = Fixture::new();
        f.init();
        ok(f.work("TASK-E01-03", "claude", "state", "feature"));
        let p = f.path("claude", "TASK-E01-03", "state");
        let file = if state == "untracked" {
            "precious"
        } else {
            "README.md"
        };
        fs::write(p.join(file), "keep").unwrap();
        if state == "staged" {
            ok(f.cmd("git", &["add", file], &p));
        }
        let claim = fs::read(f.claim("TASK-E01-03").join("owner.json")).unwrap();
        let status = f.cmd("git", &["status", "--porcelain"], &p).stdout;
        let o = f.remove("TASK-E01-03", "claude");
        assert!(!o.status.success());
        assert!(String::from_utf8_lossy(&o.stderr).contains("Dirty worktree"));
        assert_eq!(fs::read(p.join(file)).unwrap(), b"keep");
        assert_eq!(f.cmd("git", &["status", "--porcelain"], &p).stdout, status);
        assert_eq!(
            fs::read(f.claim("TASK-E01-03").join("owner.json")).unwrap(),
            claim
        );
    }
}
#[test]
fn unmerged_worktree_is_preserved() {
    let f = Fixture::new();
    f.init();
    ok(f.work("TASK-E01-03", "claude", "state", "feature"));
    let p = f.path("claude", "TASK-E01-03", "state");
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
            "claude"
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
    ok(f.work("TASK-E01-03", "claude", "hot", "hotfix"));
    let p = f.path("claude", "TASK-E01-03", "hot");
    fs::write(p.join("hot"), "fix").unwrap();
    ok(f.cmd("git", &["add", "hot"], &p));
    ok(f.cmd("git", &["commit", "-m", "fix: urgent"], &p));
    ok(f.git(&["switch", "develop"]));
    ok(f.git(&[
        "merge",
        "--no-ff",
        "hotfix/task-e01-03-claude-hot",
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
            "claude"
        ])
        .status
        .success()
    );
    ok(f.git(&["switch", "main"]));
    ok(f.git(&[
        "merge",
        "--no-ff",
        "hotfix/task-e01-03-claude-hot",
        "-m",
        "merge stable",
    ]));
    ok(f.run(&[
        "repo",
        "remove-worktree",
        "--task",
        "TASK-E01-03",
        "--agent",
        "claude",
    ]));
    assert!(!p.exists());
}
#[test]
fn no_nested_worktrees() {
    let f = Fixture::new();
    f.init();
    ok(f.work("TASK-E01-03", "claude", "state", "feature"));
    let p = f.path("claude", "TASK-E01-03", "state");
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

// REQ-E15-02: assert resource effects with real Git, rather than source text.
#[test]
fn concurrent_task_claim_has_one_winner() {
    let f = Fixture::new();
    f.init();
    let barrier = std::sync::Barrier::new(2);
    let results = std::thread::scope(|scope| {
        let a = scope.spawn(|| {
            barrier.wait();
            f.work("TASK-E01-03", "claude", "race-a", "feature")
        });
        let b = scope.spawn(|| {
            barrier.wait();
            f.work("TASK-E01-03", "claude", "race-b", "feature")
        });
        [a.join().unwrap(), b.join().unwrap()]
    });
    assert_eq!(results.iter().filter(|o| o.status.success()).count(), 1);
    let (winner, loser) = if results[0].status.success() {
        ("race-a", "race-b")
    } else {
        ("race-b", "race-a")
    };
    let rec: Value =
        serde_json::from_slice(&fs::read(f.claim("TASK-E01-03").join("owner.json")).unwrap())
            .unwrap();
    assert_eq!(rec["agent"], "claude");
    assert_eq!(rec["task"], "TASK-E01-03");
    assert_eq!(
        rec["branch"],
        format!("feature/task-e01-03-claude-{winner}")
    );
    assert_eq!(
        Path::new(rec["path"].as_str().unwrap())
            .canonicalize()
            .unwrap(),
        f.path("claude", "TASK-E01-03", winner)
            .canonicalize()
            .unwrap()
    );
    assert!(
        f.path("claude", "TASK-E01-03", winner)
            .join(".git")
            .is_file()
    );
    assert!(!f.path("claude", "TASK-E01-03", loser).exists());
    assert!(
        !f.git(&[
            "show-ref",
            "--verify",
            &format!("refs/heads/feature/task-e01-03-claude-{loser}")
        ])
        .status
        .success()
    );
    let error = &results.iter().find(|o| !o.status.success()).unwrap().stderr;
    assert!(String::from_utf8_lossy(error).contains("Task already claimed"));
}

#[test]
fn failed_creation_preserves_existing_resources_and_releases_own_claim() {
    let f = Fixture::new();
    f.init();
    let path = f.path("claude", "TASK-E01-03", "existing");
    fs::create_dir_all(&path).unwrap();
    fs::write(path.join("precious"), "keep").unwrap();
    assert!(
        !f.work("TASK-E01-03", "claude", "existing", "feature")
            .status
            .success()
    );
    assert_eq!(fs::read(path.join("precious")).unwrap(), b"keep");
    assert!(!f.claim("TASK-E01-03").exists());
    ok(f.git(&["branch", "feature/task-e01-03-claude-collision"]));
    let before = f
        .git(&["rev-parse", "feature/task-e01-03-claude-collision"])
        .stdout;
    assert!(
        !f.work("TASK-E01-03", "claude", "collision", "feature")
            .status
            .success()
    );
    assert!(!f.claim("TASK-E01-03").exists());
    assert_eq!(
        f.git(&["rev-parse", "feature/task-e01-03-claude-collision"])
            .stdout,
        before
    );
    ok(f.work("TASK-E01-03", "claude", "retry", "feature"));
}

#[test]
fn invalid_worktree_arguments_have_no_side_effects() {
    let f = Fixture::new();
    f.init();
    let refs = f.git(&["show-ref"]).stdout;
    for (id, agent, slug, kind) in [
        ("../../escape", "claude", "valid", "feature"),
        ("TASK-E99-99", "claude", "valid", "feature"),
        ("TASK-E01-03", "../claude", "valid", "feature"),
        ("TASK-E01-03", "claude", "../escape", "feature"),
        ("TASK-E01-03", "claude", "C:\\escape", "feature"),
        ("TASK-E01-03", "claude", "valid", "../main"),
    ] {
        assert!(!f.work(id, agent, slug, kind).status.success());
        assert_eq!(f.git(&["show-ref"]).stdout, refs);
        assert!(!f.root.join(".worktrees").exists());
        assert!(!f.root.join(".git/auralis/claims").exists());
    }
}

#[test]
fn interrupted_claim_is_preserved_without_automatic_takeover() {
    let f = Fixture::new();
    f.init();
    fs::create_dir_all(f.claim("TASK-E01-03")).unwrap();
    assert!(
        !f.work("TASK-E01-03", "claude", "retry", "feature")
            .status
            .success()
    );
    assert!(f.claim("TASK-E01-03").is_dir());
    assert!(!f.path("claude", "TASK-E01-03", "retry").exists());
}

#[test]
fn integrated_feature_removal_cleans_only_its_resources() {
    let f = Fixture::new();
    f.init();
    ok(f.work("TASK-E01-03", "claude", "done", "feature"));
    ok(f.work("TASK-E02-03", "claude", "keep", "chore"));
    let p = f.path("claude", "TASK-E01-03", "done");
    fs::write(p.join("new"), "merged").unwrap();
    ok(f.cmd("git", &["add", "new"], &p));
    ok(f.cmd("git", &["commit", "-m", "feat: complete fixture"], &p));
    ok(f.git(&["switch", "develop"]));
    ok(f.git(&[
        "merge",
        "--no-ff",
        "feature/task-e01-03-claude-done",
        "-m",
        "merge fixture",
    ]));
    let review_claim = fs::read(f.claim("TASK-E02-03").join("owner.json")).unwrap();
    ok(f.remove("TASK-E01-03", "claude"));
    assert!(!p.exists());
    assert!(!f.claim("TASK-E01-03").exists());
    assert!(
        !f.git(&[
            "show-ref",
            "--verify",
            "refs/heads/feature/task-e01-03-claude-done"
        ])
        .status
        .success()
    );
    assert_eq!(fs::read(f.root.join("new")).unwrap(), b"merged");
    assert_eq!(
        fs::read(f.claim("TASK-E02-03").join("owner.json")).unwrap(),
        review_claim
    );
    assert!(
        f.path("claude", "TASK-E02-03", "keep")
            .join(".git")
            .is_file()
    );
}

fn integrated_feature_for_deletion() -> (Fixture, PathBuf) {
    let f = Fixture::new();
    f.init();
    ok(f.work("TASK-E01-03", "claude", "deletion", "feature"));
    let p = f.path("claude", "TASK-E01-03", "deletion");
    fs::write(p.join("new"), "merged in develop only").unwrap();
    ok(f.cmd("git", &["add", "new"], &p));
    ok(f.cmd("git", &["commit", "-m", "feat: deletion fixture"], &p));
    ok(f.git(&["switch", "develop"]));
    ok(f.git(&[
        "merge",
        "--no-ff",
        "feature/task-e01-03-claude-deletion",
        "-m",
        "merge fixture",
    ]));
    (f, p)
}

fn assert_branch_deletion_preflight_preserves_resources(f: &Fixture, p: &Path) {
    let claim = fs::read(f.claim("TASK-E01-03").join("owner.json")).unwrap();
    let head = f.cmd("git", &["rev-parse", "HEAD"], p).stdout;
    let o = f.remove("TASK-E01-03", "claude");
    assert!(!o.status.success());
    assert!(
        p.join(".git").is_file(),
        "worktree removed before branch deletion was validated"
    );
    assert_eq!(fs::read(p.join("new")).unwrap(), b"merged in develop only");
    assert_eq!(f.cmd("git", &["rev-parse", "HEAD"], p).stdout, head);
    assert_eq!(
        fs::read(f.claim("TASK-E01-03").join("owner.json")).unwrap(),
        claim
    );
    ok(f.git(&[
        "show-ref",
        "--verify",
        "refs/heads/feature/task-e01-03-claude-deletion",
    ]));
    let error = String::from_utf8_lossy(&o.stderr);
    assert!(
        error.contains("Branch deletion not merged into upstream/checkout HEAD; removal refused"),
        "{error}"
    );
}

fn assert_deletion_resources_removed(f: &Fixture, p: &Path) {
    assert!(!p.exists());
    assert!(!f.claim("TASK-E01-03").exists());
    assert!(
        !f.git(&[
            "show-ref",
            "--verify",
            "refs/heads/feature/task-e01-03-claude-deletion"
        ])
        .status
        .success()
    );
}

#[test]
fn removal_preserves_integrated_feature_when_central_head_is_unmerged() {
    let (f, p) = integrated_feature_for_deletion();
    ok(f.git(&["switch", "main"]));
    assert_branch_deletion_preflight_preserves_resources(&f, &p);
    // A normal retry works after choosing the integration branch centrally.
    ok(f.git(&["switch", "develop"]));
    ok(f.remove("TASK-E01-03", "claude"));
    assert_deletion_resources_removed(&f, &p);
}

#[test]
fn removal_preflights_configured_upstream() {
    let (f, p) = integrated_feature_for_deletion();
    ok(f.git(&[
        "branch",
        "--set-upstream-to=main",
        "feature/task-e01-03-claude-deletion",
    ]));
    // Even with central HEAD in develop, branch -d prefers the upstream main.
    assert_branch_deletion_preflight_preserves_resources(&f, &p);
    ok(f.git(&[
        "branch",
        "--set-upstream-to=develop",
        "feature/task-e01-03-claude-deletion",
    ]));
    ok(f.git(&["switch", "main"]));
    // Conversely, a merged upstream permits deletion while central HEAD is main.
    ok(f.remove("TASK-E01-03", "claude"));
    assert_deletion_resources_removed(&f, &p);
    assert_eq!(
        String::from_utf8_lossy(&f.git(&["branch", "--show-current"]).stdout).trim(),
        "main"
    );
}

#[test]
fn hotfix_merged_only_to_main_is_preserved() {
    let f = Fixture::new();
    f.init();
    ok(f.work("TASK-E01-03", "claude", "hot", "hotfix"));
    let p = f.path("claude", "TASK-E01-03", "hot");
    fs::write(p.join("hot"), "fix").unwrap();
    ok(f.cmd("git", &["add", "hot"], &p));
    ok(f.cmd("git", &["commit", "-m", "fix: fixture"], &p));
    ok(f.git(&[
        "merge",
        "--no-ff",
        "hotfix/task-e01-03-claude-hot",
        "-m",
        "main only",
    ]));
    assert!(!f.remove("TASK-E01-03", "claude").status.success());
    assert_eq!(fs::read(p.join("hot")).unwrap(), b"fix");
    assert!(f.claim("TASK-E01-03").join("owner.json").exists());
}

fn check_removal_after_checkout_drift(detached: bool) {
    let f = Fixture::new();
    f.init();
    ok(f.work("TASK-E01-03", "claude", "drift", "feature"));
    let p = f.path("claude", "TASK-E01-03", "drift");
    if detached {
        ok(f.cmd("git", &["switch", "--detach"], &p));
    } else {
        ok(f.cmd("git", &["switch", "-c", "fix/task-e02-03-claude-other"], &p));
    }
    fs::write(p.join("precious"), "unmerged").unwrap();
    ok(f.cmd("git", &["add", "precious"], &p));
    ok(f.cmd("git", &["commit", "-m", "fix: preserve actual HEAD"], &p));
    let head = f.cmd("git", &["rev-parse", "HEAD"], &p).stdout;
    let claim = fs::read(f.claim("TASK-E01-03").join("owner.json")).unwrap();
    assert!(
        !f.remove("TASK-E01-03", "claude").status.success(),
        "removed drifted worktree (detached={detached})"
    );
    assert_eq!(fs::read(p.join("precious")).unwrap(), b"unmerged");
    assert_eq!(f.cmd("git", &["rev-parse", "HEAD"], &p).stdout, head);
    assert_eq!(
        fs::read(f.claim("TASK-E01-03").join("owner.json")).unwrap(),
        claim
    );
}

#[test]
fn removal_preserves_switched_worktree() {
    check_removal_after_checkout_drift(false);
}

#[test]
fn removal_preserves_detached_worktree() {
    check_removal_after_checkout_drift(true);
}

fn check_inconsistent_claim(key: &str, value: &str, expected_error: &str) {
    let f = Fixture::new();
    f.init();
    ok(f.work("TASK-E01-03", "claude", "metadata", "feature"));
    let file = f.claim("TASK-E01-03").join("owner.json");
    let mut rec: Value = serde_json::from_slice(&fs::read(&file).unwrap()).unwrap();
    rec[key] = json!(value);
    fs::write(&file, rec.to_string()).unwrap();
    let before = fs::read(&file).unwrap();
    let o = f.remove("TASK-E01-03", "claude");
    assert!(!o.status.success(), "accepted inconsistent {key}");
    let error = String::from_utf8_lossy(&o.stderr);
    assert!(error.contains(expected_error), "{error}");
    assert!(
        f.path("claude", "TASK-E01-03", "metadata")
            .join(".git")
            .is_file()
    );
    assert_eq!(fs::read(&file).unwrap(), before);
}

#[test]
fn removal_preserves_inconsistent_task() {
    check_inconsistent_claim("task", "TASK-E02-03", "Claim task differs; preserve it");
}

#[test]
fn removal_preserves_inconsistent_base() {
    check_inconsistent_claim("base", "main", "Claim base differs; preserve it");
}

#[test]
fn removal_preserves_inconsistent_branch() {
    check_inconsistent_claim("branch", "develop", "Invalid claimed branch; preserve it");
}

#[test]
fn removal_by_other_owner_preserves_claim_and_worktree() {
    let f = Fixture::new();
    f.init();
    ok(f.work("TASK-E01-03", "claude", "owner", "feature"));
    let file = f.claim("TASK-E01-03").join("owner.json");
    let mut rec: Value = serde_json::from_slice(&fs::read(&file).unwrap()).unwrap();
    rec["agent"] = json!("someone-else");
    fs::write(&file, rec.to_string()).unwrap();
    let claim = fs::read(&file).unwrap();
    let o = f.remove("TASK-E01-03", "claude");
    assert!(!o.status.success());
    let error = String::from_utf8_lossy(&o.stderr);
    assert!(
        error.contains("Worktree belongs to another agent"),
        "{error}"
    );
    assert_eq!(
        fs::read(f.claim("TASK-E01-03").join("owner.json")).unwrap(),
        claim
    );
    assert!(
        f.path("claude", "TASK-E01-03", "owner")
            .join(".git")
            .is_file()
    );
}

#[test]
fn removal_preserves_foreign_repository_and_claim() {
    let f = Fixture::new();
    f.init();
    ok(f.work("TASK-E01-03", "claude", "foreign", "feature"));
    let foreign = Fixture::new();
    foreign.init();
    let file = f.claim("TASK-E01-03").join("owner.json");
    let mut rec: Value = serde_json::from_slice(&fs::read(&file).unwrap()).unwrap();
    rec["path"] = json!(foreign.root);
    fs::write(&file, rec.to_string()).unwrap();
    let claim = fs::read(&file).unwrap();
    let index = fs::read(foreign.root.join(".git/index")).unwrap();
    let o = f.remove("TASK-E01-03", "claude");
    assert!(!o.status.success());
    let error = String::from_utf8_lossy(&o.stderr);
    assert!(error.contains("Unsafe claimed worktree path"), "{error}");
    assert_eq!(fs::read(&file).unwrap(), claim);
    assert_eq!(fs::read(foreign.root.join(".git/index")).unwrap(), index);
    assert!(
        f.path("claude", "TASK-E01-03", "foreign")
            .join(".git")
            .is_file()
    );
}

#[cfg(unix)]
#[test]
fn symlink_worktree_destination_is_preserved() {
    let f = Fixture::new();
    f.init();
    let outside = f._temp.path().join("outside");
    fs::create_dir(&outside).unwrap();
    fs::write(outside.join("precious"), "keep").unwrap();
    std::os::unix::fs::symlink(&outside, f.root.join(".worktrees")).unwrap();
    assert!(
        !f.work("TASK-E01-03", "claude", "escape", "feature")
            .status
            .success()
    );
    assert_eq!(fs::read(outside.join("precious")).unwrap(), b"keep");
    assert!(!f.claim("TASK-E01-03").exists());
    assert!(!outside.join("claude").exists());
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
fn guard_accepts_agent_chore_branch_without_task_but_not_arbitrary_names() {
    let f = Fixture::new();
    f.init();
    ok(f.git(&["switch", "-c", "chore/claude-sole-owner"]));
    ok(f.run(&["guard-branch"]));
    ok(f.git(&["switch", "-c", "chore/anything-else"]));
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
    let result = f.work("TASK-E01-03", "claude", "new", "feature");
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
    ok(f.git(&["switch", "-c", "fix/task-e01-03-claude-hooktest"]));
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

#[cfg(unix)]
#[test]
fn worktree_rejects_external_agent_alias_before_claim() {
    let f = Fixture::new();
    f.init();
    let outside = tempfile::tempdir().unwrap();
    fs::create_dir(f.root.join(".worktrees")).unwrap();
    std::os::unix::fs::symlink(outside.path(), f.root.join(".worktrees/claude")).unwrap();
    assert!(
        !f.work("TASK-E01-03", "claude", "escape", "feature")
            .status
            .success()
    );
    assert!(!f.root.join(".git/auralis/claims/TASK-E01-03").exists());
    assert_eq!(fs::read_dir(outside.path()).unwrap().count(), 0);
}
