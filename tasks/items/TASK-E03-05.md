# TASK-E03-05 — Entregar FTS, filtros e views de biblioteca

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E03-05 / REQ-E03-05 |
| Fase / prioridade | M2 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | library — pessoa a designar |
| Esforço provisório | 5–12 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E03-04](TASK-E03-04.md) — Implementar leitura de tags e capas segura

## Entrega concreta

Query port indexado, albums/artists/genres/folders e pagination.

Documento canônico: [spec](../../docs/library/01-library-playlists.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Search100k warmp95≤100ms; filtros combinados não executam SQL do usuário; ordering estável.

## Verificação e evidência

QA-006 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
