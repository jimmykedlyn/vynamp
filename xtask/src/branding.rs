use crate::util::*;
use anyhow::{Context, Result, ensure};
use serde_json::Value;
use std::{collections::BTreeSet, fs, path::Path};

pub fn references(root: &Path) -> Result<()> {
    let m = read_json(root.join("assets/branding/asset-manifest.json"))?;
    ensure!(
        m["product"] == "Vynamp" && m["concept"] == "C",
        "Wrong approved branding"
    );
    let assets = m["assets"].as_array().context("Brand references absent")?;
    ensure!(
        assets.len() == 3,
        "Expected three original brand references"
    );
    for row in assets {
        let path = safe_existing(root, string(row, "path")?)?;
        let bytes = fs::read(&path)?;
        ensure!(
            bytes.len() >= 33 && &bytes[..8] == b"\x89PNG\r\n\x1a\n",
            "Brand reference is not PNG"
        );
        ensure!(
            row["sha256"] == sha(&path)? && row["bytes"] == bytes.len() as u64,
            "Brand reference hash/size mismatch"
        );
        let width = u32::from_be_bytes(bytes[16..20].try_into()?);
        let height = u32::from_be_bytes(bytes[20..24].try_into()?);
        ensure!(
            row["width"] == width && row["height"] == height && row["status"] == "reference",
            "Brand reference dimensions/state mismatch"
        );
        if string(row, "path")?.ends_with("symbol-c-transparent.png") {
            ensure!(bytes[25] == 6, "Transparent symbol must use RGBA PNG");
        }
    }
    let tokens = read_json(root.join("assets/branding/brand-tokens.json"))?;
    ensure!(
        tokens["name"] == "Vynamp" && tokens["concept"] == "C",
        "Wrong branding tokens"
    );
    for (name, color) in [
        ("brand-accent", "#FFBF69"),
        ("brand-background", "#11131B"),
        ("brand-wordmark-light", "#F0F2F5"),
    ] {
        ensure!(
            tokens["colors"][name] == color,
            "Brand color differs from approved C identity"
        );
    }
    Ok(())
}

pub fn exports(root: &Path) -> Result<()> {
    let cfg = read_json(root.join("config/project.json"))?;
    ensure!(
        cfg["branding_exports_ready"] == true,
        "Native branding exports pending; reference PNGs do not satisfy installer gates"
    );
    references(root)?;
    let m = read_json(root.join("assets/branding/asset-manifest.json"))?;
    let receipt = read_json(root.join("packaging/icons/export-manifest.json"))?;
    ensure!(
        receipt["concept"] == "C"
            && receipt["source_digest"] == crate::release::source_digest(root)?,
        "Branding evidence concept/digest mismatch"
    );
    let reviewers: BTreeSet<_> = receipt["reviewers"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect();
    ensure!(
        reviewers == BTreeSet::from(["claude"]),
        "Branding export reviews missing"
    );
    let files = receipt["files"]
        .as_array()
        .context("Native branding file inventory missing")?;
    for platform in crate::release::PLATFORMS {
        ensure!(
            m["native_exports"][platform]["status"] == "verified"
                && receipt["platforms"][platform]["status"] == "verified",
            "Unverified branding platform: {platform}"
        );
        let row = &receipt["platforms"][platform];
        let report = safe_existing(root, string(row, "report")?)?;
        ensure!(
            row["sha256"] == sha(&report)?,
            "Branding platform report hash mismatch"
        );
        for expected in m["native_exports"][platform]["files"]
            .as_array()
            .context("Expected platform branding files")?
        {
            let name = expected.as_str().context("Branding file path")?;
            let record = files
                .iter()
                .find(|r| r["path"] == name)
                .context("Required native branding file absent from inventory")?;
            let path = safe_existing(root, name)?;
            ensure!(
                record["sha256"] == sha(&path)?,
                "Native branding file hash mismatch"
            );
            let bytes = fs::read(&path)?;
            ensure!(!bytes.is_empty(), "Empty native branding file");
            native_format(&path, &bytes)?;
            if path.extension().is_some_and(|x| x == "png") {
                let expected = &m["expected_png_dimensions"][name];
                let (width, height) = png_dimensions(&bytes)?;
                ensure!(
                    expected["width"] == width && expected["height"] == height,
                    "Native PNG differs from required platform dimensions"
                );
            }
        }
    }
    Ok(())
}

fn png_dimensions(bytes: &[u8]) -> Result<(u32, u32)> {
    let mut decoder = png::Decoder::new(std::io::Cursor::new(bytes));
    decoder.set_limits(png::Limits {
        bytes: 16 * 1024 * 1024,
    });
    let mut reader = decoder.read_info()?;
    let info = reader.info();
    ensure!(
        info.width > 0
            && info.width <= 1024
            && info.height == info.width
            && info.animation_control.is_none(),
        "Invalid native PNG dimensions/animation"
    );
    let dims = (info.width, info.height);
    let mut buf = vec![0; reader.output_buffer_size().context("PNG decode buffer")?];
    let output = reader.next_frame(&mut buf)?;
    ensure!(
        matches!(
            output.color_type,
            png::ColorType::Rgb | png::ColorType::Rgba
        ) && output.bit_depth == png::BitDepth::Eight,
        "Native icons require RGB/RGBA8 PNG"
    );
    reader.finish()?;
    Ok(dims)
}
fn native_format(path: &Path, bytes: &[u8]) -> Result<()> {
    match path.extension().and_then(|s| s.to_str()) {
        Some("png") => {
            png_dimensions(bytes)?;
        }
        Some("ico") => {
            ensure!(
                bytes.len() >= 6 && &bytes[..4] == b"\0\0\x01\0",
                "Invalid ICO header"
            );
            let count = u16::from_le_bytes(bytes[4..6].try_into()?) as usize;
            ensure!(
                (1..=32).contains(&count) && bytes.len() >= 6 + count * 16,
                "Missing ICO frames"
            );
            let mut sizes = BTreeSet::new();
            let mut ranges = Vec::new();
            for i in 0..count {
                let row = &bytes[6 + i * 16..22 + i * 16];
                let width = if row[0] == 0 { 256 } else { row[0] as u32 };
                let height = if row[1] == 0 { 256 } else { row[1] as u32 };
                let len = u32::from_le_bytes(row[8..12].try_into()?) as usize;
                let offset = u32::from_le_bytes(row[12..16].try_into()?) as usize;
                let end = offset.checked_add(len).context("ICO size overflow")?;
                ensure!(
                    len > 0 && offset >= 6 + count * 16 && end <= bytes.len(),
                    "Invalid ICO frame offset/length"
                );
                ensure!(
                    png_dimensions(&bytes[offset..end])? == (width, height),
                    "ICO dimensions differ from payload"
                );
                ranges.push((offset, end));
                sizes.insert(width);
            }
            ranges.sort();
            ensure!(
                ranges.windows(2).all(|v| v[0].1 <= v[1].0),
                "Overlapping ICO payloads"
            );
            ensure!(
                BTreeSet::from([16, 20, 24, 32, 40, 48, 64, 128, 256]).is_subset(&sizes),
                "Missing required Windows icon sizes"
            );
        }
        Some("icns") => {
            ensure!(
                bytes.len() >= 8
                    && &bytes[..4] == b"icns"
                    && u32::from_be_bytes(bytes[4..8].try_into()?) as usize == bytes.len(),
                "Invalid ICNS header/length"
            );
            let mut pos = 8;
            let mut sizes = BTreeSet::new();
            let mut tags = BTreeSet::new();
            while pos < bytes.len() {
                ensure!(bytes.len() - pos >= 8, "Truncated ICNS chunk");
                let len = u32::from_be_bytes(bytes[pos + 4..pos + 8].try_into()?) as usize;
                let end = pos.checked_add(len).context("ICNS overflow")?;
                ensure!(len >= 8 && end <= bytes.len(), "Invalid ICNS chunk length");
                tags.insert(<[u8; 4]>::try_from(&bytes[pos..pos + 4])?);
                let size = match &bytes[pos..pos + 4] {
                    b"icp4" => Some(16),
                    b"icp5" | b"ic11" => Some(32),
                    b"icp6" | b"ic12" => Some(64),
                    b"ic07" => Some(128),
                    b"ic08" | b"ic13" => Some(256),
                    b"ic09" | b"ic14" => Some(512),
                    b"ic10" => Some(1024),
                    _ => None,
                };
                if let Some(size) = size {
                    ensure!(
                        png_dimensions(&bytes[pos + 8..end])? == (size, size),
                        "ICNS dimensions differ from payload"
                    );
                    sizes.insert(size);
                }
                pos = end;
            }
            ensure!(
                [
                    *b"icp4", *b"ic11", *b"icp5", *b"ic12", *b"ic07", *b"ic13", *b"ic08", *b"ic14",
                    *b"ic09", *b"ic10"
                ]
                .iter()
                .all(|t| tags.contains(t)),
                "Missing required ICNS 1x/2x roles"
            );
            ensure!(
                BTreeSet::from([16, 32, 64, 128, 256, 512, 1024]).is_subset(&sizes),
                "Missing required modern PNG-backed ICNS frames"
            );
        }
        Some("svg" | "xml") => {
            let body = std::str::from_utf8(bytes)?;
            let mut reader = quick_xml::Reader::from_str(body);
            let mut depth = 0usize;
            let mut roots = 0;
            let mut root = String::new();
            let mut paths = 0;
            let svg = path.extension().is_some_and(|x| x == "svg");
            loop {
                let event = reader.read_event()?;
                let empty = matches!(&event, quick_xml::events::Event::Empty(_));
                match event {
                    quick_xml::events::Event::Start(e) | quick_xml::events::Event::Empty(e) => {
                        let name = e.name().as_ref().to_owned();
                        if depth == 0 {
                            roots += 1;
                            root = name.clone();
                        }
                        if name == "path" {
                            paths += 1;
                        }
                        if svg {
                            ensure!(
                                matches!(name.as_str(), "svg" | "g" | "path" | "title" | "desc"),
                                "SVG contains unsupported/active elements"
                            );
                            for attr in e.attributes() {
                                let a = attr?;
                                let key = a.key.as_ref();
                                // quick-xml 0.42 deprecou `unescape_value` em favor de
                                // `normalized_value`, que alem de desescapar entidades
                                // normaliza espaco em branco literal conforme a spec XML. Para
                                // um gate que procura `url(` o resultado e' igual ou mais
                                // restritivo -- nunca mais permissivo.
                                let value =
                                    a.normalized_value(quick_xml::XmlVersion::Implicit1_0)?;
                                ensure!(
                                    matches!(
                                        key,
                                        "xmlns"
                                            | "viewBox"
                                            | "width"
                                            | "height"
                                            | "fill"
                                            | "fill-rule"
                                            | "clip-rule"
                                            | "d"
                                            | "transform"
                                            | "opacity"
                                            | "id"
                                            | "role"
                                            | "aria-label"
                                            | "stroke"
                                            | "stroke-width"
                                            | "stroke-linejoin"
                                            | "stroke-linecap"
                                    ) && !key.to_lowercase().starts_with("on")
                                        && !key.ends_with("href")
                                        && !value.to_ascii_lowercase().contains("url("),
                                    "SVG external/active reference"
                                );
                            }
                        }
                        if !empty {
                            depth += 1;
                        }
                    }
                    quick_xml::events::Event::End(_) => {
                        depth = depth.checked_sub(1).context("Unbalanced icon XML")?
                    }
                    quick_xml::events::Event::DocType(_) => anyhow::bail!("Icon XML DTD forbidden"),
                    quick_xml::events::Event::Eof => break,
                    _ => {}
                }
            }
            ensure!(roots == 1 && depth == 0, "Invalid icon XML root");
            if svg {
                ensure!(
                    root == "svg" && paths > 0,
                    "SVG must have svg root and vector paths"
                );
            } else {
                ensure!(
                    matches!(root.as_str(), "vector" | "adaptive-icon" | "resources"),
                    "Invalid Android icon XML root"
                );
            }
        }
        _ => anyhow::bail!("Unexpected native branding format"),
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_gate_rejects_empty_icon_containers_and_non_svg_xml() {
        assert!(native_format(Path::new("vynamp.ico"), b"\0\0\x01\0\0\0x").is_err());
        assert!(native_format(Path::new("Vynamp.icns"), b"icns\0\0\0\0x").is_err());
        assert!(native_format(Path::new("vynamp.svg"), b"<x/>").is_err());
    }
    fn sample_png(size: u32) -> Vec<u8> {
        let mut out = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut out, size, size);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            writer
                .write_image_data(&vec![255; (size * size * 4) as usize])
                .unwrap();
            writer.finish().unwrap();
        }
        out
    }
    #[test]
    fn native_gate_accepts_decodable_multisize_png_icon_containers() {
        let sizes = [16u32, 20, 24, 32, 40, 48, 64, 128, 256];
        let frames: Vec<_> = sizes.iter().map(|s| sample_png(*s)).collect();
        let mut ico = vec![0, 0, 1, 0, 9, 0];
        let mut offset = 6 + 16 * sizes.len();
        for (size, frame) in sizes.iter().zip(&frames) {
            let side = if *size == 256 { 0 } else { *size as u8 };
            ico.extend_from_slice(&[side, side, 0, 0, 1, 0, 32, 0]);
            ico.extend_from_slice(&(frame.len() as u32).to_le_bytes());
            ico.extend_from_slice(&(offset as u32).to_le_bytes());
            offset += frame.len();
        }
        for frame in frames {
            ico.extend(frame);
        }
        assert!(native_format(Path::new("vynamp.ico"), &ico).is_ok());
        let mut icns = b"icns\0\0\0\0".to_vec();
        for (tag, size) in [
            (b"icp4", 16),
            (b"ic11", 32),
            (b"ic12", 64),
            (b"ic13", 256),
            (b"ic14", 512),
            (b"icp5", 32),
            (b"icp6", 64),
            (b"ic07", 128),
            (b"ic08", 256),
            (b"ic09", 512),
            (b"ic10", 1024),
        ] {
            let frame = sample_png(size);
            icns.extend(tag);
            icns.extend_from_slice(&((frame.len() + 8) as u32).to_be_bytes());
            icns.extend(frame);
        }
        let len = icns.len() as u32;
        icns[4..8].copy_from_slice(&len.to_be_bytes());
        assert!(native_format(Path::new("Vynamp.icns"), &icns).is_ok());
        assert!(native_format(Path::new("vynamp.png"), &sample_png(32)).is_ok());
    }
    #[test]
    fn native_svg_rejects_active_content_and_accepts_static_geometry() {
        assert!(native_format(Path::new("vynamp.svg"),br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><path d="M0 0L5 10L10 0Z" fill="#FFBF69"/></svg>"##).is_ok());
        assert!(native_format(Path::new("vynamp.svg"), b"<svg><script/></svg>").is_err());
        assert!(
            native_format(
                Path::new("vynamp.svg"),
                b"<svg><path href='https://example.invalid/logo.svg'/></svg>"
            )
            .is_err()
        );
    }
    /// O gate procura `url(` no valor **desescapado**: um `url(` escrito com
    /// entidades (`&#117;rl(`) e' a mesma referencia externa que o literal, e um
    /// valor lido cru passaria. quick-xml 0.42 trocou `unescape_value` por
    /// `normalized_value`, que desescapa e ainda normaliza espaco em branco --
    /// entao o teste tambem fixa que a normalizacao nao esconde o `url(`.
    #[test]
    fn native_svg_rejects_external_reference_hidden_behind_entities() {
        for body in [
            br##"<svg><path fill="&#117;rl(#x)"/></svg>"##.as_slice(),
            br##"<svg><path fill="URL(&#40;data:x)"/></svg>"##.as_slice(),
        ] {
            assert!(
                native_format(Path::new("vynamp.svg"), body).is_err(),
                "{} should be rejected",
                String::from_utf8_lossy(body)
            );
        }
    }
    #[test]
    fn icns_requires_retina_roles_in_addition_to_pixel_sizes() {
        let mut icns = b"icns\0\0\0\0".to_vec();
        for (tag, size) in [
            (b"icp4", 16),
            (b"icp5", 32),
            (b"icp6", 64),
            (b"ic07", 128),
            (b"ic08", 256),
            (b"ic09", 512),
            (b"ic10", 1024),
        ] {
            let frame = sample_png(size);
            icns.extend(tag);
            icns.extend_from_slice(&((frame.len() + 8) as u32).to_be_bytes());
            icns.extend(frame);
        }
        let len = icns.len() as u32;
        icns[4..8].copy_from_slice(&len.to_be_bytes());
        assert!(native_format(Path::new("Vynamp.icns"), &icns).is_err());
    }
}
