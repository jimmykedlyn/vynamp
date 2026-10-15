# TASK-E01-10 — Endurecer engine e documentação dos limites

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E01-10 / REQ-E01-10 |
| Fase / prioridade | M3 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | audio — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E01-06](TASK-E01-06.md) — Implementar gapless sample-accurate
- [TASK-E01-09](TASK-E01-09.md) — Entregar channel mapping e DSD para PCM
- [TASK-E04-06](TASK-E04-06.md) — Entregar modo transparente e diagnósticos hi-res

## Entrega concreta

Soak/recovery de engine e relatório de suporte por perfil.

Documento canônico: [spec](../../docs/audio/01-engine.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

8h24/192 controladas sem underrun causado pelo app; decoder hang pode ser cancelado; limitações publicadas.

## Verificação e evidência

QA-001, QA-002, QA-011 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
