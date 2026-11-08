# TASK-E07-03 — Implementar broker de permissões

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E07-03 / REQ-E07-03 |
| Fase / prioridade | M3 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | extensions — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E07-02](TASK-E07-02.md) — Definir pacote e SDK de plugins v1
- [TASK-E05-02](TASK-E05-02.md) — Entregar HTTP Range e reprodução progressiva

## Entrega concreta

Acesso mediado a rede, roots selecionadas e biblioteca, com grants persistidos.

Documento canônico: [spec](../../extensions/02-plugins.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Negar e revogar bloqueia requests/handles; redirect revalida domínio; plugin não recebe shell, env vars ou filesystem completo.

## Verificação e evidência

QA-009 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
