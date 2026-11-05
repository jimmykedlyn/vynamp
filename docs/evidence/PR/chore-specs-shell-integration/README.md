# Evidência — chore/claude-specs-shell-integration (PR3)

Data: 2026-11-05 · Autor: Claude Opus 5.5 (Orquestrador) · Revisor: subagente Opus (contexto limpo).

Escopo: especificação e assets de integração com o shell dos sistemas operacionais e pacotes de ícones de arquivo. **Nenhuma integração foi implementada ou testada em sistema operacional** — esta pasta registra apenas o que foi verificado localmente.

## Ambiente

| Item | Valor |
|---|---|
| SO | Windows 11 Pro 10.0.26300 |
| Rust | 1.99.0 |
| Worktree | `repo/.worktrees/claude/specs-shell-integration` |
| Branch | `chore/claude-specs-shell-integration` |
| Base | `chore/claude-specs-mvp-reference` (37653d3) |
| Commit revisado | 7ba0635 |

## Verificações locais

| Comando | Resultado |
|---|---|
| `cargo xtask check-docs` | `result: passed`, 0 erros, 327 arquivos Markdown, 182 tarefas, 182 requisitos, 68 entradas de catálogo |
| `cargo xtask check-delivery` | `result: passed`, 0 erros |
| `cargo test -p xtask` | 85 aprovados, 0 falhas |
| Hook de pre-push (`xtask ci docs` + testes de `xtask` e `tests/tooling.rs`) | Aprovado — 51 + 34 testes, 0 falhas |

## Revisão independente

Subagente Opus com contexto limpo (alto risco: define arquitetura de integração multi-plataforma) auditou:

- `docs/architecture/adr/ADR-012-shell-integration.md`
- `docs/platforms/03-system-integration.md`
- `assets/icons/README.md`, `manifest.json`, `preview.html` e os vetores SVG
- `tasks/items/TASK-E16-03.md`, `TASK-E16-04.md`, `TASK-E16-05.md`, `TASK-E16-08.md` e `tasks/backlog.json`
- `docs/INDEX.md`, `docs/GLOSSARY.md`, `CHANGELOG.md` e `VALIDATION.json`

**Veredito: APROVADO** — coerência dos contratos de shell por plataforma, fidelidade das 3 famílias de ícones ao MVP rev.38, cobertura dos 17 formatos, separação das ferramentas Restore/Converter, paridade entre `tasks/items` e `tasks/backlog.json`, e integridade de índice e glossário. Nenhum achado exigiu correção.

## Limitações

- **CI Gate (Woodpecker) não verificado.** A rede do agente não alcançou `api.github.com` com estabilidade ao longo do trabalho; o mantenedor fica responsável por conferir o CI Gate do SHA final antes do merge.
- **macOS não verificado** (sem agente Mac neste ambiente) e **hardware de saída de áudio não testado**.
- Nenhuma integração real foi exercitada: a validação é documental e estrutural.

## Artefato

O corpo do PR está versionado em [pr-body.md](pr-body.md).
