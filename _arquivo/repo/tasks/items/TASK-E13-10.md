# TASK-E13-10 — Fechar aceite Android 1.0 em hardware

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E13-10 / REQ-E13-10 |
| Fase / prioridade | M3 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | qa-android — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E13-09](TASK-E13-09.md) — Entregar tablet/foldable e distribuição Android
- [TASK-E04-10](TASK-E04-10.md) — Validar qualidade numérica e hardware matrix
- [TASK-E05-09](TASK-E05-09.md) — Endurecer rede e diagnóstico de streaming

## Entrega concreta

Relatório de formatos comuns/DSP/HLS, lifecycle/focus/storage/TalkBack e energia.

Documento canônico: [spec](../../quality/03-matriz-multiplataforma.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Gates comuns e Android aprovados no aparelho real; saída hi-res apenas quando negociada; flag AAudio não é prova de bit-perfect.

## Verificação e evidência

QA-019, QA-020, QA-021, QA-003 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
