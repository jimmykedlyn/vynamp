# TASK-E03-09 — Entregar letras, bookmarks e histórico

Versão: 0.7.1 · Data: 2026-10-28 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E03-09 / REQ-E03-09 |
| Fase / prioridade | M3 / P1 |
| Estado | Planejada; não implementada |
| Responsável funcional | library — pessoa a designar |
| Esforço provisório | 5–15 dias úteis; reestimar após M0 |
| Risco | médio |

## Dependências

- [TASK-E03-04](TASK-E03-04.md) — Implementar leitura de tags e capas segura
- [TASK-E03-02](TASK-E03-02.md) — Implementar queue independente e shuffle
- Plano 04 (metadados e CUE) da biblioteca privada `vynamp-decode` (https://github.com/jimmykedlyn/vynamp-decode), com TASK-E03-04: leitura nativa de USLT/SYLT e campos de letras. A tabela de famílias de tags no [doc canônico](../../docs/library/02-tags-lyrics-tools.md#letras-componente-resumido-sob-o-vu-e-fontes) é baseline relatada de um protótipo externo, **não suporte nativo**.

## Entrega concreta

LRC/plain lyrics, progress/bookmarks, local play history e opt-in providers.

Inclui o componente resumido de letras sob o VU (LYR-01..LYR-15), com fontes, estados, persistência, concorrência, acessibilidade e desempenho definidos em [Letras: componente resumido sob o VU e fontes](../../docs/library/02-tags-lyrics-tools.md#letras-componente-resumido-sob-o-vu-e-fontes). Resumo do escopo:

- apresentação de três linhas sob o VU, sem reduzir os 12 VUs (LYR-01..06; posição em [02-layouts](../../docs/design/02-layouts.md#letras-sob-o-vu-2026-10-28));
- leitura automática de letra embutida, texto sem tempos sem simular canto, importar LRC e voltar aos metadados, menu de opções (LYR-07..10);
- Desligar global × Instrumental por faixa, estados distintos, origem discreta/DEMO, independência do VU (LYR-11..15);
- continuam obrigatórios os controles do painel completo (auto-scroll, offset, tamanho, busca opt-in, edição, cópia, leitura assistiva); clique em linha para seek segue proposta.

Documento canônico: [spec](../../docs/library/01-library-playlists.md); letras: [doc canônico de letras](../../docs/library/02-tags-lyrics-tools.md#letras-componente-resumido-sob-o-vu-e-fontes). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Synced lyrics seguem clock consumido; rescan preserva bookmark; seek não conta como consumo; sem rede involuntária.

Critérios de letras LYR-AC-01..LYR-AC-12 (cenários completos no [doc canônico](../../docs/library/02-tags-lyrics-tools.md#critérios-de-aceite)): SYLT suportado e relógio (01); troca de trecho, pausa e retomada sem deriva (02); seek, Stop e troca de faixa recalculam sem texto da faixa anterior (03); texto simples sem simular canto (04); fixtures por família, UTF, LRC com offset e múltiplos tempos, tags truncadas (05); LRC válido e inválido, retorno aos metadados (06); Desligar persistente, Instrumental por faixa e reversível (07); resultados assíncronos obsoletos descartados (08); movimento reduzido, teclado e foco (09); 12 VUs, temas, tela estreita e texto ampliado (10); estados distintos e nenhuma consulta online involuntária (11); regressão de transporte, waveform, VU, biblioteca/fila e DSP (12).

Estado de evidência: **especificado**. Implementação e validação no protótipo externo (revisão 14 do Codex) são evidência relatada, não verificada neste repositório. Nativo: nada implementado nem validado; o protótipo não conta como aceite desta tarefa.

## Verificação e evidência

QA-006, QA-016 em [QA comum](../../docs/quality/01-test-strategy.md) e [matriz multiplataforma](../../docs/quality/03-multiplatform-matrix.md). Registrar commit, comando, hardware/ABI/OS e hashes de fixtures. Ausência de hardware é limitação, não resultado aprovado; emulator/cross-build não substitui Mac Intel/Silicon ou Android físico.

Fixtures de aceite de letras (LYR-AC-01..12): um arquivo por família de tags (ID3v2.3/2.4 USLT e SYLT em ms, FLAC/Vorbis, Ogg/Opus, M4A `©lyr`, WAV com bloco ID3), LRC com offset e múltiplos timestamps, UTF suportados, tags truncadas, SYLT em quadros MPEG (não suportado), LRC inválido após válido, e cenário de corrida assíncrona com troca de faixa e Desligar durante leitura/importação. **Pendente:** criar o item QA correspondente em `docs/quality/01-test-strategy.md` e `docs/quality/04-integration-codecs-dsp.md` (não existe hoje); registrar hashes das fixtures ao criá-las.

## Execução e limites

Confirmar ports/contratos e dependências; decompor itens grandes em slices de 1–3 dias; implementar sucesso e erro/cancelamento; executar os critérios apropriados; atualizar schema/docs/notices; aceitar somente com evidência e revisão. Preservar dados, budgets e acessibilidade. Stubs/botões desabilitados não contam. Esforço é estimativa, não data prometida. Atualizar backlog JSON e requisito junto de qualquer mudança.
