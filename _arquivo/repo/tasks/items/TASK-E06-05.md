# TASK-E06-05 — Entregar meters e espectro

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E06-05 / REQ-E06-05 |
| Fase / prioridade | M2 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | extensions — pessoa a designar |
| Esforço provisório | 5–12 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E01-05](TASK-E01-05.md) — Implementar mixer, drain e transições básicas
- [TASK-E02-03](TASK-E02-03.md) — Construir Classic Stack e transporte

## Entrega concreta

Canal de observação de áudio independente, FFT, VU, espectro e osciloscópio.

Documento canônico: [spec](../../extensions/01-temas.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Renderer lento não causa underrun; escalas de FFT correspondem à taxa; minimizar e reduzir movimento diminuem processamento.

## Verificação e evidência

QA-017 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
