# Catálogo de funcionalidades e destino

Versão: 0.8.0 · Data: 2026-11-05 · Estado: especificação proposta, implementação não iniciada.

Esta é uma cobertura funcional planejada, sem compatibilidade de arquivo/ABI e sem alegação de reproduzir cada submenu histórico. Todo recurso tem fase/tarefa; exceções são explícitas. “Fiel” refere-se ao conceito clássico, controles e riqueza funcional; cores/medidas são tokens próprios.

| Família | Recursos contemplados | Fase / tarefas |
|---|---|---|
| Transporte | Abrir,play,pause,stop,prev/next,seek,volume,mute,pan,shuffle,repeat,stop-after,A/B | M1–M3 E01/E03/E04; A/B segue complemento abaixo |
| Playlist | Várias abas,queue separada,drag/drop,sort/group,search,undo,lock,smart,CUE,imports | M1–M3 E02/E03 |
| Biblioteca | Pastas,artistas/álbuns,gênero/ano,ratings,histórico,favorites,100k,metadata/covers | M2–M3 E03 |
| Som clássico | EQ10,preamp,visualizer,VU,waveform,ReplayGain,gapless | M1–M3 E01/E04/E06 |
| Som avançado | EQ20/31/paramétrico32,compressor/limiter,tone,pitch/tempo,crossfade,channel mapping; vitrola (vinil + válvula, após E04-03, [spec](../audio/10-vitrola.md)) | M2–M3 E04 |
| Hi-res | PCM16/24/32 até384k,exclusive/ASIO opcional,transparent,DACprofiles/diagnostics | M3 E04 |
| Formatos modernos | FLAC/ALAC/AAC/HE-AAC/Opus/Vorbis/WAV/AIFF/MP3/WebM/MKA e lossless adicionais | M1–M3 E01 |
| Rádio | URL,ICY,stationbookmarks,retry,metadata,HLSlive,record,timeshift | M2–M3 E05 |
| Long-form | Podcastfeed/download/bookmarks/velocidade,chapters/audiobooks | M2–M4 E05/E08 |
| Aparência | Classic/Modern/Mini,Copper,DPI,zoom,docking,editor,own themes | M1–M3 E02/E06 |
| Extensões | Own WASM/provider/lyrics/visualizer/utility,SDK,helper sandbox,registry | M3–M4 E07/E09 |
| Utilitários | Batchtagwrite,converter,RG/BPMscan,rename/dedupe,scheduler/alarm/sleep,CLI | M3 E03/E08 |
| Ferramentas de arquivo | Vynamp Restore (análise, diagnóstico de defeitos, restauração, exportação preservando metadados) e Vynamp Converter (transcodificação em lote) como apps desktop separados ([spec do Restore](../tools/01-vynamp-restore.md) · [spec do Converter](../tools/02-vynamp-converter.md)) | M3 E18 |
| Integrações | Media keys/tray/SMTC,hotkeys,file assoc,scrobble/presence,remotecontrol | M2–M4 E08 |
| Plataforma | Catálogo,VOD,live,accounts,rights,sync,admin,ingest,CDN,registry | M4 E09 |
| Expansão | ARM adicional Windows/Linux,DSDnative/DoP/768k,MIDI/trackers/raros,CD,VST3,DASH/cloud,casting,EQ64 gráfico e paramétrico 64 nativos/1M (UI dos cinco modos já obrigatória no MVP HTML/SVG/JS, [equalizadores](../audio/08-equalizers.md)) | M5 E12 |
| Não reproduzido | WinampDLL/MAKI/skins,video/DVD,AndroidDVC/EQglobal,DRM comercial,store legado | Fora de escopo conforme visão |

## Complementos para evitar lacunas

A/B loop fica em TASK-E01-05: range sample/ticks válido, A<B, repetição sem drift e saída automática na troca da fonte. Atalhos globais ficam em TASK-E08-06, com conflito/rebinding. Spectrum/waveform abaixo de qualidade são builtins E06; não é necessário converter plugins históricos. Dados de rating/bookmarks independem do formato.

Mídia vídeo pode conter faixa de áudio suportada se demuxer validado; não há tela de vídeo/syncAV prometida. CD ripping, legacy codecs e casting só contam como completos quando seus gates M5 passarem. EQ global fora do app seria outro produto e exigiria privilégios/driver próprios.

PlataformasM0–M3: Windows/Linux/macOS Intel+Silicon/Android. UI phone/tablet, MediaSession/focus/SAF/TalkBack/background são parte do produto básico Android, épicoE13; matrix/CI/release comum emE14. Linux/Mac tasksE12-01/02 foram promovidas aM3 mantendo IDs.

## Complementos 0.6

[Auditoria](08-completeness-audit.md): E16 fecha associação/Open With/grants/batch/IPC/perfis/backup/deep links. E17 fecha perfis modernos, trim/DRC, guard final, visualizer runtime, proxy, Bluetooth e manifest por artefato. [Catálogo](../../config/format-catalog.json) diferencia required/conditional/optional/unsupported e planejamento de capacidade real.

## Complementos 0.8 (2026-11-05)

E18 fecha as **ferramentas de arquivo** como apps desktop separados: [Vynamp Restore](../tools/01-vynamp-restore.md) (cadeia normativa de 11 estágios DSP, 12 detectores de defeito, exportação determinística e não destrutiva) e [Vynamp Converter](../tools/02-vynamp-converter.md) (matriz de formatos, fila de jobs persistida, lib própria `vynamp-encode`). E08-02 entrega os codificadores; E18-02 entrega o app que os usa. A restauração não é anunciada como criação de qualidade: a conversão lossy nunca é rotulada como restauração hi-res ([ADR-013](../architecture/adr/ADR-013-vynamp-restore.md)).
