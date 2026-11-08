# Vynamp Restore — análise e restauração acústica

Versão: 0.1.1 · Data: 2026-11-05 · Estado: especificação normativa; implementação não iniciada.

Documento canônico da ferramenta desktop **Vynamp Restore** (`vynamp-restore`). Complementa [ADR-013](../../adr/ADR-013-restore.md) (decisão), [ADR-012](../../adr/ADR-012-integracao-shell.md) (integração com o shell e identidade), [ADR-011](../../adr/ADR-011-pilha-propria.md) (pilha de áudio própria) e [spec de plataforma](../../platforms/03-integracao-sistema.md) (contratos de CLI e registro). Matriz normativa de requisitos: [03-requirements](../../product/03-requisitos.md).

## 1. Escopo e fronteiras

### 1.1 O que é

Ferramenta desktop que **analisa** um arquivo de áudio, **identifica defeitos** de gravação/acervo, **sugere correções**, permite **ajuste fino** das ferramentas de restauração e **exporta** o resultado em arquivo novo, preservando os metadados da origem.

### 1.2 O que **não** é

| Fora de escopo | Motivo |
|---|---|
| Separação de voz por IA / remoção de vocal | Declarado no MVP como pendente; capacidade sem evidência ([ADR-013](../../adr/ADR-013-restore.md#4-correção-das-incoerências-do-mvp)). Registrada como **fase futura planejada e não prometida** em [restauração por IA](03-vynamp-restore-ia.md) e [ADR-015](../../adr/ADR-015-restauracao-ia.md); nada muda na 1.0 |
| Reconstrução de master a partir de master degradado | Não é restauração, é remasterização; exige outra decisão |
| Upscaling de hi-res / aumento real de resolução | Extensão de banda é **síntese**, não recuperação; o resultado é rotulado como tal. A super-resolução por IA é fase futura, com o mesmo rótulo obrigatório ([ADR-015](../../adr/ADR-015-restauracao-ia.md)) |
| Redução de ruído espectral (Wiener) | Módulo inerte no MVP; fora de escopo na 1.0 ([ADR-013](../../adr/ADR-013-restore.md#4-correção-das-incoerências-do-mvp)) |
| Dublagem, corte de silêncio, normalização destrutiva | Ferramentas distintas, com tarefas próprias |
| Editar tags de verdade | O Restore **preserva** metadados; a edição é do player ([TASK-E08-04](../../tasks/items/TASK-E08-04.md)) |

### 1.3 Fronteira do aplicativo

O Restore é um **executável separado** ([ADR-013](../../adr/ADR-013-restore.md#1-processo-separado-motor-compartilhado)). Não exige o player em execução, não conversa com ele e não depende de instância aberta.

**Contrato de invocação:**

```
vynamp-restore [opções] <caminhos...>
```

| Opção | Efeito |
|---|---|
| `--analyze` | Analisa os arquivos sem abrir a janela de edição (modo não interativo, para script e para a ação do shell) |
| `--list-defects` | Imprime o relatório de defeitos em texto estruturado e sai |
| `--export <dir>` | Exporta os arquivos da fila com o projeto corrente, sem abrir a interface |
| `--format <fmt>` | Formato de saída: `flac`, `wav`, `wav-f32`, `aiff`, `alac` (padrão: igual ao contêiner de origem, quando suportado) |
| `--no-metadata` | Exporta sem metadados (padrão: preservar) |
| `--overwrite` | Permite sobrescrever a origem (padrão: proibido) |
| `--version`, `--help` | Saída e código 0 |

**Códigos de saída:** `0` sucesso · `1` erro de uso · `2` arquivo ilegível ou formato não suportado · `3` falha na escrita (disco cheio, permissão) · `4` cancelado pelo usuário · `5` falha interna do motor (com diagnóstico).

**Comportamento com caminhos:** os arquivos entram na **fila de análise** na ordem recebida. Chamado pelo shell, o lote chega atômico ([ADR-012](../../adr/ADR-012-integracao-shell.md#3-preservação-de-lotes-batching-atômico)); o Restore não reordena nem descarta silenciosamente.

**Instância única:** uma instância do app por usuário ([TASK-E16-02](../../tasks/items/TASK-E16-02.md)). Uma segunda chamada entrega os arquivos à janela aberta via IPC local autenticado, em vez de abrir outra janela.

## 2. Motor de restauração

O motor é **biblioteca interna do workspace Rust**, versionada, com fixtures de referência. Não usa crates de terceiros ([ADR-011](../../adr/ADR-011-pilha-propria.md)); decodifica por `vynamp-decode` e opera sobre PCM decodificado.

- **Entrada:** PCM decodificado (f32 interleaved), taxa da origem preservada, layout de canais do contêiner preservado.
- **Processamento em blocos**, com teto de memória configurável (padrão 512 MiB). O motor **não** exige o arquivo inteiro em memória.
- **Determinismo:** o mesmo arquivo, com a mesma versão de motor e o mesmo estado de projeto, produz **byte a byte** a mesma saída. Nenhum estágio usa aleatoriedade, relógio ou paralelismo que altere ordem de operação.

### 2.1 Modelo de estado

O estado de um projeto de restauração é uma estrutura versionada e validada. **Valores fora de faixa são rejeitados, não saturados.**

| Campo | Tipo | Faixa / valores | Padrão |
|---|---|---|---|
| `version` | inteiro | `1` | `1` |
| `duration` | segundos | `0 .. 86400` (24 h) | derivado da origem |
| `enabled` | booleano | — | `false` |
| `monitor` | enum | `original` \| `treated` \| `removed` | `treated` |
| `scope` | enum | `all` \| `regions` | `all` |
| `tools` | objeto | ver [§3.2](#32-parâmetros-das-ferramentas) | ver tabela |
| `regions` | lista | até 128 regiões, sem sobreposição | vazia |
| `diagnostics` | objeto \| nulo | relatório estruturado | `null` |
| `engine_version` | string | versão do motor | da sessão |

**Monitor:**

| Modo | Saída | Significado |
|---|---|---|
| `original` | `original` | Sem tratamento |
| `treated` | `treated` | Sinal processado |
| `removed` | `original − treated` | **O que foi removido**, com o sinal invertido. Útil para ouvir a diferença |

A troca entre modos é feita por **mixagem suavizada** (~6 ms), sem clique. Com `enabled = false`, o monitor é forçado a `original`.

**Escopo (`scope`):** `all` aplica as ferramentas em toda a faixa; `regions` aplica as ferramentas **apenas dentro das regiões**, com fade nas bordas. Independentemente do escopo, o **ganho por região** é sempre aplicado.

### 2.2 Regiões

Uma região é um intervalo temporal com **ganho de atenuação** e **fade nas bordas**.

| Campo | Faixa | Notas |
|---|---|---|
| `id` | string ≤ 80 caracteres, único no projeto | Identificador opaco |
| `start` | `0 .. duration` s | |
| `end` | `0 .. duration` s, `end − start ≥ 0.01` s | Duração mínima de 10 ms |
| `gainDB` | `−90 .. 0` dB | `≤ −80` equivale a silêncio total |
| `fade` | `0.005 .. 1` s | Efetivamente limitado a `(end − start) / 2` |
| `kind` | `between` | **Único tipo na 1.0** — ver nota |

**Não há sobreposição:** as regiões são ordenadas por `start` e uma região não pode começar antes do fim da anterior. Regiões sobrepostas são **rejeitadas na validação**, não ajustadas.

**Ganho aplicado:** `ganho = 1 + (alvo − 1) · fade`, com curva de cosseno elevado nas bordas; `alvo = 10^(gainDB/20)` ou `0` quando `gainDB ≤ −80`.

**Nota sobre `kind`:** o MVP oferece um segundo tipo, `overlay`, associado à separação de voz por IA. Esse tipo **não é portado** ([ADR-013](../../adr/ADR-013-restore.md#4-correção-das-incoerências-do-mvp)). A 1.0 tem apenas regiões de ganho com fade. Regiões não alteram a **duração** da faixa — não há time-stretching. Reabrir `overlay` é decisão de escopo própria: a separação de fontes é fase futura e depende disso ([restauração por IA §4.2](03-vynamp-restore-ia.md#42-separação-de-fontes)).

## 3. Cadeia de restauração

### 3.1 Ordem dos estágios

A ordem é **normativa e fixa**. Não é configurável pelo usuário.

| # | Estágio | Condição | Observação |
|---|---|---|---|
| 1 | Reparo de crepitação (AR) | `decrackle > 0` | 2 passagens por canal |
| 2 | Passa-baixa de fase linear | `decrackle > 0` | 63 taps, Kaiser β=6; corte `min(18000 Hz, 0.4·fs)` |
| 3 | Remoção de artefatos de compressão | `deartifact > 0` | STFT |
| 4 | Extensão de banda | `bwe > 0` | Síntese de alta faixa |
| 5 | De-click | `click = true` | Substituição por média de vizinhos |
| 6 | Remoção de offset DC | `dc = true` | Passa-altas de 1 polo a 5 Hz |
| 7 | De-rumble | `rumble > 0` | Passa-altas Butterworth 2ª ordem |
| 8 | Notches de zumbido | `hum ≠ 0` ou `humAdaptive = true` | Fundamental + harmônicos 2 e 3 |
| 9 | Gate de ruído | `gate = true` | Envelope com ataque e relaxamento |
| 10 | Anti-saturação | `declip < 0` | Joelho em `tanh` |
| 11 | Regiões e ganho | sempre | Mistura e ganho por região |

**Alinhamento:** os estágios 1–4 têm latência (o de-crackle em duas passagens, o filtro de fase linear, a STFT e a extensão de banda). Quando qualquer deles está ativo, o sinal **original** é atrasado pela mesma quantidade, de modo que `original` e `treated` permaneçam temporalmente alinhados em todas as comparações. A latência total é exposta e exibida.

**Por que esta ordem:** o reparo e a filtragem de banda vêm antes porque as detecções seguintes operam sobre um sinal já saneado; a remoção de artefatos precede a extensão de banda porque a extensão sintetiza a partir do que sobrou; o gate vem antes da anti-saturação para que a atenuação de ganho aplicada pelo declip não descalibre a detecção de envelope do limiar de ruído (o piso que o gate mede está mais de 40 dB abaixo do joelho do declip, então um não corrige o que o outro corrige).

**Herança da ordem (declarada):** esta sequência reproduz exatamente o MVP web rev.38, onde ela **não** foi uma decisão acústica, e sim um artefato de partição da arquitetura JS: os estágios em bloco (reparo AR → passa-baixa → STFT → extensão de banda) rodam em `this.pre`, antes do laço por amostra, e o restante (de-click, DC, rumble, zumbido, gate, declip, ganho de região) vive dentro desse laço. A ordem deixa de ser inocente: o modelo AR de ordem 48 assume sinal de média zero, então um offset DC contamina a autocorrelação com um termo `+ μ²` e enviesa a estimação dos polos, e cliques impulsivos corrompem o frame de estimação. A 1.0 **mantém** a ordem por fidelidade ao protótipo; mover DC e de-click para antes do estágio 1 é melhoria acústica defensável, mas exige divergência explícita declarada em [ADR-013](../../adr/ADR-013-restore.md) e revalidação com fixtures novas — não é silenciosa.

### 3.2 Parâmetros das ferramentas

| Ferramenta | Parâmetro | Faixa | Padrão | Algoritmo |
|---|---|---|---|---|
| Remoção de DC | `dc` | booleano | `false` | Passa-altas de 1 polo, 5 Hz |
| Zumbido de rede | `hum` | `0` \| `50` \| `60` | `0` | Biquad notch Q=30 no fundamental e nas harmônicas 2 e 3 |
| Zumbido adaptativo | `humAdaptive` | booleano | `false` | Com `hum = 0`, aplica **50 e 60 Hz** simultaneamente |
| Gate de ruído | `gate` | booleano | `false` | Porta por envelope; ataque 3 ms, relaxamento 120 ms |
| | `threshold` | `−72 .. −12` dBFS | `−45` | Limiar do gate |
| De-click | `click` | booleano | `false` | Substitui a amostra pela média dos vizinhos quando o resíduo excede o limiar |
| | `clickThreshold` | `0.03 .. 0.9` (amplitude linear) | `0.2` | Sensibilidade |
| De-rumble | `rumble` | `0 .. 80` Hz (`0` = desligado) | `0` | Passa-altas Butterworth 2ª ordem, Q=0.7071 |
| De-crackle | `decrackle` | `0 .. 0.5` (`0` = desligado) | `0` | Modelo AR/L-SAR, ordem 48 |
| | `decrackleMode` | `soft` \| `strong` | `strong` | `soft` corta só o excedente; `strong` ressintetiza o trecho |
| | `decrackleCutoff` | `8000 .. 22000` Hz | `18000` | Corte do passa-baixa Kaiser do de-crackle; `min(corte, 0.4·fs)` |
| Anti-saturação | `declip` | `−12 .. 0` dB (`0` = desligado) | `0` | Atenua e arredonda picos acima do joelho em `−1 dBFS` |
| Artefatos de compressão | `deartifact` | `0 .. 1` | `0` | STFT: supressão de *birdies* e de pré-eco |
| Extensão de banda | `bwe` | `0 .. 1` | `0` | Síntese da faixa alta a partir da faixa-fonte |
| | `bweCutoff` | `8000 .. 22000` Hz | `16000` | Frequência de corte |

**Notas de comportamento:**

- **Zumbido adaptativo** com `hum` explícito usa **somente** a frequência explícita. Sem frequência explícita, aplica 50 **e** 60 Hz.
- **Anti-saturação** trabalha em joelho suave em `−1 dBFS`: o sinal passa a ser comprimido em vez de recortado, e o ganho é aplicado antes do joelho.
- **Extensão de banda** é **síntese**, não recuperação: o resultado **não** recupera informação perdida e é rotulado como estimativa. A extensão só é possível se existir uma oitava abaixo do corte; abaixo dessa condição a ferramenta fica indisponível com motivo explícito.
- **Gate de ruído** não é redução de ruído espectral — apenas atenua abaixo do limiar. A redução espectral não é prometida na 1.0.
- **De-crackle** tem latência própria e é a etapa mais cara da cadeia.
- **Passa-baixa do de-crackle:** o corte efetivo é `min(decrackleCutoff, 0.4·fs)`. Em qualquer taxa ≥ 44,1 kHz o corte travado do MVP (18 kHz) elimina conteúdo acima de 18 kHz, incluindo o espectro ultrassônico de gravações de 96 kHz/192 kHz. Restaurações de masterizações de hi-res que precisem preservar essa banda devem elevar `decrackleCutoff` ou desligar o de-crackle (`decrackle = 0`) — este comportamento é intencional e está declarado em [ADR-013](../../adr/ADR-013-restore.md#4-correção-das-incoerências-do-mvp).
- **Artefatos de compressão** operam em STFT e implicam atraso próprio, exibido ao usuário.

### 3.3 Presets

**5 presets integrados**, imutáveis. Cada um declara o bloco `tools` exato que aplica; o denoise Wiener do MVP está fora de escopo na 1.0, então a ação principal de cada preset usa as ferramentas existentes (de-crackle, de-click, gate, DC, rumble, zumbido).

| id | Nome | `tools` aplicado | Cobre |
|---|---|---|---|
| `builtin-scratched-cd` | CD arranhado | `dc: true`, `gate: true`, `threshold: −45`, `click: true`, `clickThreshold: 0.18`, `decrackle: 0`, `rumble: 0`, `hum: 0` | Cliques isolados e chiado leve de superfície |
| `builtin-vinyl` | Vinil / LP | `dc: true`, `hum: 60`, `gate: false`, `threshold: −45`, `click: true`, `clickThreshold: 0.22`, `decrackle: 0.15`, `rumble: 30` | Crepitação densa, zumbido de rede, ronco grave |
| `builtin-cassette` | Fita K7 | `dc: true`, `hum: 0`, `gate: false`, `threshold: −45`, `click: false`, `clickThreshold: 0.2`, `decrackle: 0`, `rumble: 20` | Chiado de fita e saturação leve |
| `builtin-phone-recording` | Gravação celular | `dc: true`, `hum: 0`, `gate: true`, `threshold: −40`, `click: false`, `clickThreshold: 0.2`, `decrackle: 0`, `rumble: 40` | Ruído ambiente, possível offset DC, agudos limitados |
| `builtin-low-mp3` | MP3 baixa qualidade | `dc: false`, `hum: 0`, `gate: false`, `threshold: −45`, `click: false`, `clickThreshold: 0.2`, `decrackle: 0`, `rumble: 0` | Artefatos de compressão, chiado digital, perda de agudos |

**Presets de usuário:** armazenamento próprio, com o mesmo esquema de validação dos parâmetros. Campos: `id` (≤ 80 caracteres), `name` (1–60 caracteres), `description` (≤ 120 caracteres, truncada). Um preset de usuário define **apenas** as ferramentas presentes; as demais mantêm o valor corrente. Regiões e diagnósticos **não** são afetados por preset.

**Aviso obrigatório:** aplicar um preset com **zumbido ativo** (`hum: 50` ou `hum: 60`, ou `humAdaptive = true`) ou **passe-baixa agressivo** (corte abaixo de 8 kHz) exibe aviso de que pode perder conteúdo acima de 8 kHz. A condição de "redução de ruído alta" do denoise do MVP foi removida da 1.0 com o módulo denoise (fora de escopo).

## 4. Diagnóstico

O diagnóstico é o produto central da ferramenta ([ADR-013](../../adr/ADR-013-restore.md#6-diagnóstico-é-o-produto-não-um-extra)). É uma **análise do arquivo**, não do trecho visível.

### 4.1 Formato do relatório

Relatório **estruturado** (não texto), com:

| Campo | Conteúdo |
|---|---|
| `duration`, `sampleRate`, `channels` | Medidas do arquivo |
| `effectiveNyquist` | Frequência mais alta com energia acima de −60 dB |
| `defects[]` | Lista de defeitos (ver abaixo) |
| `chain[]` | Cadeia de ferramentas sugerida, em ordem |
| `summary` | Resumo em texto para humans |

Cada defeito:

| Campo | Conteúdo |
|---|---|
| `type` | Identificador do tipo de defeito |
| `severity` | `mild` \| `moderate` \| `severe` |
| `evidence` | Valores medidos que motivaram a detecção |
| `suggestion` | Texto de sugestão exibido ao usuário |
| `fix` | Patch de parâmetro aplicável, ou `null` quando não há correção |
| `examples` | Até 6 marcações de tempo (segundos) |

**Severidade** é ordinal e determinística: `severe` > `moderate` > `mild`. Não há confiança numérica — a evidência numérica é o campo `evidence`, e o usuário a vê.

### 4.2 Detectores

| Tipo | Detecção | Severidade | Correção sugerida |
|---|---|---|---|
| `clipping` | Amostras acima de 0.999; contagem de blocos contíguos, overs e pico | `severe` se há overs ou > 60 blocos/min; `moderate` se > 10 | `declip: <headroomDB>`, sendo `headroomDB = −min(12, max(1, ⌈pico dBFS + 1⌉))`; em `moderate`/`severe` acrescenta `decrackle: 0.15` |
| `true_peak` | Interpolação linear 2× entre amostras; reportado só quando não há clipping | `severe` se > 1.1, senão `moderate` | `declip: −2` |
| `click` | Impulsos adaptativos: resíduo acima do piso e acima de 20× a média central; vizinhos não impulsivos | Por taxa: `severe` > 30/min, `moderate` > 5/min | `click: true` com limiar derivado |
| `crackle` | Mesmo detector. **Mono**: opera no canal único (não existe side). **Estéreo**: aplica a mid `(L+R)/2` e side `(L−R)/2`. **Surround (≥ 3 canais)**: opera **por canal**, sem par side, e a contagem é a soma dos canais | Por taxa: `severe` > 120/min, `moderate` > 20/min | `decrackle: 0.15` |
| `hum50` / `hum60` | FFT com harmônicos 1–4; proeminência > 10 dB; rejeição de tons musicais | `severe` se proeminência > 20 dB, senão `moderate` | `hum: <frequência>` |
| `hiss` | Energia acima de 4 kHz em janelas; proxy de achatamento espectral | `severe` se razão > 0.5, `moderate` se > 0.35 | `null` |
| `rumble` | RMS na banda 5–40 Hz | `severe` se > −20 dBFS, senão `moderate` | `rumble: 30` |
| `discontinuity` | Saltos entre amostras adjacentes acima de 12× a média local, excluindo cliques isolados | Por taxa: `severe` > 20/min, `moderate` > 3/min | `null` |
| `skip` | Correlação cruzada de blocos consecutivos acima de 0.995 | `severe` se > 10 ocorrências, senão `moderate` | `null` |
| `dc_offset` | Média por canal acima de 0.005 | `severe` se > 0.02, senão `moderate` | `dc: true` |
| `band_limit` | Corte de banda de compressão com perda (MP3/AAC): degrau ≥ 15 dB entre 10 kHz e 0.45·fs | `moderate` se corte < 17 kHz, senão `mild` | `bwe: 0.5` + `bweCutoff` + `deartifact: 0.5` |

**Defeitos sem correção** (`hiss`, `discontinuity`, `skip`) são **reportados sem correção inventada**. A ausência de correção é informação.

**Detector de cliques, um só:** o MVP calculava cliques duas vezes — o detector `click` do diagnóstico e um módulo separado (`restore-click-detector.js`), cujo resultado ia para `report.clickDetection` e não era lido por nenhum consumidor da UI. A 1.0 mantém **apenas** o detector `click` desta tabela, que já entrega severidade, evidência e correção sugerida no mesmo relatório ([ADR-013 §4](../../adr/ADR-013-restore.md#4-correção-das-incoerências-do-mvp)).

**Rejeição de tom musical:** o detector de zumbido não pode acusar zumbido em nota musical. Um pico fundamental dentro de 3 % de uma frequência de nota musical é rejeitado (5 % de tolerância para o fundamental).

### 4.3 Aplicação de correções

- O motor **propõe**; o usuário **confirma**. Nenhuma correção é aplicada automaticamente na abertura do arquivo.
- "Aplicar correções recomendadas" aplica apenas defeitos com `fix` aplicável e severidade `moderate` ou `severe`.
- Cada aplicação é **reversível** (undo) e **registrada** na evidência da sessão.
- Aplicar uma correção liga o tratamento e move o monitor para `treated`.

## 5. Comparação

Duas formas de comparação, ambas com o original:

**Monitor A/B/C** — modos `original` / `treated` / `removed` do [§2.1](#21-modelo-de-estado), com troca suavizada.

**Comparação em cortina** — divide a janela visível entre original e tratado:

| Aspecto | Valor |
|---|---|
| Modos | `normal`, `wipe` (cortina), `stacked` (empilhado) |
| Razão de divisão | `0.02 .. 0.98`, padrão `0.5` |
| Pré-roll | 0.25 s antes da janela, para os filtros assentarem |
| Colunas de pico | até 2048 |

**Métricas exibidas:** RMS do original, RMS do tratado, RMS do que foi removido, **razão removido relativo** (dB), pico do original, pico do tratado, e indicação de "idêntico neste trecho" quando a diferença é nula. Piso de silêncio na referência: −120 dBFS.

## 6. Lote

| Aspecto | Valor |
|---|---|
| Limite de fila | 32 arquivos |
| Processamento por rodada | 8 arquivos |
| Concorrência | **Sequencial** na 1.0 — restauração é pesada e não disputa recursos com o player |
| Cancelamento | Por arquivo e da fila inteira; remove temporários; **a origem nunca é tocada** |
| Relatório agregado | Totais por estado, defeitos somados por tipo com a maior severidade, cadeia mais comum |

O relatório agregado lista: total, concluídos, falhas, cancelados, os defeitos mais frequentes com a severidade mais alta observada, e a cadeia de ferramentas mais comum entre os arquivos.

**Ordem:** a ordem dos arquivos na fila é preservada do início ao fim; um arquivo que falha **não** interrompe a fila.

## 7. Metadados

**Extração** da origem: ID3v2.2/2.3/2.4, chunks `LIST`/`INFO`, `id3 ` e `_PMX` de RIFF/WAV, e XMP quando presente. Cada bloco preservado tem teto de 16 MiB; cada campo textual tem teto de 1024 caracteres.

**Re-embedding** na saída: os campos são mapeados para o contêiner de destino. **Campos desconhecidos não são descartados** — blocos são copiados quando o contêiner de destino os suporta.

**Precedência:** quando a mesma informação aparece em duas fontes com codificações diferentes, prevalece a leitura que preserva acentos (UTF-8 sobre Latin-1).

**Falha não bloqueia:** erro ao gravar metadados é **informado** e não impede a exportação do áudio restaurado.

## 8. Exportação

| Aspecto | Contrato |
|---|---|
| Destino | Arquivo novo por padrão; a **origem nunca é sobrescrita** sem `--overwrite` explícito |
| Formatos | `flac`, `wav` (PCM 16/24/32 bits), `wav-f32`, `aiff`, `alac` |
| Escolha do formato | Igual ao contêiner de origem quando suportado; caso contrário, o usuário escolhe |
| Escrita | Temporário no **mesmo volume** → `fsync` → **validação pós-escrita no temporário** → `rename` sobre o destino |
| Validação pós-escrita | O arquivo **temporário** é relido e decodificado; contêiner, taxa, canais e duração precisam coincidir. Em caso de falha, o **temporário é removido** e a origem permanece intacta |
| Limiter | **Opcional**, ligado por padrão: teto de true-peak em `−1.0 dBTP`, lookahead de 5 ms, medição com oversampling 4× e margem de segurança de 0.5 dB |
| Normalização | **Opcional**, desligada por padrão: ganho único para pico em `−1 dBFS`, com hard-clip em ±1 |

**Rotulagem obrigatória:** a saída registra o que foi feito — versão do motor, parâmetros usados, defeitos detectados e correções aplicadas — em log de sessão e em arquivo de evidência. Isso torna a restauração **auditável**.

**Advertência de qualidade:** conversão com perda nunca é anunciada como restauração hi-res; a restauração **não** cria informação que não existia na origem.

## 9. Projeto `.vynamp-restore`

Arquivo JSON versionado, exportável e importável:

```json
{
  "format": "vynamp-restore",
  "version": 1,
  "source": { "name": "faixa.flac", "fingerprint": "..." },
  "engine_version": "0.1.0",
  "state": { "version": 1, "duration": 245.0, "enabled": true, "monitor": "treated", "scope": "all", "tools": {}, "regions": [], "diagnostics": null }
}
```

| Regra | Valor |
|---|---|
| Formato | `format` deve ser `vynamp-restore`; `version` deve ser `1` |
| Fingerprint | Identifica a origem (nome + tamanho + data de modificação). Um projeto **não aplicável** a outro arquivo é recusado |
| Tamanho máximo | 256 KiB |
| Projetos guardados | Até 12 rascunhos automáticos |
| Importação inválida | Rejeitada com motivo; nunca aplica parcialmente |

## 10. Limites

| Limite | Valor |
|---|---|
| Duração por arquivo | 24 h |
| Canais | Até 32; layout do contêiner preservado, sem re-mapeamento silencioso |
| Memória de trabalho | Teto configurável, padrão 512 MiB; decodificação em blocos |
| Regiões por projeto | 128 |
| Profundidade de undo/redo | 60 passos |
| Tamanho de projeto | 256 KiB |
| Arquivos por fila | 32 (8 por rodada) |

## 11. Matriz de plataformas

| Recurso | Windows 11 / 10 | Linux | macOS | Android |
|---|---|---|---|---|
| Abrir/analisar/exportar | ✅ | ✅ | ✅ | ❌ (fora de escopo na 1.0) |
| Menu de contexto | ✅ (moderno + clássico) | ✅ (Nautilus/Dolphin/Thunar/Nemo) | ✅ (Quick Actions) | ❌ |
| Ícone de arquivo do SO | ✅ | ✅ | ❌ (apenas in-app, por [ADR-012](../../adr/ADR-012-integracao-shell.md)) | ❌ |
| Fila e relatório agregado | ✅ | ✅ | ✅ | ❌ |

Android fica fora de escopo na 1.0: a decisão do mantenedor em 2026-11-03 restringiu Restore e Converter a **desktop** ([ADR-012](../../adr/ADR-012-integracao-shell.md#1-arquitetura-de-integração-por-plataforma)).

## 11.1 Identidade

Símbolo C / Pulso com **badge de restauração** (ondas de cura em ciano `#66D9EF`), ícone próprio `vynamp-restore.ico` / `tools/vynamp-restore.svg`, conforme [ADR-012](../../adr/ADR-012-integracao-shell.md#5-identidade-das-ferramentas-auxiliares). A entrada do manifesto é `vynamp-restore`, com `executable` `vynamp-restore` e `badge_color` `#66D9EF` ([manifesto de ícones](../../assets/icons/manifest.json)).

## 12. Verificação

| Evidência | Referência |
|---|---|
| Comandos e resultados | `evidence/PR/chore-specs-vynamp-restore/` |
| Fixtures por defeito | [Matriz de testes DSP](../../quality/04-matriz-dsp.md) |
| Estratégia de QA | [01-test-strategy](../../quality/01-estrategia-testes.md) — QA-015, QA-016, QA-023, QA-024, QA-031 |
| Matriz multiplataforma | [03-multiplatform-matrix](../../quality/03-matriz-multiplataforma.md) |

A validação deste documento é **documental e estrutural**. Nenhuma integração foi executada em sistema operacional, nenhum arquivo foi restaurado e nenhum hardware de saída de áudio foi exercitado. A ausência de hardware é limitação declarada, não resultado aprovado.
