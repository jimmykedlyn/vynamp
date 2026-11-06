# vitrola — Vinyl Simulator e Tube Amplifier Simulator no DSP

Versão: 0.8.0 · Data: 2026-11-05 · Estado: **design incorporado; integração não implementada**. O design aprovado em 2026-10-23 era de uma biblioteca externa; passou a ser **módulo DSP interno do app** e está integralmente em [§8](#8-design-aprovado-incorporado-em-2026-11-05) ([ADR-014](../architecture/adr/ADR-014-public-github-actions-and-in-app-dsp.md)). Não há repositório externo. Nada foi implementado em nativo.

## 1. Fonte da verdade e escopo

- A definição acordada está **integralmente neste documento**, em [§8 Design aprovado](#8-design-aprovado-incorporado-em-2026-11-05). **Não existe mais biblioteca nem repositório externo**: o mantenedor decidiu, em 2026-11-05, que vinil e válvula são **módulos DSP internos do app** ([ADR-014](../architecture/adr/ADR-014-public-github-actions-and-in-app-dsp.md)). O repositório antigo foi **arquivado**, não aberto.
- Consequência: antes este documento **integrava por referência** um design que vivia fora, e o que valesse lá era a regra. Agora ele é a **fonte única** — decisões, nomes, parâmetros, "inteligência" e restrições valem como estão aqui em §8. As seções 2 a 7 descrevem a integração, a UX hospedeira, os testes e as divergências.
- Dois efeitos com identidade própria: **Vinyl Simulator** (simulação de um disco real tocando aquela faixa) e **Tube Amplifier Simulator** (amplificador valvulado). Os dois são **opcionais, colorativos e intencionais**, com bypass e intensidade gradual. Não são um equalizador, nem um exciter genérico, nem processamento obrigatório.
- Anti-objetivos herdados: sem UI visual de toca-discos/prato/braço/válvulas; sem emulação circuito a circuito nem nomes de marcas; sem loop de ruído gravado, padrão perceptível ou exagero caricato no padrão; sem cliques ao ligar/desligar; sem alterar o caminho bit-perfect ou DSD nativo (o efeito se desativa sozinho); sem alocação ou lock na callback.

## 2. Comportamentos aprovados (resumo do design)

| Elemento | Comportamento acordado | Módulo vitrola |
|---|---|---|
| Raio do disco | a posição na faixa/lado vira raio. Perto do miolo há perda leve de agudos/definição e mais distorção (inner-groove); o crosstalk depende do raio | `vinyl::radius` |
| Superfície | chiado/crackle/pops guiados por um seguidor de loudness: aparecem em trechos quietos e nos silêncios entre faixas e são mascarados nas partes altas. Nunca formam padrão repetido | `vinyl::surface` |
| Rotação | wow ligado à volta do prato (33⅓/45 rpm, período = 60/rpm s) + flutter + oscilação periódica de disco empenado, via linha de atraso fracionária interpolada | `vinyl::transport` |
| Rituais | agulha descendo (~1,5 s) antes da 1ª faixa do lado; sulco final em loop (até parar, ou 30 s + fade); virada A→B opcional. Síntese procedural, sem samples gravados. **Alteram a duração** (`pre_roll()`/`tail()`) | `vinyl::rituals` |
| Cápsula | ressonância/roll-off e rumble leve | `vinyl::cartridge` |
| Envelhecimento | desgaste por disco (álbum): cada reprodução desgasta a região da faixa. Riscos permanentes voltam no mesmo ponto; poeira/crackle variam a cada audição. "Comprar disco novo" zera aquele álbum | `WearState` |
| Lados A/B | álbum em ordem: corte no limite de faixa mais próximo da metade, com ~22 min por lado (álbuns longos viram 2 discos/4 lados). Playlist/shuffle/avulsa: um lado virtual por faixa. Sem `Context` (rádio/stream): lado virtual de 20 min e rituais desligados | `vinyl::radius` |
| Aquecimento | alguns segundos até o som ficar cheio | `tube::warmup` |
| Saturação | depende da dinâmica: o sinal suave fica limpo; picos ganham harmônicos pares e compressão leve (triodo assimétrico, bias dinâmico) | `tube::preamp`, `tube::power` |
| Sag | a fonte "cede" em batidas e graves fortes | `tube::sag` |
| Transformador | roll-off LF/HF e saturação leve | `tube::transformer` |
| Perfis | "pré quente", "push-pull clássico", "single-ended" (genéricos, sem marcas) | `tube::power` |
| Controles | **um botão principal de intensidade** move tudo de forma coerente (`macro`), mais um **painel avançado por elemento**. Padrão **sutil** | `macro` |
| Inteligência | só seguidores em tempo real (loudness com ponderação perceptiva, transientes, energia de graves); sem pré-análise offline | — |
| Qualidade | oversampling polifásico próprio de 2×–8× conforme a taxa, automático (ex.: 8× a 44,1/48 kHz, 2× a ≥ 176,4 kHz); smoothing de tudo; bypass com crossfade | — |
| Taxas e canais | 44,1–384 kHz; mono/estéreo; em multicanal o vinil fica desligado e a válvula atua por canal | — |
| Opção avançada | "acoplar ao volume": a válvula "abre" com volume alto | `macro` |

## 3. Integração no grafo do Vynamp

- **Adaptador** (planejado após TASK-E04-03): a trait `Processor` do vitrola (`prepare(sample_rate, max_block, channels)` → `process(&mut block, &Context)` → `reset`) é encaixada no contrato de nó do [DSP](03-dsp.md#contrato-conceitual-dos-nós). A fronteira aceita f32 intercalado ou planar; a posição chega em samples.
- **`Context` por bloco** (fornecido pelo host): posição/duração da faixa, índice da faixa no álbum, total de faixas, duração do álbum, eventos de transporte (start, stop, pause, seek, fim de álbum, `track_changed` gapless no mesmo bloco) e contagem de reproduções. É opcional.
- **Posição** (design §8.8, decisão 1): depois do EQ e **antes do volume do usuário** (ganho master). Na cadeia real fica no segmento por stream, depois da correção de fone, antes de dinâmica/imagem/criativos e antes de mixer/master/resampler/proteção final ([diagrama](03-dsp.md#cadeia-real-de-processamento)). Ordem dentro do segmento: **proposta**. Reordenação livre não é prometida.
- **Conflito aberto D-DSP-05**: o vitrola diz "depois do resampler", mas o [contrato do sinal](07-dsp-signal-contract.md) tem um único resampler final, depois do master. A decisão do vitrola fica **preservada** como requisito e o contrato 07 **não** foi alterado. Opções: (A) o vitrola roda na taxa do stream, já suportada de 44,1 a 384 kHz, com o oversampling automático cobrindo o aliasing; (B) um resampler por stream antes dos efeitos, que exige emendar o 07 por decisão. Isso também resolveria a mistura de fontes de taxas diferentes no mixer, outro ponto **dependente de investigação**. Fontes abaixo de 44,1 kHz ficam fora da faixa do vitrola: o efeito fica indisponível com motivo, salvo decisão (B).
- **Ponto aberto D-DSP-06, crossfade**: a referência pressupõe um "disco" contínuo, com gapless via `track_changed`, e não define o crossfade. Candidato **a validar**: com o vitrola ligado, o crossfade fica desativado (com aviso), ou só a válvula continua por stream.
- **Rituais e transporte**: `pre_roll()`/`tail()` são inseridos pelo host no transporte. A duração apresentada e a posição consideram esse áudio extra. Rituais desligáveis.
- **WearState**: bytes estáveis e versionados, com migração. O Vynamp persiste **por disco (álbum)** na biblioteca local. "Comprar disco novo" exige confirmação. O WearState não faz parte de presets nem de exportação de preset (proposto).
- **Bit-perfect / DSD / transparente**: desativação automática com o motivo visível ("desativado: caminho bit-perfect / DSD nativo / transparente").
- **Uso isolado ou simultâneo**: Vinyl e Tube são **módulos separados** dentro da categoria Vitrola, cada um com `intensity` e `bypass` próprios, e o painel DSP expõe o toggle de bypass por módulo ([referência do MVP](../design/06-mvp-reference.md) `mvp/dsp-catalog.js`, `mvp/dsp-workspace.js`). Acima deles há o macro "Intensidade geral (Vinyl + Tube)", que move as duas intensidades juntas e declara que "condição, preset e bypass continuam independentes", e um botão único da categoria. Um módulo só entra na cadeia com o enable do módulo **e** o botão da Vitrola ligados. O MVP **não** tem contrato de "intensidade 0 = bypass da seção" nem API de host para isso: um enable separado por seção na UI nativa seria decisão nova de produto, não herança do MVP ([D-MVP-02](../product/05-decisions.md#decisão-o-mvp-é-a-fonte-única-da-ui-2026-11-05)).
- **Headroom e picos**: medidores pré e pós-vitrola. Estalos e pops sintéticos têm pico limitado (limite numérico a definir no construtor; teste VIT-T-04) e não escondem clipping, que continua indicado. A proteção final true-peak continua fixa.

## 4. Itens do pedido v1.3 versus referência

O pedido traz parâmetros "candidatos a validar". A regra: o que o design aprovado ([§8](#8-design-aprovado-incorporado-em-2026-11-05)) define prevalece; o que ele não define fica **a validar**; conflitos ficam registrados abaixo.

| Item do pedido | Situação | Tratamento |
|---|---|---|
| Caráter tonal, saturação, separação estéreo | definido (radius, cartridge, crosstalk por raio) | manter |
| Hiss, crackle, clicks, rumble, desgaste | definido (surface, cartridge, WearState) | manter; controles por tipo no painel avançado (layout deixado ao construtor, §9 da referência) |
| "Ruídos desligados ou discretos por padrão" | compatível: o padrão é **sutil** | manter "sutil" |
| "Evitar ruído em pausa/silêncio" | **conflito**: a referência põe chiado e crackle **nos silêncios entre faixas**, e o sulco final em loop após o fim | **manter a referência**. Pausa → silêncio com fade curto (agulha levantada): **a validar**, porque a referência só lista `pause` como evento. Seek → `reset` sem estalo |
| Wow & flutter | definido | manter |
| 33⅓ / 45 rpm | definido | manter |
| 78 rpm | não definido | **a validar** (só como perfil de característica; nunca altera a velocidade digital) |
| RIAA | não definido | esclarecimento: a equalização RIAA **não é reaplicada** a arquivos digitais. Um filtro "inspirado em RIAA" só existiria como emulação justificada, **a validar** |
| Controles Amount/Tone/Noise/Crackle/Clicks/Rumble/Wow/Flutter/Stereo Character | Amount = macro de intensidade (definido); os demais correspondem a elementos do painel avançado | nomes finais e layout: construtor/UX, **a validar** |
| Random seed do usuário | **diverge**: a semente é automática (faixa+reprodução / disco+contagem) e fixa só nos testes | manter automática; seed exposta ao usuário: **a validar** |
| Ganho de saída compensado | não definido explicitamente | **a validar** (recomendado para A/B justo) |
| Presets Clean/Warm/Vintage Vinyl (pedido) | **diverge do pedido**: o MVP implementa três presets de fábrica do Vitrola — **Sutil**, **Quente** e **Rústico** — exibidos em "Presets do módulo" quando a categoria é Vitrola e aplicados por "Carregar" (Sutil → Vinyl "Bem conservado" / Tube "Limpo"; Quente → "Usado" / "Quente"; Rústico → "Gasto" / "Saturado") | vale o MVP ([D-MVP-02](../product/05-decisions.md#decisão-o-mvp-é-a-fonte-única-da-ui-2026-11-05)); a nomenclatura em inglês "Subtle/Warm/Rustic" e a marcação `[proposal]` não existem nele |
| Drive, Warmth/Tone | correspondem a saturação/macro e transformador | **a validar** como controles nomeados |
| Bias | **diverge**: na referência o bias é **dinâmico/interno** | manter interno; controle de usuário **a validar** |
| Mix dry/wet | não definido | **a validar** |
| Oversampling/Quality manual | **diverge**: o nível de qualidade é **automático** pela taxa | manter automático; override avançado **a validar** |
| Hum/noise da válvula | não definido | **a validar**; se existir, fica independente e desligado por padrão |
| Sag, assimetria, perfis | definidos | manter |
| Presets Clean/Warm/Driven Tube | não definidos; os perfis acordados são pré quente / push-pull clássico / single-ended | perfis mantidos; presets **a validar** |
| Ordem "correção tonal → emulação → controle de picos" | compatível com §8.8 | manter (ver D-DSP-05) |

## 5. UX hospedeira

- Local: Preferências › DSP tem o botão "Abrir DSP e Vitrola…", que abre o workspace DSP; lá a Vitrola é **categoria própria** ("Vitrola"), com Vinyl Simulator e Tube Amplifier. Atalhos: "Vitrola…" na barra do equalizador e "Vitrola" na faixa de atalhos de áudio junto ao instrumento. O MVP **não** tem atalho para a Vitrola no inspector da cadeia (Fluxo modular); um atalho ali seria decisão nova ([D-MVP-02](../product/05-decisions.md#decisão-o-mvp-é-a-fonte-única-da-ui-2026-11-05)). Só componentes padrão do Vynamp. Cards separados para Vinil e Válvula, com bypass próprio, intensidade, indicador "coloração intencional", presets, A/B, restaurar padrões e "guia de som".
- Nível 1: botão principal "Intensidade do vitrola" de 0–100% (padrão sutil), On/Off com crossfade e estado (Ativo / Desativado automaticamente com motivo / Aquecendo… / Pronto).
- Nível 2: painel avançado por elemento (lado A/B e disco atual em texto, rpm, superfície, rituais com aviso "altera a duração", cápsula, barra de desgaste somente leitura + "Comprar disco novo", perfil da válvula, sag, transformador, "acoplar ao volume").
- **Proibido**: ilustração de toca-discos, agulha, disco girando, braço ou válvulas animadas. Um ícone genérico pequeno de categoria é permitido.
- A/B com loudness igualado (**proposto**): o efeito não deve parecer "melhor" só por estar mais alto.
- **Atalho Vitrola (DSP-UX-02, v1.4).** O atalho "Vitrola" junto à área do instrumento e a família "Vitrola" do painel são **entradas para a área existente** (Vinil e Válvula, com configurações, presets, prioridades e bypass independentes). Não fundem os dois simuladores nem duplicam o estado dos parâmetros entre acessos. No protótipo o destino inicial é o vinil. Contrato: [painel DSP §8](../design/05-dsp-panel.md#8-acesso-e-controles-dsp-ux).

### Estado no protótipo (rev. 15, relatado) × nativo

Evidência relatada da referência externa rev. 14/15, não verificada neste repositório. **Especificado**: este documento e a referência vitrola. **Implementado no protótipo (relatado)**: a configuração completa do catálogo para Vinyl e Tube, **sem processamento** desses modelos no grafo modular; o preset de cadeia "Vinyl" carrega essa configuração indisponível para processamento, sem simular som de vinil. **Validado no protótipo (relatado)**: nenhuma validação sonora; a referência diz que esses modelos dependem de adaptadores e algoritmos próprios e de validação acústica ainda não feitos. **Nativo**: nada. A Saturação suave do protótipo (tanh, oversampling 2×) **não é** o Tube Amplifier. Fases, prioridade e a pendência de TASK (seção 6) não mudam.

## 6. Roadmap (status herdado)

- Ordem acordada (design §8.6; STATE.md): `vynamp-decode` Camada 1 → **TASK-E04-03** (grafo DSP, M2) → vitrola. O módulo pode ser construído e testado offline antes disso. O marco da integração é **M3, depois de E04-03** ([D-DSP-07](../product/05-decisions.md#decisões-dsp-e-equalizadores-2026-10-25)).
- O `ROADMAP-rodadas.md` da raiz lista só TASKs e não tem uma linha vitrola. A antiga pendência "criar uma TASK de integração" **está resolvida**: [TASK-E19-07](../../tasks/items/TASK-E19-07.md) integra o módulo no grafo do player (M3, depois de E04-03 e de [TASK-E19-03](../../tasks/items/TASK-E19-03.md)). A prioridade não foi deslocada.
- CPU: meta < 5% de um núcleo moderno a 96 kHz estéreo com tudo ligado. É meta, não gate; confirmar por benchmark.

## 7. Testes de integração planejados

Os testes do próprio módulo (determinismo com semente, espectro de harmônicos, aliasing, densidade de crackle × loudness, período de wow, denormais, alocação zero, crossfade) estão no design incorporado, [§8.5](#85-verificação-original-5). No Vynamp (cenário QA-033, [estratégia](../quality/01-test-strategy.md)):

| ID | Objetivo | Método | Evidência | Critério |
|---|---|---|---|---|
| VIT-T-01 | Bypass transparente | vinil e válvula ligados → desligados; comparar com o sinal seco após o crossfade | diff de amostras | idêntico ao seco depois do fim do crossfade; sem degrau |
| VIT-T-02 | Desativação automática | entrar em transparente/DSD/DoP com o vitrola ligado | log de estado + UI | vitrola em bypass com motivo; selo correto |
| VIT-T-03 | Multicanal | fonte 5.1 | estado + análise por canal | vinil indisponível com aviso; válvula por canal |
| VIT-T-04 | Picos de estalos | crackle/pops no máximo sobre um silêncio digital | sample/true-peak pós-vitrola | pico ≤ limite declarado; clipping nunca escondido |
| VIT-T-05 | Pausa/seek/fim de álbum | pausar, seek rápido, fim do lado e rituais | captura + log de transporte | sem estalo no seek; tail/pre_roll inseridos e contabilizados na duração; comportamento em pausa conforme decisão |
| VIT-T-06 | Persistência do WearState | tocar, reiniciar o app, "Comprar disco novo" | bytes persistidos por álbum | desgaste preservado; reset só do álbum pedido |
| VIT-T-07 | Vinil + válvula + EQ +15 dB | combinação extrema | medidores por estágio + QA-027 | clipping cumulativo indicado; teto final respeitado |
| VIT-T-08 | Aliasing da válvula | senos 1/5/10/15 kHz, níveis −20/−6/0 dBFS, 44,1–192 kHz | espectro | aliasing abaixo do limiar definido na biblioteca |
| VIT-T-09 | CPU/latência | 96 kHz estéreo com tudo ligado | bench | publicar o número; meta < 5% de um núcleo |
| VIT-T-10 | A/B com loudness igualado | comparar on/off com a compensação | LUFS integrado das duas versões | diferença dentro da tolerância a definir |

## 8. Design aprovado (incorporado em 2026-11-05)

**Origem:** documento datado de 2026-10-23, aprovado pelo mantenedor, que delegou a revisão final a Claude. Vivia num repositório próprio; o mantenedor decidiu em 2026-11-05 que vinil e válvula são **recursos do DSP dentro da aplicação**. O texto abaixo é o design **integral, preservado por incorporação** — não reescrito nem reinterpretado. As subseções 8.1 a 8.9 são as seções 1 a 9 originais. Duas alterações, e só duas: os nomes das bibliotecas citadas foram atualizados para `vynamp-decode` (nome atual) e a restrição "repositório privado" em 8.7 foi corrigida, porque não há mais repositório. A cópia literal do original está preservada em [evidência](../evidence/PR/task-e19-03/vitrola-design.md).

### 8.1 Intenção (original §1)

Um efeito "inusitado" para o DSP do Vynamp que encante quem ama vinil: não um ruído colado por cima, mas a **simulação de um disco real tocando aquela música** num **amplificador valvulado**. O efeito deve ter **inteligência** — reagir à música, à posição na faixa/álbum e ao histórico — e **nunca ser estático**. Biblioteca própria, sem terceiros, aberta no futuro, como o `vynamp-decode`. (Nota pós-2026-11-05: a "biblioteca própria" virou módulo interno; a intenção de som, não a embalagem, é o que se preserva.)

"vitrola" (o aparelho inteiro: toca-discos + válvula) — nome escolhido por Claude; trocável.

### 8.2 Comportamentos aprovados pelo mantenedor (original §2)

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

### 8.3 Não-objetivos / anti-objetivos (original §3)

- **Somente simulação de áudio** (decisão do mantenedor): nenhuma interface visual, desenho de toca-discos, animação de prato/braço ou skin. O módulo não tem UI; o Vynamp expõe apenas os controles (botão principal + painel avançado) com os componentes de UI que já usa. "vitrola" é só o nome.
- Não é emulação circuito-a-circuito de equipamento de marca; sem nomes de marcas.
- Anti: loop de ruído gravado, padrão perceptível, exagero caricato no padrão, cliques ao ligar/desligar, alterar o caminho bit-perfect ou DSD nativo (efeito desativa automaticamente), alocação ou lock no callback de áudio.

### 8.4 Arquitetura (original §4)

- Rust, só `std`, `#![forbid(unsafe_code)]`. Processamento interno em f64 para estados/filtros, f32/f64 de entrada/saída.
- Trait neutra própria `Processor` (prepare(sample_rate, max_block, channels) → process(&mut block, &Context) → reset), sem alocação em `process`. O Vynamp escreve um adaptador para o grafo DSP da TASK-E04-03.
- `Context` vindo do host por bloco: posição/duração da faixa, índice da faixa no álbum, total de faixas e duração do álbum, eventos de transporte (start, stop, pause, seek, fim de álbum), contagem de reproduções da faixa.
- Estado de envelhecimento: o módulo expõe `WearState` serializável (bytes próprios estáveis/versionados); o Vynamp persiste por faixa.
- Aleatoriedade: PRNG próprio (PCG/xoshiro) com semente por faixa+reprodução → reprodutível em testes, variado na prática.
- Não-linearidades com oversampling próprio (polifásico 2×–8× conforme taxa) para evitar aliasing.
- Suavização de todos os parâmetros; bypass com crossfade (sem cliques).
- Taxas 44,1 k–384 kHz; mono/estéreo (multicanal: vinil desativado, válvula por canal).

**Módulos**
- `vinyl::radius` — mapeia posição→raio (lado A/B pelo índice da faixa e duração acumulada), controla perda de HF, distorção de traçado e crosstalk.
- `vinyl::surface` — ruído de superfície, crackle e pops com densidade guiada por seguidor de loudness (mascaramento), desgaste e intensidade.
- `vinyl::transport` — wow (frequência da volta do prato, empeno periódico) + flutter, via linha de atraso fracionária interpolada.
- `vinyl::rituals` — síntese procedural (não samples gravados) de agulha descendo, run-out loop, virada de lado.
- `vinyl::cartridge` — ressonância/roll-off de cápsula e rumble leve.
- `tube::warmup`, `tube::preamp` (triodo assimétrico, bias dinâmico → harmônicos pares), `tube::power` (push-pull vs single-ended), `tube::sag`, `tube::transformer` (roll-off LF/HF, saturação leve).
- `macro` — mapa do botão principal para todos os parâmetros.

### 8.5 Verificação (original §5)

- Determinismo com semente fixa (golden de métricas, não de waveform frágil).
- Propriedades medidas: espectro de harmônicos (pares dominam em perfis triodo/SE), saturação crescente com nível, aliasing abaixo de limiar, densidade de crackle inversamente proporcional ao loudness, período de wow = 60/rpm s, ausência de denormais, sem alocação em `process` (contador por thread como no E01-02), crossfade sem descontinuidade.
- Orçamento de CPU: meta < 5% de um núcleo moderno a 96 kHz estéreo com tudo ligado (a confirmar por benchmark).
- Escuta: renderizar amostras A/B para o mantenedor avaliar (decisão subjetiva dele).

### 8.6 Dependências e ordem (original §6)

Depende do grafo DSP do Vynamp (TASK-E04-03) para integrar; o módulo em si pode ser construído e testado antes, offline. Ordem geral: `vynamp-decode` Camada 1 → E04-03 → o módulo vitrola.

### 8.7 Restrições (original §7)

Licença MIT OR Apache-2.0; mesmas regras de entrega; documentação: README, guia de som (o que cada elemento faz, com exemplos de áudio), rustdoc, CONTRIBUTING, SECURITY, CHANGELOG. **Revisado em 2026-11-05:** o design original exigia "repositório privado". Não há mais repositório: o módulo vive no workspace do app ([ADR-014](../architecture/adr/ADR-014-public-github-actions-and-in-app-dsp.md)).

### 8.8 Decisões complementares (original §8)

1. **Contrato com o host**: `prepare` recebe taxa, bloco máximo e canais; fronteira em f32 intercalado ou planar (ambos); posição em samples. Posição no grafo do Vynamp: depois do resampler e do EQ, **antes** do volume do usuário — a saturação depende do nível da gravação, não do volume; opção avançada "acoplar ao volume" para quem quer a válvula "abrindo" com o volume alto. `Context` é opcional por bloco: sem ele (rádio/stream), lado virtual de 20 min pelo tempo de sessão e rituais desligados. Transições gapless chegam como evento `track_changed` no mesmo bloco.
2. **Lados A/B**: com álbum tocando em ordem, o módulo divide no limite de faixa mais próximo da metade da duração, limitado a ~22 min por lado (como LPs reais; álbuns longos viram 2 discos/4 lados). Em playlist/shuffle/faixa avulsa: cada faixa ocupa um lado virtual próprio (início = borda, fim proporcional à duração).
3. **Rituais alteram a duração**: agulha descendo (~1,5 s) antes da 1ª faixa do lado e sulco final em loop após a última (até o usuário parar, ou 30 s e fade; configurável). API `pre_roll()`/`tail()` gera esse áudio; o host insere no transporte. Virada de lado: tail do lado A + pre_roll do lado B. Desligável.
4. **Inteligência**: apenas seguidores em tempo real (loudness com ponderação perceptiva, detector de transientes, energia de graves para o sag); sem pré-análise offline.
5. **Desgaste**: por disco (álbum). "Comprar disco novo" zera aquele álbum. Cada reprodução desgasta a região do sulco daquela faixa. **Defeitos permanentes** (riscos) têm posição fixa por disco e voltam no mesmo ponto a cada audição, como num disco real; **poeira/crackle** variam a cada reprodução (semente = disco + contagem). Assim há repetição só onde um disco real repetiria.
6. **Versões**: só os bytes do `WearState` são estáveis e versionados (com migração); o som pode evoluir entre versões; goldens validam métricas, não waveform.
7. **CPU**: < 5% de um núcleo a 96 kHz estéreo é meta, não gate; nível de qualidade automático reduz oversampling em taxas altas (ex.: 8× a 44,1/48 k, 2× a ≥ 176,4 k).
8. Toolchain/plataformas iguais ao `vynamp-decode`; harness de benchmark e contador de alocação próprios.

### 8.9 Deixado ao construtor (original §9)

Modelos matemáticos exatos (waveshaper, filtros, geradores de crackle), coeficientes, curvas do macro, layout dos parâmetros avançados, formato interno do WearState.
