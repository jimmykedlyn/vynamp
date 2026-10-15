# TASK-E05-05 — Entregar feeds e episódios de podcasts

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E05-05 / REQ-E05-05 |
| Fase / prioridade | M2 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | streaming — pessoa a designar |
| Esforço provisório | 5–12 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E05-04](TASK-E05-04.md) — Entregar buffers, cache e downloads
- [TASK-E03-04](TASK-E03-04.md) — Implementar leitura de tags e capas segura

## Entrega concreta

RSS/Atom, assinaturas, polling condicional, downloads e progresso por episódio.

Documento canônico: [spec](../../docs/streaming/01-client.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Desabilitar entidades XML externas; GUID por feed; cancelar download preserva parcial válido; eventos de progresso são idempotentes.

## Verificação e evidência

QA-016 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
