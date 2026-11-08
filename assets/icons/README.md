# Sistema de pacotes de ícones de arquivo do Vynamp

Versão 0.8.0 · Especificação de assets e catálogo normativo (conforme [ADR-012](../../adr/ADR-012-integracao-shell.md) e [ADR-008](../../adr/ADR-008-marca.md)).

## Visão geral

O Vynamp fornece um sistema modular de pacotes de ícones para arquivos de áudio associados ao player, permitindo personalização visual tanto no shell do sistema operacional (Windows Explorer e Linux desktop) quanto dentro da interface do aplicativo (macOS, Android e UI desktop interna).

O sistema é diretamente inspirado na iconografia vetorial refinada no protótipo web de referência (rev.38, `icons.js`) e fundamentado nos tokens da identidade visual Conceito C ("Pulso").

## As três famílias visuais

| Família | Slug | Inspiração visual no MVP | Metáfora e estilo | Uso recomendado |
|---|---|---|---|---|
| **Documento** | `document` | Glifo `file-audio` | Arquivo / partitura clássica com dobra no canto superior direito, etiqueta com nome do formato e glifo musical. | Usuários que priorizam legibilidade clássica em exploradores de arquivos e listas densas. |
| **Disco** | `disc` | Glifo `vinyl` | Disco de vinil estilizado com ranhuras concêntricas escuras (`#11131B`), furo central e rótulo com cor semântica do formato. | Usuários entusiastas do conceito analógico e da identidade visual central do Vynamp. |
| **Sinal** | `signal` | Glifo `waveform` | Chassis grafite escuro com barras verticais de forma de onda/espectro sonoro e tipografia técnica de alta resolução. | Perfis voltados para produção, estúdio, masterização e análise acústica. |

## Formatos cobertos (17 formatos)

O catálogo cobre todos os 17 formatos suportados pelo motor de áudio e catálogo de extensões do Vynamp:

| Formato | Categoria | Extensões associadas | Tipo MIME primário | Cor semântica | Token de destaque |
|---|---|---|---|---|---|
| **FLAC** | Lossless | `.flac` | `audio/flac` | Âmbar Vynamp | `#FFBF69` |
| **WAV** | Lossless PCM | `.wav` | `audio/wav` | Laranja Quente | `#FF9F43` |
| **AIFF** | Lossless PCM | `.aif`, `.aiff` | `audio/aiff` | Coral Técnico | `#EE5253` |
| **ALAC** | Lossless Apple | `.m4a`, `.alac` | `audio/mp4` | Roxo Claro | `#9B59B6` |
| **APE** | Lossless Monkey | `.ape` | `audio/x-ape` | Marrom Dourado | `#D35400` |
| **WV** | Lossless WavPack | `.wv` | `audio/x-wavpack` | Âmbar Suave | `#F39C12` |
| **MP3** | Lossy MPEG | `.mp3` | `audio/mpeg` | Azul Elétrico | `#0ABDE3` |
| **AAC** | Lossy MPEG-4 | `.aac` | `audio/aac` | Ciano Suave | `#48DBFB` |
| **M4A** | Lossy / Container | `.m4a` | `audio/mp4` | Roxo Vivo | `#5F27CD` |
| **OGG** | Lossy Vorbis | `.ogg` | `audio/ogg` | Verde Menta | `#1DD1A1` |
| **OPUS** | Lossy Modern | `.opus` | `audio/opus` | Verde Esmeralda | `#10AC84` |
| **WMA** | Lossy Windows | `.wma` | `audio/x-ms-wma` | Azul Real | `#2E86DE` |
| **MP2** | Lossy Legado | `.mp2` | `audio/x-mp2` | Azul Petróleo | `#54A0FF` |
| **WEBM** | Lossy Web | `.webm` | `audio/webm` | Magenta Claro | `#FF6B6B` |
| **DSF** | DSD Sony | `.dsf` | `audio/x-dsf` | Ouro Metálico | `#F1C40F` |
| **DFF** | DSD Philips | `.dff` | `audio/x-dff` | Ouro Intenso | `#F39C12` |
| **AUDIO** | Genérico | Diversas | `audio/*` | Grafite Neutro | `#8395A7` |

## Especificação de artefatos binários (.ico e hicolor)

### 1. Windows `.ico` Multi-Resolução

Para cada um dos 17 formatos em cada família, é gerado um arquivo `.ico` contendo 7 camadas de resolução:
- `16×16`: Detalhes simplificados, espessura mínima de 1px, sem gradientes finos para máxima nitidez na visualização de detalhes/lista do Explorer.
- `24×24`: Otimizado para telas com escala 150% no Windows 10/11.
- `32×32`: Visualização padrão de ícones pequenos/médios do Explorer.
- `48×48`: Visualização de blocos e lista de tarefas.
- `64×64`: Visualização média-grande.
- `128×128`: Visualização de ícones grandes.
- `256×256`: Visualização de ícones extra grandes do Explorer (comprimido em PNG interno conforme especificação moderna de `.ico`).

### 2. Linux Hicolor Icon Theme

No Linux, os ícones são instalados na hierarquia padrão de temas do usuário ou do sistema:
- `~/.local/share/icons/hicolor/scalable/mimetypes/vynamp-<family>-<format>.svg`
- `~/.local/share/icons/hicolor/<size>/mimetypes/vynamp-<family>-<format>.png` (tamanhos 16, 24, 32, 48, 64, 128, 256).

## Identidade das ferramentas auxiliares

As ferramentas auxiliares desktop possuem ícones próprios baseados no Conceito C "Pulso":
1. **Vynamp Player:** Símbolo V grafite (`#11131B`) com triângulo play separado em âmbar (`#FFBF69`). Arquivo: `vynamp-symbol-c.svg`.
2. **Vynamp Restore (`vynamp-restore`):** Símbolo C com badge de restauração acústica (círculo grafite com glifo de forma de onda corrigida em ciano `#66D9EF`). Arquivo: `tools/vynamp-restore.svg`.
3. **Vynamp Converter (`vynamp-converter`):** Símbolo C com badge de conversão (círculo grafite com setas circulares de transcodificação em verde-lima `#B8F35A`). Arquivo: `tools/vynamp-converter.svg`.

## Catálogo interativo de preview

Para inspecionar visualmente todos os 51 ícones (3 famílias × 17 formatos) e as logos das ferramentas, abra o arquivo `assets/icons/preview.html` em qualquer navegador web moderno.
