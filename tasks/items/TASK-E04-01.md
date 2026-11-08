# TASK-E04-01 — Implementar EQ10, ganho e ReplayGain playback

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E04-01 / REQ-E04-01 |
| Fase / prioridade | M1 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | audio — pessoa a designar |
| Esforço provisório | 3–8 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E01-05](TASK-E01-05.md) — Implementar mixer, drain e transições básicas

## Entrega concreta

EQ gráfico clássico, preamp, volume/pan e RG track/album.

Documento canônico: [spec](../../reference/audio/04-outputs-hires.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

On/off/flat coerentes; metadata absent usa ganho configurado; coefficient swap sem zipper e sem allocRT.

## Verificação e evidência

QA-005 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
