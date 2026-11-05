# TASK-E18-02 — Entregar Vynamp Converter e a lib de encoders

Versão: 0.8.0 · Data: 2026-11-05 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E18-02 / REQ-E18-02 |
| Fase / prioridade | M3 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | library — pessoa a designar |
| Esforço provisório | 15–35 dias úteis; reestimar após M0 |
| Risco | alto |

## Dependências

- [TASK-E08-01](TASK-E08-01.md) — Implementar jobs persistidos
- [TASK-E01-07](TASK-E01-07.md) — Adicionar codecs modernos e capítulos
- [TASK-E08-02](TASK-E08-02.md) — Entregar conversor e encoder registry
- [TASK-E11-03](TASK-E11-03.md) — Resolver licenças e inventário de dependências

## Entrega concreta

Ferramenta desktop `vynamp-converter` com fila de jobs persistida, apoiada na lib própria `vynamp-encode` como única fonte de codificadores.

Documento canônico: [spec](../../docs/tools/02-vynamp-converter.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Conversão lossy nunca é anunciada como restauração hi-res; degradação de profundidade ou taxa exige confirmação e aparece no relatório; cancelamento e disco cheio preservam a source; saída é validada após escrita; colisão de nome não sobrescreve.

## Verificação e evidência

QA-015, QA-016, QA-023, QA-031 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md); conformidade de codec em [matriz de integração](../../docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
