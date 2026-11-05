# TASK-E18-01 — Entregar Vynamp Restore como ferramenta desktop de restauração

Versão: 0.8.0 · Data: 2026-11-05 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E18-01 / REQ-E18-01 |
| Fase / prioridade | M3 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | library — pessoa a designar |
| Esforço provisório | 10–25 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E08-01](TASK-E08-01.md) — Implementar jobs persistidos
- [TASK-E01-07](TASK-E01-07.md) — Adicionar codecs modernos e capítulos
- [TASK-E16-03](TASK-E16-03.md) — Registrar menu de contexto no Windows
- [TASK-E16-04](TASK-E16-04.md) — Integrar ações de arquivo no Linux
- [TASK-E16-05](TASK-E16-05.md) — Publicar Quick Actions no macOS

## Entrega concreta

Ferramenta desktop `vynamp-restore` com motor de restauração próprio, diagnóstico de defeitos e exportação preservando metadados.

Documento canônico: [spec](../../docs/tools/01-vynamp-restore.md). [ADR](../../docs/architecture/adr/ADR-013-vynamp-restore.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

A fonte nunca é sobrescrita sem confirmação; escrita atômica com validação pós-escrita preserva origem e metadados; defeitos reportados têm severidade e evidência; correção nunca é aplicada sem previsualização; saída registra versão do motor e parâmetros usados.

## Verificação e evidência

QA-015, QA-016, QA-023, QA-024, QA-031 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md); fixtures por defeito em [matriz DSP](../../docs/quality/05-dsp-test-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
