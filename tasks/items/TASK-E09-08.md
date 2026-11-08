# TASK-E09-08 — Entregar sincronização de dados do ouvinte

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E09-08 / REQ-E09-08 |
| Fase / prioridade | M4 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | streaming — pessoa a designar |
| Esforço provisório | 8–20 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E09-03](TASK-E09-03.md) — Implementar identidade e autorização
- [TASK-E09-04](TASK-E09-04.md) — Implementar catálogo e administração editorial
- [TASK-E03-10](TASK-E03-10.md) — Completar recovery, export e large-library gate

## Entrega concreta

Outbox, changelog, cursores, revisões, tombstones e UI de conflitos.

Documento canônico: [spec](../../streaming/03-plataforma.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Merge offline de reorder/remove preserva intenção; 409 preserva edição; operação repetida idempotente; paths locais não sobem por padrão.

## Verificação e evidência

QA-013 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
