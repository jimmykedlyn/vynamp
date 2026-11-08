# TASK-E13-06 — Integrar streaming, podcasts e downloads Android

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E13-06 / REQ-E13-06 |
| Fase / prioridade | M2 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | android-streaming — pessoa a designar |
| Esforço provisório | 5–12 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E13-03](TASK-E13-03.md) — Entregar audio focus e rotas Android
- [TASK-E13-04](TASK-E13-04.md) — Entregar MediaStore, SAF e permissões
- [TASK-E05-03](TASK-E05-03.md) — Entregar rádio Icecast/SHOUTcast e ICY
- [TASK-E05-05](TASK-E05-05.md) — Entregar feeds e episódios de podcasts

## Entrega concreta

Adapters do core para HTTP/ICY, podcasts e jobs de download respeitando condições do sistema.

Documento canônico: [spec](../../platforms/02-android.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Cancel/retry e rádio screen-off funcionam; permissões, rede e jobs respeitados; não iniciar dois decoders produzindo áudio simultaneamente.

## Verificação e evidência

QA-008, QA-019, QA-020 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
