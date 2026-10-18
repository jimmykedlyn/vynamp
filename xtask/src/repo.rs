use crate::util::*;
use anyhow::{Result, bail, ensure};
use regex::Regex;
use serde_json::{Value, json};
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
const PROTECTED: [&str; 2] = ["main", "develop"];
/// GitHub answers HTTP 403 with this text when a private repository's plan lacks protection.
fn plan_blocks_protection(error: &str) -> bool {
    error.contains("Upgrade to GitHub Pro")
}
fn protection_status(result: &Result<Value, String>) -> String {
    match result {
        Ok(p) => {
            let gate = p["required_status_checks"]["strict"] == true
                && p["required_status_checks"]["contexts"]
                    .as_array()
                    .is_some_and(|c| c.iter().any(|c| c == "CI Gate"));
            let locked = p["allow_force_pushes"]["enabled"] == false
                && p["allow_deletions"]["enabled"] == false
                && p["enforce_admins"]["enabled"] == true;
            // PR required even with review_count 0 (solo account); absent fields fail closed.
            let reviewed = p["required_pull_request_reviews"]["dismiss_stale_reviews"] == true
                && p["required_conversation_resolution"]["enabled"] == true;
            if gate && locked && reviewed {
                "applied"
            } else {
                "incomplete"
            }
            .to_owned()
        }
        Err(e) if plan_blocks_protection(e) => "unavailable_plan".to_owned(),
        Err(e) if e.contains("Branch not protected") => "absent".to_owned(),
        Err(e) => format!("error: {}", e.trim()),
    }
}
/// Compare the remote state with the Gitflow policy; every gap is listed, never inferred as applied.
fn audit_report(meta: &Value, protections: &[(&str, Result<Value, String>)]) -> Value {
    let mut pending = Vec::new();
    if meta["private"] != true {
        pending.push("repository is not private".to_owned());
    }
    if meta["default_branch"] != "develop" {
        pending.push(format!(
            "default_branch is {} instead of develop",
            meta["default_branch"]
        ));
    }
    if meta["allow_merge_commit"] != true
        || meta["allow_squash_merge"] != false
        || meta["allow_rebase_merge"] != false
    {
        pending.push("merge methods are not merge commit only".to_owned());
    }
    if meta["delete_branch_on_merge"] != true {
        pending.push(format!(
            "delete_branch_on_merge is {} instead of true",
            meta["delete_branch_on_merge"]
        ));
    }
    let mut status = serde_json::Map::new();
    for (branch, result) in protections {
        let s = protection_status(result);
        if s != "applied" {
            pending.push(format!(
                "{branch} protection {s}: CI Gate/PR/stale-review/conversation/force-push/deletion rules not enforced"
            ));
        }
        status.insert((*branch).to_owned(), json!(s));
    }
    json!({"repository":meta["full_name"],"private":meta["private"],"default_branch":meta["default_branch"],"merge":{"allow_merge_commit":meta["allow_merge_commit"],"allow_squash_merge":meta["allow_squash_merge"],"allow_rebase_merge":meta["allow_rebase_merge"],"delete_branch_on_merge":meta["delete_branch_on_merge"]},"protection":status,"pending":pending})
}
fn checked_repo(repo: &str) -> Result<()> {
    ensure!(
        Regex::new(r"^[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+$")?.is_match(repo),
        "Use OWNER/REPOSITORY"
    );
    Ok(())
}
fn repo_meta(root: &Path, repo: &str) -> Result<Value> {
    Ok(serde_json::from_str(&capture(
        root,
        "gh",
        &["api", &format!("repos/{repo}")],
    )?)?)
}
/// Read-only: print the real remote state and fail while any Gitflow rule is pending.
pub fn audit(root: &Path, repo: &str, out: Option<&Path>) -> Result<()> {
    checked_repo(repo)?;
    let meta = repo_meta(root, repo)?;
    let protections: Vec<_> = PROTECTED
        .iter()
        .map(|b| {
            let r = capture(
                root,
                "gh",
                &["api", &format!("repos/{repo}/branches/{b}/protection")],
            )
            .map_err(|e| e.to_string())
            .and_then(|s| serde_json::from_str(&s).map_err(|e| e.to_string()));
            (*b, r)
        })
        .collect();
    let report = audit_report(&meta, &protections);
    if let Some(out) = out {
        write_json(out, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    let pending = report["pending"].as_array().map_or(0, Vec::len);
    ensure!(
        pending == 0,
        "Gitflow not fully enforced: {pending} pending item(s)"
    );
    Ok(())
}
pub fn policy(root: &Path, repo: &str, reviews: u8) -> Result<()> {
    checked_repo(repo)?;
    ensure!(
        repo_meta(root, repo)?["private"] == true,
        "Repository not confirmed private; nothing changed"
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
    for b in PROTECTED {
        if let Err(e) = capture(
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
        ) {
            if plan_blocks_protection(&e.to_string()) {
                bail!(
                    "PENDING: branch protection unavailable for this private repository plan (HTTP 403). Repository settings were applied (merge commit only, delete branch on merge, default develop); main/develop are NOT protected and CI Gate is NOT required."
                );
            }
            return Err(e.context(format!(
                "Protecting {b} failed after repository settings were applied"
            )));
        }
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

#[cfg(test)]
mod github_policy_tests {
    use super::*;
    const PLAN_403: &str = "Command failed: gh: gh: Upgrade to GitHub Pro or make this repository public to enable this feature. (HTTP 403)";
    fn meta(default: &str, squash: bool) -> serde_json::Value {
        json!({"full_name":"o/r","private":true,"default_branch":default,"allow_merge_commit":true,"allow_squash_merge":squash,"allow_rebase_merge":false,"delete_branch_on_merge":true})
    }
    fn applied() -> serde_json::Value {
        json!({"required_status_checks":{"strict":true,"contexts":["CI Gate"]},"enforce_admins":{"enabled":true},"required_pull_request_reviews":{"dismiss_stale_reviews":true,"required_approving_review_count":0},"required_conversation_resolution":{"enabled":true},"allow_force_pushes":{"enabled":false},"allow_deletions":{"enabled":false}})
    }
    fn assert_weakened_is_incomplete(weak: serde_json::Value) {
        let r = audit_report(
            &meta("develop", false),
            &[("main", Ok(weak)), ("develop", Ok(applied()))],
        );
        assert_eq!(r["protection"]["main"], "incomplete");
        assert_eq!(r["protection"]["develop"], "applied");
        assert_eq!(r["pending"].as_array().unwrap().len(), 1, "{r}");
    }
    #[test]
    fn protection_without_required_pull_request_is_incomplete() {
        let mut weak = applied();
        weak["required_pull_request_reviews"] = json!(null);
        assert_weakened_is_incomplete(weak.clone());
        weak.as_object_mut()
            .unwrap()
            .remove("required_pull_request_reviews");
        assert_weakened_is_incomplete(weak);
    }
    #[test]
    fn protection_keeping_stale_reviews_is_incomplete() {
        let mut weak = applied();
        weak["required_pull_request_reviews"]["dismiss_stale_reviews"] = json!(false);
        assert_weakened_is_incomplete(weak.clone());
        weak["required_pull_request_reviews"]
            .as_object_mut()
            .unwrap()
            .remove("dismiss_stale_reviews");
        assert_weakened_is_incomplete(weak);
    }
    #[test]
    fn protection_without_conversation_resolution_is_incomplete() {
        let mut weak = applied();
        weak["required_conversation_resolution"]["enabled"] = json!(false);
        assert_weakened_is_incomplete(weak.clone());
        weak.as_object_mut()
            .unwrap()
            .remove("required_conversation_resolution");
        assert_weakened_is_incomplete(weak);
    }
    #[test]
    fn solo_review_count_zero_still_counts_as_applied() {
        let mut solo = applied();
        solo["required_pull_request_reviews"]["required_approving_review_count"] = json!(0);
        let r = audit_report(
            &meta("develop", false),
            &[("main", Ok(solo)), ("develop", Ok(applied()))],
        );
        assert_eq!(r["pending"], json!([]));
    }
    #[test]
    fn delete_branch_on_merge_drift_is_pending() {
        for drift in [json!(false), json!(null)] {
            let mut m = meta("develop", false);
            m["delete_branch_on_merge"] = drift;
            let r = audit_report(&m, &[("main", Ok(applied())), ("develop", Ok(applied()))]);
            let pending = r["pending"].as_array().unwrap();
            assert_eq!(pending.len(), 1, "{pending:?}");
            assert!(pending[0].as_str().unwrap().contains("delete_branch_on_merge"));
        }
    }
    #[test]
    fn plan_limited_protection_is_recognised() {
        assert!(plan_blocks_protection(PLAN_403));
        assert!(!plan_blocks_protection("gh: Not Found (HTTP 404)"));
    }
    #[test]
    fn fully_enforced_gitflow_has_no_pending_items() {
        let r = audit_report(
            &meta("develop", false),
            &[("main", Ok(applied())), ("develop", Ok(applied()))],
        );
        assert_eq!(r["protection"]["main"], "applied");
        assert_eq!(r["pending"], json!([]));
    }
    #[test]
    fn plan_limit_and_settings_drift_are_reported_not_hidden() {
        let r = audit_report(
            &meta("main", true),
            &[
                ("main", Err(PLAN_403.to_owned())),
                (
                    "develop",
                    Err("gh: Branch not protected (HTTP 404)".to_owned()),
                ),
            ],
        );
        assert_eq!(r["protection"]["main"], "unavailable_plan");
        assert_eq!(r["protection"]["develop"], "absent");
        let pending = r["pending"].as_array().unwrap();
        assert_eq!(pending.len(), 4, "{pending:?}");
        assert!(
            pending
                .iter()
                .any(|p| p.as_str().unwrap().contains("default_branch"))
        );
        assert!(
            pending
                .iter()
                .any(|p| p.as_str().unwrap().contains("merge commit only"))
        );
    }
    #[test]
    fn protection_without_ci_gate_or_allowing_force_push_is_incomplete() {
        let mut weak = applied();
        weak["allow_force_pushes"]["enabled"] = json!(true);
        let r = audit_report(
            &meta("develop", false),
            &[
                ("main", Ok(weak)),
                ("develop", Ok(json!({"required_status_checks":null}))),
            ],
        );
        assert_eq!(r["protection"]["main"], "incomplete");
        assert_eq!(r["protection"]["develop"], "incomplete");
        assert_eq!(r["pending"].as_array().unwrap().len(), 2);
    }
    #[test]
    fn public_repository_is_pending() {
        let mut m = meta("develop", false);
        m["private"] = json!(false);
        let r = audit_report(&m, &[("main", Ok(applied())), ("develop", Ok(applied()))]);
        assert_eq!(r["private"], false);
        assert!(r["pending"][0].as_str().unwrap().contains("private"));
    }
}
