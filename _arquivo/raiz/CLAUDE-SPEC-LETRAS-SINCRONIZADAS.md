# Vynamp — adendo de requisitos: letras integradas ao VU meter

Versão do adendo: 1.0 · Data: 2026-10-25 · Destinatário: Claude, responsável por atualizar as especificações.

## 1. Tarefa para o Claude

Incorpore os requisitos deste documento às specs do Vynamp, mantendo sua terminologia, rastreabilidade e organização. Atualize os requisitos de produto, design, leitura de metadados, acessibilidade, persistência e testes relacionados a letras. Atualize também `SKIN-REQUIREMENTS-CODEX.md` e `SKIN-REQUIREMENTS-GPT-MVP-OBRIGATORIO-v1.2.md`, ou seus sucessores vigentes, evitando duas definições divergentes do mesmo comportamento.

Este adendo registra os pedidos do usuário e o comportamento implementado na referência HTML/SVG/JS, revisão 14. **A referência web não prova implementação no aplicativo nativo nem atendimento integral ao MVP.** Diferencie requisito de produto, evidência de protótipo e pendência de implementação.

Não refaça o layout aprovado nem substitua os 12 VU meters estéreo originais. A tarefa solicitada aqui é atualizar as specs; a implementação nativa deve seguir o fluxo do projeto.

## 2. Origem e integração com os requisitos existentes

Pedidos explícitos do usuário:

> “Exibição da letra da música abaixo do VU meter”; mostrar a linha cantada no momento e as linhas acima e abaixo mais apagadas; animação passando para cima; opção para desligar ou indicar que a música não tem letra.

> “A letra pode ter nos metadados da música.”

Na especificação web v1.2, a seção **5.16 Lyrics** já prevê letras simples e sincronizadas LRC/SYLT, relógio consumido, controles avançados e estados. Este adendo **acrescenta a apresentação resumida sob o VU e explicita a prioridade das fontes e os estados**; não elimina o painel completo de letras.

Pontos a atualizar ou vincular:

| Local | Alteração esperada |
|---|---|
| §3 e §4 — layouts/inventário | Incluir componente de letra resumida imediatamente abaixo do VU no layout estendido aprovado e suas adaptações responsivas. |
| §5.13 — VU/visualizadores | Registrar integração espacial, independência dos estados de letra/VU e preservação dos instrumentos originais. |
| §5.16 — Lyrics | Incorporar o contrato funcional deste adendo; manter os controles e o painel completo já previstos. |
| §5.18 — preferências/privacidade | Registrar persistência de visibilidade e marca instrumental, leitura local e rede somente mediante adesão explícita. |
| §8 e §9 — skins/acessibilidade | Acrescentar tokens, contraste, foco, teclado, movimento reduzido e escalonamento do componente. |
| §12 — implementação/validação | Exigir fonte e status honestos, concorrência segura e testes reproduzíveis. |
| §13 / MVP-09 | Acrescentar subitens rastreáveis de letras; não marcar todo o grupo como concluído por causa deste recurso. |
| `docs/library/02-tags-lyrics-tools.md` e `TASK-E03-09`, se presentes | Consolidar regras de origem, sincronização, metadados, estados e critérios de aceite. |

Confira os documentos canônicos antes de editar; registre qualquer conflito real. Não exclua requisitos anteriores apenas porque ainda não estão no protótipo.

## 3. Requisitos funcionais e visuais

Os IDs `LYR-*` são propostos para rastreabilidade deste adendo; podem ser mapeados para o padrão oficial do repositório, sem perder a correspondência.

| ID | Requisito obrigatório |
|---|---|
| LYR-01 | No layout estendido aprovado, mostrar o componente imediatamente abaixo do VU, integrado à superfície da skin, sem moldura ou caixa adicional. |
| LYR-02 | Exibir três posições de texto: linha anterior acima, linha atual central e próxima linha abaixo. As adjacentes têm menor destaque, mantendo legibilidade; a atual usa peso e contraste superiores. |
| LYR-03 | Cada posição ocupa uma única linha visual. Texto longo não deve quebrar o grid nem criar rolagem horizontal da página. Oferecer acesso ao texto completo para leitura sem truncamento. |
| LYR-04 | Na passagem normal para o próximo trecho durante reprodução, mover o conjunto para cima, com a próxima linha ocupando a posição central. Pausa congela o acompanhamento. |
| LYR-05 | Usar a posição efetiva de reprodução como fonte única de tempo. Seek, troca de faixa, Stop e reinício devem recalcular o trecho; não acumular um relógio independente para as letras. No produto nativo, seguir o relógio consumido previsto na spec. |
| LYR-06 | Respeitar redução de movimento e saltos de posição: atualizar imediatamente, sem percorrer artificialmente todos os versos intermediários. Animação nunca altera o instante usado para selecionar o trecho. |
| LYR-07 | Ler automaticamente letras embutidas no arquivo local, quando o contêiner, campo e codificação forem suportados. Não exigir importação manual quando já existe uma fonte utilizável. |
| LYR-08 | Texto sem timestamps não autoriza estimar o que está sendo cantado. Informar “A letra está nos metadados, mas não tem sincronização” e oferecer “Ver letra completa”. |
| LYR-09 | Permitir importar um LRC local e voltar explicitamente à fonte dos metadados. LRC inválido deve mostrar erro compreensível e preservar a fonte válida anterior. |
| LYR-10 | Disponibilizar menu de opções junto ao título da letra: Mostrar letra, Desligar letra, Música instrumental / sem letra, Importar LRC e Usar letra dos metadados. Nenhum select permanente é necessário. |
| LYR-11 | Desligar é uma preferência global de exibição; não é equivalente a marcar uma música como instrumental. A preferência deve sobreviver à troca de faixa e à reabertura. |
| LYR-12 | A marca Instrumental/sem letra é uma escolha reversível para a faixa atual. Não deduzi-la automaticamente da ausência de tags. Mostrar letra remove essa marca da faixa. |
| LYR-13 | Distinguir fonte sincronizada, texto simples, procurando metadados, nenhuma letra encontrada, instrumental, desligada, erro e formato não suportado. Ausência de letra, falha de leitura e formato não suportado não são o mesmo estado. |
| LYR-14 | Mostrar a origem de forma discreta: metadados/campo, LRC local ou demonstração. Dados de exemplo devem estar identificados como DEMO e não sugerir canto real em uma demonstração silenciosa. |
| LYR-15 | Letra e VU têm estados independentes: desligar a letra não desliga o instrumento; trocar/desligar o visualizador não deve apagar a preferência ou a fonte da letra. |

Antes do primeiro timestamp, mostrar um estado discreto de espera. Não repetir a primeira linha como se já estivesse sendo cantada. Nas extremidades, posições sem linha anterior/próxima ficam vazias. Um segmento sincronizado vazio pode indicar intervalo instrumental, sem marcar automaticamente toda a faixa como instrumental.

## 4. Fontes, metadados e sincronização

Ordem de escolha: **LRC explicitamente importado para a faixa na sessão → metadados embutidos → estado informativo sem letra utilizável**. A leitura inicial usa os metadados; importar um LRC é uma escolha explícita que os sobrepõe. “Usar letra dos metadados” remove essa substituição e tenta novamente a fonte embutida. Não descobrir arquivos auxiliares ou consultar provedores silenciosamente: isso exige um requisito separado.

Cobertura implementada na referência web, a documentar como baseline verificável:

| Família | Cobertura da referência / ressalva |
|---|---|
| ID3v2.3 / ID3v2.4 | USLT, SYLT em milissegundos e campos textuais de letras reconhecidos pelo leitor; tratar codificações suportadas e estruturas inválidas. |
| FLAC | Letras em comentários Vorbis, inclusive conteúdo LRC nos campos reconhecidos. |
| Ogg Vorbis / Opus | Comentários Vorbis / OpusTags com campos de letras reconhecidos. |
| M4A / MP4 | Campo `©lyr` e campo livre de letras reconhecido pelo leitor. |
| WAV / RIFF | Bloco ID3 com os campos de letras suportados; não presumir letras em todo arquivo WAV. |
| LRC local | Timestamps de linha, múltiplos timestamps por linha e offset; validar estrutura e limites. |

**Ter um campo de letra não garante que ele contenha tempos.** O conteúdo pode ser somente texto. A extensão do arquivo também não garante a presença de uma tag válida. Não escrever “suporte universal” com base nessa tabela.

SYLT com timestamps em quadros MPEG não é convertido pela referência. Exibir indisponibilidade com motivo e alternativa LRC; não interpretar quadros como milissegundos. Se o produto nativo suportar conversão correta, registrá-la com testes próprios, sem herdar a limitação do navegador como proibição do produto.

Selecionar o último timestamp menor ou igual à posição atual, respeitando o offset efetivo. Não transformar texto sem tempos em sincronizado. Timing por palavra, tradução e estimativa automática de canto não foram solicitados neste ajuste.

## 5. Estados, persistência e concorrência

- Persistir a preferência global de exibição e a marca instrumental por identidade estável de faixa. No nativo, usar a identidade da biblioteca; nome de arquivo isolado não é suficiente.
- Na referência, fontes extraídas e LRC importado ficam no cache da sessão; somente preferências leves persistem. Persistência de arquivos LRC no produto deve ser especificada separadamente, sem pressupor que já exista.
- Não modificar as tags do arquivo ao desligar a letra ou marcar instrumental. Eventual edição de tags continua no fluxo específico já previsto.
- Uma leitura assíncrona pertence à faixa que a iniciou: não pode colocar sua letra sobre outra música.
- Uma importação/leitura pendente não pode religar a exibição após uma escolha posterior de Desligar, nem sobrepor uma escolha posterior de fonte. Descartar resultados obsoletos ou armazená-los apenas no cache da faixa correta.
- Estado sem letra deve oferecer uma ação útil, como importar LRC; busca online permanece opt-in conforme a spec existente. Falha de rede não deve afetar a leitura local.

## 6. Design system, acessibilidade e desempenho

Referência visual inicial: altura de linha de 26 DIP no desktop e 30 DIP em telas estreitas; texto atual de 14 DIP, adjacentes de 13 DIP; deslocamento ascendente de 260 ms. São valores da revisão 14, sujeitos ao sistema de tokens e à escala de texto — não tamanhos fixos que autorizem corte. Preservar as regras mínimas de alvo, foco e contraste do projeto.

Usar tokens semânticos da skin, evitando cores fixas que só funcionem em um tema. A linha atual não pode depender apenas de cor para se distinguir. A atenuação das vizinhas não deve torná-las ilegíveis; validar o contraste final após qualquer transparência. Em alto contraste, preservar hierarquia por peso/marcação.

Menu acessível por teclado, rótulos claros, foco visível e retorno ao acionador ao fechar. Não exigir hover para descobrir todas as ações. Não anunciar automaticamente cada mudança de verso ao leitor de tela; manter leitura sob demanda e feedback de ações/erros separado.

Adaptar o espaço sem diminuir excessivamente o VU, distorcer suas proporções ou encobrir controles de reprodução. Testar texto ampliado e layouts compactos; a regra de três linhas se aplica onde o componente resumido é exibido, sem impor sua presença no modo Mini quando faltar espaço.

Usar o relógio existente, atualizar o DOM somente ao mudar trecho/estado e cancelar transições obsoletas. Não ler repetidamente o arquivo a cada atualização de posição. Extração por trechos, limites de tamanho, validação de offsets e tratamento de arquivos malformados devem proteger a responsividade. A referência limita cada leitura a 16 MiB, o orçamento de leitura a 32 MiB e o conteúdo textual a 512 KiB; documentar limites próprios e mensagens na implementação final.

Inserir letras como texto, nunca HTML executável. Não enviar áudio, letra ou caminho local a serviços sem adesão explícita. Manter os requisitos existentes de licença/crédito para provedores online.

## 7. Requisitos anteriores que devem continuar nas specs

O componente resumido **não substitui** os seguintes itens já obrigatórios de §5.16: painel/aba completa, pausar auto-scroll, voltar à linha atual, offset −/+100 ms com valor e reset, tamanho do texto, busca online opt-in com crédito/licença, edição, cópia e leitura sob demanda por tecnologia assistiva. Preservar o status de proposta do clique em linha para seek, salvo decisão explícita posterior do mantenedor.

Esses controles não estão todos implementados no novo componente web. Não promovê-los a concluídos porque as três linhas funcionam; manter pendências e evidências separadas. Não usar este adendo para reduzir a cobertura obrigatória da spec v1.2.

## 8. Critérios de aceite

| ID | Cenário verificável / resultado esperado |
|---|---|
| LYR-AC-01 | Abrir faixa com SYLT suportado: origem embutida reconhecida, três posições visíveis e linha atual correta para o relógio de reprodução. |
| LYR-AC-02 | Reproduzir e atravessar um timestamp: próxima linha sobe para o centro; pausar mantém o trecho; retomar continua sem deriva acumulada. |
| LYR-AC-03 | Avançar/voltar, Stop e trocar de faixa: recalcular imediatamente; não exibir texto da faixa anterior. |
| LYR-AC-04 | Abrir USLT/texto simples: informar ausência de sincronização, permitir texto completo, não simular acompanhamento do canto. |
| LYR-AC-05 | Ler fixtures válidas das famílias declaradas; testar UTF suportado, LRC com offset e múltiplos tempos, tags truncadas e timestamps não suportados. |
| LYR-AC-06 | Importar LRC válido e depois inválido: o válido acompanha a faixa; o inválido não destrói a fonte anterior. Voltar aos metadados restaura a escolha embutida. |
| LYR-AC-07 | Desligar e reabrir/trocar de faixa: permanece desligado. Marcar instrumental afeta somente a faixa escolhida e pode ser revertido. Ausência de tags não marca instrumental. |
| LYR-AC-08 | Atrasar artificialmente uma leitura/importação, trocar de música ou desligar: nenhum resultado obsoleto altera a música/preferência atual. |
| LYR-AC-09 | Redução de movimento: acompanhamento correto sem transição ascendente. Navegar por teclado, fechar menu e verificar retorno de foco. |
| LYR-AC-10 | Verificar os 12 VUs originais e temas, desktop alto/baixo, tela estreita e texto ampliado: sem distorção, cortes essenciais ou overflow da página; texto longo tem alternativa completa. |
| LYR-AC-11 | Sem letra, erro e formato não suportado: mensagens distintas, origem honesta e recuperação possível; nenhuma consulta online involuntária. |
| LYR-AC-12 | Regressão: transporte, áudio local, waveform, troca de VU, biblioteca/fila e DSP continuam operando; a letra não altera processamento ou reprodução. |

## 9. Referência implementada e evidências

Caminhos relativos a este adendo:

- `interactive/index.html`: protótipo executável, revisão 14.
- `interactive/lyrics-metadata.js`, `interactive/lyrics.js`, `interactive/lyrics.css`: leitor e componente.
- `interactive/player.js`: integração pelo evento `vynamp:playback`.
- `LETRAS-SINCRONIZADAS.md`: descrição da implementação e seus limites.
- `lyrics-metadata-validation.json`, `lyrics-ui-validation.json`, `lyrics-race-validation.json`, `lyrics-layout-validation.json`: resultados dos testes da referência.
- `interactive/tests/lyrics-*.cjs`: testes reproduzíveis; instruções no README de testes.
- `MVP-COMPLIANCE.md`: cobertura geral ainda incompleta.

As evidências são do protótipo Chrome/Windows; não certificam o produto nativo, todos os navegadores, todas as tags existentes ou conformidade WCAG integral.

## 10. Entrega esperada do Claude

Atualizar as specs pertinentes e apresentar: arquivos/seções alterados, correspondência dos IDs deste adendo com os requisitos canônicos, conflitos resolvidos e pendências preservadas. Separar claramente **especificado**, **implementado no protótipo**, **validado no protótipo** e **implementado/validado no produto nativo**. Não considerar o MVP inteiro concluído por essa atualização documental.
