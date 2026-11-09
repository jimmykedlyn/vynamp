# TASK-E13-08 — Entregar temas e plugins mobile

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E13-08 / REQ-E13-08 |
| Fase / prioridade | M3 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | android-extensions — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E13-05](TASK-E13-05.md) — Entregar interface phone e navegação acessível
- [TASK-E06-04](TASK-E06-04.md) — Produzir três temas originais
- [TASK-E07-08](TASK-E07-08.md) — Entregar documentação e harness do SDK

## Entrega concreta

Perfis phone/tablet, manager mobile e runtime de extensões validado para Android.

Documento canônico: [spec](../../platforms/02-android.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Fallback acessível; budgets e permissões funcionam; nenhum helper nativo externo; runtime e distribuição compatíveis documentados, sem promessa de JIT não provado.

## Verificação e evidência

QA-009, QA-020, QA-021 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
