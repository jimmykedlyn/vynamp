use crate::{ci, release, signing::Signing, util::*};
use anyhow::{Context, Result, ensure};
use regex::Regex;
use serde_json::Value;
use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};
fn licenses(root: &Path, to: &Path) -> Result<()> {
    for n in ["LICENSE", "THIRD_PARTY_NOTICES"] {
        copy_file(&root.join(n), &to.join(n))?;
    }
    Ok(())
}
fn binary(root: &Path, target: &str, cfg: &Value) -> Result<PathBuf> {
    run(root, "rustup", &["target", "add", target])?;
    run(
        root,
        "cargo",
        &[
            "build",
            "--release",
            "--locked",
            "--package",
            string(cfg, "desktop_package")?,
            "--bin",
            string(cfg, "desktop_binary")?,
            "--target",
            target,
        ],
    )?;
    let p = root.join(format!(
        "target/{target}/release/{}{}",
        string(cfg, "desktop_binary")?,
        if target.contains("windows") {
            ".exe"
        } else {
            ""
        }
    ));
    ensure!(
        p.is_file() && fs::metadata(&p)?.len() > 0,
        "Real application binary not produced"
    );
    Ok(p)
}
fn dependencies(root: &Path, platform: &str, to: &Path) -> Result<()> {
    let cfg = read_json(root.join("packaging/runtime-files.json"))?;
    ensure!(
        cfg["audit_complete"] == true,
        "Runtime inventory audit incomplete"
    );
    let entries = cfg["files"][platform]
        .as_array()
        .context("Explicit platform runtime inventory required")?;
    for row in entries {
        let src = safe_existing(root, row[0].as_str().context("Runtime source")?)?;
        let rel = Path::new(row[1].as_str().context("Runtime destination")?);
        ensure!(
            !rel.is_absolute()
                && rel
                    .components()
                    .all(|x| matches!(x, std::path::Component::Normal(_))),
            "Unsafe runtime destination"
        );
        copy_file(&src, &to.join(rel))?;
    }
    Ok(())
}
fn zip_dir(dir: &Path, out: &Path) -> Result<()> {
    let mut zip = zip::ZipWriter::new(fs::File::create(out)?);
    for p in files(dir)? {
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);
        zip.start_file(
            p.strip_prefix(dir)?.to_string_lossy().replace('\\', "/"),
            options,
        )?;
        let mut buf = vec![];
        fs::File::open(&p)?.read_to_end(&mut buf)?;
        zip.write_all(&buf)?;
    }
    zip.finish()?;
    Ok(())
}
fn windows(
    root: &Path,
    version: &str,
    cfg: &Value,
    out: &Path,
    stage: &Path,
    s: &Signing,
) -> Result<()> {
    copy_file(
        &binary(root, string(&cfg["targets"], "windows-x64")?, cfg)?,
        &stage.join("vynamp.exe"),
    )?;
    licenses(root, stage)?;
    dependencies(root, "windows-x64", stage)?;
    for p in files(stage)? {
        if p.extension()
            .and_then(|s| s.to_str())
            .is_some_and(|s| matches!(s.to_lowercase().as_str(), "exe" | "dll"))
        {
            s.windows(&p)?;
        }
    }
    let tool = if std::process::Command::new("makensis")
        .arg("/VERSION")
        .output()
        .is_ok()
    {
        "makensis".into()
    } else {
        PathBuf::from(required("ProgramFiles(x86)")?)
            .join("NSIS/makensis.exe")
            .to_string_lossy()
            .into_owned()
    };
    let parsed = semver::Version::parse(version)?;
    let numeric = format!("{}.{}.{}.0", parsed.major, parsed.minor, parsed.patch);
    let exe = out.join(format!("vynamp-{version}-windows-x64-setup.exe"));
    run(
        root,
        &tool,
        &[
            &format!(
                "/DICON={}",
                root.join("packaging/icons/vynamp.ico").display()
            ),
            &format!("/DVERSION={version}"),
            &format!("/DNUMERIC_VERSION={numeric}"),
            &format!("/DSTAGING={}", stage.display()),
            &format!("/DOUTPUT={}", exe.display()),
            "packaging/windows/installer.nsi",
        ],
    )?;
    s.windows(&exe)?;
    zip_dir(
        stage,
        &out.join(format!("vynamp-{version}-windows-x64-portable.zip")),
    )
}
fn linux(root: &Path, version: &str, cfg: &Value, out: &Path, stage: &Path) -> Result<()> {
    let maintainer = required("VYNAMP_MAINTAINER")?;
    ensure!(
        !maintainer.contains(['\n', '\r']),
        "Invalid maintainer field"
    );
    let appdir = stage.join("Vynamp.AppDir");
    let bin = appdir.join("usr/bin/vynamp");
    copy_file(
        &binary(root, string(&cfg["targets"], "linux-x64")?, cfg)?,
        &bin,
    )?;
    make_executable(&bin)?;
    copy_file(
        &root.join("packaging/linux/vynamp.desktop"),
        &appdir.join("vynamp.desktop"),
    )?;
    copy_file(
        &root.join("packaging/icons/vynamp.svg"),
        &appdir.join("vynamp.svg"),
    )?;
    copy_file(&root.join("packaging/linux/AppRun"), &appdir.join("AppRun"))?;
    make_executable(&appdir.join("AppRun"))?;
    licenses(root, &appdir)?;
    dependencies(root, "linux-x64", &appdir)?;
    let tools = read_json(root.join("packaging/tools.lock.json"))?;
    let url = string(&tools["appimagetool"], "url")?;
    ensure!(
        url.starts_with("https://"),
        "AppImage tool requires HTTPS URL"
    );
    let hash = string(&tools["appimagetool"], "sha256")?;
    let tool = stage.join("appimagetool");
    run(
        root,
        "curl",
        &[
            "--fail",
            "--location",
            "--proto",
            "=https",
            "--output",
            tool.to_str().unwrap(),
            url,
        ],
    )?;
    ensure!(sha(&tool)? == hash, "AppImage tool integrity mismatch");
    make_executable(&tool)?;
    run(
        root,
        tool.to_str().unwrap(),
        &[
            "--appimage-extract-and-run",
            appdir.to_str().unwrap(),
            out.join(format!("vynamp-{version}-linux-x64.AppImage"))
                .to_str()
                .unwrap(),
        ],
    )?;
    let deb = stage.join("deb");
    fs::create_dir_all(deb.join("DEBIAN"))?;
    copy_tree(&appdir.join("usr"), &deb.join("usr"))?;
    copy_file(
        &appdir.join("vynamp.desktop"),
        &deb.join("usr/share/applications/vynamp.desktop"),
    )?;
    copy_file(
        &appdir.join("vynamp.svg"),
        &deb.join("usr/share/icons/hicolor/scalable/apps/vynamp.svg"),
    )?;
    licenses(root, &deb.join("usr/share/doc/vynamp"))?;
    fs::write(
        deb.join("DEBIAN/control"),
        format!(
            "Package: vynamp\nVersion: {}\nArchitecture: amd64\nMaintainer: {maintainer}\nSection: sound\nPriority: optional\nDepends: libc6 (>= 2.35), libasound2\nDescription: Independent Rust multimedia audio player\n",
            version.replace('-', "~")
        ),
    )?;
    run(
        root,
        "dpkg-deb",
        &[
            "--build",
            "--root-owner-group",
            deb.to_str().unwrap(),
            out.join(format!("vynamp-{version}-linux-x64.deb"))
                .to_str()
                .unwrap(),
        ],
    )
}
fn mac(
    root: &Path,
    version: &str,
    cfg: &Value,
    out: &Path,
    stage: &Path,
    s: &Signing,
) -> Result<()> {
    let app = stage.join("Vynamp.app");
    let macos = app.join("Contents/MacOS");
    fs::create_dir_all(&macos)?;
    let intel = binary(root, string(&cfg["targets"], "macos-intel")?, cfg)?;
    let arm = binary(root, string(&cfg["targets"], "macos-silicon")?, cfg)?;
    let exe = macos.join("vynamp");
    run(
        root,
        "lipo",
        &[
            "-create",
            intel.to_str().unwrap(),
            arm.to_str().unwrap(),
            "-output",
            exe.to_str().unwrap(),
        ],
    )?;
    run(
        root,
        "lipo",
        &["-verify_arch", "x86_64", "arm64", exe.to_str().unwrap()],
    )?;
    let v = semver::Version::parse(version)?;
    let numeric = format!("{}.{}.{}", v.major, v.minor, v.patch);
    fs::write(
        app.join("Contents/Info.plist"),
        text(root.join("packaging/macos/Info.plist"))?.replace("@VERSION@", &numeric),
    )?;
    let res = app.join("Contents/Resources");
    fs::create_dir_all(&res)?;
    copy_file(
        &root.join("packaging/icons/Vynamp.icns"),
        &res.join("Vynamp.icns"),
    )?;
    write_json(
        res.join("version.json"),
        &serde_json::json!({"version":version}),
    )?;
    licenses(root, &res)?;
    dependencies(root, "macos-universal", &app)?;
    let mut code = walkdir::WalkDir::new(&app)
        .into_iter()
        .collect::<std::result::Result<Vec<_>, _>>()?;
    code.sort_by_key(|e| std::cmp::Reverse(e.depth()));
    for e in code {
        let p = e.path();
        let ext = p.extension().and_then(|s| s.to_str()).unwrap_or("");
        if (e.file_type().is_file()
            && (matches!(ext, "dylib" | "so") || p.parent() == Some(macos.as_path())))
            || (e.file_type().is_dir() && ext == "framework")
        {
            s.mac(p)?;
        }
    }
    s.mac(&app)?;
    run(
        root,
        "codesign",
        &["--verify", "--deep", "--strict", app.to_str().unwrap()],
    )?;
    let zip = out.join(format!("vynamp-{version}-macos-universal.zip"));
    run(
        root,
        "ditto",
        &[
            "-c",
            "-k",
            "--keepParent",
            app.to_str().unwrap(),
            zip.to_str().unwrap(),
        ],
    )?;
    s.submit(&zip)?;
    s.staple(&app)?;
    run(
        root,
        "ditto",
        &[
            "-c",
            "-k",
            "--keepParent",
            app.to_str().unwrap(),
            zip.to_str().unwrap(),
        ],
    )?;
    let dmg = out.join(format!("vynamp-{version}-macos-universal.dmg"));
    run(
        root,
        "hdiutil",
        &[
            "create",
            "-volname",
            "Vynamp",
            "-srcfolder",
            stage.to_str().unwrap(),
            "-ov",
            "-format",
            "UDZO",
            dmg.to_str().unwrap(),
        ],
    )?;
    s.mac(&dmg)?;
    s.submit(&dmg)?;
    s.staple(&dmg)
}
fn fingerprint(report: &str) -> Result<String> {
    let re = Regex::new(r"(?:SHA256:|certificate SHA-256 digest:)\s*([A-Fa-f0-9:]{64,95})")?;
    let value = re
        .captures(report)
        .context("Cannot confirm Android signing fingerprint")?[1]
        .replace(':', "")
        .to_lowercase();
    ensure!(value.len() == 64, "Invalid SHA256 fingerprint");
    Ok(value)
}
fn android(root: &Path, version: &str, out: &Path, s: &Signing) -> Result<()> {
    ci::gradle(root, &["assembleRelease", "bundleRelease"], &s.envs)?;
    for (ext, folder) in [("apk", "apk/release"), ("aab", "bundle/release")] {
        let paths = files(&root.join("apps/android/app/build/outputs").join(folder))?;
        let candidates: Vec<_> = paths
            .iter()
            .filter(|p| p.extension().is_some_and(|e| e == ext))
            .collect();
        ensure!(
            candidates.len() == 1,
            "Expected exactly one signed Android {ext}"
        );
        copy_file(
            candidates[0],
            &out.join(format!("vynamp-{version}-android-arm64.{ext}")),
        )?;
    }
    let tools = read_json(root.join("config/toolchains.json"))?;
    let signer = PathBuf::from(required("ANDROID_HOME")?)
        .join("build-tools")
        .join(string(&tools, "android_build_tools")?)
        .join(if cfg!(windows) {
            "apksigner.bat"
        } else {
            "apksigner"
        });
    let apk = s.call(
        signer.to_str().unwrap(),
        &[
            "verify",
            "--print-certs",
            out.join(format!("vynamp-{version}-android-arm64.apk"))
                .to_str()
                .unwrap(),
        ],
    )?;
    let aab = out.join(format!("vynamp-{version}-android-arm64.aab"));
    s.call("jarsigner", &["-verify", aab.to_str().unwrap()])?;
    let key = s.call(
        "keytool",
        &[
            "-J-Duser.language=en",
            "-list",
            "-v",
            "-keystore",
            &s.value("ANDROID_KEYSTORE_PATH")?,
            "-storepass:env",
            "ANDROID_STORE_PASSWORD",
            "-alias",
            &required("ANDROID_KEY_ALIAS")?,
        ],
    )?;
    let bundle = s.call(
        "keytool",
        &[
            "-J-Duser.language=en",
            "-printcert",
            "-jarfile",
            aab.to_str().unwrap(),
        ],
    )?;
    ensure!(
        fingerprint(&key)? == fingerprint(&apk)? && fingerprint(&key)? == fingerprint(&bundle)?,
        "APK/AAB signed by unexpected identity"
    );
    Ok(())
}
pub fn build(root: &Path, platform: &str, version: &str) -> Result<()> {
    let cfg = release::preflight(root, version)?;
    check_host(platform)?;
    ci::toolchains(root, platform == "android-arm64")?;
    let out = root.join("dist/release");
    fs::create_dir_all(&out)?;
    let staging = root.join("dist/staging");
    fs::create_dir_all(&staging)?;
    let stage = tempfile::Builder::new()
        .prefix(platform)
        .tempdir_in(&staging)?;
    let signing = Signing::provision(root, platform)?;
    let result = match platform {
        "windows-x64" => windows(root, version, &cfg, &out, stage.path(), &signing),
        "linux-x64" => linux(root, version, &cfg, &out, stage.path()),
        "macos-universal" => mac(root, version, &cfg, &out, stage.path(), &signing),
        "android-arm64" => android(root, version, &out, &signing),
        _ => anyhow::bail!("Unknown platform"),
    };
    let cleanup = signing.finish();
    result?;
    cleanup?;
    Ok(())
}
