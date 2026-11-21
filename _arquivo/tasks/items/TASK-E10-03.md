# TASK-E10-03 — Criar harness numérico de áudio

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E10-03` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E10-03` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M2 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E04-03`, `TASK-E10-01` |
| Bloqueia | `TASK-E10-07`, `TASK-E17-08` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–12 dias úteis |
| Risco | médio |

## Entrega

Oráculos PCM, resposta FFT, delay/padding, crossfade e sample conversion.

## Boundary

- **Não** gera nem versiona o corpus, os hashes e as licenças das fixtures: isso é `TASK-E10-01`, que já entregou `MANIFEST.txt` com 13 linhas em pcm, 31 em flac e 33 em mpeg. O harness consome essas linhas e reprova quando o hash muda; reescrever a fixture aqui criaria um segundo dono do corpus.
- **Não** implementa o crossfade nem o grafo DSP que ele mede: o equal-power crossfade é entrega de `TASK-E04-03`, dependência desta TASK. Aqui só existe a resposta esperada e o detector de descontinuidade; implementar o crossfade no harness faria o oráculo testar a si mesmo.
- **Não** fixa o teto true-peak do buffer final nem estima dBTP: isso é `TASK-E17-08`, que esta TASK bloqueia e que mede o PCM quantizado do backend. Um estimador de true-peak dentro do harness numérico seria a segunda implementação da mesma medida.
- **Não** mede throughput, p99.9 da callback nem tempo de launch: `TASK-E10-07` é quem instrumenta e publica baseline. O harness compara amostras, não tempo, e transformar contagem de amostras em benchmark daria números sem ambiente registrado.
- **Não** decide a qualidade do resampler nem o packing PCM24/32 do produto: o ADR e as metas de aliasing são de `TASK-E04-02`. Aqui só se afirma o que a conversão produziu, amostra por amostra, contra a referência independente.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e10-03-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E10-03-AC-1` | um WAV de referência de 48 kHz estéreo com 4800 frames em `libs/decode/tests/fixtures/pcm/sine-s16le.wav` — 997 Hz à esquerda e 1499 Hz à direita — e quatro cópias adulteradas: uma com um frame extra no fim, uma com 576 amostras de encoder delay trimadas a menos no início, uma com os canais 0 e 1 trocados, e uma derivada de `libs/decode/tests/fixtures/pcm/sine-f32le.wav` com NaN na amostra 1200 do canal 0, porque s16 não representa NaN | o comparador do harness roda sobre cada cópia contra a referência e emite, por defeito, a primeira divergência com índice de amostra, canal, valor esperado e valor observado | Detectar frame extra, trim incorreto, mapa de canais errado e NaN |
| `TASK-E10-03-AC-2` | o oráculo calculado fora do código sob teste: um seno de 440 Hz a 48 kHz amostrado por `f64::sin` da std, uma FFT radix-2 escrita no próprio harness e uma `Track` sintética que declara 4800 frames com delay 576 e padding 2400, no molde do demuxer sintético de `libs/decode/crates/vynamp-decode/tests/facade.rs`, porque STREAMINFO de FLAC não carrega delay nem padding | o harness alimenta `vynamp-decode` e o grafo de `libs/engine` com o PCM sintetizado e com a `Track` de delay/padding e compara amostra a amostra, bin a bin e frame a frame contra esse oráculo, com a tolerância declarada por caso | referência independente da implementação sob teste. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `cargo test --manifest-path libs/decode/Cargo.toml --locked` mais o crate novo do harness rodado sobre quatro cópias adulteradas do WAV de 4800 frames, cada adulteração revertida com `git checkout --` depois do registro do exit code | Hoje o oráculo numérico existe só no repo `vynamp-decode` e só sobre o corpus já decodificado: `oracle_pcm.rs`, `oracle_flac.rs` e `oracle_mpeg.rs` em `libs/decode/crates/vynamp-decode/tests/` leem as 13/31/33 linhas dos manifestos e comparam cada amostra com `tools/fixtures/source-sine.wav`, e `oracle_pcm.rs:161` exige `rows >= 12`. Não existe oráculo de FFT (`grep -rli fft --include=*.rs libs` não acha nada), nem código de crossfade (`grep -rli crossfade --include=*.rs libs` também não), e `libs/dsp/src/lib.rs` e `libs/engine/src/lib.rs` são só `pub const VERSION`. A evidência a produzir: cada um dos quatro defeitos falha nomeando amostra, canal e valor; um trim errado aparece como diferença de exatamente 576 amostras; a resposta em frequência fecha em ±0,2 dB contra a referência, tolerância de QA-005. Nada disso está medido hoje. |
| Hardware | nenhum: o harness compara arrays de amostras em memória e o corpus é WAV/FLAC versionado em `libs/decode/tests/fixtures/`; nenhum caso abre dispositivo de áudio, e a captura do backend é o host `Capture` de `vynamp-output-core`, que grava bytes em arquivo e um `.manifest` key=value | ausência é limitação registrada, não aprovação |

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
