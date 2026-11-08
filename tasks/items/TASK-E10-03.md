# TASK-E10-03 — Criar harness numérico de áudio

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E10-03 / REQ-E10-03 |
| Fase / prioridade | M2 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | quality — pessoa a designar |
| Esforço provisório | 5–12 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E04-03](TASK-E04-03.md) — Construir grafo DSP e transições sem glitches
- [TASK-E10-01](TASK-E10-01.md) — Criar corpus de testes licenciado

## Entrega concreta

Oráculos PCM, resposta FFT, delay/padding, crossfade e sample conversion.

Documento canônico: [spec](../../quality/01-estrategia-testes.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Detectar frame extra, trim incorreto, mapa de canais errado e NaN; referência independente da implementação sob teste.

## Verificação e evidência

QA-002, QA-004, QA-005 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
