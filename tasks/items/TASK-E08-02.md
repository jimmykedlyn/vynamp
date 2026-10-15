# TASK-E08-02 — Entregar conversor e encoder registry

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E08-02 / REQ-E08-02 |
| Fase / prioridade | M3 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | library — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E08-01](TASK-E08-01.md) — Implementar jobs persistidos
- [TASK-E01-07](TASK-E01-07.md) — Adicionar codecs modernos e capítulos
- [TASK-E11-03](TASK-E11-03.md) — Resolver licenças e inventário de dependências

## Entrega concreta

Conversão batch para FLAC/WAV/Opus/Vorbis e MP3 quando permitido, com preview.

Documento canônico: [spec](../../docs/library/02-tags-lyrics-tools.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Validar output após escrita; cancelamento/disco cheio preservam source; conversão lossy não é anunciada como restauração hi-res.

## Verificação e evidência

QA-015, QA-016 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
