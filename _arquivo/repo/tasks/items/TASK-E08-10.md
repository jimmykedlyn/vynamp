# TASK-E08-10 — Entregar controle remoto autenticado

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E08-10 / REQ-E08-10 |
| Fase / prioridade | M4 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | library — pessoa a designar |
| Esforço provisório | 8–20 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E07-03](TASK-E07-03.md) — Implementar broker de permissões
- [TASK-E08-07](TASK-E08-07.md) — Entregar CLI e IPC de instância única
- [TASK-E09-03](TASK-E09-03.md) — Implementar identidade e autorização

## Entrega concreta

API local desligada por padrão, pairing e opção LAN com TLS/rate limits.

Documento canônico: [spec](../../library/02-tags-letras.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Cliente sem pairing não controla; nenhum endpoint de shell/filesystem; rotação e revogação de token funcionam.

## Verificação e evidência

QA-009, QA-017 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
