use crate::util::*;
use anyhow::{Context, Result, ensure};
use regex::Regex;
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
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
    let all = doc_files(root)?;
    let md: Vec<_> = all
        .iter()
        .filter(|p| p.extension().is_some_and(|e| e == "md"))
        .collect();
    let links = Regex::new(r"\[[^\]]*\]\(([^)]+)\)")?;
    let url = Regex::new(r"^[a-zA-Z][a-zA-Z0-9+.-]*://")?;
    let index = text(root.join("INDEX.md"))?;
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
    let matrix = text(root.join("product/03-requisitos.md"))?;
    let matrixids: Vec<String> = Regex::new(r"(?m)^\| (REQ-E\d{2}-\d{2}) \|")?
        .captures_iter(&matrix)
        .map(|m| m[1].into())
        .collect();
    let matrixset: BTreeSet<_> = matrixids.iter().cloned().collect();
    c.check(
        matrixset == reqset && matrixset.len() == matrixids.len(),
        "Requirement matrix mismatch/duplicates",
    );
    let qa = text(root.join("quality/01-estrategia-testes.md"))?;
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
    let design = text(root.join("design/01-sistema-visual.md"))?;
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
/// SHA and the aggregate gate fails closed on any non-success dependency.
///
/// The umbrella is not an application: it validates the specification, the
/// tooling and the architecture rules, and builds no desktop or Android
/// target (D-CI-1). It also carries no release workflow, because it builds no
/// binary to release (D-CI-3) -- that assertion lives in `archived_release`.
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
    // D-CI-1: the umbrella builds no app. A `cargo xtask ci core` or an
    // Android build here would open a second place where the product proves
    // itself, for a tree that has no app to build.
    c.check(
        !ci.is_some_and(|v| {
            let s = v.to_string();
            s.contains("cargo xtask ci core") || s.contains("cargo xtask ci android")
        }),
        "The umbrella CI must not build an app (D-CI-1); module repos own their builds",
    );
    // D-CI-3: release belongs to the app that publishes a binary.
    c.check(
        !flows.contains_key("release"),
        "The umbrella must not carry a release workflow (D-CI-3); the app repo publishes",
    );
    // The required status check is the job id `gate`; a renamed or narrowed
    // gate would stop blocking merges without failing anything else.
    c.check(ci_jobs.contains("gate"), "CI Gate job absent from ci.yml");
    const UMBRELLA_JOBS: [&str; 4] = ["specs", "tooling", "submodules", "arch"];
    c.check(
        UMBRELLA_JOBS.iter().all(|j| ci_jobs.contains(*j)),
        "CI workflow must keep specs, tooling, submodules and arch jobs",
    );
    c.check(
        ci.is_some_and(|v| {
            strings(&v["jobs"]["gate"]["needs"])
                == UMBRELLA_JOBS.iter().map(|j| (*j).to_owned()).collect()
        }),
        "CI Gate job must depend on specs, tooling, submodules and arch",
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
            ["needs.specs.result", "needs.tooling.result"]
                .iter()
                .all(|r| gate.contains(r))
        }),
        "CI Gate must read the dependency results, not assume success",
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

/// The umbrella publishes no binary, so its `release.yml` was archived in
/// `_arquivo/workflows/` (D-CI-3). That archive is the source the application
/// repos inherit, so it keeps being validated: a release must start only from
/// a `v*` tag and must stay fail-closed before publishing. Absent is valid --
/// the archive is scratch that step 38 removes.
fn archived_release(root: &Path, c: &mut Checks) -> Result<()> {
    let path = root.join("_arquivo/workflows/release.yml");
    if !path.is_file() {
        return Ok(());
    }
    let v: Value = serde_yaml::from_str(&text(&path)?)?;
    c.check(
        triggers(&v) == BTreeSet::from(["push".to_owned()])
            && strings(&v["on"]["push"]["tags"]) == BTreeSet::from(["v*".to_owned()]),
        "Archived release workflow must trigger only on push of tags v*",
    );
    c.check(
        v.to_string().contains("release preflight"),
        "Archived release workflow must run release preflight",
    );
    c.check(
        v.to_string().contains("exit 1"),
        "Archived release workflow must keep a fail-closed publish step (exit 1)",
    );
    Ok(())
}
const AGENT_MODELS: [&str; 3] = ["opus", "sonnet", "haiku"];
const AGENT_KEYS: [&str; 4] = ["name", "description", "model", "tools"];

/// Validates every subagent definition in `.claude/agents/`: an opening `---`, a closing
/// `---`, the four required keys, a `model:` from the known set, and no repeated key.
fn agent_definitions(root: &Path, c: &mut Checks) {
    let dir = root.join(".claude/agents");
    if !dir.is_dir() {
        c.check(false, "Missing .claude/agents/");
        return;
    }
    let mut seen = std::collections::BTreeSet::new();
    let Ok(entries) = fs::read_dir(&dir) else {
        c.check(false, ".claude/agents/ unreadable");
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_none_or(|e| e != "md") {
            continue;
        }
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        seen.insert(name.clone());
        let Ok(body) = text(&path) else {
            c.check(false, format!("Agent definition unreadable: {name}"));
            continue;
        };
        let lines: Vec<&str> = body.lines().collect();
        if lines.first() != Some(&"---") {
            c.check(false, format!("Agent without frontmatter: {name}"));
            continue;
        }
        let Some(close) = lines.iter().skip(1).position(|l| *l == "---") else {
            c.check(false, format!("Agent frontmatter unclosed: {name}"));
            continue;
        };
        let mut found = std::collections::BTreeSet::new();
        for line in &lines[1..=close] {
            let Some((key, value)) = line.split_once(':') else {
                continue;
            };
            let key = key.trim();
            let value = value.trim();
            // Continuation lines of a folded description start with a space and have no
            // key; they must not be mistaken for one.
            if line.starts_with(' ') || key.contains(' ') {
                continue;
            }
            if !AGENT_KEYS.contains(&key) {
                c.check(false, format!("Agent {name}: unknown key '{key}'"));
                continue;
            }
            if !found.insert(key.to_string()) {
                c.check(false, format!("Agent {name}: repeated key '{key}'"));
            }
            if key == "model" && !AGENT_MODELS.contains(&value) {
                c.check(false, format!("Agent {name}: invalid model '{value}'"));
            }
            if key != "model" && value.is_empty() {
                c.check(false, format!("Agent {name}: empty '{key}'"));
            }
        }
        for key in AGENT_KEYS {
            c.check(
                found.contains(key),
                format!("Agent {name}: missing '{key}'"),
            );
        }
    }
    for required in [
        "orquestrador.md",
        "implementador.md",
        "checador.md",
        "revisor.md",
    ] {
        c.check(
            seen.contains(required),
            format!("Missing agent definition: {required}"),
        );
    }
}

pub fn delivery(root: &Path) -> Result<()> {
    // Packaging belongs to the app that ships it, not to the umbrella, and a repo without
    // a binary has nothing to install. D-CI-2 of step 34 removed the installer, branding
    // and packaging checks from this gate. packaging/ and assets/branding/ still exist
    // here on purpose: they are the source the app repos inherit, not this repo's own
    // release inputs. The three reference PNGs, however, are in _arquivo/ and the manifest
    // is checked against them there (branding::references).
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
    archived_release(root, &mut c)?;
    // A malformed `model:` makes a subagent definition silently fail to load, and
    // check-docs no longer scans .claude/ -- so the gate lives here, where it is the
    // only thing left watching those files.
    agent_definitions(root, &mut c);
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
        // The umbrella owns no app: it validates specs, the tooling and the
        // architecture rules, and builds no desktop or Android target. See
        // D-CI-1 and D-CI-3 in 34-ci.md.
        let ci = format!(
            "name: CI
on:
  pull_request:
    branches: [develop]
  push:
    branches: [develop, main]
jobs:
  specs:
    runs-on: ubuntu-22.04
    steps:
      - uses: actions/checkout@{SHA}
      - run: cargo xtask check-docs
  tooling:
    needs: specs
    runs-on: ubuntu-22.04
    steps:
      - uses: actions/checkout@{SHA}
  submodules:
    needs: specs
    runs-on: ubuntu-22.04
    steps:
      - run: git submodule status --recursive
  arch:
    needs: specs
    runs-on: ubuntu-22.04
    steps:
      - run: cargo build --manifest-path libs/decode/Cargo.toml
  gate:
    name: CI Gate
    if: always()
    needs: [specs, tooling, submodules, arch]
    runs-on: ubuntu-22.04
    steps:
      - run: |
          test needs.specs=${{{{ needs.specs.result }}}} = success
          test needs.tooling=${{{{ needs.tooling.result }}}} = success
"
        );
        std::fs::write(w.join("ci.yml"), ci).unwrap();
        // No `release.yml`: the umbrella publishes no binary (D-CI-3). The
        // release assertions live in `release_errors`, checked against a
        // workflow that does not belong to this repository.
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
    fn agent_fixture(model: &str, extra: &str) -> (tempfile::TempDir, std::path::PathBuf) {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join(".claude/agents");
        std::fs::create_dir_all(&dir).unwrap();
        for name in ["orquestrador", "implementador", "checador", "revisor"] {
            std::fs::write(
                dir.join(format!("{name}.md")),
                format!(
                    "---
name: {name}
description: d
model: {model}
tools: Read{extra}
---
body
"
                ),
            )
            .unwrap();
        }
        (tmp, dir)
    }
    fn agent_errors(root: &Path) -> Vec<String> {
        let mut c = Checks::default();
        agent_definitions(root, &mut c);
        c.errors
    }
    #[test]
    fn agent_definitions_accept_the_four_known_models() {
        let (tmp, _) = agent_fixture("sonnet", "");
        assert!(agent_errors(tmp.path()).is_empty());
    }
    #[test]
    fn agent_definitions_reject_invalid_model() {
        let (tmp, _) = agent_fixture("opuss", "");
        let e = agent_errors(tmp.path());
        assert!(
            e.iter().any(|e| e.contains("invalid model 'opuss'")),
            "{e:?}"
        );
    }
    #[test]
    fn agent_definitions_reject_missing_key_and_unclosed_frontmatter() {
        let (tmp, dir) = agent_fixture("sonnet", "");
        std::fs::write(
            dir.join("revisor.md"),
            "---
name: revisor
model: sonnet
",
        )
        .unwrap();
        let e = agent_errors(tmp.path());
        assert!(
            e.iter().any(|e| e.contains("frontmatter unclosed")),
            "{e:?}"
        );
        std::fs::write(
            dir.join("revisor.md"),
            "no frontmatter at all
",
        )
        .unwrap();
        let e = agent_errors(tmp.path());
        assert!(e.iter().any(|e| e.contains("without frontmatter")), "{e:?}");
    }
    #[test]
    fn agent_definitions_report_a_missing_definition() {
        let (tmp, dir) = agent_fixture("sonnet", "");
        std::fs::remove_file(dir.join("checador.md")).unwrap();
        let e = agent_errors(tmp.path());
        assert!(
            e.iter()
                .any(|e| e.contains("Missing agent definition: checador.md")),
            "{e:?}"
        );
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
            "needs: [specs, tooling, submodules, arch]",
            "needs: [specs, tooling, submodules]",
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
        edit(f.path(), "ci.yml", "needs.specs.result", "specs");
        let e = ci_errors(f.path());
        assert!(e.iter().any(|e| e.contains("dependency results")), "{e:?}");
    }
    #[test]
    fn github_actions_rejects_release_workflow_in_the_umbrella() {
        // D-CI-3: release belongs to the app that publishes a binary. A
        // `release.yml` here could only fail at its preflight.
        let f = ci_fixture();
        std::fs::write(
            f.path().join(".github/workflows/release.yml"),
            "name: Release
on:
  push:
    tags: ['v*']
jobs:
  publish:
    runs-on: ubuntu-22.04
    steps:
      - run: exit 1
",
        )
        .unwrap();
        let e = ci_errors(f.path());
        assert!(
            e.iter()
                .any(|e| e.contains("must not carry a release workflow")),
            "{e:?}"
        );
    }
    #[test]
    fn github_actions_rejects_app_builds_in_the_umbrella() {
        // D-CI-1: an app build here would open a second place where the
        // product proves itself, and it would consume the module repos'
        // matrix for a tree that has no app.
        let f = ci_fixture();
        edit(
            f.path(),
            "ci.yml",
            "  arch:",
            "  core:
    runs-on: macos-15-intel
    steps:
      - run: cargo xtask ci core --target x86_64-apple-darwin
  arch:",
        );
        let e = ci_errors(f.path());
        assert!(
            e.iter().any(|e| e.contains("must not build an app")),
            "{e:?}"
        );
    }
    #[test]
    fn github_actions_requires_the_umbrella_jobs() {
        let f = ci_fixture();
        edit(
            f.path(),
            "ci.yml",
            "needs: [specs, tooling, submodules, arch]",
            "needs: [specs, tooling, submodules]",
        );
        let e = ci_errors(f.path());
        assert!(e.iter().any(|e| e.contains("CI Gate")), "{e:?}");
    }
    #[test]
    fn archived_release_workflow_stays_valid_for_the_app_to_copy() {
        // D-CI-3 moved release.yml to `_arquivo/workflows/`, where it is the
        // source the app repos inherit. The rules did not disappear with it:
        // a mangled archive would poison every app that copies from it.
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("_arquivo/workflows");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("release.yml"),
            "name: Release
on:
  push:
    branches: [develop]
jobs:
  publish:
    runs-on: ubuntu-22.04
    steps:
      - run: |
          echo 'publishing'
",
        )
        .unwrap();
        let mut c = Checks::default();
        archived_release(tmp.path(), &mut c).unwrap();
        let e = c.errors;
        assert!(e.iter().any(|e| e.contains("tags v*")), "{e:?}");
        assert!(e.iter().any(|e| e.contains("fail-closed")), "{e:?}");
    }
    #[test]
    fn archived_release_is_optional() {
        let tmp = tempfile::tempdir().unwrap();
        let mut c = Checks::default();
        archived_release(tmp.path(), &mut c).unwrap();
        assert!(c.errors.is_empty(), "{:?}", c.errors);
    }
}
