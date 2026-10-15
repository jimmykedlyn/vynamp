# Biblioteca, busca e playlists

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

SQLite local com migrations e FTS; banco é índice, arquivos são fonte de mídia. Importação incremental por scan jobs; observar mudanças com debounce e reconciliação periódica. Root offline (NAS/unplug) fica indisponível, não é removido. Identity baseada em locator + fingerprint barato; full hash opcional para dedupe, sem calcular todo acervo no primeiro launch.

Entidades: MediaItem, MediaLocation, Album, Artist, Playlist, PlaylistEntry, QueueSession, Bookmark, PlayHistory, Source, DeviceProfile. Arquivo com vários artistas mantém relação n:n. Track virtual CUE/chapter referencia range do mesmo media asset. Metadata track number e sort keys não se confundem com ordem de playlist.

Views: músicas, álbuns, artistas/album artists, gêneros, anos, pastas, recente, favorito, hi-res, streams. Colunas configuráveis, grouping por template seguro, sorting estável e virtualização da lista. Busca FTS por termos, filtro faceted e query textual opcional; usuário nunca fornece SQL. Large library de 100k como gate1.0; 1M é alvo M5.

Playlist editável tem revision; undo/redo de inserir/remover/reordenar pelo menos100 operações em sessão. Duplicatas permitidas; comando remover duplicatas não muda fila sem pedido. Smart playlists guardam AST allowlisted: field/operator/value, sort, limit, scope; avaliação indexada e cancelável. Folders playlists atualizam por regras; lock playlist impede edição, não playback.

Fila persistida separadamente com item ID, play generation, index, seek position, shuffle seed, repeat policy e source state. Checkpoint a cada5s e eventos; restore após crash oferece retomar, sem autoplay padrão. Item desaparecido é marcado e pulado com erro, mantendo snapshot. “Tocar próxima” tem ordem estável em múltiplos inserts.

Histórico local registra evento de começo, consumo efetivo e conclusão; track played padrão≥50% ou240s, exclui seeks que não foram ouvidos. Scrobble providers usam política própria, opt-in, retries idempotentes; sem publicar o histórico local por padrão.

Aceite: move/rename sem perder rating/bookmark; offline NAS sem exclusão; 100k search p95≤100ms warm e page scroll sem decode de todas as capas; crash recovery; Unicode; duplicate tracks; CUE ranges; sorting enquanto toca não altera queue; undo não restaura arquivo fisicamente deletado.
