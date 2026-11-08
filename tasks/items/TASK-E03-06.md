# TASK-E03-06 — Entregar playlists import/export e CUE

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E03-06 / REQ-E03-06 |
| Fase / prioridade | M2 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | library — pessoa a designar |
| Esforço provisório | 5–12 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E03-02](TASK-E03-02.md) — Implementar queue independente e shuffle
- [TASK-E03-04](TASK-E03-04.md) — Implementar leitura de tags e capas segura

## Entrega concreta

M3U/M3U8/PLS/XSPF/CUE, relative locators e virtual ranges.

Documento canônico: [spec](../../library/01-biblioteca-playlists.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Loop de playlists bloqueado; CUE75fps preserva offsets; export sem token; undo/redo100 ops da sessão.

## Verificação e evidência

QA-002, QA-006 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
