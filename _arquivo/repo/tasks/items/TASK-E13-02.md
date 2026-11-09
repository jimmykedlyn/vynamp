# TASK-E13-02 — Entregar serviço e MediaSession com engine Rust

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E13-02 / REQ-E13-02 |
| Fase / prioridade | M1 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | android — pessoa a designar |
| Esforço provisório | 3–8 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E13-01](TASK-E13-01.md) — Provar Android arm64, bridge Rust e output
- [TASK-E01-03](TASK-E01-03.md) — Implementar state machine e generation cancellation

## Entrega concreta

Serviço foreground proprietário da reprodução, Player adapter, notificação/controles e bridge lifetime.

Documento canônico: [spec](../../platforms/02-android.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Activity não é owner do áudio; lockscreen/play/pause usam fila única Rust; cleanup idempotente; serviço segue restrições do SDK.

## Verificação e evidência

QA-019 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
