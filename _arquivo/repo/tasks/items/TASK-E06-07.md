# TASK-E06-07 — Entregar editor e preview de temas

Versão: 0.7.1 · Data: 2026-11-03 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E06-07 / REQ-E06-07 |
| Fase / prioridade | M3 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | extensions — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E06-04](TASK-E06-04.md) — Produzir três temas originais
- [TASK-E06-06](TASK-E06-06.md) — Entregar visualizações avançadas

Referência visual: vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452 (rev.38), caminhos `mvp/...` e `out/...` conforme [referência do MVP](../../design/07-pin-mvp.md). Caminhos relevantes: `out/themes/themes.json`.

## Entrega concreta

Editor de tokens/layout, undo, preview por DPI/estado e exportação.

Documento canônico: [spec](../../extensions/01-temas.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Exportar e importar preserva configuração; avisar problemas de contraste/constraints; preview não altera tema ativo nem áudio.

## Verificação e evidência

QA-009, QA-010 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
