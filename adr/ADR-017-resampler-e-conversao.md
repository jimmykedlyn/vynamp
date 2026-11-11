# ADR-017 — qualidade do resampler e da conversão de amostras

Versão: 0.1.0 · Data: 2026-11-11 · Estado: decisão de projeto do spike de M0. Os números desta versão são **alvo derivado do projeto do filtro**, não medição; a revisão que acompanha a evidência da [TASK-E04-02](../libs/dsp/tasks/items/TASK-E04-02.md) troca cada um pelo medido.

Status: escrita pela TASK-E04-02 em 2026-11-11 e **sujeita à aprovação do mantenedor**. Não decide o nome nem o repositório da biblioteca de resampling — isso continua aberto em D-DSP-08 e na [ADR-011](ADR-011-pilha-propria.md). Preenche a lacuna que a [matriz DSP](../quality/04-matriz-dsp.md) já citava como "metas do ADR de E04-02": até esta ADR existir, o `DSP-T-13` exigia aliasing e ruído "dentro das metas" de um documento que ninguém tinha escrito.

## Contexto

A [ADR-011](ADR-011-pilha-propria.md) tirou o resampler de terceiros do caminho de áudio e decidiu uma biblioteca própria; a D-DSP-08 registrou a decisão em 2026-11-03 e apontou o spike para a TASK-E04-02. Nenhum dos dois documentos diz **quanto** de rejeição de alias basta, **qual** latência é aceitável, **como** a conversão para inteiro deve se comportar, nem **onde** o dither entra. Sem esses números, "resampler de alta qualidade" é um adjetivo: não há como aprovar nem reprovar uma entrega.

Três fontes normativas já fixam o que a biblioteca **não** pode fazer, e esta ADR não as altera:

- [04-outputs-hires](../reference/audio/04-outputs-hires.md) §Resampling/dither: "Modos balanced/high; latência e CPU medidas; tratar razão não inteira e estado entre blocos. Dither TPDF somente ao reduzir precisão para saída inteira, último estágio, nunca DSD/float nem bypass transparente. Noise shaping opcional M5. Não elevar resolução para promover marketing."
- [09-dsp-catalog](../reference/audio/09-dsp-catalog.md) §Resampler/SRC: posição final única no grafo (o conflito D-DSP-05 com o vitrola fica aberto e não é desta ADR).
- [07-dsp-signal-contract](../reference/audio/07-dsp-signal-contract.md) §Caminho: o resampler roda **antes** da proteção final, e nada depois dela pode resamplear.

O que faltava era a camada do meio: os números.

## Decisão

### 1. Família de algoritmo: sinc com janela de Kaiser, tabela exata por fase racional

Um único kernel `h(u) = sinc(2·fc·u) · kaiser(u/half_eff)`. As posições de saída vêm de um acumulador de fase **inteiro**: reduzidas as taxas por `gcd`, a posição da amostra de saída `m` é `m·a/b` exata, com `a = fs_in/g` e `b = fs_out/g`. Como `gcd(a,b) = 1`, o conjunto de fases `{(m·a) mod b}` é exatamente `0..b-1`, então a fase é o **índice de linha** da tabela de coeficientes — sem mapa, sem interpolação e sem erro de quantização de coeficiente. Razão inteira e razão não inteira são o mesmo caminho.

Isso é o que decide a forma do kernel: os coeficientes são **pré-computados na construção**, `b` linhas por `taps`, e o caminho quente é convolução pura. Avaliar o kernel no caminho quente custaria cerca de 12,5 milhões de pares `sinc`+`besseli` por segundo e por canal já em `high` a 48 kHz (48 000 saídas × 261 taps) — inviável como orçamento de CPU, e sem ganho nenhum sobre a tabela.

Rejeitadas: interpolação polinomial (B-spline, cúbica) — barata demais para o alvo de alias; FFT overlap-add — latência de bloco incompatível com o requisito de estado entre blocos; tabela com subfases interpoladas — o acumulador inteiro já dá a tabela exata, e interpolar seria trocar exatidão por memória que não está apertada.

**Normalização por fase.** Cada linha é dividida pela soma dos seus próprios coeficientes. O ganho DC fica unitário em toda fase, em vez de "aproximadamente unitário" — a alternativa é um ripple de DC que aparece como ondulação de baixa frequência no sweep.

**Largura do filtro ao decimar.** A meia-janela efetiva é `half_eff = ceil(half_base / min(1, r))`, com `r = fs_out/fs_in`. Sem esse alargamento o filtro decimado deixa passar o que devia barrar: a seletividade exigida é relativa à taxa **de saída**, e a mesma janela em amostras de entrada fica `1/r` vezes mais estreita em frequência normalizada.

**A tabela é compartilhada.** `Kernel` é construída uma vez e o `Resampler` de cada canal guarda um `Arc` dela. Sem isso, um octeto carregaria oito cópias de até 1,3 MB — e a tabela não depende do canal.

### 2. Os dois modos, e o que cada um compra

Dimensões pelo projeto de Kaiser: `N ≈ (A − 8) / (14,357·Δf)`, `β = 0,1102·(A − 8,7)`, com `Δf` a largura de transição em ciclos por amostra e `A` a atenuação de projeto.

| Modo | Passband até | Stopband de | `A` | Taps | β | Custo por amostra |
|---|---|---|---|---|---|---|
| `balanced` | 0,44·min(fs_in,fs_out) | 0,50·min | 90 dB | 97 | 8,96 | 97 MAC |
| `high` | 0,47·min(fs_in,fs_out) | 0,50·min | 120 dB | 261 | 12,27 | 261 MAC |

Os dois modos diferem **na transição e na profundidade**, não só no comprimento: `high` preserva 0,47 do Nyquist em vez de 0,44 e aprofunda 30 dB. `high` custa 2,69× em MAC e em latência. Medido no spike, é essa a diferença que justifica a escolha existir; um segundo modo que só alongasse o mesmo filtro compraria decibéis a preço cheio e mais nada.

O custo real depende do par de taxas, e não é constante — é isso que torna o orçamento de fase e o alargamento por `1/r` itens de projeto, e não detalhe. Em `high`:

| Par | `b` (fases) | `half_eff` | Taps | Tabela | Latência |
|---|---|---|---|---|---|
| 44 100 → 48 000 | 160 | 130 | 261 | 163 KiB | 141 frames ≈ 2,95 ms |
| 48 000 → 44 100 | 147 | 142 | 285 | 164 KiB | 130 frames ≈ 2,96 ms |
| 44 100 → 96 000 | 320 | 130 | 261 | 326 KiB | 283 frames ≈ 2,95 ms |
| 96 000 → 44 100 | 147 | 283 | 567 | 326 KiB | 130 frames ≈ 2,95 ms |
| 192 000 → 44 100 | 147 | 566 | 1 133 | 651 KiB | 130 frames ≈ 2,95 ms |
| 48 000 → 192 000 | 4 | 130 | 261 | 4 KiB | 520 frames ≈ 2,71 ms |
| 44 100 → 384 000 | 1 280 | 130 | 261 | 1 305 KiB | 1 132 frames ≈ 2,95 ms |
| 384 000 → 44 100 | 147 | 1 132 | 2 265 | 1 301 KiB | 130 frames ≈ 2,95 ms |

Duas leituras dessa tabela valem a decisão: a **latência em tempo** fica em ~2,95 ms em quase todo par, e por construção não depende da direção — ela é `half_base / min(fs_in, fs_out)`, sempre o lado da taxa menor. E o **custo** cresce com a razão que separa as taxas: um par inteiro (48 000 → 192 000) cabe em 4 KiB com `b = 4`, enquanto 44 100 → 384 000 precisa de 1 280 fases e 1,3 MB. Por isso o número de fases é orçado e checado na construção, não descoberto na primeira alocação.

Não há modo "low": decodificação de saída para 16/44,1 com filtro pior que `balanced` reprovaria o `DSP-T-13`, e um modo que só existe para ser reprovado não é um modo.

**Latência.** O kernel é simétrico e centrado na posição de saída, então não há defasagem de grupo: o conteúdo no instante `τ` sai no instante `τ` da linha do tempo de saída. O que existe é **antecipação** — a amostra de saída em `τ` usa `half_eff` amostras *futuras* —, e é isso que vira atraso no fluxo: a amostra de saída `m` só pode ser emitida quando a entrada chegou até `m·r + half_eff`. Em frames de saída, a latência publicada é `half_eff·r`, e com `half_eff = half_base/min(1,r)` isso dá `half_base / min(fs_in, fs_out)` frames de saída — nos pares reais da tabela da §2, ~2,95 ms em `high` em qualquer direção. A instância arranca com `half_eff` amostras de zeros, o que alinha o índice de saída ao de entrada e faz o pico do impulso cair na posição ideal.

### 3. Alvo de qualidade, e como ele é medido

| Propriedade | Alvo | Como o spike mede |
|---|---|---|
| Ganho DC | ≤ 1 LSB de f32 em toda fase, fora das `half_eff` amostras de borda | entrada constante, saída constante; desvio máximo contra a entrada |
| Ripple de passband | ≤ 0,1 dB até a borda de passband do modo | sweep logarítmico, magnitude por Goertzel nos bins de teste |
| Rejeição de alias (decimar) | ≥ 80 dB (`balanced`), ≥ 110 dB (`high`) | tom acima do Nyquist de saída, energia no bin dobrado |
| Rejeição de imagem (interpolar) | ≥ 80 dB / ≥ 110 dB | tom na banda, energia na imagem refletida |
| Alinhamento temporal | pico do impulso a menos de 1 frame de saída da posição ideal | impulso unitário em índice conhecido |
| Bypass em taxa igual | **bit a bit** idêntico, latência 0 | `fs_in == fs_out` copia a entrada, sem filtrar |
| Latência | publicada = observada | quantos frames de entrada faltam para a primeira saída sair |

O alvo de ripple sai da margem de ±0,2 dB que a QA-005 já exige da resposta em frequência do grafo inteiro: o resampler não pode gastar mais da metade dela sozinho.

**Bypass em taxa igual é bypass de verdade, não filtro unitário.** Se `fs_in == fs_out`, o `Resampler` copia a entrada bit a bit e publica latência zero. Filtrar em taxa igual atenuaria o topo do espectro sem que ninguém tivesse pedido conversão, e quebraria a promessa do caminho transparente — o `DSP-T-13` e a `04-outputs-hires` chamam isso de "bypass transparente".

### 4. Conversão para inteiro: justificada à esquerda, e o dither tem uma condição só

Amostras inteiras vivem em container `i32` com os bits válidos nos **mais significativos**, que é o que o `WAVEFORMATEXTENSIBLE` define quando `wValidBitsPerSample < wBitsPerSample` e é a convenção que o "PCM24/32 packing" do deliverable nomeia. 24 bits ocupam `31..8`; 16 bits, `31..16`.

| De → para | O que acontece | Dither |
|---|---|---|
| mesma profundidade | cópia do valor, **bit a bit** | impossível — não há o que arredondar |
| maior profundidade | valor reinterpretado, bits baixos ficam zero | nunca |
| menor profundidade | deslocamento à direita com arredondamento | TPDF, se o chamador fornecer a fonte |

**Dither só ao reduzir precisão** vira uma propriedade do código, não uma recomendação: a conversão não toca na fonte de dither quando `dst >= src`, e o teste fixa que a saída continua bit a bit igual à do caminho sem dither **e que o estado do gerador não avançou**. Inverter essa ordem — ditherar ao subir de precisão — é o defeito que a frase normativa existe para impedir, porque adiciona ruído sem comprar resolução nenhuma.

TPDF é a diferença de duas uniformes independentes: em LSB inteiros, `u1 − u2` com `u1, u2 ∈ {0, 1}`, ou seja `{−1, 0, +1}` com probabilidades ¼, ½, ¼ — triangular, pico a pico 2 LSB, média zero, e em aritmética inteira. O gerador é determinístico e recebe semente do chamador, porque um dither que não se reproduz não se testa.

### 5. A forma da API, porque ela é o contrato

- **Uma instância por canal, um kernel compartilhado.** A biblioteca não tem o conceito de canal; ela processa `&[f32]` e o chamador mantém um `Resampler` por canal, todos apontando para a mesma `Kernel`. Mantém a fronteira da §2 da spec: nenhum tipo de domínio, nenhuma aritmética de mapa de canais.
- **Sem alocação depois da construção.** `max_block_frames` é declarado na construção e todo o buffer é reservado ali. O spike prova isso com um alocador global contando por thread, a mesma receita que a [evidência da TASK-E01-02](../evidence/PR/task-e01-02/README.md) registrou depois de um teste de alocação falhar por medir o processo inteiro.
- **Erro é dado, não `panic!`** (spec §6): taxa zero, bloco maior que o declarado, buffers de tamanho incompatível e tabela de fases acima do orçamento devolvem `Error`, e a instância continua utilizável.
- **Estado entre blocos é obrigatório, não opcional.** O teste que fixa isso processa o mesmo sinal em um bloco e em blocos de tamanho primo e compara as saídas amostra a amostra; um resampler que reinicia o filtro a cada bloco falha nesse teste imediatamente.
- **Saída menor que a capacidade não perde amostra.** Se o buffer de saída encher, o `process` para e devolve o que produziu; o resto sai na chamada seguinte, a partir do mesmo estado.

### 6. O que esta ADR deliberadamente não decide

- **Nome e repositório da biblioteca.** D-DSP-08 e a ADR-011 deixam isso com o mantenedor, e a TASK-E04-02 proíbe fechar essa lacuna no spike.
- **Posição no grafo e relação com o vitrola.** D-DSP-05 continua aberto; o spike prova o resampler isolado, e quem o liga na cadeia é a TASK-E04-03.
- **Noise shaping.** Fica para M5, como a referência já diz.
- **Conversão de byte order e escrita de container.** É do `decode`/`output`; aqui o container é `i32`.
- **Decimação em cascata.** Uma razão muito pequena alarga a janela por `1/r` e o custo cresce na mesma proporção; estágio múltiplo resolveria, e fica registrado como lacuna medida, não como pendência desta ADR.

## Lacunas

| Lacuna | Quem cobre | Estado |
|---|---|---|
| Nome, repositório e release da biblioteca | Mantenedor (D-DSP-08) | Aberto; o spike vive em `vynamp-dsp` sem criar o sexto repositório |
| Modo de qualidade por dispositivo e a troca em tempo real | TASK-E04-03 (grafo) | Sem plano |
| Noise shaping | M5 | Fora do escopo de M0 |
| Decimação em cascata para razões muito pequenas | Esta ADR registra o custo de `1/r`; a cascata é otimização | Medido no spike |
| Razões cujo denominador reduzido estoura o orçamento da tabela | Esta ADR: a construção devolve erro em vez de alocar sem limite | Medido no spike |
| Validação acústica e de hardware | TASK-E04-10 | Fora do escopo: nenhum DAC é tocado |

## Alternativas

- **Aceitar a rejeição que os taps atuais dão e escrever o alvo depois.** Mais rápido, e é o que se faz quando não há norma. Rejeitado: o `DSP-T-13` precisa de um número antes para poder reprovar, e um alvo escrito a partir do resultado nunca reprova nada.
- **Um único modo.** Menos API, menos teste. Rejeitado porque a referência nomeia balanced/high e porque latência e CPU são justamente o que distingue os dois casos de uso.
- **Kernel avaliado em tempo de execução, sem tabela.** Nenhuma memória de coeficiente, e a forma da API não mudaria. Rejeitado pelo custo transcendental no caminho quente, que inviabiliza um orçamento de CPU realista.
- **Tabela de subfases com interpolação linear.** Serve razões arbitrárias em memória fixa. Rejeitado porque o acumulador inteiro entrega a tabela exata pelo mesmo esforço; o preço é o orçamento de fases, que é explícito e checado na construção.
- **Filtrar também em taxa igual.** Uma implementação só, sem ramo. Rejeitado: atenuaria o topo do espectro sem conversão pedida e quebraria o caminho transparente.
- **Dither obrigatório em toda conversão para inteiro.** Mais simples de invocar. Rejeitado pela norma: ditherar sem reduzir precisão adiciona ruído e não compra resolução.

## Consequências

- `DSP-T-13` passa a ter um alvo verificável, e a `04-outputs-hires` deixa de citar um documento que não existia.
- O grafo ganha `half_base / min(fs_in, fs_out)` frames de latência por instância — ~2,95 ms em `high`, ~1,1 ms em `balanced`, independente da direção da conversão. Isso precisa entrar na compensação de clock da TASK-E04-03: latência de resampler que não é compensada vira dessincronia de medidor e de posição.
- A `Kernel` custa `b · taps · 4` bytes, uma vez por configuração e compartilhada entre canais: 4 KiB em 48 000 → 192 000, 163 KiB em 44 100 → 48 000, 1,3 MB em 44 100 → 384 000. Não é memória de caminho quente, mas é memória que o perfil de um octeto vai mostrar, e por isso o número de fases é orçado na construção.
- O custo medido dos dois modos é a base do orçamento de CPU do grafo em M2, junto do `p99.9` da callback que a TASK-E01-02 já mediu.
- Duas fontes de verdade em potencial: esta ADR e a tabela de modos da spec de `vynamp-dsp` §3. A spec cita esta ADR e não repete os números; se um número mudar, muda aqui.
- Se a aprovação do mantenedor exigir um modo adicional ou outra família de filtro, o custo é reescrever esta ADR e o módulo de resampling — o resto do pacote (packing, dither, conversão inteira) não depende da família de filtro.
