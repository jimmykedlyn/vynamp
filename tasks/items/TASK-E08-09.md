# TASK-E08-09 — Entregar audiobooks e progresso offline

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E08-09 / REQ-E08-09 |
| Fase / prioridade | M4 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | library — pessoa a designar |
| Esforço provisório | 8–20 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E05-10](TASK-E05-10.md) — Integrar provider API, offline e direitos
- [TASK-E03-09](TASK-E03-09.md) — Entregar letras, bookmarks e histórico

## Entrega concreta

Agrupamento de livros, capítulos, bookmarks e adapter de sync de progresso.

Documento canônico: [spec](../../library/02-tags-letras.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Velocidade por categoria; retomada explícita; conflito de retrocesso não usa máximo sempre; sem rede continua reproduzindo.

## Verificação e evidência

QA-013, QA-016 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
