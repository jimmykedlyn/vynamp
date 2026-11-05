# Integração nativa, associação de arquivos, menus de contexto e pacotes de ícones

Versão 0.8.0 · Contrato normativo de produto e especificação arquitetural (conforme [ADR-012](../architecture/adr/ADR-012-shell-integration.md)).

## Regras comuns e contratos de invocação CLI

A integração profunda com o shell dos sistemas operacionais é requisito essencial para o Vynamp como player desktop de alta fidelidade e controle de biblioteca. Candidacy em Abrir com/chooser é tratada no M2; menu de contexto completo, lote atômico, upgrade/uninstall/portable e pacotes de ícones pertencem ao M3 ([TASK-E16-03](../../tasks/items/TASK-E16-03.md) a [TASK-E16-08](../../tasks/items/TASK-E16-08.md)).

Registrar handler não significa tomar o default sem permissão. A configuração de formatos deriva das capabilities verificadas do artefato, nunca de listas teóricas. A extensão identifica o container candidato, nunca todos os perfis contidos nele. Arquivo com decoder ausente gera erro `UnsupportedFormat` / `UnsupportedProfile`, nunca fallback silencioso. Consulte [Registry](../audio/06-codec-capability-registry.md) e o catálogo em `config/format-catalog.json`.

Preferências do usuário oferecem seleção granular de extensões para registro e botão explícito que aciona o diálogo nativo de aplicativos padrão do SO. O Vynamp nunca sobrescreve escolhas de outros players, não repete prompts agressivos de associação, não requer privilégios de administrador para tocar e não autoassocia formatos não solicitados. Tipos próprios do ecossistema Vynamp utilizam o namespace `com.jimmykedlyn.vynamp`; as extensões já fixadas `.vyntheme`, `.vynplug` e `.vyndiag` são instaladas/importadas por fluxo explícito separado de playback.

### Contrato padronizado de linha de comando (CLI)

Todas as plataformas mapeiam os verbos dos menus de contexto para os seguintes comandos canônicos:

| Verbo no Menu | Comando / Flag CLI | Comportamento Operacional |
|---|---|---|
| **Reproduzir** (Padrão / Duplo clique) | `vynamp --play <paths...>` | Limpa a playlist ativa e inicia a reprodução imediata do primeiro item do lote recebido. |
| **Abrir em nova lista** | `vynamp --new-playlist <paths...>` | Cria uma nova aba de playlist com o nome da pasta ou da primeira faixa e inicia a reprodução. |
| **Adicionar à lista** | `vynamp --add-playlist <paths...>` | Acrescenta todos os itens do lote ao final da playlist ativa no momento, sem interromper o playback atual. |
| **Adicionar à fila** | `vynamp --enqueue <paths...>` | Enfileira os itens na fila dinâmica de reprodução prioritária (Queue), preservando a playlist persistida. |
| **Editar etiquetas** | `vynamp --edit-tags <paths...>` | Abre o diálogo de edição rápida de metadados com todos os arquivos do lote carregados. |
| **Converter** | `vynamp-converter <paths...>` | Executa a ferramenta dedicada Vynamp Converter, abrindo a janela com os arquivos adicionados à fila de transcodificação. |
| **Restaurar com Vynamp Restore** | `vynamp-restore <paths...>` | Executa a ferramenta dedicada Vynamp Restore para análise acústica e restauração das faixas selecionadas. |

Os caminhos informados nos argumentos CLI são sempre validados como dados limpos, nunca concatenados em scripts de shell ou executados diretamente via interpretadores de comando.

---

## Windows 11 x64

### 1. Menu de contexto moderno e Pacote Esparso (Sparse Package)

No Windows 11, o menu de contexto principal do Explorer exige que o aplicativo possua **identidade de pacote** (Package Identity) para registrar comandos sob `desktop4:FileExplorerContextMenus`. O Vynamp adota a arquitetura de **Pacote Esparso com Localização Externa** (Sparse MSIX Package), que permite conceder identidade de pacote ao executável Win32 clássico instalado na máquina sem colocá-lo dentro de contêiner virtualizado e sem alterar o instalador NSIS per-user:

- **Manifesto do Pacote (`AppxManifest.xml`):** Define a identidade `Vynamp.MediaPlayer`, dependência mínima da versão do Windows 11 (Build 22000+) e extensões `windows.fileTypeAssociation` e `desktop4:fileExplorerContextMenus`.
- **Servidor COM `IExplorerCommand`:** Uma biblioteca COM dedicada implementa a interface `IExplorerCommand` (com métodos `GetTitle`, `GetIcon`, `GetState` e `Invoke`).
- **Submenu "Vynamp":** A implementação COM expõe um comando raiz "Vynamp" com o ícone oficial da marca (Conceito C / Pulso em 16×16) e implementa a interface `IExplorerCommandEnumerator` para retornar os comandos filhos (Reproduzir, Abrir em nova lista, Adicionar à lista, Adicionar à fila, Editar etiquetas, Converter, Restaurar com Vynamp Restore).
- **Tratamento de Lotes via `IShellItemArray`:** No método `Invoke`, a extensão COM recebe um ponteiro para `IShellItemArray`, iterando por todos os arquivos selecionados e transmitindo o lote completo em uma única mensagem IPC para o processo primário único do Vynamp (conforme [TASK-E16-02](../../tasks/items/TASK-E16-02.md)), evitando a criação de múltiplos processos.
- **Registro do Pacote Esparso:** Realizado durante a instalação pelo instalador NSIS no escopo do usuário via chamada da API WinRT `PackageManager.AddPackageByUriAsync` ou script auxiliar assinado. Na desinstalação, o pacote é desregistrado via `PackageManager.RemovePackageAsync`.
- **Certificado de Assinatura:** O registro de pacote esparso exige que o pacote esteja assinado por um certificado confiável no computador. Para desenvolvimento e testes locais, o projeto gera um certificado autoassinado instalado na raiz de certificados do usuário (`Cert:\CurrentUser\Root`). A aquisição de certificado comercial Authenticode para distribuição pública é mantida como decisão aberta de infraestrutura.

### 2. Menu clássico do Windows 10 e "Mostrar mais opções"

Para o Windows 10 e o fallback de "Mostrar mais opções" (Shift+F10) no Windows 11, o instalador cria o registro clássico HKCU:
- ProgIDs em `HKEY_CURRENT_USER\Software\Classes\Vynamp.<format>`.
- Chave `DefaultIcon` apontando para o arquivo `.ico` correspondente à família de ícones ativa.
- Submenu em cascata clássico via chave `shell\Vynamp` com valor de string `MUIVerb` = "Vynamp", `Icon` apontando para o executável do player e `SubCommands` listando as ações:
  - `play`, `newplaylist`, `addplaylist`, `enqueue`, `edittags`, `convert`, `restore`.
- Configuração de lote clássico: `MultiSelectModel=Player` com verificação de dispatch único via IPC na inicialização do player.

### 3. Escopo de Instalação e Modo Portátil

- **Instalador NSIS:** Opera no escopo do usuário atual (`%LOCALAPPDATA%\Programs\Vynamp`), sem requerer privilégios de administrador (UAC). Registra capacidades em `HKEY_CURRENT_USER\Software\RegisteredApplications`.
- **Desinstalação:** Remove integralmente os ProgIDs, chaves de registro HKCU, associações e pacotes esparsos próprios, preservando os dados do usuário em `%APPDATA%\Vynamp` e sem adulterar chaves padrão do sistema.
- **Modo Portátil (ZIP):** Não realiza registros automáticos no sistema. O usuário pode acionar a integração explícita através da interface de Preferências, que valida se o caminho do diretório portátil é estável.

---

## Linux x64 — X11 e Wayland

### 1. Especificações Freedesktop e Desktop Actions

O arquivo desktop oficial (`vynamp.desktop`) é instalado em `~/.local/share/applications/` (escopo do usuário) ou `/usr/share/applications/` (instalação de sistema via DEB):
- `Exec=vynamp --play -- %U` (com `%U` desprovido de aspas individuais, repassando todos os URIs em uma única chamada de processo).
- Define as Desktop Actions padronizadas:
  - `[Desktop Action NewPlaylist]` -> `Exec=vynamp --new-playlist -- %U`
  - `[Desktop Action AddPlaylist]` -> `Exec=vynamp --add-playlist -- %U`
  - `[Desktop Action Enqueue]` -> `Exec=vynamp --enqueue -- %U`
  - `[Desktop Action Convert]` -> `Exec=vynamp-converter -- %U`
  - `[Desktop Action Restore]` -> `Exec=vynamp-restore -- %U`

### 2. Extensões de Gerenciadores de Arquivos

Para garantir a presença do submenu "Vynamp" dentro do menu de contexto de clique direito nos gerenciadores de arquivos dominantes:
- **KDE Plasma (Dolphin):** Arquivo de ServiceMenu KIO em `~/.local/share/kio/servicemenus/vynamp-servicemenu.desktop` associado aos tipos MIME de áudio, definindo o submenu "Vynamp" e as ações correspondentes.
- **GNOME Files (Nautilus):** Extensão leve (Nautilus Python ou plugin C compilado) que injeta o item de menu `Nautilus.MenuItem` com submenu agrupado e repassa a lista de arquivos para a instância ativa via socket IPC.
- **XFCE / Cinnamon (Thunar e Nemo):** Ações personalizadas configuradas em `~/.config/Thunar/uca.xml` ou `~/.local/share/nemo/actions/`.

### 3. Pacotes de Ícones e MIME

- Registro de tipos MIME via `shared-mime-info` (`~/.local/share/mime/packages/vynamp.xml`).
- Instalação dos ícones vetoriais SVG e PNGs na hierarquia de temas de ícones `hicolor` (`~/.local/share/icons/hicolor/<size>/mimetypes/`).
- Atualização atômica de caches após alteração de preferências: `update-desktop-database`, `update-mime-database` e `gtk-update-icon-cache`.
- Controle de playback via interface MPRIS D-Bus (`org.mpris.MediaPlayer2.vynamp`), implementando `org.mpris.MediaPlayer2` e `org.mpris.MediaPlayer2.Player` com controle de faixas, busca e metadados.

---

## macOS Intel e Apple Silicon

### 1. Bundle de Aplicativo e Declaração de Tipos

- O aplicativo é distribuído como um único Universal2 Application Bundle (`Vynamp.app`).
- O arquivo `Info.plist` declara `CFBundleDocumentTypes` para cada formato suportado com `CFBundleTypeRole=Viewer`, `LSHandlerRank=Alternate` e tipos de conteúdo uniforme (UTTypes) canônicos (ex.: `public.mp3`, `org.xiph.flac`, `com.microsoft.waveform-audio`).

### 2. Ações Rápidas (Quick Actions) e Serviços do Finder

- O `Info.plist` declara a chave `NSServices` para expor o menu de serviços no Finder ("Serviços" / "Quick Actions" na barra lateral e menu de contexto):
  - "Reproduzir no Vynamp"
  - "Adicionar à fila do Vynamp"
  - "Converter com Vynamp"
  - "Restaurar com Vynamp Restore"
- Os serviços invocam métodos delegados no `NSApplicationDelegate` (`application(_:openFiles:)`), recebendo a lista de URLs e roteando para o motor de playback em Rust sem recriar o processo.

### 3. Exibição de Ícones no macOS

No macOS, os ícones de arquivo associados a um aplicativo são determinados estaticamente pelo arquivo `Info.plist` e assinados no bundle. A alteração dinâmica de pacotes de ícones em tempo de execução no nível do Finder exigiria a reescrita do bundle e quebra da assinatura de código / selo de notarização da Apple.

Portanto, no macOS, o seletor de pacotes de ícones de arquivo altera a apresentação visual **exclusivamente dentro da interface do Vynamp** (na lista de faixas da playlist, no seletor de arquivos embutido e no inspetor de metadados). Os arquivos `.icns` do bundle oficial exibem a família padrão da marca (Conceito C / Pulso).

---

## Android API 29+ (arm64)

### 1. Filtros de Intent e Diálogos do Sistema

O arquivo `AndroidManifest.xml` registra filtros de Intent para integração com o seletor do sistema e o menu de compartilhamento:
- `ACTION_VIEW` com categoria `DEFAULT`: Trata toques em arquivos de áudio no gerenciador de arquivos do dispositivo ou em navegadores.
- `ACTION_SEND` e `ACTION_SEND_MULTIPLE` com tipos MIME de áudio: Recebe compartilhamento de uma ou várias faixas a partir de outros apps.
- Suporte a lote ordenado via `EXTRA_STREAM` e `ClipData`.
- O Vynamp apresenta ao usuário um diálogo de escolha rápida: "Reproduzir agora" ou "Adicionar à fila".

### 2. Storage Access Framework (SAF) e Permissões

- Todas as referências a arquivos externos chegam como `content://` URIs.
- O Vynamp utiliza o Storage Access Framework para abrir os descritores de arquivo (`ContentResolver.openFileDescriptor`), validando permissões persistentes via `takePersistableUriPermission` quando suportado pelo provedor de documentos.
- Caso o usuário revogue a permissão ou o arquivo temporário expire, a playlist mantém a entrada anotada com estado de "indisponível", sem corromper a base de dados de mídia.

---

## Sistema de Pacotes de Ícones de Arquivo

O Vynamp implementa um sistema modular de pacotes de ícones de arquivos de áudio, inspirado na linguagem visual refinada no protótipo web de referência (rev.38, `icons.js`).

### 1. As Três Famílias Visuais

1. **Documento (`document`):** Metáfora de arquivo/partitura clássico com canto superior direito dobrado (folded corner), campo de extensão tipográfica de alta legibilidade, cor semântica por formato e glifo da nota musical no rodapé.
2. **Disco (`disc`):** Metáfora de disco de vinil estilizado com ranhuras concêntricas escuras em grafite `#11131B`, furo central e rótulo central colorido contendo o nome do formato e anel de destaque.
3. **Sinal (`signal`):** Metáfora técnica/estúdio com chassis grafite escuro, barras verticais estilizadas de forma de onda/espectro de áudio e rotulagem técnica com tipografia contrastante.

### 2. Formatos Cobertos (17 Tipos)

| Formato | Categoria | Extensão | MIME Type | Cor Semântica Principal |
|---|---|---|---|---|
| **FLAC** | Lossless | `.flac` | `audio/flac` | Âmbar Vynamp (`#FFBF69`) |
| **WAV** | Lossless PCM | `.wav` | `audio/wav` | Laranja Quente (`#FF9F43`) |
| **AIFF** | Lossless PCM | `.aif`, `.aiff` | `audio/aiff` | Coral Técnico (`#EE5253`) |
| **ALAC** | Lossless Apple | `.m4a`, `.alac` | `audio/mp4` | Roxo Claro (`#9B59B6`) |
| **APE** | Lossless Monkey | `.ape` | `audio/x-ape` | Marrom Dourado (`#D35400`) |
| **WV** | Lossless WavPack| `.wv` | `audio/x-wavpack` | Âmbar Suave (`#F39C12`) |
| **MP3** | Lossy MPEG | `.mp3` | `audio/mpeg` | Azul Elétrico (`#0ABDE3`) |
| **AAC** | Lossy MPEG-4 | `.aac` | `audio/aac` | Ciano Suave (`#48DBFB`) |
| **M4A** | Lossy / Container | `.m4a` | `audio/mp4` | Roxo Vivo (`#5F27CD`) |
| **OGG** | Lossy Vorbis | `.ogg` | `audio/ogg` | Verde Menta (`#1DD1A1`) |
| **OPUS** | Lossy Modern | `.opus` | `audio/opus` | Verde Esmeralda (`#10AC84`) |
| **WMA** | Lossy Windows | `.wma` | `audio/x-ms-wma` | Azul Real (`#2E86DE`) |
| **MP2** | Lossy Legado | `.mp2` | `audio/x-mp2` | Azul Petróleo (`#54A0FF`) |
| **WEBM** | Lossy Web | `.webm` | `audio/webm` | Magenta Claro (`#FF6B6B`) |
| **DSF** | DSD Sony | `.dsf` | `audio/x-dsf` | Ouro Metálico (`#F1C40F`) |
| **DFF** | DSD Philips | `.dff` | `audio/x-dff` | Ouro Intenso (`#F39C12`) |
| **AUDIO**| Genérico | Diversos | `audio/*` | Grafite Neutro (`#8395A7`) |

### 3. Geração de Artefatos Multi-Resolução

- **Windows (`.ico`):** Para cada formato, é gerado um arquivo `.ico` contendo as 7 resoluções padrão do Windows Explorer: 16×16, 24×24, 32×32, 48×48, 64×64, 128×128 e 256×256 pixels. As resoluções de 16px e 24px utilizam versões vetoriais pixel-hinted com simplificação de detalhes para garantir máxima nitidez.
- **Linux (Hicolor):** Instalação dos SVGs canônicos em `hicolor/scalable/mimetypes/` e renderizações PNG em 16×16, 32×32, 48×48, 64×64, 128×128 e 256×256.
- **Identidade das Ferramentas:**
  - Vynamp Player: Símbolo C Pulso (V grafite com play âmbar `#FFBF69`).
  - Vynamp Restore: Símbolo C com badge de restauração (glifo de onda em ciano `#66D9EF`).
  - Vynamp Converter: Símbolo C com badge de conversão (setas circulares em verde-lima `#B8F35A`).

---

## Seção de Preferências: "Integração com o Sistema"

Nas Preferências do aplicativo desktop, a aba "Integração com o Sistema" agrupa:

1. **Associações de Arquivos:**
   - Lista filtrável de todos os 17 formatos suportados, com indicação de estado ("Associado ao Vynamp", "Associado a outro player", "Não registrado").
   - Checkboxes de seleção rápida: "Todos os formatos sem perda", "Todos os formatos com perda", "Selecionar todos".
   - Botão "Abrir configurações do sistema" para acionar o painel nativo de aplicativos padrão.
2. **Menu de Contexto:**
   - Chave mestre: "Habilitar menu de contexto Vynamp no gerenciador de arquivos".
   - Opções secundárias:
     - "Exibir ação 'Converter com Vynamp Converter'".
     - "Exibir ação 'Restaurar com Vynamp Restore'".
     - "Exibir ação 'Abrir em nova lista'".
3. **Pacote de Ícones:**
   - Seletor com três opções visuais interativas (Documento, Disco, Sinal).
   - Mostruário de preview renderizando os ícones da família selecionada em tempo real com zoom ajustável.
   - Botão "Aplicar aos arquivos do sistema" (no Windows e Linux; no macOS e Android é exibida a anotação informativa "No macOS/Android, os pacotes de ícones são aplicados à interface interna do player").
