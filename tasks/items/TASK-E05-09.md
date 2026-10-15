# TASK-E05-09 — Endurecer rede e diagnóstico de streaming

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E05-09 / REQ-E05-09 |
| Fase / prioridade | M3 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | streaming — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E05-07](TASK-E05-07.md) — Entregar HLS live, ABR e timeshift
- [TASK-E05-08](TASK-E05-08.md) — Entregar gravação de rádios

## Entrega concreta

Política para cada URL filha, corpus de falhas e painel de saúde sanitizado.

Documento canônico: [spec](../../docs/streaming/01-client.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Revalidar SSRF/DNS/redirects conforme origem; esconder segredos; reconexão, cancelamento e stall não travam a interface.

## Verificação e evidência

QA-008, QA-009 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
