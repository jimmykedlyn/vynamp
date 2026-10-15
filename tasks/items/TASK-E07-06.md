# TASK-E07-06 — Entregar templates e plugins de exemplo

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E07-06 / REQ-E07-06 |
| Fase / prioridade | M3 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | extensions — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E07-04](TASK-E07-04.md) — Implementar lifecycle e circuit breaker

## Entrega concreta

Templates Rust para WASM: letras locais, espectro e provider simulado.

Documento canônico: [spec](../../docs/extensions/02-plugins.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Exemplos funcionam no harness offline com permissões mínimas; documentação de lifecycle acompanha a versão do SDK.

## Verificação e evidência

QA-009, QA-017 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
