# TASK-E14-04 — Executar corpus e outputs por alvo

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E14-04 / REQ-E14-04 |
| Fase / prioridade | M3 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | qa-platform — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E12-01](TASK-E12-01.md) — Entregar Linux nativo e MPRIS na 1.0
- [TASK-E12-02](TASK-E12-02.md) — Entregar macOS Intel e Apple Silicon nativos na 1.0
- [TASK-E13-07](TASK-E13-07.md) — Entregar recovery de processo e controle de energia
- [TASK-E04-06](TASK-E04-06.md) — Entregar modo transparente e diagnósticos hi-res

## Entrega concreta

Fixtures e hardware por OS/ABI, com clock/seek/drain/gapless dos backends.

Documento canônico: [spec](../../reference/audio/05-platform-backends.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Mac Intel e arm64 testados separadamente; Linux nativo e Android real; nenhum fim cortado/buffer antigo; exclusive não presumido.

## Verificação e evidência

QA-003, QA-004, QA-018, QA-019 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
