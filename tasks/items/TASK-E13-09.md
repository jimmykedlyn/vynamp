# TASK-E13-09 — Entregar tablet/foldable e distribuição Android

Versão: 0.7.1 · Data: 2026-11-03 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E13-09 / REQ-E13-09 |
| Fase / prioridade | M3 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | android-release — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E13-06](TASK-E13-06.md) — Integrar streaming, podcasts e downloads Android
- [TASK-E13-07](TASK-E13-07.md) — Entregar recovery de processo e controle de energia
- [TASK-E13-08](TASK-E13-08.md) — Entregar temas e plugins mobile
- [TASK-E11-04](TASK-E11-04.md) — Completar CI e supply chain

Referência visual: vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452 (rev.38), caminhos `mvp/...` e `out/...` conforme [referência do MVP](../../docs/design/06-mvp-reference.md). Caminhos relevantes: `mvp/` (layouts) e `out/svg/rendered/`.

## Entrega concreta

UI adaptativa, APK/AAB assinado, ABIs/NDK/page size e requisitos de publicação.

Documento canônico: [spec](../../docs/design/04-mobile-tablet.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Phone/tablet/landscape/hinge/fonte passam; update preserva dados; target SDK e políticas revisados; aparelho arm64 real além do emulador.

## Verificação e evidência

QA-020, QA-021 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
