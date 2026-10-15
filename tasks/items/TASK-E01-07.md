# TASK-E01-07 — Adicionar codecs modernos e capítulos

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E01-07 / REQ-E01-07 |
| Fase / prioridade | M2 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | audio — pessoa a designar |
| Esforço provisório | 5–12 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E01-04](TASK-E01-04.md) — Entregar fonte local e decoder registry

## Entrega concreta

Adapters AAC-LC/HE-AAC/ALAC/Vorbis/Opus e M4B conforme prova/licença.

Documento canônico: [spec](../../docs/audio/01-engine.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Fixture por perfil suporta duration/seek/metadata quando aplicável; HE-AAC não é anunciado por decode AAC-LC.

## Verificação e evidência

QA-015 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
