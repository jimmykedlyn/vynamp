# TASK-E14-06 — Fechar release 1.0 multiplataforma

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E14-06 / REQ-E14-06 |
| Fase / prioridade | M3 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | qa-platform — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E14-04](TASK-E14-04.md) — Executar corpus e outputs por alvo
- [TASK-E14-05](TASK-E14-05.md) — Validar packaging e update em cada alvo
- [TASK-E13-10](TASK-E13-10.md) — Fechar aceite Android 1.0 em hardware
- [TASK-E10-08](TASK-E10-08.md) — Executar soak e matriz de hardware

- [TASK-E16-07](TASK-E16-07.md) — Certificar ativação e associações em cinco alvos
- [TASK-E16-08](TASK-E16-08.md) — Validar portable, backup e limpeza de integração
- [TASK-E17-12](TASK-E17-12.md) — Publicar manifest real de capacidades por artefato
- [TASK-E17-20](TASK-E17-20.md) — Fechar gate combinado de codecs, DSP e visualizers

## Entrega concreta

Matriz de evidência agregada para Windows/Linux/macOS Intel/macOS Silicon/Android.

Documento canônico: [spec](../../quality/03-matriz-multiplataforma.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Todos os alvos M1–M3 aprovados com hardware e acessibilidade; alvo ausente bloqueia stable multiplataforma; zero bugs P0/P1 e exceções por capability explícitas.

## Verificação e evidência

QA-018, QA-019, QA-020, QA-021 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.

## Identidade aprovada Vynamp C

Incluir QA da identidade Vynamp C nos cinco alvos: legibilidade em tamanhos pequenos, alpha/DPI/máscaras/temas, launchers/shortcuts/tray; report por hardware e hashes. [Guia canônico](../../design/04-marca.md). Este requisito complementar não altera IDs/estado nem dispensa o contrato acima.
