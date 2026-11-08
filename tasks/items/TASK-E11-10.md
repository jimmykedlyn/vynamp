# TASK-E11-10 — Preparar operação da plataforma

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E11-10 / REQ-E11-10 |
| Fase / prioridade | M4 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | delivery — pessoa a designar |
| Esforço provisório | 8–20 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E09-01](TASK-E09-01.md) — Definir catálogo, direitos, escala e custos

## Entrega concreta

IaC, secrets, DB/PITR, métricas de CDN/custo e runbooks de restore/rollback.

Documento canônico: [spec](../../delivery/02-release-devops.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Drill avalia RPO 15 min/RTO 4 h; métricas sem PII; rollback compatível; responsável operacional designado.

## Verificação e evidência

QA-014 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
