# TASK-E14-01 — Provar os cinco targets e features portáveis

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E14-01 / REQ-E14-01 |
| Fase / prioridade | M0 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | platform — pessoa a designar |
| Esforço provisório | 2–5 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E11-01](TASK-E11-01.md) — Criar workspace e baseline de engenharia

## Entrega concreta

Matriz de toolchains/ABIs Windows/Linux/macOS Intel/macOS Silicon/Android e dependencies.

Documento canônico: [spec](../../platforms/01-matriz-suporte.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Core e codecs nativos resolvem em cada ABI; outputs/files/keychain provados; baselines e licenças suportam todos os alvos antes freeze.

## Verificação e evidência

QA-018, QA-021 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
