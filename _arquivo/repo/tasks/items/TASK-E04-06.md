# TASK-E04-06 — Entregar modo transparente e diagnósticos hi-res

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E04-06 / REQ-E04-06 |
| Fase / prioridade | M3 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | audio — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E04-05](TASK-E04-05.md) — Entregar saída exclusiva e device profiles
- [TASK-E04-03](TASK-E04-03.md) — Construir grafo DSP e transições sem glitches

## Entrega concreta

Integer direct path, active conditions, source/output labels e validated combinations.

Documento canônico: [spec](../../reference/audio/04-outputs-hires.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

PCM24/32 capture exata em hardware validado; EQ/RG/volume digital desligados; fonte lossy não recebe selo errado.

## Verificação e evidência

QA-003, QA-004 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
