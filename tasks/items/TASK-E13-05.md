# TASK-E13-05 — Entregar interface phone e navegação acessível

Versão: 0.7.1 · Data: 2026-11-03 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E13-05 / REQ-E13-05 |
| Fase / prioridade | M1 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | android-ui — pessoa a designar |
| Esforço provisório | 3–8 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E13-02](TASK-E13-02.md) — Entregar serviço e MediaSession com engine Rust
- [TASK-E02-02](TASK-E02-02.md) — Implementar tokens e componentes semânticos

Referência visual: vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452 (rev.38), caminhos `mvp/...` e `out/...` conforme [referência do MVP](../../docs/design/06-mvp-reference.md). Caminhos relevantes: `mvp/` (layouts) e `out/svg/rendered/`.

## Entrega concreta

Shell Compose com tokens compartilhados, transporte/fila/EQ básico e TalkBack.

Documento canônico: [spec](../../docs/design/04-mobile-tablet.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Targets de 48 dp; fonte 200% faz reflow; nenhuma fila duplicada; back/rotation não reiniciam áudio; paleta e geometria phone conferidas.

## Verificação e evidência

QA-020, QA-010 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
