# TASK-E12-03 — Validar ARM64 adicional Windows/Linux e helpers

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E12-03 / REQ-E12-03 |
| Fase / prioridade | M5 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | expansion — pessoa a designar |
| Esforço provisório | 10–25 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E12-01](TASK-E12-01.md) — Entregar Linux nativo e MPRIS na 1.0
- [TASK-E12-02](TASK-E12-02.md) — Entregar macOS Intel e Apple Silicon nativos na 1.0

## Entrega concreta

Matriz opcional ARM64 Windows/Linux; Android arm64 e macOSarm64 já são obrigatórios emM3.

Documento canônico: [spec](../../docs/delivery/01-roadmap.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Arquiteturas adicionais têm build/hardware/output próprios; nenhum fallback oculto; a expansão não posterga Android/Mac arm64 obrigatórios.

## Verificação e evidência

QA-015, QA-017 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
