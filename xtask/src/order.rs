//! Gera e confere os três documentos de estado que derivam do backlog.
//!
//! `tasks/backlog.json` é a fonte única de status (AGENTS.md §4.7). `STATE.md`,
//! `tasks/NEXT.md` e `tasks/ORDER.md` são **leituras** dele, e leitura que ninguém
//! confere envelhece: o `NEXT.md` listava `TASK-E01-04` como pendente depois de ela
//! estar em `develop`, e o `STATE.md` morava fora do git, então quem clonava não
//! recebia resumo nenhum.
//!
//! A correção não é prometer disciplina, é tirar a parte que envelhece da mão
//! humana. Cada documento carrega um bloco entre marcadores, gerado por
//! `cargo xtask docs-sync` e conferido por `check-docs`. A prosa fora do bloco
//! continua humana e o gate não a julga -- ele só compara o que é derivável.
//!
//! `ORDER.md` é o caso extremo: o arquivo inteiro é gerado, porque a ordem de
//! execução é exatamente a ordem topológica das dependências e não há prosa a
//! preservar. `STATE.md` e `NEXT.md` têm blocos porque o resto deles é narrativa.

use crate::util::*;
use anyhow::{Context, Result, bail};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

/// Nome do bloco gerado em `STATE.md`.
const STATE: &str = "state";
/// Nome do bloco gerado em `tasks/NEXT.md`.
const NEXT: &str = "next";

/// Uma TASK reduzida ao que os três documentos mostram.
struct Task {
    id: String,
    title: String,
    module: String,
    phase: String,
    priority: String,
    status: String,
    deps: Vec<String>,
}

/// Posição da fase na ordem de entrega: `M0`..`M5`, depois `F6`.
///
/// `F6` é a fase 6, que sai depois do 1.0. Ela ranqueia acima de todo marco, e não
/// abaixo de `M0` por `F` ordenar antes de `M` -- a mesma leitura que
/// `validate::phase` faz para a regra de dependência.
fn rank(phase: &str) -> u32 {
    match phase.strip_prefix('M') {
        Some(digits) => digits.parse().unwrap_or(u32::MAX),
        None if phase == "F6" => 6,
        None => u32::MAX,
    }
}

/// `P0` antes de `P1` antes de `P2`; qualquer outra coisa vai para o fim.
fn prio(priority: &str) -> u32 {
    match priority {
        "P0" => 0,
        "P1" => 1,
        "P2" => 2,
        _ => u32::MAX,
    }
}

fn load(root: &Path) -> Result<Vec<Task>> {
    let data = read_json(root.join("tasks/backlog.json"))?;
    let tasks = data["tasks"].as_array().context("Backlog tasks")?;
    let mut out = Vec::with_capacity(tasks.len());
    for t in tasks {
        out.push(Task {
            id: string(t, "id")?.to_owned(),
            title: string(t, "title")?.to_owned(),
            module: string(t, "module")?.to_owned(),
            phase: string(t, "phase")?.to_owned(),
            priority: string(t, "priority")?.to_owned(),
            status: string(t, "status")?.to_owned(),
            deps: t["dependencies"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect(),
        });
    }
    Ok(out)
}

/// Ordem de leitura dentro de uma onda: fase, prioridade, ID.
///
/// A onda diz o que **pode** começar; esta ordem diz o que ler primeiro. Sem ela a
/// lista sairia na ordem do backlog, que é por épico e não por urgência.
fn reading_order(tasks: &[Task], ids: &[String]) -> Vec<String> {
    let by: BTreeMap<&str, &Task> = tasks.iter().map(|t| (t.id.as_str(), t)).collect();
    let mut ids = ids.to_vec();
    ids.sort_by(|a, b| {
        let (ta, tb) = (by[a.as_str()], by[b.as_str()]);
        (rank(&ta.phase), prio(&ta.priority), &ta.id).cmp(&(
            rank(&tb.phase),
            prio(&tb.priority),
            &tb.id,
        ))
    });
    ids
}

/// Ondas topológicas a partir do que já está `implemented`.
///
/// A onda 1 é o que está destravado agora. Uma TASK entra na onda `n` quando todas
/// as suas dependências estão em ondas anteriores. O backlog é acíclico e não tem
/// dependência pendurada -- `check-tasks` já reprova as duas coisas -- então o
/// laço sempre termina; o `bail!` existe para o dia em que uma das duas garantias
/// cair e este gerador for o primeiro a notar.
fn waves(tasks: &[Task]) -> Result<Vec<Vec<String>>> {
    let done: BTreeSet<&str> = tasks
        .iter()
        .filter(|t| t.status != "planned")
        .map(|t| t.id.as_str())
        .collect();
    let mut placed: BTreeSet<String> = done.iter().map(|s| (*s).to_owned()).collect();
    let mut remaining: BTreeSet<String> = tasks
        .iter()
        .filter(|t| t.status == "planned")
        .map(|t| t.id.clone())
        .collect();
    let mut out = Vec::new();
    while !remaining.is_empty() {
        let ready: Vec<String> = remaining
            .iter()
            .filter(|id| {
                tasks
                    .iter()
                    .find(|t| &t.id == *id)
                    .is_some_and(|t| t.deps.iter().all(|d| placed.contains(d)))
            })
            .cloned()
            .collect();
        if ready.is_empty() {
            bail!(
                "dependency cycle or dangling dependency among {} tasks",
                remaining.len()
            );
        }
        for id in &ready {
            remaining.remove(id);
            placed.insert(id.clone());
        }
        out.push(reading_order(tasks, &ready));
    }
    Ok(out)
}

fn delivered(tasks: &[Task]) -> Vec<&Task> {
    let mut v: Vec<&Task> = tasks.iter().filter(|t| t.status != "planned").collect();
    v.sort_by(|a, b| a.id.cmp(&b.id));
    v
}

/// Bloco de `STATE.md`: contagem por estado e a lista do que já foi entregue.
///
/// Não carrega data nem SHA. Data envelhece no dia seguinte e SHA muda a cada
/// merge; os dois transformariam o gate em ruído. O que muda por TASK entregue é
/// a lista, e é ela que o gate compara.
fn render_state(tasks: &[Task]) -> String {
    let done = delivered(tasks);
    let planned = tasks.len() - done.len();
    let mut s = String::from("\n");
    s.push_str("| Estado | TASKs |\n|---|---|\n");
    s.push_str(&format!("| `implemented` | {} |\n", done.len()));
    s.push_str(&format!("| `planned` | {planned} |\n"));
    s.push_str(&format!("| **total** | {} |\n\n", tasks.len()));
    s.push_str("Entregues, em ordem de ID:\n\n");
    for t in &done {
        s.push_str(&format!("- `{}` — {}\n", t.id, t.title));
    }
    s
}

/// Bloco de `tasks/NEXT.md`: o que fechou e o que está destravado agora.
fn render_next(tasks: &[Task], waves: &[Vec<String>]) -> String {
    let by: BTreeMap<&str, &Task> = tasks.iter().map(|t| (t.id.as_str(), t)).collect();
    let done = delivered(tasks);
    let mut s = String::from("\n");
    s.push_str(&format!(
        "{} TASKs entregues de {}. A onda 1 tem {} destravada(s) agora.\n\n",
        done.len(),
        tasks.len(),
        waves.first().map_or(0, Vec::len)
    ));
    // Sem cabeçalho próprio: a seção que hospeda o bloco no Markdown já se chama
    // "O que está destravado agora", e repetir o título dentro dele leria como dois.
    s.push_str("| ID | Título | Módulo | Fase | Prio |\n|---|---|---|---|---|\n");
    for id in waves.first().map(Vec::as_slice).unwrap_or(&[]) {
        let t = by[id.as_str()];
        s.push_str(&format!(
            "| `{}` | {} | `{}` | {} | {} |\n",
            t.id, t.title, t.module, t.phase, t.priority
        ));
    }
    s
}

/// `tasks/ORDER.md` inteiro: a sequência topológica completa.
fn render_order(tasks: &[Task], waves: &[Vec<String>]) -> String {
    let by: BTreeMap<&str, &Task> = tasks.iter().map(|t| (t.id.as_str(), t)).collect();
    let done = delivered(tasks);
    let mut s = String::from(
        "# Ordem de execução das TASKs\n\n\
         Versão: 0.1.0 · Data: 2026-11-13 · Estado: **gerado** a partir de\n\
         [`backlog.json`](backlog.json) por `cargo xtask docs-sync`.\n\n\
         Este arquivo não se edita à mão. A ordem é a topológica das dependências do\n\
         backlog: uma TASK entra na onda `n` quando todas as suas dependências estão\n\
         em ondas anteriores. A onda diz o que **pode** começar, não o que **deve**\n\
         começar primeiro -- dentro de uma onda a ordem é fase, prioridade e ID, e a\n\
         escolha entre elas é do mantenedor. O status de cada uma vive no backlog, e\n\
         `cargo xtask check-docs` reprova este arquivo quando ele diverge de lá.\n\n",
    );
    s.push_str(&format!(
        "{} TASKs entregues, {} planejadas, {} ondas.\n\n",
        done.len(),
        tasks.len() - done.len(),
        waves.len()
    ));
    for (i, wave) in waves.iter().enumerate() {
        s.push_str(&format!("## Onda {} — {} TASK(s)\n\n", i + 1, wave.len()));
        s.push_str("| ID | Título | Módulo | Fase | Prio | Depende de |\n");
        s.push_str("|---|---|---|---|---|---|\n");
        for id in wave {
            let t = by[id.as_str()];
            let deps = if t.deps.is_empty() {
                "—".to_owned()
            } else {
                t.deps
                    .iter()
                    .map(|d| format!("`{d}`"))
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            s.push_str(&format!(
                "| `{}` | {} | `{}` | {} | {} | {} |\n",
                t.id, t.title, t.module, t.phase, t.priority, deps
            ));
        }
        s.push('\n');
    }
    s
}

/// Conteúdo entre os marcadores de um bloco gerado.
fn block<'a>(body: &'a str, name: &str) -> Option<&'a str> {
    let begin = format!("<!-- BEGIN GENERATED: {name} -->");
    let end = format!("<!-- END GENERATED: {name} -->");
    let start = body.find(&begin)? + begin.len();
    let rest = &body[start..];
    let stop = rest.find(&end)?;
    Some(&rest[..stop])
}

/// Troca o conteúdo do bloco, preservando a prosa em volta.
fn replace(body: &str, name: &str, inner: &str) -> Result<String> {
    let begin = format!("<!-- BEGIN GENERATED: {name} -->");
    let end = format!("<!-- END GENERATED: {name} -->");
    let start = body
        .find(&begin)
        .with_context(|| format!("missing marker {begin}"))?
        + begin.len();
    let rest = &body[start..];
    let stop = rest
        .find(&end)
        .with_context(|| format!("missing marker {end}"))?;
    Ok(format!("{}{}{}", &body[..start], inner, &rest[stop..]))
}

/// Reescreve os três documentos. Idempotente: rodar duas vezes não muda byte.
pub fn sync(root: &Path) -> Result<()> {
    let tasks = load(root)?;
    let waves = waves(&tasks)?;
    fs::write(root.join("tasks/ORDER.md"), render_order(&tasks, &waves))?;
    for (path, name, inner) in [
        ("STATE.md", STATE, render_state(&tasks)),
        ("tasks/NEXT.md", NEXT, render_next(&tasks, &waves)),
    ] {
        let full = root.join(path);
        let body = text(&full).with_context(|| format!("{path} absent"))?;
        fs::write(&full, replace(&body, name, &inner)?)?;
    }
    Ok(())
}

/// Divergências entre os documentos e o backlog. Vazio é o esperado.
///
/// Devolve em vez de imprimir para que `check-docs` junte estas linhas às dele e
/// o veredito saia num relatório só.
pub fn check(root: &Path) -> Result<Vec<String>> {
    let tasks = load(root)?;
    let waves = waves(&tasks)?;
    let mut out = Vec::new();
    let order = root.join("tasks/ORDER.md");
    match text(&order) {
        Ok(body) if body == render_order(&tasks, &waves) => {}
        Ok(_) => out.push("tasks/ORDER.md is stale: run `cargo xtask docs-sync`".to_owned()),
        Err(_) => out.push("tasks/ORDER.md absent: run `cargo xtask docs-sync`".to_owned()),
    }
    for (path, name, inner) in [
        ("STATE.md", STATE, render_state(&tasks)),
        ("tasks/NEXT.md", NEXT, render_next(&tasks, &waves)),
    ] {
        match text(root.join(path)) {
            Ok(body) => match block(&body, name) {
                Some(found) if found == inner => {}
                Some(_) => out.push(format!(
                    "{path} block `{name}` is stale: run `cargo xtask docs-sync`"
                )),
                None => out.push(format!("{path} has no `{name}` generated block")),
            },
            Err(_) => out.push(format!("{path} absent")),
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn task(id: &str, status: &str, deps: &[&str]) -> Task {
        Task {
            id: id.to_owned(),
            title: format!("title {id}"),
            module: "vynamp".to_owned(),
            phase: "M1".to_owned(),
            priority: "P1".to_owned(),
            status: status.to_owned(),
            deps: deps.iter().map(|d| (*d).to_owned()).collect(),
        }
    }

    /// A onda 1 é o que está destravado agora, e não o que veio primeiro no arquivo.
    #[test]
    fn waves_follow_dependencies_not_file_order() {
        let tasks = vec![
            task("TASK-A", "planned", &["TASK-B"]),
            task("TASK-B", "planned", &["TASK-C"]),
            task("TASK-C", "implemented", &[]),
        ];
        let w = waves(&tasks).unwrap();
        assert_eq!(
            w,
            vec![vec!["TASK-B".to_owned()], vec!["TASK-A".to_owned()]]
        );
    }

    /// Duas TASKs sem dependência entre si caem na mesma onda: a onda não é fila.
    #[test]
    fn independent_tasks_share_a_wave() {
        let tasks = vec![
            task("TASK-A", "planned", &[]),
            task("TASK-B", "planned", &[]),
        ];
        let w = waves(&tasks).unwrap();
        assert_eq!(w.len(), 1);
        assert_eq!(w[0].len(), 2);
    }

    /// Um ciclo não pode virar laço infinito: o gerador para e diz o que houve.
    #[test]
    fn a_cycle_is_an_error_not_a_hang() {
        let tasks = vec![
            task("TASK-A", "planned", &["TASK-B"]),
            task("TASK-B", "planned", &["TASK-A"]),
        ];
        assert!(waves(&tasks).is_err());
    }

    /// O bloco é trocado e a prosa em volta sobrevive byte a byte.
    #[test]
    fn replace_keeps_the_prose_around_the_block() {
        let body = "# T\n\nantes\n<!-- BEGIN GENERATED: state -->\nvelho\n<!-- END GENERATED: state -->\ndepois\n";
        let out = replace(body, STATE, "\nnovo\n").unwrap();
        assert_eq!(
            out,
            "# T\n\nantes\n<!-- BEGIN GENERATED: state -->\nnovo\n<!-- END GENERATED: state -->\ndepois\n"
        );
        assert_eq!(block(&out, STATE), Some("\nnovo\n"));
    }

    /// Sem marcador o gerador não adivinha onde escrever: ele reprova.
    #[test]
    fn a_missing_marker_is_an_error() {
        assert!(replace("# T\n", STATE, "x").is_err());
    }
}
