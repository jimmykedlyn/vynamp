# TASK-E17-18 — Certificar expansão MIDI, trackers e DSD especial

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E17-18` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E17-18` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M5 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E12-05`, `TASK-E12-04` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Fixtures de subsongs/loops/SF2/WavPack DSD/DST e native/DoP após implementações E12. Extração SACD ISO legal não protegida, sem handler ISO genérico.

## Boundary

- **Não** implementa os adapters libopenmpt, o synth MIDI nem a escolha do SF2 do usuário. Isso é `TASK-E12-05`, onde a matriz por perfil e licença é construída e o helper nativo é fixado; aqui a mesma matriz é consumida para certificar, e escrever um segundo synth colocaria dois donos na licença do backend.
- **Não** implementa o packing DoP, o caminho DSD nativo por DAC nem o PCM a 768 kHz. `TASK-E12-04` é a dona: `libs/output/crates/vynamp-output-core/src/dop.rs` e os 11 testes de `tests/dop.rs` já são dela, e esta TASK só compara os quatro caminhos de DSD com o que aquela entrega declara.
- **Não** decide o selo lossless de WavPack nem o caso de WV sem .wvc ou com correção inválida. O selo de inteiro sample-exact, os níveis híbridos e o .wvc ausente são `TASK-E17-05`; o WavPack DSD citado na Entrega entra aqui só como fixture de DSD, e emitir selo daqui criaria dois donos do mesmo veredito.
- **Não** publica o manifest de capacidades por artefato nem converte `planned` em disponível. `TASK-E17-12` gera o manifest com backend, feature, licença e hash; a entrada `sacd-iso` de config/format-catalog.json tem `association_allowed` igual a false, e é naquele manifest que a associação é negada, não nesta certificação.
- **Não** roda o gate combinado de codecs, DSP e visualizers nos cinco alvos. `TASK-E17-20` fecha esse gate e exige que nenhum gate 1.0 dependa de feature M4/M5; a evidência desta TASK é M5 e não pode virar condição do gate de 1.0.
- **Não** põe libopenmpt, o synth de SF2 nem o extrator de SACD ISO dentro do helper nativo isolado. Sandbox de OS, RPC autenticado e consentimento de plugin nativo são `TASK-E07-05`; aqui o backend é o adapter que `TASK-E12-05` entregou, e trazer o isolamento de processo para cá levaria a TASK a especificar sandbox no meio da certificação.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e17-18-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E17-18-AC-1` | as fixtures .mid, .rmi, .xmf e .kar em tests/fixtures/midi com SOURCE.txt e licença, o synth que TASK-E12-05 entrega ligado, e a entrada `midi` de config/format-catalog.json com operations `demux` e `render` e sem `decode` | cada fixture é aberta pelo probe e o registro de decoders de vynamp-decode-core é consultado para o codec id `midi-events`, comparando o tuple devolvido pelo probe com a linha do catálogo e com o que `CodecDescriptor` declara | Rendering MIDI não é decode PCM |
| `TASK-E17-18-AC-2` | um SF2 fornecido pelo usuário em tests/fixtures/sf2 acompanhado de SOURCE.txt e do arquivo de licença, e um segundo caso em que nenhum SF2 é passado ao synth | um .mid de cada perfil é aberto para render e, nos dois casos, o resultado da abertura, o caminho absoluto do SF2 usado e o id de licença registrado na capability são lidos | SoundFont licenciado explícito |
| `TASK-E17-18-AC-3` | um MOD cuja ordem salta para a própria posição em loop infinito, um XM no máximo de vozes do formato e um MIDI de trilha longa, todos abertos com os `Limits` de vynamp-decode-core (max_bytes 64 GiB, max_frames 66 355 200 000, max_channels 32, max_sample_rate 768 000) | o render de cada arquivo é conduzido até o limite de loops e o de frames, e as contagens de loops executados, de vozes simultâneas e de bytes alocados por período são registradas junto do erro ou da parada | loops/memória bound |
| `TASK-E17-18-AC-4` | quatro entradas distintas — um DSF DSD64 raw, um DFF com frames DST, o PCM decimado do mesmo DSF e um stream DoP aberto com `SampleFormat::Dop` em dsd_rate 2 822 400 — mais duas imagens SACD ISO, uma obtida legalmente e não protegida e outra com área protegida | cada uma das quatro entradas é aberta e o caminho de saída escolhido é registrado, e as duas ISOs passam pela mesma abertura, lendo a capability de `dsd-pcm`, `dsd-dst`, `dsd-native` e `sacd-iso` em cada resultado | DSD raw/DST/PCM/DoP distinguíveis. SACD ISO separa extração raw/DST, perfis e capacidade legal, sem contornar proteção. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `cargo xtask check-docs`, que valida config/format-catalog.json contra schemas/format-catalog.schema.json e roda `catalog::validate` (xtask/src/validate.rs:386), mais três arquivos de teste que ainda não existem e são criados aqui — tests/midi_tracker.rs, tests/tracker.rs e tests/dsd.rs em libs/decode/crates/vynamp-decode, alimentados por um corpus novo em tests/fixtures/midi, tracker, sf2, dsd e sacd — e o já existente `cargo test -p vynamp-output-core --test dop --locked` como linha de base do lado DoP | Hoje não existe nenhuma dessas fixtures: `find` por .mid, .mod, .sf2, .dsf, .dff e .iso no repo volta vazio. libs/decode tem 9 crates e 355 `#[test]` (aiff, core, flac, mp3, mpa, pcm, riff, testkit, facade), `traits.rs` só declara `Demuxer::next_packet`, `Demuxer::seek` e `Decoder::decode` — não há operação render — e `Limits` em crates/vynamp-decode-core/src/limits.rs não tem orçamento de loops nem de vozes. Em libs/output, `SampleFormat::Dop` existe e `tests/dop.rs` passa 11 testes (`cargo test -p vynamp-output-core --test dop --locked`, 11 passed em 0,18 s) com marker 0x05/0xFA e rate 176 400, mas nenhuma variante de DSD nativo foi adicionada — format.rs:103 ainda diz que um formato DSD_U8/U16/U32 'adds its own width here' — e libs/output/docs/support-matrix.md marca DoP como library-verified apenas em null e capture, com `-` em wasapi, alsa, pipewire, pulse, coreaudio e aaudio; tests/hardware não existe e config/project.json tem application_ready igual a false, então `cargo xtask quality-gate` recusa antes de rodar qualquer coisa. Depois da TASK, a evidência exigida é: o tuple de probe de cada uma das quatro entradas de DSD distinto entre si e sem `decode` no tuple de `midi-events`; o render de MOD e XM parando no limite de loops com a contagem de loops, de vozes e de bytes por período registrada; o SF2 com caminho e id de licença no registro e o caso sem SF2 recusado; e a ISO protegida recusada sem extração, com as duas capacidades de extração e a legalidade separadas na capability |
| Hardware | um DAC com DSD nativo (DSD64/128/256 por ASIO ou WASAPI exclusive) e uma imagem SACD ISO não protegida de origem legal; nada disso existe aqui — este ambiente é Windows 11 x86_64 sem placa pro e sem DAC DSD, e não há aparelho Android, iOS nem máquina macOS. O packing DoP, a distinção raw/DST/PCM/DoP por bytes e o limite de loops rodam no host Capture/Null sem hardware; o DSD nativo por DAC e o DSD512+ ficam registrados como limitação | ausência é limitação registrada, não aprovação |

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
