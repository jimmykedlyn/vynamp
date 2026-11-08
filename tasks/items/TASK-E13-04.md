# TASK-E13-04 — Entregar MediaStore, SAF e permissões

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E13-04 / REQ-E13-04 |
| Fase / prioridade | M2 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | android-library — pessoa a designar |
| Esforço provisório | 5–12 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E13-02](TASK-E13-02.md) — Entregar serviço e MediaSession com engine Rust
- [TASK-E03-03](TASK-E03-03.md) — Implementar import incremental e roots offline

## Entrega concreta

Adapters de document URI/FD, grants persistidos, indexação e capabilities de escrita de tags.

Documento canônico: [spec](../../platforms/02-android.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Deny/revoke preserva biblioteca e rádio; não exigir acesso irrestrito a arquivos; Content URI não vira path fictício; backup/limites de escrita explícitos.

## Verificação e evidência

QA-020, QA-007 em [QA comum](../../quality/01-estrategia-testes.md) e [matriz multiplataforma](../../quality/03-matriz-multiplataforma.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
