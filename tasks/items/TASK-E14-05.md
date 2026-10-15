# TASK-E14-05 — Validar packaging e update em cada alvo

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E14-05 / REQ-E14-05 |
| Fase / prioridade | M3 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | release-platform — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E11-07](TASK-E11-07.md) — Entregar distribuição nativa por plataforma
- [TASK-E12-01](TASK-E12-01.md) — Entregar Linux nativo e MPRIS na 1.0
- [TASK-E12-02](TASK-E12-02.md) — Entregar macOS Intel e Apple Silicon nativos na 1.0
- [TASK-E13-09](TASK-E13-09.md) — Entregar tablet/foldable e distribuição Android

## Entrega concreta

Installer/packages/Universal 2/APK-AAB, migrations, dependencies nativas e SBOM.

Documento canônico: [spec](../../docs/platforms/01-support-matrix.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Artifacts instalam/assinam/atualizam; permissões preservadas; portais Linux, notarização macOS e native pages Android conferidos no release.

## Verificação e evidência

QA-012, QA-021 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
