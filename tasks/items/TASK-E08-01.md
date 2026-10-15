# TASK-E08-01 — Implementar jobs persistidos

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E08-01 / REQ-E08-01 |
| Fase / prioridade | M2 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | library — pessoa a designar |
| Esforço provisório | 5–12 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E03-01](TASK-E03-01.md) — Implementar schema SQLite e migrations seguras

## Entrega concreta

Scheduler de jobs com checkpoints, leases, progresso e pools limitados.

Documento canônico: [spec](../../docs/library/02-tags-lyrics-tools.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Crash retoma jobs seguros; cancelar remove temporários sem apagar source; erros parciais são visíveis e progresso é consistente.

## Verificação e evidência

QA-016 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
