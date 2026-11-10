# TASK-E14-04 — Executar corpus e outputs por alvo

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E14-04` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E14-04` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M3 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E12-01`, `TASK-E12-02`, `TASK-E13-07`, `TASK-E04-06` |
| Bloqueia | `TASK-E10-08`, `TASK-E14-06`, `TASK-E17-10` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | alto |

## Entrega

Fixtures e hardware por OS/ABI, com clock/seek/drain/gapless dos backends.

## Boundary

- **Não** entrega o backend CoreAudio, a UI/acessibilidade macOS nem o DMG/ZIP Universal2. Mac Intel e Mac Apple Silicon são construídos e assinados em `TASK-E12-02`; aqui as duas arquiteturas são apenas executadas e medidas, e confundir as duas coisas levaria esta TASK a resolver build e notarização no meio da execução do corpus.
- **Não** entrega a porta Linux x86_64, o PipeWire/PulseAudio, o MPRIS, o Orca ou o pacote `.deb`/AppImage. Tudo isso é `TASK-E12-01`; esta TASK só roda o corpus no binário Linux que aquela entrega e registra o host efetivo de cada execução.
- **Não** entrega o AudioManager, o audio focus, o noisy nem o roteamento Bluetooth do Android, nem o lifecycle de energia. Foco, ruído e rota são `TASK-E13-03`, dona do AudioManager e do output clock; checkpoint, reconexão e wake locks são `TASK-E13-07`, e o aceite Android 1.0 em hardware que lê tudo isso é `TASK-E13-10`. Aqui o aparelho entra só como alvo de execução do corpus, e trazer o tratamento de foco para cá faria três donos do mesmo callback.
- **Não** gera o corpus nem os pares gapless. Fixtures, hashes, licenças e manifestos são `TASK-E10-01`, e nenhum par gapless existe hoje — o corpus versionado cobre tons e variantes de codec. Esta TASK consome o corpus; gerá-lo aqui criaria um segundo dono dos mesmos hashes.
- **Não** implementa o modo transparente, o integer direct path, os rótulos source/output nem as combinações validadas. Isso é `TASK-E04-06`, dependência direta desta TASK; aqui se prova que o comportamento medido bate com o que aquela TASK declara, não se constrói o caminho transparente.
- **Não** monta a matriz de evidência agregada de release nem decide o gate de stable multiplataforma. A agregação por alvo e o bloqueio de stable estão em `TASK-E14-06`, e a matriz de hardware do soak de 8 h com unplug e stress de CPU é `TASK-E10-08`; esta TASK só produz as linhas por alvo que as duas leem.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e14-04-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E14-04-AC-1` | um Mac Intel (x86_64-apple-darwin) e um Mac Apple Silicon (aarch64-apple-darwin) reais, cada um com o build nativo da própria arquitetura, CoreAudio e um DAC pela rota do sistema, e o corpus versionado que o repo de fato tem em `libs/decode/tests/fixtures/` — 13 arquivos em `pcm/` (WAV, RF64, W64, AIFF e AIFC), 22 FLAC em `flac/` com 9 WAV de referência em `ref/` e 66 MPEG em `mpeg/` — sem nenhum par gapless, porque `tests/fixtures/audio/` é só uma linha de path reservado em `tests/fixtures/README.md` | o mesmo corpus é executado em cada máquina separadamente, com o host coreaudio e a arquitetura do processo registrados em cada log (sysctl.proc_translated igual a 0, sem Rosetta), e cada execução vira uma linha própria de libs/output/docs/hardware-matrix.md com device, driver, ABI, comando, commit e hash da captura digital | Mac Intel e arm64 testados separadamente |
| `TASK-E14-04-AC-2` | um Linux x86_64 nativo (Ubuntu 24.04, acima do baseline 22.04 de `platforms/01-matriz-suporte.md`, com PipeWire e ALSA instalados) e um aparelho Android arm64 real com AAudio, sem emulador, ambos com DAC ou saída USB pela rota do sistema, e nenhum dos dois rodando o binário por Wine ou shim de API | o mesmo corpus roda nas duas máquinas com o host pipewire/alsa e o host aaudio, e no Android também os cenários de focus, noisy, Bluetooth, USB, tela apagada e disconnect disparados pelo AudioManager do sistema, com o resultado de cada host virando linha de hardware-matrix.md com OS, ABI e device nomeados | Linux nativo e Android real |
| `TASK-E14-04-AC-3` | uma faixa de conteúdo de exatamente 352 frames — 5 períodos de 64 frames mais 32 frames dentro do período 5, a geometria de drain_completes_exactly_after_the_last_content_frame em `crates/vynamp-output-core/tests/stream.rs:227` com `StopAfter` — seguida por uma segunda faixa lossless que entra logo depois, com o CaptureHost em 2 canais e 64 frames por período e o oráculo ChannelRamp; esse par de faixas não existe hoje e a lacuna é registrada por `TASK-E10-01` | o stream toca até o fim e `drain(Duration::from_secs(5))` devolve `Drained` com 0 source_underruns: na montagem de faixa única do teste saem 7 períodos, 448 frames com os 96 últimos de silêncio depois do frame de conteúdo 351, e no par gapless a segunda faixa começa no offset de frame 352, sem silêncio inserido nem amostra da faixa anterior repetida; os bytes capturados são comparados bit a bit com o `ChannelRamp` no arquivo de captura e no seu `<path>.manifest` | nenhum fim cortado/buffer antigo |
| `TASK-E14-04-AC-4` | um endpoint cujo exclusive/direct não foi negociado — a linha do host ainda em `-` em `libs/output/docs/support-matrix.md`, `support()` de `crates/vynamp-output-core/src/support.rs:85` devolvendo `None` para ele e o backend sem nenhuma entrada `ShareMode::Exclusive`, que só existe no `default_capability_entries` do Null/Capture em `null.rs:281` — com o app abrindo o stream pedindo exclusive | a abertura termina, `stream.config().mode` mostra o modo efetivo, o rótulo source/output e o motivo da recusa (`Error::ExclusiveUnavailable { reason }`, `Error::FormatUnsupported` ou `Event::Downgraded`) são lidos no relatório da execução, e a linha do alvo é escrita em hardware-matrix.md como captura em shared, sem nenhuma linha desse alvo marcada como exclusive ou bit-perfect | exclusive não presumido. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `cd libs/output && cargo test -p vynamp-output-core --features capture --test capture` e `cargo test -p vynamp-output-core --test stream` como linha de base nesta máquina (medido em 2026-11-10: 6 e 10 testes passando, 0 falhas) e, em cada alvo, a lista de fixtures de `libs/decode/tests/fixtures/` executada pelo app com captura digital em arquivo, comparando o `<path>.manifest` (format, rate, channels, period_frames, frames, overruns) e os bytes capturados contra o oráculo `ChannelRamp`; cada execução vira uma linha de `libs/output/docs/hardware-matrix.md` com device, driver, OS, ABI, comando, commit e hash da captura | Hoje só existe o plano: `libs/output` tem 42 arquivos `.rs`, 40 deles em 3 crates (core 29, facade 7, testkit 4) e 2 em `tools/output-probe`; os únicos hosts são `null` e `capture`, e `docs/support-matrix.md` lista wasapi, alsa, pipewire, pulse, coreaudio e aaudio com `-` nas seis colunas porque `support()` (`support.rs:85`) devolve `None` para os seis. As 7 linhas de `docs/hardware-matrix.md` estão `not run`, com device e comando `(pending)`, e a de macOS diz `not run (no Mac agent)`. `drain` existe (`crates/vynamp-output-core/src/stream.rs:173`) e `tests/stream.rs:227` espera 7 períodos entregues e 0 source_underruns; não há `seek` em crate de output (grep por `fn seek` volta vazio), não há par gapless no corpus (`TASK-E10-01` registra a lacuna) e `apps/player/src/main.rs` é um `println!` de 3 linhas, sem player nem UI que rode o corpus. `config/project.json` declara 6 alvos e `xtask/src/release.rs:7` lista 5 em `PLATFORMS`, mas `check_host` em `xtask/src/util.rs:384` aceita windows-x64, linux-x64, macos-universal e android-arm64 — nenhum dos nomes `macos-intel`/`macos-silicon` que `PLATFORMS` usa, então não há gate por arquitetura Mac. Depois da TASK, a evidência exigida é: uma linha por configuração com device, driver, commit e hash reais no lugar de `(pending)`; duas linhas macOS distintas com ABI nomeada e `proc_translated=0`; na fixture de 352 frames uma captura com `frames=448` (7 períodos de 64), último frame de conteúdo no índice 351 e `overruns=0` batendo no manifesto; e nenhuma linha de alvo marcada como exclusive sem captura bit-exata |
| Hardware | Mac Intel (x86_64), Mac Apple Silicon, Linux x86_64 nativo com DAC USB e aparelho Android arm64 real: nenhum dos quatro existe aqui. Esta máquina é Windows 11 x86_64 (PROCESSOR_ARCHITECTURE=AMD64) com três endpoints onboard (Conexant ISST Audio, Áudio Intel para telas e headset USB 384KZ HIFI Audio), nenhum com captura digital, e nem a linha Windows roda hoje — `hosts()` em `libs/output/crates/vynamp-output/src/lib.rs:74` devolve só `NullHost`, então não há backend WASAPI para abrir placa nenhuma. O WSL Ubuntu existente é x86_64, sem PipeWire nem ALSA instalados e sem dispositivo de áudio, portanto não substitui o Linux nativo | ausência é limitação registrada, não aprovação |

## DoD — pronto para review

- [ ] Todos os critérios de aceite com evidência
- [ ] `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` limpos
- [ ] `cargo test` verde; testes ignorados listados com motivo
- [ ] Spec do módulo atualizada se a implementação revelou algo novo
- [ ] `backlog.json` com o estado correto
- [ ] Revisão feita por contexto limpo, não pelo implementador
- [ ] PR aberto com template preenchido; CI Gate verde no SHA final
## Notas de execução

Preenchido durante a execução, não antes.
