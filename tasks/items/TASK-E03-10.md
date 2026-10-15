# TASK-E03-10 — Completar recovery, export e large-library gate

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E03-10 / REQ-E03-10 |
| Fase / prioridade | M3 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | library — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E03-07](TASK-E03-07.md) — Implementar smart playlists e templates
- [TASK-E03-08](TASK-E03-08.md) — Implementar escrita de tags e capítulos em lote
- [TASK-E03-09](TASK-E03-09.md) — Entregar letras, bookmarks e histórico

## Entrega concreta

Crash restore/backup/export e relatório dataset100k.

Documento canônico: [spec](../../docs/library/01-library-playlists.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Fila restaura sem autoplay; missing files têm erro agrupado; ratings/bookmarks sobrevivem upgrade e rollback seguro.

## Verificação e evidência

QA-006, QA-007, QA-012 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
