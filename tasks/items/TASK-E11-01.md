# TASK-E11-01 — Criar workspace e baseline de engenharia

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. Critério de aceite: **PENDENTE** de autoria —
> a coluna `Then` é o requisito aprovado, copiado literal; `Given` e `When` ainda
> não existem.

| Campo | Valor |
|---|---|
| ID | `TASK-E11-01` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E11-01` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M0 |
| Prioridade | P0 |
| Estado | implemented |
| Depende de | `TASK-E15-02` |
| Bloqueia | `TASK-E01-01`, `TASK-E01-02`, `TASK-E02-01`, `TASK-E03-01`, `TASK-E07-01`, `TASK-E10-01`, `TASK-E11-02`, `TASK-E11-03`, `TASK-E11-04`, `TASK-E13-01`, `TASK-E14-01`, `TASK-E15-05`, `TASK-E16-01`, `TASK-E17-01` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Repositório independente, Cargo workspace mínimo, toolchain pin, ADRs e CI inicial.

## Boundary

<!-- PENDENTE: o que esta TASK não faz, mesmo parecendo relacionado. -->

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e11-01-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E11-01-AC-1` | o repositório inteiro, incluindo histórico e assets binários | uma varredura por código, mídia ou artefato do Winamp roda | Nenhum código/asset Winamp |
| `TASK-E11-01-AC-2` | um clone sem `target/` e com a toolchain de `rust-toolchain.toml` | `cargo build --workspace --locked` roda | build real com lockfile resolvido |
| `TASK-E11-01-AC-3` | o `INDEX.md` e o backlog no estado do commit | `cargo xtask check-docs` roda | validador de docs passa |
| `TASK-E11-01-AC-4` | cada script de fixture que referencia um arquivo de áudio | o caminho é resolvido a partir da raiz do repositório | fixtures têm paths definidos. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | PENDENTE | comparação numérica, não "parece certo" |
| Hardware | PENDENTE | ausência é limitação registrada, não aprovação |

## DoD — pronto para review

- [ ] Todos os critérios de aceite com evidência
- [ ] `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` limpos
- [ ] `cargo test` verde; testes ignorados listados com motivo
- [ ] Spec do módulo atualizada se a implementação revelou algo novo
- [ ] `backlog.json` com o estado correto
- [ ] Revisão feita por contexto limpo, não pelo implementador
- [ ] PR aberto com template preenchido; CI Gate verde no SHA final
## Notas de execução

Fechada em 2026-11-09 por verificação cláusula a cláusula contra o estado atual do
repositório, não por leitura da evidência original (Codex, 2026-10-18). O que a
evidência dizia continua no arquivo; o que segue foi medido agora.

| AC | Verificação executada | Resultado |
|---|---|---|
| AC-1 | `git grep -In -i -E "winamp|nullsoft"` na árvore rastreada (fora de `_arquivo/`) | 0 arquivos; nenhum asset binário Winamp |
| AC-2 | `cargo build --workspace --locked` | `Finished dev profile … in 10.41s`, exit 0 |
| AC-3 | `cargo xtask check-docs` | `"result": "passed"`, 193 TASKs, `spec_version 0.7.0` |
| AC-4 | `libs/decode/tools/fixtures/generate.ps1:14` | raiz resolvida por `Split-Path -Parent (Split-Path -Parent $PSScriptRoot)`; saídas em `tests/fixtures/{pcm,flac,mpeg}` |

AC-4 não é a mesma coisa que "existe fixture": o que o critério pede é que o
caminho seja resolvido a partir da raiz do repositório, e ele é — nenhum script
depende do diretório de onde foi chamado.

O que esta TASK **não** provou, e continua fora dela: nenhum áudio real foi
decodificado, nenhum hardware foi tocado, e as flags `application_ready`,
`release_enabled` e `branding_exports_ready` seguem `false`.
