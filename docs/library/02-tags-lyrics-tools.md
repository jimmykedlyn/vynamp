# Tags, letras, capas e ferramentas

Versão: 0.7.1 · Data: 2026-10-28 · Estado: especificação proposta, implementação não iniciada.

Editor de tags em lote com preview antes de write. Campos mistos aparecem “valores diferentes”; vazio e não alterar são estados distintos. Preservar tags desconhecidas/encoding quando possível. Write em arquivo temporário no mesmo volume + sync/rename; manter backup configurável e relatório. Arquivo tocando pode exigir esperar release; nunca corromper devido lock. Não sincronizar alterações a provider remoto sem API/permissão.

Campos: título, artistas, album artist, álbum, gênero, data, track/disc totals, compositor, comentário, copyright, BPM, key, rating mapeado, ISRC, RG track/album e peak, letras e capítulos. IDs externos e origem separados. Identificação online é opt-in, preview/confidence; não auto-reescrever tags de baixa confiança.

Capa: embutida > imagem da pasta conforme regra > cache online opt-in > fallback original. PNG/JPEG/WebP, thumbnails em disco por hash e tamanho, orientações aplicadas, imagem decompression-bomb limitada. Letra plain/synced LRC; offset ajustável; linha corrente segue relógio consumido; usuário pode pausar auto-scroll. Provider documenta licença/credit, sem scraping secreto.

Ferramentas M3: converter em batch, extrair faixas CUE, ReplayGain/loudness scan, análise BPM experimental com confidence, renomear por template, organizar pastas, dedupe preview, exportar metadados, gravar fonte autorizada, sleep timer e scheduler. Templates usam whitelist e sanitizam separators/reserved names; plano completo antes aplicar; overwrite desativado por padrão. Converter usa fila de workers limitada e cancelamento que remove temporários, source intacta.

CD M5: playback digital e ripping com offset/secure mode quando backend permitir, metadata opt-in e relatório de erros; checksum externo exige integração licenciada/documentada. Não prometer bit-perfect rip sem verificação. Podcast não é copiado como música sem intenção.

Scheduler: play playlist/station, stop, pause, fade, iniciar/parar gravação, alarm e sleep timer por hora/duração/fim de faixa. Timezone explícita e regras DST; persistir próxima execução. Desligar/suspender OS somente opção habilitada e confirmação de configuração, nunca consequência padrão. Quiet-hours e teto de volume para alarmes. Wake-from-sleep depende de OS e será capability, não promessa.

## Letras: componente resumido sob o VU e fontes

Origem: adendo de requisitos "letras integradas ao VU meter" v1.0 (2026-10-25), pedido do usuário. Os IDs `LYR-*` e `LYR-AC-*` abaixo são os do adendo e valem como IDs canônicos até um mapeamento oficial. Este é o documento canônico de letras; [layouts](../design/02-layouts.md#letras-sob-o-vu-2026-10-28), [acessibilidade](../design/03-accessibility-i18n.md#letras-acessibilidade-e-localização), [requisitos](../product/03-requirements.md#rastreabilidade-de-letras-lyr), [decisões](../product/05-decisions.md#decisões-de-letras-2026-10-28) e [TASK-E03-09](../../tasks/items/TASK-E03-09.md) referenciam as âncoras daqui.

O componente **acrescenta** a apresentação resumida sob o VU e a prioridade de fontes; **não substitui** o painel completo de letras já previsto (ver [requisitos anteriores](#requisitos-anteriores-que-continuam)). Não refaz o layout aprovado nem substitui os 12 VU meters estéreo originais.

### Níveis de evidência

Quatro níveis nunca se misturam neste documento:

| Nível | Situação hoje |
|---|---|
| Especificado | Todos os LYR-01..15 e LYR-AC-01..12 (proposta; este PR é só Markdown) |
| Implementado no protótipo | Relatado: revisão 14 da referência HTML/SVG/JS do Codex, **fora deste repositório**; nenhum arquivo dela é linkado aqui |
| Validado no protótipo | Relatado: testes e resultados de validação da referência, em Chrome/Windows; **evidência relatada da referência externa, não verificada neste repositório** |
| Nativo | **Nada** implementado ou validado. O protótipo não prova o aplicativo nativo nem o MVP inteiro |

A evidência do protótipo não certifica o nativo, todos os navegadores, todas as tags existentes nem conformidade WCAG integral. Registrar a referência no repositório do produto fica pendente (D-LYR-08).

### Componente resumido sob o VU

| ID | Requisito |
|---|---|
| LYR-01 | No layout estendido aprovado, o componente fica imediatamente abaixo do VU, integrado à superfície da skin, sem moldura ou caixa adicional. |
| LYR-02 | Três posições de texto: linha anterior acima, atual no centro, próxima abaixo. As adjacentes têm menor destaque, mas legíveis; a atual usa peso e contraste superiores. |
| LYR-03 | Cada posição ocupa uma linha visual. Texto longo não quebra o grid nem cria rolagem horizontal da página; há acesso ao texto completo sem truncamento. |
| LYR-04 | Na passagem normal ao próximo trecho em reprodução, o conjunto sobe e a próxima linha ocupa o centro. Pausa congela o acompanhamento. |
| LYR-05 | A posição efetiva de reprodução é a fonte única de tempo. Seek, troca de faixa, Stop e reinício recalculam o trecho; sem relógio próprio acumulado. No nativo vale o relógio consumido (ver [engine](../audio/01-engine.md)). |
| LYR-06 | Com redução de movimento ou salto de posição, atualiza de imediato, sem percorrer versos intermediários. A animação nunca altera o instante usado para escolher o trecho. |

Antes do primeiro timestamp mostra-se um estado discreto de espera; a primeira linha não é repetida como se já fosse cantada. Nas extremidades, posições sem linha anterior ou próxima ficam vazias. Um segmento sincronizado vazio pode indicar intervalo instrumental, sem marcar a faixa inteira como instrumental.

Valores iniciais da revisão 14 do protótipo (relatados): linha de 26 DIP no desktop e 30 DIP em tela estreita; texto atual 14 DIP, adjacentes 13 DIP; deslocamento ascendente de 260 ms. São referências sujeitas ao sistema de tokens e à escala de texto, não tamanhos fixos que autorizem corte. Posicionamento nos layouts: [layouts](../design/02-layouts.md#letras-sob-o-vu-2026-10-28).

### Fontes, metadados e sincronização

| ID | Requisito |
|---|---|
| LYR-07 | Ler automaticamente letras embutidas no arquivo local quando contêiner, campo e codificação forem suportados; sem importação manual quando já há fonte utilizável. |
| LYR-08 | Texto sem timestamps não autoriza estimar o que é cantado. Informar "A letra está nos metadados, mas não tem sincronização" e oferecer "Ver letra completa". |
| LYR-09 | Permitir importar um LRC local e voltar explicitamente à fonte dos metadados. LRC inválido mostra erro compreensível e preserva a fonte válida anterior. |
| LYR-10 | Menu de opções junto ao título da letra: Mostrar letra, Desligar letra, Música instrumental / sem letra, Importar LRC e Usar letra dos metadados. Nenhum select permanente é necessário. |

Ordem de escolha: **LRC explicitamente importado para a faixa na sessão, depois metadados embutidos, depois estado informativo sem letra utilizável**. A leitura inicial usa os metadados; importar um LRC é escolha explícita que os sobrepõe; "Usar letra dos metadados" remove a substituição e tenta de novo a fonte embutida. Descobrir arquivos auxiliares ou consultar provedores silenciosamente exige requisito separado (a busca online segue opt-in, conforme a seção de capas e letras acima).

A tabela abaixo é a **baseline relatada da referência do protótipo**, não suporte nativo. O nativo depende de [TASK-E03-04](../../tasks/items/TASK-E03-04.md) e do plano 04 (metadados e CUE) da biblioteca privada `vynamp-decode` (https://github.com/jimmykedlyn/vynamp-decode).

| Família | Cobertura relatada da referência / ressalva |
|---|---|
| ID3v2.3 / ID3v2.4 | USLT, SYLT em milissegundos e campos textuais de letras reconhecidos pelo leitor; tratar codificações suportadas e estruturas inválidas |
| FLAC | Letras em comentários Vorbis, inclusive conteúdo LRC nos campos reconhecidos |
| Ogg Vorbis / Opus | Comentários Vorbis / OpusTags com campos de letras reconhecidos |
| M4A / MP4 | Campo `©lyr` e campo livre de letras reconhecido pelo leitor |
| WAV / RIFF | Bloco ID3 com os campos de letras suportados; não presumir letras em todo WAV |
| LRC local | Timestamps de linha, múltiplos timestamps por linha e offset; validar estrutura e limites |

**Ter um campo de letra não garante tempos**; o conteúdo pode ser só texto, e a extensão não garante tag válida. Não escrever "suporte universal" com base nessa tabela. SYLT com timestamps em quadros MPEG não é convertido pela referência: exibir indisponibilidade com motivo e alternativa LRC, sem interpretar quadros como milissegundos. Se o nativo converter corretamente, registrar com testes próprios; a limitação do navegador não é proibição do produto.

Seleciona-se o último timestamp menor ou igual à posição atual, respeitando o offset efetivo. Texto sem tempos nunca vira sincronizado. Timing por palavra, tradução e estimativa automática de canto não foram pedidos neste ajuste.

### Estados, persistência e concorrência

| ID | Requisito |
|---|---|
| LYR-11 | Desligar é preferência **global** de exibição, não equivale a marcar música como instrumental; sobrevive à troca de faixa e à reabertura. |
| LYR-12 | A marca Instrumental/sem letra é escolha reversível para a faixa atual; nunca deduzida da ausência de tags. Mostrar letra remove a marca da faixa. |
| LYR-13 | Distinguir: sincronizada, texto simples, procurando metadados, nenhuma letra encontrada, instrumental, desligada, erro e formato não suportado. Ausência de letra, falha de leitura e formato não suportado são estados diferentes. |
| LYR-14 | Mostrar a origem de forma discreta: metadados/campo, LRC local ou demonstração. Dados de exemplo ficam identificados como DEMO e não sugerem canto real em demonstração silenciosa. |
| LYR-15 | Letra e VU têm estados independentes: desligar a letra não desliga o instrumento; trocar ou desligar o visualizador não apaga a preferência nem a fonte da letra. |

- Persistir a preferência global de exibição e a marca instrumental por **identidade estável de faixa**; no nativo, a identidade da biblioteca (nome de arquivo isolado não basta).
- Na referência, fontes extraídas e LRC importado ficam em cache de sessão e só preferências leves persistem. Persistir arquivos LRC no produto exige especificação própria; não se presume que exista.
- Desligar a letra ou marcar instrumental nunca modifica as tags do arquivo; edição de tags segue o fluxo próprio.
- Leitura assíncrona pertence à faixa que a iniciou e não pode pôr sua letra sobre outra música.
- Importação ou leitura pendente não religa a exibição depois de um Desligar posterior, nem sobrepõe escolha posterior de fonte; resultados obsoletos são descartados ou guardados só no cache da faixa correta.
- Estado sem letra oferece ação útil (como importar LRC); busca online continua opt-in e falha de rede não afeta a leitura local.

### Design system, acessibilidade e desempenho

Usar tokens semânticos da skin, sem cores fixas que só funcionem em um tema. A linha atual não depende só de cor; a atenuação das vizinhas não as torna ilegíveis e o contraste é validado após qualquer transparência. Em alto contraste, hierarquia por peso/marcação. Menu acessível por teclado, rótulos claros, foco visível e retorno ao acionador; sem exigir hover. Mudanças de verso não são anunciadas automaticamente ao leitor de tela (leitura sob demanda; feedback de ações e erros separado). Detalhes em [acessibilidade](../design/03-accessibility-i18n.md#letras-acessibilidade-e-localização).

Desempenho: usar o relógio existente; atualizar a UI só ao mudar trecho ou estado; cancelar transições obsoletas; não reler o arquivo a cada atualização de posição. Extração por trechos, limites de tamanho, validação de offsets e tratamento de arquivos malformados protegem a responsividade. A referência limita (relatado) cada leitura a 16 MiB, o orçamento de leitura a 32 MiB e o texto a 512 KiB; o produto documenta limites e mensagens próprios. Letras entram como texto, nunca HTML executável. Nenhum áudio, letra ou caminho local vai a serviço externo sem adesão explícita; licença e crédito de provedores online continuam obrigatórios.

### Requisitos anteriores que continuam

O componente resumido não substitui os itens obrigatórios do painel de letras: painel/aba completa, pausar auto-scroll, voltar à linha atual, offset −/+100 ms com valor e reset, tamanho do texto, busca online opt-in com crédito/licença, edição, cópia e leitura sob demanda por tecnologia assistiva. O clique em linha para seek permanece **proposta**, salvo decisão posterior do mantenedor. Esses controles não estão todos no componente web; não são promovidos a concluídos porque as três linhas funcionam. Pendências e evidências ficam separadas, e este adendo não reduz a cobertura obrigatória da spec v1.2.

### Critérios de aceite

Estado de todos: **especificado**; validação nativa pendente.

| ID | Cenário verificável / resultado esperado |
|---|---|
| LYR-AC-01 | Faixa com SYLT suportado: origem embutida reconhecida, três posições visíveis, linha atual correta para o relógio de reprodução. |
| LYR-AC-02 | Ao atravessar um timestamp em reprodução, a próxima linha sobe ao centro; pausar mantém o trecho; retomar continua sem deriva acumulada. |
| LYR-AC-03 | Avançar/voltar, Stop e trocar de faixa recalculam de imediato; nunca exibe texto da faixa anterior. |
| LYR-AC-04 | USLT/texto simples: informa ausência de sincronização, permite texto completo, não simula acompanhamento do canto. |
| LYR-AC-05 | Fixtures válidas das famílias declaradas; UTF suportado, LRC com offset e múltiplos tempos, tags truncadas e timestamps não suportados. |
| LYR-AC-06 | LRC válido e depois inválido: o válido acompanha a faixa; o inválido não destrói a fonte anterior; voltar aos metadados restaura a escolha embutida. |
| LYR-AC-07 | Desligar persiste na troca de faixa e na reabertura; instrumental afeta só a faixa escolhida e é reversível; ausência de tags não marca instrumental. |
| LYR-AC-08 | Leitura/importação atrasada artificialmente, seguida de troca de música ou Desligar: nenhum resultado obsoleto altera a música ou a preferência atuais. |
| LYR-AC-09 | Com redução de movimento: acompanhamento correto sem transição ascendente. Teclado, fechamento do menu e retorno de foco verificados. |
| LYR-AC-10 | 12 VUs originais e temas, desktop alto/baixo, tela estreita e texto ampliado: sem distorção, corte essencial ou overflow da página; texto longo tem alternativa completa. |
| LYR-AC-11 | Sem letra, erro e formato não suportado: mensagens distintas, origem honesta, recuperação possível, nenhuma consulta online involuntária. |
| LYR-AC-12 | Regressão: transporte, áudio local, waveform, troca de VU, biblioteca/fila e DSP continuam operando; a letra não altera processamento nem reprodução. |

Evidência do protótipo (relatada, não verificada aqui): testes reproduzíveis de metadados, UI, corrida assíncrona e layout em Chrome/Windows. Status nativo: nenhum critério foi executado. A cobertura geral do MVP da referência permanece incompleta; esta atualização documental não conclui o MVP nem o grupo MVP-09.
