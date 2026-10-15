use crate::util::*;
use anyhow::{Result, ensure};
use base64::Engine;
use std::path::{Path, PathBuf};
pub struct Signing {
    temp: tempfile::TempDir,
    pub envs: Vec<(String, String)>,
    keychain: Option<PathBuf>,
    cert: Option<String>,
    root: PathBuf,
}
impl Signing {
    pub fn provision(root: &Path, platform: &str) -> Result<Self> {
        let mut s = Self {
            temp: tempfile::tempdir()?,
            envs: vec![],
            keychain: None,
            cert: None,
            root: root.into(),
        };
        match platform {
            "windows-x64" => {
                let p = s.decode("WIN_PFX_BASE64", "certificate.pfx")?;
                required("WIN_PFX_PASSWORD")?;
                s.envs
                    .push(("VYNAMP_PFX_PATH".into(), p.to_string_lossy().into_owned()));
                let thumb=s.call("powershell",&["-NoProfile","-Command","$p=ConvertTo-SecureString $env:WIN_PFX_PASSWORD -AsPlainText -Force; (Import-PfxCertificate -FilePath $env:VYNAMP_PFX_PATH -CertStoreLocation Cert:\\CurrentUser\\My -Password $p | Where-Object HasPrivateKey | Select-Object -First 1).Thumbprint"])?;
                ensure!(!thumb.is_empty(), "No imported certificate selector");
                s.cert = Some(thumb.clone());
                s.envs.push(("WIN_CERT_SHA1".into(), thumb));
            }
            "macos-universal" => {
                let cert = s.decode("MAC_P12_BASE64", "certificate.p12")?;
                let mut random = [0u8; 32];
                getrandom::fill(&mut random)
                    .map_err(|_| anyhow::anyhow!("Secure randomness unavailable"))?;
                let pass = random
                    .iter()
                    .map(|b| format!("{b:02x}"))
                    .collect::<String>();
                let keychain = s.temp.path().join("signing.keychain-db");
                s.call(
                    "security",
                    &["create-keychain", "-p", &pass, keychain.to_str().unwrap()],
                )?;
                s.keychain = Some(keychain.clone());
                s.call(
                    "security",
                    &[
                        "set-keychain-settings",
                        "-lut",
                        "21600",
                        keychain.to_str().unwrap(),
                    ],
                )?;
                s.call(
                    "security",
                    &["unlock-keychain", "-p", &pass, keychain.to_str().unwrap()],
                )?;
                s.call(
                    "security",
                    &[
                        "import",
                        cert.to_str().unwrap(),
                        "-k",
                        keychain.to_str().unwrap(),
                        "-P",
                        &required("MAC_P12_PASSWORD")?,
                        "-T",
                        "/usr/bin/codesign",
                    ],
                )?;
                s.call(
                    "security",
                    &[
                        "set-key-partition-list",
                        "-S",
                        "apple-tool:,apple:,codesign:",
                        "-s",
                        "-k",
                        &pass,
                        keychain.to_str().unwrap(),
                    ],
                )?;
                s.envs.push((
                    "VYNAMP_SIGN_KEYCHAIN".into(),
                    keychain.to_string_lossy().into_owned(),
                ));
                let key = s.decode("MAC_NOTARY_KEY_BASE64", "notary.p8")?;
                s.envs.push((
                    "VYNAMP_NOTARY_KEY_PATH".into(),
                    key.to_string_lossy().into_owned(),
                ));
                for name in [
                    "MAC_NOTARY_KEY_ID",
                    "MAC_NOTARY_ISSUER",
                    "MAC_SIGN_IDENTITY",
                ] {
                    required(name)?;
                }
            }
            "android-arm64" => {
                let p = s.decode("ANDROID_KEYSTORE_BASE64", "release.jks")?;
                s.envs.push((
                    "ANDROID_KEYSTORE_PATH".into(),
                    p.to_string_lossy().into_owned(),
                ));
                for name in [
                    "ANDROID_STORE_PASSWORD",
                    "ANDROID_KEY_ALIAS",
                    "ANDROID_KEY_PASSWORD",
                ] {
                    required(name)?;
                }
            }
            "linux-x64" => {}
            _ => anyhow::bail!("Unknown signing platform"),
        };
        Ok(s)
    }
    fn decode(&self, var: &str, name: &str) -> Result<PathBuf> {
        let p = self.temp.path().join(name);
        private_file(
            &p,
            &base64::engine::general_purpose::STANDARD.decode(required(var)?)?,
        )?;
        Ok(p)
    }
    pub fn value(&self, name: &str) -> Result<String> {
        if let Some((_, v)) = self.envs.iter().find(|(k, _)| k == name) {
            Ok(v.clone())
        } else {
            required(name)
        }
    }
    pub fn call(&self, exe: &str, args: &[&str]) -> Result<String> {
        secret_call(&self.root, exe, args, &self.envs)
    }
    pub fn windows(&self, path: &Path) -> Result<()> {
        let cfg = read_json(self.root.join("config/toolchains.json"))?;
        let tool = if std::process::Command::new("signtool")
            .arg("/?")
            .output()
            .is_ok()
        {
            "signtool".into()
        } else {
            PathBuf::from(required("ProgramFiles(x86)")?)
                .join("Windows Kits/10/bin")
                .join(string(&cfg, "windows_sdk_version")?)
                .join("x64/signtool.exe")
                .to_string_lossy()
                .into_owned()
        };
        self.call(
            &tool,
            &[
                "sign",
                "/fd",
                "SHA256",
                "/sha1",
                &self.value("WIN_CERT_SHA1")?,
                "/tr",
                &required("WIN_TIMESTAMP_URL")?,
                "/td",
                "SHA256",
                path.to_str().unwrap(),
            ],
        )?;
        self.call(&tool, &["verify", "/pa", "/all", path.to_str().unwrap()])?;
        Ok(())
    }
    pub fn mac(&self, path: &Path) -> Result<()> {
        self.call(
            "codesign",
            &[
                "--force",
                "--options",
                "runtime",
                "--timestamp",
                "--keychain",
                &self.value("VYNAMP_SIGN_KEYCHAIN")?,
                "--sign",
                &required("MAC_SIGN_IDENTITY")?,
                path.to_str().unwrap(),
            ],
        )?;
        Ok(())
    }
    pub fn submit(&self, path: &Path) -> Result<()> {
        self.call(
            "xcrun",
            &[
                "notarytool",
                "submit",
                path.to_str().unwrap(),
                "--key",
                &self.value("VYNAMP_NOTARY_KEY_PATH")?,
                "--key-id",
                &required("MAC_NOTARY_KEY_ID")?,
                "--issuer",
                &required("MAC_NOTARY_ISSUER")?,
                "--wait",
            ],
        )?;
        Ok(())
    }
    pub fn staple(&self, path: &Path) -> Result<()> {
        self.call("xcrun", &["stapler", "staple", path.to_str().unwrap()])?;
        self.call("xcrun", &["stapler", "validate", path.to_str().unwrap()])?;
        Ok(())
    }
    fn cleanup(&mut self) -> Result<()> {
        if let Some(p) = self.keychain.as_ref() {
            self.call("security", &["delete-keychain", p.to_str().unwrap()])?;
            self.keychain = None;
        }
        if let Some(thumb) = self.cert.as_ref() {
            let mut envs = self.envs.clone();
            envs.push(("VYNAMP_IMPORTED_CERT".into(), thumb.clone()));
            secret_call(
                &self.root,
                "powershell",
                &[
                    "-NoProfile",
                    "-Command",
                    "Remove-Item ('Cert:\\CurrentUser\\My\\' + $env:VYNAMP_IMPORTED_CERT)",
                ],
                &envs,
            )?;
            self.cert = None;
        }
        Ok(())
    }
    pub fn finish(mut self) -> Result<()> {
        self.cleanup()
    }
}
impl Drop for Signing {
    fn drop(&mut self) {
        if self.cleanup().is_err() {
            eprintln!(
                "Signing credential cleanup failed; inspect the ephemeral runner before reuse."
            );
        }
    }
}
