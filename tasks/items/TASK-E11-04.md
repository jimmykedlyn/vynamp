# TASK-E11-04 — Completar CI e supply chain

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E11-04 / REQ-E11-04 |
| Fase / prioridade | M1 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | delivery — pessoa a designar |
| Esforço provisório | 3–8 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E11-01](TASK-E11-01.md) — Criar workspace e baseline de engenharia
- [TASK-E11-03](TASK-E11-03.md) — Resolver licenças e inventário de dependências
- [TASK-E14-01](TASK-E14-01.md) — Provar os cinco targets e features portáveis
- [TASK-E15-04](TASK-E15-04.md) — Ativar workflows e CI Gate multiplataforma

## Entrega concreta

CI Rust e hosts Windows x64/Linux x64/macOS x64+arm64/Android arm64, native dependencies, audits, SBOM e artifacts.

Documento canônico: [spec](../../docs/delivery/02-release-devops.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Todos os targets obrigatórios compilam e executam smoke apropriado; cross-build não substitui hardware; features/hashes/NDK/Gradle travados e secrets isolados.

## Verificação e evidência

QA-012, QA-015 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
