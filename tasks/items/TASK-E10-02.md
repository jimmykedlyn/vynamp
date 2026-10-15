# TASK-E10-02 — Testar domínio e propriedades de fila

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E10-02 / REQ-E10-02 |
| Fase / prioridade | M1 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | quality — pessoa a designar |
| Esforço provisório | 3–8 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E01-03](TASK-E01-03.md) — Implementar state machine e generation cancellation
- [TASK-E03-02](TASK-E03-02.md) — Implementar queue independente e shuffle
- [TASK-E10-01](TASK-E10-01.md) — Criar corpus de testes licenciado

## Entrega concreta

Suite de state machine, comandos aleatórios, ordering e queue invariants.

Documento canônico: [spec](../../docs/quality/01-test-strategy.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Sequências aleatórias preservam generation e invariantes; shuffle/history/repeat/stop-after são exercitados.

## Verificação e evidência

QA-001, QA-006 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
