# TASK-E02-06 — Entregar Mini, preferências e estados de erro

Versão: 0.7.1 · Data: 2026-11-03 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E02-06 / REQ-E02-06 |
| Fase / prioridade | M2 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | design — pessoa a designar |
| Esforço provisório | 5–12 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E02-05](TASK-E02-05.md) — Construir Library Workspace e inspector

## Entrega concreta

Mini420×112, navegação preferences e empty/loading/offline/recovery.

Referência visual: vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452 (rev.38), caminhos `mvp/...` e `out/...` conforme [referência do MVP](../../design/07-pin-mvp.md).

Documento canônico: [spec](../../design/02-layouts.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Trocar modo mantém áudio; Escape/focus return corretos; erros mostram ações úteis sem stacktrace no fluxo.

## Verificação e evidência

QA-001, QA-010 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
