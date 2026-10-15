use crate::util::*;
use anyhow::{Result, bail, ensure};
use serde_json::Value;
use std::path::Path;
pub fn confirmed(v: &Value, expected: &str) -> bool {
    match v {
        Value::Object(m) => {
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
                && active
                && installed
                && version == Some(expected))
                || m.values().any(|x| confirmed(x, expected))
        }
        Value::Array(a) => a.iter().any(|x| confirmed(x, expected)),
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
                confirmed(&data, string(&expected, "expected_version")?),
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
    #[test]
    fn plugin_unknown_or_disabled_is_not_installed() {
        assert!(!confirmed(
            &json!({"name":"superpowers","version":"7.0.0"}),
            "7.0.0"
        ));
        assert!(!confirmed(
            &json!({"name":"superpowers","version":"7.0.0","enabled":false}),
            "7.0.0"
        ));
        assert!(confirmed(
            &json!({"plugins":[{"name":"superpowers@claude-plugins-official","version":"7.0.0","enabled":true}]}),
            "7.0.0"
        ));
        assert!(!confirmed(
            &json!({"name":"superpowers","version":"8.0.0","enabled":true}),
            "7.0.0"
        ));
    }
}
