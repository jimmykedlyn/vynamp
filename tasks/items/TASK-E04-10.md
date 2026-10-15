# TASK-E04-10 — Validar qualidade numérica e hardware matrix

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E04-10 / REQ-E04-10 |
| Fase / prioridade | M3 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | audio — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E04-06](TASK-E04-06.md) — Entregar modo transparente e diagnósticos hi-res
- [TASK-E04-07](TASK-E04-07.md) — Entregar dynamics, tone e channel controls
- [TASK-E04-08](TASK-E04-08.md) — Entregar tempo, pitch e efeitos criativos
- [TASK-E04-09](TASK-E04-09.md) — Entregar perfis e headphone preset import

## Entrega concreta

Relatório DSP/output com fixtures, driver versions e combinations.

Documento canônico: [spec](../../docs/audio/04-outputs-hires.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Testes100/192k/rate switch/channel mismatches; callback budget e8hsoak; claims limitados a combinations comprovadas.

## Verificação e evidência

QA-003, QA-004, QA-005, QA-011 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
