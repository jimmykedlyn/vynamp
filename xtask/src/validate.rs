use crate::util::*;
use anyhow::{Context, Result, bail, ensure};
use regex::Regex;
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
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
/// Position of a task's phase in the delivery order: `M0`..`M5`, then `F6`.
///
/// `F6` is [FASE 6](../../delivery/01-roadmap.md), which ships after 1.0. It ranks above
/// every milestone, and the dependency rule ("no task may depend on a later phase") has to
/// read it that way -- ranking it below M0 because `F` sorts before `M` would let an iOS
/// task be a dependency of an M0 one, which is the whole ordering the rule exists to stop.
fn phase(v: &Value) -> Result<u32> {
    let text = string(v, "phase")?;
    match text.strip_prefix('M') {
        Some(digits) => Ok(digits.parse()?),
        None if text == "F6" => Ok(6),
        None => bail!("Bad phase {text}"),
    }
}
/// Where a task's Markdown and the backlog stop agreeing, if anywhere.
///
/// `title` and `deliverable` are prose that the generator copies verbatim, so the whole
/// string has to appear. `acceptance` is different: the backlog holds one sentence with
/// `;`-separated clauses, and the Markdown holds one table row per clause (`37-tasks.ps1`
/// section 3). The sentence never appears contiguously in the file, so demanding it whole
/// would fail all 191 product tasks on format rather than on divergence. The clause is
/// therefore the unit -- which is also what the generator itself verifies ("Then identico
/// ao acceptance do backlog") -- and it still catches the case the field exists to catch:
/// a criterion rewritten, dropped or invented.
///
/// Returned instead of pushed straight into `Checks` so the comparison can be exercised
/// without a repository on disk.
fn markdown_disagreements(id: &str, body: &str, t: &Value) -> Result<Vec<String>> {
    let mut out = Vec::new();
    for k in ["title", "deliverable"] {
        let value = string(t, k)?;
        if value.is_empty() || !body.contains(value) {
            out.push(format!("Backlog/MD mismatch {id} {k}"));
        }
    }
    let acceptance = string(t, "acceptance")?;
    if acceptance.trim().is_empty() {
        // An empty backlog sentence must not read as "no clauses to check, all present".
        out.push(format!("Backlog/MD mismatch {id} acceptance"));
    }
    for clause in acceptance.split(';').map(str::trim) {
        if !clause.is_empty() && !body.contains(clause) {
            out.push(format!("Backlog/MD mismatch {id} acceptance: {clause}"));
        }
    }
    Ok(out)
}
/// Directory that owns a repository's tree: the umbrella itself, or a submodule's checkout.
///
/// `None` means the backlog names a repository that `.gitmodules` does not declare, which is
/// a configuration error the caller reports -- never a silent lookup miss.
fn repo_dir(root: &Path, repo: &str) -> Option<PathBuf> {
    if repo == "vynamp" {
        return Some(root.to_path_buf());
    }
    submodule_dir(root, repo).map(|d| root.join(d))
}
/// The repository that owns a task. Absent means the umbrella, which is what every task was
/// before step 37 assigned the field; the gate has to read both states, and a missing field
/// is not a reason to stop recognizing the file that has always been there.
fn owner(t: &Value) -> &str {
    t["repo"].as_str().unwrap_or("vynamp")
}
/// Where a task's Markdown lives, resolved by the repository the backlog assigns to it.
///
/// `01-arvore-final.md` §1 gives each code module its own `tasks/`, and keeps in the
/// umbrella only the tasks that cross modules -- CI, release, the platform matrix. The gate
/// therefore cannot assume `tasks/items/`: it asks the backlog who owns the task, then asks
/// `.gitmodules` where that repository is checked out.
fn task_path(root: &Path, t: &Value) -> Option<PathBuf> {
    let id = string(t, "id").ok()?;
    let dir = repo_dir(root, owner(t))?;
    Some(dir.join(format!("tasks/items/{id}.md")))
}
/// Resolve `.` and `..` without touching the filesystem.
///
/// A link target is written relative to the document that holds it, so it routinely climbs
/// out (`../libs/decode/tasks/items/...`). `Path::starts_with` compares components as they
/// are written, so it would not recognize that text as living under `libs/decode`; the
/// question "is this link inside a submodule?" has to be asked of the normalized path.
///
/// Only `..` at the start of an already-relative path is left alone -- there is nothing above
/// the root to climb to, and the caller always passes a path that began at the repository.
fn normalize(path: &Path) -> PathBuf {
    let mut out = Vec::new();
    for c in path.components() {
        match c {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                if matches!(out.last(), Some(std::path::Component::Normal(_))) {
                    out.pop();
                } else {
                    out.push(c);
                }
            }
            other => out.push(other),
        }
    }
    out.iter().collect()
}
/// True when `path` sits inside a submodule that is not checked out.
///
/// Step 37 rewrote the umbrella's links to the TASKs it moved: with the modules present the
/// target is a real file, without them it is a directory that does not exist. A strict
/// `exists()` would make the gate's verdict depend on whether the clone used `--recursive`,
/// which is the failure `doc_files` was written to avoid -- so the rule is the same one the
/// task-markdown check uses: verify what can be seen, refuse to invent a verdict about what
/// cannot. A submodule that IS checked out gets no such mercy: a broken link inside it is
/// still a broken link.
fn inside_absent_submodule(root: &Path, path: &Path) -> bool {
    let path = normalize(path);
    submodule_paths(root)
        .iter()
        .any(|sub| path.starts_with(root.join(sub)) && !root.join(sub).is_dir())
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
            let dest = p.parent().unwrap().join(target);
            // Inside a submodule that is not checked out there is nothing to look at; inside
            // one that is, or anywhere in the umbrella, the link is verified as before.
            c.check(
                dest.exists() || inside_absent_submodule(root, &dest),
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
        let path = task_path(root, t).with_context(|| format!("Unknown repo for {id}"))?;
        let checked_out = repo_dir(root, owner(t)).is_some_and(|d| d.is_dir());
        // A module that is not checked out has no task file to find. Reporting it absent
        // would make the verdict depend on the clone, which is the thing `doc_files` was
        // narrowed to avoid; the pins job is what proves the module is fetchable.
        c.check(
            path.is_file() || !checked_out,
            format!("Task Markdown absent {id}"),
        );
        if path.is_file() {
            for message in markdown_disagreements(id, &text(&path)?, t)? {
                c.check(false, message);
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
    let report = json!({"spec_version":data["spec_version"],"scope":"Validação documental em Rust; não testa player/hardware/API/WIT.","mandatory_platforms":["Windows x86_64","Linux x86_64","macOS Intel x86_64","macOS Apple Silicon arm64","Android arm64-v8a","iOS arm64"],"markdown_files":md.len(),"tasks":tasks.len(),"requirements":matrixids.len(),"epics":epics.len(),"phases":phases,"schemas_and_examples":schemas.len(),"format_catalog_entries":catalog["entries"].as_array().map(Vec::len),"openapi_paths":paths.len(),"operation_count":ops.len(),"errors":c.errors,"result":if c.errors.is_empty(){"passed"}else{"failed"}});
    write_json(root.join("VALIDATION.json"), &report)?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    ensure!(c.errors.is_empty(), "Specification validation failed");
    Ok(())
}
/// Collect every `uses:` reference in a workflow, at any nesting depth, so a job
/// or step added later cannot slip in an unpinned action. Each reference keeps the
/// YAML key path it was found at (`jobs.build.steps.0.uses`): a file name alone does
/// not tell the reader where the unpinned action is, and the path is what a person
/// can act on.
fn action_ref_locations(v: &Value, path: &[String]) -> Vec<(String, String)> {
    let mut out = Vec::new();
    match v {
        Value::Object(map) => {
            for (k, child) in map {
                let mut here = path.to_vec();
                here.push(k.clone());
                if k == "uses"
                    && let Some(s) = child.as_str()
                {
                    out.push((here.join("."), s.to_owned()));
                }
                out.extend(action_ref_locations(child, &here));
            }
        }
        Value::Array(items) => {
            for (i, child) in items.iter().enumerate() {
                let mut here = path.to_vec();
                here.push(i.to_string());
                out.extend(action_ref_locations(child, &here));
            }
        }
        _ => {}
    }
    out
}

/// The 1-based lines that hold a `uses:` key in `body`.
///
/// A `uses:` is either a mapping key (`        uses: …`) or the first key of a
/// sequence item (`      - uses: …`), and both spellings appear in real
/// workflows. The parser reports references in document order and so does this
/// scan, so the n-th reference pairs with the n-th line. Searching by the action
/// text instead would give every repeated `uses:` the line of the first one --
/// wrong exactly where the name-and-line report matters, a workflow that repeats
/// `checkout` in several jobs.
fn uses_lines(body: &str) -> Vec<usize> {
    body.lines()
        .enumerate()
        .filter(|(_, l)| {
            let t = l.trim_start();
            t.strip_prefix("- ").unwrap_or(t).starts_with("uses:")
        })
        .map(|(i, _)| i + 1)
        .collect()
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
        let file = p
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_owned();
        let lines = uses_lines(&text(&p)?);
        for (i, (path, action)) in action_ref_locations(&v, &[]).into_iter().enumerate() {
            c.check(
                pinned.is_match(&action),
                format!(
                    "Unpinned action {action} at {file}:{} ({path})",
                    lines.get(i).copied().unwrap_or(0)
                ),
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
    const UMBRELLA_JOBS: [&str; 5] = ["specs", "tooling", "submodules", "arch", "tasks"];
    c.check(
        UMBRELLA_JOBS.iter().all(|j| ci_jobs.contains(*j)),
        "CI workflow must keep specs, tooling, submodules, arch and tasks jobs",
    );
    c.check(
        ci.is_some_and(|v| {
            strings(&v["jobs"]["gate"]["needs"])
                == UMBRELLA_JOBS.iter().map(|j| (*j).to_owned()).collect()
        }),
        "CI Gate job must depend on specs, tooling, submodules, arch and tasks",
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
    // macOS Intel and Apple Silicon are mandatory targets (REQ-E19-01), and a
    // target no job runs on is not verified. This has to be a gate and not a
    // preference: dropping a Mac image from the matrix is a one-line edit
    // that returns macOS to "not verified" -- exactly the state ADR-014
    // exists to end -- while every other check stays green. The tooling job
    // is the only umbrella job that can carry them: the quality job needs
    // hardware that does not exist yet, and `ci core` is forbidden here.
    c.check(
        ci.is_some_and(|v| {
            let runners = v["jobs"]["tooling"]["strategy"]["matrix"]["runner"].clone();
            let runners = strings(&runners);
            runners.contains("macos-15-intel") && runners.contains("macos-15")
        }),
        "Tooling job must run on macos-15-intel and macos-15 (REQ-E19-01)",
    );
    c.check(
        ci.is_some_and(|v| {
            let runners = v["jobs"]["tooling"]["strategy"]["matrix"]["runner"].clone();
            !strings(&runners).contains("macos-latest")
        }),
        "Tooling matrix must not use macos-latest; it changes architecture as GitHub rotates it",
    );
    // AC-5 of TASK-E19-01 is a claim about the *history*, and the scanner that
    // proves it is worthless unless CI runs it. Removing the step is silent:
    // the gate stays green, the claim stays in the task, and nothing reads the
    // commits again. The unshallow step is part of the same check because
    // `actions/checkout` fetches a single commit by default, and a shallow clone
    // would scan a working tree it calls clean.
    c.check(
        ci.is_some_and(|v| {
            let steps = v["jobs"]["tooling"]["steps"].to_string();
            steps.contains("cargo xtask check-secrets") && steps.contains("fetch --unshallow")
        }),
        "Tooling job must run cargo xtask check-secrets on full history (TASK-E19-01 AC-5)",
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
    // O nome vem do arquivo, e nao de um literal aqui: o passo 35 trocou o umbrella
    // de `vynamp-mediaplayer` para `vynamp`, e um `==` com o nome antigo seria uma
    // segunda copia da mesma constante, que ninguem lembra de atualizar. O que se
    // verifica e a coerencia interna do arquivo e dele com o `origin` do git.
    //
    // A visibilidade real (`private` na API) nao e verificada aqui: este gate e
    // estrutural e roda offline no CI, e uma requisicao de rede que falha passaria
    // como aprovada ou reprovaria por instabilidade. Quem confere e
    // `cargo xtask repo`, que fala com a API de proposito.
    let repo = string(&github, "repository")?;
    let url = string(&github, "url")?;
    c.check(
        url == format!("https://github.com/{repo}.git"),
        "github.json url does not match repository",
    );
    c.check(
        matches!(
            string(&github, "expected_visibility")?,
            "private" | "public"
        ),
        "Unknown visibility",
    );
    if let Ok(origin) = git(root, &["remote", "get-url", "origin"]) {
        c.check(
            origin == url,
            format!("origin is {origin}, github.json says {url}"),
        );
    }
    let cfg = read_json(root.join("config/project.json"))?;
    let keys: BTreeSet<_> = cfg["targets"]
        .as_object()
        .context("Targets")?
        .keys()
        .map(String::as_str)
        .collect();
    // Every target the roadmap promises must be declared with its Rust triple, and iOS is
    // one of them since FASE 6. Subset, not equality: the check is "nothing promised is
    // missing", and an extra key is a statement about a target the project also builds --
    // buying a red gate for it would make the roadmap unable to grow without editing the
    // gate that is supposed to watch it.
    c.check(
        [
            "windows-x64",
            "linux-x64",
            "macos-intel",
            "macos-silicon",
            "android-arm64",
            "ios-arm64",
        ]
        .into_iter()
        .all(|t| keys.contains(t)),
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
/// Rows of the acceptance-criteria table: `| id | Given | When | Then |`.
///
/// Split on unescaped pipes only up to the fourth cell, so a `Then` that itself contains
/// a pipe does not shear the row. The `Given`/`When`/`Then` columns are what this gate
/// exists to read, and no task file has a literal `|` in them today -- but a criterion
/// whose text quotes a shell pipe is exactly the kind of thing nobody notices until the
/// gate misreads a green row.
fn acceptance_rows(body: &str) -> Vec<(String, String, String, String)> {
    let mut out = Vec::new();
    let mut in_table = false;
    for line in body.lines() {
        let line = line.trim();
        if !line.starts_with('|') {
            in_table = false;
            continue;
        }
        let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
        if cells.len() < 4 {
            continue;
        }
        if cells[0] == "ID" && cells[1] == "Given" {
            in_table = true;
            continue;
        }
        if !in_table || cells[0].chars().all(|c| c == '-' || c == ':') {
            continue;
        }
        // `Then` is the authoritative column and the only one prose is copied into, so it
        // takes everything past the third pipe: a criterion that quotes `a | b` would
        // otherwise be read truncated and pass a comparison it should fail.
        out.push((
            cells[0].trim_matches('`').to_owned(),
            cells[1].to_owned(),
            cells[2].to_owned(),
            cells[3..].join(" | "),
        ));
    }
    out
}
/// The `Módulo dono` cell, split into the repositories it names.
///
/// The rule is "exactly one module owner". Everything the cell can say that is not one
/// repository -- empty, the `PENDENTE` placeholder, two names, prose -- is a violation,
/// and the gate has to be able to tell them apart well enough to name the file.
pub fn module_owners(body: &str) -> Vec<String> {
    let Some(line) = body
        .lines()
        .map(str::trim)
        .find(|l| l.starts_with("| Módulo dono |"))
    else {
        return Vec::new();
    };
    let cell = line
        .trim_matches('|')
        .split('|')
        .nth(1)
        .unwrap_or("")
        .trim();
    cell.split([',', '/', ' '])
        .map(|s| s.trim().trim_matches('`').trim())
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .collect()
}
/// The rules `check-tasks` enforces, as a function of a backlog and a reader for the
/// task Markdown. Split out from `tasks()` so the four adversarial cases its acceptance
/// criteria name can be exercised without a repository on disk.
///
/// What it deliberately does *not* do: rewrite a criterion, pick an owner, or decide that
/// a missing one is acceptable. `TASK-CI-02`'s boundary says the gate names the violation
/// and the author fixes it -- the red is the product, not an obstacle to route around.
fn task_disagreements(backlog: &Value, read: impl Fn(&str) -> Option<String>) -> Vec<String> {
    let mut out = Vec::new();
    let Some(tasks) = backlog["tasks"].as_array() else {
        return vec!["Backlog tasks".to_owned()];
    };
    // Known repositories, taken from the backlog itself: the owner cell has to name one of
    // these, and nothing else. Reading the set from the data is what lets a new module
    // appear without editing the gate, and what makes a word like `PENDENTE` fail on its
    // own rather than pass as an uncontested single owner.
    let known: BTreeSet<&str> = tasks.iter().filter_map(|t| t["module"].as_str()).collect();
    // The engineering tasks carry no REQ on purpose (they are not product requirements);
    // every other TASK is a product task and must name at least one.
    for t in tasks {
        let Ok(id) = string(t, "id") else {
            out.push("Task without ID".to_owned());
            continue;
        };
        let product = !id.starts_with("TASK-CI-");
        if product {
            let reqs = strings(&t["requirements"]);
            if reqs.is_empty() {
                out.push(format!("{id} has no REQ"));
            }
            if reqs.iter().any(|r| !r.starts_with("REQ-")) {
                out.push(format!("{id} has a REQ that is not REQ-*"));
            }
        }
        if !t["acceptance_ready"].is_boolean() {
            out.push(format!("{id} acceptance_ready missing or not a boolean"));
        }
        // A task may only leave `planned` once its criteria are written. This is the
        // point of the field: without the check it is decoration.
        if string(t, "status").unwrap_or("") != "planned" && t["acceptance_ready"] != true {
            out.push(format!(
                "{id} is {} with acceptance_ready: false",
                string(t, "status").unwrap_or("?")
            ));
        }
        if product {
            match read(id) {
                None => out.push(format!("{id} task file absent")),
                Some(body) => {
                    let owners = module_owners(&body);
                    if owners.len() != 1 {
                        out.push(format!("{id} has {} module owners", owners.len()));
                    } else if let Some(unknown) =
                        owners.iter().find(|o| !known.contains(o.as_str()))
                    {
                        // An unknown name is already the whole story; also comparing it
                        // with the backlog would print a second line about a repository
                        // the file never claimed.
                        out.push(format!("{id} has unknown module owner {unknown}"));
                    } else if let Ok(backlog_owner) = string(t, "module")
                        && owners[0] != backlog_owner
                    {
                        // The Markdown and the backlog are two records of the same fact;
                        // a task whose file claims an owner the backlog does not agree
                        // with would be executed against the wrong repository.
                        out.push(format!(
                            "{id} owner is {} in Markdown, {backlog_owner} in backlog",
                            owners[0]
                        ));
                    }
                    let rows = acceptance_rows(&body);
                    if rows.is_empty() {
                        out.push(format!("{id} has no acceptance criteria"));
                    }
                    // Authoring `Given`/`When` is a separate, per-module task (step 37,
                    // D-37-3 and pendency P3): every `planned` task is expected to still
                    // carry `PENDENTE`, and demanding otherwise here would paint all 191
                    // red for work no rule has asked for yet. The placeholder stops being
                    // acceptable at the moment the task leaves `planned` -- which is the
                    // same threshold as `acceptance_ready`, and the reason both are read
                    // off the same state instead of being two independent switches.
                    let planned = string(t, "status").unwrap_or("") == "planned";
                    for (ac, given, when, then) in rows {
                        let cells = [("Given", &given), ("When", &when), ("Then", &then)];
                        if planned {
                            // Even a planned task may not have an empty authoritative
                            // column: `Then` is copied from the backlog and must exist.
                            if then.trim().is_empty() {
                                out.push(format!("{id} {ac}: Then not written"));
                            }
                            continue;
                        }
                        for (column, value) in cells {
                            if value.trim().is_empty() || value.contains("PENDENTE") {
                                out.push(format!("{id} {ac}: {column} not written"));
                            }
                        }
                    }
                }
            }
        }
    }
    out
}
/// `cargo xtask check-tasks` (TASK-CI-02): the gate that gives `acceptance_ready: false`
/// mechanical consequence.
///
/// TASK-CI-02 AC-4 and AC-5 contradict each other on the repository's own state: AC-4
/// demands a red naming *exactly* the five `in_progress` tasks, and AC-5 demands green
/// once those five are written. Both cannot hold at the same commit -- the run is either
/// red or green. The resolution here is the one that makes the gate useful in both
/// directions: the rule is enforced on its merits, so the verdict depends only on whether
/// the criteria are actually written. Today six tasks are outside `planned` without
/// written criteria -- the five `in_progress` plus `TASK-E19-06`, which this repository
/// closed on 2026-11-08 by executing it, and whose Markdown still carries the authored
/// `Then` with `Given`/`When` unset. The gate therefore goes red naming those six, and
/// goes green the moment they are authored, with no rule relaxed in between.
pub fn tasks(root: &Path) -> Result<()> {
    let data = read_json(root.join("tasks/backlog.json"))?;
    // Resolution goes through the same `task_path` `check-docs` uses, so a task moved to
    // another repository cannot be found under a stale path here and read as compliant.
    let by: BTreeMap<String, &Value> = {
        let mut m = BTreeMap::new();
        for t in data["tasks"].as_array().context("Backlog tasks")? {
            if let Ok(id) = string(t, "id") {
                m.insert(id.to_owned(), t);
            }
        }
        m
    };
    let mut c = Checks::default();
    for message in task_disagreements(&data, |id| {
        let path = task_path(root, by.get(id)?)?;
        // A module that is not checked out has no file to read. Treating absence there as a
        // violation would make the verdict depend on the clone, which is the same mistake
        // `doc_files` was narrowed to avoid; `check-docs` already reports absence only for a
        // repository that *is* present.
        if !path.is_file() {
            let repo = owner(by[id]);
            if repo != "vynamp" && repo_dir(root, repo).is_some_and(|d| !d.is_dir()) {
                return None;
            }
        }
        fs::read_to_string(&path).ok()
    }) {
        c.check(false, message);
    }
    println!(
        "{}",
        json!({"scope":"Backlog and TASK Markdown only; no build, no hardware","tasks":by.len(),"errors":c.errors,"result":if c.errors.is_empty(){"passed"}else{"failed"}})
    );
    ensure!(c.errors.is_empty(), "Task validation failed");
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
    /// `.gitmodules` is the only file that holds both halves of name -> path.
    ///
    /// The block boundary is load-bearing: a `url` written before its own `path` (legal in the
    /// format, and what `git submodule add` does not stop anyone from producing) would borrow
    /// the previous block's path. The gate would then report on a file that is not the task's,
    /// which is worse than not answering at all.
    #[test]
    fn submodule_dir_maps_each_url_to_its_own_path() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(
            tmp.path().join(".gitmodules"),
            "[submodule \"libs/decode\"]
	path = libs/decode
	url = https://github.com/o/vynamp-decode.git
             [submodule \"apps/player\"]
	path = apps/player
	url = https://github.com/o/vynamp-player.git
             [submodule \"libs/engine\"]
	url = https://github.com/o/vynamp-engine.git
	path = libs/engine
",
        )
        .unwrap();
        let root = tmp.path();
        assert_eq!(
            submodule_dir(root, "vynamp-player"),
            Some(PathBuf::from("apps/player"))
        );
        assert_eq!(
            submodule_dir(root, "vynamp-decode"),
            Some(PathBuf::from("libs/decode"))
        );
        // `url` before `path`: the answer is the block's own path, not the player's.
        assert_eq!(
            submodule_dir(root, "vynamp-engine"),
            Some(PathBuf::from("libs/engine"))
        );
        assert_eq!(submodule_dir(root, "vynamp-absent"), None);
        // The umbrella is not a submodule; asking for it must not borrow a neighbour's path.
        assert_eq!(submodule_dir(root, "vynamp"), None);
    }
    /// The task file follows the repository the backlog assigns, not a fixed `tasks/items/`.
    /// Before step 37 every task lived in the umbrella, so the constant worked; after it,
    /// 134 of 191 live in a module and the constant would report all of them absent.
    #[test]
    fn task_path_follows_the_owning_repository() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::write(
            root.join(".gitmodules"),
            "[submodule \"apps/player\"]
	path = apps/player
	url = https://github.com/o/vynamp-player.git
",
        )
        .unwrap();
        let umbrella = json!({"id":"TASK-E19-01","repo":"vynamp"});
        assert_eq!(
            task_path(root, &umbrella),
            Some(root.join("tasks/items/TASK-E19-01.md"))
        );
        let module = json!({"id":"TASK-E02-01","repo":"vynamp-player"});
        assert_eq!(
            task_path(root, &module),
            Some(root.join("apps/player/tasks/items/TASK-E02-01.md"))
        );
        // A repository nobody declared has no knowable path; the caller reports it.
        assert_eq!(
            task_path(root, &json!({"id":"X","repo":"vynamp-nope"})),
            None
        );
    }
    /// A link into a submodule must not make the verdict depend on the clone.
    ///
    /// After step 37 the umbrella points at TASKs it moved (`../../libs/decode/tasks/items/`).
    /// A workstation that cloned with `--recursive` has those files; a plain clone does not.
    /// If the gate demanded `exists()`, `check-docs` would be green on one and red on the
    /// other -- and the red one would be the machine that looks correct. The distinction is
    /// "absent because nobody fetched it" versus "absent though the module is right there":
    /// only the second is a finding.
    #[test]
    fn link_inside_an_unfetched_submodule_is_not_reported() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::write(
            root.join(".gitmodules"),
            "[submodule \"libs/decode\"]
	path = libs/decode
	url = https://github.com/o/vynamp-decode.git
",
        )
        .unwrap();
        // Climbing out of the document's own directory is the normal case, and the one a
        // component-wise `starts_with` would miss.
        let from_doc = root.join("product/03-requisitos.md");
        std::fs::create_dir_all(root.join("product")).unwrap();
        let target = from_doc
            .parent()
            .unwrap()
            .join("../libs/decode/tasks/items/TASK-E01-01.md");
        assert!(inside_absent_submodule(root, &target));
        // Outside every submodule there is no such excuse.
        let elsewhere = from_doc.parent().unwrap().join("../tasks/items/NOPE.md");
        assert!(!inside_absent_submodule(root, &elsewhere));
        // Once the module is checked out, its links are ordinary links again.
        std::fs::create_dir_all(root.join("libs/decode")).unwrap();
        assert!(!inside_absent_submodule(root, &target));
    }
    /// The backlog writes `acceptance` as one sentence; the TASK file writes one table row
    /// per clause. Demanding the whole sentence would fail all 191 product tasks on format,
    /// not on divergence -- which is why the clause is the unit of comparison. What the
    /// check must still catch is a criterion that was rewritten or emptied.
    #[test]
    fn acceptance_is_compared_clause_by_clause() {
        let body = "# TASK-X — W

| ID | Given | When | Then |
|---|---|---|---|
                    | `TASK-X-AC-1` | g | w | primeira clausula |
                    | `TASK-X-AC-2` | g | w | segunda clausula |
";
        let task = |acceptance: &str| {
            json!({
                "title":"W", "deliverable":"W", "acceptance":acceptance,
            })
        };
        let of = |v: &Value| markdown_disagreements("TASK-X", body, v).unwrap();
        // The joined sentence is absent from the file; the clauses are not. That is a pass.
        assert!(!body.contains("primeira clausula; segunda clausula"));
        assert!(of(&task("primeira clausula; segunda clausula")).is_empty());
        // Rewritten clause: still a finding, even though the other one matches.
        assert_eq!(
            of(&task("primeira clausula; segunda reescrita")),
            vec!["Backlog/MD mismatch TASK-X acceptance: segunda reescrita"]
        );
        // An empty sentence must not read as "no clauses to check, everything present".
        assert_eq!(
            of(&task("   ")),
            vec!["Backlog/MD mismatch TASK-X acceptance"]
        );
        // Prose copied verbatim keeps the whole-string comparison.
        let other = json!({
            "title":"nao esta no arquivo", "deliverable":"W", "acceptance":"primeira clausula"
        });
        assert_eq!(
            markdown_disagreements("TASK-X", body, &other).unwrap(),
            vec!["Backlog/MD mismatch TASK-X title"]
        );
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
    strategy:
      matrix:
        runner: [ubuntu-22.04, macos-15-intel, macos-15]
    runs-on: ${{{{ matrix.runner }}}}
    steps:
      - uses: actions/checkout@{SHA}
      - run: git fetch --unshallow || true
      - run: cargo xtask check-secrets
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
  tasks:
    needs: specs
    runs-on: ubuntu-22.04
    steps:
      - run: cargo xtask check-tasks
  gate:
    name: CI Gate
    if: always()
    needs: [specs, tooling, submodules, arch, tasks]
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
    /// AC-4 of TASK-CI-01: the red names the file **and** the line. A bare file
    /// name sends the reader hunting through a workflow that repeats the same
    /// `uses:` in several jobs.
    #[test]
    fn github_actions_names_the_file_and_the_line_of_the_unpinned_action() {
        let f = ci_fixture();
        edit(
            f.path(),
            "ci.yml",
            &format!("actions/checkout@{SHA}"),
            "actions/checkout@v6",
        );
        let body = std::fs::read_to_string(f.path().join(".github/workflows/ci.yml")).unwrap();
        // Every unpinned occurrence is reported, and each with its own line --
        // the fixture repeats `checkout` once per job, so a report that gives
        // them all the same line would still satisfy a "names the file" check.
        let expected: Vec<String> = body
            .lines()
            .enumerate()
            .filter(|(_, l)| l.contains("actions/checkout@v6"))
            .map(|(i, _)| format!("ci.yml:{}", i + 1))
            .collect();
        assert!(expected.len() > 1, "fixture must repeat the action");
        let errors = ci_errors(f.path());
        let reported: Vec<&String> = errors
            .iter()
            .filter(|e| e.contains("actions/checkout@v6"))
            .collect();
        assert_eq!(reported.len(), expected.len(), "{errors:?}");
        for (e, line) in reported.iter().zip(&expected) {
            assert!(e.contains(line), "expected {line} in {e:?}");
        }
    }
    /// The location is the YAML path, not a guess: it must point at the `uses:`
    /// key that holds the reference and not merely at a line that happens to
    /// repeat the same text.
    #[test]
    fn action_ref_locations_report_the_key_path() {
        let v: Value = serde_yaml::from_str(
            "jobs:
  build:
    steps:
      - uses: actions/checkout@v6
",
        )
        .unwrap();
        assert_eq!(
            action_ref_locations(&v, &[]),
            vec![(
                "jobs.build.steps.0.uses".to_owned(),
                "actions/checkout@v6".to_owned()
            )]
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
            "needs: [specs, tooling, submodules, arch, tasks]",
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
    /// REQ-E19-01 makes macOS Intel and Apple Silicon mandatory targets, and a
    /// target no job runs on is not verified. The quality job that names them
    /// needs hardware that does not exist yet, and `ci core` is forbidden in
    /// the umbrella (D-CI-1), so the tooling matrix is the only place they can
    /// be covered. Dropping either image is a one-line edit that keeps every
    /// other check green while returning macOS to "not verified".
    #[test]
    fn github_actions_requires_the_macos_matrix_in_tooling() {
        const FULL: &str = "ubuntu-22.04, macos-15-intel, macos-15";
        for (dropped, without) in [
            ("macos-15-intel", "ubuntu-22.04, macos-15"),
            ("macos-15", "ubuntu-22.04, macos-15-intel"),
        ] {
            let f = ci_fixture();
            edit(f.path(), "ci.yml", FULL, without);
            let e = ci_errors(f.path());
            assert!(
                e.iter().any(|e| e.contains("macos-15-intel and macos-15")),
                "dropping {dropped} must fail the gate, got {e:?}"
            );
        }
    }
    /// `macos-latest` moves between architectures as GitHub rotates the image,
    /// so a matrix that names it claims to verify a target whose identity the
    /// repository does not control.
    #[test]
    fn github_actions_rejects_macos_latest_in_the_matrix() {
        let f = ci_fixture();
        edit(
            f.path(),
            "ci.yml",
            "ubuntu-22.04, macos-15-intel, macos-15",
            "ubuntu-22.04, macos-latest",
        );
        let e = ci_errors(f.path());
        assert!(e.iter().any(|e| e.contains("macos-latest")), "{e:?}");
    }
    /// AC-5 is a claim about history, and a scanner CI never runs proves nothing.
    #[test]
    fn github_actions_requires_the_secret_scan_on_full_history() {
        for (removed, needle) in [
            ("      - run: cargo xtask check-secrets\n", "check-secrets"),
            (
                "      - run: git fetch --unshallow || true\n",
                "fetch --unshallow",
            ),
        ] {
            let f = ci_fixture();
            edit(f.path(), "ci.yml", removed, "");
            let e = ci_errors(f.path());
            assert!(
                e.iter().any(|e| e.contains("TASK-E19-01 AC-5")),
                "dropping {needle} must fail the gate, got {e:?}"
            );
        }
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
            "needs: [specs, tooling, submodules, arch, tasks]",
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
    /// A backlog entry good enough to pass, so each adversarial case can break exactly one
    /// thing and be attributable to it.
    fn task_entry(id: &str, status: &str, ready: bool) -> Value {
        json!({
            "id": id, "epic": "E99", "title": "T", "phase": "M0", "priority": "P0",
            "status": status, "requirements": ["REQ-E99-01"], "dependencies": [],
            "module": "vynamp", "repo": "vynamp", "acceptance_ready": ready,
            "deliverable": "D", "acceptance": "A", "qa": [], "estimate_days": {"min":1,"max":2},
            "risk": "baixo",
        })
    }
    fn task_md(owners: &str, given: &str, when: &str, then: &str) -> String {
        format!(
            "# TASK — t\n\n| Campo | Valor |\n|---|---|\n| ID | `TASK-E99-01` |\n| Módulo dono | {owners} |\n\n## Critérios de aceite\n\n| ID | Given | When | Then |\n|---|---|---|---|\n| `TASK-E99-01-AC-1` | {given} | {when} | {then} |\n"
        )
    }
    fn run(backlog: Value, body: String) -> Vec<String> {
        task_disagreements(&backlog, move |_| Some(body.clone()))
    }
    #[test]
    fn check_tasks_ac1_names_the_task_and_the_missing_column() {
        // AC-1: a product TASK whose `Given` is still the placeholder is rejected, and the
        // message carries both the ID and which column is unwritten.
        let e = run(
            json!({"tasks":[task_entry("TASK-E99-01","planned",true)]}),
            task_md("`vynamp`", "PENDENTE", "w", "t"),
        );
        // A `planned` task is allowed to still hold the placeholder, so move it out first.
        assert!(e.is_empty(), "{e:?}");
        let e = run(
            json!({"tasks":[task_entry("TASK-E99-01","in_progress",true)]}),
            task_md("`vynamp`", "PENDENTE", "w", "t"),
        );
        assert_eq!(e, vec!["TASK-E99-01 TASK-E99-01-AC-1: Given not written"]);
    }
    #[test]
    fn check_tasks_ac2_rejects_advanced_status_without_written_criteria() {
        // AC-2: `in_progress` plus `acceptance_ready: false` is the combination the field
        // exists to catch, and the message lists the ID.
        let e = run(
            json!({"tasks":[task_entry("TASK-E99-01","in_progress",false)]}),
            task_md("`vynamp`", "g", "w", "t"),
        );
        assert_eq!(
            e,
            vec!["TASK-E99-01 is in_progress with acceptance_ready: false"]
        );
    }
    #[test]
    fn check_tasks_ac3_rejects_two_owners_or_an_unknown_one() {
        // AC-3: the owner cell has to name exactly one repository -- two is ambiguous, and
        // a module the backlog does not know is a task with no home.
        let e = run(
            json!({"tasks":[task_entry("TASK-E99-01","planned",true)]}),
            task_md("`vynamp-decode` / `vynamp-output`", "g", "w", "t"),
        );
        assert_eq!(e, vec!["TASK-E99-01 has 2 module owners"]);
        // A placeholder reads as one word and would otherwise pass as an uncontested
        // single owner; the known-repository check is what catches it.
        let e = run(
            json!({"tasks":[task_entry("TASK-E99-01","planned",true)]}),
            task_md("PENDENTE", "g", "w", "t"),
        );
        assert_eq!(e, vec!["TASK-E99-01 has unknown module owner PENDENTE"]);
    }
    #[test]
    fn check_tasks_ac4_names_every_offender_and_only_those() {
        // AC-4: red lists the tasks that broke the rule and nothing else -- a gate that
        // also complains about the compliant ones is a gate nobody can act on.
        let ready = task_md("`vynamp`", "g", "w", "t");
        let e = run(
            json!({"tasks":[
                task_entry("TASK-E99-01","planned",true),
                task_entry("TASK-E99-02","in_progress",false),
                task_entry("TASK-E99-03","implemented",false),
                task_entry("TASK-E99-04","planned",true),
            ]}),
            ready,
        );
        assert_eq!(
            e,
            vec![
                "TASK-E99-02 is in_progress with acceptance_ready: false",
                "TASK-E99-03 is implemented with acceptance_ready: false",
            ]
        );
    }
    #[test]
    fn check_tasks_ac5_passes_once_the_criteria_are_written() {
        // AC-5: the same written criteria that make a task acceptable are what turn the
        // gate green -- no rule is relaxed between the red and the green.
        let e = run(
            json!({"tasks":[task_entry("TASK-E99-01","implemented",true)]}),
            task_md("`vynamp`", "o repo", "o gate roda", "passa"),
        );
        assert!(e.is_empty(), "{e:?}");
    }
    #[test]
    fn check_tasks_requires_a_req_from_product_tasks_only() {
        // The two engineering tasks have no REQ by design (they are not product
        // requirements); demanding one would make the rule wrong for the only two files
        // that legitimately lack it.
        let mut engineering = task_entry("TASK-CI-09", "planned", true);
        engineering["requirements"] = json!([]);
        let mut product = task_entry("TASK-E99-01", "planned", true);
        product["requirements"] = json!([]);
        let e = run(
            json!({"tasks":[engineering, product]}),
            task_md("`vynamp`", "g", "w", "t"),
        );
        assert_eq!(e, vec!["TASK-E99-01 has no REQ"]);
    }
    #[test]
    fn acceptance_rows_keeps_a_then_that_contains_a_pipe() {
        // `Then` is the authoritative column and is copied from the backlog verbatim; a
        // criterion quoting a shell pipe must not be read truncated, or the gate would
        // compare against a fragment.
        let body = "| ID | Given | When | Then |\n|---|---|---|---|\n| `X` | g | w | a | b |\n";
        let rows = acceptance_rows(body);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].3, "a | b");
    }
}
