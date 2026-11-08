# TASK-E04-04 — Entregar EQ paramétrico e presets

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E04-04 / REQ-E04-04 |
| Fase / prioridade | M3 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | audio — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E04-03](TASK-E04-03.md) — Construir grafo DSP e transições sem glitches

## Entrega concreta

32bands/tipos/Q/canal, EQ20/31 e preset schema/precedence.

Documento canônico: [spec](../../reference/audio/04-outputs-hires.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Curva±0.2dB da referência; frequências válidas porNyquist; import inválido mantém preset anterior.

## Verificação e evidência

QA-005 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
