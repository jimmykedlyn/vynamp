# TASK-E08-03 — Entregar análise de loudness, ReplayGain e BPM

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E08-03 / REQ-E08-03 |
| Fase / prioridade | M3 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | library — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E08-01](TASK-E08-01.md) — Implementar jobs persistidos
- [TASK-E04-01](TASK-E04-01.md) — Implementar EQ10, ganho e ReplayGain playback

## Entrega concreta

Jobs com padrão/versão de análise, grouping de álbum e escrita opcional de tags.

Documento canônico: [spec](../../docs/library/02-tags-lyrics-tools.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Loudness conferido contra fixtures; agrupamento correto; BPM inclui confiança; scan não interfere callback; write requer preview.

## Verificação e evidência

QA-005, QA-016 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
