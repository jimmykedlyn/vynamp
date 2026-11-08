# TASK-E14-02 — Entregar ports de arquivos, credenciais e sistema

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E14-02 / REQ-E14-02 |
| Fase / prioridade | M1 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | platform — pessoa a designar |
| Esforço provisório | 3–8 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E14-01](TASK-E14-01.md) — Provar os cinco targets e features portáveis
- [TASK-E01-03](TASK-E01-03.md) — Implementar state machine e generation cancellation

## Entrega concreta

Traits/adapters para Path/FD/Content URI, clocks/media controls/config/keychain por host.

Documento canônico: [spec](../../platforms/01-matriz-suporte.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Domain não depende de Win32; grants/focus encapsulados; nenhum fallback plaintext; callback não chama UI ou operação bloqueante de OS. A slice M1 abre arquivo e executa play/pause/seek/stop nativamente em Windows, Linux, Mac Intel, Mac Silicon e Android.

## Verificação e evidência

QA-018, QA-019, QA-021 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
