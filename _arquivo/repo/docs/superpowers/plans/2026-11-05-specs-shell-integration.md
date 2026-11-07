# Plano de Especificação — PR3: Integração com o Sistema e Ícones

Data: 2026-11-05 · Autor: Claude Opus 5.5 (Orquestrador) · Alvo: PR3 (`chore/claude-specs-shell-integration`)

## Objetivo

Formalizar a especificação normativa da integração do Vynamp com o shell dos sistemas operacionais suportados (Windows 11, Windows 10, Linux x64, macOS e Android), o sistema de pacotes de ícones de arquivo baseados no MVP rev.38, o catálogo interativo de ícones e as ações de menu de contexto das ferramentas auxiliares (Vynamp Restore e Vynamp Converter).

## Tarefas do Plano

### Tarefa 1: ADR-012 — Integração nativa com shell de sistemas operacionais e pacotes de ícones
- Criar `docs/architecture/adr/ADR-012-shell-integration.md`.
- Cobrir:
  - Windows 11: Pacote esparso assinado (sparse MSIX identity) para menu de contexto moderno via COM `IExplorerCommand`.
  - Windows 10 e Mostrar mais opções: Registro clássico HKCU ProgIDs.
  - Submenu "Vynamp" com 7 verbos: Reproduzir (default), Abrir em nova lista, Adicionar à lista, Adicionar à fila, Editar etiquetas, Converter, Restaurar com Vynamp Restore.
  - Instalador Windows NSIS user-scope (`$LOCALAPPDATA\Programs\Vynamp`), sem requerer admin. Registro de sparse package via `PackageManager`. Certificado de assinatura como decisão aberta.
  - Linux: Desktop Actions (`.desktop`) e extensões de gerenciadores de arquivos na 1.0 (Nautilus Python/C, Dolphin ServiceMenus, Thunar Custom Actions).
  - macOS: Quick Actions na 1.0 (Finder Services / LaunchServices document types / `CFBundleDocumentTypes`).
  - Android: `ACTION_VIEW` e `ACTION_SEND` / `ACTION_SEND_MULTIPLE` com tipos MIME registrados.
  - Pacotes de ícones: 3 famílias (Documento, Disco, Sinal), 17 formatos de áudio, geração `.ico` multi-res (16–256px). Ícones do SO alterados em Windows e Linux; macOS e Android apenas na UI do app.
  - Ferramentas separadas: Vynamp Restore (`vynamp-restore`) e Vynamp Converter (`vynamp-converter`) como executáveis desktop independentes.
  - Preferências: Seção dedicada "Integração com o sistema".

### Tarefa 2: Atualização de `docs/platforms/03-system-integration.md`
- Atualizar versão para 0.8.0.
- Detalhar contratos operacionais:
  - CLI unificada: `--play`, `--new-playlist`, `--add-playlist`, `--enqueue`, `--edit-tags`, `--convert`, `--restore`.
  - Contrato e manifesto do pacote esparso (`AppxManifest.xml` com `desktop4:FileExplorerContextMenus`).
  - Lote (batching) e COM `IShellItemArray` para preservação atômica da ordem e quantidade de arquivos.
  - Contratos Linux (Desktop Entry, KIO ServiceMenus, extensões de gerenciadores).
  - Contratos macOS (LaunchServices, AppKit openFiles, Services).
  - Contratos Android (SAF, Intent filters, persistência de grants).
  - Seção "Integração com o sistema" nas Preferências do Vynamp.

### Tarefa 3: Sistema de Ícones em `assets/icons/`
- Criar `assets/icons/README.md`:
  - Guia normativo do sistema de pacotes de ícones.
  - As 3 famílias visuais do MVP: Documento, Disco, Sinal.
  - Os 17 formatos de áudio: MP3, FLAC, WAV, AAC, M4A, OGG, OPUS, AIFF, ALAC, WMA, APE, WV, MP2, WEBM, DSF, DFF, AUDIO.
  - Especificação do `.ico` multirresolução (16 a 256px).
  - Paleta semântica e tokens por formato.
- Criar `assets/icons/manifest.json`:
  - Metadados completos do sistema de ícones, famílias, formatos, MIME types e ferramentas.
- Criar `assets/icons/preview.html`:
  - Catálogo interativo autocontido com renderização SVG ao vivo das 3 famílias × 17 formatos (51 ícones) + símbolo C e badges das ferramentas.
  - Controles de troca de família, escala de resolução (16px a 128px), fundo claro/escuro e inspetor de tokens semânticos.
- Criar vetores SVG:
  - `assets/icons/vynamp-symbol-c.svg`: Símbolo C canônico (V grafite com play âmbar #FFBF69).
  - `assets/icons/tools/vynamp-restore.svg`: Símbolo C com badge de restauração.
  - `assets/icons/tools/vynamp-converter.svg`: Símbolo C com badge de conversão.
  - `assets/icons/families/document/`: `mp3.svg`, `flac.svg`, `wav.svg`, `audio.svg`.
  - `assets/icons/families/disc/`: `mp3.svg`, `flac.svg`, `wav.svg`, `audio.svg`.
  - `assets/icons/families/signal/`: `mp3.svg`, `flac.svg`, `wav.svg`, `audio.svg`.

### Tarefa 4: Atualização de Tarefas E16 e Backlog
- Atualizar `tasks/items/TASK-E16-03.md`: Windows association, sparse package, IExplorerCommand, classic context menu, icon pack application.
- Atualizar `tasks/items/TASK-E16-04.md`: Linux MIME, Desktop Actions, file manager extensions (Nautilus, Dolphin, Thunar/Nemo), MIME icon packs.
- Atualizar `tasks/items/TASK-E16-05.md`: macOS document types, Quick Actions, LaunchServices, in-app icon pack rendering.
- Atualizar `tasks/items/TASK-E16-08.md`: Portable mode, integration cleanup, uninstallation de sparse package e HKCU ProgIDs.
- Atualizar `tasks/backlog.json`: Ajustar deliverables e acceptance criteria das tarefas E16.

### Tarefa 5: Atualização de Índices, Glossário e Changelog
- Atualizar `docs/INDEX.md`: Registrar `ADR-012-shell-integration.md` e `assets/icons/README.md`.
- Atualizar `docs/GLOSSARY.md`: Adicionar termos novos (Sparse Package, IExplorerCommand, ServiceMenu, Quick Actions, Famílias de Ícones de Áudio, Vynamp Restore, Vynamp Converter).
- Atualizar `CHANGELOG.md`: Adicionar registro da versão 0.8.0 correspondente ao PR3.

### Tarefa 6: Validação e Revisão
- Executar `cargo xtask check-docs` para validar links, títulos e integridade do índice.
- Executar `cargo xtask check-delivery` para validar integridade do backlog.
- Executar `cargo test -p xtask`.
- Despachar subagente de revisão com contexto limpo.
