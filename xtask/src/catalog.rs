use anyhow::{Context, Result, ensure};
use serde_json::Value;
use std::collections::BTreeSet;

/// The five targets every required format must reach from M0 to M3.
const CORE_TARGETS: [&str; 5] = [
    "windows-x64",
    "linux-x64",
    "macos-intel",
    "macos-silicon",
    "android-arm64",
];

/// The phase iOS joins the mandatory set, and the reason it is spelled `F6` rather than
/// `M6`: iOS is [FASE 6](../../delivery/01-roadmap.md) of the delivery roadmap, shipping
/// after 1.0, not the sixth milestone of it. No catalog entry is at `F6` today -- the phase
/// is declared here so that the day one is, the rule that governs it already exists instead
/// of being written to fit whatever the entry happened to say.
const IOS_PHASE: &str = "F6";

/// Position of a phase in the delivery order: `M0`..`M5`, then `F6`.
///
/// Ordering by the phase string itself is wrong and quietly so: `"F6" < "M1"` compares
/// `F` (0x46) against `M` (0x4D), so every 1.0 milestone would sort *after* the iOS phase
/// and inherit its sixth target. The roadmap's order is a number the letters do not carry.
fn phase_rank(phase: &str) -> Option<u8> {
    match phase.strip_prefix('M') {
        Some(digits) => digits.parse::<u8>().ok(),
        None if phase == IOS_PHASE => Some(6),
        None => None,
    }
}

/// Every target a required format scheduled at `phase` must reach.
///
/// Five from M0 to M3, six from FASE 6 on: iOS ships after 1.0, so no 1.0 milestone may be
/// asked for iOS evidence, and no iPhone build may ship without the formats 1.0 called
/// required. A phase the roadmap does not define has no floor to state, and the caller
/// reports the omission rather than guessing one.
fn required_floor(phase: &str) -> Option<BTreeSet<&'static str>> {
    let ios = phase_rank(IOS_PHASE)?;
    let rank = phase_rank(phase)?;
    let mut floor: BTreeSet<&'static str> = CORE_TARGETS.into_iter().collect();
    if rank >= ios {
        floor.insert(IOS_TARGET);
    }
    Some(floor)
}

/// The platform introduced at [`IOS_PHASE`].
const IOS_TARGET: &str = "ios-arm64";

pub fn validate(value: &Value) -> Result<()> {
    let rows = value["entries"].as_array().context("Catalog entries")?;
    let mut seen = BTreeSet::new();
    for row in rows {
        let id = row["id"].as_str().context("Catalog ID")?;
        ensure!(seen.insert(id), "Duplicate catalog ID: {id}");
        if row["kind"] == "bluetooth-transport" {
            ensure!(
                row["association_allowed"] == false,
                "Bluetooth cannot be a file handler: {id}"
            );
            ensure!(
                row["extensions"].as_array().is_some_and(Vec::is_empty),
                "Bluetooth cannot declare file extensions: {id}"
            );
        }
        if row["codec_id"] == "iamf" {
            ensure!(
                row["kind"] == "immersive-model",
                "IAMF must be an immersive model: {id}"
            );
        }
        let webm = row["containers"]
            .as_array()
            .is_some_and(|a| a.iter().any(|s| s == "WebM"));
        if webm {
            ensure!(
                row["codec_id"] != "flac",
                "FLAC is not conventional WebM: {id}"
            );
            let flac_profile = row["profiles"].as_array().is_some_and(|a| {
                a.iter().any(|s| {
                    s.as_str()
                        .is_some_and(|s| s.to_lowercase().contains("flac"))
                })
            });
            ensure!(!flac_profile, "FLAC profile in conventional WebM: {id}");
        }
        if row["policy"] == "required" {
            ensure!(
                matches!(row["phase"].as_str(), Some("M0" | "M1" | "M2" | "M3")),
                "Required format postponed: {id}"
            );
            let targets: BTreeSet<_> = row["target_platforms"]
                .as_array()
                .context("Catalog targets")?
                .iter()
                .filter_map(Value::as_str)
                .collect();
            // Five targets are mandatory from M0 to M3; iOS joins from FASE 6 on. The
            // rule is a floor per phase, not equality with one fixed set: equality would
            // have made an iOS target *illegal* to declare, and the whole point of the
            // sixth platform is that a required format reaches it too.
            let floor = required_floor(row["phase"].as_str().unwrap_or_default())
                .context("Required format has no roadmap phase")?;
            ensure!(
                floor.is_subset(&targets),
                "Required format missing a target: {id}"
            );
        }
    }
    Ok(())
}

/// Additional byte bounds for specification examples; not a player IPC server.
pub fn open_request_bytes(bytes: &[u8]) -> Result<Value> {
    ensure!(
        bytes.len() <= 1024 * 1024,
        "OpenRequest envelope exceeds 1 MiB"
    );
    let value: Value = serde_json::from_slice(bytes)?;
    for entry in value["entries"].as_array().context("OpenRequest entries")? {
        ensure!(
            entry["locator"]
                .as_str()
                .context("OpenRequest locator")?
                .len()
                <= 8192,
            "OpenRequest locator exceeds 8192 UTF-8 bytes"
        );
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn fixture() -> Value {
        serde_json::from_str(include_str!("../../config/format-catalog.json")).unwrap()
    }
    #[test]
    fn duplicate_id_and_missing_required_platform_rejected() {
        let mut value = fixture();
        value["entries"][1]["id"] = value["entries"][0]["id"].clone();
        assert!(validate(&value).is_err());
        let mut value = fixture();
        value["entries"][0]["target_platforms"] = json!(["windows-x64"]);
        assert!(validate(&value).is_err());
    }
    /// iOS is a mandatory target from FASE 6, and the rule that says so has to fail on the
    /// format that forgets it -- otherwise adding `ios-arm64` to `config/project.json` would
    /// be a line no format ever has to honour.
    #[test]
    fn required_floor_grows_by_ios_and_never_shrinks_below_five() {
        // Inside 1.0 the five original targets are the whole floor: a format at M3 that
        // omits iOS is compliant, because iOS ships after it.
        let core: BTreeSet<&str> = CORE_TARGETS.into_iter().collect();
        assert_eq!(required_floor("M0"), Some(core.clone()));
        assert_eq!(required_floor("M3"), Some(core.clone()));

        // At FASE 6 it no longer is: the sixth target joins, and the five stay mandatory
        // beside it rather than being replaced.
        let mut ios = core.clone();
        ios.insert(IOS_TARGET);
        assert_eq!(required_floor(IOS_PHASE), Some(ios.clone()));
        assert!(ios.contains("android-arm64") && ios.contains("windows-x64"));
        assert_eq!(ios.len(), 6);

        // Ordering is the roadmap's, not the alphabet's: `"F6" < "M1"` as strings, which
        // is why a string comparison would have given every 1.0 milestone the iOS target.
        assert!("F6" < "M1");
        assert!(phase_rank("M1") < phase_rank(IOS_PHASE));

        // A phase the roadmap does not define states no floor instead of inventing one.
        assert_eq!(required_floor("F9"), None);
        assert_eq!(required_floor(""), None);
    }
    #[test]
    fn incorrect_format_category_and_webm_binding_rejected() {
        let mut value = fixture();
        let entries = value["entries"].as_array_mut().unwrap();
        let model = entries
            .iter_mut()
            .find(|r| r["codec_id"] == "iamf")
            .unwrap();
        model["kind"] = json!("audio");
        assert!(validate(&value).is_err());
        let mut value = fixture();
        value["entries"][0]["containers"] = json!(["WebM"]);
        value["entries"][0]["codec_id"] = json!("flac");
        assert!(validate(&value).is_err());
    }
    #[test]
    fn planned_capability_cannot_advertise_availability() {
        let schema: Value =
            serde_json::from_str(include_str!("../../schemas/codec-capability.schema.json"))
                .unwrap();
        let mut example: Value =
            serde_json::from_str(include_str!("../../schemas/examples/codec-capability.json"))
                .unwrap();
        assert!(crate::validate::schema_example(&schema, &example).is_ok());
        example["available"] = json!(true);
        assert!(crate::validate::schema_example(&schema, &example).is_err());
        example["state"] = json!("verified");
        example["licensed"] = json!(true);
        assert!(crate::validate::schema_example(&schema, &example).is_err());
    }
    #[test]
    fn bluetooth_cannot_be_file_handler_and_oversized_request_rejected() {
        let mut value = fixture();
        let row = value["entries"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|r| r["kind"] == "bluetooth-transport")
            .unwrap();
        row["association_allowed"] = json!(true);
        assert!(validate(&value).is_err());
        let schema: Value =
            serde_json::from_str(include_str!("../../schemas/open-request.schema.json")).unwrap();
        let mut example: Value =
            serde_json::from_str(include_str!("../../schemas/examples/open-request.json")).unwrap();
        example["entries"][0]["locator"] = json!("a".repeat(8193));
        assert!(crate::validate::schema_example(&schema, &example).is_err());
    }
    #[test]
    fn canonical_planning_catalog_passes() {
        assert!(validate(&fixture()).is_ok());
    }
    #[test]
    fn multibyte_locators_and_envelope_limits_are_bytes() {
        let schema: Value =
            serde_json::from_str(include_str!("../../schemas/open-request.schema.json")).unwrap();
        let mut example: Value =
            serde_json::from_str(include_str!("../../schemas/examples/open-request.json")).unwrap();
        example["entries"][0]["locator"] = json!("é".repeat(4097));
        assert!(crate::validate::schema_example(&schema, &example).is_ok());
        assert!(open_request_bytes(&serde_json::to_vec(&example).unwrap()).is_err());
        example["entries"][0]["locator"] = json!("é".repeat(4096));
        assert!(open_request_bytes(&serde_json::to_vec(&example).unwrap()).is_ok());
        let mut bytes = serde_json::to_vec(&example).unwrap();
        bytes.resize(1024 * 1024 + 1, b' ');
        assert!(open_request_bytes(&bytes).is_err());
    }
    #[test]
    fn visualizer_preset_cannot_embed_code_or_unbounded_fft() {
        let schema: Value =
            serde_json::from_str(include_str!("../../schemas/visualizer-preset.schema.json"))
                .unwrap();
        let mut example: Value = serde_json::from_str(include_str!(
            "../../schemas/examples/visualizer-preset.json"
        ))
        .unwrap();
        crate::validate::schema_example(&schema, &example).unwrap();
        example["shader"] = json!("arbitrary code");
        assert!(crate::validate::schema_example(&schema, &example).is_err());
        example.as_object_mut().unwrap().remove("shader");
        example["fft_size"] = json!(1048576);
        assert!(crate::validate::schema_example(&schema, &example).is_err());
    }
}
