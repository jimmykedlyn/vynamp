# TASK-E03-07 — Implementar smart playlists e templates

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E03-07 / REQ-E03-07 |
| Fase / prioridade | M3 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | library — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E03-05](TASK-E03-05.md) — Entregar FTS, filtros e views de biblioteca
- [TASK-E03-06](TASK-E03-06.md) — Entregar playlists import/export e CUE

## Entrega concreta

AST allowlisted, folders rules, sort/group/lock e refresh.

Documento canônico: [spec](../../docs/library/01-library-playlists.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Query invalid rejeitada; atualização cancelável; playlist lock não bloqueia playback; snapshots consistentes.

## Verificação e evidência

QA-006 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
