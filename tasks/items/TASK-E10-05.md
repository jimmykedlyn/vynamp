# TASK-E10-05 — Fuzz parsers, pacotes e RPC

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E10-05 / REQ-E10-05 |
| Fase / prioridade | M3 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | quality — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E06-02](TASK-E06-02.md) — Entregar instalação atômica de temas
- [TASK-E07-03](TASK-E07-03.md) — Implementar broker de permissões
- [TASK-E10-01](TASK-E10-01.md) — Criar corpus de testes licenciado

## Entrega concreta

Fuzz targets, corpus adversarial, testes de limites e cobertura do threat model.

Documento canônico: [spec](../../docs/quality/01-test-strategy.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Mídia/imagem/HLS/ZIP/RPC inválidos não causam OOM/hang; permissões negativas e URLs filhas cobertas.

## Verificação e evidência

QA-009, QA-015 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
