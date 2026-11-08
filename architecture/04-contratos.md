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

Os exemplos e JSON Schemas em [schemas](../schemas/README.md) especificam a forma mínima de SourceDescriptor/tema/plugin/preset. A proposta estruturada de API web está em schemas/openapi-platform.json e será refinada/congelada com contract tests em M4; não tratar este catálogo de endpoints como servidor existente.

## Semantica da API da plataforma (M4)

Versão 0.7.0 · Contrato proposto para M4, não servidor implementado.

O [OpenAPI](../schemas/openapi-platform.json) define 25 paths iniciais. Esta página define regras que o schema estrutural sozinho não garante. TASK-E09-02 congela o contrato após testes mock/cliente/servidor; alterações major exigem nova versão.

## Autenticação e autorização

Capabilities pode ser público. Catálogo público depende da política do provider; a proposta exige bearer nos demais endpoints para simplificar direitos. `catalog.read`, `playback`, `sync.write`, `account.self`, `registry.read`, `admin.catalog`, `admin.ingest`, `admin.registry` são escopos de operações. Scopes não substituem verificação por objeto/território/conta.

Listener lê catálogo permitido e resolve mídia autorizada. Editor altera catálogo e solicita publicação. Operator configura ingest/live/grade. Admin administra publishers/revogação e políticas. Acesso admin usa sessão separada e pode exigir MFA. Não incluir secrets de ingest no objeto Station público. OAuth issuer/client ID serão configuração confiável, não aceitos cegamente de URL de estação.

## Resolve e renew

Request inclui ID estável, tipo de mídia, sessão do cliente, preferência de qualidade e codecs/transports aceitos. Seleção prefere codec/profile suportado, qualidade compatível com direitos e limite de taxa. `lossless` não cai silenciosamente para lossy: retornar alternativa proposta ou erro com capacidade faltante. `auto` pode selecionar rendition diferente.

Response contém sessão server, SourceDescriptor, rendition, expiry e referência opaca para renew. URLs de mídia não usam API bearer. Refresh reference é secreto, vinculado a conta/sessão e não URL pública de renovação. Repetir resolve com mesma Idempotency-Key e corpo retorna mesma operação enquanto registro existir; mesmo key com outro corpo retorna 409.

Renew é permitido antes expiração e após falha de acesso atribuível a expiry. Revalidar rights; não ampliar download/gravação automaticamente. HLS renew conserva sequence/timeline quando source permanece. Interrupção de catálogo/direitos gera erro específico, não retry infinito.

## Paginação, revisões e escrita

Cursor opaco inclui posição/revisão de ordenação e expiração; nunca confiar em cursor como autorização. Limit 1–100. Busca vazia lista conteúdo autorizado. Item deletado gera 404; expired cursor gera erro de reinício seguro sem apagar biblioteca do cliente.

PATCH de track aceita campos parciais e `base_revision` obrigatório; revisão antiga retorna 409 com versão atual sanitizada. Publicação exige asset ready, rights válidos e metadados mínimos. Trocar asset cria revision e preserva histórico; não modificar rendition em URL imutável já publicada.

## Modelos sincronizados

| Tipo | Payload mínimo | Regra |
|---|---|---|
| playlist | name, kind, rules quando smart | Nome não é identity; revision por objeto |
| playlist-entry | playlist_id, entry_id, media_ref, position_key, range opcional | Duplicatas têm entry IDs distintos; locator secreto não é mídia |
| favorite | media_ref, enabled | Mudança explícita, não inferida de rating |
| bookmark | media_ref, position_ticks, timebase, label | Posição/range validados; edição mantém ID |
| progress | media_ref, session_id, position_ticks, timebase, consumed_frames, updated_at | Retorno intencional não perde para posição maior |

`media_ref` contém provider/id ou fingerprint local opcional. Path local não sobe por padrão; servidor não recebe acesso a arquivo do dispositivo. Payload por tipo tem schema específico no contrato final. Envelope aberto atual não autoriza campos arbitrários no servidor.

Batch máximo 100 operações; resultados por item `applied/duplicate/conflict/denied`, com revision atual quando autorizado. A proposta usa resposta 200 para batch parcialmente aplicado; 400 para envelope inválido, 401 para sessão ausente. Não usar 409 de HTTP para esconder resultados individuais. Delete persiste tombstone; retention inicial 90 dias, cliente com cursor mais antigo precisa full reconcile antes enviar operação antiga. Full reconcile usa GET /v1/sync/snapshot: primeira página cria snapshot_id e watermark consistente, páginas seguintes usam cursor vinculado àquele snapshot. Snapshot expira em 15 min; cliente reinicia se expirado, sem aplicar resposta parcial como coleção completa. Após import, consumir changes a partir do watermark e enviar operações locais preservadas com base revisions corretas.

## Upload, jobs e direitos

Ticket de upload expira em 15 min por padrão, limite 10 GiB na proposta, conta/quota verificada antes emissão. Finalize valida tamanho/hash/probe; uma vez finalizado, repetir é idempotente. Client-provided MIME não basta. Job expõe status/progresso/erro, sem path do worker. Admin publicação e ingest têm audit com ator/ID/revision/timestamp.

Rights inclui territórios, janela de disponibilidade, permissão download/gravação. Ausência de data final não significa ausência de todos os limites. Checar na publicação e no resolve/renew; cache de autorização com TTL curto e revogação. Export/account delete são jobs consultados em GET /v1/me/jobs/{id}, sempre vinculados à conta; result de export contém link privado com expiry. A criação exige reautenticação quando aplicável; disponibilizar artifact privado com expiry, sem link por log público.

## Erros e limites

Problem Details contém `type/title/status/code/request_id`, detalhes sanitizados e ação sugerida em extensão documentada. 401 permite refresh uma vez; 403 rights/permission não é retentado sem mudança; 404 encerra recurso; 409 preserva edição para merge; 429 respeita Retry-After; 5xx usa jitter e limite. Body máximo 1 MiB nos endpoints normais, operações de upload direto fora desse body. JSON malformed/unknown required enum retorna 400. Proxy/timeouts não podem bloquear o player local.

## Validação

Contract tests verificam escopos, IDOR, partial sync, idempotência, expiry, direitos, alter revision, publish-before-ready e token redaction. O player usa provider mock antes qualquer infraestrutura real. Resolver source não prova que cada codec/URL vai tocar: registry e erros do cliente continuam necessários.

Provider remoto resolve somente HTTP/ICY/HLS autorizados. `android-content`/`local-file` existem no contrato cliente para locators locais e nunca são respostas da plataforma web nem destinos automáticos de manifest remoto.
