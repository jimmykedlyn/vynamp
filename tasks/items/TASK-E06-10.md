# TASK-E06-10 — Validar fidelidade visual dos temas

Versão: 0.7.1 · Data: 2026-11-03 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E06-10 / REQ-E06-10 |
| Fase / prioridade | M3 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | extensions — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E06-08](TASK-E06-08.md) — Entregar galeria local e recovery visual
- [TASK-E02-09](TASK-E02-09.md) — Completar localização e testes visuais

Referência visual: vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452 (rev.38), caminhos `mvp/...` e `out/...` conforme [referência do MVP](../../docs/design/06-mvp-reference.md). Caminhos relevantes: `out/themes/themes.json`, `out/svg/rendered/` e `out/check/report.json`.

## Entrega concreta

Relatório de cores, medidas, estados e telas comparado à referência própria.

Documento canônico: [spec](../../docs/extensions/01-themes.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Conferir todos os modos/DPI e hierarchy clássica; falhas de acessibilidade não são ocultadas por aprovação de screenshots.

## Verificação e evidência

QA-010 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
