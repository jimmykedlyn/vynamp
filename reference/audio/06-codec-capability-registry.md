# Catálogo técnico, capabilities e cobertura por perfil

Versão 0.7.0 · Catálogo de planejamento completo nesta revisão; suporte real ainda não implementado.

## Três verdades separadas

[Catálogo](../../config/format-catalog.json) enumera desejos/restrições/fase, [schema](../../schemas/format-catalog.schema.json) valida forma e guardas semânticas. Não é registry runtime. Build manifest resulta de backend/version/features/configure flags/targets/licenças reais; RuntimeCapabilities resulta de probe do build+OS+dependências+dispositivo. Só registry verified e available pode gerar associação, menu de encoding ou promessa de playback. planned/experimental/unavailable têm rótulos claros. Sem decoder implementado neste pacote, nenhum perfil é anunciado disponível.

[CodecCapability](../../schemas/codec-capability.schema.json): operation demux/decode/encode/render/passthrough; container, codec, profile/level/framing, backend/version/build/platform/arch, limites de taxa/canais/sample formats, seek/gapless/chapters/DRC/metadata e evidence/license IDs. Probe retorna o tuple real e motivo de rejeição, não decide por extensão. Suporte demux não significa decode; decode PCM não significa renderização espacial/Atmos/passthrough; playback não habilita encode ou tag-write.

## Catálogo moderno e bindings

PCM WAV/AIFF/RF64/Wave64/BWF/CAF/AIFF-C/AU por endianness, valid bits e codec; FLAC nativo/Ogg/Matroska e MP4 separado; MP3 CBR/VBR/gapless; AAC-LC/HE v1/HE v2/USAC xHE/AAC-LD/ELD/ELDv2 separados. AAC ADTS/ADIF/LOAS/LATM não são framing equivalente. xHE usa AOT42 e DRC/loudness normativos, não é AAC-LC com extensão diferente. Android decoder do OS é candidato, não prova do adapter/extractor/grant/clock do Vynamp.

ALAC MP4/CAF; Vorbis/Opus Ogg, WebM convencional Opus/Vorbis; Matroska MKA/MKV permite outros bindings conforme mapping. Não listar FLAC em WebM como suporte convencional. MP4 Opus/FLAC/MP3 são bindings específicos a provar, não consequência de aceitar M4A. MOV/MP4/MKV/WebM/TS podem conter faixa de áudio selecionada; UI escolhe language/name/default quando múltiplas e respeita duração/seek/chapters da faixa. Não renderizar vídeo/sync AV nem assumir default para todas extensões de vídeo. AVI/FLV/PS e codecs de fala são expansão por perfil.

WavPack: lossless inteiro/float, hybrid com .wvc correto lossless; sem correction é lossy, nunca badge lossless. .wvc é sidecar não reproduzível sozinho; respeitar grants/caminho/case/hash. WavPack DSD, DFF DST e raw DSD são distintos. APE versões/níveis, TTA, WMA v1/v2/Pro/Lossless/Voice, MPC SV7/SV8, MP1/MP2 têm corpus próprio. DSF/DFF DSD→PCM M3; native/DoP/DSD512+/768k é M5/hardware; ISO SACD não autoriza DRM/circumvention.

MPEG-H 3D Audio (MP4/mha1/mhm1/MHAS, baseline/level), AC-4 (apresentações/DRC), IAMF (modelo/elementos/mix presentation/renderer e codec subjacente) são expansão M5 com spikes M0. IAMF não é codec único: versão/spec/framing e renderer se somam a Opus/AAC-LC/FLAC/LPCM. Dolby Atmos não é formato genérico autônomo: metadata/renderer em codecs específicos; TrueHD/MLP/E-AC3/DTS core/HD-HR/HD-MA são entradas separadas. Downmix PCM básico precisa ser identificado e não rotulado Atmos/binaural licenciado sem prova. Passthrough é caminho distinto, não passa pelo DSP PCM nem pode ser analisado por visualizer PCM quando não há decode.

MIDI/RMI/XMF/KAR com synth/SoundFont e trackers MOD/XM/IT/S3M/MO3/MTM/UMX são expansão M5, não import de plugin Winamp. SoundFont origem/licença do usuário, loops/voice count/CPU bounded. Arquivo RAW PCM exige parâmetros explícitos (rate/bits/endian/layout), não detecção mágica. A entrada unsupported explícita do catálogo é DRM comercial. Pacotes antigos, arquivo sem binding e vídeo não autorizado são rejeições da política de abertura, não entradas de codec artificiais.

## Bluetooth não é associação de arquivo

SBC, AAC, aptX variantes, LDAC, LC3 LE Audio e LC3plus são transportes/implementações distintas. Rota BT pertence ao OS/hardware; availability/negotiated codec só mostrado quando API confiável expõe. Não registrar .lc3/.ldac/.aptx por existir Bluetooth nem converter FLAC em lossless radio. Arquivo/framing experimental de LC3/LC3plus requer novo binding/fixture/license e permanece M5 opcional. DVC é nome de comportamento proprietário de outro app: implementar gain/headroom/volume próprio, sem prometer bypass global de Android ou clone ABI.

## Dados técnicos, gain e timeline

Opus pre-skip/end trim/output_gain e R128; MP3 Xing/LAME delay/padding; MP4 edit lists/priming; Matroska CodecDelay/SeekPreRoll/DiscardPadding; Ogg chaining/discontinuities; AAC/USAC MPEG-D DRC. Decoder devolve timeline/trim/gain aplicados e metadata pendente, para engine não dobrar ganho/DRC/RG. Loudness codec intrínseca e ReplayGain são políticas diferentes; preferência album/track e target explícitos, sem combinar números de unidades diferentes.

Codec capability informa se DRC pode ser controlada, se decoder já a aplicou e se há caminho neutro; se inevitável, modo transparente indisponível. Channel layout físico/canais identificáveis e downmix matrix documentados; objeto espacial exige renderer. Mudança de config/sample rate/frame size reinicia apenas estado necessário com generation, sem frames stale. Rate/bit-depth de PCM produzido por lossy não vira origem hi-res lossless.

Tags/capas/chapter-read/write independentes: ID3 CHAP/CTOC/USLT/SYLT, MP4/Nero/QuickTime chapters por binding, Vorbis/Opus/FLAC, Matroska tags/chapters, CAF metadata, AIFF ID3 e BWF/iXML/cue, sidecar LRC/podcast JSON/WebVTT. Parser de sidecar não acessa rede/arquivo fora grant por metadados. Escrita somente roundtrip comprovado; unknown fields/version preservados; .wvc/cue/chapters/ratings não se perdem em rename.

## Fixture e gates

Cada tuple tem fixtures autorais/licenciadas+SHA, backend/build/version/configure/platform, oracle, expected samples/duration/trim/gain/layout/seek e corrupt variants. Inteiro lossless sample-exact; lossy/float tolerance independente explícita; renderer por apresentação com referência. Testar EOF, rate change, gapless, seek exact/approx, cancel/grants/nonseekable, DRC/R128 sem dupla aplicação, .wvc ausente/inválido, multichannel stems, giant file/u64/budget.

M3 exige formatos common/required do catálogo em todos os cinco alvos. conditional M3 exige spike, inventário e erro/limitação honesta se não puder distribuir; não esconder nem inventar decoder. M5 é expansão explícita, não pretexto para atrasar comum. Todo release atualiza catálogo/fontes, audita novas versões/formatos e revê license/CVE; não prometer uma lista universal eternamente completa.

### Resultado de decode e DRC

Capability inclui level nullable, drc_control (unknown/unsupported/fixed/controllable) e neutral_path separado do suporte a DRC. Cada resultado de decode informa intrinsic_gain_applied_db, drc_applied, política/target_lufs quando aplicável e metadata pendente. Engine registra estágios/unidades e não reaplica o que o backend já processou. Valor desconhecido impede alegação de caminho neutro. E17-02/04 implementam e certificam esse contrato.
