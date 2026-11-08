# TASK-E04-02 — Provar resampler e sample conversion

Versão: 0.7.1 · Data: 2026-11-03 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E04-02 / REQ-E04-02 |
| Fase / prioridade | M0 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | audio — pessoa a designar |
| Esforço provisório | 2–5 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E01-01](TASK-E01-01.md) — Provar decode, gapless e precisão de samples
- [TASK-E01-02](TASK-E01-02.md) — Provar callback e output clock

## Entrega concreta

Spike da lib própria de resampling (ADR-011), PCM24/32 packing e ADR de qualidade.

Documento canônico: [spec](../../reference/audio/04-outputs-hires.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Impulse/sweep e alias/CPU/latência registrados; integer bypass sample-exact e dither só ao reduzir precisão.

## Verificação e evidência

QA-003, QA-004, QA-005 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
