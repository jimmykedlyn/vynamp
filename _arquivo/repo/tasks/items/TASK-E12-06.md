# TASK-E12-06 — Entregar CD playback e ripping

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E12-06 / REQ-E12-06 |
| Fase / prioridade | M5 / P2 |
| Estado | Planejada; não implementada |
| Responsável funcional | expansion — pessoa a designar |
| Esforço provisório | 10–25 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E08-02](TASK-E08-02.md) — Entregar conversor e encoder registry
- [TASK-E03-08](TASK-E03-08.md) — Implementar escrita de tags e capítulos em lote

## Entrega concreta

Adapter de drive, secure mode/offset, metadados e relatório de checksums.

Documento canônico: [spec](../../delivery/01-roadmap.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Erros de leitura identificados; cancelamento seguro; rip verificado apenas com checksum real; licenças de providers atendidas.

## Verificação e evidência

QA-015, QA-016 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
