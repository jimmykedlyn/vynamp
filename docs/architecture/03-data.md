# Modelo de dados e migrations

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Identidades e schema lógico

| Entidade | Campos centrais | Invariantes |
|---|---|---|
| media_items | id,type,title,duration_ticks,timebase,codec,rate,bits,channels,metadata_revision | origem unknown explícita, duração opcional para live |
| media_locations | id,media_id,source_id,locator,etag,fingerprint,availability | vários locators por item, secrets referenciados por key ID |
| artists / media_artists | artist_id,name,sort_name,role,position | multi-value ordenado, sem overwrite de créditos |
| albums | id,title,album_artist_id,release_date | edição/versão pode diferir do nome |
| playlists | id,name,kind,revision,rules_json | rules validadas AST, sem SQL livre |
| playlist_entries | id,playlist_id,media_id,position_key,range_start/end | IDs estáveis mesmo com duplicata |
| queue_sessions / queue_entries | id,seed,index,repeat,position,generation | não confundir snapshot com playlist |
| bookmarks | id,media_id,position_ticks,label,updated_at | posição dentro de range conhecido |
| play_events | id,session,media,kind,consumed_frames,time | append-only e idempotência |
| source_accounts | id,provider,subject,keychain_ref | não guardar access/refresh token plaintext |
| device_profiles | id,backend,capabilities,settings,preset_id | capability snapshot revalidado |
| feeds / episodes | id,guid,source,download_state,progress | guid scoped ao feed |
| jobs | id,kind,state,checkpoint,lease_until,error | recovery após crash |
| packages / grants | id,version,hash,state,permissions | grant por versão major/changes |
| sync_changes | sequence,entity_id,revision,operation | tombstones e cursor por conta |

Paths locais normalizados apenas para comparação, preservar representação de exibição; case sensitivity por filesystem. Não assumir UTF-8 em OS path internals. Locators remotos redacted em diagnóstico; canonicalization não remove query sem saber semântica. Dates UTC em contratos, release dates sem timezone quando só dia/ano.

## Migrations

Schema_version monotônico, migrations SQL versionadas com hash, executadas em transação quando possível. Backup consistente antes mudanças destrutivas; validar espaço e integridade. FTS reconstruível em background. Se migration falha, rollback e abrir recovery read-only; não apagar DB para “corrigir”. Downgrade do app só com compatibilidade ou backup restaurado mediante escolha.

Config schema separado do DB: migração determinística, defaults explícitos e unknown fields preservados se seguros. Save temp/sync/rename, nunca sobrescrever com arquivo parcial. Packages e dados plugin têm namespaces e quota. Remover plugin não apaga media/favorites; oferece apagar seus dados.

## Sync futura

Local IDs permanecem mesmo offline; mappings server IDs por provider. Revision/object e client_op_id únicos, device_id não é segredo. Deletes viram tombstone com retention definido. Queue local não sincroniza automaticamente sessão ativa; playlists/favorites/progress sim. Nenhuma dedupe por filename cru entre dispositivos. Testar merge offline, rename, concurrent remove+reorder e reappearance após tombstone.

Locators Android usam Content URI/document IDs e grants, nunca conversão fictícia para path. Store refs scoped ao provider; grant revogado significa unavailable. File rewrite/rename é capability do document provider; confirmar limitações e backup se não oferecer replace atômico. Database/cache vivem em app-private storage. AndroidKeystore/LinuxSecretService/macOSKeychain protegem credenciais conforme host.
