# TASK-E05-06 — Entregar HLS sob demanda

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E05-06 / REQ-E05-06 |
| Fase / prioridade | M3 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | streaming — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E05-04](TASK-E05-04.md) — Entregar buffers, cache e downloads
- [TASK-E01-07](TASK-E01-07.md) — Adicionar codecs modernos e capítulos

## Entrega concreta

Parser master/media, segmentos, discontinuity, byte ranges e AES-128 autorizado.

Documento canônico: [spec](../../streaming/01-cliente.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Limitar manifests inválidos/aninhados; tratar segmentos ausentes; seek preserva timeline; não registrar chaves ou URLs secretas.

## Verificação e evidência

QA-008 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
