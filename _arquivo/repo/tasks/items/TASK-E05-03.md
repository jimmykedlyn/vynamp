# TASK-E05-03 — Entregar rádio Icecast/SHOUTcast e ICY

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E05-03 / REQ-E05-03 |
| Fase / prioridade | M2 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | streaming — pessoa a designar |
| Esforço provisório | 5–12 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E05-02](TASK-E05-02.md) — Entregar HTTP Range e reprodução progressiva
- [TASK-E02-06](TASK-E02-06.md) — Entregar Mini, preferências e estados de erro

## Entrega concreta

Parser ICY, metadados, estações favoritas, entrada de URL e indicador LIVE.

Documento canônico: [spec](../../streaming/01-cliente.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Separar metadados fragmentados dos bytes de áudio; limitar tags inválidas; reconexão limitada e cancelável; não inventar duração.

## Verificação e evidência

QA-008 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
