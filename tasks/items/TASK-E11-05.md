# TASK-E11-05 — Implementar config, logs e keychain

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E11-05 / REQ-E11-05 |
| Fase / prioridade | M2 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | delivery — pessoa a designar |
| Esforço provisório | 5–12 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E11-02](TASK-E11-02.md) — Definir fronteiras de segurança
- [TASK-E03-01](TASK-E03-01.md) — Implementar schema SQLite e migrations seguras

## Entrega concreta

Schema/migrations de configuração, escrita atômica, redaction e retenção.

Documento canônico: [spec](../../docs/delivery/02-release-devops.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Fixtures de token/URL/path redacted; portable não grava secrets plaintext; crash permite recovery; telemetria desligada por padrão.

## Verificação e evidência

QA-009, QA-012 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
