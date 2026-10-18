# TASK-E11-01 — Criar workspace e baseline de engenharia

Versão: 0.7.0 · Atualização: 2026-10-18 · Estado: em execução; baseline de tooling existente, revisão/CI/integração da entrega E11-01 pendentes.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E11-01 / REQ-E11-01 |
| Fase / prioridade | M0 / P0 |
| Estado | Em execução; sem aceite antecipado |
| Responsável funcional | delivery — pessoa a designar |
| Esforço provisório | 2–5 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E15-02](TASK-E15-02.md) — Validar worktrees, claims e handoff

## Entrega concreta

Repositório independente, Cargo workspace mínimo, toolchain pin, ADRs e CI inicial.

Documento canônico: [spec](../../docs/delivery/02-release-devops.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Nenhum código/asset Winamp; build real com lockfile resolvido; validador de docs passa; fixtures têm paths definidos.

## Verificação e evidência

QA-012 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.

## Execução E11-01

[Plano](../../docs/superpowers/plans/2026-10-18-e11-01-workspace.md), [ADR-009](../../docs/architecture/adr/ADR-009-engineering-baseline.md), [paths de fixtures](../../tests/fixtures/README.md) e [evidência do autor](../../docs/evidence/PR/task-e11-01/README.md). Reutiliza workspace/pin/lockfile/CI já integrados; não adiciona app, corpus nem testes de áudio. QA-012 nesta entrega limita-se aos guards/recovery do tooling existente; updater/migrations de produto permanecem pendentes. Revisão Claude do SHA exato, CI remoto e merge pelo mantenedor são gates separados.
