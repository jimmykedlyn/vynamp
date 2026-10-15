# Contratos, eventos e erros

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Ports de domínio

AudioOutput::enumerate/negotiate/open/start/pause/drain/close; Source::capabilities/read/seek/cancel; Decoder::probe/decode/reset/metadata; Library::query/upsert/transaction; Provider::catalog/resolve/refresh/restrictions; Theme::validate/render_binding; PluginHost::invoke/revoke/shutdown. Traits síncronos/async conforme thread; buffers de áudio têm ownership e tamanho explícitos. Não criar API com `Vec<u8>` ilimitado vindo de extensão.

Command envelope: schema_version, request_id UUID, command, payload, issued_at. Exemplos: playback.open(locator,mode), playback.pause, playback.seek(position), queue.insert(entry,after_id), dsp.apply(preset), output.select(profile), source.cancel. Response aceita/rejeita com reason e generation; aceitação não significa áudio já emitido. Acknowledge correlacionável por request ID.

Events: PlaybackStateChanged, TrackChanged, PositionSnapshot, OutputNegotiated, BufferHealth, QueueRevisionChanged, LibraryJobProgress, MetadataChanged, PackageStatus, RecoverableError. Campos sequence/generation/occurred_at/schema_version; control events ordenados por sessão, progress snapshots coalescidos. Subscribers não recebem segredo; stable IDs como referência.

## Tipos de erro

InvalidInput, UnsupportedFormat/Profile, SourceUnavailable, AccessDenied, AuthenticationRequired, TokenExpired, NetworkTimeout, TlsRejected, BufferUnderrun, DeviceBusy/Lost, PluginDenied/TimedOut/Crashed, InvalidPackage, StorageFull, IntegrityFailed, Conflict, Cancelled. Cada erro define retryable, user_message_key, safe_actions, correlation_id e detalhe técnico redacted. Não capturar todos como “erro desconhecido”. Cancelled não vira notificação vermelha de falha.

## Provider API v1

`GET /v1/capabilities`; `GET /v1/catalog/tracks?cursor=&limit=`; `GET /v1/catalog/tracks/{id}`; `GET /v1/stations`; `POST /v1/playback/resolve`; `POST /v1/playback/renew`; `GET /v1/sync/changes?cursor=`; `POST /v1/sync/operations`. Endpoints admin/ingest separados por role. Rate limit429+Retry-After; Problem Details JSON em4xx/5xx; logs request IDs.

Resolve: track/station ID, device capabilities, quality preference e session ID; response session ID, source descriptor, expires_at, restrictions, selected rendition e refresh reference. Locator assinado é segredo efêmero; não persistir no objeto playlist. URL refresh não é playback restart: manter timeline/seq quando suportado. Protocol unknown gera unsupported, não busca por extensão.

Pagination cursor opaco com stable ordering; limit max100. Idempotency-Key em resolve/sync/create jobs, expires24h conforme backend. Versioning /v1, backward additive fields aceitos; required enums unknown explicit unsupported. Auth scopes catalog.read/playback/sync.write/admin.edit; provider externo pode oferecer só subset.

Os exemplos e JSON Schemas em [schemas](../../schemas/README.md) especificam a forma mínima de SourceDescriptor/tema/plugin/preset. A proposta estruturada de API web está em schemas/openapi-platform.json e será refinada/congelada com contract tests em M4; não tratar este catálogo de endpoints como servidor existente.
