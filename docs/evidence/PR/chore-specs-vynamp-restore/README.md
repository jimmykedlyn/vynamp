# Evidência — chore/claude-specs-vynamp-restore (PR4)

Data: 2026-11-05 · Autor: Claude Opus 5.5 (Orquestrador) · Revisor: subagente Opus (contexto limpo).

Escopo: especificação normativa do **Vynamp Restore** (ferramenta desktop dedicada) e preparação do **Vynamp Converter**, com ADR, épico E18, matriz normativa e backlog. **Nenhuma linha de código de restauração, conversão ou encoder foi implementada ou executada** — esta pasta registra apenas o que foi verificado localmente.

## Ambiente

| Item | Valor |
|---|---|
| SO | Windows 11 Pro 10.0.26300 |
| Rust | 1.99.0 |
| Worktree | `repo/.worktrees/claude/specs-vynamp-restore` |
| Branch | `chore/claude-specs-vynamp-restore` |
| Base | `chore/claude-specs-shell-integration` (64b7487) |
| Referência factual | MVP web rev.38 em `vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452` |

## Entregas

| Artefato | Conteúdo |
|---|---|
| `docs/architecture/adr/ADR-013-vynamp-restore.md` | Decisão: Restore como processo separado com motor de restauração reaproveitável; escopo do que **não** é restauração; correção das incoerências do MVP |
| `docs/tools/01-vynamp-restore.md` | Spec normativa do `vynamp-restore`: CLI, cadeia de 11 estágios DSP com equações e faixas de parâmetro, 12 detectores de defeito, regiões, A/B, lote, metadados, exportação determinística, projeto `.vynamp-restore`, matriz de plataformas |
| `docs/tools/02-vynamp-converter.md` | Escopo preparatório do `vynamp-converter`: matriz de formatos, rótulagem lossless/lossy, fila de jobs, fronteira da lib `vynamp-encode` |
| `tasks/epics/E18.md` | Épico E18 — Ferramentas de arquivo: Restore e Converter |
| `tasks/items/TASK-E18-01.md`, `TASK-E18-02.md` | Contratos das duas tarefas |
| `docs/product/03-requirements.md` | REQ-E18-01 e REQ-E18-02 (184 requisitos/tarefas em 18 épicos) |
| `tasks/backlog.json` | Entradas TASK-E18-01 e TASK-E18-02 com paridade verbatim de `title`/`deliverable`/`acceptance` |
| `docs/INDEX.md`, `docs/GLOSSARY.md`, `docs/product/06-feature-parity.md`, `CHANGELOG.md` | Índice, glossário, catálogo de funcionalidades e histórico |

## Verificações locais

| Comando | Resultado |
|---|---|
| `cargo xtask check-docs` | `result: passed`, 0 erros, 336 arquivos Markdown, 184 tarefas, 184 requisitos, 18 épicos, 68 entradas de catálogo |
| `cargo xtask check-delivery` | `result: passed`, 0 erros |
| `cargo test -p xtask` | 85 aprovados, 0 falhas |
| Hook de pre-push (`xtask ci docs` + testes de `xtask` e `tests/tooling.rs`) | Aprovado — 51 + 34 testes, 0 falhas |

## Fidelidade ao MVP rev.38 e correções

O inventário factual (algoritmos, faixas de parâmetro, diagnósticos, presets) vem do MVP rev.38 fixado por hash. Três incoerências do protótipo foram corrigidas na versão nativa e **declaradas** em [ADR-013 §4](../../../architecture/adr/ADR-013-vynamp-restore.md):

1. **De-crackle** — o módulo `mid/side` do MVP é código morto; o de-crackle realmente executado é o modelo AR/L-SAR, que é o que a spec normativa fixa (ordem P=48, 2 passagens por canal).
2. **Denoise Wiener** — inalcançável no MVP (sem UI e descartado pelo state store); declarado **fora de escopo** da 1.0 em vez de silenciosamente ausente.
3. **Região `kind: overlay`** — placeholder ("AI separation pending") no MVP; a 1.0 suporta apenas regiões de ganho/fade (`kind: between`).

Nenhum número de parâmetro foi inventado: todos vêm do código do MVP no pin.

## Revisão independente

Subagente Opus com contexto limpo (alto risco: define arquitetura de uma ferramenta de áudio) auditou:

- `docs/architecture/adr/ADR-013-vynamp-restore.md`
- `docs/tools/01-vynamp-restore.md` e `docs/tools/02-vynamp-converter.md`
- `tasks/epics/E18.md`, `tasks/items/TASK-E18-01.md`, `TASK-E18-02.md` e `tasks/backlog.json`
- `docs/product/03-requirements.md`, `docs/INDEX.md`, `docs/GLOSSARY.md`, `docs/product/06-feature-parity.md` e `CHANGELOG.md`

**Veredito e achados:** ver a seção de revisão ao final deste arquivo.

## Limitações

- **CI Gate (Woodpecker) não verificado** — a instrução vigente do mantenedor é ignorar o CI/CD neste momento; o mantenedor confere o CI Gate do SHA final antes do merge.
- **Nenhuma restauração, conversão ou codificação foi executada.** A validação é documental e estrutural.
- **macOS não verificado** (sem agente Mac neste ambiente) e **hardware de saída de áudio não testado**.
- O **PR5** (execução: lib `vynamp-encode`, fila de jobs, artefatos de release) está fora deste PR; este entrega o escopo fechado.
- A especificação do Converter é **preparatória**: os documentos de contrato final de `vynamp-encode` e a evidência de execução ficam no PR5.

## Artefato

O corpo do PR está versionado em [pr-body.md](pr-body.md).
