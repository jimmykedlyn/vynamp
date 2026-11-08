# TASK-E08-05 — Entregar sleep timer e scheduler

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E08-05 / REQ-E08-05 |
| Fase / prioridade | M3 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | library — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E08-01](TASK-E08-01.md) — Implementar jobs persistidos
- [TASK-E01-03](TASK-E01-03.md) — Implementar state machine e generation cancellation

## Entrega concreta

Timers, alarmes, regras de horário/DST, quiet hours e persistência.

Documento canônico: [spec](../../library/02-tags-letras.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Parar ao fim de faixa funciona; DST definido; desligar OS só se habilitado; alarmes respeitam teto de volume.

## Verificação e evidência

QA-016 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
