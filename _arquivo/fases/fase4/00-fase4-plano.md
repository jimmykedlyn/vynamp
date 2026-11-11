# FASE 4 — M0 de verdade: as provas que faltam

Versão 2.1 · 2026-11-11 · Plano de execução. Substitui a v1 de 2026-11-09, cujas premissas
envelheceram (ver §8). Mantenedor aprova antes do passo 44.

## 1. Por que esta fase existe

A FASE 3 acabou (passos 30–38, PRs #40–#45 e #7–#23). O repositório tem a forma final:
umbrella `jimmykedlyn/vynamp`, 9 submódulos, 8 repositórios de módulo, CI Gate em todos,
199 TASKs no backlog, 10 repositórios públicos e protegidos.

O que **não** existe é o produto. Medido em 2026-11-11 nesta árvore:

| Repo | `.rs` | LOC | `#[test]` | Estado real |
|---|---|---|---|---|
| `libs/decode` | 106 | 29 100 | 505 | decoders PCM/FLAC/MP3/MPA/AIFF/RIFF funcionando, com oráculo FFmpeg |
| `libs/output` | 42 | 7 768 | 130 | backend nulo, ring SPSC, seqlock de clock, DoP, capture |
| `libs/dsp` | 11 | 1 662 | 21 | **resampler e conversão implementados e medidos** (TASK-E04-02, 2026-11-11) |
| `libs/encode` | 1 | 6 | 0 | stub |
| `libs/engine` | 1 | 6 | 0 | stub |
| `apps/player` | 1 | 3 | 0 | stub |
| `apps/converter` | 1 | 3 | 0 | stub |
| `apps/restore` | 1 | 3 | 0 | stub |

Traduzindo: **três das cinco libs provaram alguma coisa; duas estão vazias e os três apps
também.** M0 pede "ADRs aprovados com evidência, não só hello-world".

A FASE 4 é o caminho de M0 atual até M0 completo, sem inventar escopo novo: cada item
abaixo é uma TASK `phase: M0` que já está no `tasks/backlog.json`.

## 2. O que já fechou desde a v1

Duas TASKs que a v1 listava como trabalho a fazer já não são:

- **TASK-E01-02** (output, callback e clock) — `implemented`. A limitação de clock foi
  aceita e registrada; a correção pertence à E01-05.
- **TASK-E02-11** (MVP de referência congelado) — `implemented`. O pacote vive em
  `vynamp-mvp-reference` e a TASK aponta para lá.

E uma que a v1 punha no meio da fila de spikes já foi executada:

- **TASK-E04-02** (resampler, PCM24/32, dither) — `implemented` em 2026-11-11. `libs/dsp`
  saiu do stub: 11 arquivos, 1 662 LOC, 21 testes verdes, alias medido em −94,4 dB
  (`balanced`) e −128,4 dB (`high`) contra alvos de 80 e 110. Evidência em
  `evidence/PR/task-e04-02/`; a spec `libs/dsp/specs/dsp.md` §3/§5/§6/§9/§10 foi autorada
  e **continua não aprovada** — quem aprova é o mantenedor, por `GOVERNANCE.md`.

Estado do backlog medido agora: **199 TASKs, 187 `planned`, 12 `implemented`**, e
**todas as 199 com `acceptance_ready: true`**. Nenhuma `in_progress`.

### 2.1 O que a v1 errava, e por que isso importa

A v1 foi escrita quando o backlog tinha 193 TASKs e 182 sem critérios. A §2.2 dela
("autoria dos Given/When/Then que faltam", 36 TASKs de M0+M1) era o **maior bloco de
trabalho do plano** e hoje tem **tamanho zero**: a autoria aconteceu nos passos 39–43 e
nas PRs que fecharam E01-02, E02-11 e E04-02.

Isso não é detalhe de contabilidade: metade do plano da FASE 4 era um passo que já não
existe. Executar a v1 significaria procurar trabalho que não há.

## 3. O que a FASE 4 entrega

Uma entrega só: **as 16 TASKs de M0 que faltam**, em ordem de dependência, mais o
congelamento das ADRs que elas gateiam.

| # | TASK | Repo | O que prova | Bloqueia | Est. (dias) |
|---|---|---|---|---|---|
| 1 | E15-01 | vynamp | plugin/agentes ativos e skill discovery em máquina compatível | E15-02 | 2–5 |
| 2 | E15-02 | vynamp | worktrees, claim atômico, remoção segura | — | 2–5 |
| 3 | E11-02 | vynamp | threat model, limites, IPC, sandbox | — | 2–5 |
| 4 | E11-03 | vynamp | licenças de codecs, fontes e assets | — | 2–5 |
| 5 | E10-01 | vynamp | corpus de fixtures licenciado com hashes | — | 2–5 |
| 6 | E14-01 | vynamp | matriz de toolchains e ABIs nos cinco alvos (alvos já instalados; falta cross-linker) | E17-01 | 2–5 |
| 7 | E01-01 | vynamp-decode | decode/gapless/precisão contra referência | E13-01, E17-01 | 2–5 |
| 8 | E02-01 | vynamp-player | egui/AccessKit com a11y real nos três desktops | — | 2–5 |
| 9 | E17-01 | vynamp-decode | catálogo por binding, capacidades, true-peak | — | 2–5 |
| 10 | E07-01 | vynamp-player | sandbox WASM com fuel e memória | — | 2–5 |
| 11 | E13-01 | vynamp-player | Android arm64 real: NDK, AAudio, Media3 | — | 2–5 |
| 12 | E16-01 | vynamp-player | OpenRequest, limite de lote, ordem | — | 2–5 |
| 13 | E02-12 | vynamp-player | gabarito visual — primeiro conjunto | — | 2–5 |
| 14 | E02-13 | vynamp-player | gabarito visual — movimento e medidores | — | 3–6 |
| 15 | E02-14 | vynamp-player | gabarito visual — terceiro conjunto | — | 2–5 |
| 16 | E02-15 | vynamp-player | gabarito visual — quarto conjunto | — | 2–4 |

**Soma: 34–81 dias úteis.** Não é orçamento de calendário — é o que o backlog declara, e o
roadmap já avisa que não se soma faixa como cronograma.

Os quatro gabaritos (`E02-12` a `E02-15`) entram na fila agora porque a dependência deles
— o pacote da `E02-11` — está resolvida desde que aquela TASK fechou.

## 4. O que não dá para fazer nesta máquina, e por quê

Três itens de M0 exigem hardware ou SDK que não existe aqui, medido agora — e um
quarto achado de configuração que não bloqueia M0:

| Item | Falta | Consequência |
|---|---|---|
| **E13-01** (Android real) | `config/toolchains.json` tem `android_ndk: null`, `android_compile_sdk: null`, `android_target_sdk: null`, `android_build_tools: null`; não há SDK em `AppData/Local/Android/Sdk` nem `ANDROID_HOME`/`ANDROID_NDK_HOME` | o spike não roda. Vira spec + script reprodutível + limitação registrada, que é o que o `Then` de E13-01 já admite |
| **E02-01** (a11y desktop) | dos quatro leitores que o `Then` exige (Narrator, NVDA, Orca, VoiceOver), esta máquina tem **um**: `C:/Windows/System32/Narrator.exe`. NVDA **não** está instalado (nem em `Program Files`, nem em `(x86)`, nem no PATH); Orca e VoiceOver são de Linux/macOS | prova parcial: Narrator real, os outros três por relatório de terceiro ou limitação registrada |
| **E14-01** (cinco ABIs) | **nenhum linker cruzado**: `gcc`, `clang`, `cc`, `zig`, `x86_64-linux-gnu-gcc`, `aarch64-linux-gnu-gcc` todos ausentes do PATH. Os alvos do `rustup` **já estão instalados** (ver medida abaixo) e não resolvem isso | `cargo check` passa nos cinco; `cargo build` só liga em `x86_64-pc-windows-msvc`. Falta instalar cross-toolchain por alvo (MSVC do VS 2022 já existe e liga) |
| **`ios-arm64` no `config/project.json`** | `aarch64-apple-ios` não está instalado, e nenhuma TASK de M0 o cita | não é bloqueio de M0 — iOS é a FASE 6 (`E20`). Mas o config declara o alvo e nenhum gate compara config com ambiente; fica registrado para a E14-01 decidir se ele entra na matriz agora ou na FASE 6 |

Isso não cancela as TASKs. Muda o que "fechada" significa nelas: **critério atendido com
limitação medida e escrita**, nunca afirmado sem prova. É a mesma regra que fez a E19-01
fechar com AC-3 explicado em vez de fingido.

**Acrescentado na v2:** a instrução do mantenedor de 2026-11-11 — mobile roda só o player,
sem Restore e sem Converter, e o painel DSP é adaptado ao toque em Android e iOS — está
registrada em `D-MOB-01`/`D-MOB-02` e nas specs de plataforma. Ela **não** cria TASK nova e
não muda marco: o painel iOS continua em `E20-05` (FASE 6) e o layout phone Android em
`E13-05` (M1). O que ela faz é tirar das TASKs de mobile a tentação de incluir Restore ou
Converter "por simetria com o desktop".

### 4.1 A medida que corrige a v2, e por quê

A v2 afirmava que `rustup target list --installed` tinha **só**
`x86_64-pc-windows-msvc` e que `rustup target add` fecharia a E14-01. Medido em
2026-11-11:

```
aarch64-apple-darwin        aarch64-linux-android      aarch64-pc-windows-msvc
aarch64-unknown-linux-gnu   x86_64-apple-darwin        x86_64-pc-windows-msvc
x86_64-unknown-linux-gnu
```

Sete alvos, não um. `rustup target add` já foi feito e **não fecha nada**: alvo
instalado dá `std` pré-compilado, o que basta para `cargo check`, mas ligar um
binário exige linker e sysroot do alvo. Medido por módulo, `--all-targets`
(inclui bins de teste, que ligam):

| alvo | `cargo check --all-targets` | `cargo build --tests` |
|---|---|---|
| `x86_64-pc-windows-msvc` | ok nos 8 módulos | **liga** |
| `x86_64-unknown-linux-gnu` | ok nos 8 módulos | `linker 'cc' not found` |
| `aarch64-unknown-linux-gnu` | ok nos 8 módulos | `linker 'cc' not found` |
| `x86_64-apple-darwin` | ok nos 8 módulos | `linker 'cc' not found` |
| `aarch64-apple-darwin` | ok nos 8 módulos | `linker 'cc' not found` |
| `aarch64-linux-android` | ok nos 8 módulos | `linker 'cc' not found` |

Duas consequências para o passo 45. A primeira: a E14-01 deixa de ser "instalar
alvos" e vira "instalar cross-toolchain e provar ligação", que é mais caro e mais
lento — o caminho barato (`zig cc` como linker, ou MSVC para os alvos que ele
cobre) precisa ser decidido antes. A segunda: `cargo check` verde nos cinco não é
prova de ABI, e o `Then` da E14-01 fala de "resolve em cada ABI" — check sozinho
não satisfaz. Tratar check como prova seria exatamente o "afirmado sem prova" que
a §4 proíbe.

Uma terceira, que vale para o plano inteiro: `cargo build --workspace` no umbrella
**não constrói o produto**. O `Cargo.toml` da raiz declara `default-members =
["xtask"]` e exclui os oito módulos, cada um repositório próprio. Quem medir
"o workspace compila" e concluir que as libs compilam mediu o `xtask`.

Quarta consequência, e a que mais custa tempo de diagnóstico: **há dois
toolchains nesta máquina e eles não têm os mesmos alvos.** `rust-toolchain.toml`
existe no umbrella e nos oito módulos, todos em `channel = "1.99.0"`; dentro
deles o rustup resolve `1.99.0-x86_64-pc-windows-msvc`, com 8 alvos (os 7 acima
mais `wasm32-wasip1`). Fora deles — inclusive em
`D:/Projects/vynamp-media-player`, onde o `STATE.md` mora — resolve o
`stable-x86_64-pc-windows-msvc`, com 5. O `rustc` é o mesmo binário nos dois; o
que falta é `rust-std` por alvo. Compilar de um diretório sem
`rust-toolchain.toml` falha com `error[E0463]: can't find crate for 'std'`, e o
erro não menciona toolchain nenhuma.

Uma quinta, sobre a E01-01: `ffmpeg`, `ffprobe`, `sox`, `flac` e `lame` **não estão
no PATH** desta máquina. Isso **não** bloqueia o spike como parece à primeira
vista: `libs/decode/crates/vynamp-decode/tests/oracle_flac.rs` explica que o FFmpeg
é ferramenta de geração, nunca dependência — o PCM de referência está commitado em
`tests/fixtures/flac/ref/` ao lado do FLAC, e o teste lê essa referência pelo
caminho PCM já provado, em `Mode::Strict` com MD5 de STREAMINFO. As 42 suítes de
`libs/decode` rodam verdes aqui sem FFmpeg. O que falta para a E01-01 é o gapless
e o congelamento da ADR-002, não o oráculo.

E a metade wasm da E07-01 é viável aqui: `rustup target add wasm32-wasip1` e um
`cdylib` de teste com `panic = "abort"` produziu `wprobe.wasm` de 573 B, ligado
pelo `rust-lld` que o toolchain já traz — sem cross-linker externo. O runtime
(`wasmtime`) ainda precisa ser adicionado; crates.io respondeu 200.

## 5. Ordem de execução proposta

### Passo 44 — os spikes baratos (2 PRs) — **não depende de nenhuma decisão da §7**

1. **E15-01, E15-02** — toolchain e worktrees. O hook existe, o claim existe; falta a
   prova escrita com comando e saída.
2. **E11-02, E11-03** — documentos com prova: threat model e inventário de licenças.
   Não é código, é decisão escrita com teste mapeado.

### Passo 45 — as provas de áudio e de build (2–3 PRs)

3. **E10-01** — corpus de fixtures. O `decode` já tem o gerador; falta o conjunto do
   umbrella com manifesto e licença.
4. **E14-01** — os sete alvos do `rustup` já estão instalados; o que falta é
   cross-linker por alvo e a prova de **ligação**, não de `check` (§4.1). Decidir
   `zig cc` versus toolchain por SO antes de começar; sem isso o passo não fecha.
5. **E01-01** — o decode já decodifica; falta a prova de gapless contra referência e o
   ADR-002 congelado. Provavelmente o spike mais curto da lista.

### Passo 46 — as provas de código (3–4 PRs)

6. **E17-01** — catálogo por binding, capacidades e true-peak.
7. **E07-01** — sandbox WASM com fuel e memória.
8. **E16-01** — OpenRequest, limite de lote, ordem.

### Passo 47 — os dois com limitação de hardware (2 PRs)

9. **E02-01** — a11y desktop, com Windows real e o resto declarado.
10. **E13-01** — Android com spec + script + limitação; **nenhum `application_ready` vira
    `true` sem aparelho real**.

### Passo 48 — os quatro gabaritos visuais (1–2 PRs)

11. **E02-12..E02-15** — derivam do pacote da E02-11, que já está fechada.

### Passo 49 — ADRs que M0 pede para congelar

Quatro ADRs ainda não estão aceitas por inteiro:

| ADR | Gate declarado | Situação medida |
|---|---|---|
| ADR-002 (audio core) | E01-01 / E01-02 | **aceito em parte em 2026-11-11**; o resíduo é a limitação de clock da E01-02, coberta pela E01-05 |
| ADR-003 (temas) | E02-01 | `proposto` — fecha no passo 47 |
| ADR-004 (extensões) | E07-01 | `proposto` — fecha no passo 46 |
| ADR-006 (multiplataforma) | M0 baselines | escopo aceito por instrução do usuário; tecnologias/baselines fecham na E14-01 (passo 45) |

Congelar é mudar `Status` para aceito com a evidência que o passo anterior mediu. Um ADR
que continua `proposto` depois do spike é um spike que ninguém leu.

## 6. Onde esta fase termina

M0 fecha quando:

1. as 21 TASKs de M0 estão `implemented`, cada uma com a evidência no lugar;
2. as 4 ADRs acima estão aceitas com a evidência citada;
3. `check-tasks` e `check-authoring` estão verdes com **199** TASKs;
4. o `STATE.md` é atualizado a partir do backlog, não escrito à mão.

Depois disso o roadmap manda M1 — a primeira vertical slice: abrir, tocar, seek, pause,
stop, formato básico, recuperação e disciplina de callback, nos cinco alvos. Isso é a
FASE 5, e não está neste plano.

## 7. O que eu preciso do mantenedor

Duas decisões. A primeira já estava na v1 e continua aberta porque nenhuma das opções é
minha para tomar; a segunda apareceu na medição da v2.1:

**Escopo de M0 nesta máquina.** Aceitar as três limitações de hardware (§4 — sem SDK
Android, sem Orca/VoiceOver, nenhum cross-linker instalado) como *"critério atendido
com limitação registrada"*, ou parar os itens que dependem delas e fechar M0 sem eles?

E uma segunda, que só apareceu na medida da v2.1 e é sua porque envolve instalar
toolchain nesta máquina:

**Cross-linker da E14-01.** `zig cc` cobre os cinco alvos com uma instalação só, mas
não é o linker nativo de nenhum deles; toolchain por SO (gcc cruzado para Linux,
`cargo-zigbuild`/osxcross para macOS, NDK para Android) é mais fiel e custa quatro
instalações e vários GB — o D: tem 80 GB livres. Qual caminho?

As duas decisões que a v1 também listava estão **resolvidas**: E01-02 fechou com a
limitação de clock declarada e correção na E01-05; E02-11 fechou apontando para
`vynamp-mvp-reference`.

**Recomendação, que não substitui a decisão:** aceitar as três limitações como
*"critério atendido com limitação registrada"* — é o que a E19-01 já fez com o
AC-3, e parar os itens deixaria M0 fechado por exclusão, que é mais fraco do que
M0 fechado com a lacuna escrita. Para o cross-linker, `zig cc`: uma instalação
cobre os cinco alvos, é reproduzível em qualquer máquina (o que o `cargo-zigbuild`
também dá) e o custo de não ser o linker nativo de cada um fica registrado na
matriz da E14-01 em vez de escondido. Toolchain por SO é a alternativa fiel e a
mais cara; se o mantenedor quiser fidelidade sobre portabilidade do procedimento,
é essa.

## 8. Riscos

| Risco | Mitigação |
|---|---|
| Spikes de áudio medidos por quem não ouve | a evidência numérica (hash, frames, p95) é o juiz; a validação acústica continua declarada como ausente |
| Android sem hardware gerar "prova" de papel | a TASK declara a limitação no `Then`; nenhum `application_ready` vira `true` sem aparelho real |
| A fase virar 40 PRs e perder o fio | passos 44–49, um PR por passo onde couber, `STATE.md` atualizado a cada passo |
| **A spec do dsp ser tratada como aprovada porque o código existe** | o estado na spec diz "rascunho autorado pelo spike, não aprovado"; aprovar é ato do mantenedor, por `GOVERNANCE.md` |

## 9. Diferenças em relação à v1

| Item | v1 (2026-11-09) | v2 (2026-11-11) |
|---|---|---|
| Backlog | 193 TASKs, 182 sem critérios | **199 TASKs, 0 sem critérios** |
| `libs/dsp` | 1 `.rs`, 6 LOC, stub | **11 `.rs`, 1 662 LOC, 21 testes** |
| E01-02, E02-11 | `in_progress`, a fechar | `implemented` |
| E04-02 | spike #8 da fila | `implemented`, fora da fila |
| Autoria de Given/When/Then | passo 41, 36 TASKs | **não existe mais** — já feito |
| M0 aberto | 17 a autorar + 13 spikes | **16 TASKs**, 34–81 dias |
| Decisões pendentes | 3 | **2** (escopo de M0 nesta máquina; cross-linker da E14-01) |
| Alvos Rust instalados | (v2 dizia: só `x86_64-pc-windows-msvc`) | **7**, medido; falta linker, não alvo |
| Mobile | ausente do plano | `D-MOB-01`/`D-MOB-02` registradas; nenhuma TASK nova |
