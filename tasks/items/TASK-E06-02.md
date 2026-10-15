# TASK-E06-02 — Entregar instalação atômica de temas

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E06-02 / REQ-E06-02 |
| Fase / prioridade | M2 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | extensions — pessoa a designar |
| Esforço provisório | 5–12 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E06-01](TASK-E06-01.md) — Definir schema e validador de temas
- [TASK-E11-02](TASK-E11-02.md) — Definir fronteiras de segurança

## Entrega concreta

Validador de ZIP, staging, limites, checksums, instalação por versão e rollback.

Documento canônico: [spec](../../docs/extensions/01-themes.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Rejeitar traversal, paths absolutos, symlinks e zip bombs; update inválido mantém tema atual; limite de 20 MiB extraídos.

## Verificação e evidência

QA-009 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
