# TASK-E12-08 — Entregar DASH, WebDAV e clouds

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E12-08 / REQ-E12-08 |
| Fase / prioridade | M5 / P2 |
| Estado | Planejada; não implementada |
| Responsável funcional | expansion — pessoa a designar |
| Esforço provisório | 10–25 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E05-10](TASK-E05-10.md) — Integrar provider API, offline e direitos
- [TASK-E09-10](TASK-E09-10.md) — Validar operação, carga e recuperação

## Entrega concreta

Providers autorizados, DASH áudio, locators WebDAV e capability discovery.

Documento canônico: [spec](../../docs/delivery/01-roadmap.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Negociar auth/seek/codec/qualidade; DRM desconhecido gera erro explícito; roots privadas só com permissão.

## Verificação e evidência

QA-008, QA-013 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
