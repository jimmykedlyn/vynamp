# Evidência — TASK-E21-01 e as 16 TASKs de E21

Data: 2026-11-12 · Ambiente: Windows 11 Pro 10.0.26300, bash (Git Bash), Rust toolchain do `rust-toolchain.toml`
Escopo: autoria documental. Nenhuma linha de código de encoder foi escrita e nenhum teste foi executado, porque nenhum teste existe.

## Gates do umbrella, executados neste commit

```
$ cargo xtask check-tasks
{"errors":[],"result":"passed","scope":"Backlog and TASK Markdown only; no build, no hardware","tasks":215}

$ cargo xtask check-authoring
{"blocking":0,"result":"passed","scope":"TASK Markdown only; no build, no hardware","tasks":215,"warnings":15}

$ cargo xtask check-docs   (recorte de "errors")
  "errors": [
    "Missing index: engineering/guia-desenvolvimento.md"
  ],
  "format_catalog_entries": 68,
```

O único erro de `check-docs` é o arquivo não rastreado `engineering/guia-desenvolvimento.md`,
que não é deste PR e não foi tocado. Os 15 avisos de `check-authoring` são todos anteriores;
zero deles é de TASK de E21. Num worktree limpo do mesmo commit, com os nove submódulos
inicializados — que é o estado do runner —, `check-docs` devolve `"errors": []`.

## CI do GitHub Actions, no SHA final

Os três PRs têm o CI verde no SHA revisado:

| PR | SHA | Jobs |
|---|---|---|
| [vynamp#56](https://github.com/jimmykedlyn/vynamp/pull/56) | `e043418` | Authoring, CI Gate, Regras de arquitetura, Specs, Submodules fixados, TASKs, Tooling ×3 — 9 de 9 |
| [vynamp-encode#7](https://github.com/jimmykedlyn/vynamp-encode/pull/7) | `be5859d` | CI Gate, clippy, fmt, machete, test — 5 de 5 |
| [vynamp-converter#14](https://github.com/jimmykedlyn/vynamp-converter/pull/14) | `873595f` | CI Gate, build ×4, clippy, fmt, test — 8 de 8 |

O merge dos três é do mantenedor; os PRs não o fazem.

## Por que 16 TASKs e de onde sai cada uma

| TASK | REQ | Spec | Prioridade | Depende de |
|---|---|---|---|---|
| `TASK-E21-01` | `REQ-E21-01` | `libs/encode/specs/encode.md` | P0 | — |
| `TASK-E21-02` | `REQ-E21-02` | `libs/encode/specs/encode.md` | P1 | `TASK-E21-01`, `TASK-E11-03` |
| `TASK-E21-03` | `REQ-E21-03` | `libs/encode/specs/encode.md` | P0 | `TASK-E21-01`, `TASK-E01-07` |
| `TASK-E21-04` | `REQ-E21-04` | `libs/encode/specs/encode.md` | P1 | `TASK-E21-01` |
| `TASK-E21-05` | `REQ-E21-05` | `libs/encode/specs/encode.md` | P0 | `TASK-E21-01`, `TASK-E10-03` |
| `TASK-E21-06` | `REQ-E21-06` | `libs/encode/specs/encode.md` | P1 | `TASK-E21-01` |
| `TASK-E21-07` | `REQ-E21-07` | `libs/encode/specs/encode.md` | P1 | `TASK-E21-01` |
| `TASK-E21-08` | `REQ-E21-08` | `libs/encode/specs/encode.md` | P2 | `TASK-E21-01` |
| `TASK-E21-09` | `REQ-E21-09` | `libs/encode/specs/encode.md` | P1 | `TASK-E21-01` |
| `TASK-E21-10` | `REQ-E21-10` | `libs/encode/specs/formatos.md` | P0 | `TASK-E21-01` |
| `TASK-E21-11` | `REQ-E21-11` | `libs/encode/specs/formatos.md` | P1 | `TASK-E21-10` |
| `TASK-E21-12` | `REQ-E21-12` | `libs/encode/specs/formatos.md` | P1 | `TASK-E21-10` |
| `TASK-E21-13` | `REQ-E21-13` | `libs/encode/specs/formatos.md` | P1 | `TASK-E21-10` |
| `TASK-E21-14` | `REQ-E21-14` | `libs/encode/specs/formatos.md` | P1 | `TASK-E21-10` |
| `TASK-E21-15` | `REQ-E21-15` | `libs/encode/specs/formatos.md` | P1 | `TASK-E21-01` |
| `TASK-E21-16` | `REQ-E21-16` | `libs/encode/specs/formatos.md` | P2 | `TASK-E21-01` |

O `Then` de cada critério de aceite é cópia literal da tabela §5 da spec dona;
`Given` e `When` foram autorados. O backlog é derivado do Markdown, então as duas
leituras do mesmo fato não podem divergir sem que o gate veja.

## O conflito de nome que este PR resolve

`ENCODE-REQ-01` põe `Registry`, `availability()`, `formats()` e `open()` dentro de
`vynamp-encode`. O título de `TASK-E08-02` dizia "conversor e encoder registry" e a
fronteira dela dizia "Aqui se entrega o registry que escolhe o encoder". AGENTS.md §2
proíbe o mesmo nome com dois significados.

Resolução: o registry é `TASK-E21-01`, na lib; o conversor fica com a orquestração do
lote e a validação pós-escrita. Mudaram, no mesmo PR:

- título de `TASK-E08-02` → "Entregar conversor em lote e validação pós-escrita", nos quatro lugares que o repetem (backlog, matriz, épico E08, README do repo);
- bullet de Boundary de `TASK-E08-02` sobre o registry de decodificação;
- bullet de Boundary de `TASK-E18-02` que atribuía o registry a `TASK-E08-02`;
- `reference/audio/02-formats.md` e `reference/tools/02-vynamp-converter.md`.

Nenhum ID mudou. Nenhum código mudou.

## O que não existe, e por isso não foi provado

Limitação declarada não reprova a entrega; limitação omitida reprova.

| Fato medido | Comando | Efeito |
|---|---|---|
| `libs/encode/src/lib.rs` tem 6 linhas e só `pub const VERSION` | `wc -l libs/encode/src/lib.rs` | nenhum tipo das duas specs existe |
| `[dependencies]` do `Cargo.toml` está vazio | `cat libs/encode/Cargo.toml` | não há `vynamp-dsp`, não há feature `resample` |
| não existe `tests/` no repo | `ls libs/encode/tests` | nenhum dos comandos de §9 das specs roda |
| o repo `apps/converter` tem 1 `.rs` de 3 linhas | `git -C apps/converter ls-files "*.rs"` | a varredura de tabela de lossy de E21-15 devolve zero por ausência de código |

Nenhuma das 16 TASKs pode sair de `planned` até a spec ser aprovada pelo mantenedor:
as duas specs estão em "rascunho autorado, não aprovado" no cabeçalho.

## Arquivos tocados

- `libs/encode` (submódulo): `tasks/README.md`, `tasks/items/TASK-E21-01..16.md`, `specs/encode.md`, `specs/formatos.md`
- `apps/converter` (submódulo): `tasks/README.md`, `tasks/items/TASK-E08-02.md`, `tasks/items/TASK-E18-02.md`
- umbrella: `tasks/backlog.json`, `product/03-requisitos.md`, `tasks/epics/E21.md`, `tasks/epics/E08.md`, `INDEX.md`, `reference/audio/02-formats.md`, `reference/tools/02-vynamp-converter.md`
