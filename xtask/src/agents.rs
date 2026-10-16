use crate::util::*;
use anyhow::{Context, Result, bail, ensure};
use serde_json::Value;
use std::path::Path;
fn same_path(a: &str, b: &Path) -> bool {
    let norm = |s: &str| {
        let s = s.strip_prefix(r"\\?\").unwrap_or(s).replace('/', "\\");
        let s = s.trim_end_matches('\\');
        if cfg!(windows) {
            s.to_lowercase()
        } else {
            s.to_owned()
        }
    };
    norm(a) == norm(&b.to_string_lossy())
}
pub fn expected_version<'a>(lock: &'a Value, agent: &str) -> Result<&'a str> {
    lock["expected_versions"][agent]
        .as_str()
        .with_context(|| format!("Lock has no expected_versions.{agent}"))
}
pub fn confirmed(v: &Value, expected: &str, root: &Path) -> bool {
    match v {
        Value::Object(m) => {
            // Claude lists project-scoped installs of every project; only this root counts.
            let applies = m
                .get("projectPath")
                .and_then(Value::as_str)
                .is_none_or(|p| same_path(p, root));
            let identity = m
                .get("name")
                .or_else(|| m.get("id"))
                .or_else(|| m.get("pluginId"))
                .and_then(Value::as_str)
                .unwrap_or("");
            let active = m.get("enabled") == Some(&Value::Bool(true))
                || m.get("status")
                    .and_then(Value::as_str)
                    .is_some_and(|s| matches!(s.to_lowercase().as_str(), "active" | "enabled"));
            let installed =
                m.get("installed").or_else(|| m.get("isInstalled")) != Some(&Value::Bool(false));
            let version = m
                .get("version")
                .or_else(|| m.get("installedVersion"))
                .and_then(Value::as_str);
            ((identity == "superpowers" || identity.starts_with("superpowers@"))
                && applies
                && active
                && installed
                && version == Some(expected))
                || m.values().any(|x| confirmed(x, expected, root))
        }
        Value::Array(a) => a.iter().any(|x| confirmed(x, expected, root)),
        _ => false,
    }
}
pub fn doctor(root: &Path, agent: &str) -> Result<()> {
    let expected = read_json(root.join("config/superpowers.lock.json"))?;
    let mut failures = vec![];
    for a in if agent == "both" {
        vec!["codex", "claude"]
    } else {
        vec![agent]
    } {
        let result = (|| -> Result<()> {
            let output = capture(root, a, &["plugin", "list", "--json"])?;
            let data: Value = serde_json::from_str(&output)?;
            ensure!(
                confirmed(&data, expected_version(&expected, a)?, root),
                "{a}: Superpowers active/version not confirmed; verify native /plugins (Codex) or /plugin (Claude); update parser/lock from real output if needed"
            );
            println!(
                "{a}: plugin manager confirmed Superpowers; confirm skill discovery in session"
            );
            Ok(())
        })();
        if let Err(e) = result {
            failures.push(format!("{a}: {e}"));
        }
    }
    ensure!(failures.is_empty(), "{}", failures.join("\n"));
    Ok(())
}
pub fn install(root: &Path, agent: &str) -> Result<()> {
    if agent == "claude" || agent == "both" {
        run(
            root,
            "claude",
            &[
                "plugin",
                "install",
                "superpowers@claude-plugins-official",
                "--scope",
                "project",
            ],
        )?;
    }
    if agent == "codex" || agent == "both" {
        bail!(
            "Codex: open codex, use /plugins, search superpowers and Install Plugin. Then restart and run cargo xtask doctor. No installation is being claimed."
        );
    }
    doctor(root, agent)
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    const ROOT: &str = r"D:\Projects\vynamp-media-player\repo\.worktrees\claude\x";
    #[test]
    fn plugin_unknown_or_disabled_is_not_installed() {
        let root = Path::new(ROOT);
        assert!(!confirmed(
            &json!({"name":"superpowers","version":"7.0.0"}),
            "7.0.0",
            root
        ));
        assert!(!confirmed(
            &json!({"name":"superpowers","version":"7.0.0","enabled":false}),
            "7.0.0",
            root
        ));
        assert!(confirmed(
            &json!({"plugins":[{"name":"superpowers@claude-plugins-official","version":"7.0.0","enabled":true}]}),
            "7.0.0",
            root
        ));
        assert!(!confirmed(
            &json!({"name":"superpowers","version":"8.0.0","enabled":true}),
            "7.0.0",
            root
        ));
    }
    // Shapes captured from `codex plugin list --json` (codex-cli 0.162.1) and
    // `claude plugin list --json` on Windows, 2026-10-15.
    #[test]
    fn real_codex_manager_output_is_confirmed() {
        let out = json!({"installed":[
            {"pluginId":"pdf@openai-primary-runtime","name":"pdf","version":"26.1013.11914","installed":true,"enabled":true},
            {"pluginId":"superpowers@openai-curated-remote","name":"superpowers","marketplaceName":"openai-curated-remote","version":"6.4.2","installed":true,"enabled":true}
        ],"available":[]});
        assert!(confirmed(&out, "6.4.2", Path::new(ROOT)));
        assert!(!confirmed(&out, "6.4.1", Path::new(ROOT)));
    }
    #[test]
    fn claude_entry_of_another_project_does_not_count() {
        let other = json!([{"id":"superpowers@claude-plugins-official","version":"6.4.1","scope":"project","enabled":true,"projectPath":"D:\\Projects\\vigesima","projectEnabled":true}]);
        assert!(!confirmed(&other, "6.4.1", Path::new(ROOT)));
        let user = json!([{"id":"superpowers@claude-plugins-official","version":"6.4.1","scope":"user","enabled":true,"projectEnabled":true}]);
        assert!(confirmed(&user, "6.4.1", Path::new(ROOT)));
        let here = json!([{"id":"superpowers@claude-plugins-official","version":"6.4.1","scope":"project","enabled":true,"projectPath":ROOT}]);
        assert!(confirmed(&here, "6.4.1", Path::new(ROOT)));
        let lower = json!([{"id":"superpowers@claude-plugins-official","version":"6.4.1","scope":"project","enabled":true,"projectPath":ROOT.to_lowercase()}]);
        // Windows accepts case aliases and extended paths; Unix must not accept
        // a differently cased project as the current project.
        #[cfg(windows)]
        assert!(confirmed(
            &lower,
            "6.4.1",
            Path::new(&format!(r"\\?\{ROOT}"))
        ));
        #[cfg(not(windows))]
        assert!(!confirmed(&lower, "6.4.1", Path::new(ROOT)));
    }
    #[test]
    fn lock_pins_each_agent_separately() {
        let lock = json!({"expected_versions":{"codex":"6.4.2","claude":"6.4.1"}});
        assert_eq!(expected_version(&lock, "codex").unwrap(), "6.4.2");
        assert_eq!(expected_version(&lock, "claude").unwrap(), "6.4.1");
        assert!(expected_version(&lock, "gemini").is_err());
    }
}
