# TASK-E09-02 — Materializar OpenAPI v1 e contract tests

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E09-02 / REQ-E09-02 |
| Fase / prioridade | M4 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | streaming — pessoa a designar |
| Esforço provisório | 8–20 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E09-01](TASK-E09-01.md) — Definir catálogo, direitos, escala e custos
- [TASK-E05-01](TASK-E05-01.md) — Implementar SourceDescriptor e provider simulado

## Entrega concreta

OpenAPI de catálogo/resolve/sync/stations, erros e testkit do provider.

Documento canônico: [spec](../../streaming/03-plataforma.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Exemplos validam; mock e real compartilham contrato; cursor/idempotência/escopos e enums desconhecidos são definidos.

## Verificação e evidência

QA-013 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
