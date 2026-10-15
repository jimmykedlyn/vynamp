# Integração nativa, associação de arquivos e Abrir com

Versão 0.7.0 · Contrato de produto, não integração já implementada.

## Regras comuns

M2: candidacy em Abrir com/chooser de cada um dos cinco alvos; M3: lote, upgrade/uninstall/portable e QA completos. Registrar handler não significa tomar o default. Configuração de formatos deriva de capabilities verificadas do artefato, não da lista de desejos. Extensão identifica um candidato de container, nunca todos os codecs/perfis dentro dele. Decoder inexistente gera UnsupportedFormat/Profile, não fallback para interpretar como AAC-LC. [Registry](../audio/06-codec-capability-registry.md), [catálogo](../../config/format-catalog.json).

Preferências: selecionar extensões/tipos para registrar; ação explícita abre configuração nativa de defaults. Nunca sobrescrever escolhas de outro app, repetir prompts agressivos, requerer admin para playback ou auto-associar todos os tipos. Build parcial mostra disponibilidade real. M3U/M3U8/PLS/XSPF/CUE só após parser validado. Não assumir ownership de áudio padrão/ISO/vídeo ou registrar *.* / */*. Novos tipos próprios usam namespace com.jimmykedlyn.vynamp; extensões já fixadas .vyntheme/.vynplug/.vyndiag são instaladas/importadas por fluxo explícito separado de playback.

## Windows 11 x64

Installer user-scope gera registro HKCU: Software\Classes\Vynamp.Audio.<binding>, Applications\vynamp.exe, SupportedTypes/OpenWithProgids, Software\Vynamp\Capabilities\FileAssociations e Software\RegisteredApplications. ProgID descreve container/binding e versão estável; ícone e executable path absolutos e quotados. Registrar verbos Open e Enqueue opcionais com display/localização. Exemplo CLI: "<instalação>\vynamp.exe" --open -- "%1". Locators são dados e não fragmentos de shell; nunca cmd /c ou execução derivada do nome da faixa. Notificar Shell após mudanças próprias.

Não alterar UserChoice/hash, usar SetAppAsDefaultAll legado como bypass ou impor browser/protocol default. Abrir Settings Default Apps com registeredAppUser/registro apropriado conforme API vigente; instruções só mediante ação do usuário. Menu moderno Windows11 é capability separada de verbos em Mostrar mais opções; se precisar COM/IExplorerCommand/packaged identity, provar assinatura/install/distribution antes anunciar. Não tornar MSIX obrigatório por esse recurso.

Batch: %1 e MultiSelectModel=Player isoladamente não garantem entrega de seleção inteira. Spike COM/ExecuteCommand/DropTarget com IShellItemArray ou alternativa documentada deve preservar 1/2/20/100 itens, ordem e uma única alteração de fila. Modelo suportado por versão/build entra em manifest. Caso dispatcher receba ativações individuais sem ID de lote, só agrupar por contrato demonstrado; debounce heurístico não é aceite de atomicidade. [Ativação](04-activation-ownership.md).

Uninstall remove ProgIDs/verbs/OpenWith/Capabilities próprios somente se ainda apontarem para esta instalação e se nenhum artefato Vynamp ativo depender deles. Nunca limpar classes genéricas/.ext default de terceiros nem restaurar à força um default antigo. Config/dados permanecem fora do InstallDir. Portable ZIP não registra automaticamente: integração explícita per-user com caminho estável; mover/apagar o portable permite reparar/remover com confirmação contextual.

## Linux x64 — X11/Wayland

Desktop Entry com Name=Vynamp, Icon=vynamp e Exec=vynamp --open -- %U, sem colocar %U entre aspas ou invocar shell. TryExec e desktop ID estáveis; lista MimeType gerada do build. Usar tipos shared-mime-info existentes; XML MIME próprio apenas para formatos próprios/ausentes e bem definidos, não reivindicar audio/* indiscriminadamente. update-desktop-database/MIME/cache de ícones por ferramenta/escopo do pacote; respeitar mimeapps.list e default escolhido pelo usuário.

DEB integra menu/MIME/hicolor conforme pacote. AppImage isolada não instala persistência no desktop: oferecer integrar/remover de forma explícita, com arquivo no escopo do usuário e caminho estável; AppImage não é sandbox Flatpak. Flatpak fica expansão opcional, com document portal/permissões próprias, não gate obrigatório na 1.0.

Desktop Action Enqueue pode aparecer em launchers que a suportem; não garante menu de contexto universal em Nautilus/Dolphin. Integrar file-manager extension só após capability/teste/documentação específicos. FileChooser portal seleciona arquivos e concede acesso; OpenURI delega abertura a outro app, não são intercambiáveis. No Wayland não exigir hotkey global/display capture sem APIs/portais e consentimento próprios.

MPRIS MediaPlayer2/Player: SupportedMimeTypes/SupportedUriSchemes/CanSeek/CanControl são capacidades reais; OpenUri usa mesma fila/ownership e pipeline seguro. Metadata/seeking/position seguem áudio consumido, não timer de GUI. MPRIS não substitui MIME/Abrir com. Serviços D-Bus e Unix socket por usuário/sessão/perfil, sem expor controle LAN por padrão.

## macOS Intel e Apple Silicon

Mesmo bundle Universal2/document types/ícones em ambas arquiteturas. CFBundleDocumentTypes + LSItemContentTypes, CFBundleTypeRole=Viewer para mídia, LSHandlerRank=Alternate para candidacy. Usar UTTypes existentes e UTImportedTypeDeclarations quando necessário; UTExportedTypeDeclarations apenas para tipos realmente próprios. Não assumir Owner de MP3/FLAC ou aplicar todos arquivos genéricos. Registrar por LaunchServices e deixar usuário escolher Abrir com/Get Info; não editar defaults globais silenciosamente.

Finder/Dock/LaunchServices abrem via callbacks AppKit application(_:openFiles:) e open URLs apropriados, não apenas argv. Solicitações antes de didFinishLaunching ficam bufferizadas e idempotentes até bootstrap. Lote mantém a semântica comum; drag/drop usa os mesmos locators e grants. Now Playing/media keys usam sessão correta e relógio real; nenhum áudio reinicia por evento de janela.

Developer ID/notarização não equivale a App Sandbox. Se App Sandbox adotado, entitlements mínimos, grants/security-scoped bookmarks balanceados start/stop, stale refresh e revogação. Selecionar playlist não concede acesso automático a todos irmãos. File Provider/iCloud placeholder não dispara downloads de biblioteca sem política; abrir explicitamente permite solicitar materialização, com cancel/erro/tempo e sem I/O na callback. Rosetta não substitui relatório Intel e Silicon.

## Android API 29+ / arm64

Manifest gerado por capabilities: VIEW + DEFAULT para MIME comprovados; SEND/SEND_MULTIPLE + DEFAULT para compartilhar áudio/playlists. android:exported explícito; BROWSABLE apenas em filtro dedicado de deep links M4 com verificação. Não reivindicar todos HTTP/HTTPS, */* ou application/octet-stream. Tipos genéricos recebidos por picker explícito podem passar probe seguro, sem instalar handler indiscriminado.

VIEW recebe data; SEND recebe EXTRA_STREAM/ClipData e SEND_MULTIPLE recebe lote ordenado. Validar todas as URIs/grants/quantidades antes tocar; tratar cold onCreate e warm onNewIntent sem repetir após rotation/process recreation. ContentResolver/FD/document stream, nunca tratar content:// como path. Provider pode não oferecer seek, tamanho ou persistência: capabilities e erros honestos. Grant temporário de share não é persistível por magia; takePersistableUriPermission só quando flags/provider o permitem. Para retomar depois, usuário escolhe SAF ou cópia de importação; revogação preserva playlist/ratings e marca indisponível.

VIEW é ação explícita de abrir e pode tocar após autorização/focus; SEND mostra escolha/preview Open ou Enqueue, sem autoplay surpresa. Activity encaminha ao serviço dono da sessão Rust, sem segunda engine/fila. Audio focus antes de saída; cumprir restrições atuais de foreground mediaPlayback, including top-app/FGS quando exigido por target/API. Permissão negada para biblioteca não bloqueia rádio ou documento concedido. Arquivo compartilhado não autoriza instalar plugin nem escrever tags.

## Gate

E16 implementa a matriz; QA-022/023/024/031 com Explorer/Finder/Files/Dolphin/Nautilus e launchers Android reais. Instalar/atualizar/remover, handlers/defaults de terceiros, nomes especiais, grants expirados, lote misto e launch simultâneo. Só indicar funcional após evidência por OS/ABI/build; template/manifest não demonstra integração.
