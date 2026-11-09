# TASK-E01-09 — Entregar channel mapping e DSD para PCM

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E01-09 / REQ-E01-09 |
| Fase / prioridade | M3 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | audio — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E01-08](TASK-E01-08.md) — Completar codecs desktop e large files
- [TASK-E04-02](TASK-E04-02.md) — Provar resampler e sample conversion

## Entrega concreta

Mapas mono/stereo/5.1/7.1 e DSF/DFF decimator com diagnóstico.

Documento canônico: [spec](../../reference/audio/01-engine.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Canal mapeado corretamente, headroom aplicado em downmix; DSD64/128/256 fixture não é rotulado native.

## Verificação e evidência

QA-003, QA-015 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
