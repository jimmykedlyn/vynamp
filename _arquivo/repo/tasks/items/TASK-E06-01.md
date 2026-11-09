# TASK-E06-01 — Definir schema e validador de temas

Versão: 0.7.1 · Data: 2026-11-03 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E06-01 / REQ-E06-01 |
| Fase / prioridade | M2 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | extensions — pessoa a designar |
| Esforço provisório | 5–12 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E02-02](TASK-E02-02.md) — Implementar tokens e componentes semânticos

Referência visual: vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452 (rev.38), caminhos `mvp/...` e `out/...` conforme [referência do MVP](../../design/07-pin-mvp.md). Caminhos relevantes: `out/themes/themes.json`.

## Entrega concreta

Schemas de manifest/tokens/layout, parser e erros por JSON Pointer.

Documento canônico: [spec](../../extensions/01-temas.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Rejeitar schema major desconhecido; avisar campos opcionais desconhecidos; detectar ciclos e ações essenciais ausentes.

## Verificação e evidência

QA-009, QA-010 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.

## Identidade aprovada Vynamp C

Preservar tokens semânticos de temas e manter marca Vynamp C como identidade do app, separada de skins. Extensão própria .vyntheme; schema e exemplos usam namespace vynamp. [Guia canônico](../../design/04-marca.md). Este requisito complementar não altera IDs/estado nem dispensa o contrato acima.
