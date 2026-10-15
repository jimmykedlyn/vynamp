# TASK-E10-04 — Criar servidor de streaming adversarial

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E10-04 / REQ-E10-04 |
| Fase / prioridade | M2 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | quality — pessoa a designar |
| Esforço provisório | 5–12 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E05-03](TASK-E05-03.md) — Entregar rádio Icecast/SHOUTcast e ICY
- [TASK-E10-01](TASK-E10-01.md) — Criar corpus de testes licenciado

## Entrega concreta

Fixtures HTTP Range/ICY fragmentado/stall/redirect e scripts de timeline.

Documento canônico: [spec](../../docs/quality/01-test-strategy.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

CI cobre cancel/retry/resume/generation; 401/TLS não entram em retry cego; resultados são reproduzíveis.

## Verificação e evidência

QA-008 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
