# TASK-E14-03 — Entregar paridade de presets e schema de temas

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E14-03 / REQ-E14-03 |
| Fase / prioridade | M2 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | platform — pessoa a designar |
| Esforço provisório | 5–12 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E14-02](TASK-E14-02.md) — Entregar ports de arquivos, credenciais e sistema
- [TASK-E06-01](TASK-E06-01.md) — Definir schema e validador de temas

## Entrega concreta

Registry de capabilities por OS e perfis desktop/phone/tablet de tema e preset.

Documento canônico: [spec](../../docs/platforms/01-support-matrix.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Tema carrega com fallback em todos; registry mostra formatos reais; preset compartilhado tem resposta numérica equivalente; ações não aplicáveis explícitas.

## Verificação e evidência

QA-015, QA-018, QA-020 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
