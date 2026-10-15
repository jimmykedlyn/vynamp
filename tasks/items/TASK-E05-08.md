# TASK-E05-08 — Entregar gravação de rádios

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E05-08 / REQ-E05-08 |
| Fase / prioridade | M3 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | streaming — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E05-03](TASK-E05-03.md) — Entregar rádio Icecast/SHOUTcast e ICY
- [TASK-E08-02](TASK-E08-02.md) — Entregar conversor e encoder registry

## Entrega concreta

Gravação passthrough ou encoding escolhido, divisão de arquivos/CUE e monitor de espaço.

Documento canônico: [spec](../../docs/streaming/01-client.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Proibição explícita do provider bloqueia gravação; disco cheio encerra arquivo válido; título ICY não é tratado como fronteira exata de samples.

## Verificação e evidência

QA-016 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
