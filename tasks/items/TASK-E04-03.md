# TASK-E04-03 — Construir grafo DSP e transições sem glitches

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E04-03 / REQ-E04-03 |
| Fase / prioridade | M2 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | audio — pessoa a designar |
| Esforço provisório | 5–12 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E04-01](TASK-E04-01.md) — Implementar EQ10, ganho e ReplayGain playback
- [TASK-E04-02](TASK-E04-02.md) — Provar resampler e sample conversion

## Entrega concreta

Compiled graph, parameter interpolation, latency annotation e crossfade.

Documento canônico: [spec](../../docs/audio/04-outputs-hires.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Graph swap foraRT; no cycles/NaN; equal-power crossfade preserva duração, desligado não altera gapless.

## Verificação e evidência

QA-002, QA-005 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
