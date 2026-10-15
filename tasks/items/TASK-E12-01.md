# TASK-E12-01 — Entregar Linux nativo e MPRIS na 1.0

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E12-01 / REQ-E12-01 |
| Fase / prioridade | M3 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | expansion — pessoa a designar |
| Esforço provisório | 10–25 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E14-02](TASK-E14-02.md) — Entregar ports de arquivos, credenciais e sistema
- [TASK-E02-05](TASK-E02-05.md) — Construir Library Workspace e inspector

## Entrega concreta

Porta Linux x86_64, PipeWire/PulseAudio, MPRIS, Orca, Secret Service e packaging testado.

Documento canônico: [spec](../../docs/platforms/01-support-matrix.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Passar corpus comum e gates M1–M3 no Linux nativo; Wayland/X11 e portas/saída testados; não usar Wine como substituto; instalar pacote real.

## Verificação e evidência

QA-003, QA-010, QA-017 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
