# TASK-E09-03 — Implementar identidade e autorização

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E09-03 / REQ-E09-03 |
| Fase / prioridade | M4 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | streaming — pessoa a designar |
| Esforço provisório | 8–20 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E09-02](TASK-E09-02.md) — Materializar OpenAPI v1 e contract tests

## Entrega concreta

OIDC/PKCE, keychain do cliente, roles, isolamento por conta e revogação.

Documento canônico: [spec](../../docs/streaming/03-platform.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Negar IDOR/acesso entre contas; login pelo browser; não enviar bearer à CDN; logout limpa apenas dados privados.

## Verificação e evidência

QA-013 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
