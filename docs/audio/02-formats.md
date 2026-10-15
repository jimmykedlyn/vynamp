# Matriz de formatos, codecs e tags

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

Um container não garante suporte a todos os codecs dentro dele. Features e versões de decoder devem ser travadas após prova M0. Cada formato exige fixtures licenciadas, reprodução, seek quando aplicável, duração, metadados, corrupt input e memória limitada. Decoder preferido é candidato, não disponibilidade comprovada.

| Grupo / container | Codec / perfis | Fase | Estratégia candidata / restrições |
|---|---|---|---|
| WAV, AIFF | PCM 8/16/24/32 inteiro, float32/64 | M1 | Symphonia/reader próprio pequeno; endianness e extensible |
| FLAC | lossless 16/24/32 conforme decoder | M1/M2 | Symphonia; PCM32 validado em M2 |
| MP3 | MPEG Layer III CBR/VBR | M1 | Symphonia; Xing/LAME para gapless |
| M4A/MP4/M4B | AAC-LC, ALAC; capítulos | M2 | Symphonia quando suportado; fallback auditado |
| ADTS/MP4 | HE-AAC v1/v2 | M2 | Backend validado/FFmpeg opcional; não confundir AAC-LC com HE |
| Ogg | Vorbis, Opus | M2 | Symphonia para suportado; libopus adapter se necessário |
| WebM | Opus, Vorbis | M3 | WebM convencional não inclui FLAC |
| Matroska/MKA | Opus, Vorbis, FLAC e outros bindings provados | M3 | CodecDelay/SeekPreRoll/DiscardPadding e seleção de áudio |
| WavPack, APE, TTA | lossless/hybrid conforme perfil | M3 | Libraries/FFmpeg isolado, licença revisada |
| ASF | WMA standard/lossless sem DRM | M3 | Sistema ou FFmpeg; perfis testados separados |
| MP1/MP2, MPC | MPEG I/II, Musepack | M3 | Backend adicional, fixture por perfil |
| DSF/DFF | DSD64/128/256 → PCM | M3 | Decimator documentado, sem ganho implícito |
| DSF/DFF | Native/DoP DSD64–512+ | M5 | Requer backend/DAC; DSP incompatível |
| MOD/XM/IT/S3M | tracker/module | M5 | libopenmpt isolado, loops limitados |
| MIDI/RMI | eventos + SoundFont | M5 | Synth próprio/adaptado, SF2 fornecido pelo usuário |
| AC3/E-AC3/DTS | multichannel | M5 | Licenças/perfis; downmix declarado |
| TAK/OFR/SPX/ATRAC/MO3 | legados/especiais | M5 | Plugin decoder opcional; não promete todos |
| RF64/Wave64 | PCM >4GB | M3 | offsets u64 e leitura incremental |
| SACD ISO | extração de áudio DSD | M5 | Biblioteca validada, sem proteção proprietária |

## Limites e descoberta

PCM16/24/32; 44.1/48/88.2/96/176.4/192/352.8/384 kHz são alvos até M3, com capacidade real por backend. Taxas inferiores usuais também; 768 kHz é M5. Até 8 canais em M3; multichannel não pode ser ativado em driver stereo. Decoder informa suporte por perfil em registry; UI “formatos disponíveis” vem do build real. Extensão errada não impede sniff seguro; sem executar arquivos.

Import/export: M3U/M3U8, PLS, XSPF e CUE em M2; playlists UTF-8 padrão, fallback legacy selecionável, URIs relativas resolvidas contra arquivo de origem. Limites de profundidade/referências previnem loops de playlists. XSPF/XML desabilita entidades externas e acesso de rede do parser. CUE representa faixas virtuais sem duplicar mídia, timestamps de 75 frames/s, gaps e offsets explícitos.

Tags: ID3v1/v2.3/v2.4, Vorbis comments, FLAC pictures, MP4 atoms, APEv2, WAV INFO e BWF conforme parser. Leitura M2; escrita por formato suportado M3. Campos desconhecidos preservados. Rating local separado da tag; lista multi-artista/gênero sem split destrutivo por “/”. Capa PNG/JPEG/WebP, fallback; limitar dimensão e bytes antes decode. Letras USLT/SYLT/LRC, sidecar e providers com permissão.

## Encoders

M3: FLAC/WAV/Opus/Vorbis; MP3 se distribuição/encoder permitirem. AAC opcional mediante build auditado. Conversão lossless→lossy não é “hi-res”. Sem transcodificação de fonte DRM. Registro separado de decode/encode; codec presente para playback não autoriza anunciar encoding.

## Catálogo ampliado por binding

Fonte canônica: [JSON](../../config/format-catalog.json) e [contrato](06-codec-capability-registry.md). São 68 entradas de planejamento, não 68 codecs disponíveis. `required` deve passar nos cinco alvos até M3; `conditional` exige avaliação/evidência ou limitação explícita; `optional` e M5 são expansão; `unsupported` não promete reprodução. Perfis secundários posteriores estão discriminados nas restrições.

| ID | Container/framing | Codec/perfis | Fase | Política | Operações |
|---|---|---|---|---|---|
| pcm-wave | WAV, RF64, Wave64 | pcm: u8, s16, s24, s32, f32, f64 | M1 | required | demux, decode |
| pcm-aiff | AIFF, AIFF-C | pcm: s8, s16, s24, s32, f32, f64 | M1 | required | demux, decode |
| pcm-caf | CAF | pcm: integer, float | M3 | required | demux, decode |
| alac-caf | CAF | alac: lossless | M3 | required | demux, decode |
| pcm-au | AU/SND | pcm: linear PCM | M3 | required | demux, decode |
| g711 | WAV, AU/SND | g711: A-law, mu-law | M3 | required | demux, decode |
| adpcm-wave | WAV | adpcm: IMA, Microsoft ADPCM | M3 | required | demux, decode |
| pcm-raw | RAW | pcm: manual rate/channels/endian/validbits | M3 | optional | demux, decode |
| flac-native | FLAC | flac: 16-bit, 24-bit, 32-bit | M1 | required | demux, decode |
| flac-ogg | Ogg | flac: Ogg FLAC mapping | M3 | required | demux, decode |
| flac-mp4 | MP4, fMP4 | flac: fLaC binding | M3 | conditional | demux, decode |
| flac-matroska | Matroska | flac: A_FLAC | M3 | required | demux, decode |
| mp3 | MPEG elementary | mp3: MPEG1 Layer III, MPEG2 Layer III, MPEG2.5 Layer III | M1 | required | demux, decode |
| mp3-mp4 | MP4 | mp3: MPEG audio binding | M3 | conditional | demux, decode |
| mp1-mp2 | MPEG elementary | mpeg-audio: Layer I, Layer II | M3 | required | demux, decode |
| aac-lc-mp4 | MP4, fMP4, M4B | aac: AAC-LC | M2 | required | demux, decode |
| aac-lc-adts | ADTS | aac: AAC-LC | M2 | required | demux, decode |
| aac-he | MP4, fMP4, ADTS | aac: HE-AAC v1 SBR, HE-AAC v2 PS | M2 | required | demux, decode |
| aac-latm | LOAS/LATM, MPEG-TS | aac: AAC-LC, HE-AAC v1, HE-AAC v2 | M3 | conditional | demux, decode |
| aac-adif | ADIF | aac: AAC-LC | M3 | conditional | demux, decode |
| aac-usac | MP4, fMP4 | usac: xHE-AAC AOT42, USAC | M3 | conditional | demux, decode |
| aac-low-delay | MP4, RTP | aac: AAC-LD, AAC-ELD, AAC-ELDv2 | M5 | optional | demux, decode |
| alac-mp4 | MP4, M4A | alac: lossless | M2 | required | demux, decode |
| vorbis-ogg | Ogg | vorbis: Vorbis I | M2 | required | demux, decode |
| opus-ogg | Ogg | opus: Opus mono/stereo, mapping families supported | M2 | required | demux, decode |
| opus-mp4 | MP4, fMP4 | opus: Opus dOps binding | M3 | conditional | demux, decode |
| webm-audio | WebM | vorbis-opus: Opus, Vorbis | M3 | required | demux, decode |
| matroska-audio | Matroska | multiple: Opus, Vorbis, PCM, AAC, MP3 | M3 | required | demux, decode |
| ts-audio | MPEG-TS | multiple: AAC-LC, HE-AAC, MP2 | M3 | conditional | demux, decode |
| mov-audio | MOV | multiple: PCM, AAC, ALAC | M3 | conditional | demux, decode |
| wavpack-lossless | WavPack | wavpack: integer lossless, float lossless | M3 | required | demux, decode |
| wavpack-hybrid | WavPack + WVC | wavpack: hybrid lossy without WVC, hybrid lossless valid WVC | M3 | required | demux, decode |
| wavpack-dsd | WavPack | wavpack: DSD lossless | M5 | optional | demux, decode |
| ape | Monkey Audio | ape: compression levels supported | M3 | required | demux, decode |
| tta | TTA | tta: lossless | M3 | required | demux, decode |
| wma-standard | ASF | wma: WMA v1, WMA v2 | M3 | required | demux, decode |
| wma-lossless | ASF | wma-lossless: WMA Lossless | M3 | required | demux, decode |
| wma-pro | ASF | wma-pro: WMA Pro | M3 | conditional | demux, decode |
| wma-voice | ASF | wma-voice: WMA Voice | M5 | optional | demux, decode |
| musepack | Musepack | mpc: SV7, SV8 | M3 | required | demux, decode |
| dsd-pcm | DSF, DFF | dsd: DSD64, DSD128, DSD256 raw | M3 | required | demux, decode |
| dsd-dst | DFF | dst: DST compressed DSD | M5 | optional | demux, decode |
| dsd-native | DSF, DFF | dsd: Native DSD, DoP | M5 | conditional | demux, passthrough |
| amr | AMR, 3GP | amr: AMR-NB, AMR-WB | M5 | optional | demux, decode |
| speex | Ogg | speex: Speex NB, WB, UWB | M5 | optional | demux, decode |
| ac3 | AC3, MP4, Matroska | ac3: AC-3 | M5 | conditional | demux, decode, passthrough |
| eac3 | EAC3, MP4, Matroska | eac3: E-AC-3, JOC metadata | M5 | conditional | demux, decode, passthrough |
| truehd-mlp | Matroska, elementary | truehd-mlp: TrueHD, MLP | M5 | conditional | demux, decode, passthrough |
| dts | DTS, Matroska | dts: DTS Core, DTS-HD HR, DTS-HD MA | M5 | conditional | demux, decode, passthrough |
| mpeg-h | MP4, MHAS | mpegh: mha1, mhm1, profile/level declared | M5 | conditional | demux, decode, render |
| ac4 | MP4, elementary | ac4: AC-4 presentations | M5 | conditional | demux, decode, render |
| iamf | IAMF, MP4 | iamf: Opus, AAC-LC, FLAC, LPCM | M5 | conditional | demux, decode, render |
| midi | SMF, RIFF MIDI, XMF | midi-events: MIDI, RMI, XMF, KAR | M5 | optional | demux, render |
| tracker | tracker module | tracker-module: MOD, XM, IT, S3M, MO3, MTM, UMX | M5 | optional | demux, render |
| tak | TAK | tak: supported profiles only | M5 | optional | demux, decode |
| optimfrog | OptimFROG | optimfrog: supported profiles only | M5 | optional | demux, decode |
| atrac | OMA, WAV | atrac: supported profiles only | M5 | optional | demux, decode |
| shorten | Shorten | shorten: supported profiles only | M5 | optional | demux, decode |
| legacy-video-audio | AVI, FLV, PS | multiple: supported profiles only | M5 | optional | demux, decode |
| bt-classic | OS Bluetooth route | bluetooth-transport: SBC, AAC, aptX, aptX HD, aptX Adaptive, aptX Lossless, LDAC | M3 | conditional | observe |
| bt-le | OS Bluetooth route | bluetooth-transport: LC3 LE Audio, LC3plus where OS reports | M3 | conditional | observe |
| lc3-files | experimental framing | lc3: explicit LC3 framing | M5 | optional | demux, decode |
| m3u | M3U, M3U8 | playlist: version declared | M2 | required | import, export |
| pls | PLS | playlist: version declared | M2 | required | import, export |
| xspf | XSPF | playlist: version declared | M2 | required | import, export |
| cue | CUE | playlist: version declared | M2 | required | import, export |
| encrypted-drm | protected containers | encrypted: Widevine, FairPlay, PlayReady | M5 | unsupported | inspect |

| sacd-iso | SACD ISO | dsd-dst: raw DSD/DST extraction | M5 | optional | demux, decode |

### Encoding e metadados

Decodificar não habilita converter ou gravar tags. E08-02 mantém encoder registry próprio: WAV/FLAC e opções AAC/MP3/Opus/Vorbis somente com encoder licenciado validado, parâmetros/bitrate/canais e formato de saída explícitos. Lossy→lossless não recupera informação. Formatos rara/imersivos/BT não recebem encoder por herança. Cada binding expõe leitura/escrita de tags e capítulos separadamente; preservar campos desconhecidos e origem. RAW só importa mediante parâmetros, sem associação automática; arquivos WVC e SoundFont são dependências, não handlers genéricos.
