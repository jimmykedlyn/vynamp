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

- **Não** executa o soak de 8 h da máquina de referência nem publica a matriz de combinações por célula. Isso é `TASK-E10-08`, cuja AC-1 fixa a máquina (Windows 11 x64, 4 núcleos, 16 GiB, SSD, DAC USB 24/192 em shared e exclusivo) e cuja AC-4 publica uma célula por combinação com driver e build; aqui a matriz é entrada medida e citada, e refazê-la criaria dois donos do mesmo relatório de referência.
- **Não** implementa nem conserta o caminho que está sendo medido: WASAPI exclusive e perfis são `TASK-E04-05`, o modo transparente e os labels de cadeia são `TASK-E04-06`, dynamics/tone são `TASK-E04-07`, tempo/pitch são `TASK-E04-08` e o import de preset de fone é `TASK-E04-09`. Esta TASK mede o que eles entregaram; uma falha de 192 kHz vira achado no relatório, não patch de backend aqui.
- **Não** altera os níveis de `docs/support-matrix.md` nem promove linha de `docs/hardware-matrix.md`. A declaração de combinação validada é de `TASK-E04-06`, e `crates/vynamp-output/tests/support_matrix.rs` exige que o arquivo seja saída literal de `support_matrix_markdown()`; editar a célula à mão para refletir o relatório reprova esse teste e daria ao relatório poder de autoaprovação.
- **Não** constrói o harness numérico de comparação (oráculos PCM, resposta em FFT, delay/padding, crossfade, sample conversion). Ele é `TASK-E10-03`; aqui se consome o que ele expõe e se registra no relatório o que ainda faltar dele.
- **Não** instrumenta a callback nem monta histograma e baseline de performance. Métricas sanitizadas, histograma de duração e benchmarks são `TASK-E10-07`; sem o histograma dela o p99.9 da AC-2 fica não medido no relatório, e duplicar a coleta aqui faria dois p99.9 com definições diferentes.
- **Não** cobre DoP nem 768 kHz, que são `TASK-E12-04`, nem matriz de receiver, Atmos e passthrough, que são `TASK-E17-16`. Os dois são dependentes desta TASK e o teto de taxa da matriz daqui é o que `TASK-E04-05` e `TASK-E04-06` anunciarem no capability set, não uma taxa que a spec de hi-res mencione.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e04-10-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E04-10-AC-1` | uma matriz de combinações de saída com a taxa 100 kHz que o DAC não anuncia, 192 kHz anunciada em 24 bits, a troca de taxa entre 44,1, 96 e 192 kHz no meio do stream, e pedidos de 6 canais e de mapa 5.1 num dispositivo que declara só 2 canais | cada linha da matriz é aberta no backend com o pedido que ela declara e, ao fim de cada abertura, o valor devolvido é lido de Stream::config() ou do Error::FormatUnsupported e transcrito na linha do relatório | Testes100/192k/rate switch/channel mismatches |
| `TASK-E04-10-AC-2` | um soak de 8 h a 24 bits e 192 kHz com o período negociado anotado no cabeçalho, o histograma de duração de callback exportado por atomics fora da thread RT e a versão de driver de cada dispositivo usado registrada junto | o soak é encerrado e a análise lê o histograma exportado e o contador de alocações por thread, confrontando p99.9, underruns e alocações com o período negociado de cada combinação | callback budget e8hsoak |
| `TASK-E04-10-AC-3` | um relatório em que as combinações comprovadas apontam para captura ou medição própria com hash SHA-256, e as demais linhas de docs/hardware-matrix.md continuam em not run com o motivo escrito | cada linha do relatório é conferida contra o artefato da evidência que ela cita, e a claim de hi-res, de bit-perfect ou de suporte por plataforma é escrita só na linha que tem hash | claims limitados a combinations comprovadas. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | Em `D:/Projects/vynamp/libs/output` (o workspace raiz do umbrella tem `members = ["xtask"]` e exclui `libs/output`, entao rodar dali termina em `package ID specification vynamp-output-core did not match any packages`): `cargo test -p vynamp-output-core --test capture`, `--test negotiate`, `--test zero_alloc_device`, `cargo test -p vynamp-output --test support_matrix` e `cargo run -p vynamp-output --example support_matrix` conferido contra `docs/support-matrix.md`; a matriz de 100 kHz, de 192 kHz a 24 bits, de troca 44,1/96/192 e de 6 canais e 5.1 num dispositivo de 2 canais e uma fixture de capabilities que nao existe, e o soak exige um host de OS que nao existe no crate. | Hoje so ha evidencia de host virtual, medida agora: `capture.rs` passa 6 testes e compara I24Packed byte a byte contra `ChannelRamp` em 5 periodos de 64 frames a 48 kHz (manifest com `frames=320`, `valid_bits=24`); `zero_alloc_device.rs` mede 0 alocacoes na thread do dispositivo a 96 kHz com periodo de 32; `negotiate.rs` monta tabelas com 44 100, 96 000 e 192 000 e escolhe a taxa mais proxima para cima (pedido de 48 kHz contra capabilities de 44 100, 96 000 e 192 000 cai em 96 000). `docs/support-matrix.md` esta com wasapi, alsa, pipewire, pulse, coreaudio e aaudio todos em `-` e `docs/hardware-matrix.md` com as 7 linhas em `not run` e device/comando `(pending)`. `play_sine_null` nao serve de soak: abre `default_host()`, que e `NullHost` a 48 kHz por 200 ms (`hosts()` so devolve Null), logo nao alcanca DAC nenhum. Nenhum histograma existe (`histogram` nao aparece em `.rs` nenhum) e `PresentationTiming` publica so `device_frame`, `at`, `latency_frames` e `source`, entao p99.9 nao e calculavel ate `TASK-E10-07`. O maior run medido e o spike da TASK-E01-02, 30 s em WASAPI shared a 384 kHz: `probe-steady.json` com 3 022 callbacks, 0 underruns e `device_callback_p999_ns_upper_bound` 131 071 ns (1,31% do periodo de 10 ms); `probe-stall.json` com 3 052 callbacks, 25 underruns e o mesmo p999. O criterio pede p99.9 < 25% do periodo e 0 underruns no relatorio, e falta fixar se `100/192k` e 100 kHz como taxa de recusa mais 192 kHz anunciada, ou 96 e 192 kHz. |
| Hardware | um DAC USB real que anuncie 24/96 e 24/192 com 16/44,1 como base, o driver de cada sistema, e mais uma maquina Linux com ALSA hw, um Mac e um aparelho Android para as linhas multiplataforma de `docs/hardware-matrix.md`. Neste ambiente Windows 11 o unico endpoint de render com DeviceState ACTIVE na chave MMDevices\Audio\Render e o onboard da `Conexant ISST Audio` (`Alto-falante`); o `Headset (USB 384KZ HIFI Audio)` esta NOTPRESENT e o `Audio Remoto` e endpoint de software, sem DAC. 24/96, 24/192 e modo exclusivo nunca foram medidos aqui: o spike da TASK-E01-02 rodou em outro checkout, por CPAL, em shared a 384 kHz. Nao ha Linux, macOS nem Android, e o crate nao tem host WASAPI para negociar 192 kHz. | ausência é limitação registrada, não aprovação |

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
