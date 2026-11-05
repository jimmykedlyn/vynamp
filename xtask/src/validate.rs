use crate::util::*;
use anyhow::{Context, Result, ensure};
use regex::Regex;
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};
#[derive(Default)]
struct Checks {
    errors: Vec<String>,
}
impl Checks {
    fn check(&mut self, ok: bool, msg: impl Into<String>) {
        if !ok {
            self.errors.push(msg.into());
        }
    }
}
fn strings(v: &Value) -> BTreeSet<String> {
    v.as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect()
}
fn phase(v: &Value) -> Result<u32> {
    Ok(string(v, "phase")?
        .strip_prefix('M')
        .context("Bad phase")?
        .parse()?)
}
fn visit(
    id: &str,
    by: &BTreeMap<String, &Value>,
    visiting: &mut BTreeSet<String>,
    done: &mut BTreeSet<String>,
    c: &mut Checks,
) {
    if visiting.contains(id) {
        c.check(false, format!("DAG cycle: {id}"));
        return;
    }
    if done.contains(id) {
        return;
    }
    visiting.insert(id.into());
    for d in strings(&by[id]["dependencies"]) {
        if by.contains_key(&d) {
            visit(&d, by, visiting, done, c);
        }
    }
    visiting.remove(id);
    done.insert(id.into());
}
pub fn schema_example(schema: &Value, example: &Value) -> Result<()> {
    jsonschema::meta::validate(schema).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let validator = jsonschema::options()
        .with_draft(jsonschema::Draft::Draft202012)
        .should_validate_formats(true)
        .build(schema)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
    validator
        .validate(example)
        .map_err(|e| anyhow::anyhow!(e.to_string()))
}
fn api_refs(v: &Value, api: &Value, c: &mut Checks) {
    match v {
        Value::Object(m) => {
            if let Some(r) = m
                .get("$ref")
                .and_then(Value::as_str)
                .filter(|s| s.starts_with("#/"))
            {
                c.check(
                    api.pointer(&r[1..]).is_some(),
                    format!("Missing OpenAPI ref: {r}"),
                );
            }
            for x in m.values() {
                api_refs(x, api, c);
            }
        }
        Value::Array(a) => {
            for x in a {
                api_refs(x, api, c);
            }
        }
        _ => {}
    }
}
pub fn xml(path: &Path) -> Result<()> {
    let body = text(path)?;
    let mut reader = quick_xml::Reader::from_str(&body);
    let mut depth = 0usize;
    let mut roots = 0usize;
    loop {
        match reader.read_event()? {
            quick_xml::events::Event::Start(_) => {
                if depth == 0 {
                    roots += 1;
                }
                depth += 1;
            }
            quick_xml::events::Event::Empty(_) => {
                if depth == 0 {
                    roots += 1;
                }
            }
            quick_xml::events::Event::End(_) => {
                depth = depth.checked_sub(1).context("Unbalanced XML")?
            }
            quick_xml::events::Event::Eof => break,
            _ => {}
        }
    }
    ensure!(depth == 0 && roots == 1, "Invalid XML root count");
    Ok(())
}
fn lum(color: &str) -> Result<f64> {
    ensure!(color.len() == 7 && color.starts_with('#'), "Invalid color");
    let mut x = [0.; 3];
    for (i, v) in x.iter_mut().enumerate() {
        *v = u8::from_str_radix(&color[1 + i * 2..3 + i * 2], 16)? as f64 / 255.;
        *v = if *v <= 0.04045 {
            *v / 12.92
        } else {
            ((*v + 0.055) / 1.055).powf(2.4)
        };
    }
    Ok(0.2126 * x[0] + 0.7152 * x[1] + 0.0722 * x[2])
}
pub fn spec(root: &Path) -> Result<()> {
    let mut c = Checks::default();
    let all = files(root)?;
    let md: Vec<_> = all
        .iter()
        .filter(|p| p.extension().is_some_and(|e| e == "md"))
        .collect();
    let links = Regex::new(r"\[[^\]]*\]\(([^)]+)\)")?;
    let url = Regex::new(r"^[a-zA-Z][a-zA-Z0-9+.-]*://")?;
    let index = text(root.join("docs/INDEX.md"))?;
    for p in &md {
        let body = text(p)?;
        let rel = p.strip_prefix(root)?.to_string_lossy().replace('\\', "/");
        c.check(body.starts_with("# "), format!("No title: {rel}"));
        c.check(index.contains(&rel), format!("Missing index: {rel}"));
        for cap in links.captures_iter(&body) {
            let link = cap[1].trim_matches(['<', '>']);
            if url.is_match(link) || link.starts_with('#') {
                continue;
            }
            let target = link.split('#').next().unwrap();
            c.check(
                p.parent().unwrap().join(target).exists(),
                format!("Missing link {rel}: {target}"),
            );
        }
    }
    let data = read_json(root.join("tasks/backlog.json"))?;
    let tasks = data["tasks"].as_array().context("Backlog tasks")?;
    let by: BTreeMap<_, _> = tasks
        .iter()
        .map(|t| Ok((string(t, "id")?.to_owned(), t)))
        .collect::<Result<_>>()?;
    c.check(by.len() == tasks.len(), "Duplicate TASK IDs");
    let reqs: Vec<_> = tasks
        .iter()
        .flat_map(|t| strings(&t["requirements"]))
        .collect();
    let reqset: BTreeSet<_> = reqs.iter().cloned().collect();
    c.check(reqset.len() == reqs.len(), "Duplicate REQ IDs");
    let matrix = text(root.join("docs/product/03-requirements.md"))?;
    let matrixids: Vec<String> = Regex::new(r"(?m)^\| (REQ-E\d{2}-\d{2}) \|")?
        .captures_iter(&matrix)
        .map(|m| m[1].into())
        .collect();
    let matrixset: BTreeSet<_> = matrixids.iter().cloned().collect();
    c.check(
        matrixset == reqset && matrixset.len() == matrixids.len(),
        "Requirement matrix mismatch/duplicates",
    );
    let qa = text(root.join("docs/quality/01-test-strategy.md"))?;
    let qaset: BTreeSet<_> = Regex::new(r"QA-\d{3}")?
        .find_iter(&qa)
        .map(|m| m.as_str().to_owned())
        .collect();
    let mut phases: BTreeMap<String, usize> = BTreeMap::new();
    let mut epics = BTreeSet::new();
    for t in tasks {
        let id = string(t, "id")?;
        c.check(
            matches!(
                string(t, "status")?,
                "planned" | "in_progress" | "implemented" | "verified" | "accepted" | "blocked"
            ),
            format!("Invalid status {id}"),
        );
        let path = root.join(format!("tasks/items/{id}.md"));
        c.check(path.is_file(), format!("Task Markdown absent {id}"));
        if path.is_file() {
            let body = text(&path)?;
            for k in ["title", "deliverable", "acceptance"] {
                let value = string(t, k)?;
                c.check(
                    !value.is_empty() && body.contains(value),
                    format!("Backlog/MD mismatch {id} {k}"),
                );
            }
        }
        c.check(
            root.join(string(t, "spec")?).exists(),
            format!("Missing spec {id}"),
        );
        c.check(
            strings(&t["qa"]).is_subset(&qaset),
            format!("Unknown QA {id}"),
        );
        for d in strings(&t["dependencies"]) {
            c.check(by.contains_key(&d), format!("Unknown dependency {id} {d}"));
            if let Some(dep) = by.get(&d) {
                c.check(
                    phase(dep)? <= phase(t)?,
                    format!("Future dependency {id} {d}"),
                );
            }
        }
        *phases.entry(string(t, "phase")?.into()).or_default() += 1;
        epics.insert(string(t, "epic")?);
    }
    let (mut visiting, mut done) = (BTreeSet::new(), BTreeSet::new());
    for id in by.keys() {
        visit(id, &by, &mut visiting, &mut done, &mut c);
    }
    for id in ["TASK-E12-01", "TASK-E12-02", "TASK-E13-10", "TASK-E14-06"] {
        c.check(by.contains_key(id), format!("Missing platform gate {id}"));
        if let Some(t) = by.get(id) {
            c.check(phase(t)? <= 3, format!("Mandatory platform postponed {id}"));
        }
    }
    c.check(
        by.get("TASK-E10-10")
            .is_some_and(|t| strings(&t["dependencies"]).contains("TASK-E14-06")),
        "Release missing platform gate",
    );
    let schemas: Vec<_> = all
        .iter()
        .filter(|p| {
            p.parent() == Some(root.join("schemas").as_path())
                && p.to_string_lossy().ends_with(".schema.json")
        })
        .collect();
    for p in &schemas {
        let name = p
            .file_name()
            .unwrap()
            .to_string_lossy()
            .replace(".schema.json", "");
        if let Err(e) = (|| -> Result<()> {
            let example_path = root.join(format!("schemas/examples/{name}.json"));
            schema_example(&read_json(p)?, &read_json(&example_path)?)?;
            if name == "open-request" {
                crate::catalog::open_request_bytes(&std::fs::read(&example_path)?)?;
            }
            Ok(())
        })() {
            c.check(false, format!("Schema {name}: {e}"));
        }
    }
    let catalog = read_json(root.join("config/format-catalog.json"))?;
    if let Err(e) = schema_example(
        &read_json(root.join("schemas/format-catalog.schema.json"))?,
        &catalog,
    )
    .and_then(|()| crate::catalog::validate(&catalog))
    {
        c.check(false, format!("Format catalog: {e}"));
    }
    c.check(
        catalog["revision"] == data["spec_version"],
        "Catalog revision differs from backlog",
    );
    let api = read_json(root.join("schemas/openapi-platform.json"))?;
    c.check(api["openapi"] == "3.1.0", "Unexpected OpenAPI version");
    api_refs(&api, &api, &mut c);
    let mut ops = Vec::new();
    let params = Regex::new(r"\{([^}]+)\}")?;
    let paths = api["paths"].as_object().context("OpenAPI paths")?;
    for (path, methods) in paths {
        for (_, op) in methods.as_object().context("OpenAPI methods")? {
            ops.push(string(op, "operationId")?.to_owned());
            c.check(
                op.get("responses").is_some(),
                format!("Missing response {path}"),
            );
            let names: BTreeSet<_> = op["parameters"]
                .as_array()
                .into_iter()
                .flatten()
                .filter(|v| v["in"] == "path")
                .filter_map(|v| v["name"].as_str())
                .map(str::to_owned)
                .collect();
            c.check(
                names
                    == params
                        .captures_iter(path)
                        .map(|m| m[1].to_owned())
                        .collect(),
                format!("OpenAPI params mismatch {path}"),
            );
        }
    }
    c.check(
        ops.iter().collect::<BTreeSet<_>>().len() == ops.len(),
        "Duplicate operation IDs",
    );
    for (name, schema) in api["components"]["schemas"]
        .as_object()
        .context("OpenAPI schemas")?
    {
        if let Err(e) = jsonschema::meta::validate(schema) {
            c.check(false, format!("OpenAPI schema {name}: {e}"));
        }
    }
    for p in all.iter().filter(|p| {
        p.parent() == Some(root.join("assets").as_path())
            && p.extension().is_some_and(|e| e == "svg")
    }) {
        if let Err(e) = xml(p) {
            c.check(false, format!("SVG {}: {e}", p.display()));
        }
    }
    let tokens = read_json(root.join("assets/design-tokens.json"))?;
    let design = text(root.join("docs/design/01-visual-system.md"))?;
    for (name, color) in tokens["colors"].as_object().context("Colors")? {
        c.check(
            design.contains(color.as_str().context("Color value")?),
            format!("Token mismatch {name}"),
        );
    }
    for fg in ["text-primary", "text-secondary", "lime", "cyan", "amber"] {
        for bg in ["chassis", "panel-inset", "display"] {
            let a = lum(string(&tokens["colors"], fg)?)?;
            let b = lum(string(&tokens["colors"], bg)?)?;
            c.check(
                (a.max(b) + 0.05) / (a.min(b) + 0.05) >= 4.5,
                format!("Contrast {fg}/{bg}"),
            );
        }
    }
    let report = json!({"spec_version":data["spec_version"],"scope":"Validação documental em Rust; não testa player/hardware/API/WIT.","mandatory_platforms":["Windows x86_64","Linux x86_64","macOS Intel x86_64","macOS Apple Silicon arm64","Android arm64-v8a"],"markdown_files":md.len(),"tasks":tasks.len(),"requirements":matrixids.len(),"epics":epics.len(),"phases":phases,"schemas_and_examples":schemas.len(),"format_catalog_entries":catalog["entries"].as_array().map(Vec::len),"openapi_paths":paths.len(),"operation_count":ops.len(),"errors":c.errors,"result":if c.errors.is_empty(){"passed"}else{"failed"}});
    write_json(root.join("VALIDATION.json"), &report)?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    ensure!(c.errors.is_empty(), "Specification validation failed");
    Ok(())
}
/// Collect every `uses:` reference in a workflow, at any nesting depth, so a
/// job or step added later cannot slip in an unpinned action.
fn action_refs(v: &Value) -> Vec<String> {
    let mut out = Vec::new();
    match v {
        Value::Object(map) => {
            for (k, child) in map {
                if k == "uses"
                    && let Some(s) = child.as_str()
                {
                    out.push(s.to_owned());
                }
                out.extend(action_refs(child));
            }
        }
        Value::Array(items) => {
            for child in items {
                out.extend(action_refs(child));
            }
        }
        _ => {}
    }
    out
}
/// `on:` may be a mapping, a scalar or a sequence; normalise it to the set of
/// triggers GitHub would honour.
fn triggers(v: &Value) -> BTreeSet<String> {
    match v.get("on") {
        Some(Value::String(s)) => BTreeSet::from([s.clone()]),
        Some(Value::Array(items)) => items
            .iter()
            .filter_map(Value::as_str)
            .map(str::to_owned)
            .collect(),
        Some(Value::Object(map)) => map.keys().cloned().collect(),
        _ => BTreeSet::new(),
    }
}
/// CI runs on GitHub Actions (ADR-014). Every action is pinned by full commit
/// SHA, the aggregate gate fails closed on any non-success dependency, release
/// only starts from a `v*` tag and stays fail-closed before publishing, and
/// the five mandatory targets have a hosted runner.
fn github_actions(root: &Path, c: &mut Checks) -> Result<usize> {
    c.check(
        !root.join(".woodpecker").exists(),
        "Woodpecker pipelines present; CI is GitHub Actions (ADR-014)",
    );
    let pinned = Regex::new(r"^[^@\s]+@[0-9a-f]{40}$")?;
    let dir = root.join(".github/workflows");
    if !dir.is_dir() {
        c.check(false, "GitHub Actions workflows absent");
        return Ok(0);
    }
    let mut flows = BTreeMap::new();
    for p in files(&dir)? {
        let name = p
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_owned();
        let v: Value = serde_yaml::from_str(&text(&p)?)?;
        // A workflow must be reachable: either it validates a pull request or
        // exposes workflow_dispatch. `release` is the documented exception —
        // its only valid trigger is a `v*` tag push, checked below.
        c.check(
            name == "release"
                || triggers(&v).contains("pull_request")
                || triggers(&v).contains("workflow_dispatch"),
            format!("Workflow triggers absent: {name}"),
        );
        for action in action_refs(&v) {
            c.check(
                pinned.is_match(&action),
                format!("Unpinned action {action} in {name}"),
            );
        }
        flows.insert(name, v);
    }
    let ci = flows.get("ci");
    let jobs = |flow: Option<&Value>| -> BTreeSet<String> {
        flow.and_then(|v| v["jobs"].as_object())
            .map(|j| j.keys().cloned().collect())
            .unwrap_or_default()
    };
    let ci_jobs = jobs(ci);
    for target in [
        "x86_64-pc-windows-msvc",
        "x86_64-unknown-linux-gnu",
        "x86_64-apple-darwin",
        "aarch64-apple-darwin",
    ] {
        c.check(
            ci.is_some_and(|v| v.to_string().contains(target)),
            format!("Missing desktop target {target}"),
        );
    }
    c.check(
        ci.is_some_and(|v| v.to_string().contains("cargo xtask ci android")),
        "Missing Android CI",
    );
    // The required status check is the job id `gate`; a renamed or narrowed
    // gate would stop blocking merges without failing anything else.
    c.check(ci_jobs.contains("gate"), "CI Gate job absent from ci.yml");
    c.check(
        ["docs", "tooling", "core", "android"]
            .iter()
            .all(|j| ci_jobs.contains(*j)),
        "CI workflow must keep docs, tooling, core and android jobs",
    );
    c.check(
        ci.is_some_and(|v| {
            strings(&v["jobs"]["gate"]["needs"])
                == BTreeSet::from([
                    "docs".to_owned(),
                    "tooling".to_owned(),
                    "core".to_owned(),
                    "android".to_owned(),
                ])
        }),
        "CI Gate job must depend on docs, tooling, core and android",
    );
    // `if: always()` is what makes the gate run after a cancelled or failed
    // dependency; without it the gate is skipped and never turns red.
    c.check(
        ci.is_some_and(|v| {
            v["jobs"]["gate"]["if"]
                .as_str()
                .is_some_and(|s| s.contains("always()"))
        }),
        "CI Gate job must run with if: always()",
    );
    c.check(
        ci.is_some_and(|v| {
            let gate = v["jobs"]["gate"].to_string();
            ["needs.docs.result", "needs.tooling.result"]
                .iter()
                .all(|r| gate.contains(r))
        }),
        "CI Gate must read the dependency results, not assume success",
    );
    let release = flows.get("release");
    let release_triggers = release.map(triggers).unwrap_or_default();
    c.check(
        release_triggers == BTreeSet::from(["push".to_owned()])
            && release.is_some_and(|v| {
                strings(&v["on"]["push"]["tags"]) == BTreeSet::from(["v*".to_owned()])
            }),
        "Release workflow must trigger only on push of tags v*",
    );
    c.check(
        release.is_some_and(|v| v.to_string().contains("release preflight")),
        "Release workflow must run release preflight",
    );
    c.check(
        release.is_some_and(|v| v.to_string().contains("exit 1")),
        "Release workflow must keep a fail-closed publish step (exit 1)",
    );
    // Hardware needs real devices: it must stay manual and out of the gate.
    let quality = flows.get("quality");
    c.check(
        quality.is_some_and(|v| triggers(v) == BTreeSet::from(["workflow_dispatch".to_owned()])),
        "Quality workflow must be manual only (workflow_dispatch)",
    );
    c.check(
        quality.is_some_and(|v| v.to_string().contains("cargo xtask quality-gate")),
        "Quality workflow must run cargo xtask quality-gate",
    );
    Ok(flows.len())
}
pub fn delivery(root: &Path) -> Result<()> {
    let mut c = Checks::default();
    c.check(read_json(root.join(".claude/settings.json"))?["enabledPlugins"]["superpowers@claude-plugins-official"]==true,"Claude plugin not enabled");
    let lock = read_json(root.join("config/superpowers.lock.json"))?;
    c.check(
        lock["required"] == true && lock["expected_versions"]["claude"].is_string(),
        "Superpowers lock missing",
    );
    let github = read_json(root.join("config/github.json"))?;
    c.check(
        github["repository"] == "jimmykedlyn/vynamp-mediaplayer"
            && github["expected_visibility"] == "public",
        "Wrong repository/privacy",
    );
    let cfg = read_json(root.join("config/project.json"))?;
    let keys: BTreeSet<_> = cfg["targets"]
        .as_object()
        .context("Targets")?
        .keys()
        .map(String::as_str)
        .collect();
    c.check(
        keys == BTreeSet::from([
            "windows-x64",
            "linux-x64",
            "macos-intel",
            "macos-silicon",
            "android-arm64",
        ]),
        "Missing platform target",
    );
    let ignore = text(root.join(".gitignore"))?;
    for name in [".worktrees/", ".superpowers/", "*.pfx", "*.jks"] {
        c.check(ignore.contains(name), format!("Missing ignore {name}"));
    }
    let count = github_actions(root, &mut c)?;
    xml(&root.join("packaging/macos/Info.plist"))?;
    crate::branding::references(root)?;
    c.check(
        text(root.join("packaging/windows/installer.nsi"))?.contains("RequestExecutionLevel user"),
        "Unexpected Windows elevation",
    );
    c.check(
        !files(root)?
            .iter()
            .any(|p| p.extension().is_some_and(|e| e == "py")),
        "Python artifact still present",
    );
    println!(
        "{}",
        json!({"scope":"Structural config validation; no remote CI/signing/hardware claim","workflow_files":count,"errors":c.errors,"result":if c.errors.is_empty(){"passed"}else{"failed"}})
    );
    ensure!(c.errors.is_empty(), "Delivery validation failed");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn draft_schema_rejects_invalid_example_and_format() {
        let s = json!({"$schema":"https://json-schema.org/draft/2020-12/schema","type":"object","properties":{"id":{"type":"string","format":"uuid"}},"required":["id"],"additionalProperties":false});
        assert!(schema_example(&s, &json!({"id":"123e4567-e89b-12d3-a456-426614174000"})).is_ok());
        assert!(schema_example(&s, &json!({"id":"bad"})).is_err());
        assert!(schema_example(&s, &json!({"other":1})).is_err());
    }
    #[test]
    fn invalid_schema_rejected() {
        assert!(schema_example(&json!({"type":42}), &json!({})).is_err());
    }
    const SHA: &str = "11bd71901bbe5b1630ceea73d27597364c9af683";
    fn ci_fixture() -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        let w = tmp.path().join(".github/workflows");
        std::fs::create_dir_all(&w).unwrap();
        let ci = format!(
            "name: CI
on:
  pull_request:
    branches: [develop]
  push:
    branches: [develop, main]
jobs:
  docs:
    runs-on: ubuntu-22.04
    steps:
      - uses: actions/checkout@{SHA}
  tooling:
    needs: docs
    runs-on: ubuntu-22.04
    steps:
      - uses: actions/checkout@{SHA}
  core:
    needs: docs
    runs-on: macos-15-intel
    steps:
      - run: cargo xtask ci core --target x86_64-apple-darwin
  core-win:
    needs: docs
    runs-on: windows-2022
    steps:
      - run: cargo xtask ci core --target x86_64-pc-windows-msvc
  core-linux:
    needs: docs
    runs-on: ubuntu-22.04
    steps:
      - run: cargo xtask ci core --target x86_64-unknown-linux-gnu
  core-arm:
    needs: docs
    runs-on: macos-15
    steps:
      - run: cargo xtask ci core --target aarch64-apple-darwin
  android:
    needs: docs
    runs-on: ubuntu-24.04
    steps:
      - run: cargo xtask ci android
  gate:
    name: CI Gate
    if: always()
    needs: [docs, tooling, core, android]
    runs-on: ubuntu-22.04
    steps:
      - run: |
          test needs.docs=${{{{ needs.docs.result }}}} = success
          test needs.tooling=${{{{ needs.tooling.result }}}} = success
"
        );
        let release = format!(
            "name: Release
on:
  push:
    tags: ['v*']
jobs:
  preflight:
    runs-on: ubuntu-22.04
    steps:
      - uses: actions/checkout@{SHA}
      - run: cargo xtask release preflight --version ${{{{ steps.version.outputs.version }}}}\"
  publish:
    runs-on: ubuntu-22.04
    steps:
      - run: |
          echo 'Release publication is not enabled; see ADR-014'
          exit 1
"
        );
        std::fs::write(w.join("ci.yml"), ci).unwrap();
        std::fs::write(w.join("release.yml"), release).unwrap();
        std::fs::write(
            w.join("quality.yml"),
            format!(
                "name: Quality
on:
  workflow_dispatch:
jobs:
  trusted-hardware:
    runs-on: [self-hosted, vynamp-hardware]
    steps:
      - uses: actions/checkout@{SHA}
      - run: cargo xtask quality-gate
"
            ),
        )
        .unwrap();
        tmp
    }
    fn ci_errors(root: &Path) -> Vec<String> {
        let mut c = Checks::default();
        github_actions(root, &mut c).unwrap();
        c.errors
    }
    fn edit(root: &Path, name: &str, from: &str, to: &str) {
        let p = root.join(".github/workflows").join(name);
        let t = std::fs::read_to_string(&p).unwrap();
        assert!(t.contains(from), "fixture lacks {from}");
        std::fs::write(p, t.replace(from, to)).unwrap();
    }
    #[test]
    fn github_actions_config_valid_passes() {
        let f = ci_fixture();
        assert_eq!(ci_errors(f.path()), Vec::<String>::new());
    }
    #[test]
    fn github_actions_rejects_unpinned_action() {
        let f = ci_fixture();
        edit(
            f.path(),
            "ci.yml",
            &format!("actions/checkout@{SHA}"),
            "actions/checkout@v6",
        );
        assert!(
            ci_errors(f.path())
                .iter()
                .any(|e| e.contains("Unpinned action"))
        );
    }
    #[test]
    fn github_actions_rejects_short_sha() {
        let f = ci_fixture();
        edit(
            f.path(),
            "ci.yml",
            &format!("actions/checkout@{SHA}"),
            "actions/checkout@11bd719",
        );
        assert!(
            ci_errors(f.path())
                .iter()
                .any(|e| e.contains("Unpinned action"))
        );
    }
    #[test]
    fn github_actions_rejects_woodpecker() {
        let f = ci_fixture();
        std::fs::create_dir_all(f.path().join(".woodpecker")).unwrap();
        std::fs::write(f.path().join(".woodpecker/linux.yaml"), "when: []\n").unwrap();
        assert!(ci_errors(f.path()).iter().any(|e| e.contains("Woodpecker")));
    }
    #[test]
    fn github_actions_rejects_gate_missing_dependency() {
        let f = ci_fixture();
        edit(
            f.path(),
            "ci.yml",
            "needs: [docs, tooling, core, android]",
            "needs: [docs, tooling, core]",
        );
        assert!(ci_errors(f.path()).iter().any(|e| e.contains("CI Gate")));
    }
    #[test]
    fn github_actions_rejects_gate_skipping_on_failure() {
        let f = ci_fixture();
        edit(f.path(), "ci.yml", "if: always()", "if: success()");
        let e = ci_errors(f.path());
        assert!(e.iter().any(|e| e.contains("always()")), "{e:?}");
    }
    #[test]
    fn github_actions_rejects_gate_ignoring_results() {
        let f = ci_fixture();
        edit(f.path(), "ci.yml", "needs.docs.result", "docs");
        let e = ci_errors(f.path());
        assert!(e.iter().any(|e| e.contains("dependency results")), "{e:?}");
    }
    #[test]
    fn github_actions_requires_platform_targets() {
        let f = ci_fixture();
        edit(f.path(), "ci.yml", "x86_64-apple-darwin", "x");
        edit(f.path(), "ci.yml", "cargo xtask ci android", "true");
        let e = ci_errors(f.path());
        assert!(e.iter().any(|e| e.contains("Missing Android CI")));
        assert!(
            e.iter()
                .any(|e| e.contains("Missing desktop target x86_64-apple-darwin"))
        );
    }
    #[test]
    fn github_actions_requires_release_workflow() {
        let f = ci_fixture();
        std::fs::remove_file(f.path().join(".github/workflows/release.yml")).unwrap();
        assert!(ci_errors(f.path()).iter().any(|e| e.contains("Release")));
    }
    #[test]
    fn github_actions_rejects_release_on_branch_push() {
        let f = ci_fixture();
        edit(
            f.path(),
            "release.yml",
            "tags: ['v*']",
            "branches: [develop]",
        );
        assert!(ci_errors(f.path()).iter().any(|e| e.contains("Release")));
    }
    #[test]
    fn github_actions_rejects_release_without_preflight() {
        let f = ci_fixture();
        edit(f.path(), "release.yml", "release preflight", "version");
        assert!(ci_errors(f.path()).iter().any(|e| e.contains("Release")));
    }
    #[test]
    fn github_actions_rejects_release_publish_not_fail_closed() {
        let f = ci_fixture();
        edit(f.path(), "release.yml", "          exit 1\n", "");
        let e = ci_errors(f.path());
        assert!(e.iter().any(|e| e.contains("fail-closed")), "{e:?}");
    }
}
