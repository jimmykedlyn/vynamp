# TASK-E09-05 — Implementar ingest e transcode isolados

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E09-05 / REQ-E09-05 |
| Fase / prioridade | M4 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | streaming — pessoa a designar |
| Esforço provisório | 8–20 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E09-04](TASK-E09-04.md) — Implementar catálogo e administração editorial
- [TASK-E11-03](TASK-E11-03.md) — Resolver licenças e inventário de dependências

## Entrega concreta

Upload assinado, quarantine/probe/rights, outbox e workers de renditions.

Documento canônico: [spec](../../docs/streaming/03-platform.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Jobs duplicados idempotentes; limites de mídia e SSRF aplicados; master privado; ready somente após validações.

## Verificação e evidência

QA-014, QA-015 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
