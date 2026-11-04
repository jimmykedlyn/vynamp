# TASK-E10-06 — Automatizar semântica e regressão visual

Versão: 0.7.1 · Data: 2026-11-03 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E10-06 / REQ-E10-06 |
| Fase / prioridade | M2 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | quality — pessoa a designar |
| Esforço provisório | 5–12 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E02-06](TASK-E02-06.md) — Entregar Mini, preferências e estados de erro
- [TASK-E06-04](TASK-E06-04.md) — Produzir três temas originais

Referência visual: vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452 (rev.38), caminhos `mvp/...` e `out/...` conforme [referência do MVP](../../docs/design/06-mvp-reference.md). Caminhos relevantes: `out/svg/rendered/` e `out/check/report.json`.

## Entrega concreta

Workflows de teclado, screenshots por DPI e checklist de leitores de tela.

Documento canônico: [spec](../../docs/quality/01-test-strategy.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Nenhum controle crítico ausente; validar DPI, PT-BR/EN e reduced motion; evidência manual de screen reader registrada.

## Verificação e evidência

QA-010 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
