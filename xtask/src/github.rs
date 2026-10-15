use crate::{ci, util::*};
use anyhow::{Result, ensure};
use serde_json::Value;
use std::{fs, path::Path};
fn available_import_branch(refs: &str) -> String {
    let base = "chore/task-e15-01-codex-project-bootstrap";
    let mut name = base.to_owned();
    let mut suffix = 2u64;
    while refs
        .lines()
        .any(|line| line.split_whitespace().last() == Some(format!("refs/heads/{name}").as_str()))
    {
        name = format!("{base}-{suffix}");
        suffix += 1;
    }
    name
}

pub fn import(root: &Path, destination: &Path, push: bool) -> Result<()> {
    let target = if destination.is_absolute() {
        destination.to_path_buf()
    } else {
        root.join(destination)
    };
    ensure!(!target.exists(), "Destination exists; choose a new folder");
    let parent = target.parent().unwrap().canonicalize()?;
    let target = parent.join(target.file_name().unwrap());
    ensure!(
        !target.starts_with(root.canonicalize()?),
        "Destination must be outside source package"
    );
    let cfg = read_json(root.join("config/github.json"))?;
    let repo = string(&cfg, "repository")?;
    let url = string(&cfg, "url")?;
    let meta: Value =
        serde_json::from_str(&capture(root, "gh", &["api", &format!("repos/{repo}")])?)?;
    ensure!(meta["private"] == true, "Destination not confirmed private");
    let refs = capture(root, "git", &["ls-remote", "--heads", url])?;
    run(root, "git", &["clone", url, &command_path(&target)?])?;
    let empty = refs.is_empty();
    let base = if refs.lines().any(|l| l.ends_with("refs/heads/develop")) {
        "develop"
    } else {
        string(&meta, "default_branch")?
    };
    let branch = available_import_branch(&refs);
    if empty {
        run(&target, "git", &["symbolic-ref", "HEAD", "refs/heads/main"])?;
    } else {
        run(
            &target,
            "git",
            &["switch", "-c", &branch, &format!("origin/{base}")],
        )?;
    }
    copy_distribution(root, &target)?;
    for h in ["pre-commit", "pre-push"] {
        make_executable(&target.join(".githooks").join(h))?;
    }
    ci::docs(&target)?;
    run(&target, "git", &["config", "core.hooksPath", ".githooks"])?;
    run(&target, "git", &["add", "--all"])?;
    run(
        &target,
        "git",
        &[
            "commit",
            "-m",
            "chore: configure Vynamp Rust tooling and delivery",
        ],
    )?;
    if empty {
        run(&target, "git", &["branch", "develop"])?;
        if push {
            run(&target, "git", &["push", "-u", "origin", "main"])?;
            run(&target, "git", &["push", "-u", "origin", "develop"])?;
        }
    } else if push {
        run(&target, "git", &["push", "-u", "origin", &branch])?;
        let body = target.join(".superpowers/import-pr.md");
        fs::create_dir_all(body.parent().unwrap())?;
        fs::write(
            &body,
            "Migra automação para Rust/cargo xtask; preserva histórico, agentes, Gitflow, gates e contratos multiplataforma.\n\nValidação: cargo xtask ci docs. Player ainda não implementado; release permanece bloqueada.\n",
        )?;
        run(
            &target,
            "gh",
            &[
                "pr",
                "create",
                "--repo",
                repo,
                "--base",
                base,
                "--head",
                &branch,
                "--title",
                "chore: configure Vynamp Rust development tooling",
                "--body-file",
                &command_path(&body)?,
            ],
        )?;
    }
    println!(
        "Import prepared at {}. {}",
        target.display(),
        if push {
            "Pushed; inspect PR/bootstrap and activate protections."
        } else {
            "No push."
        }
    );
    Ok(())
}

fn copy_distribution(root: &Path, target: &Path) -> Result<()> {
    const SCRIPTS: &[&str] = &[
        "agents.py",
        "ci.py",
        "github_import.py",
        "guard_branch.py",
        "package.py",
        "quality_gate.py",
        "release.py",
        "repo.py",
        "signing.py",
        "validate_delivery.py",
        "validate_spec.py",
        "requirements.txt",
    ];
    const TESTS: &[&str] = &["test_repo_workflow.py", "test_release_gate.py"];
    let paths = SCRIPTS
        .iter()
        .map(|p| std::path::PathBuf::from("scripts").join(p))
        .chain(
            TESTS
                .iter()
                .map(|p| std::path::PathBuf::from("tests/tooling").join(p)),
        )
        .collect::<Vec<_>>();
    let mut paths = paths;
    paths.extend(
        [
            "packaging/linux/auralis.desktop",
            "packaging/icons/auralis.svg",
        ]
        .iter()
        .map(std::path::PathBuf::from),
    );
    for p in &paths {
        validate_destination(target, p)?;
    }
    validate_destination(target, Path::new("scripts/README.md"))?;
    for p in files(root)? {
        validate_destination(target, p.strip_prefix(root)?)?;
    }
    // Validate every cleanup/copy destination before the first deletion.
    for p in paths {
        let p = target.join(p);
        if p.is_file() {
            fs::remove_file(p)?;
        }
    }
    let readme = target.join("scripts/README.md");
    if readme.is_file() && text(&readme)?.starts_with("# Validação e manutenção do pacote") {
        fs::remove_file(readme)?;
    }
    copy_tree(root, target)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn overlay_removes_legacy_readme_and_preserves_unrelated_files() {
        let tmp = tempfile::tempdir().unwrap();
        let source = tmp.path().join("source");
        let target = tmp.path().join("target");
        fs::create_dir_all(source.join("xtask")).unwrap();
        fs::create_dir_all(target.join("scripts")).unwrap();
        fs::write(source.join("xtask/README.md"), "# Rust tooling\n").unwrap();
        fs::write(
            target.join("scripts/README.md"),
            "# Validação e manutenção do pacote\nLegacy tooling\n",
        )
        .unwrap();
        fs::write(target.join("scripts/repo.py"), "legacy").unwrap();
        fs::write(target.join("unrelated.txt"), "preserve").unwrap();
        copy_distribution(&source, &target).unwrap();
        assert!(!target.join("scripts/repo.py").exists());
        assert!(!target.join("scripts/README.md").exists());
        assert_eq!(
            fs::read_to_string(target.join("unrelated.txt")).unwrap(),
            "preserve"
        );
        assert!(target.join("xtask/README.md").is_file());
    }
}
#[cfg(all(test, unix))]
mod symlink_tests {
    use super::*;
    #[test]
    fn legacy_cleanup_never_deletes_outside_checkout() {
        for rel in ["scripts", "tests/tooling"] {
            let tmp = tempfile::tempdir().unwrap();
            let source = tmp.path().join("source");
            let target = tmp.path().join("target");
            let outside = tmp.path().join("outside");
            fs::create_dir_all(&source).unwrap();
            fs::create_dir_all(&target).unwrap();
            fs::create_dir_all(&outside).unwrap();
            let filename = if rel == "scripts" {
                "repo.py"
            } else {
                "test_repo_workflow.py"
            };
            let precious = outside.join(filename);
            fs::write(&precious, b"SYNTHETIC USER FILE").unwrap();
            fs::create_dir_all(target.join(rel).parent().unwrap()).unwrap();
            std::os::unix::fs::symlink(&outside, target.join(rel)).unwrap();
            assert!(copy_distribution(&source, &target).is_err());
            assert_eq!(fs::read(&precious).unwrap(), b"SYNTHETIC USER FILE");
        }
    }
}
#[cfg(test)]
mod brand_migration_tests {
    use super::*;
    #[test]
    fn overlay_removes_previous_brand_templates() {
        let tmp = tempfile::tempdir().unwrap();
        let source = tmp.path().join("source");
        let target = tmp.path().join("target");
        fs::create_dir_all(&source).unwrap();
        fs::create_dir_all(target.join("packaging/linux")).unwrap();
        fs::create_dir_all(target.join("packaging/icons")).unwrap();
        fs::write(
            target.join("packaging/linux/auralis.desktop"),
            "[Desktop Entry]\nName=Auralis\n",
        )
        .unwrap();
        fs::write(target.join("packaging/icons/auralis.svg"), "<svg/>\n").unwrap();
        copy_distribution(&source, &target).unwrap();
        assert!(!target.join("packaging/linux/auralis.desktop").exists());
        assert!(!target.join("packaging/icons/auralis.svg").exists());
    }
}

#[cfg(test)]
mod import_branch_tests {
    use super::*;
    #[test]
    fn existing_remote_bootstrap_branches_are_preserved() {
        let base = "chore/task-e15-01-codex-project-bootstrap";
        let refs = format!("abc\trefs/heads/{base}\ndef\trefs/heads/{base}-2\n");
        assert_eq!(available_import_branch(&refs), format!("{base}-3"));
        assert_eq!(available_import_branch("abc\trefs/heads/main\n"), base);
    }
}
