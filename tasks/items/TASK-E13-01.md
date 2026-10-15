# TASK-E13-01 — Provar Android arm64, bridge Rust e output

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E13-01 / REQ-E13-01 |
| Fase / prioridade | M0 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | android — pessoa a designar |
| Esforço provisório | 2–5 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E11-01](TASK-E11-01.md) — Criar workspace e baseline de engenharia
- [TASK-E01-01](TASK-E01-01.md) — Provar decode, gapless e precisão de samples
- [TASK-E01-02](TASK-E01-02.md) — Provar callback e output clock

## Entrega concreta

Spike NDK/ABI Rust, ownership de buffers, AAudio/Oboe e adapter Media3, com decisões de UI/runtime.

Documento canônico: [spec](../../docs/platforms/02-android.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Tocar WAV/FLAC em aparelho arm64 real; callback sem JNI/alloc; native build compatível com requisitos de páginas; comparar lifecycle e runtime antes congelar.

## Verificação e evidência

QA-019, QA-021 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
