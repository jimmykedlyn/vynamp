# TASK-E02-02 — Implementar tokens e componentes semânticos

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E02-02 / REQ-E02-02 |
| Fase / prioridade | M1 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | design — pessoa a designar |
| Esforço provisório | 3–8 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E02-01](TASK-E02-01.md) — Validar toolkit Rust com UI clássica acessível

## Entrega concreta

Theme token loader builtin, buttons/sliders/list rows com estados completos.

Documento canônico: [spec](../../docs/design/02-layouts.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Usar tokens canônicos; focus/hover/pressed/disabled diferem; contraste medido e nome acessível em cada controle.

## Verificação e evidência

QA-010 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.

## Identidade aprovada Vynamp C

Aplicar nome Vynamp e identidade C — Pulso em shells/headers/About, mantendo os tokens semânticos Classic Graphite. Não confundir marca com botão play. Wordmark/ícones grandes não substituem controles acessíveis. [Guia canônico](../../docs/design/04-brand-identity.md). Este requisito complementar não altera IDs/estado nem dispensa o contrato acima.
