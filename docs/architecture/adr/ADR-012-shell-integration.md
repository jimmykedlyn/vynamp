# ADR-012 — integração nativa com o shell, menus de contexto e pacotes de ícones

Versão: 0.1.0 · Data: 2026-11-05 · Estado: decisão do mantenedor; especificação normativa de integração e assets.

Status: aceito pelo mantenedor em 2026-11-04. Complementa [ADR-006](ADR-006-multiplatform.md) e [ADR-008](ADR-008-vynamp-brand.md); orienta as tarefas do épico E16 ([TASK-E16-03](../../../tasks/items/TASK-E16-03.md), [TASK-E16-04](../../../tasks/items/TASK-E16-04.md), [TASK-E16-05](../../../tasks/items/TASK-E16-05.md) e [TASK-E16-08](../../../tasks/items/TASK-E16-08.md)).

## Contexto

O Vynamp é um reprodutor de áudio desktop e mobile voltado para alta fidelidade e controle de coleção. A experiência de uso cotidiana depende de integração profunda e sem atritos com o gerenciador de arquivos e o shell de cada sistema operacional:

1. **Windows 11:** O Windows Explorer introduziu um menu de contexto moderno e simplificado. Entradas nesse menu exigem identidade de pacote (sparse package com localização externa via MSIX) e uma implementação COM da interface `IExplorerCommand`. Os registros tradicionais em `Software\Classes` só aparecem após o usuário clicar em "Mostrar mais opções" (Shift+F10), o que degradaria a experiência se fosse o único mecanismo.
2. **Windows 10 e legados:** Utilizam o menu de contexto clássico via ProgIDs em `HKEY_CURRENT_USER\Software\Classes`. O instalador padrão do Vynamp é NSIS em escopo de usuário (`$LOCALAPPDATA\Programs\Vynamp`), sem requerer privilégios de administrador (UAC).
3. **Assinatura de código no Windows:** Pacotes esparsos exigem assinatura válida com certificado confiável na máquina para que a API `PackageManager.AddPackageByUriAsync` registre a identidade. O certificado oficial de produção é um custo/decisão externa em aberto; o ambiente de desenvolvimento e testes locais utiliza certificado autoassinado instalado na raiz de certificados do usuário.
4. **Linux:** Não há um gerenciador de arquivos único. A integração universal via Freedesktop Desktop Actions (`.desktop`) fornece verbos no launcher, mas o menu de contexto dentro do gerenciador de arquivos exige extensões ou ServiceMenus específicos para os ambientes dominantes: GNOME Files (Nautilus), KDE Plasma (Dolphin) e XFCE/Cinnamon (Thunar e Nemo).
5. **macOS:** O Finder restringe a injeção arbitrária em menus de contexto. A integração legítima na 1.0 se dá via Quick Actions (Finder Services baseados em `NSServices`), declarações de tipo de documento em `CFBundleDocumentTypes` e callbacks padrão de `NSApplicationDelegate`.
6. **Android:** A integração ocorre via Android Intent Filters para `ACTION_VIEW` e `ACTION_SEND` / `ACTION_SEND_MULTIPLE`, mediada pelo Storage Access Framework (SAF).
7. **Ferramentas auxiliares:** Em 2026-11-03, o mantenedor decidiu que o **Vynamp Restore** e o **Vynamp Converter** são executáveis desktop separados (`vynamp-restore` e `vynamp-converter`), construídos sobre as mesmas bibliotecas do player, mas com escopos funcionais mais amplos que os overlays do protótipo. Ambas as ferramentas devem figurar como ações de primeiro nível no menu de contexto do Vynamp.
8. **Pacotes de ícones de arquivo:** O protótipo web de referência (rev.38) introduziu conceitos visuais distintos para identificação de tipos de áudio. O mantenedor decidiu disponibilizar 3 famílias de ícones (Documento, Disco e Sinal) para 17 formatos de áudio, permitindo que o usuário escolha visualmente a apresentação no sistema operacional (Windows e Linux) e na interface interna (macOS e Android).

## Decisão

### 1. Arquitetura de Integração por Plataforma

| Plataforma | Menu Principal (Contexto) | Mecanismo de Registro | Escopo de Instalação | Verbos Padronizados |
|---|---|---|---|---|
| **Windows 11 x64** | Submenu nativo "Vynamp" de 1º nível | Pacote esparso MSIX assinado + `IExplorerCommand` COM out-of-process/in-process | User scope (`%LOCALAPPDATA%\Programs\Vynamp`) via `PackageManager` | Reproduzir, Abrir em nova lista, Adicionar à lista, Adicionar à fila, Editar etiquetas, Converter, Restaurar com Vynamp Restore |
| **Windows 10 x64** | Menu clássico / "Mostrar mais opções" | HKCU ProgID (`Software\Classes\Vynamp.<ext>\shell`) | User scope via NSIS | Mesmos 7 verbos clássicos com `MUIVerb` e `SubCommands` |
| **Linux x64** | Menu de contexto nativo + Desktop Actions | Nautilus extension (Python/C), Dolphin ServiceMenu (`.desktop`), Thunar Custom Actions | User scope (`~/.local/share/`) ou System (`/usr/share/`) via DEB | Reproduzir, Adicionar à lista, Adicionar à fila, Converter, Restaurar com Vynamp Restore |
| **macOS (Intel/Apple Silicon)** | Quick Actions no Finder + Abrir com | `NSServices` no `Info.plist` + `CFBundleDocumentTypes` | Application Bundle (`/Applications` ou `~/Applications`) | Reproduzir no Vynamp, Adicionar à fila do Vynamp, Converter, Restaurar |
| **Android arm64** | Diálogo Abrir com / Compartilhar | `AndroidManifest.xml` com intent-filters `VIEW` e `SEND`/`SEND_MULTIPLE` | APK / AAB instalado pelo sistema | Reproduzir, Enfileirar no Vynamp |

### 2. Estrutura do Submenu "Vynamp"

No Windows 11 (menu moderno), Windows 10 (menu clássico em cascata) e Linux (gerenciadores com suporte a submenu), as ações são agrupadas sob o título **Vynamp** com o ícone oficial da marca (Conceito C / Pulso em 16×16):

1. **Reproduzir** (Verbo padrão / default ao clicar duas vezes): Limpa a lista atual e inicia a reprodução imediatamente com os arquivos/pastas selecionados. CLI: `vynamp --play <paths...>`.
2. **Abrir em nova lista:** Cria uma nova aba de playlist com o nome da pasta ou da primeira faixa e inicia o playback. CLI: `vynamp --new-playlist <paths...>`.
3. **Adicionar à lista:** Acrescenta os itens selecionados ao final da playlist ativa no momento. CLI: `vynamp --add-playlist <paths...>`.
4. **Adicionar à fila:** Enfileira os itens na fila dinâmica de reprodução prioritária (Queue), sem alterar a playlist persistida. CLI: `vynamp --enqueue <paths...>`.
5. **Editar etiquetas:** Abre o diálogo de edição rápida de metadados/tags com todos os arquivos selecionados carregados em lote. CLI: `vynamp --edit-tags <paths...>`.
6. **Converter:** Abre o executável dedicado **Vynamp Converter** com os arquivos selecionados adicionados à fila de conversão. CLI: `vynamp-converter <paths...>`.
7. **Restaurar com Vynamp Restore:** Abre o executável dedicado **Vynamp Restore** com os arquivos selecionados prontos para análise acústica e restauração. CLI: `vynamp-restore <paths...>`.

### 3. Preservação de Lotes (Batching Atômico)

Para evitar que a seleção de múltiplos arquivos (10, 50 ou 100 itens) dispare dezenas de instâncias paralelas de processos do player:
- No Windows 11, o `IExplorerCommand` recebe um `IShellItemArray`, iterando todos os itens em uma única chamada de método `Invoke` e transmitindo o lote completo via IPC para o processo primário único (conforme [TASK-E16-02](../../../tasks/items/TASK-E16-02.md)).
- No Windows 10 clássico, utiliza-se `MultiSelectModel=Player` com verificação de DDE/DropTarget ou pooling com debounce determinístico no dispatcher IPC.
- No Linux, a linha de comando do `.desktop` utiliza `%U` (sem aspas individuais), repassando todos os URIs em uma única invocação.
- No macOS, o AppKit entrega o lote completo no método `application(_:openFiles:)`.

### 4. Sistema de Pacotes de Ícones de Arquivo

O Vynamp define três famílias visuais de ícones para arquivos de áudio, derivadas diretamente do MVP web rev.38:
1. **Documento (`document`):** Metáfora de arquivo/partitura com canto superior direito dobrado, etiqueta com nome da extensão e nota musical/glifo no rodapé. Ideal para usuários que preferem organização clássica de documentos.
2. **Disco (`disc`):** Metáfora de disco de vinil estilizado, com ranhuras concêntricas escuras e rótulo central colorido contendo o nome do formato e informações de qualidade. Reflete a identidade do Vynamp e o conceito analógico.
3. **Sinal (`signal`):** Metáfora técnica/estúdio com chassis grafite escuro, barras verticais de forma de onda/espectro sonoro e tipografia contrastante em verde-lima ou ciano. Indicada para perfis de produção e análise.

**Formatos cobertos (17 tipos):**
- *Lossless:* FLAC, WAV, AIFF, ALAC, APE, WV.
- *Lossy:* MP3, AAC, M4A, OGG, OPUS, WMA, MP2, WEBM.
- *DSD / Hi-res:* DSF, DFF.
- *Genérico:* AUDIO (para extensões desconhecidas associadas ao player).

**Aplicação por SO:**
- **Windows e Linux:** A escolha da família de ícones ativa altera os ícones de arquivo exibidos pelo sistema operacional no Explorer/gerenciador de arquivos. No Windows, são gerados arquivos `.ico` contendo resoluções de 16×16, 24×24, 32×32, 48×48, 64×64, 128×128 e 256×256 pixels, registrados nas chaves `DefaultIcon` dos respectivos ProgIDs. No Linux, os SVGs e PNGs são instalados na hierarquia `hicolor` do usuário.
- **macOS e Android:** Devido a restrições de segurança e sandbox dos sistemas operacionais (onde o bundle do app não pode reescrever seus próprios tipos de documento dinamicamente sem invalidar assinaturas), a seleção da família de ícones afeta exclusivamente a exibição dentro da interface do Vynamp (seletor de arquivos interno, lista de faixas e painel de metadados).

### 5. Identidade das Ferramentas Auxiliares

Seguindo a especificação de marca [ADR-008](ADR-008-vynamp-brand.md):
- A logo do Vynamp principal é o **Conceito C / Pulso** (V grafite com play âmbar `#FFBF69`).
- O **Vynamp Restore** utiliza o símbolo C com o badge de restauração (glifo de diagnóstico/onda restaurada em ciano `#66D9EF`).
- O **Vynamp Converter** utiliza o símbolo C com o badge de conversão (setas circulares em verde-lima `#B8F35A`).
- Ambas as ferramentas possuem executáveis próprios e arquivos de ícone dedicados (`vynamp-restore.ico`, `vynamp-converter.ico`).

### 6. Seção de Preferências: "Integração com o Sistema"

A interface de preferências do Vynamp contará com uma aba dedicada:
- **Associações de arquivos:** Lista de extensões suportadas com checkboxes e estado de registro atual no sistema. Botão para acionar a tela padrão de aplicativos padrão do SO.
- **Menu de contexto:** Chave geral para habilitar/desabilitar a integração de menu de contexto, e seletores individuais para visibilidade das ações auxiliares ("Converter" e "Restaurar com Vynamp Restore").
- **Aparência dos ícones de arquivo:** Seletor visual das três famílias (Documento, Disco, Sinal), com preview em tempo real e botão "Aplicar aos arquivos do sistema" (no Windows e Linux).

## Alternativas Consideradas

1. **Distribuir exclusivamente via MSIX da Microsoft Store:**
   - *Rejeitado.* Elimina a possibilidade de instalação portátil (ZIP portable) e o instalador leve per-user NSIS preferido por usuários tradicionais de desktop. A solução de pacote esparso (sparse package) oferece o menu moderno do Windows 11 mantendo o executável livre e portátil.
2. **Shell Extension clássica em DLL in-process no Windows Explorer:**
   - *Rejeitado.* Extensões in-process em C++ podem causar travamentos gerais no Explorer caso ocorra uma falha de memória. O `IExplorerCommand` moderno fora de processo ou desacoplado é muito mais seguro e alinhado aos padrões atuais da Microsoft.
3. **Manter Restore e Converter como diálogos embutidos no processo do player:**
   - *Rejeitado pelo mantenedor em 2026-11-03.* O usuário prefere executáveis separados para tarefas pesadas de restauração de áudio e transcodificação em lote, evitando impacto na estabilidade e tempo real da engine de playback do player.

## Consequências

- O build do Vynamp no Windows gera o pacote esparso (`AppxManifest.xml`) e o script de registro `vynamp-sparse.msix`.
- O instalador NSIS executa o registro do pacote esparso no escopo do usuário via PowerShell ou chamada nativa de WinRT `PackageManager`.
- Criação de `assets/icons/` no repositório com o catálogo interativo `preview.html`, manifesto `manifest.json` e fontes vetoriais SVG das três famílias e das ferramentas.
- O documento normativo `docs/platforms/03-system-integration.md` é atualizado para a versão 0.8.0 com todos os detalhes operacionais de registro e contratos CLI.
- As tarefas `TASK-E16-03`, `TASK-E16-04`, `TASK-E16-05` e `TASK-E16-08` têm seus contratos e critérios de aceite refinados para cobrir o escopo estabelecido.
