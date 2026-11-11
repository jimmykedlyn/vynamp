//! Gate das regras de autoria que o guia impõe e que `check-tasks` não vê.
//!
//! `check-tasks` compara o Markdown com o backlog -- título, entrega e cada cláusula
//! `Then`. Ele não julga se o `Given`/`When` foi de fato autorado, se o bullet de
//! `Boundary` nomeia um dono, nem se a tabela está bem formada. Foi assim que três
//! linhas de AC de `TASK-E08-02` viveram em `develop` sem o `|` de fechamento: a
//! cláusula `Then` estava lá e o gate a encontrou, mas o parser de tabela contava
//! três células em vez de quatro.
//!
//! As regras vêm de `Tasks.ps1`/guia de autoria. Cada uma é mecânica: não tenta
//! julgar prosa, só detectar a forma que o guia proíbe.
//!
//! | Regra | O que detecta | Bloqueia |
//! |---|---|---|
//! | `R1` | bullet de `Boundary` sem dono (ID de TASK, ADR existente ou spec) | sim |
//! | `R2` | bullet de `Boundary` curto demais para explicar a confusão | não |
//! | `R3` | `Given`/`When` com menos de quatro palavras | não |
//! | `R4` | `Given`/`When` vazio ou `PENDENTE` | sim |
//! | `R5` | oráculo usando "verde no CI Gate" como evidência | sim |
//! | `R6` | célula de verificação/hardware ainda `PENDENTE` | sim |
//! | `R7` | linha de AC com número de colunas diferente de quatro | sim |
//! | `R8` | `Then` vazio | sim |
//! | `R9` | marcador de template ainda presente | sim |
//!
//! `R5` merece uma nota: "verde no CI Gate" na linha `Teste automatizado` é a
//! verdade do template e continua correta. O que o guia proíbe é ela ser a
//! evidência do *oráculo*, porque uma suíte vazia a satisfaz -- o gate afirmaria
//! cobertura que não há.

use crate::util::*;
use anyhow::{Context, Result};
use regex::Regex;
use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Regras cuja violação reprova o gate. As demais são aviso: pedem revisão de
/// prosa, e um bullet curto pode ser legítimo quando o escopo é óbvio.
const BLOCKING: [&str; 7] = ["R1", "R4", "R5", "R6", "R7", "R8", "R9"];

struct Finding {
    task: String,
    rule: &'static str,
    detail: String,
}

/// Uma seção `## Título` do documento, sem os cabeçalhos vizinhos.
fn section<'a>(body: &'a str, title: &str) -> &'a str {
    let header = format!("## {title}");
    let Some(start) = body.find(&header) else {
        return "";
    };
    let rest = &body[start + header.len()..];
    // O título tem que terminar a linha: `## Boundary` não pode casar com
    // `## Boundary da TASK`. Depois do título só resta a quebra de linha.
    let rest = match rest.find('\n') {
        Some(i) if rest[..i].trim().is_empty() => &rest[i + 1..],
        _ => return "",
    };
    match rest.find("\n## ") {
        Some(end) => &rest[..end + 1],
        None => rest,
    }
}

/// Bullets de uma seção, com as linhas de continuação já juntadas.
///
/// Um bullet markdown quebra em várias linhas indentadas, e o dono costuma
/// estar na segunda -- julgar linha a linha acusaria de `R1` quase todo bullet
/// correto.
fn bullets(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current: Option<String> = None;
    for line in text.lines() {
        if line.starts_with("- ") {
            if let Some(b) = current.take() {
                out.push(b);
            }
            current = Some(line.trim().to_owned());
        } else if line.trim().is_empty() {
            if let Some(b) = current.take() {
                out.push(b);
            }
        } else if let Some(b) = current.as_mut() {
            b.push(' ');
            b.push_str(line.trim());
        }
    }
    if let Some(b) = current {
        out.push(b);
    }
    out
}

fn words(s: &str) -> usize {
    s.split_whitespace().count()
}

/// Audita um documento de TASK e devolve os achados, sem tocar no backlog.
///
/// Devolve `Vec<Finding>` em vez de escrever direto no coletor para que as
/// regras possam ser exercitadas contra Markdown de teste, sem repositório.
fn audit(task: &str, body: &str, adrs: &BTreeSet<String>, roots: &[&Path]) -> Result<Vec<Finding>> {
    let mut rows = Vec::new();

    // R9 é o marcador de template do gerador. Quando ele está presente o arquivo
    // inteiro é rascunho, e auditar o resto produziria uma parede de achados
    // derivados que esconde o único que importa.
    if body.contains("<!-- PENDENTE") {
        rows.push(Finding {
            task: task.to_owned(),
            rule: "R9",
            detail: "ainda tem marcador de template".to_owned(),
        });
        return Ok(rows);
    }

    let task_id = Regex::new(r"TASK-[A-Z0-9]+-\d+")?;
    // Nem toda fronteira tem TASK dona. Uma decisão do mantenedor -- o produto
    // não usar código do Winamp, a marca, a pilha própria, a cota do plano Free
    // do CI -- não pertence a TASK nenhuma do backlog, e inventar uma para
    // satisfazer o gate produziria um dono fictício. Nesses casos o próprio
    // guia manda nomear a autoridade real, e a autoridade é uma ADR ou uma spec.
    //
    // A ADR citada tem de existir: `ADR-999` no texto não é autoridade nenhuma,
    // e aceitar qualquer `ADR-\d+` trocaria um bullet sem dono por um bullet com
    // dono inventado, que é pior -- o primeiro se vê, o segundo não.
    let adr_ref = Regex::new(r"ADR-\d+")?;
    // A outra autoridade possível é um documento normativo: a spec do módulo em
    // `specs/`, o documento de `design/`, de `platforms/` ou um `.md` na raiz.
    // A lista de diretórios seria uma enumeração que envelhece a cada pasta
    // nova -- a spec de iOS, que é a mais recente, já mora em `platforms/` e não
    // em `specs/`. O que a regra exige é que o documento citado exista no
    // repositório: um caminho inventado é o mesmo dono fictício que uma ADR
    // inexistente, e o ceticismo vale igual.
    //
    // O caminho é resolvido tanto na raiz do umbrella quanto na raiz do repo da
    // TASK: a spec do módulo mora em `apps/player/specs/ui.md`, e um bullet de
    // TASK do player que diz `specs/ui.md` está citando autoridade real, mesmo
    // que esse caminho não exista a partir do umbrella. Olhar só uma das duas
    // raízes reprovaria o bullet correto -- e o corrigiria para um caminho
    // errado, que é o efeito contrário do que a regra quer.
    let doc_ref = Regex::new(r"(?:[A-Za-z0-9_]+/)*[A-Za-z0-9_-]+\.md")?;
    for b in bullets(section(body, "Boundary")) {
        if !b.starts_with("- **N") {
            continue;
        }
        let named_adr = adr_ref.find_iter(&b).any(|m| adrs.contains(m.as_str()));
        // `README.md` e `INDEX.md` são citados o tempo todo por outros motivos --
        // listar a TASK, apontar o backlog -- e aceitá-los como dono deixaria
        // passar um bullet sem dono sempre que ele mencionasse um deles.
        let named_doc = doc_ref
            .find_iter(&b)
            .map(|m| m.as_str())
            .filter(|d| !matches!(*d, "README.md" | "INDEX.md"))
            .any(|d| roots.iter().any(|r| r.join(d).is_file()));
        let has_owner = task_id.is_match(&b) || named_adr || named_doc;
        // `SEM-DONO` é o nome que o processo de autoria dá ao caso "não há TASK
        // dona", não uma frase para o documento. O bullet tem de dizer qual é a
        // autoridade, e um marcador interno no lugar dela deixaria o leitor sem
        // resposta -- exatamente o defeito que R1 existe para pegar.
        if b.contains("SEM-DONO") {
            rows.push(Finding {
                task: task.to_owned(),
                rule: "R1",
                detail: format!("marcador interno SEM-DONO no texto: {}", truncate(&b, 90)),
            });
        } else if !has_owner {
            rows.push(Finding {
                task: task.to_owned(),
                rule: "R1",
                detail: truncate(&b, 110),
            });
        } else if words(&b) < 12 {
            rows.push(Finding {
                task: task.to_owned(),
                rule: "R2",
                detail: truncate(&b, 110),
            });
        }
    }

    let ac_row = Regex::new(r"^\| `(?P<ac>[A-Z0-9-]+-AC-\d+)` \|(?P<rest>.*)$")?;
    for line in body.lines() {
        let Some(hit) = ac_row.captures(line) else {
            continue;
        };
        let ac = &hit["ac"];
        let cells: Vec<&str> = hit["rest"].split('|').collect();
        if cells.len() != 4 {
            rows.push(Finding {
                task: task.to_owned(),
                rule: "R7",
                detail: format!("{ac} tem {} celulas", cells.len()),
            });
            continue;
        }
        let cell = |i: usize| cells[i].trim();
        for (name, val) in [("Given", cell(0)), ("When", cell(1))] {
            if val.is_empty() || val == "PENDENTE" {
                rows.push(Finding {
                    task: task.to_owned(),
                    rule: "R4",
                    detail: format!("{ac} {name}"),
                });
            } else if words(val) < 4 {
                rows.push(Finding {
                    task: task.to_owned(),
                    rule: "R3",
                    detail: format!("{ac} {name}: {}", truncate(val, 70)),
                });
            }
        }
        if cell(2).is_empty() {
            rows.push(Finding {
                task: task.to_owned(),
                rule: "R8",
                detail: ac.to_owned(),
            });
        }
    }

    let verification = section(body, "Verificação");
    for line in verification.lines() {
        if !line.starts_with('|') {
            continue;
        }
        let cells: Vec<&str> = line
            .trim()
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .collect();
        // Qualquer célula depois do rótulo, em qualquer tabela: contar colunas
        // primeiro deixaria passar uma tabela de duas células com `PENDENTE`.
        if cells.len() >= 2 && cells[1..].contains(&"PENDENTE") {
            rows.push(Finding {
                task: task.to_owned(),
                rule: "R6",
                detail: format!("celula vazia de template: {}", truncate(cells[0], 40)),
            });
        }
        if line.starts_with("| Oráculo independente |") && line.contains("verde no CI Gate") {
            rows.push(Finding {
                task: task.to_owned(),
                rule: "R5",
                detail: "oraculo usa 'verde no CI Gate'".to_owned(),
            });
        }
    }

    Ok(rows)
}

/// Corta em fronteira de caractere, não de byte: o texto é UTF-8 acentuado.
fn truncate(s: &str, max: usize) -> String {
    match s.char_indices().nth(max) {
        Some((i, _)) => s[..i].to_owned(),
        None => s.to_owned(),
    }
}

pub fn check(root: &Path) -> Result<()> {
    let data = read_json(root.join("tasks/backlog.json"))?;
    let tasks = data["tasks"].as_array().context("Backlog tasks")?;

    let mut files = 0usize;
    let mut seen = BTreeSet::new();
    let mut rows: Vec<(String, &'static str, String)> = Vec::new();
    let adrs = adr_ids(root)?;

    for t in tasks {
        let Ok(id) = string(t, "id") else {
            continue;
        };
        let Some(path) = task_path(root, t) else {
            continue;
        };
        // Um módulo que não está no checkout não tem arquivo para ler. Tratar a
        // ausência como violação faria o veredito depender do clone -- o mesmo
        // erro que `doc_files` foi estreitado para evitar.
        if !path.is_file() {
            let repo = owner(t);
            if repo != "vynamp" && repo_dir(root, repo).is_some_and(|d| !d.is_dir()) {
                continue;
            }
        }
        let Ok(body) = fs::read_to_string(&path) else {
            continue;
        };
        files += 1;
        // A spec do próprio módulo é autoridade para uma fronteira dele: o
        // caminho relativo `specs/x.md` vale na raiz do repo que hospeda a TASK.
        let module_root = repo_dir(root, owner(t));
        let roots: Vec<&Path> = match module_root.as_deref() {
            Some(d) if d != root => vec![root, d],
            _ => vec![root],
        };
        for f in audit(id, &body, &adrs, &roots)? {
            if seen.insert((f.task.clone(), f.rule, f.detail.clone())) {
                rows.push((f.task, f.rule, f.detail));
            }
        }
    }

    let blocking: Vec<_> = rows
        .iter()
        .filter(|(_, rule, _)| BLOCKING.contains(rule))
        .collect();
    let warnings = rows.len() - blocking.len();

    for (task, rule, detail) in &rows {
        let label = if BLOCKING.contains(rule) {
            "erro"
        } else {
            "aviso"
        };
        println!("{label} {task} {rule}: {detail}");
    }
    println!(
        "{}",
        json!({
            "scope": "TASK Markdown only; no build, no hardware",
            "tasks": files,
            "blocking": blocking.len(),
            "warnings": warnings,
            "result": if blocking.is_empty() { "passed" } else { "failed" },
        })
    );
    anyhow::ensure!(
        blocking.is_empty(),
        "Authoring gate failed: {} achado(s) bloqueante(s)",
        blocking.len()
    );
    Ok(())
}

/// Os identificadores das ADRs que existem em `adr/`.
///
/// Uma fronteira sem TASK dona pode apontar para a decisão do mantenedor que a
/// fixa, e a decisão mora numa ADR. Sem esta lista o gate aceitaria `ADR-999`
/// escrito no texto, que é um dono inventado com aparência de autoridade.
fn adr_ids(root: &Path) -> Result<BTreeSet<String>> {
    let dir = root.join("adr");
    let mut ids = BTreeSet::new();
    let entries = match fs::read_dir(&dir) {
        Ok(e) => e,
        // Sem o diretório não há ADR a citar: todo bullet volta a precisar de
        // TASK dona. Um erro aqui reprovaria o gate por um diretório ausente,
        // que não é o que a regra mede.
        Err(_) => return Ok(ids),
    };
    let adr = Regex::new(r"^(ADR-\d+)")?;
    for e in entries.flatten() {
        if let Some(name) = e.file_name().to_str()
            && let Some(hit) = adr.captures(name)
        {
            ids.insert(hit[1].to_owned());
        }
    }
    Ok(ids)
}

/// Onde a TASK mora, resolvido pelo repositório que o backlog lhe atribui.
///
/// Reimplementado aqui em vez de exposto por `validate` porque o gate roda
/// antes do backlog de autoria ser compactado: a resolução precisa ser a mesma
/// que `check-tasks` usa, e duplicá-la em um helper público custaria mais do que
/// as quatro linhas abaixo.
fn task_path(root: &Path, t: &Value) -> Option<PathBuf> {
    let id = string(t, "id").ok()?;
    let dir = repo_dir(root, t["repo"].as_str().unwrap_or("vynamp"))?;
    Some(dir.join(format!("tasks/items/{id}.md")))
}

fn owner(t: &Value) -> &str {
    t["repo"].as_str().unwrap_or("vynamp")
}

fn repo_dir(root: &Path, repo: &str) -> Option<PathBuf> {
    if repo == "vynamp" {
        return Some(root.to_path_buf());
    }
    submodule_dir(root, repo).map(|d| root.join(d))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Audita contra uma raiz mínima: uma ADR só, e nenhum `.md` de verdade.
    ///
    /// A raiz de teste tem de ser vazia de documentos para que `README.md` de
    /// fato não exista; do contrário a regra do dono passaria por acidente e o
    /// teste não provaria nada.
    fn rules(body: &str) -> Vec<&'static str> {
        with_docs(body, &["ADR-001"], &[])
    }

    fn with_docs(body: &str, adrs: &[&str], docs: &[&str]) -> Vec<&'static str> {
        let dir = std::env::temp_dir().join("vynamp-authoring-test");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        for d in docs {
            let p = dir.join(d);
            fs::create_dir_all(p.parent().unwrap()).unwrap();
            fs::write(&p, "# doc\n").unwrap();
        }
        let set: BTreeSet<String> = adrs.iter().map(|s| (*s).to_owned()).collect();
        audit("TASK-E00-01", body, &set, &[&dir])
            .unwrap()
            .iter()
            .map(|f| f.rule)
            .collect()
    }

    /// A spec do módulo é citada pelo caminho relativo ao repo dela, e não ao
    /// umbrella. O dono é real, e o gate tem de resolver o caminho na raiz do
    /// repo da TASK para enxergá-lo.
    #[test]
    fn module_relative_spec_is_resolved_in_the_task_repo() {
        let root = std::env::temp_dir().join("vynamp-authoring-root");
        let module = std::env::temp_dir().join("vynamp-authoring-module");
        for d in [&root, &module] {
            let _ = fs::remove_dir_all(d);
            fs::create_dir_all(d).unwrap();
        }
        fs::create_dir_all(module.join("specs")).unwrap();
        fs::write(module.join("specs/ui.md"), "# ui\n").unwrap();

        let body = "## Boundary\n\n- **Não** decide a curva do medidor: o valor sai de \
                    `specs/ui.md`, e adivinhar a curva faria o medidor divergir do \
                    que o MVP mediu.\n";
        let set = BTreeSet::new();
        let rules: Vec<_> = audit("TASK-E00-01", body, &set, &[&root, &module])
            .unwrap()
            .iter()
            .map(|f| f.rule)
            .collect();
        assert_eq!(rules, Vec::<&str>::new());
    }

    /// O mesmo caminho relativo não vale se nenhuma raiz o resolve: aceitar por
    /// prefixo deixaria passar um dono inventado.
    #[test]
    fn module_relative_spec_outside_both_roots_is_blocking() {
        let root = std::env::temp_dir().join("vynamp-authoring-root2");
        let module = std::env::temp_dir().join("vynamp-authoring-module2");
        for d in [&root, &module] {
            let _ = fs::remove_dir_all(d);
            fs::create_dir_all(d).unwrap();
        }
        let body = "## Boundary\n\n- **Não** decide a curva do medidor: o valor sai de \
                    `specs/ui.md`, e adivinhar a curva faria o medidor divergir do \
                    que o MVP mediu.\n";
        let set = BTreeSet::new();
        let rules: Vec<_> = audit("TASK-E00-01", body, &set, &[&root, &module])
            .unwrap()
            .iter()
            .map(|f| f.rule)
            .collect();
        assert_eq!(rules, vec!["R1"]);
    }

    #[test]
    fn full_row_has_no_finding() {
        let body = "## Critérios de aceite\n\n| ID | Given | When | Then |\n\
|---|---|---|---|\n\
| `TASK-E00-01-AC-1` | um arquivo FLAC de 44,1 kHz com delay declarado | o Reader e aberto ate o fim | o PCM sai identico byte a byte |\n";
        assert_eq!(rules(body), Vec::<&str>::new());
    }

    /// A regressão de `TASK-E08-02`: o `Then` estava presente, mas a linha
    /// fechava sem o `|` final e a tabela tinha três células.
    #[test]
    fn row_without_closing_pipe_is_reported() {
        let body = "## Critérios de aceite\n\n| ID | Given | When | Then |\n\
|---|---|---|---|\n\
| `TASK-E00-01-AC-1` | um arquivo FLAC de 44,1 kHz com delay declarado | o Reader e aberto ate o fim | o PCM sai identico byte a byte\n";
        assert_eq!(rules(body), vec!["R7"]);
    }

    #[test]
    fn boundary_bullet_without_owner_is_blocking() {
        let body = "## Boundary\n\n- **Não** implementa a consulta da biblioteca; ela mora em outro lugar.\n";
        assert_eq!(rules(body), vec!["R1"]);
    }

    #[test]
    fn boundary_bullet_with_owner_passes() {
        let body = "## Boundary\n\n- **Não** implementa a consulta da biblioteca. O banco e `TASK-E03-01`; aqui e a tela que le o que ele expoe, e trazer a consulta para ca faria dois donos do mesmo schema.\n";
        assert_eq!(rules(body), Vec::<&str>::new());
    }

    /// O dono costuma estar na linha de continuação, não na primeira.
    #[test]
    fn continuation_line_satisfies_the_owner_rule() {
        let body = "## Boundary\n\n- **Não** implementa a consulta da biblioteca. O banco e\n  `TASK-E03-01`, e trazer a consulta para ca faria dois donos do mesmo schema.\n";
        assert_eq!(rules(body), Vec::<&str>::new());
    }

    /// Há fronteira que não pertence a TASK nenhuma, e o guia manda nomear a
    /// autoridade real: uma ADR. Exigir ID aí empurraria o autor a inventar um
    /// dono.
    #[test]
    fn boundary_bullet_with_an_adr_owner_passes() {
        let body = "## Boundary\n\n- **Não** copia código nem mídia do Winamp. Nenhuma TASK do backlog e dona disso: a autoridade e a ADR-001, que fixa o produto independente.\n";
        assert_eq!(rules(body), Vec::<&str>::new());
    }

    /// A autoridade também pode ser a spec do módulo, não só uma ADR.
    #[test]
    fn boundary_bullet_with_a_spec_owner_passes() {
        let body = "## Boundary\n\n- **Não** redefine a taxa de amostragem anunciada. A matriz de `specs/08-hires.md` e a autoridade, e mudar aqui divergiria dela sem ninguem medir.\n";
        assert_eq!(
            with_docs(body, &["ADR-001"], &["specs/08-hires.md"]),
            Vec::<&str>::new()
        );
    }

    /// Citar uma ADR que não existe é dono inventado, não autoridade.
    #[test]
    fn boundary_bullet_with_a_missing_adr_is_blocking() {
        let body = "## Boundary\n\n- **Não** copia codigo do Winamp. A autoridade e a ADR-999, que fixa o produto independente e ninguem revisou.\n";
        assert_eq!(rules(body), vec!["R1"]);
    }

    /// `design/` é normativo no umbrella, e conta como autoridade.
    #[test]
    fn boundary_bullet_with_a_design_doc_owner_passes() {
        let body = "## Boundary\n\n- **Não** copia arquivo do MVP para este repositorio. A regra esta em `design/07-pin-mvp.md`, que fixa URL, SHA e caminho da referencia e diz que codigo do MVP nao e codigo de produto.\n";
        assert_eq!(
            with_docs(body, &["ADR-001"], &["design/07-pin-mvp.md"]),
            Vec::<&str>::new()
        );
    }

    /// Um documento citado que não existe no repositório não é autoridade.
    #[test]
    fn boundary_bullet_with_a_missing_doc_is_blocking() {
        let body = "## Boundary\n\n- **Não** copia arquivo do MVP. A regra esta em `design/99-inexistente.md`, que fixa o pin e ninguem escreveu ainda.\n";
        assert_eq!(rules(body), vec!["R1"]);
    }

    /// `README.md` é citado para listar TASK, não para nomear dono de escopo.
    #[test]
    fn readme_is_not_an_owner() {
        let body = "## Boundary\n\n- **Não** implementa a consulta da biblioteca. Veja o README.md para a lista de tarefas e note que a consulta mora em outro lugar.\n";
        assert_eq!(rules(body), vec!["R1"]);
    }

    /// `SEM-DONO` é vocabulário do processo, não do documento: no bullet ele
    /// ocupa o lugar do dono sem dizer quem é.
    #[test]
    fn internal_marker_is_not_an_owner() {
        let body = "## Boundary\n\n- **Não** escolhe o toolkit. Compara as duas alternativas e entrega a evidencia para a ADR-003; a decisao e SEM-DONO, do mantenedor.\n";
        assert_eq!(rules(body), vec!["R1"]);
    }

    /// Um bullet que não nomeia TASK nem ADR não escapou: continua bloqueante.
    #[test]
    fn boundary_bullet_with_only_prose_is_still_blocking() {
        let body = "## Boundary\n\n- **Não** implementa a consulta da biblioteca; ela mora em outro lugar e nao e nosso trabalho aqui.\n";
        assert_eq!(rules(body), vec!["R1"]);
    }

    #[test]
    fn template_marker_short_circuits_the_file() {
        let body = "<!-- PENDENTE: autorar -->\n\n## Boundary\n\n- **Não** faz nada.\n";
        assert_eq!(rules(body), vec!["R9"]);
    }

    #[test]
    fn empty_given_is_blocking_and_short_given_is_a_warning() {
        // Vazio bloqueia; curto apenas avisa. Os dois convivem: uma célula vazia
        // também tem menos de quatro palavras, e o achado que importa é o R4.
        assert_eq!(
            rules("| `TASK-E00-01-AC-1` |  | o Reader abre ate o fim | sai do reader |\n"),
            vec!["R4"]
        );
        assert_eq!(
            rules("| `TASK-E00-01-AC-1` | um FLAC | o Reader abre ate o fim | sai do reader |\n"),
            vec!["R3"]
        );
    }

    #[test]
    fn empty_then_is_blocking() {
        assert_eq!(
            rules(
                "| `TASK-E00-01-AC-1` | um arquivo FLAC de 44,1 kHz | o Reader abre ate o fim |  |\n"
            ),
            vec!["R8"]
        );
    }

    /// "verde no CI Gate" na linha de teste automatizado é a verdade do
    /// template; só no oráculo ela é proibida.
    #[test]
    fn ci_gate_is_only_forbidden_as_the_oracle() {
        let ok = "## Verificação\n\n| Teste automatizado | verde no CI Gate |\n";
        assert_eq!(rules(ok), Vec::<&str>::new());
        let bad = "## Verificação\n\n| Oráculo independente | verde no CI Gate |\n";
        assert_eq!(rules(bad), vec!["R5"]);
    }

    #[test]
    fn pending_verification_cell_is_blocking() {
        let body = "## Verificação\n\n| Hardware | PENDENTE |\n";
        assert_eq!(rules(body), vec!["R6"]);
    }

    #[test]
    fn a_section_title_that_only_starts_the_same_way_is_not_matched() {
        let body = "## Boundary da TASK\n\n- **Não** faz nada sem dono.\n";
        assert_eq!(rules(body), Vec::<&str>::new());
    }
}
