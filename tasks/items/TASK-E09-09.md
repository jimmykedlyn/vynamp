# TASK-E09-09 — Entregar registry de temas e plugins

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E09-09 / REQ-E09-09 |
| Fase / prioridade | M4 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | streaming — pessoa a designar |
| Esforço provisório | 8–20 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E09-04](TASK-E09-04.md) — Implementar catálogo e administração editorial
- [TASK-E07-08](TASK-E07-08.md) — Entregar documentação e harness do SDK
- [TASK-E06-08](TASK-E06-08.md) — Entregar galeria local e recovery visual

## Entrega concreta

Admin de publisher, validação de pacotes, assinatura, versão imutável e revogação.

Documento canônico: [spec](../../streaming/03-plataforma.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Rejeitar pacotes adversariais; conferir hash por artifact; não expor chaves privadas; instalação manual continua sem conta.

## Verificação e evidência

QA-009, QA-013 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
