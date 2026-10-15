# QA complementar — integração, codecs, DSP e capacidades

Versão 0.7.0 · Cenários de implementação; nenhum teste de aplicativo declarado.

| QA | Cenário | Oráculo |
|---|---|---|
| QA-022 | Associações/Abrir com nos cinco alvos | Handler candidato abre a mesma fila; defaults de terceiros preservados; sem admin obrigatório |
| QA-023 | Batch, CLI, IPC, deep link e grants | Ordem, commit único, idempotência/Conflict e generation; metacaracteres não executam comandos |
| QA-024 | Owner, perfis, portable e locks | Vinte launches elegem um owner; stale PID não autoriza reset; não há dois writers no perfil comum |
| QA-025 | Codec/container/perfil, encode e metadata | Fixture/hash/oráculo por binding; WebM não aceita FLAC convencional; trim/gain/canais corretos |
| QA-026 | Áudio imersivo, DRC, render e passthrough | Decoder e renderer separados; referência por apresentação/layout; licenças e limitações explícitas |
| QA-027 | Teto true-peak no sinal final | PCM quantizado do backend após mix/gain/resample não excede ceiling +0.1 dBTP; sem NaN ou drift |
| QA-028 | Visualizers, budgets, fallback e acessibilidade | Falha GPU/worker não bloqueia áudio; Android background sem rendering; FFT/tap/Fs coerentes |
| QA-029 | Proxy, stream bindings e reconnect | 407 distinto de 401; credenciais separadas; sem direct fallback silencioso; cancel e ABR seguros |
| QA-030 | Manifest real e associações | Só capacidades verificadas, disponíveis e licenciadas geram handler; catálogo não concede suporte |
| QA-031 | Upgrade, uninstall, portable e backup | Remove apenas registros próprios; preserva mídia/defaults; staging/rollback e lock respeitados |

## Entrada pelos sistemas

Nomes com Unicode, espaços, aspas, apóstrofos, percent, hash, ampersand e hífen inicial. Extensão errada/ausente; lotes de 1/2/20/100 com inacessíveis, missing, DRM e unsupported. Duplicatas intencionais versus reentrega; Finder antes do bootstrap; onNewIntent/rotation; grants revogados; bookmark stale; placeholder cloud; NAS offline; fonte sem seek; lote totalmente inválido preservando fila. M3U8 local/HLS, relativas em CUE/PLS/XSPF, ciclos/XXE e playlist remota tentando file/content locais. Versões IPC incompatíveis e instalado/portable disputando perfil.

## Corpus de áudio

Música autoral/licenciada, PCM gerado, tones/impulses/sweeps e canais identificáveis. Pares gapless; MP3 trim, AAC priming/USAC DRC, Opus pre-skip/R128/Ogg chaining e Matroska CodecDelay/DiscardPadding. WVC válido/ausente/corrompido; DSD/decimador/duração/downmix; PCM24/32/float, endianness, valid bits, taxas e RF64 u64. DRC/RG aplicados uma vez. Encode/tag-write só após oráculo e roundtrip preservando campos desconhecidos.

## DSP, render e rede

Crossfade correlacionado, EQ/preamp extremos, 44.1↔48/192/384 kHz, pitch, routing, tails e dither 16/24/32-bit. Oráculo true-peak independente no buffer final; medir apenas o nó limiter é insuficiente. Testar GPU ausente/device-loss, fullscreen/Escape, resize/DPI, background Android, reduced motion e budgets agregados. Grafo/preset/plugin com limites de fuel e tempo não causa deadlock.

Servidor/proxy local determinístico: TLS, Range, ICY, HLS, 407, token renewal, discontinuity, DNS e redirects maliciosos. Perfil desconhecido é rejeitado antes de ABR switch. CI não depende de rádio pública. Fuzz de mídia, IPC, playlist, registry e scenes; fixtures/licenças versionadas. Relatório inclui build/OS/ABI/device, hash de entrada, comandos, esperado/observado, latência e limitações; documento não concede available.

QA-023 M3 exclui os cenários deep link/OAuth de M4; executar e anexar estes na fase M4. QA-024 inclui duas sessões e aliases do mesmo perfil. QA-023 inclui receipt atomicamente persistido, crash pós-commit/pré-ACK e locator Unicode dentro de maxLength mas fora do limite UTF-8; expiração/capacidade não autoriza retry silencioso.
