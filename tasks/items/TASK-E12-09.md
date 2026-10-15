# TASK-E12-09 — Entregar casting e companion

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E12-09 / REQ-E12-09 |
| Fase / prioridade | M5 / P2 |
| Estado | Planejada; não implementada |
| Responsável funcional | expansion — pessoa a designar |
| Esforço provisório | 10–25 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E08-10](TASK-E08-10.md) — Entregar controle remoto autenticado
- [TASK-E09-10](TASK-E09-10.md) — Validar operação, carga e recuperação

## Entrega concreta

Bridges Chromecast/UPnP/AirPlay conforme APIs e controles pareados.

Documento canônico: [spec](../../docs/delivery/01-roadmap.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Capacidade real do destino; limites de clock/transferência visíveis; negar cliente não pareado; sem claims hi-res não verificados.

## Verificação e evidência

QA-009, QA-017 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
