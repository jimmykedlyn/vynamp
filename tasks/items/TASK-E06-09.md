# TASK-E06-09 — Preparar publicação de temas

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E06-09 / REQ-E06-09 |
| Fase / prioridade | M4 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | extensions — pessoa a designar |
| Esforço provisório | 8–20 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E06-08](TASK-E06-08.md) — Entregar galeria local e recovery visual
- [TASK-E09-09](TASK-E09-09.md) — Entregar registry de temas e plugins

## Entrega concreta

Metadados assinados, autoria, licença e imagens de preview para registry.

Documento canônico: [spec](../../extensions/01-temas.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Versão publicada é imutável; revogação e rollback funcionam; instalação manual continua disponível sem conta ou rede.

## Verificação e evidência

QA-009, QA-013 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
