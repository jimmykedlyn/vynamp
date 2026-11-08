# TASK-E09-10 — Validar operação, carga e recuperação

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E09-10 / REQ-E09-10 |
| Fase / prioridade | M4 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | streaming — pessoa a designar |
| Esforço provisório | 8–20 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E09-06](TASK-E09-06.md) — Entregar streaming VOD e CDN
- [TASK-E09-07](TASK-E09-07.md) — Entregar ingest live e programação de rádio
- [TASK-E09-08](TASK-E09-08.md) — Entregar sincronização de dados do ouvinte
- [TASK-E09-09](TASK-E09-09.md) — Entregar registry de temas e plugins
- [TASK-E11-10](TASK-E11-10.md) — Preparar operação da plataforma

## Entrega concreta

Teste de 1.000 sessões, custos, restore drill, alertas e runbooks.

Documento canônico: [spec](../../streaming/03-plataforma.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

API p95 ≤250 ms no ambiente de referência; CDN medida separadamente; demonstrar RPO/RTO; rollback mantém contratos.

## Verificação e evidência

QA-014 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
