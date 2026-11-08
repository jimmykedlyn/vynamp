# TASK-E13-07 — Entregar recovery de processo e controle de energia

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E13-07 / REQ-E13-07 |
| Fase / prioridade | M2 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | android — pessoa a designar |
| Esforço provisório | 5–12 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E13-03](TASK-E13-03.md) — Entregar audio focus e rotas Android
- [TASK-E03-02](TASK-E03-02.md) — Implementar queue independente e shuffle

## Entrega concreta

Checkpoint, reconexão ao serviço, WorkManager, quotas de cache e wake locks limitados.

Documento canônico: [spec](../../platforms/02-android.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Process death permite restore sem autostart proibido; rotation mantém sessão; screen-off 2 h sem underrun causado pelo app; wake locks liberados e visualizer desativado no background.

## Verificação e evidência

QA-019, QA-020 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
