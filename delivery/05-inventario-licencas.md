# Inventário de licenças, codecs e assets

Versão: 0.1.0 · Data: 2026-11-12 · Estado: inventário medido. As decisões de marca e de
distribuição continuam do mantenedor: o que este documento faz é registrar o estado e
listar as pendências em §9, não decidi-las.

Este documento responde à `TASK-E11-03` e separa três coisas que costumam ser
confundidas: o que o projeto **distribui**, o que ele **usa para se construir** e o que
ele apenas **consulta como referência**. Nenhum número aqui vale por si — cada seção
nomeia o comando que o produziu, e a saída bruta está em `evidence/PR/task-e11-03/`.

## 1. Método e oráculo

Toda contagem deste documento veio de uma execução em 2026-11-12, a partir da raiz do
umbrella:

```bash
cargo metadata --format-version 1 --locked --all-features > metadata-umbrella.json
cargo deny --format json check licenses > deny-licenses-umbrella.json 2>&1
cargo tree --locked -p xtask -e normal,build --prefix none --no-dedupe --target all
cargo deny --format json check licenses        # dentro de libs/decode
```

O oráculo independente é o **`cargo deny` 0.20.2**: não é código deste projeto e resolve
o `Cargo.lock` de verdade, em vez de repetir o que o `Cargo.toml` declara. A saída bruta:

| Arquivo | Conteúdo | SHA-256 |
|---|---|---|
| `deny-licenses-umbrella.json` | 160 linhas JSON: 158 diagnósticos `rejected`, 1 aviso de configuração e 1 `summary` com `errors: 158` | `7048a251c6a448dc7363b6cf02222887f5ae9175858dce905c8dd1f18ad411d3` |
| `deny-licenses-decode.json` | 11 linhas; só os 9 crates de path do módulo, cada um `MIT OR Apache-2.0` | `553705b712d66c9e1af4540087851343048fd83fa3e093e555a78785cc237dac` |
| `licence-counts-20261112.txt` | contagens agregadas derivadas da `metadata` | `471efd46f09a5767bce20b38230aca43cf8af070336ac4931e28ef8102d914fb` |

**Por que 158 erros, e por que isso não é defeito.** O repositório não tem `deny.toml`,
então o `cargo deny` cai na configuração padrão, que não permite licença nenhuma, e sai
com código 4. O que presta nessa saída é o campo `span`, que traz a **expressão declarada
de cada pacote**, e o campo `graphs`, que traz quem puxa quem. Criar o `deny.toml` com a
lista de permitidas é `TASK-E11-04`, junto com o SBOM e o audit.

**O que o `cargo deny` não vê.** Ele avalia 158 pares `(nome, versão)`, não os 164 do
lock: poda nós que só existem como proc-macro (`syn 2.0.119`, `zerocopy-derive`) e arestas
opcionais não habilitadas no conjunto de alvos resolvido. A diferença está nomeada em
`licence-counts-20261112.txt`, e os 7 pacotes que ele não avalia foram lidos no lock.

## 2. Terceiro no umbrella: 164 crates, um único risco copyleft

O `Cargo.lock` da raiz tem **165 pacotes**: 164 de registry e 1 de path (`xtask`, versão
0.7.0, `publish = false`). Os 164 são todos dependências do `xtask` — ferramenta de
desenvolvimento, **não distribuída**; o workspace do umbrella tem só ele.

| Expressão declarada | Crates |
|---|---|
| `MIT OR Apache-2.0` | 89 |
| `MIT` | 21 |
| `Unicode-3.0` | 18 |
| `Apache-2.0 OR MIT` | 12 |
| `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT` | 4 |
| `Unlicense OR MIT` | 3 |
| `MIT OR Zlib OR Apache-2.0`, `BSD-2-Clause OR Apache-2.0 OR MIT`, `MIT/Apache-2.0`, `MIT OR Apache-2.0 OR LGPL-2.1-or-later`, `Unlicense/MIT` | 2 cada |
| `0BSD OR MIT OR Apache-2.0`, `MIT-0`, `Apache-2.0/MIT`, `Apache-2.0 OR BSL-1.0`, `(MIT OR Apache-2.0) AND Unicode-3.0`, `Zlib`, `Apache-2.0` | 1 cada |

São 18 expressões distintas, somando 164. **Todas oferecem um caminho permissivo**, e
duas merecem nota:

- **`r-efi 5.3.0` e `6.0.0`** são `MIT OR Apache-2.0 OR LGPL-2.1-or-later`. Escolher MIT
  ou Apache não copylefta nada; escolher LGPL seria uma decisão diferente. Ambos são
  UEFI-only e não entram no build do Windows — estão entre os 19 pacotes fora do alvo
  `x86_64-pc-windows-msvc`.
- **Cinco pacotes usam barra em vez de `OR`**, em três expressões distintas
  (`Apache-2.0/MIT`, `MIT/Apache-2.0`, `Unlicense/MIT`): `bytecount`, `num-cmp`,
  `same-file`, `version_check`, `walkdir`. A convenção de barra é ambígua entre "ou" e
  "e"; nos cinco, a licença real é `OR` — é como os próprios arquivos `LICENSE-*` de cada
  crate leem. Fica registrado como fragilidade de metadado, não como risco.

**Cobertura de arquivo de licença.** 160 dos 164 publicam um arquivo de licença no pacote;
os 4 que não publicam são `r-efi 5.3.0`, `r-efi 6.0.0`, `uuid-simd 0.8.0` e `vsimd 0.8.0`,
e os quatro declaram a licença no `Cargo.toml`. Não há crate sem expressão declarada.

**Cinco no lock e fora de toda árvore**: `futures-core`, `futures-task`, `futures-util`,
`pin-project-lite` e `slab`, todos `MIT OR Apache-2.0` ou `Apache-2.0 OR MIT`. Nenhum
deles está no grafo compilado; eles existem no lock porque ficam atrás de aresta
**opcional** não ativada — `futures-util` é dependência obrigatória de `js-sys`, que é
opcional de `getrandom`, que por sua vez só é alcançável por nós opcionais. Estão no lock
porque ele é resolvido com todas as features (`--all-features`); não estão no binário.
Medido pelo complemento: `cargo tree -e normal,build --target all` tem 160 pares, e são
exatamente esses cinco de registry que ficam de fora dos 164 do lock — os outros 159 estão
no grafo.

**As 17 dependências diretas do `xtask`**, com o requisito do manifesto, a versão que o
lock resolveu e a licença medida, estão em `licence-counts-20261112.txt` §"dependências
diretas do xtask": `anyhow`, `base64`, `clap`, `getrandom`, `jsonschema` (`MIT`), `png`,
`quick-xml` (`MIT`), `regex`, `semver`, `serde`, `serde_json`, `serde_yaml_ng` (`MIT`),
`sha2`, `tempfile`, `toml`, `walkdir` (`Unlicense/MIT`) e `zip` (`MIT`). Nenhuma copyleft.

O arquivo lista **todas** as versões que o lock tem para cada nome, e não só a que o
requisito direto resolve, porque cinco nomes aparecem duas vezes no lock: `getrandom` está
em 0.3.4 (o requisito direto `^0.3`, compartilhado com `ahash`) **e** em 0.4.3 (puxado por
`tempfile` e `zip`); `base64` em 0.23.1 (direto) e 0.22.1 (transitivo); `syn` em 2.0.119
e 3.0.9; `miniz_oxide` em 0.8.9 e 0.9.1; e `r-efi` em 5.3.0 e 6.0.0. Citar só uma das
versões de cada seria uma contagem errada por omissão, e é por isso que a tabela tem uma
coluna de versões resolvidas em vez de uma só.

## 3. Os módulos: zero terceiro

O umbrella não distribui binário, e nenhum módulo de biblioteca puxa crate externo. Medido
pelo `Cargo.lock` de cada um dos oito submódulos de módulo, todos checados out nesta
máquina em 2026-11-12:

| Módulo | Pacotes no lock | De terceiro |
|---|---|---|
| `libs/decode` | 9 | 0 |
| `libs/output` | 4 | 0 |
| `libs/dsp`, `libs/encode`, `libs/engine` | 1 cada | 0 |
| `apps/player`, `apps/converter`, `apps/restore` | 1 cada | 0 |

O `cargo deny` dentro de `libs/decode` confirma pelo outro lado: 9 crates, os nove de path,
todos `MIT OR Apache-2.0`, nenhuma licença de terceiro a avaliar. Isso é a prova viva de
[`AGENTS.md` §1.3](../AGENTS.md) — primitivos atravessam, dependência externa não entra
sem passar pelo inventário.

**Consequência direta:** hoje não existe `THIRD_PARTY_NOTICES` a emitir para código Rust, e
[`LEGAL.md`](../LEGAL.md) pode afirmar "nenhum ativo de terceiros foi admitido" sem
contradição. O primeiro crate externo que entrar em qualquer módulo reabre esta seção.

## 4. Codecs: a licença do código e a licença da patente são coisas distintas

Esta é a distinção em que a alegação falsa nasce, e ela vale para cada linha da tabela
abaixo: a **implementação** deste projeto é `MIT OR Apache-2.0`; o **formato** tem status
de patente próprio, independente de quem o implementa. Licença permissiva no código não
concede direito sobre patente de terceiro, e patente expirada não torna um formato livre de
outras restrições.

O `vynamp-decode` implementa do zero, sem dependência externa, o que está na tabela. O
catálogo de planejamento é [`config/format-catalog.json`](../config/format-catalog.json); a
cobertura de cada perfil é de `TASK-E17-01`, não desta TASK.

| Codec | Crate | Licença do código | Status do formato, medido na fonte |
|---|---|---|---|
| PCM (WAV, AIFF) | `vynamp-decode-pcm`, `-riff`, `-aiff` | `MIT OR Apache-2.0` | sem patente conhecida |
| FLAC | `vynamp-decode-flac` | `MIT OR Apache-2.0` | Xiph afirma: "Neither the FLAC nor Ogg FLAC formats nor any of the implemented encoding/decoding methods are covered by any known patent" ([licença Xiph](https://xiph.org/flac/license.html), lida em 2026-11-12) |
| MPEG audio L1/L2 (MP2) | `vynamp-decode-mpa` | `MIT OR Apache-2.0` | MPEG-1/2 Audio, patentes da era 1990; sem programa de licenciamento ativo localizado |
| MPEG audio L3 (MP3) | `vynamp-decode-mp3` | `MIT OR Apache-2.0` | Fraunhofer: o programa de licenciamento "has been terminated", em "April 23, 2017" ([iis.fraunhofer.de](https://www.iis.fraunhofer.de/en/ff/amm/consumer-electronics/mp3.html), lida em 2026-11-12). **A página não diz que as patentes expiraram** — diz que o programa terminou. As duas coisas não são a mesma afirmação e este documento não as troca |

**Fora do escopo desta TASK, com dono nomeado.** AAC-LC/HE-AAC/ALAC/Vorbis/Opus são
`TASK-E01-07`; WMA/APE/WV/TTA/MPC/WebM/RF64 são `TASK-E01-08`; AC-3/E-AC-3/TrueHD/DTS são
`TASK-E17-16`. Nenhum deles tem implementação hoje, e o inventário de patente deles não é
escrito aqui. A política de features que os governa já existe e está em
[`libs/decode/LEGAL.md`](../libs/decode/LEGAL.md): `default` para especificação aberta ou
patente expirada, `encumbered` e `reverse-engineered` desligados por padrão.

## 5. FFmpeg: oráculo de desenvolvimento, nunca artefato distribuído

Medido em 2026-11-12: `find libs apps -name build.rs` não devolve nada, e nenhum
`Cargo.toml` de módulo ou app menciona `ffmpeg` ou `libav`. O binário local, em
`.cache/tools/ffmpeg/`, é `n8.1.3-35-gf1acb0a266-20261111`, um build `--enable-gpl
--enable-version3` da comunidade — **licença que este projeto não pode distribuir** se um
dia viesse a embarcá-lo.

Por isso a política é curta e não muda com a versão do binário:

- FFmpeg **não** é dependência de build nem de teste. Os testes comparam contra fixtures
  versionadas com hash, não contra uma execução ao vivo.
- Os scripts de `libs/decode/tools/fixtures/` leem `$env:FFMPEG` ou `-FFmpeg` e só rodam
  quando alguém quer regerar fixture. **Não há modo que regenere só o manifesto**: rodar o
  gerador inteiro é o único caminho, e ele reescreve todos os arquivos que produz.
- O pin do oráculo está em [`adr/ADR-002-audio-core.md`](../adr/ADR-002-audio-core.md)
  (`n8.1.3-23-g5c4f5744cd-20261022`); `engineering/08-reinicio-windows.md` só ensina onde
  baixar e como expor `$env:FFMPEG`, sem fixar versão. As fixtures registram a versão do
  gerador no próprio `MANIFEST.txt`, e é por isso que o número acima é uma citação, não uma
  suposição.

- **Nenhuma feature GPL ou nonfree entra em pacote permissivo.** Se um dia houver bundle
  FFmpeg, ele é build próprio, com a lista de `--enable` travada, separado do que é
  permissivo — e essa decisão é de release, não desta TASK. A regra já está escrita em
  [`security/02-privacidade-licencas.md`](../security/02-privacidade-licencas.md).

**Uma limitação do gerador, medida e registrada, não desta TASK.** Rodar
`generate-mpeg-dev.ps1` inteiro regera quatro fixtures de
`crates/vynamp-decode-mpa/tests/fixtures/` com bytes diferentes e o mesmo tamanho
(`cbr128-44k-stereo.mp3`, `cbr128-48k-stereo.mp3`, `mpeg2-22k-stereo.mp3`,
`nolame-cbr128-48k.mp3`); as quinze do gapless são reproduzíveis byte a byte. Está
registrada em
[`libs/decode/tasks/items/TASK-E01-01.md`](../libs/decode/tasks/items/TASK-E01-01.md) e
pertence à TASK dona do corpus MPEG, não a esta. A falta de manifesto em três raízes de
fixture é outra lacuna e está medida em §7.

## 6. ASIO: dual-licenciado, e a rota proprietária proíbe redistribuir o SDK

O ASIO SDK **não está no repositório** e não é dependência de nada. A leitura da fonte
oficial (SDK 2.3.4, `ASIOSDK/LICENSE.txt`, baixado de
`steinberg.net/asiosdk` em 2026-11-12) dá o texto que governa qualquer uso futuro:

> This Software Development Kit is licensed under the terms of the Steinberg ASIO License,
> or alternatively under the terms of the General Public License (GPL) Version 3.

E, na opção proprietária:

> The Software Development Kit may not be distributed in parts or its entirety without
> prior written agreement by Steinberg Media Technologies GmbH.

Duas consequências que o projeto precisa carregar:

1. **Não existe caminho "ASIO no núcleo".** A rota proprietária exige acordo assinado com a
   Steinberg e proíbe redistribuir o SDK; a rota GPLv3 contaminaria o resto do produto, que
   é `MIT OR Apache-2.0`. Por isso o [ADR-011](../adr/ADR-011-pilha-propria.md) já coloca
   ASIO "fora do núcleo, adapter privado opcional futuro, sem plano".
2. **Trazer o SDK para o repo é uma decisão, não um `git add`.** Nem pela rota GPLv3 ela é
   trivial: o SDK inteiro passaria a ser GPLv3 e conviveria com código incompatível. Nada
   disso foi feito, e o registro fica como pendência do mantenedor (§9).

O que **não** está provado aqui: se um dia houver binário com ASIO, o cumprimento do acordo
assinado. Isso é verificação de release, com o contrato em mãos, não de inventário.

## 7. Fontes e assets

**Fontes próprias: nenhuma.** `git ls-files` não devolve nenhum `.ttf`, `.otf` ou `.woff2`
em nenhum dos nove submódulos nem no umbrella. A única ocorrência de fonte é o
`codicon` do Playwright, em três arquivos dentro de
`reference/vynamp-mvp/node_modules/playwright-core/lib/vite/` — dois como
`codicon-DCmgc-ay.ttf` (`dashboard/assets/` e `recorder/assets/`) e um como
`codicon.DCmgc-ay.ttf` com ponto (`traceViewer/`). `node_modules/` está no `.gitignore` da
referência e nenhum dos três é versionado; eles aparecem só no checkout central do
umbrella, onde o `npm install` foi rodado — worktree recém-criada não os tem. Não entram
no build e não são asset do produto. A decisão de tipografia em
[`design/01-sistema-visual.md`](../design/01-sistema-visual.md) fala em "system sans ou
Noto Sans com licença incluída": o Noto é OFL-1.1 (confirmado em
`notofonts/notofonts.github.io`, lido em 2026-11-12), mas **nenhum arquivo de fonte foi
admitido ainda**, então não há aviso a emitir. A escolha final da fonte de display é
pendência declarada em §9.

**Assets de marca: gerados para o projeto.** `assets/branding/asset-manifest.json` registra
três PNGs com sha256, dimensões e papel — a prancha C aprovada, o símbolo transparente e a
prancha histórica A/B/C/D. `state` é `approved-reference-native-exports-pending`, e os
exports nativos (ICO/ICNS/SVG/PNG por tamanho no Android) aparecem como `pending` por
plataforma. O [ADR-008](../adr/ADR-008-marca.md) é o dono: `branding_exports_ready=false`
até os gates reais. Os 15 SVGs de `assets/icons/`, mais 4 pranchas de layout em `assets/`
(`classic`, `mini`, `library`, `android`), são desenho do próprio projeto.

**Corpus de conformidade FLAC: CC0 1.0.** 20 arquivos `.flac` em `subset/` e 10 em
`faulty/` (o arquivo 10 do grupo `faulty`, com comentário Vorbis inválido, foi omitido de
propósito), 36 arquivos no total contando os textos, 8.211.668 bytes = 7,83 MiB, de
`ietf-wg-cellar/flac-test-files` no commit
`aa7b0c6cf32994c106ae517a08134c28a96ff5b2`, não modificados.
`libs/decode/tests/fixtures/flac-conformance/SOURCE.txt` e `LICENSE.txt` registram origem
e licença ao lado dos arquivos — que é o que
[`engineering/05-testes.md`](../engineering/05-testes.md) exige de fixture.

**Fixtures sintéticas.** Todo o resto do corpus é gerado por FFmpeg a partir de um tom
sintético (`tools/fixtures/source-sine.wav`, 48 kHz estéreo s32, 4800 frames, 997 Hz à
esquerda e 1499 Hz à direita, escrito por
`crates/vynamp-decode-testkit/examples/gen_source_sine.rs`), sem material de terceiro. É o
que permite versioná-las sem notícia de licença: não há obra alheia nelas.

**O que falta nelas, medido.** [`engineering/05-testes.md`](../engineering/05-testes.md)
exige que o manifesto de fixture traga **hash, licença, formato e oráculo próprio**. Cinco
raízes de fixture existem; só três têm hash versionado:

| Raiz | Arquivos | Manifesto |
|---|---|---|
| `libs/decode/tests/fixtures/` | 152 | `pcm/`, `flac/` e `mpeg/` têm `MANIFEST.txt` |
| `crates/vynamp-decode/tests/fixtures/` | 16 | `mpeg/triangulation.sha256` |
| `crates/vynamp-decode-flac/tests/fixtures/` | 4 | **nenhum** |
| `crates/vynamp-decode-mp3/tests/fixtures/` | 47 | **nenhum** |
| `crates/vynamp-decode-mpa/tests/fixtures/` | 12 | **nenhum** |

São 63 arquivos sem hash versionado. A lacuna não é desta TASK — o corpus pertence ao
`vynamp-decode` — e não muda o inventário de licença, porque são fixtures sintéticas: não
há obra de terceiro a atribuir. Fica registrada aqui porque um inventário que só olhasse
`pcm/`, `flac/` e `mpeg/` diria que o corpus está coberto, e não está.

## 8. O que este inventário **não** prova

Três limites, registrados para não virarem alegação:

- **Não é parecer jurídico.** É medição de metadado e leitura de fonte oficial, com data.
  Quem distribui responde pelas patentes da sua jurisdição, como o
  [`LEGAL.md`](../LEGAL.md) já diz.
- **Não cobre o que não tem artefato.** Nenhum binário foi publicado, então não há
  `THIRD_PARTY_NOTICES` de release a conferir, nem SBOM de artefato — isso é
  `TASK-E11-04` (SBOM e audit no CI) e `TASK-E11-08` (notices de lançamento).
- **Não decide.** Nenhuma licença de terceiro foi escolhida, nenhuma dependência foi
  trocada, nenhum pin de NDK/Gradle foi fixado. Os três são decisão registrada do
  mantenedor.

## 9. Pendências que só o mantenedor fecha

| # | Pendência | Estado medido | Dono |
|---|---|---|---|
| 1 | Nome/marca e licença final dos ativos | marca decidida (ADR-008); exports nativos `pending`; `branding_exports_ready=false` | mantenedor, pelo ADR-008 |
| 2 | Fonte de display e fonte de UI | nenhuma fonte admitida; Noto é OFL-1.1 se for a escolha | mantenedor, em `design/01` |
| 3 | ASIO: rota proprietária (acordo assinado) ou GPLv3, ou nenhuma | nada no repositório; ADR-011 diz "fora do núcleo, sem plano" | mantenedor, pelo ADR-011 |
| 4 | Bundle de FFmpeg, se algum dia houver | não há bundle; binário local é GPL e não distribuível | mantenedor, decisão de release |
| 5 | `deny.toml` com a lista de licenças permitidas | não existe; é por isso que o `cargo deny` recusa as 158 | `TASK-E11-04` |
| 6 | `THIRD_PARTY_NOTICES` e SBOM de release | não existem; nada de terceiro foi admitido | `TASK-E11-04`, `TASK-E11-08` |

A linha 3 é a que tem prazo mais curto na prática: qualquer trabalho em `vynamp-output`
para Windows que queira ASIO esbarra nela antes de escrever a primeira linha de código.
