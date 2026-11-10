# TASK-E04-10 — Validar qualidade numérica e hardware matrix

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E04-10` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E04-10` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E04-06`, `TASK-E04-07`, `TASK-E04-08`, `TASK-E04-09` |
| Bloqueia | `TASK-E10-08`, `TASK-E12-04`, `TASK-E12-07`, `TASK-E12-10`, `TASK-E13-10`, `TASK-E17-16` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | alto |

## Entrega

Relatório DSP/output com fixtures, driver versions e combinations.

## Boundary

- **Não** executa o soak de 8 h do ambiente de referência com DAC removido, driver ocupado, sleep/resume e stress de CPU. O drill é `TASK-E10-08`; aqui o soak aparece como uma condição medida e citada no relatório, e refazê-lo criaria dois donos do mesmo número de underruns de referência.
- **Não** implementa nem conserta o caminho que está sendo medido: WASAPI exclusive e perfis são `TASK-E04-05`, o modo transparente e os labels de cadeia são `TASK-E04-06`, dynamics/tone são `TASK-E04-07`, tempo/pitch são `TASK-E04-08` e o import de preset de fone é `TASK-E04-09`. Esta TASK mede o que eles entregaram; uma falha de 192 kHz vira achado no relatório, não patch de backend aqui.
- **Não** altera os níveis de verificação declarados em `docs/support-matrix.md` nem promove linha de `docs/hardware-matrix.md`. A declaração de combinação validada é de `TASK-E04-06`, e o teste `crates/vynamp-output/tests/support_matrix.rs` exige que o arquivo seja saída literal do gerador; editar a célula à mão para refletir o relatório quebraria esse teste e daria ao relatório poder de autoaprovação.
- **Não** constrói o harness numérico de comparação (oráculos PCM, resposta em FFT, delay/padding, crossfade, sample conversion). Ele é `TASK-E10-03`; aqui se consome o que ele expõe e se registra no relatório, o que ainda faltar dele.
- **Não** instrumenta a callback nem monta histograma e baseline de performance. Métricas sanitizadas, histograma e benchmarks são `TASK-E10-07`; duplicar a coleta aqui faria dois histogramas com definições de p99.9 potencialmente diferentes.
- **Não** cobre DSD nativo, DoP e 768 kHz, nem matriz de receiver, Atmos ou passthrough. Isso é `TASK-E12-04` e `TASK-E17-16`, ambos dependentes desta TASK; a matriz daqui é PCM até a taxa que `TASK-E04-05` e `TASK-E04-06` entregarem.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e04-10-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E04-10-AC-1` | uma matriz de combinações de saída com a taxa 100 kHz que o DAC não anuncia, 192 kHz anunciada em 24 bits, a troca de taxa entre 44,1, 96 e 192 kHz no meio do stream, e pedidos de 6 canais e de mapa 5.1 num dispositivo que declara só 2 canais | cada combinação da matriz é executada contra o backend e o relatório registra taxa, formato e canais realmente negociados, ou o motivo exato da recusa quando a negociação falha | Testes100/192k/rate switch/channel mismatches |
| `TASK-E04-10-AC-2` | um soak de 8 h a 24 bits e 192 kHz com período negociado anotado, o histograma de callback exportado por atomics fora da thread RT e a versão de driver de cada dispositivo usado registrada no cabeçalho | o soak termina e o relatório compara o p99.9 da callback com o período negociado, soma underruns e alocações na thread do dispositivo e publica o budget por combinação | callback budget e8hsoak |
| `TASK-E04-10-AC-3` | um relatório em que as combinações comprovadas têm captura ou medição própria com hash, e as demais linhas de docs/hardware-matrix.md continuam em not run | o relatório é conferido linha por linha contra as evidências e as claims de hi-res, de bit-perfect e de suporte por plataforma são escritas a partir dele | claims limitados a combinations comprovadas. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `cargo test -p vynamp-output-core --test capture` e `--test negotiate` e `--test zero_alloc_device` em `libs/output`, `cargo run -p vynamp-output --example support_matrix` conferido contra `docs/support-matrix.md` por `cargo test -p vynamp-output --test support_matrix`, e o soak executado com `cargo run -p vynamp-output --example play_sine_null --release` numa máquina com o DAC ligado | Hoje só há evidência dos hosts virtuais: `capture.rs` compara I24Packed byte a byte contra `ChannelRamp` em 5 períodos de 64 frames a 48 kHz (manifest com `frames=320`, `valid_bits=24`), `zero_alloc_device.rs` mede 0 alocações na thread do dispositivo a 96 kHz com período de 32, e `negotiate.rs` já monta tabelas com 44 100, 96 000 e 192 000 escolhendo a taxa mais próxima para cima. `docs/support-matrix.md` está com wasapi, alsa, pipewire, pulse, coreaudio e aaudio todos em `-` e `docs/hardware-matrix.md` com as 7 linhas em `not run`, então 192 kHz em hardware, troca de taxa e mismatch de canais não têm hoje onde ser provados — não existe host WASAPI no crate. Não existe soak de 8 h: o maior run registrado é o spike da TASK-E01-02, 3 052 callbacks em WASAPI shared a 384 kHz com p99.9 ≤ 131,1 µs (1,31% do período de 10 ms), 0 alocações e 0 underruns sem stall. O critério pede p99.9 < 25% do período e 0 underruns no relatório; falta ainda fixar se `100/192k` significa 100 kHz como taxa de recusa e 192 kHz anunciada, ou 96 e 192 kHz — hoje só a segunda leitura tem fixture. |
| Hardware | um DAC USB real que anuncie 24/96 e 24/192 (com 16/44,1 como base) e o driver de cada sistema, mais máquina Linux com ALSA hw, um Mac e um aparelho Android para as linhas multiplataforma; este ambiente é Windows 11 e só existe o `Headset (USB 384KZ HIFI Audio)` que o spike da TASK-E01-02 mediu em WASAPI shared a 384 kHz, sem exclusivo nem 24/192 medidos, sem Linux, macOS ou Android, e sem host WASAPI no crate para negociar 192 kHz | ausência é limitação registrada, não aprovação |

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
