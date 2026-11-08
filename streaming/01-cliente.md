# Cliente de streaming, rádio e fontes remotas

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Protocolos por fase

M2: HTTP(S) arquivo progressivo com Range quando suportado, MP3/AAC/Opus/Vorbis contínuo, Icecast/SHOUTcast via HTTP/ICY, M3U/PLS e podcast RSS/Atom. M3: HLS áudio live/VOD com AAC-LC/HE-AAC validado em TS/fMP4/ADTS conforme backend, master/media playlists, discontinuity e byte ranges. M4: API da plataforma e signed URLs, reconciliação de catálogo. M5: DASH áudio, WebDAV/providers e casting. HLS Opus/FLAC é perfil experimental, não suporte universal.

Resolver retorna SourceDescriptor com transport, codec hints, seekability, duration, live window, headers permitidos e restrictions. UI não conhece HTTP internals. Arquivo remoto seek só habilitado com capability confirmada; 200 após Range é handled/restart ou “seek indisponível”, não interpretado como206.

## Política de rede

HTTPS default; URL http de rádio pública digitada pelo usuário pode ser aceita com aviso inline de conexão sem criptografia e sem credenciais. Timeout DNS/connect10s, headers15s, stall15s configurável; retry base1/2/4/8/16s com jitter, teto30s e max5 consecutivos por erro recuperável; cancelar é imediato na control thread. 401/403, unsupported codec e TLS inválido não entram em retry cego. Redirect max5; nunca reenviar Authorization para outra origem nem downgrade tokenizado.

Clientes aceitam URLs públicas escolhidas pelo usuário e NAS privados explícitos; conteúdo remoto não pode induzir acesso automático a loopback/link-local/metadata endpoints. Cada URL filha de manifest, playlist, redirect e plugin passa política própria; restrição DNS reaplicada após resolução/rebind. Schemes file/exec/javascript/data não são fontes remotas. Credenciais no keychain; query signed é redacted de log/cache export.

## Buffers e cache

VOD target3s; rádio8s, config2–30; memory encoded cap32MiB por stream, decoded cap16MiB e duração limitada conforme PCM rate. Para hi-res alto, adaptar duração mantendo limites e avisar se buffer não atende. Rebuffer: mute/fade curto, retomar após threshold; não repetir áudio velho. Disk LRU2GiB default opcional, pode desabilitar; separar cache de downloads e nunca cachear auth header. Cache privado por conta/source, purge logout; expiração/ETag validados. Downloads offline só quando rights descriptor permitir.

ICY: parse metaint em bytes incluindo frames partidos, retirar blocos de metadata antes decoder, encoding selecionável, sanitizar markup. Metadata malformada não causa áudio de tags ou endless allocation. Radio LIVE sem duração falsa; meter bitrate aproximado indica estimativa.

## HLS

Parser incremental com limites: manifest≤2MiB, nesting≤3, entries≤20k, segment body≤64MiB ajustável ao perfil. TTL segundo target duration; sequence e discontinuity preservam timeline. Start live a3 segments da ponta por padrão; adapt à janela curta; live latency exposta como estimativa. Variant auto usa throughput EWMA/headroom30%, sobe após3 segmentos estáveis, desce cedo; manual quality impede troca salvo erro com confirmação.

AES-128 HLS com key URI autorizada M3; SAMPLE-AES/DRM não suportados sem ADR. Atualizar auth por provider, keys sem log e TTL limitado; não confundir criptografia de transporte com DRM. Timeshift M3 usa ring em disco de30min/512MiB, respeita restrictions; seek só dentro da janela, live edge retorna atual. Sem timeshift, pause retoma live e informa salto.

## Aceite

Server fixtures: slow/chunked, ICY fragmented, Range206/200/416, redirects, stalled, expired token, DNS/TLS errors, sequence reset, segment missing, live window sliding, discontinuity sample-rate change, cancel during opening e network restore. Stream health expõe startup, bitrate, rebuffer count, live delay; nenhuma métrica contém URL privada completa.
