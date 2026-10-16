use crate::util::*;
use anyhow::{Result, bail, ensure};
use regex::Regex;
use serde_json::json;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
fn common(root: &Path) -> Result<PathBuf> {
    Ok(root
        .join(git(root, &["rev-parse", "--git-common-dir"])?)
        .canonicalize()?)
}
fn task(root: &Path, id: &str) -> Result<()> {
    ensure!(
        Regex::new(r"^TASK-E\d{2}-\d{2}$")?.is_match(id),
        "Invalid task ID"
    );
    let data = read_json(root.join("tasks/backlog.json"))?;
    ensure!(
        data["tasks"]
            .as_array()
            .is_some_and(|tasks| tasks.iter().any(|t| t["id"] == id)),
        "Unknown task ID"
    );
    Ok(())
}
pub fn init(root: &Path) -> Result<()> {
    ensure!(
        !root.join(".git").exists(),
        "Repository already initialized"
    );
    ensure!(
        git(root, &["rev-parse", "--show-toplevel"]).is_err(),
        "Nested repository refused"
    );
    run(root, "git", &["init", "-b", "main"])?;
    for args in [
        ["config", "pull.ff", "only"],
        ["config", "fetch.prune", "true"],
        ["config", "core.hooksPath", ".githooks"],
    ] {
        run(root, "git", &args)?;
    }
    run(root, "git", &["add", "--all"])?;
    run(
        root,
        "git",
        &[
            "commit",
            "-m",
            "chore: initialize independent Vynamp project",
        ],
    )?;
    run(root, "git", &["branch", "develop"])?;
    println!("Initialized main/develop. No remote created or pushed.");
    Ok(())
}
pub fn worktree(root: &Path, agent: &str, id: &str, slug: &str, kind: &str) -> Result<()> {
    task(root, id)?;
    ensure!(
        Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")?.is_match(slug),
        "Invalid slug"
    );
    ensure!(
        git(root, &["rev-parse", "--show-superproject-working-tree"])?.is_empty(),
        "Submodule refused"
    );
    let common = common(root)?;
    ensure!(
        PathBuf::from(git(root, &["rev-parse", "--absolute-git-dir"])?).canonicalize()? == common,
        "Already in isolated worktree; do not nest"
    );
    ensure!(
        Command::new("git")
            .args(["check-ignore", "-q", "--no-index", ".worktrees/"])
            .current_dir(root)
            .status()?
            .success(),
        ".worktrees/ must be ignored"
    );
    let relative = format!(".worktrees/{agent}/{}-{slug}", id.to_lowercase());
    validate_destination(root, Path::new(&relative))?;
    let claim = common.join("auralis/claims").join(id);
    fs::create_dir_all(claim.parent().unwrap())?;
    if fs::create_dir(&claim).is_err() {
        bail!("Task already claimed: {id}");
    }
    let branch = format!("{kind}/{}-{agent}-{slug}", id.to_lowercase());
    let path = root.join(format!(".worktrees/{agent}/{}-{slug}", id.to_lowercase()));
    let base = if kind == "hotfix" { "main" } else { "develop" };
    let result = (|| -> Result<()> {
        ensure!(!path.exists(), "Worktree destination exists");
        fs::create_dir_all(path.parent().unwrap())?;
        let record = json!({"task":id,"agent":agent,"path":path,"branch":branch,"base":base});
        write_json(claim.join("owner.json"), &record)?;
        run(
            root,
            "git",
            &[
                "worktree",
                "add",
                "-b",
                &branch,
                &command_path(&path)?,
                base,
            ],
        )?;
        println!("{}", serde_json::to_string_pretty(&record)?);
        Ok(())
    })();
    if result.is_err() && !path.join(".git").exists() {
        let _ = fs::remove_file(claim.join("owner.json"));
        let _ = fs::remove_dir(&claim);
    }
    result
}
pub fn remove(root: &Path, agent: &str, id: &str) -> Result<()> {
    task(root, id)?;
    let shared = common(root)?;
    ensure!(
        git(root, &["rev-parse", "--show-superproject-working-tree"])?.is_empty()
            && PathBuf::from(git(root, &["rev-parse", "--absolute-git-dir"])?).canonicalize()?
                == shared
            && PathBuf::from(git(root, &["rev-parse", "--show-toplevel"])?).canonicalize()?
                == root.canonicalize()?,
        "Run removal from the root checkout, not a worktree or submodule"
    );
    let claim = shared.join("auralis/claims").join(id);
    let rec = read_json(claim.join("owner.json"))?;
    ensure!(rec["task"] == id, "Claim task differs; preserve it");
    ensure!(rec["agent"] == agent, "Worktree belongs to another agent");
    let branch = string(&rec, "branch")?;
    let pattern = Regex::new(
        r"^(feature|fix|chore|hotfix)/(task-e\d{2}-\d{2})-(codex|claude)-([a-z0-9]+(?:-[a-z0-9]+)*)$",
    )?;
    let parts = pattern
        .captures(branch)
        .ok_or_else(|| anyhow::anyhow!("Invalid claimed branch; preserve it"))?;
    ensure!(
        parts[2] == id.to_lowercase() && &parts[3] == agent,
        "Claim branch differs from task/owner; preserve it"
    );
    let base = if &parts[1] == "hotfix" {
        "main"
    } else {
        "develop"
    };
    ensure!(rec["base"] == base, "Claim base differs; preserve it");
    let relative = PathBuf::from(format!(
        ".worktrees/{agent}/{}-{}",
        id.to_lowercase(),
        &parts[4]
    ));
    validate_destination(root, &relative)?;
    let path = PathBuf::from(string(&rec, "path")?);
    ensure!(
        path.is_absolute() && path.canonicalize()? == root.join(relative).canonicalize()?,
        "Unsafe claimed worktree path"
    );
    ensure!(
        path.join(".git").is_file()
            && common(&path)? == shared
            && PathBuf::from(git(&path, &["rev-parse", "--show-toplevel"])?).canonicalize()?
                == path.canonicalize()?,
        "Claim does not identify this repository's worktree; preserve it"
    );
    ensure!(
        git(&path, &["branch", "--show-current"])? == branch,
        "Worktree branch differs from claim (or HEAD is detached); preserve it"
    );
    ensure!(
        git(&path, &["status", "--porcelain"])?.is_empty(),
        "Dirty worktree; removal refused"
    );
    // Check the worktree's actual commit, not merely a possibly stale claim ref.
    let head = git(&path, &["rev-parse", "HEAD"])?;
    let bases = if branch.starts_with("hotfix/") {
        vec!["develop", "main"]
    } else {
        vec!["develop"]
    };
    for base in bases {
        ensure!(
            Command::new("git")
                .args(["merge-base", "--is-ancestor", &head, base])
                .current_dir(root)
                .status()?
                .success(),
            "Unmerged commits in {base}; removal refused"
        );
    }
    // branch -d uses its configured upstream, falling back to the central HEAD.
    // Preflight that extra condition before removing the worktree, so a normal
    // retry can still inspect its path/claim if branch deletion would refuse.
    let deletion_base = git(
        root,
        &["rev-parse", "--verify", &format!("{branch}@{{upstream}}")],
    )
    .unwrap_or_else(|_| "HEAD".to_owned());
    ensure!(
        Command::new("git")
            .args(["merge-base", "--is-ancestor", &head, &deletion_base])
            .current_dir(root)
            .status()?
            .success(),
        "Branch deletion not merged into upstream/checkout HEAD; removal refused"
    );
    run(root, "git", &["worktree", "remove", &command_path(&path)?])?;
    run(root, "git", &["branch", "-d", branch])?;
    fs::remove_file(claim.join("owner.json"))?;
    fs::remove_dir(claim)?;
    Ok(())
}
pub fn guard(root: &Path) -> Result<()> {
    if git(root, &["rev-parse", "--verify", "HEAD"]).is_err() {
        return Ok(());
    }
    let b = git(root, &["branch", "--show-current"])?;
    ensure!(
        !b.is_empty() && b != "main" && b != "develop",
        "Commit blocked on main/develop/detached HEAD"
    );
    ensure!(Regex::new(r"^(?:(?:feature|fix|chore|hotfix)/task-e\d{2}-\d{2}-(?:codex|claude)-[a-z0-9]+(?:-[a-z0-9]+)*|release/\d+\.\d+\.\d+(?:-[a-z0-9.-]+)?)$")?.is_match(&b),"Commit blocked: invalid Gitflow branch");
    Ok(())
}
pub fn policy(root: &Path, repo: &str, reviews: u8) -> Result<()> {
    ensure!(
        Regex::new(r"^[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+$")?.is_match(repo),
        "Use OWNER/REPOSITORY"
    );
    run(
        root,
        "gh",
        &[
            "api",
            &format!("repos/{repo}"),
            "--method",
            "PATCH",
            "-F",
            "allow_merge_commit=true",
            "-F",
            "allow_squash_merge=false",
            "-F",
            "allow_rebase_merge=false",
            "-F",
            "delete_branch_on_merge=true",
            "-f",
            "default_branch=develop",
        ],
    )?;
    let p = json!({"required_status_checks":{"strict":true,"contexts":["CI Gate"]},"enforce_admins":true,"required_pull_request_reviews":{"dismiss_stale_reviews":true,"require_code_owner_reviews":false,"required_approving_review_count":reviews},"restrictions":null,"required_linear_history":false,"allow_force_pushes":false,"allow_deletions":false,"required_conversation_resolution":true});
    let tmp = tempfile::tempdir()?;
    let file = tmp.path().join("protection.json");
    write_json(&file, &p)?;
    for b in ["main", "develop"] {
        run(
            root,
            "gh",
            &[
                "api",
                &format!("repos/{repo}/branches/{b}/protection"),
                "--method",
                "PUT",
                "--input",
                &command_path(&file)?,
            ],
        )?;
    }
    println!("Protection applied. Configure protected signing/publish environments separately.");
    Ok(())
}

pub fn prepare_agents(root: &Path) -> Result<()> {
    const ID: &str = "TASK-E15-01";
    task(root, ID)?;
    let shared = common(root)?;
    ensure!(
        PathBuf::from(git(root, &["rev-parse", "--absolute-git-dir"])?).canonicalize()? == shared,
        "Run prepare-agents from the root checkout, not a worktree"
    );
    let code_rel = Path::new(".worktrees/codex/task-e15-01-bootstrap");
    let review_rel = Path::new(".worktrees/claude/review-bootstrap");
    validate_destination(root, code_rel)?;
    validate_destination(root, review_rel)?;
    let code = root.join(code_rel);
    let review = root.join(review_rel);
    let claim = shared.join("auralis/claims").join(ID);
    let branch;
    if claim.exists() {
        let record = read_json(claim.join("owner.json"))?;
        ensure!(
            record["agent"] == "codex" && record["task"] == ID,
            "Bootstrap task already claimed by another owner"
        );
        ensure!(
            PathBuf::from(string(&record, "path")?).canonicalize()? == code.canonicalize()?,
            "Existing claim points to a different worktree; preserve it"
        );
        ensure!(
            code.join(".git").is_file(),
            "Claim has no registered Codex worktree; preserve it"
        );
        ensure!(
            PathBuf::from(git(&code, &["rev-parse", "--absolute-git-dir"])?).canonicalize()?
                != shared,
            "Codex must have its own Git index"
        );
        ensure!(
            common(&code)? == shared,
            "Codex worktree belongs to another repository"
        );
        branch = string(&record, "branch")?.to_owned();
        ensure!(
            git(&code, &["branch", "--show-current"])? == branch,
            "Codex branch differs from claim"
        );
    } else {
        ensure!(
            !code.exists(),
            "Codex path exists without claim; preserve it"
        );
        let current = git(root, &["branch", "--show-current"])?;
        if Regex::new(r"^chore/task-e15-01-codex-project-bootstrap(?:-[0-9]+)?$")?
            .is_match(&current)
        {
            ensure!(
                git(root, &["status", "--porcelain"])?.is_empty(),
                "Import checkout has pending changes; commit or preserve them before preparing agents"
            );
            fs::create_dir_all(claim.parent().unwrap())?;
            fs::create_dir(&claim)?;
            let result = (|| -> Result<()> {
                fs::create_dir_all(code.parent().unwrap())?;
                write_json(
                    claim.join("owner.json"),
                    &json!({"task":ID,"agent":"codex","path":code,"branch":current,"base":"import-bootstrap"}),
                )?;
                run(root, "git", &["switch", "--detach"])?;
                run(
                    root,
                    "git",
                    &["worktree", "add", &command_path(&code)?, &current],
                )
            })();
            if let Err(error) = result {
                if !code.join(".git").exists() {
                    let _ = fs::remove_file(claim.join("owner.json"));
                    let _ = fs::remove_dir(&claim);
                    let _ = run(root, "git", &["switch", &current]);
                }
                return Err(error);
            }
            branch = current;
        } else {
            ensure!(
                matches!(current.as_str(), "main" | "develop"),
                "Unexpected root branch; finish existing work before bootstrap"
            );
            ensure!(
                git(root, &["rev-parse", "--verify", "develop"]).is_ok(),
                "Develop missing: import the package or integrate the bootstrap PR first"
            );
            worktree(root, "codex", ID, "bootstrap", "feature")?;
            branch = "feature/task-e15-01-codex-bootstrap".to_owned();
        }
    }
    if review.exists() {
        ensure!(
            review.join(".git").is_file(),
            "Review path is not a registered worktree; preserve it"
        );
        ensure!(
            PathBuf::from(git(&review, &["rev-parse", "--absolute-git-dir"])?).canonicalize()?
                != shared,
            "Reviewer must have its own Git index"
        );
        ensure!(
            common(&review)? == shared && git(&review, &["branch", "--show-current"])?.is_empty(),
            "Existing review path must be a detached worktree of this repository; preserve it"
        );
    } else {
        fs::create_dir_all(review.parent().unwrap())?;
        run(
            root,
            "git",
            &[
                "worktree",
                "add",
                "--detach",
                &command_path(&review)?,
                &branch,
            ],
        )?;
    }
    println!(
        "{}",
        json!({"codex_worktree":code,"codex_branch":branch,"claude_review_worktree":review,"task":ID,"review_only":true,"note":"No task has been accepted; verify Superpowers and prerequisites, review commits before feature work."})
    );
    Ok(())
}
