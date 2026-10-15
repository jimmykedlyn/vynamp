# TASK-E12-10 — Completar EQ64 e escala avançada

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E12-10 / REQ-E12-10 |
| Fase / prioridade | M5 / P2 |
| Estado | Planejada; não implementada |
| Responsável funcional | expansion — pessoa a designar |
| Esforço provisório | 10–25 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E04-10](TASK-E04-10.md) — Validar qualidade numérica e hardware matrix
- [TASK-E03-10](TASK-E03-10.md) — Completar recovery, export e large-library gate
- [TASK-E12-03](TASK-E12-03.md) — Validar ARM64 adicional Windows/Linux e helpers

## Entrega concreta

EQ64, bandas por canal, surround opcional, biblioteca de 1 milhão e RTL completo.

Documento canônico: [spec](../../docs/delivery/01-roadmap.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Medir CPU/latência e biblioteca; emulação surround rotulada; RTL/a11y validados; voice remover experimental explicitado.

## Verificação e evidência

QA-005, QA-006, QA-010 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
