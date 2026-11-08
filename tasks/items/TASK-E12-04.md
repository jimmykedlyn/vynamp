# TASK-E12-04 — Entregar DSD nativo/DoP e PCM 768 kHz

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E12-04 / REQ-E12-04 |
| Fase / prioridade | M5 / P2 |
| Estado | Planejada; não implementada |
| Responsável funcional | expansion — pessoa a designar |
| Esforço provisório | 10–25 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E04-10](TASK-E04-10.md) — Validar qualidade numérica e hardware matrix
- [TASK-E01-09](TASK-E01-09.md) — Entregar channel mapping e DSD para PCM

## Entrega concreta

Backends condicionados ao DAC, packing DoP e capabilities de taxa/DSD.

Documento canônico: [spec](../../delivery/01-roadmap.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

DSP desligado em DSD nativo; markers/bytes validados; equipamento sem suporte falha sem conversão oculta.

## Verificação e evidência

QA-003, QA-004, QA-015 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
