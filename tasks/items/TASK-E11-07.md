# TASK-E11-07 — Entregar distribuição nativa por plataforma

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E11-07 / REQ-E11-07 |
| Fase / prioridade | M3 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | delivery — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E11-04](TASK-E11-04.md) — Completar CI e supply chain
- [TASK-E07-05](TASK-E07-05.md) — Implementar helper nativo isolado
- [TASK-E11-06](TASK-E11-06.md) — Implementar updater assinado

## Entrega concreta

Windowsinstaller/ZIP, Linuxpackages, MacUniversal2 ou packages x64/arm64 e AndroidAPK/AAB, com signing/symbols.

Documento canônico: [spec](../../docs/delivery/02-release-devops.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Instalar/update/remove em cada alvo; preservar dados/grants; Mac slices nativas/notarização e Androidnative/targetSDK/page size checks; interfaces de distribuição específicas do host.

## Verificação e evidência

QA-012, QA-017 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
