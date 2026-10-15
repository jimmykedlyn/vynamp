# TASK-E08-07 — Entregar CLI e IPC de instância única

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E08-07 / REQ-E08-07 |
| Fase / prioridade | M3 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | library — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E01-03](TASK-E01-03.md) — Implementar state machine e generation cancellation
- [TASK-E08-06](TASK-E08-06.md) — Integrar controles do Windows

## Entrega concreta

Comandos open/enqueue/diagnostics/validate e named pipe com validação de peer.

Documento canônico: [spec](../../docs/library/02-tags-lyrics-tools.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Segundo launch não executa comando arbitrário; Unicode preservado; CLI retorna exit codes; IPC não expõe filesystem geral.

## Verificação e evidência

QA-009, QA-017 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
