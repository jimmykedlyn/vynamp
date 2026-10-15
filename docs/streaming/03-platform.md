# Plataforma web futura e catálogo

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

M4 é programa separado conectado ao player por provider API v1. A plataforma será administração/editorial + conta do ouvinte + delivery áudio; não será necessária para tocar arquivo local. Antes iniciar, definir catálogo, autorizações de distribuição, região, volume concorrente e orçamento por ouvinte-hora.

## Arquitetura proposta

API Rust Axum, PostgreSQL, object storage S3-compatible, CDN, workers de ingest/transcode e fila durável. Começar monólito modular; serviços separados só por carga/isolamento demonstrado. Outbox transacional evita job perdido entre banco e fila. FFmpeg worker isolado para transcode; imagem e build auditados; não executar shell derivado de metadados. Frontend web TypeScript/React candidato para admin e catálogo; isso não muda o núcleo desktop Rust. Player web depende de codecs/APIs do navegador e não promete hi-res/exclusivo como desktop.

Domínios: Identity, Catalog, MediaAssets/Renditions, Stations/Programs, PlaybackSessions, Rights, UserLibrary/Sync, ExtensionsRegistry, Audit. IDs UUID estáveis; catálogo publicado não expõe object keys internos.

## Ingest e delivery

Asset→upload direto assinado e limitado→quarantine→probe/verify→rights validation→transcode→loudness analysis→publish. States uploaded/quarantined/processing/ready/rejected/deleted, jobs idempotentes por hash+profile version. Guardar master lossless privado. Perfis iniciais: AAC-LC128/256k HLS para compatibilidade; Opus96/160k progressivo para clients capazes; FLAC lossless progressivo com Range para hi-res autorizado. HLS lossless apenas após prova cliente/browser/CDN, não arquitetura presumida.

Live: source encoder/SRT ou RTMP ingest autenticado, packager HLS áudio 4–6s target, origin/CDN e now-playing event feed. Não usar WebRTC sem necessidade de baixa latência; rádio convencional tolera HLS. Redundância de ingest, silêncio detectado, filler stream permitido pelo operador, alertas e replay log. Grade/horários com timezone e DST; publicação editorial com revisão e audit.

## Conta e API

OIDC/OAuth Authorization Code+PKCE no desktop pelo navegador sistema; refresh token keychain; logout revoga e limpa cache privado. Sem embedded password form. Catalog query paginada, playback resolve devolve URL temporária e restrictions; bearer da API não vai à CDN. Revogar token reduz acesso futuro conforme TTL; não fingir revogar bytes já baixados.

Sync: playlists/favorites/bookmarks/progress e perfis opcionais, não arquivos locais por padrão. Append-only change log, cursor opaco, revision por objeto, idempotency key. Conflicts não descartam edição: merges por entry IDs, delete tombstone e resposta409 para base revision antiga. Progresso de audiobook resolve por evento de sessão e oferece conflito; “maior posição sempre vence” é incorreto quando usuário volta.

## Segurança e operações

Roles listener/editor/operator/admin; tenant scope desde modelo, sem multi-tenant deployment obrigatório. Upload com tamanho/MIME/codec limits, AV quando apropriado, SSRF de ingest externo, quotas e rate limiting. Signed URLs expiram; CORS restrito; secret rotation; backups restaurados em drill. Analytics opt-in ou agregado conforme configuração, sem capturar caminhos locais. Retenção/auditoria e export/delete de dados documentados.

Gate inicial:1000 sessões simultâneas simuladas, API read p95≤250ms excluindo CDN, SLO API99.9%/mês alvo, playback startup p95≤3s em rede de referência, load tests separados para lossless. Custos medidos por bandwidth/cache hit/transcode/storage. Não contratar escala global antes do baseline.

Entregas: mock provider no desktop antes backend; API/catalog/ingest/VOD; conta/sync; live/editorial; operational readiness; portal de extensões. Billing/DRM/monetização requerem novos ADRs e requisitos.
