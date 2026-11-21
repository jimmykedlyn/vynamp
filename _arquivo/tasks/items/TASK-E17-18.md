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
- **Não** implementa o packing DoP, o caminho DSD nativo por DAC nem o PCM a 768 kHz. `TASK-E12-04` é a dona: `libs/output/crates/vynamp-output-core/src/dop.rs` e os 11 testes de `tests/dop.rs` são a evidência que ela declara, e esta TASK só compara os quatro caminhos de DSD com o que aquela entrega declara.
- **Não** decide o selo lossless de WavPack nem o caso de WV sem .wvc ou com correção inválida. O selo de inteiro sample-exact, os níveis híbridos e o .wvc ausente são `TASK-E17-05`, que já declara em seu Boundary que WavPack DSD não herda o selo; aqui ele entra só como fixture de DSD, e emitir selo daqui criaria dois donos do mesmo veredito.
- **Não** publica o manifest de capacidades por artefato nem converte `planned` em disponível. `TASK-E17-12` gera o manifest com backend, feature, licença e hash; a entrada `sacd-iso` de config/format-catalog.json tem `association_allowed` igual a false, e é naquele manifest que a associação é negada, não nesta certificação.
- **Não** roda o gate combinado de codecs, DSP e visualizers nos cinco alvos. `TASK-E17-20` fecha esse gate (AC-3 exige que nenhum job dependa de feature M4/M5); a evidência desta TASK é M5 e não pode virar condição do gate de 1.0.
- **Não** põe libopenmpt, o synth de SF2 nem o extrator de SACD ISO dentro do helper nativo isolado. Sandbox de OS, RPC autenticado e consentimento de plugin nativo são `TASK-E07-05`; aqui o backend é o adapter que `TASK-E12-05` entregou, e trazer o isolamento de processo para cá levaria a TASK a especificar sandbox no meio da certificação.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e17-18-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E17-18-AC-1` | as fixtures .mid, .rmi, .xmf e .kar em libs/decode/tests/fixtures/midi com SOURCE.txt, LICENSE.txt e MANIFEST.txt de sha256, o synth que TASK-E12-05 entrega ligado, e a entrada `midi` de config/format-catalog.json com codec_id `midi-events`, operations `demux` e `render` e sem `decode` | cada fixture é aberta por `Registry::probe` de vynamp-decode-core com a extensão como hint, e para o codec id `midi-events` o tuple devolvido é comparado com a linha do catálogo e com o que o `CodecDescriptor` registrado declara em name, verification e supports, conferindo que nenhum deles expõe `Decoder::decode` para MIDI | Rendering MIDI não é decode PCM |
| `TASK-E17-18-AC-2` | um SF2 fornecido pelo usuário em libs/decode/tests/fixtures/sf2 acompanhado de SOURCE.txt, do arquivo de licença e do sha256 no MANIFEST.txt, e um segundo caso em que nenhum SF2 é passado ao synth | um .mid do perfil MIDI é aberto para render nos dois casos e o resultado da abertura, o caminho absoluto do SF2 usado e o id de licença registrado na capability são lidos; no caso sem SF2 a abertura é recusada em vez de render silêncio | SoundFont licenciado explícito |
| `TASK-E17-18-AC-3` | um MOD de 4 padrões cuja posição de ordem 0 salta para si mesma por comando Bxx, um XM nos 32 canais de padrão do formato com nota em todos na mesma linha, e um MIDI de 3 minutos, todos abertos com os `Limits` de vynamp-decode-core (max_bytes 64 GiB, max_frames 66 355 200 000, max_channels 32, max_sample_rate 768 000) | o render de cada arquivo é conduzido até o limite de loops e o de frames, e as contagens de loops executados, de vozes simultâneas e de bytes alocados por período — esta última lida do CountingAlloc de vynamp-decode-testkit — são registradas junto do erro ou da parada | loops/memória bound |
| `TASK-E17-18-AC-4` | quatro entradas distintas — um DSF DSD64 raw a 2 822 400 Hz e 2 canais, um DFF com frames DST, o PCM desse mesmo DSF decimado a 176 400 Hz pelo decimator de TASK-E01-09 e um stream DoP aberto com `SampleFormat::Dop` em dsd_rate 2 822 400 — mais duas imagens SACD ISO, uma obtida legalmente e não protegida e outra com área protegida | cada uma das quatro entradas é aberta e o caminho de saída escolhido é registrado, e a abertura das duas ISOs é tentada, lendo a capability de `dsd-pcm`, `dsd-dst`, `dsd-native` e `sacd-iso` em cada resultado e registrando a recusa da protegida junto da ausência de qualquer byte de áudio extraído | DSD raw/DST/PCM/DoP distinguíveis. SACD ISO separa extração raw/DST, perfis e capacidade legal, sem contornar proteção. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `cargo xtask check-docs` na raiz do umbrella, que roda `validate::spec` (xtask/src/validate.rs:244), lê config/format-catalog.json na linha 386, valida contra schemas/format-catalog.schema.json e chama `catalog::validate` na linha 391; mais três arquivos de teste que ainda não existem e são criados aqui — crates/vynamp-decode/tests/midi_tracker.rs, tracker.rs e dsd.rs no repo vynamp-decode — alimentados por um corpus novo em libs/decode/tests/fixtures/midi, tracker, sf2, dsd e sacd com SOURCE.txt, LICENSE.txt e sha256; e, dentro de libs/output, o já existente `cargo test -p vynamp-output-core --test dop --locked` como linha de base do lado DoP (o submódulo está em exclude do workspace do umbrella, então esse comando não resolve a partir da raiz) | Hoje não existe nenhuma dessas fixtures: buscar por .mid, .midi, .mod, .xm, .sf2, .dsf, .dff, .iso, .kar, .rmi e .xmf no repo volta vazio, e libs/decode/tests/fixtures tem só flac, flac-conformance, mpeg e pcm. libs/decode tem 9 crates e 505 `#[test]` em crates/ (aiff, core, flac, mp3, mpa, pcm, riff, testkit, facade), e `cargo test --locked` mediu 508 passed e 3 ignored em 47 suítes em 2026-11-10. `traits.rs` declara `Demuxer::tracks/next_packet/seek` e `Decoder::decode/reset` — não há operação render —; `CodecId` em crates/vynamp-decode-core/src/track.rs só tem Pcm, Flac, Mp1, Mp2 e Mp3, sem variante para MIDI, tracker ou DSD; e `Limits` em crates/vynamp-decode-core/src/limits.rs tem max_bytes, max_frames, max_channels, max_sample_rate, max_metadata_bytes e max_depth, sem orçamento de loops nem de vozes. Em libs/output, `SampleFormat::Dop` existe e `cargo test -p vynamp-output-core --test dop --locked` passa 11 testes (0,12 s e 0,16 s em duas execuções, então o número estável é a contagem) com marker 0x05/0xFA e rate 176 400, mas nenhuma variante de DSD nativo foi adicionada — format.rs:103 ainda diz que um formato DSD_U8/U16/U32 'adds its own width here' — e libs/output/docs/support-matrix.md marca DoP como library-verified apenas em null e capture, com `-` em wasapi, alsa, pipewire, pulse, coreaudio e aaudio; tests/hardware não existe e config/project.json tem application_ready igual a false, então `cargo xtask quality-gate` para em 'Hardware gate blocked: application not implemented' antes de rodar. Depois da TASK, a evidência exigida é: o tuple de probe de cada uma das quatro entradas de DSD distinto entre si e sem `decode` no tuple de `midi-events`; o render de MOD e XM parando no limite de loops com a contagem de loops, de vozes e de bytes por período registrada; o SF2 com caminho e id de licença no registro e o caso sem SF2 recusado; e a ISO protegida recusada sem extração, com as duas capacidades de extração e a legalidade separadas na capability |
| Hardware | um DAC com DSD nativo (DSD64/128/256 por ASIO ou WASAPI exclusive) e uma imagem SACD ISO não protegida de origem legal; nada disso existe aqui — o host é Windows 11 x86_64 com áudio Intel para telas e Conexant ISST, nenhum endpoint DSD, e os dois drivers ASIO registrados são ASIO4ALL v2 e FL Studio ASIO, shims de software e não placa pro; não há aparelho Android (apps/android não existe), aparelho iOS nem máquina macOS. O packing DoP, a distinção raw/DST/PCM/DoP por bytes e o limite de loops rodam no host Capture/Null sem hardware; o DSD nativo por DAC e o DSD512+ ficam registrados como limitação | ausência é limitação registrada, não aprovação |

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
