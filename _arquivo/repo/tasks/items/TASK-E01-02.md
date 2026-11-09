# TASK-E01-02 — Provar callback e output clock

Versão: 0.7.1 · Data: 2026-11-07 · Estado: em execução; spike medido em Windows/WASAPI, código ainda no [PR #14](https://github.com/jimmykedlyn/vynamp-mediaplayer/pull/14) (draft, não integrado), sem aceite.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E01-02 / REQ-E01-02 |
| Fase / prioridade | M0 / P0 |
| Estado | Em execução — critérios provados só em Windows/WASAPI; [evidência](../../evidence/PR/task-e01-02/README.md) |
| Responsável funcional | audio — pessoa a designar |
| Esforço provisório | 2–5 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E11-01](TASK-E11-01.md) — Criar workspace e baseline de engenharia

## Entrega concreta

Spike shared output, ring SPSC e harness de callback com instrumentation.

Documento canônico: [spec](../../reference/audio/01-engine.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Nenhum lock/alloc/I/O na callback; clock deriva consumo e device latency; underrun gera zeros sem replay.

Estado de cada critério em 2026-11-07, todos ainda **só em Windows/WASAPI shared**:

| Critério | Medição | Falta para aceitar |
|---|---|---|
| Nenhum lock/alloc/I/O na callback | 0 eventos de heap em 3 022 callbacks sem stall e 3 052 com stall; guarda estática nas regiões RT | Revisão do código; a instrumentação cobre o código próprio, não o heap de CPAL/OS ao redor |
| Clock deriva consumo e device latency | 0 regressões de posição nos dois runs; latência 12 304–12 802 frames | Aceite da semântica de latência, e o viés de drenagem da §Limitação conhecida |
| Underrun gera zeros sem replay | 25 underruns / 94 976 frames zerados sob stall de 300 ms, consumo igual ao produzido | Soak de 8 h (`QA` de performance), ainda não executado |

## Verificação e evidência

QA-001, QA-003 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

Evidência desta TASK: [evidence/PR/task-e01-02](../../evidence/PR/task-e01-02/README.md) — os dois JSON do probe com SHA-256, o ambiente e a lista do que passou para [01-engine.md](../../reference/audio/01-engine.md).

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.

## Limitação conhecida

O relógio apresentado supõe que a fila do dispositivo contém só conteúdo. Silêncio de underrun ou drenagem enfileirado também entra na latência reportada, então `presented` fica subestimado em no máximo `latency_frames` — ≈12 300–12 800 frames no hardware medido. Após um underrun atrasa até a latência inteira; na drenagem estabiliza em `content_frames − latency_frames`. Nunca superestima nem regride. Correção pertence ao dreno completo da [TASK-E01-05](TASK-E01-05.md); sem redesenho nesta TASK. Detalhe em [01-engine.md](../../reference/audio/01-engine.md) §Threads e na [evidência](../../evidence/PR/task-e01-02/README.md).
