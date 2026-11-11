# ADR-002 — núcleo Rust e backends por capacidade

Versão: 0.9.0 · Data: 2026-11-11 · Estado: resíduo aceito com evidência; as partes de decode, saída, resampling e tags foram substituídas por [ADR-011](ADR-011-pilha-propria.md).

Status: **aceito em parte, em 2026-11-11**, pelo registro do passo 43 corrigido. O que esta ADR ainda governa — domínio/engine/controle em Rust, RT callback restrita, caminho transparente inteiro separado do DSP float — fica aceito, com a evidência abaixo. O que ela governava sobre decoders e saídas de terceiros **não é mais dela**: a [ADR-011](ADR-011-pilha-propria.md) substituiu essas partes em 2026-11-03, e a lista de versões/features que esta ADR devia carregar passou a viver na §"Backend, versões e features" abaixo, medida no código.

Decisão: domain/engine/control em Rust, decode e output por adapters; RT callback restrita; caminho transparente inteiro separado de DSP float. Alternativa: wrapper de playback de alto nível; custo de ocultar clock/gapless/negotiation. Consequência: mais trabalho de engine, mas critérios verificáveis; bibliotecas nativas admitidas com isolation/license.

## Registro do passo 43 — o gate, medido de novo em 2026-11-11

A versão anterior deste registro afirmava que o spike de decoder registry **não
existia**, citando `find . -name '*.rs' | grep -i registry` vazio. **A medição estava
errada e o erro era meu.** O arquivo `crates/vynamp-decode-core/src/registry.rs`
existe em `libs/decode` desde `7f47c0a` (2026-10-23), e já existia no pin
`4af4dd5` que o umbrella carregava quando a linha errada foi escrita em `cca18ff`
(2026-11-09). Hoje o mesmo comando devolve o arquivo dos dois lugares — da raiz do
umbrella (`./libs/decode/crates/vynamp-decode-core/src/registry.rs`) e de dentro de
`libs/decode` —, então **não consigo reproduzir o vazio**, e a explicação mais
provável (submódulo não inicializado no momento da medição) é hipótese, não medição.
O que é medido é o que importa: o arquivo existia no pin daquele commit, e a linha
foi escrita sem ser conferida de novo.

O erro importa porque envenenava o gate: uma linha falsa virava motivo de bloqueio
para o marco M0 inteiro.

| O que o gate exige | Medido em 2026-11-11 | Situação |
|---|---|---|
| spike de decoder registry | **existe**: `crates/vynamp-decode-core/src/registry.rs`, 182 linhas, com `Registry`, `ContainerDescriptor`, `CodecDescriptor`, `Verification`, `probe()` com skip de ID3v2 e desempate por extensão. `default_registry()` em `crates/vynamp-decode/src/lib.rs:104` registra 4 containers (RIFF, AIFF, FLAC, MPA) e 4 codecs (PCM, FLAC, MPEG Audio Layer III, Layer I). É registry de **produto**, não de spike | **atendido**, com a ressalva de que a distinção spike × produto do `deliverable` não tem objeto: o registry nasceu no produto e o `deliverable` o pedia como spike |
| backend candidato escolhido e confrontado com o catálogo | o `Cargo.lock` do decode tem **0 pacotes de registry** — nenhuma dependência de terceiros no workspace inteiro, não só ausência de Symphonia/CPAL. O catálogo `config/format-catalog.json` rev 0.7.0 nomeia backend em **2 das 68** entradas (`flac-native` e `mp3`, ambas `vynamp-decode (ADR-011)`), 65 repetem o marcador "Adapter auditado; escolher e fixar configuração em M0" e 1 é helper isolado (`sacd-iso`). As 68 estão `state: planned` | **atendido no que a ADR-011 deixou**: o backend é `vynamp-decode`, e a lacuna do catálogo é da TASK-E17-01, não desta ADR |
| ADR-002 "com versões/features propostas" | ver §"Backend, versões e features" abaixo, medidas no código | **atendido** neste registro |
| ADR-011 já substituiu parte desta ADR | `adr/ADR-011-pilha-propria.md:5` e `:9` | **decidido** em 2026-11-03 |

## Backend, versões e features

Medido no código em 2026-11-11, no `libs/decode` pinado em `a0201b7`. Esta seção
existe porque era o único item do `deliverable` da TASK-E01-01 que nunca tinha sido
escrito — a versão anterior desta ADR dizia apenas que "a ADR tem uma página".

**Versões.** As nove crates do workspace `vynamp-decode` estão todas em `0.0.0`,
`edition 2024`, `rust-version 1.99`, licença `MIT OR Apache-2.0`. Zero é a versão
honesta aqui: nenhuma API foi publicada, e `vynamp-decode-testkit` tem
`publish = false`. A tag `v0.1.0` do repositório (`16cf103`, 2026-11-07) é **anterior
ao `develop` atual por 38 commits** — ela marca o fim do plano 02, não uma API
estável, e o umbrella não a usa: desde D-GF-6 os pins dos módulos são por commit.

**Features.** Só a crate fachada `vynamp-decode` declara features; as outras oito não
têm `[features]`, o que é deliberado — elas são dependências internas, ligadas umas
pelas outras por `path`. A fachada tem sete:

| Feature | Habilita | Default |
|---|---|---|
| `riff` | `vynamp-decode-riff` — RIFF/WAVE, RF64/BW64 e Wave64 | sim |
| `aiff` | `vynamp-decode-aiff` — AIFF/AIFC | sim |
| `pcm` | `vynamp-decode-pcm` — PCM 8/16/24/32 inteiro e 32/64 float, os dois endianness | sim |
| `flac` | `vynamp-decode-flac` — FLAC, com conformance IETF | sim |
| `mpa` | `vynamp-decode-mpa` — demux de MPEG audio Layer I/II/III, CBR/VBR/free-format, Xing/Info/LAME/VBRI | sim |
| `mp3` | `vynamp-decode-mp3` — decoders Layer I/II/III, com reservoir e trim exato de gapless | sim |
| `default` | as seis acima | — |

`default` habilitando tudo é coerente com o estado: não há custo de binário a
economizar enquanto o único consumidor é o próprio workspace.

**Formatos que o backend cobre hoje.** 4 containers, 4 registros de codec, 6
`SampleFormat` (`U8`, `I16`, `I24` — estendido em `i32`, `I32`, `F32`, `F64`). O
corpus versionado tem **151 arquivos dentro dos quatro diretórios** de
`tests/fixtures/` (mais um `.gitattributes` na raiz do diretório, que é do git e não
do corpus). Desses, **77 entradas de manifesto** em 3 `MANIFEST.txt` (`flac` 31,
`mpeg` 33, `pcm` 13), cada entrada com sha256, taxa, canais, frames e o gerador. O
gerador é o FFmpeg fixado em `n8.1.3-23-g5c4f5744cd-20261022`, usado só como
ferramenta de fixture e como oráculo de teste — nunca no caminho de reprodução. O
quarto diretório, `flac-conformance`, **não tem manifesto**: são 30 arquivos `.flac`
da suíte de conformidade do IETF WG CELLAR em `aa7b0c6`, CC0 1.0, não modificados, e
a procedência está em `SOURCE.txt` dele. A suíte completa não cabe no repositório e
roda por `VYNAMP_FLAC_TEST_FILES=<clone>`.

**O que continua planejado, e não é desta ADR.** `vynamp-decode-meta` (tags) e
`vynamp-decode-cue` não existem como crate ainda; AAC, ALAC, Ogg, CAF, AU, WavPack,
APE, TTA, MPC, DSD→PCM e Matroska estão nas Camadas 2–4 do design. A tabela de
lacunas da [ADR-011](ADR-011-pilha-propria.md) é a fonte para isso, e continua
correta — com uma exceção medida: aquela ADR descreve MP1/2/3 como planejados no
"plano 03, ainda não escrito", e o plano 03 **está escrito e executado**
(`docs/superpowers/plans/2026-10-23-vynamp-decode-03-mpeg.md`, com os crates `mpa`
e `mp3` no workspace e as suítes `oracle_mpeg` e `mpeg` verdes). A correção dessa
linha é da ADR-011, não desta; fica registrada aqui porque quem ler a §Backend das
duas precisa saber qual está atualizada.

**A saída.** `vynamp-output` está em `0.0.0` nas três crates, com
`publish = false` em todas, `Cargo.lock` com **0 pacotes de registry**, e as features
`default = []` mais `test-capture`. Os backends de SO (WASAPI, ALSA, CoreAudio,
AAudio) seguem planejados: o código tem `HostId::Wasapi`/`HostId::Alsa` como
identidade e negociação, e nenhum deles compila um backend real ainda. ADR-011
descreve o core como "em PR #1, ainda não integrado ao develop" — isso envelheceu:
o core está em `develop` desde o merge do PR #1 (`976b236`, 2026-11-07) e a suíte dá
**133 verdes, 1 ignorado, 0 falhas**.

## Por que esta ADR não é congelada inteira

Congelar o documento como está criaria duas fontes de verdade para a mesma escolha: a
ADR decide "decode e output por adapters" com Symphonia/CPAL como candidatos, e a
ADR-011, aceita pelo mantenedor em 2026-11-03, derrubou exatamente esses candidatos.
O catálogo nomeia `vynamp-decode` como backend. Um congelamento que mantivesse a
menção a Symphonia/CPAL contradiria a decisão posterior no mesmo parágrafo.

**O que ficou aceito**, e é o resíduo que a ADR-011 explicitamente preservou
(`adr/ADR-011-pilha-propria.md:13`): domain/engine/control em Rust, RT callback
restrita, caminho transparente inteiro separado do DSP float. A evidência desse
resíduo é a TASK-E01-02 (callback e clock, medidos em hardware real em 2026-11-07) e
a TASK-E01-01 (decode e gapless contra oráculo, 508 testes verdes em 2026-11-11).

**O que saiu desta ADR:** toda menção a Symphonia e CPAL como candidatos. A frase
"Decisão" acima mantém o texto histórico, e por isso nomeia os dois — mas eles não
são mais candidatos, são o registro do que a ADR-011 substituiu.

## Custo se esta leitura estiver errada

Se o mantenedor considerar que a ADR-002 deve continuar `proposta` até o passo 43
decidir o congelamento formal, então este registro vira uma emenda de fato sem
mudança de status, e o gate de M0 continua sem a ADR. O custo é baixo e reversível:
é uma linha de `Status` e um commit. O que não é reversível é deixar a linha falsa
sobre o registry no documento, porque ela já foi lida como bloqueio uma vez.
