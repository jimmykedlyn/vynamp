# TASK-E04-05 — Entregar saída exclusiva e device profiles

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E04-05 / REQ-E04-05 |
| Fase / prioridade | M3 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | audio — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E01-02](TASK-E01-02.md) — Provar callback e output clock
- [TASK-E04-02](TASK-E04-02.md) — Provar resampler e sample conversion

## Entrega concreta

WASAPI event-exclusive, stable IDs, capabilities e ASIO adapter opcional.

Documento canônico: [spec](../../docs/audio/04-outputs-hires.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

24/96 e24/192 negociados emDAC; busy/unplug/sleep recuperam com pausa; sem fallback speaker automático.

## Verificação e evidência

QA-003, QA-011 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
