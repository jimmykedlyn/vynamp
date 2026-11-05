use crate::{package, util::*};
use anyhow::{Context, Result, ensure};
use base64::Engine;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, io::Write, path::Path};
pub const PLATFORMS: [&str; 5] = [
    "windows-x64",
    "linux-x64",
    "macos-intel",
    "macos-silicon",
    "android-arm64",
];
pub const SUFFIXES: [&str; 8] = [
    "windows-x64-setup.exe",
    "windows-x64-portable.zip",
    "linux-x64.deb",
    "linux-x64.AppImage",
    "macos-universal.dmg",
    "macos-universal.zip",
    "android-arm64.apk",
    "android-arm64.aab",
];
pub fn source_digest(root: &Path) -> Result<String> {
    let mut digest = Sha256::new();
    let mut list = vec![];
    for group in [
        "crates",
        "assets",
        "apps",
        "packaging",
        "xtask",
        "config",
        ".github",
        ".cargo",
        ".githooks",
    ] {
        let dir = root.join(group);
        if dir.exists() {
            list.extend(files(&dir)?);
        }
    }
    for name in ["Cargo.toml", "Cargo.lock", "rust-toolchain.toml"] {
        let p = root.join(name);
        if p.is_file() {
            list.push(p);
        }
    }
    list.sort();
    for p in list {
        if p == root.join("config/project.json")
            || p == root.join("packaging/icons/export-manifest.json")
        {
            continue;
        }
        digest.update(
            p.strip_prefix(root)?
                .to_string_lossy()
                .replace('\\', "/")
                .as_bytes(),
        );
        digest.update(Sha256::digest(fs::read(p)?));
    }
    let mut cfg = read_json(root.join("config/project.json"))?;
    let obj = cfg.as_object_mut().context("Project config")?;
    obj.remove("release_enabled");
    obj.remove("application_ready");
    obj.remove("branding_exports_ready");
    digest.update(serde_json::to_vec(&cfg)?);
    Ok(format!("{:x}", digest.finalize()))
}
pub fn preflight(root: &Path, version: &str) -> Result<Value> {
    let cfg = read_json(root.join("config/project.json"))?;
    ensure!(
        cfg["release_enabled"] == true && cfg["application_ready"] == true,
        "Release disabled: actual player/build/hardware/signing acceptance missing"
    );
    crate::branding::exports(root)?;
    validate_version(version)?;
    ensure!(
        cfg["product_version"] == version,
        "Tag/product version mismatch"
    );
    ensure!(
        crate::ci::detect(root)?["core"] == "true",
        "Actual desktop player absent"
    );
    for name in [
        "Cargo.toml",
        "Cargo.lock",
        "rust-toolchain.toml",
        "LICENSE",
        "THIRD_PARTY_NOTICES",
        "apps/android/gradlew",
        "apps/android/gradlew.bat",
        "packaging/update.pub",
    ] {
        ensure!(
            root.join(name).is_file(),
            "Missing release prerequisite: {name}"
        );
    }
    let e = read_json(root.join(format!("docs/evidence/release/{version}.json")))?;
    ensure!(
        e["source_digest"] == source_digest(root)?,
        "Evidence source digest mismatch"
    );
    let reviewers: BTreeSet<_> = e["reviewers"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect();
    ensure!(
        reviewers == BTreeSet::from(["claude"]),
        "Independent reviews missing"
    );
    for platform in PLATFORMS {
        let row = &e["platforms"][platform];
        ensure!(
            row["status"] == "passed" && row["hardware"].as_str().is_some_and(|x| !x.is_empty()),
            "Missing real hardware evidence: {platform}"
        );
        let report = safe_existing(root, string(row, "report")?)?;
        ensure!(
            report.is_file() && row["sha256"] == sha(&report)?,
            "Missing report or evidence hash mismatch"
        );
    }
    if release_tag(
        std::env::var("GITHUB_ACTIONS").as_deref() == Ok("true"),
        std::env::var("GITHUB_REF").ok().as_deref(),
        version,
    )? {
        run(
            root,
            "git",
            &["merge-base", "--is-ancestor", "HEAD", "origin/main"],
        )?;
    }
    Ok(cfg)
}
pub fn artifact_manifest(dir: &Path, version: &str) -> Result<Value> {
    validate_version(version)?;
    let wanted: BTreeSet<_> = SUFFIXES
        .iter()
        .map(|s| format!("vynamp-{version}-{s}"))
        .collect();
    let mut rows = vec![];
    let mut found = BTreeSet::new();
    for item in fs::read_dir(dir)? {
        let p = item?.path();
        let name = p
            .file_name()
            .context("File name")?
            .to_string_lossy()
            .into_owned();
        let ext = p
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();
        if matches!(
            ext.as_str(),
            "exe" | "zip" | "dmg" | "deb" | "appimage" | "apk" | "aab"
        ) {
            let meta = fs::symlink_metadata(&p)?;
            ensure!(
                meta.is_file() && !meta.file_type().is_symlink() && meta.len() > 0,
                "Unsafe/empty artifact"
            );
            found.insert(name.clone());
            rows.push(json!({"file":name,"bytes":meta.len(),"sha256":sha(&p)?}));
        }
    }
    ensure!(
        found == wanted,
        "Missing/unexpected platform artifact set; publication refused"
    );
    rows.sort_by_key(|r| r["file"].as_str().unwrap().to_owned());
    let sums = rows
        .iter()
        .map(|r| {
            format!(
                "{}  {}\n",
                r["sha256"].as_str().unwrap(),
                r["file"].as_str().unwrap()
            )
        })
        .collect::<String>();
    fs::write(dir.join("SHA256SUMS"), sums)?;
    let result = json!({"product_version":version,"artifacts":rows});
    write_json(dir.join("manifest.json"), &result)?;
    Ok(result)
}
pub fn command(
    root: &Path,
    action: &str,
    version: Option<&str>,
    platform: Option<&str>,
) -> Result<()> {
    if action == "digest" {
        println!("{}", source_digest(root)?);
        return Ok(());
    }
    let version = version.context("--version required")?;
    preflight(root, version)?;
    match action {
        "preflight" => println!("Preflight passed"),
        "build" => package::build(root, platform.context("--platform required")?, version)?,
        "assemble" => {
            let dir = root.join("dist/release");
            fs::create_dir_all(&dir)?;
            let repo = read_json(root.join("config/github.json"))?;
            run(
                root,
                "gh",
                &[
                    "run",
                    "download",
                    &required("GITHUB_RUN_ID")?,
                    "--repo",
                    string(&repo, "repository")?,
                    "--pattern",
                    "release-*",
                    "--dir",
                    dir.to_str().unwrap(),
                ],
            )?;
            for e in fs::read_dir(&dir)? {
                let p = e?.path();
                ensure!(
                    !fs::symlink_metadata(&p)?.file_type().is_symlink(),
                    "Symlink downloaded artifact"
                );
                if p.is_dir() {
                    for item in fs::read_dir(&p)? {
                        let f = item?.path();
                        ensure!(
                            f.is_file() && !fs::symlink_metadata(&f)?.file_type().is_symlink(),
                            "Unexpected artifact directory"
                        );
                        let dst = dir.join(f.file_name().unwrap());
                        ensure!(!dst.exists(), "Duplicate artifact name");
                        fs::rename(f, dst)?;
                    }
                    fs::remove_dir(p)?;
                }
            }
            artifact_manifest(&dir, version)?;
        }
        "publish-draft" => {
            let dir = root.join("dist/release");
            artifact_manifest(&dir, version)?;
            let tmp = tempfile::tempdir()?;
            let key = tmp.path().join("signing.key");
            private_file(
                &key,
                &base64::engine::general_purpose::STANDARD
                    .decode(required("MINISIGN_SECRET_KEY_BASE64")?)?,
            )?;
            let manifest = dir.join("manifest.json");
            let sig = dir.join("manifest.json.minisig");
            secret_call(
                root,
                "minisign",
                &[
                    "-S",
                    "-s",
                    key.to_str().unwrap(),
                    "-m",
                    manifest.to_str().unwrap(),
                    "-x",
                    sig.to_str().unwrap(),
                ],
                &[],
            )?;
            run(
                root,
                "minisign",
                &[
                    "-V",
                    "-p",
                    "packaging/update.pub",
                    "-m",
                    manifest.to_str().unwrap(),
                    "-x",
                    sig.to_str().unwrap(),
                ],
            )?;
            let repo = read_json(root.join("config/github.json"))?;
            let mut args = vec!["release".into(), "create".into(), format!("v{version}")];
            for p in files(&dir)? {
                args.push(p.to_string_lossy().into_owned());
            }
            args.extend([
                "--repo".into(),
                string(&repo, "repository")?.into(),
                "--verify-tag".into(),
                "--draft".into(),
                "--generate-notes".into(),
            ]);
            run(
                root,
                "gh",
                &args.iter().map(String::as_str).collect::<Vec<_>>(),
            )?;
        }
        _ => anyhow::bail!("Unknown release command"),
    };
    Ok(())
}
/// In GitHub Actions a release is checked against the tag that triggered the
/// run; outside CI the local preflight keeps working without a tag.
fn release_tag(actions: bool, reference: Option<&str>, version: &str) -> Result<bool> {
    if !actions {
        return Ok(false);
    }
    ensure!(
        reference == Some(format!("refs/tags/v{version}").as_str()),
        "Release requires matching v* tag"
    );
    Ok(true)
}
fn version_from_tag(name: &str) -> Result<String> {
    let v = name.strip_prefix('v').context("Expected v* tag")?;
    validate_version(v)?;
    Ok(v.to_owned())
}
pub fn tag_version() -> Result<()> {
    let name = required("GITHUB_REF_NAME")?;
    let v = version_from_tag(&name)?;
    let mut out = fs::OpenOptions::new()
        .append(true)
        .open(required("GITHUB_OUTPUT")?)?;
    writeln!(out, "version={v}")?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn manifest_requires_exact_nonempty_set() {
        let tmp = tempfile::tempdir().unwrap();
        assert!(artifact_manifest(tmp.path(), "0.1.0").is_err());
        for s in SUFFIXES {
            fs::write(
                tmp.path().join(format!("vynamp-0.1.0-{s}")),
                b"signed fixture, not player",
            )
            .unwrap();
        }
        assert!(artifact_manifest(tmp.path(), "0.1.0").is_ok());
        let p = tmp.path().join("vynamp-0.1.0-android-arm64.apk");
        fs::write(&p, b"").unwrap();
        assert!(artifact_manifest(tmp.path(), "0.1.0").is_err());
        fs::write(&p, b"fixture").unwrap();
        fs::write(tmp.path().join("unexpected.exe"), b"unexpected").unwrap();
        assert!(artifact_manifest(tmp.path(), "0.1.0").is_err());
    }
    #[test]
    fn version_comes_from_v_tag_only() {
        assert_eq!(version_from_tag("v0.1.0").unwrap(), "0.1.0");
        assert!(version_from_tag("0.1.0").is_err());
        assert!(version_from_tag("vbad").is_err());
    }
    #[test]
    fn github_actions_release_requires_matching_tag() {
        assert!(!release_tag(false, None, "0.1.0").unwrap());
        assert!(release_tag(true, Some("refs/tags/v0.1.0"), "0.1.0").unwrap());
        assert!(release_tag(true, Some("refs/tags/v0.2.0"), "0.1.0").is_err());
        assert!(release_tag(true, Some("refs/heads/develop"), "0.1.0").is_err());
        assert!(release_tag(true, None, "0.1.0").is_err());
    }
    #[test]
    fn digest_covers_xtask_and_ignores_activation_flags() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("config")).unwrap();
        fs::create_dir_all(tmp.path().join("xtask/src")).unwrap();
        let cfg = tmp.path().join("config/project.json");
        write_json(
            &cfg,
            &json!({"product_version":"0.1.0","release_enabled":false,"application_ready":false}),
        )
        .unwrap();
        fs::write(tmp.path().join("xtask/src/main.rs"), "first").unwrap();
        let a = source_digest(tmp.path()).unwrap();
        write_json(
            &cfg,
            &json!({"product_version":"0.1.0","release_enabled":true,"application_ready":true}),
        )
        .unwrap();
        assert_eq!(a, source_digest(tmp.path()).unwrap());
        fs::write(tmp.path().join("xtask/src/main.rs"), "changed").unwrap();
        assert_ne!(a, source_digest(tmp.path()).unwrap());
    }
}
