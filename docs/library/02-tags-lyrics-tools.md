# Tags, letras, capas e ferramentas

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

Editor de tags em lote com preview antes de write. Campos mistos aparecem “valores diferentes”; vazio e não alterar são estados distintos. Preservar tags desconhecidas/encoding quando possível. Write em arquivo temporário no mesmo volume + sync/rename; manter backup configurável e relatório. Arquivo tocando pode exigir esperar release; nunca corromper devido lock. Não sincronizar alterações a provider remoto sem API/permissão.

Campos: título, artistas, album artist, álbum, gênero, data, track/disc totals, compositor, comentário, copyright, BPM, key, rating mapeado, ISRC, RG track/album e peak, letras e capítulos. IDs externos e origem separados. Identificação online é opt-in, preview/confidence; não auto-reescrever tags de baixa confiança.

Capa: embutida > imagem da pasta conforme regra > cache online opt-in > fallback original. PNG/JPEG/WebP, thumbnails em disco por hash e tamanho, orientações aplicadas, imagem decompression-bomb limitada. Letra plain/synced LRC; offset ajustável; linha corrente segue relógio consumido; usuário pode pausar auto-scroll. Provider documenta licença/credit, sem scraping secreto.

Ferramentas M3: converter em batch, extrair faixas CUE, ReplayGain/loudness scan, análise BPM experimental com confidence, renomear por template, organizar pastas, dedupe preview, exportar metadados, gravar fonte autorizada, sleep timer e scheduler. Templates usam whitelist e sanitizam separators/reserved names; plano completo antes aplicar; overwrite desativado por padrão. Converter usa fila de workers limitada e cancelamento que remove temporários, source intacta.

CD M5: playback digital e ripping com offset/secure mode quando backend permitir, metadata opt-in e relatório de erros; checksum externo exige integração licenciada/documentada. Não prometer bit-perfect rip sem verificação. Podcast não é copiado como música sem intenção.

Scheduler: play playlist/station, stop, pause, fade, iniciar/parar gravação, alarm e sleep timer por hora/duração/fim de faixa. Timezone explícita e regras DST; persistir próxima execução. Desligar/suspender OS somente opção habilitada e confirmação de configuração, nunca consequência padrão. Quiet-hours e teto de volume para alarmes. Wake-from-sleep depende de OS e será capability, não promessa.
