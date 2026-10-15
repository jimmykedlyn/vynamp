# TASK-E10-09 — Validar upgrades, recovery e privacidade

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E10-09 / REQ-E10-09 |
| Fase / prioridade | M3 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | quality — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E03-10](TASK-E03-10.md) — Completar recovery, export e large-library gate
- [TASK-E11-06](TASK-E11-06.md) — Implementar updater assinado
- [TASK-E10-05](TASK-E10-05.md) — Fuzz parsers, pacotes e RPC

## Entrega concreta

Drills de kill/migration/update rollback e testes de vazamento em export/log.

Documento canônico: [spec](../../docs/quality/01-test-strategy.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Dados irreproduzíveis preservados; update adulterado/replay rejeitado; token de teste não aparece em logs/diagnósticos.

## Verificação e evidência

QA-007, QA-009, QA-012 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
