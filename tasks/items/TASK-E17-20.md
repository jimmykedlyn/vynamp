# TASK-E17-20 — Fechar gate combinado de codecs, DSP e visualizers

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E17-20` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E17-20` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M3 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E17-02`, `TASK-E17-03`, `TASK-E17-04`, `TASK-E17-05`, `TASK-E17-06`, `TASK-E17-08`, `TASK-E17-10`, `TASK-E17-11`, `TASK-E17-12`, `TASK-E17-19`, `TASK-E16-07` |
| Bloqueia | `TASK-E14-06` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Evidência por build/alvo do corpus comum, condicionais, cadeia final e visualização.

## Boundary

- **Não** gera nem corrige o manifest de capacidades por artefato: o dono é `TASK-E17-12` e o gate só consome o que ele publica. Ajustar o manifest aqui seria aprovar o próprio insumo.
- **Não** fecha o release 1.0 multiplataforma. `TASK-E14-06` agrega M1–M3, acessibilidade e zero P0/P1 nos cinco alvos; esta TASK alimenta esse gate com evidência de codecs, DSP e visualização, e não o substitui.
- **Não** executa o corpus de saída por OS/ABI — clock, seek, drain, gapless e o shared/exclusive dos backends — nem gera o corpus que roda. Execução por alvo desse corpus é `TASK-E14-04`, e fixtures, hashes e licenças são `TASK-E10-01`; alvo ausente se declara como limitação, não se substitui por emulador nem por cross-compile, e regerar o corpus aqui criaria um segundo dono dos mesmos hashes e um segundo oráculo numérico para o mesmo backend.
- **Não** implementa o guard true-peak nem o harness que mede o teto: o guard após mix/master/resampler é `TASK-E17-07` e a medição no buffer do backend é `TASK-E17-08`; aqui a QA-027 é anexada, não construída.
- **Não** implementa a política de proxy e a rede isolada: `TASK-E17-11` entrega os adapters system/manual/direct, o vault e as fixtures 407/DNS/redirect; a QA-029 entra neste gate por leitura do resultado dele.
- **Não** certifica ativação e associações de shell nos cinco alvos: `TASK-E16-07` é o dono, e as QAs 022/023/024 entram aqui por referência, não são reexecutadas neste gate.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e17-20-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E17-20-AC-1` | os cinco alvos reais do release candidato — Windows x64, Linux x64, macOS Intel, macOS Silicon e Android arm64 — com os oito artefatos de `delivery/04-gates.md` construídos no mesmo SHA e o corpus comum do M3 | a matriz do gate roda o corpus de codecs, DSP e visualização em cada alvo e publica o resultado por build e por alvo | Cinco alvos passam required M3 |
| `TASK-E17-20-AC-2` | as 11 entradas `conditional` de fase M3 do catálogo — `flac-mp4`, `mp3-mp4`, `aac-latm`, `aac-adif`, `aac-usac`, `opus-mp4`, `ts-audio`, `mov-audio`, `wma-pro`, `bt-classic` e `bt-le` | o relatório do gate lista cada condicional com spike, fixture e limitação por alvo, e com o motivo explícito quando o adapter não é distribuído | conditional reportado honestamente |
| `TASK-E17-20-AC-3` | as 24 entradas M5 do catálogo e os recursos de provider que o roadmap separa como M4 | o grafo de dependências do gate é percorrido e cada job, fixture e QA é confrontado com a fase da feature que ele exige | nenhum gate depende de features M4/M5 |
| `TASK-E17-20-AC-4` | os seis QA do gate, QA-025 a QA-030, mais os limites que a 1.0 declara: PCM até 384 kHz em H-07, as cinco taxas 44,1, 48, 96, 192 e 384 kHz, 16, 24 e 32 bits com e sem dither, mono, estéreo e 5.1 canais, e o lookahead do guard true-peak em amostras e em milissegundos por taxa | o relatório final é publicado com o resultado por QA e por alvo, junto da lista de limites e das ausências declaradas | QA e limites publicados. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `cargo test --locked` em `libs/decode` (medido em 2026-11-10: 508 verdes, 3 ignorados, 47 suítes; o tempo de parede variou de 79 a 110 s entre duas execuções na mesma máquina, então o número estável é a contagem, não o relógio) e, no umbrella, `cargo xtask ci detect` e `cargo xtask ci gate`, que é onde a evidência por alvo teria de aparecer | esperado: os cinco alvos com o corpus comum do M3, os 11 condicionais com limitação por alvo e os seis QA publicados — nada disso existe hoje: `ci detect` responde `{"core":"false","android":"false"}` e `ci gate` para em `Missing configuration: DOCS` fora do workflow, `libs/dsp/src/lib.rs` tem 6 linhas e só a constante `VERSION`, `apps/player/src/main.rs` imprime `vynamp_player 0.1.0` e não há visualizer, corpus por alvo nem relatório QA-025 a QA-030; `libs/dsp` tem 1 arquivo `.rs` e nenhum teste, e `quality.yml` declara que os runners `vynamp-hardware` e `vynamp-android-device` não estão provisionados |
| Hardware | ausente: a AC-1 pede os cinco alvos e esta máquina é Windows 11 x64; os alvos `rustup` de macOS e Android estão instalados mas não há linker cruzado (`which cc gcc clang ld` vazio, sem cargo-ndk nem SDK/NDK Android em ANDROID_HOME), sem aparelho Android arm64 e sem máquina macOS; `quality.yml` manda a matriz de hardware para os runners self-hosted `vynamp-hardware` e `vynamp-android-device`, que a linha de preflight do próprio workflow declara não provisionados (TASK-E15-05) | ausência é limitação registrada, não aprovação |

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
