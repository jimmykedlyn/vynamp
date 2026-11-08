# TASK-E02-04 — Entregar playlist UI e keyboard workflow

Versão: 0.7.1 · Data: 2026-11-03 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E02-04 / REQ-E02-04 |
| Fase / prioridade | M1 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | design — pessoa a designar |
| Esforço provisório | 3–8 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E02-03](TASK-E02-03.md) — Construir Classic Stack e transporte
- [TASK-E03-02](TASK-E03-02.md) — Implementar queue independente e shuffle

## Entrega concreta

Lista virtualizada, drag insert, seleção, context menu e queue actions.

Referência visual: vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452 (rev.38), caminhos `mvp/...` e `out/...` conforme [referência do MVP](../../design/07-pin-mvp.md).

Documento canônico: [spec](../../design/02-layouts.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Tocar/agendar/adicionar têm comportamentos distintos; teclado completo e faixa atual separada da seleção.

## Verificação e evidência

QA-006, QA-010 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
