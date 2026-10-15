use anyhow::{Context, Result, ensure};
use serde_json::Value;
use std::collections::BTreeSet;

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
            ensure!(
                targets
                    == BTreeSet::from([
                        "windows-x64",
                        "linux-x64",
                        "macos-intel",
                        "macos-silicon",
                        "android-arm64"
                    ]),
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
