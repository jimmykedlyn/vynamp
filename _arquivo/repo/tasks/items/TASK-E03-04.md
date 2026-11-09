# TASK-E03-04 — Implementar leitura de tags e capas segura

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E03-04 / REQ-E03-04 |
| Fase / prioridade | M2 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | library — pessoa a designar |
| Esforço provisório | 5–12 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E03-03](TASK-E03-03.md) — Implementar import incremental e roots offline

## Entrega concreta

Tag adapters, multivalue metadata, thumbnails/cache e cover precedence.

Documento canônico: [spec](../../library/01-biblioteca-playlists.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Unknown fields preservados para futuro write; image bomb limitada; metadata Unicode mantém representação original.

## Verificação e evidência

QA-007, QA-015 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
