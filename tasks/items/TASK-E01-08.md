# TASK-E01-08 — Completar codecs desktop e large files

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E01-08 / REQ-E01-08 |
| Fase / prioridade | M3 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | audio — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E01-07](TASK-E01-07.md) — Adicionar codecs modernos e capítulos

## Entrega concreta

WMA/APE/WV/TTA/MPC/MP1/MP2/WebM/MKA/RF64 adapters com isolation quando nativos.

Documento canônico: [spec](../../reference/audio/01-engine.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Arquivos >4GiB usam offsets64; cada perfil tem fixture e lista runtime; unsupported sem crash.

## Verificação e evidência

QA-015 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
