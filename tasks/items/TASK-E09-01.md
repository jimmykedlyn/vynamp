# TASK-E09-01 — Definir catálogo, direitos, escala e custos

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E09-01 / REQ-E09-01 |
| Fase / prioridade | M4 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | streaming — pessoa a designar |
| Esforço provisório | 8–20 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E05-09](TASK-E05-09.md) — Endurecer rede e diagnóstico de streaming
- [TASK-E11-09](TASK-E11-09.md) — Preparar stable 1.0 e rollout beta

## Entrega concreta

RFC de plataforma com regiões, rights, renditions, limites e orçamento de bandwidth.

Documento canônico: [spec](../../streaming/03-plataforma.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Fixtures autorizadas; roles/territórios/janela de disponibilidade definidos; custo por ouvinte-hora estimado com dados.

## Verificação e evidência

QA-013, QA-014 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
