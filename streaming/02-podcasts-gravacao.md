# Podcasts, audiobooks, gravação e offline

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

Feeds RSS/Atom parse sem entidades externas; enclosure selecionado por MIME/capability. GUID estável por feed, fallback hash de enclosure/title/date. Redirect de feed atualiza URL canônica após validação. Poll default6h com jitter e condicional ETag/Last-Modified; feed deletion não apaga downloads sem pedido.

Episódios: unplayed/in-progress/completed; posição a cada5s/evento, chapters e bookmarks, velocidade por categoria, skip±15/30s configurável, silence trim opcional com indicação. Downloads: queued/downloading/paused/verifying/completed/failed, concurrency2, checksum/length quando disponível, partial resume validando ETag. Quota2GiB default só para downloads automáticos; manual confirma falta de espaço; limites por feed/data. Auto-delete só conforme política explícita, preservar bookmark.

Audiobooks M4: M4B e faixas agrupadas, posição por livro, chapters, bookmark nomeado, sequência estável; metadados de progresso separados de fila musical. Clientes offline preservam eventos até sync, sem impedir playback.

Gravação M3: rádios/inputs permitidos por usuário ou provider. Preferir passthrough de MP3/AAC quando seguro e frames válidos; encoder PCM→FLAC/WAV/Opus sob escolha. Split por mudança de título tem debounce e CUE; não confiar no metadado como fronteira sample-exact. Início/fim do arquivo registrado em relógio de gravação; duração não inclui pausas não gravadas. Indicar tamanho, espaço livre e origem; abort seguro sem perder segmentos anteriores.

Rules de provider: may_record/may_download, expires_at, max_offline_age e territories. Fonte pública sem descriptor aceita gravação solicitada, com responsabilidade de conteúdo documentada; plataforma own restringe conforme rights. Não remover DRM nem ignorar proibição explícita. Sem upload de gravações automático.

Offline seguro M4: manifest de downloads + arquivo criptografado opcional por política, keychain e entitlement TTL; local files continuam livres de login. Não prometer impedir cópia de áudio capturado no dispositivo. Renew entitlement quando online; expired mostra motivo e não deleta silenciosamente arquivos. Download não será “disponível offline” se somente segmentos voláteis estão em cache.
