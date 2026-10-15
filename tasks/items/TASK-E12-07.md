# TASK-E12-07 — Entregar VST3 e investigar DSP de terceiros

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E12-07 / REQ-E12-07 |
| Fase / prioridade | M5 / P2 |
| Estado | Planejada; não implementada |
| Responsável funcional | expansion — pessoa a designar |
| Esforço provisório | 10–25 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E07-10](TASK-E07-10.md) — Auditar isolamento das extensões
- [TASK-E04-10](TASK-E04-10.md) — Validar qualidade numérica e hardware matrix

## Entrega concreta

Scanner/worker VST3 isolado, compensação de latência e spike WASM DSP separado.

Documento canônico: [spec](../../docs/delivery/01-roadmap.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Plugin travado bloqueado; crash não derruba player; não executar WASM na callback sem prova de deadline; revisar SDK license.

## Verificação e evidência

QA-005, QA-009 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
