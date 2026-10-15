# TASK-E05-07 — Entregar HLS live, ABR e timeshift

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E05-07 / REQ-E05-07 |
| Fase / prioridade | M3 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | streaming — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E05-06](TASK-E05-06.md) — Entregar HLS sob demanda

## Entrega concreta

Janela live, escolha de qualidade por throughput e buffer DVR limitado.

Documento canônico: [spec](../../docs/streaming/01-client.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Mudança de sequência não duplica áudio; seleção manual de qualidade é respeitada; seek só na janela disponível; voltar ao live edge funciona.

## Verificação e evidência

QA-008 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
