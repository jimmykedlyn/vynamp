# TASK-E10-01 — Criar corpus de testes licenciado

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E10-01 / REQ-E10-01 |
| Fase / prioridade | M0 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | quality — pessoa a designar |
| Esforço provisório | 2–5 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E11-01](TASK-E11-01.md) — Criar workspace e baseline de engenharia

## Entrega concreta

Fixtures geradas de tons/sweep/gapless e corrupções, com manifests de hashes e licenças.

Documento canônico: [spec](../../quality/01-estrategia-testes.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

CI valida hashes; oráculos de frames/tags/ranges definidos; testes não dependem de rádios públicas instáveis.

## Verificação e evidência

QA-001, QA-015 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
