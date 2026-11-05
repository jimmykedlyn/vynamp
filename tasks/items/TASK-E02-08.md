# TASK-E02-08 — Entregar docking e persistência multi-monitor

Versão: 0.7.1 · Data: 2026-11-03 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E02-08 / REQ-E02-08 |
| Fase / prioridade | M3 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | design — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E02-06](TASK-E02-06.md) — Entregar Mini, preferências e estados de erro

## Entrega concreta

Painéis destacáveis, snap12DIP, saved geometry e retorno ao monitor disponível.

Referência visual: vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452 (rev.38), caminhos `mvp/...` e `out/...` conforme [referência do MVP](../../docs/design/06-mvp-reference.md).

Documento canônico: [spec](../../docs/design/02-layouts.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Monitor removido não deixa janela inacessível; Alt ignora snap; resize/dock não trava reprodução.

## Verificação e evidência

QA-010, QA-017 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
