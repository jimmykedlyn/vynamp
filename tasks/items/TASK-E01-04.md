# TASK-E01-04 — Entregar fonte local e decoder registry

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E01-04 / REQ-E01-04 |
| Fase / prioridade | M1 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | audio — pessoa a designar |
| Esforço provisório | 3–8 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E01-03](TASK-E01-03.md) — Implementar state machine e generation cancellation

## Entrega concreta

Adapters local source/probe/decode para WAV/AIFF/FLAC/MP3.

Documento canônico: [spec](../../reference/audio/01-engine.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Extensão incorreta permite sniff; corrupt/oversized falham com erro tipado; decoder lista perfis reais.

## Verificação e evidência

QA-001, QA-015 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
