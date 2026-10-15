# Jornadas e comportamento funcional

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Primeira execução

Abrir no tema Classic Graphite, estado Idle, volume 50%, ReplayGain habilitado quando existir tag, EQ flat/desligado, crossfade desligado. Não solicitar conta, indexar todo disco nem tocar automaticamente. Mostrar ações Abrir arquivo, Adicionar pasta e Abrir rádio. Importação de pastas opcional, preview de quantidade e progresso cancelável.

## Arquivos e biblioteca

Arrastar arquivos no player substitui a fonte após intenção explícita; arrastar na playlist adiciona ao final; no marcador entre linhas insere ali. Diretórios são expandidos em background, sem seguir symlinks recursivos por padrão. Arquivos inválidos não bloqueiam os demais; reportar erros agrupados. Duplo clique toca seleção; seleção múltipla com Enter cria fila na ordem visível.

Busca global combina título, artista, álbum e pasta; responde sem depender da rede. Filtros: codec, resolução, gênero, ano, rating, não reproduzidas e origem. “Remover da biblioteca” e “Excluir arquivo” são ações distintas; exclusão física requer confirmação e usa lixeira quando disponível.

## Fila e playlists

Fila ativa é snapshot independente da playlist editada. “Tocar agora” inicia uma nova sequência; “Tocar em seguida” insere depois da atual; “Adicionar à fila” acrescenta ao final. Alterar ordem não reinicia áudio. Shuffle sem reposição por ciclo, histórico reproduzível para Voltar, seed persistida. Repeat: off / sequência / faixa. Prioridade: item atual repetido manualmente respeita repeat faixa; comandos explícitos Próxima/Anterior vencem repeat.

“Parar após faixa” preserva a fila; “Parar após fila” termina o snapshot. A/B loop usa posições válidas e sai ao trocar faixa. Quando uma origem é live sem DVR, seek e loop ficam indisponíveis com explicação. Salvar playlist preserva URIs e encoding UTF-8; exportação não expõe token.

## Rádio e streaming

Colar URL → validar protocolo → conectar → bufferizar → tocar. Mostrar estação, faixa ICY quando presente e LIVE. Interromper cancela DNS/rede/decoder. Buffering não apresenta progresso falso. Falha recuperável informa reconexão; estação encerrada exige retomada explícita depois do limite. Mudar rádio cancela geração anterior e descarta áudio atrasado.

## Hi-res e dispositivo

Painel mostra origem, decode, ganho/EQ, resample e saída negociada. Usuário escolhe shared, exclusive ou ASIO (se instalado). Selecionar bit-perfect apresenta quais efeitos serão desativados e confirma a troca uma vez. DAC removido pausa; por padrão não redireciona música aos alto-falantes. Preferência “seguir saída padrão” é explícita.

## Temas e extensões

Instalar pacote local → validar → preview → aplicar. Tema inválido mantém o anterior. Plugin apresenta autor, origem, permissões e assinatura; instalação não concede rede ou acesso amplo a arquivos automaticamente. Reverter atualização restaura pacote e settings compatíveis. Safe mode por tecla na abertura e opção no launcher desativa extensões.

## Podcasts e audiobooks

Subscribe por RSS/Atom, baixar conforme preferência, progresso por episódio, retomar posição e velocidade independente. Faixa musical não herda 1.5x do podcast. Marcar concluído só após fim efetivo ou comando explícito; bookmarks não desaparecem ao rescan.

## Teclado e menus

Space play/pause quando foco não é editor; Ctrl+O abrir; Ctrl+L URL; Ctrl+F buscar; Ctrl+P preferências; Ctrl+Q sair; Alt+Left/Right anterior/próxima; setas navegam lista; Delete remove item da playlist. Nenhum atalho apaga mídia física. Atalhos globais configuráveis com detecção de conflito. Todos os ícones têm nome acessível e tooltip.
