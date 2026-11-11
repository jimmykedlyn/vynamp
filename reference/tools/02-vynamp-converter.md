# Vynamp Converter — conversão e transcodificação

Versão: 0.1.1 · Data: 2026-11-11 · Estado: **escopo preparatório**; implementação não iniciada. A execução completa fica no PR5.

Documento canônico da ferramenta desktop **Vynamp Converter** (`vynamp-converter`). Complementa [ADR-013](../../adr/ADR-013-restore.md) (fronteira de processo e motor compartilhado), [ADR-012](../../adr/ADR-012-integracao-shell.md) (ação de menu de contexto e identidade), [ADR-011](../../adr/ADR-011-pilha-propria.md) (pilha de áudio própria) e [TASK-E08-02](../../apps/converter/tasks/items/TASK-E08-02.md) (conversor e encoder registry). Matriz normativa: [03-requirements](../../product/03-requisitos.md).

> **Código-fonte:** [`jimmykedlyn/vynamp-converter`](https://github.com/jimmykedlyn/vynamp-converter)
> e a lib de codificadores em [`jimmykedlyn/vynamp-encode`](https://github.com/jimmykedlyn/vynamp-encode).
> Este documento continua sendo a fonte normativa; os repos dos módulos guardam o código e o
> esqueleto de `specs/`, não uma segunda cópia do texto normativo.

> **Estado deste documento.** O escopo, as matrizes de formato, as regras de qualidade e os contratos de arquivo estão fechados aqui. A **execução** — criação da lib `vynamp-encode`, implementação da fila de jobs, artefatos de release e evidência — é o PR5. Onde este documento diz "a 1.0", o escopo fechamento está decidido e não será reaberto sem nova ADR.

## 1. Escopo

Ferramenta desktop de **transcodificação e conversão em lote**, executável separado (`vynamp-converter`), sobre as mesmas libs do player.

| Item | Decisão |
|---|---|
| Processo | Executável próprio, uma instância por usuário, mesma fronteira que o Restore ([ADR-013](../../adr/ADR-013-restore.md#1-processo-separado-motor-compartilhado)) |
| Motor de codificação | **Biblioteca própria `vynamp-encode`**, repositório público [`jimmykedlyn/vynamp-encode`](https://github.com/jimmykedlyn/vynamp-encode), Rust `std`-only e modular. É a **única** fonte de codificadores do produto |
| Codecs | Lossless (WAV, AIFF, FLAC, ALAC) e lossy (Opus, MP3, AAC, Vorbis) |
| AAC | **Sem ressalva.** O risco de patente foi avaliado e **aceito e registrado** pelo mantenedor; a decisão é explícita, não omissa |
| MP3 | Permitido; a redação de "quando permitido" em [TASK-E08-02](../../apps/converter/tasks/items/TASK-E08-02.md) significa **quando o encoder estiver disponível na build**, não "depende de permissão a cada uso" |
| Workers | Fila de workers limitada, com cancelamento que remove temporários e **preserva a origem** |
| Áudio | Escala de bit e profundidade **não** degradadas por padrão |

**Fora de escopo na 1.0:** edição de tags (é do player), organização de pastas e dedupe ([TASK-E08-04](../../apps/player/tasks/items/TASK-E08-04.md)), extração de CUE, gravação de fonte autorizada, gravação de disco ([TASK-E08-05](../../apps/player/tasks/items/TASK-E08-05.md)).

**Fora de escopo em mobile (Android e iOS).** O Converter é **exclusivamente desktop** na 1.0. Não há build, activity, target nem extensão de `vynamp-converter` no APK/AAB nem no app iOS, e nenhum app mobile abre conversão por deep-link ou Ação de Compartilhamento. A decisão é do mantenedor, de 2026-11-03, registrada em [ADR-012 §1](../../adr/ADR-012-integracao-shell.md#1-arquitetura-de-integração-por-plataforma) — a linha Android declara só **Reproduzir** e **Enfileirar no Vynamp** — e em [D-MOB-01](../../product/05-decisoes.md#decisões-de-mobile-2026-11-11). O [Restore §11](01-vynamp-restore.md#11-matriz-de-plataformas) já declarava o mesmo para a ferramenta irmã; este parágrafo fecha a assimetria.

## 2. Contrato de invocação

```
vynamp-converter [opções] <caminhos...>
```

| Opção | Efeito |
|---|---|
| `--format <fmt>` | Formato de saída (padrão: `flac`) |
| `--bitrate <kbps>` | Taxa alvo para formatos com taxa (Opus, MP3, AAC, Vorbis) |
| `--quality <q>` | Parâmetro de qualidade para codecs de taxa variável (VBR) |
| `--sample-rate <hz>` | Taxa de saída; padrão = taxa da origem |
| `--bit-depth <n>` | Profundidade para PCM/WAV (16, 24, 32) e FLAC (16, 24) |
| `--channels <n>` | Canais de saída; padrão = canais da origem |
| `--out <dir>` | Diretório de destino |
| `--name-template <tpl>` | Template de nome de saída, com whitelist de campos e sanitização |
| `--overwrite` | Permite sobrescrever um arquivo existente (padrão: proibido) |
| `--dry-run` | Mostra o plano de conversão sem escrever |
| `--preset <id>` | Preset de codificação nomeado |
| `--version`, `--help` | Saída e código 0 |

**Códigos de saída:** `0` sucesso · `1` erro de uso · `2` entrada ilegível ou formato não suportado · `3` falha de escrita · `4` cancelado · `5` erro interno do encoder.

Com caminhos, os arquivos entram na fila de conversão na ordem recebida, preservando a ordem do lote atômico do shell ([ADR-012](../../adr/ADR-012-integracao-shell.md#3-batching-atomico)).

## 3. Matriz de formatos

**Entrada** — todos os 17 formatos de áudio do produto ([ADR-012](../../adr/ADR-012-integracao-shell.md#4-sistema-de-pacotes-de-ícones-de-arquivo)), lidos por `vynamp-decode`.

**Saída:**

| Formato | Tipo | Extensão | Parâmetros | Observações |
|---|---|---|---|---|
| FLAC | Lossless | `.flac` | 16/24 bits, taxa da origem | Subconjunto do codificador próprio |
| WAV | PCM | `.wav` | 16/24/32 bits int, 32 bits float | PCM linear, sem float→int implícito |
| AIFF | PCM | `.aiff` | 16/24 bits | Big-endian, conforme o padrão |
| ALAC | Lossless | `.m4a` | 16/24 bits | Em contêiner MP4 |
| Opus | Lossy | `.opus` | taxa 64–256 kbps, VBR | |
| MP3 | Lossy | `.mp3` | taxa 128–320 kbps, CBR/VBR | |
| AAC | Lossy | `.m4a` | taxa 96–256 kbps | Risco de patente aceito e registrado |
| Vorbis | Lossy | `.ogg` | VBR por qualidade | |

**Não há saída DSD na 1.0.**

**Regras de conversão:**

- **Sem degradação silenciosa:** converter de 24 bits para 16 bits exige confirmação explícita na interface e aparece no relatório da conversão.
- **Sem re-mapeamento silencioso de canais:** mono→estéreo e estéreo→mono são **decisões explícitas**, com a regra de downmix documentada e o ganho aplicado de forma declarada (evita queda de nível por soma de canais).
- **Taxa de amostragem:** padrão é preservar a taxa de origem. Reduzir taxa aplica um resampler próprio; **a escolha de encoder de resampler é da `vynamp-encode`** e deve ser registrada no relatório da conversão.
- **DSD:** entrada DSD é convertida para PCM na taxa equivalente **antes** da codificação, com o perfil de conversão registrado. Não há saída DSD na 1.0.

## 4. Qualidade e rotulagem

O produto distingue **lossless** (sem perdas em relação à entrada) de **lossy** (com perdas). A rotulagem é obrigatória em todo relatório e na interface:

- Uma conversão lossy **nunca** é anunciada como "restauração hi-res" nem como "sem perdas" ([TASK-E08-02](../../apps/converter/tasks/items/TASK-E08-02.md)).
- O relatório de conversão registra, por arquivo: formato de origem, taxa e profundidade de origem, formato de destino, parâmetros, **ganho aplicado**, e se houve degradação de profundidade ou de taxa.
- O relatório indica explicitamente a perda esperada: taxa alvo, tipo de encoder e a ressalva de que a qualidade perceptual é **estimativa**, não medida por auscultação.

## 5. Metadados

- **Extração** da origem com as mesmas regras do Restore ([spec do Restore §7](01-vynamp-restore.md#7-metadados)): ID3v2, `LIST`/`INFO`, XMP e campos técnicos.
- **Re-embedding** no contêiner de destino, mapeando campos para o formato de destino. **Campos desconhecidos não são descartados** quando o contêiner os suporta.
- **Capa** preservada quando o destino suporta (a entrada é extraída e embutida).
- Falha ao gravar metadados é **informada** e não impede a conversão.

## 6. Fila de jobs

| Aspecto | Contrato |
|---|---|
| Limite de fila | 100 arquivos |
| Workers | Limitados por capacidade da máquina; nunca mais que o número de núcleos menos 1 |
| Persistência | A fila é **persistida** ([TASK-E08-01](../../libs/engine/tasks/items/TASK-E08-01.md)): uma queda de energia retoma jobs seguros |
| Cancelamento | Por arquivo e da fila; remove os temporários; **a origem nunca é tocada** |
| Escrita | Temporário no **mesmo volume** → `fsync` → **validação pós-escrita no temporário** → `rename` sobre o destino |
| Validação pós-escrita | O arquivo **temporário** é relido e decodificado; contêiner, taxa, canais e duração precisam coincidir. Em caso de falha, o **temporário é removido** e a origem permanece intacta |
| Colisão de nome | Detectada antes de escrever; **não sobrescreve** por padrão; a ação é do usuário |
| Idempotência | Reprocessar o mesmo arquivo com os mesmos parâmetros produz o mesmo resultado; jobs duplicados são detectados |

**Nomes de saída:** template com **whitelist** de campos (nome da faixa, artista, álbum, faixa, ano) e sanitização de separadores e nomes reservados, conforme [TASK-E08-04](../../apps/player/tasks/items/TASK-E08-04.md). O plano completo de nomes é exibido **antes** de escrever.

## 7. Identidade

Símbolo C / Pulso com **badge de conversão** (setas circulares em verde-lima `#B8F35A`), ícone próprio `vynamp-converter.ico` / `tools/vynamp-converter.svg`, conforme [ADR-012](../../adr/ADR-012-integracao-shell.md#5-identidade-das-ferramentas-auxiliares).

## 8. Verificação

| Evidência | Referência |
|---|---|
| Comandos e resultados | `evidence/PR/chore-specs-vynamp-converter/` (PR5) |
| Fixtures de codec | [Matriz de testes DSP](../../quality/04-matriz-dsp.md), [04-integration-codecs-dsp](../../_arquivo/repo/docs/quality/04-integration-codecs-dsp.md) |
| Estratégia de QA | [01-test-strategy](../../quality/01-estrategia-testes.md) — QA-015, QA-016, QA-023, QA-024, QA-031 |

A validação deste documento é **documental e estrutural**. Nenhum arquivo foi transcodificado, nenhum encoder foi implementado e a lib `vynamp-encode` ainda não existe.
