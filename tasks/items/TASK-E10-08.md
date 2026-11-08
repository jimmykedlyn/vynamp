# TASK-E10-08 — Executar soak e matriz de hardware

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E10-08 / REQ-E10-08 |
| Fase / prioridade | M3 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | quality — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E04-10](TASK-E04-10.md) — Validar qualidade numérica e hardware matrix
- [TASK-E01-10](TASK-E01-10.md) — Endurecer engine e documentação dos limites
- [TASK-E10-07](TASK-E10-07.md) — Instrumentar e medir performance
- [TASK-E14-04](TASK-E14-04.md) — Executar corpus e outputs por alvo

## Entrega concreta

8 h em 24/192, DAC unplug, taxa, busy/sleep e stress de CPU.

Documento canônico: [spec](../../quality/01-estrategia-testes.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Zero underruns causados pelo app no ambiente de referência; pausar ao unplug; captura transparente real; limites declarados.

## Verificação e evidência

QA-003, QA-004, QA-011 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
