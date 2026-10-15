# TASK-E11-06 — Implementar updater assinado

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E11-06 / REQ-E11-06 |
| Fase / prioridade | M3 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | delivery — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E11-04](TASK-E11-04.md) — Completar CI e supply chain
- [TASK-E11-05](TASK-E11-05.md) — Implementar config, logs e keychain

## Entrega concreta

Verificação de metadados/artefatos, canais, expiry, rotação e rollback.

Documento canônico: [spec](../../docs/delivery/02-release-devops.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Rejeitar tamper/stale/replay; staging antes aplicar; não interromper playback/gravação automaticamente; DB preservado.

## Verificação e evidência

QA-012 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
