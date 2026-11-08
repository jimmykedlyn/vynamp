# TASK-E10-10 — Fechar gates e cobertura desktop

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E10-10 / REQ-E10-10 |
| Fase / prioridade | M3 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | quality — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E10-08](TASK-E10-08.md) — Executar soak e matriz de hardware
- [TASK-E10-09](TASK-E10-09.md) — Validar upgrades, recovery e privacidade
- [TASK-E02-10](TASK-E02-10.md) — Validar experiência desktop ponta a ponta
- [TASK-E07-10](TASK-E07-10.md) — Auditar isolamento das extensões
- [TASK-E14-06](TASK-E14-06.md) — Fechar release 1.0 multiplataforma

## Entrega concreta

Matriz de evidência REQ/TASK/QA e relatório de problemas conhecidos.

Documento canônico: [spec](../../quality/01-estrategia-testes.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Aceitar todos os requisitos M1–M3 ou registrar mudança explícita de escopo; zero P0/P1; ausência de hardware não vira teste aprovado.

## Verificação e evidência

QA-001, QA-002, QA-003, QA-004, QA-005, QA-006, QA-007, QA-008, QA-009, QA-010, QA-011, QA-012, QA-015, QA-016, QA-017 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
