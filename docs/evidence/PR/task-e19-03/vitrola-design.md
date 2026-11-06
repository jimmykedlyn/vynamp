# Design aprovado do vitrola — cópia literal

Cópia **literal** do design aprovado, para que o conteúdo sobreviva ao arquivamento do repositório de origem. Não editar: a versão normativa é [docs/audio/10-vitrola.md](../../../audio/10-vitrola.md) §8, que incorpora este texto.

- Origem: `jimmykedlyn/vitrola`, arquivo `docs/superpowers/specs/2026-10-23-vitrola-design.md`
- Commit da cópia: `75c4ca2` — o mais recente da branch local `develop`, **um commit à frente** do publicado (`3d44c68`). A diferença são três ocorrências de `agulha` renomeadas para `vynamp-decode`; o publicado ainda diz `agulha`.
- Data do design: 2026-10-23 · Status: aprovado pelo mantenedor, que delegou a revisão final a Claude.
- Por que isto existe: o mantenedor decidiu em 2026-11-05 que vinil e válvula são módulos DSP internos do app ([ADR-014](../../../architecture/adr/ADR-014-public-github-actions-and-in-app-dsp.md)). O repositório de origem é arquivado; esta cópia preserva o texto integral, inclusive o que a incorporação em §8 reorganizou em subseções.

---

# vitrola — efeitos de vinil e amplificador valvulado com inteligência (design)

Data: 2026-10-23 · Autor: Claude · Status: aprovado (mantenedor delegou a revisão final a Claude, 2026-10-23)

## 1. Intenção

Um efeito "inusitado" para o DSP do Vynamp que encante quem ama vinil: não um ruído colado por cima, mas a **simulação de um disco real tocando aquela música** num **amplificador valvulado**. O efeito deve ter **inteligência** — reagir à música, à posição na faixa/álbum e ao histórico — e **nunca ser estático**. Biblioteca própria, sem terceiros, aberta no futuro, como o `vynamp-decode`.

"vitrola" (o aparelho inteiro: toca-discos + válvula) — nome escolhido por Claude; trocável.

## 2. Comportamentos aprovados pelo mantenedor

**Vinil**
- Posição na faixa → raio do disco: início do lado mais aberto; perto do miolo perda leve de agudos/definição e mais distorção (inner-groove).
- Chiado/estalos que respeitam a música: aparecem em trechos quietos e silêncios entre faixas, mascarados nas partes altas; nunca padrão repetido.
- Rotação física: wow ligado à volta do prato (33⅓/45 rpm) + flutter; disco empenado gera oscilação periódica.
- Rituais: agulha descendo no começo do álbum; sulco final em loop ao acabar; opcional virada lado A→B no ponto certo.
- Envelhecimento: cada faixa guarda quantas vezes tocou; desgaste cresce devagar; "comprar disco novo" zera.

**Válvula**
- Aquecimento: alguns segundos até o som ficar cheio.
- Saturação dependente da dinâmica: suave limpo; picos ganham harmônicos pares e compressão leve.
- Sag da fonte em batidas/graves fortes.
- Perfis genéricos: "pré quente", "push-pull clássico", "single-ended" (sem marcas).

**Controles** (decisão de Claude): um botão principal de intensidade que move tudo de forma coerente + painel avançado por elemento. Padrão sutil.

## 3. Não-objetivos / anti-objetivos
- **Somente simulação de áudio** (decisão do mantenedor): nenhuma interface visual, desenho de toca-discos, animação de prato/braço ou skin. A biblioteca não tem UI; o Vynamp expõe apenas os controles (botão principal + painel avançado) com os componentes de UI que já usa. "vitrola" é só o nome.
- Não é emulação circuito-a-circuito de equipamento de marca; sem nomes de marcas.
- Anti: loop de ruído gravado, padrão perceptível, exagero caricato no padrão, cliques ao ligar/desligar, alterar o caminho bit-perfect ou DSD nativo (efeito desativa automaticamente), alocação ou lock no callback de áudio.

## 4. Arquitetura (decisões de Claude)

- Rust, só `std`, `#![forbid(unsafe_code)]`. Processamento interno em f64 para estados/filtros, f32/f64 de entrada/saída.
- Trait neutra própria `Processor` (prepare(sample_rate, max_block, channels) → process(&mut block, &Context) → reset), sem alocação em `process`. O Vynamp escreve um adaptador para o grafo DSP da TASK-E04-03.
- `Context` vindo do host por bloco: posição/duração da faixa, índice da faixa no álbum, total de faixas e duração do álbum, eventos de transporte (start, stop, pause, seek, fim de álbum), contagem de reproduções da faixa.
- Estado de envelhecimento: a biblioteca expõe `WearState` serializável (bytes próprios estáveis/versionados); o Vynamp persiste por faixa.
- Aleatoriedade: PRNG próprio (PCG/xoshiro) com semente por faixa+reprodução → reprodutível em testes, variado na prática.
- Não-linearidades com oversampling próprio (polifásico 2×–8× conforme taxa) para evitar aliasing.
- Suavização de todos os parâmetros; bypass com crossfade (sem cliques).
- Taxas 44,1 k–384 kHz; mono/estéreo (multicanal: vinil desativado, válvula por canal).

### Módulos
- `vinyl::radius` — mapeia posição→raio (lado A/B pelo índice da faixa e duração acumulada), controla perda de HF, distorção de traçado e crosstalk.
- `vinyl::surface` — ruído de superfície, crackle e pops com densidade guiada por seguidor de loudness (mascaramento), desgaste e intensidade.
- `vinyl::transport` — wow (frequência da volta do prato, empeno periódico) + flutter, via linha de atraso fracionária interpolada.
- `vinyl::rituals` — síntese procedural (não samples gravados) de agulha descendo, run-out loop, virada de lado.
- `vinyl::cartridge` — ressonância/roll-off de cápsula e rumble leve.
- `tube::warmup`, `tube::preamp` (triodo assimétrico, bias dinâmico → harmônicos pares), `tube::power` (push-pull vs single-ended), `tube::sag`, `tube::transformer` (roll-off LF/HF, saturação leve).
- `macro` — mapa do botão principal para todos os parâmetros.

## 5. Verificação
- Determinismo com semente fixa (golden de métricas, não de waveform frágil).
- Propriedades medidas: espectro de harmônicos (pares dominam em perfis triodo/SE), saturação crescente com nível, aliasing abaixo de limiar, densidade de crackle inversamente proporcional ao loudness, período de wow = 60/rpm s, ausência de denormais, sem alocação em `process` (contador por thread como no E01-02), crossfade sem descontinuidade.
- Orçamento de CPU: meta < 5% de um núcleo moderno a 96 kHz estéreo com tudo ligado (a confirmar por benchmark).
- Escuta: renderizar amostras A/B para o mantenedor avaliar (decisão subjetiva dele).

## 6. Dependências e ordem
Depende do grafo DSP do Vynamp (TASK-E04-03) para integrar; a biblioteca em si pode ser construída e testada antes, offline. Ordem geral: `vynamp-decode` Camada 1 → E04-03 → `vitrola`.

## 7. Restrições
Repositório privado `jimmykedlyn/vitrola`; licença MIT OR Apache-2.0; mesmas regras de entrega; documentação: README, guia de som (o que cada elemento faz, com exemplos de áudio), rustdoc, CONTRIBUTING, SECURITY, CHANGELOG.

## 8. Decisões complementares (de Claude, após revisão do construtor)

1. **Contrato com o host**: `prepare` recebe taxa, bloco máximo e canais; fronteira em f32 intercalado ou planar (ambos); posição em samples. Posição no grafo do Vynamp: depois do resampler e do EQ, **antes** do volume do usuário — a saturação depende do nível da gravação, não do volume; opção avançada "acoplar ao volume" para quem quer a válvula "abrindo" com o volume alto. `Context` é opcional por bloco: sem ele (rádio/stream), lado virtual de 20 min pelo tempo de sessão e rituais desligados. Transições gapless chegam como evento `track_changed` no mesmo bloco.
2. **Lados A/B**: com álbum tocando em ordem, a biblioteca divide no limite de faixa mais próximo da metade da duração, limitado a ~22 min por lado (como LPs reais; álbuns longos viram 2 discos/4 lados). Em playlist/shuffle/faixa avulsa: cada faixa ocupa um lado virtual próprio (início = borda, fim proporcional à duração).
3. **Rituais alteram a duração**: agulha descendo (~1,5 s) antes da 1ª faixa do lado e sulco final em loop após a última (até o usuário parar, ou 30 s e fade; configurável). API `pre_roll()`/`tail()` gera esse áudio; o host insere no transporte. Virada de lado: tail do lado A + pre_roll do lado B. Desligável.
4. **Inteligência**: apenas seguidores em tempo real (loudness com ponderação perceptiva, detector de transientes, energia de graves para o sag); sem pré-análise offline.
5. **Desgaste**: por disco (álbum). "Comprar disco novo" zera aquele álbum. Cada reprodução desgasta a região do sulco daquela faixa. **Defeitos permanentes** (riscos) têm posição fixa por disco e voltam no mesmo ponto a cada audição, como num disco real; **poeira/crackle** variam a cada reprodução (semente = disco + contagem). Assim há repetição só onde um disco real repetiria.
6. **Versões**: só os bytes do `WearState` são estáveis e versionados (com migração); o som pode evoluir entre versões; goldens validam métricas, não waveform.
7. **CPU**: < 5% de um núcleo a 96 kHz estéreo é meta, não gate; nível de qualidade automático reduz oversampling em taxas altas (ex.: 8× a 44,1/48 k, 2× a ≥ 176,4 k).
8. Toolchain/plataformas iguais ao `vynamp-decode`; harness de benchmark e contador de alocação próprios.

## 9. Deixado ao construtor
Modelos matemáticos exatos (waveshaper, filtros, geradores de crackle), coeficientes, curvas do macro, layout dos parâmetros avançados, formato interno do WearState.
