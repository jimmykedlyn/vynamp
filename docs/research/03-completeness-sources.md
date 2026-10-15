# Fontes da auditoria de completude

Revisão 0.6.0 · 2026-10-14. Fontes primárias orientam contratos; não comprovam implementação ou direito de distribuir codecs. Registrar versão/hash do SDK e fixtures na decisão M0 e revisar catálogo a cada release; auditoria periódica ao menos trimestral. Nenhuma biblioteca candidata está escolhida só por este documento.

| Domínio | Fontes primárias | Aplicação |
|---|---|---|
| Winamp de referência | https://github.com/manfromafar/winamp | Referência funcional histórica; sem reutilizar código/assets ou ABI. A avaliação anterior é referência, não novo clone/teste deste repositório nesta revisão |
| AIMP desktop/Android | https://aimp.ru/?do=features&lang=en e https://aimp.ru/?do=features&os=android | Biblioteca, conversor, DSP, formatos e UX; Vynamp Linux é nativo, não distribuição via Wine |
| Poweramp | https://play.google.com/store/apps/details?id=com.maxmpz.audioplayer e https://forum.powerampapp.com/files/category/5-poweramp-equalizer/ | DSP, gapless, rotas e UX Android; EQ global é outro produto, sem prometer DVC proprietário |
| Windows handlers/defaults | https://learn.microsoft.com/en-us/windows/win32/shell/default-programs e https://learn.microsoft.com/en-us/windows/win32/shell/app-registration | Registro user-scope, Capabilities, defaults por escolha |
| Windows escolha e lote | https://learn.microsoft.com/en-us/windows/apps/develop/windows-integration/default-apps-platform e https://learn.microsoft.com/en-us/windows/win32/shell/how-to-employ-the-verb-selection-model | Não modificar UserChoice; COM/selection model exige spike real |
| Linux Desktop/MIME | https://specifications.freedesktop.org/desktop-entry/latest-single/ e https://specifications.freedesktop.org/mime-apps/latest-single/ | Exec, %U e MIME; Desktop Actions não garantem menu de file manager |
| Linux portals/MPRIS | https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.FileChooser.html e https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.OpenURI.html e https://specifications.freedesktop.org/mpris/latest/Player_Interface.html | Seleção com grant versus delegar abertura; capacidade OpenUri |
| Apple documentos/tipos | https://developer.apple.com/documentation/bundleresources/information-property-list/cfbundledocumenttypes e https://developer.apple.com/documentation/uniformtypeidentifiers/defining-file-and-data-types-for-your-app | Document types, imported/exported UTTypes e LaunchServices |
| Apple grants | https://developer.apple.com/documentation/security/accessing-files-from-the-macos-app-sandbox | Sandboxing, bookmarks e revogação; notarização é outra propriedade |
| Android intents/storage | https://developer.android.com/guide/components/intents-filters e https://developer.android.com/develop/ui/compose/sharing/receive e https://developer.android.com/training/data-storage/shared/documents-files | VIEW/share, content URI, grant temporário/persistível |
| Android áudio/formats | https://developer.android.com/media/optimize/audio-focus e https://developer.android.com/media/platform/supported-formats | Focus/foreground service e capacidades por plataforma, sem presumir paridade de media codecs |
| Rust backend candidato | https://github.com/pdeljanov/Symphonia | Features não demonstram todos os perfis, gapless ou codecs da spec; exigir spike e fallback auditado |
| FFmpeg candidato/legal | https://www.ffmpeg.org/index.html e https://ffmpeg.org/legal.html | Versão/configuração/licenças/SDK efetivo; sem inferir suporte por nome em lista |
| xHE-AAC/USAC | https://www2.iis.fraunhofer.de/AAC/xhe-aac.html e https://www.iis.fraunhofer.de/en/ff/amm/impl.html | AOT42, loudness/DRC e avaliação de implementação/licença |
| MPEG-H | https://github.com/Fraunhofer-IIS/mpeghdec | Profile/level e renderer; licença de software não concede automaticamente patentes |
| AC-4 | https://professional.dolby.com/technologies/ac-4/ | Apresentações e renderização, com SDK/licença condicionais |
| IAMF | https://aomediacodec.github.io/iamf/ e https://github.com/AOMediaCodec/libiamf | Modelo imersivo e mix presentations; Opus/AAC-LC/FLAC/LPCM subjacentes |
| Bluetooth LE Audio | https://www.bluetooth.com/learn-about-bluetooth/feature-enhancements/le-audio/le-audio-specifications/ | LC3/rotas são transportes, não associações de arquivos |
| WebM/Matroska | https://www.webmproject.org/docs/container/ e https://www.matroska.org/technical/codec_specs.html | WebM Opus/Vorbis; FLAC pertence a binding Matroska distinto |
| Ogg FLAC/Opus | https://xiph.org/flac/ogg_mapping.html e https://www.rfc-editor.org/rfc/rfc7845.html | Mapping, pre-skip/end trim/output gain e R128 |
| CAF | https://developer.apple.com/library/archive/documentation/MusicAudio/Reference/CAFSpec/CAF_intro/CAF_intro.html | PCM/ALAC e metadados por binding |
| WavPack | https://github.com/dbry/WavPack/blob/master/doc/wavpack_doc.html e https://www.wavpack.com/WavPack5PortingGuide.pdf | Híbrido/WVC, inteiro/float/DSD e limites por versão |

Nenhuma fonte sustenta promessa universal de todos os formatos atuais/futuros. Novos bindings entram por RFC, licença, corpus e prova por build/alvo. DRM comercial, vídeo/DVD e compatibilidade Winamp permanecem fora do escopo acordado.
