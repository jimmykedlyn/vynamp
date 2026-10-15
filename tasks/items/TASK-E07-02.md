# TASK-E07-02 — Definir pacote e SDK de plugins v1

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E07-02 / REQ-E07-02 |
| Fase / prioridade | M3 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | extensions — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E07-01](TASK-E07-01.md) — Provar sandbox WASM e capabilities
- [TASK-E06-02](TASK-E06-02.md) — Entregar instalação atômica de temas

## Entrega concreta

Manifest/schema, interfaces WIT, lifecycle e regras de compatibilidade major.

Documento canônico: [spec](../../docs/extensions/02-plugins.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Validar tipo, plataformas e permissões; major incompatível não carrega; assinatura não permite ignorar políticas de acesso.

## Verificação e evidência

QA-009 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
