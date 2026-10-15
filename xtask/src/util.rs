use anyhow::{Context, Result, bail, ensure};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
use walkdir::WalkDir;

/// Convert native paths only when passing them as external-tool arguments.
pub fn command_path(path: &Path) -> Result<String> {
    normalize_command_path(
        path.to_str().context("External-tool path is not UTF-8")?,
        cfg!(windows),
    )
}
fn normalize_command_path(path: &str, windows: bool) -> Result<String> {
    if !windows {
        return Ok(path.to_owned());
    }
    let Some(rest) = path.strip_prefix(r"\\?\") else {
        return Ok(path.to_owned());
    };
    let normal = if rest
        .get(..4)
        .is_some_and(|s| s.eq_ignore_ascii_case("UNC\\"))
    {
        let share = &rest[4..];
        let mut parts = share.split('\\');
        ensure!(
            parts.next().is_some_and(|s| !s.is_empty())
                && parts.next().is_some_and(|s| !s.is_empty()),
            "Invalid verbatim UNC path"
        );
        format!(r"\\{share}")
    } else {
        let bytes = rest.as_bytes();
        ensure!(
            bytes.len() >= 3
                && bytes[0].is_ascii_alphabetic()
                && bytes[1] == b':'
                && bytes[2] == b'\\',
            "Unsupported Windows device path for external tool"
        );
        rest.to_owned()
    };
    ensure!(
        !normal.split('\\').any(|part| part.ends_with([' ', '.'])),
        "Windows path with trailing dot/space cannot be simplified safely"
    );
    Ok(normal)
}

pub fn read_json(path: impl AsRef<Path>) -> Result<Value> {
    let p = path.as_ref();
    serde_json::from_slice(&fs::read(p).with_context(|| format!("Read {}", p.display()))?)
        .with_context(|| format!("Parse {}", p.display()))
}
pub fn write_json(path: impl AsRef<Path>, value: &Value) -> Result<()> {
    fs::write(path, format!("{}\n", serde_json::to_string_pretty(value)?))?;
    Ok(())
}
pub fn text(path: impl AsRef<Path>) -> Result<String> {
    Ok(fs::read_to_string(path)?)
}
pub fn run(root: &Path, exe: &str, args: &[&str]) -> Result<()> {
    ensure!(
        Command::new(exe)
            .args(args)
            .current_dir(root)
            .status()
            .with_context(|| format!("Start {exe}"))?
            .success(),
        "Command failed: {exe}"
    );
    Ok(())
}
pub fn capture(root: &Path, exe: &str, args: &[&str]) -> Result<String> {
    let o = Command::new(exe)
        .args(args)
        .current_dir(root)
        .output()
        .with_context(|| format!("Start {exe}"))?;
    ensure!(
        o.status.success(),
        "Command failed: {exe}: {}",
        String::from_utf8_lossy(&o.stderr)
    );
    Ok(String::from_utf8(o.stdout)?.trim().to_owned())
}
pub fn secret_call(
    root: &Path,
    exe: &str,
    args: &[&str],
    envs: &[(String, String)],
) -> Result<String> {
    let o = Command::new(exe)
        .args(args)
        .envs(envs.iter().cloned())
        .current_dir(root)
        .output()
        .with_context(|| format!("Start signing tool {exe}"))?;
    ensure!(
        o.status.success(),
        "Signing command failed: {exe}; sensitive output suppressed"
    );
    Ok(String::from_utf8(o.stdout)?.trim().to_owned())
}
pub fn git(root: &Path, args: &[&str]) -> Result<String> {
    capture(root, "git", args)
}
pub fn sha(path: &Path) -> Result<String> {
    Ok(format!("{:x}", Sha256::digest(fs::read(path)?)))
}
pub fn files(root: &Path) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    for item in WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| !excluded(e.path().file_name().and_then(|x| x.to_str()).unwrap_or("")))
    {
        let e = item?;
        if e.file_type().is_file() {
            out.push(e.into_path());
        }
    }
    out.sort();
    Ok(out)
}
pub fn excluded(name: &str) -> bool {
    let name = name.to_ascii_lowercase();
    matches!(
        name.as_str(),
        ".git"
            | ".worktrees"
            | ".superpowers"
            | "target"
            | "dist"
            | "node_modules"
            | ".gradle"
            | "__pycache__"
            | "build"
            | ".aws"
            | ".ssh"
            | "auth.json"
            | "settings.local.json"
            | ".claude.json"
            | "credentials"
    ) || name.starts_with(".env")
        || [".pfx", ".p12", ".jks", ".p8", ".key", ".pem", ".keystore"]
            .iter()
            .any(|ext| name.ends_with(ext))
}
pub fn safe_existing(root: &Path, rel: &str) -> Result<PathBuf> {
    let path = root.join(rel).canonicalize()?;
    ensure!(
        path.starts_with(root.canonicalize()?),
        "Unsafe path outside project"
    );
    Ok(path)
}
pub fn copy_file(from: &Path, to: &Path) -> Result<()> {
    ensure!(
        !fs::symlink_metadata(from)?.file_type().is_symlink(),
        "Symlink source refused"
    );
    if let Some(p) = to.parent() {
        fs::create_dir_all(p)?;
    }
    if to.exists() {
        ensure!(
            !fs::symlink_metadata(to)?.file_type().is_symlink(),
            "Symlink destination refused"
        );
    }
    fs::copy(from, to)?;
    Ok(())
}
pub fn validate_destination(root: &Path, rel: &Path) -> Result<()> {
    ensure!(!rel.is_absolute(), "Absolute copy destination refused");
    let boundary = root.canonicalize()?;
    let mut path = root.to_path_buf();
    for component in rel.components() {
        ensure!(
            matches!(component, std::path::Component::Normal(_)),
            "Unsafe destination component"
        );
        path.push(component);
        match fs::symlink_metadata(&path) {
            Ok(meta) => {
                ensure!(
                    !meta.file_type().is_symlink(),
                    "Symlink destination refused"
                );
                ensure!(
                    path.canonicalize()?.starts_with(&boundary),
                    "Destination escapes project"
                );
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}
pub fn copy_tree(from: &Path, to: &Path) -> Result<()> {
    fs::create_dir_all(to)?;
    let paths = files(from)?;
    for p in &paths {
        validate_destination(to, p.strip_prefix(from)?)?;
    }
    for p in paths {
        copy_file(&p, &to.join(p.strip_prefix(from)?))?;
    }
    Ok(())
}
pub fn make_executable(path: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o755))?;
    }
    #[cfg(not(unix))]
    let _ = path;
    Ok(())
}
pub fn private_file(path: &Path, bytes: &[u8]) -> Result<()> {
    #[cfg(unix)]
    {
        use std::{io::Write, os::unix::fs::OpenOptionsExt};
        let mut f = fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .mode(0o600)
            .open(path)?;
        f.write_all(bytes)?;
    }
    #[cfg(not(unix))]
    {
        fs::write(path, bytes)?;
    }
    Ok(())
}
pub fn required(name: &str) -> Result<String> {
    let v = std::env::var(name).unwrap_or_default();
    ensure!(!v.is_empty(), "Missing configuration: {name}");
    Ok(v)
}
pub fn string<'a>(v: &'a Value, key: &str) -> Result<&'a str> {
    v[key]
        .as_str()
        .with_context(|| format!("Missing string: {key}"))
}
pub fn validate_version(v: &str) -> Result<()> {
    let parsed = semver::Version::parse(v)?;
    ensure!(
        parsed.build.is_empty(),
        "Release version must not contain build metadata"
    );
    Ok(())
}
pub fn check_host(platform: &str) -> Result<()> {
    let valid = match platform {
        "windows-x64" => cfg!(windows),
        "linux-x64" => cfg!(target_os = "linux"),
        "macos-universal" => cfg!(target_os = "macos"),
        "android-arm64" => true,
        _ => false,
    };
    if !valid {
        bail!("Platform must be built on its native runner");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn distribution_scan_excludes_synthetic_credentials() {
        let temp = tempfile::tempdir().unwrap();
        for name in [
            "ignored.pfx",
            "release.jks",
            "private.p12",
            "notary.p8",
            "secret.key",
            ".env",
            ".env.local",
            "auth.json",
            "settings.local.json",
        ] {
            fs::write(temp.path().join(name), b"SYNTHETIC TEST DATA").unwrap();
        }
        fs::create_dir_all(temp.path().join(".aws")).unwrap();
        fs::write(temp.path().join(".aws/credentials"), b"SYNTHETIC").unwrap();
        fs::write(temp.path().join("update.pub"), b"public fixture").unwrap();
        let paths = files(temp.path()).unwrap();
        assert_eq!(paths, vec![temp.path().join("update.pub")]);
    }
    #[cfg(unix)]
    #[test]
    fn copy_tree_rejects_symlink_destination_parent() {
        let t = tempfile::tempdir().unwrap();
        let source = t.path().join("source");
        let target = t.path().join("target");
        let outside = t.path().join("outside");
        fs::create_dir_all(source.join("config")).unwrap();
        fs::create_dir_all(&target).unwrap();
        fs::create_dir_all(&outside).unwrap();
        fs::write(source.join("config/value.json"), b"fixture").unwrap();
        std::os::unix::fs::symlink(&outside, target.join("config")).unwrap();
        assert!(copy_tree(&source, &target).is_err());
        assert!(!outside.join("value.json").exists());
    }
}

#[cfg(test)]
mod windows_command_path_tests {
    use super::*;
    #[test]
    fn git_arguments_use_normal_drive_and_unc_paths() {
        assert_eq!(
            normalize_command_path(r"\\?\D:\Projects\vynamp-git", true).unwrap(),
            r"D:\Projects\vynamp-git"
        );
        assert_eq!(
            normalize_command_path(r"\\?\UNC\server\share\Vynamp música", true).unwrap(),
            r"\\server\share\Vynamp música"
        );
        assert_eq!(
            normalize_command_path(r"D:\Projects\name with spaces", true).unwrap(),
            r"D:\Projects\name with spaces"
        );
        assert_eq!(
            normalize_command_path(r"../name with spaces", true).unwrap(),
            r"../name with spaces"
        );
    }
    #[test]
    fn unix_paths_and_unsupported_windows_namespaces_are_not_reinterpreted() {
        let literal = r"\\?\D:\Projects\literal";
        assert_eq!(normalize_command_path(literal, false).unwrap(), literal);
        assert!(normalize_command_path(r"\\?\GLOBALROOT\Device\HarddiskVolume1", true).is_err());
        assert!(normalize_command_path(r"\\?\D:\name.\child", true).is_err());
    }
    #[cfg(windows)]
    #[test]
    fn native_git_clone_accepts_canonical_windows_destination() {
        let temp = tempfile::tempdir().unwrap();
        let source = temp.path().join("empty source with spaces");
        fs::create_dir(&source).unwrap();
        run(
            temp.path(),
            "git",
            &["init", "--bare", &command_path(&source).unwrap()],
        )
        .unwrap();
        let target = temp
            .path()
            .canonicalize()
            .unwrap()
            .join("Vynamp clone with spaces");
        let argument = command_path(&target).unwrap();
        assert!(!argument.starts_with(r"\\?\"));
        run(
            temp.path(),
            "git",
            &[
                "clone",
                &command_path(&source.canonicalize().unwrap()).unwrap(),
                &argument,
            ],
        )
        .unwrap();
        assert!(target.join(".git").is_dir());
    }
}
