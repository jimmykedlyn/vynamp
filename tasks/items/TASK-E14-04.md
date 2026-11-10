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
- **Não** entrega o aceite Android 1.0 em hardware nem o lifecycle/focus/energia. O aceite de formatos comuns, DSP, HLS, TalkBack e energia é `TASK-E13-10`, e checkpoint/reconexão/wake locks são `TASK-E13-07`; aqui o aparelho Android entra apenas como alvo de execução do mesmo corpus de saída.
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
| `TASK-E14-04-AC-1` | um Mac Intel (x86_64-apple-darwin) e um Mac Apple Silicon (aarch64-apple-darwin) reais, cada um com o build nativo da própria arquitetura, CoreAudio e um DAC pela rota do sistema, e o corpus lossless/gapless de tests/fixtures/audio | o mesmo corpus é executado em cada máquina separadamente, com o host coreaudio e a arquitetura do processo registrados em cada log (sysctl.proc_translated igual a 0, sem Rosetta), e cada execução vira uma linha própria de libs/output/docs/hardware-matrix.md com device, driver, ABI, comando, commit e hash da captura digital | Mac Intel e arm64 testados separadamente |
| `TASK-E14-04-AC-2` | um Linux x86_64 nativo (Ubuntu 24.04, com PipeWire e ALSA disponíveis) e um aparelho Android arm64 real com AAudio, sem emulador, ambos com DAC ou saída USB pela rota do sistema, e nenhum dos dois rodando o binário por Wine ou shim de API | o mesmo corpus roda nas duas máquinas com o host pipewire/alsa e o host aaudio, e no Android também os cenários de focus, noisy, Bluetooth, USB, tela apagada e disconnect, com o resultado de cada host virando linha de hardware-matrix.md com OS, ABI e device nomeados | Linux nativo e Android real |
| `TASK-E14-04-AC-3` | um par gapless de duas faixas lossless cujo conteúdo termina no frame 352, isto é 32 frames depois do fim do período 5 de 64 frames — a mesma montagem do teste drain_completes_exactly_after_the_last_content_frame em crates/vynamp-output-core/tests/stream.rs — com o Capture host em 2 canais e 64 frames por período e o oráculo ChannelRamp | a faixa toca até o fim, drain roda com timeout de 5 s, a faixa seguinte entra em sequência imediatamente depois do último frame de conteúdo, e os bytes entregues são comparados bit a bit com o oráculo no arquivo de captura e no seu .manifest | nenhum fim cortado/buffer antigo |
| `TASK-E14-04-AC-4` | um endpoint cujo exclusive/direct não foi negociado — a linha do host ainda em '-' em libs/output/docs/support-matrix.md e a capability de ShareMode::Exclusive não declarada pelo backend — com o app abrindo o stream pedindo exclusive | a abertura termina, e o ShareMode efetivo, o rótulo source/output e o motivo da recusa são lidos na UI e no relatório da execução, e a linha do alvo é escrita em hardware-matrix.md como captura em shared, sem nenhuma linha desse alvo marcada como exclusive ou bit-perfect | exclusive não presumido. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `cargo test -p vynamp-output-core --features capture --test capture` e `cargo test -p vynamp-output-core --test stream` como linha de base nesta máquina, e, em cada alvo, a mesma lista de fixtures de `tests/fixtures/audio/` executada pelo app com captura digital em arquivo, comparando o `<path>.manifest` (format, rate, channels, period_frames, frames, overruns) e os bytes capturados contra o oráculo `ChannelRamp`; cada execução vira uma linha de `libs/output/docs/hardware-matrix.md` com device, driver, OS, ABI, comando, commit e hash da captura | Hoje o que existe é só o plano: `libs/output` tem 42 arquivos `.rs` em 3 crates (core, facade, testkit), e os únicos hosts reais são `null` e `capture` — `docs/support-matrix.md` lista wasapi, alsa, pipewire, pulse, coreaudio e aaudio com `-` em todas as colunas. As 7 linhas de `docs/hardware-matrix.md` estão `not run`, com device `(pending)` e comando `(pending)`, e a linha de macOS diz `not run (no Mac agent)`. `drain` existe e tem teste (`crates/vynamp-output-core/src/stream.rs:173`; drain_completes_exactly_after_the_last_content_frame espera 7 períodos entregues e 0 source_underruns), mas não há `seek` em nenhum crate de output (grep por `fn seek` volta vazio), não há par gapless no corpus (`TASK-E10-01` registra a lacuna) e `apps/player` tem 1 arquivo `.rs`, sem player que rode o corpus. `config/project.json` declara 6 alvos, `PLATFORMS` em `xtask/src/release.rs` tem 5 e `check_host` em `xtask/src/util.rs:384` aceita só windows-x64, linux-x64, macos-universal e android-arm64, sem gate por arquitetura Mac. Depois da TASK, a evidência exigida é: uma linha por configuração com device, driver, commit e hash reais no lugar de `(pending)`; duas linhas macOS distintas com ABI nomeada e `proc_translated=0`; uma captura cujo último frame é exatamente o frame 352, com `frames` e `overruns` batendo no manifesto; e nenhuma linha de alvo aparecendo como exclusive sem captura bit-exata |
| Hardware | Mac Intel (x86_64), Mac Apple Silicon, Linux x86_64 nativo com DAC USB e aparelho Android arm64 real, todos com DAC ou interface de captura digital; nenhum deles existe neste ambiente, que é Windows 11 x86_64 (PROCESSOR_ARCHITECTURE=AMD64) sem placa pro, sem aparelho Android, sem aparelho iOS e sem máquina macOS — a metade Windows do corpus roda aqui, e as outras quatro configurações ficam registradas como limitação | ausência é limitação registrada, não aprovação |

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
