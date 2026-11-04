# TASK-E02-03 — Construir Classic Stack e transporte

Versão: 0.7.1 · Data: 2026-11-03 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E02-03 / REQ-E02-03 |
| Fase / prioridade | M1 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | design — pessoa a designar |
| Esforço provisório | 3–8 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E02-02](TASK-E02-02.md) — Implementar tokens e componentes semânticos
- [TASK-E01-03](TASK-E01-03.md) — Implementar state machine e generation cancellation

## Entrega concreta

Painéis clássicos nas geometrias da spec e ligação de commands/events.

Referência visual: vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452 (rev.38), caminhos `mvp/...` e `out/...` conforme [referência do MVP](../../docs/design/06-mvp-reference.md). Caminhos relevantes: `out/svg/icons/` (icons.json, usage.json).

Documento canônico: [spec](../../docs/design/02-layouts.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Abrir/play/pause/stop/seek/volume/shuffle/repeat operam; resizing mínimo mantém targets e pipeline status.

## Verificação e evidência

QA-001, QA-010 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
