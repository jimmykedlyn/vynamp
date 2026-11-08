# Android: shell nativo e núcleo Rust

Versão: 0.7.1 · Data: 2026-11-03 · Estado: especificação proposta, implementação não iniciada.

## Arquitetura proposta

Core Rust continua responsável por domain, queue, clock, decode, DSP, library e SourceDescriptor. Shell Android em Kotlin/Jetpack Compose para lifecycle, permissões, MediaSession e acessibilidade. Ponte JNI/C ABI ou UniFFI após spike de ownership/threading; não requer reimplementar regras de reprodução em Kotlin. A escolha de UI nativa é proposta de adapter, não abandono de Rust no núcleo.

Activity/Compose → commands bridge → service playback owner → Rust control/engine → AAudio output (vynamp-output, ADR-011). Media3 MediaSession/MediaLibraryService usa adapter Player do motor Rust. Não iniciar ExoPlayer e Rust output simultaneamente nem manter duas filas independentes. Se Media3 decode for usado como fallback de perfil, registry declara seleção e mantém mesma sessão/clock/política DSP.

Bridge transmite IDs, buffers com ownership explícito e snapshots coalescidos. Não chamar JNI, UI, alocar, anexar threads JVM, acessar arquivo/rede ou realizar callbacks Kotlin dentro da callback de áudio. Life of native handles não depende de Activity; close/dispose idempotentes, sem use-after-free ao girar tela ou parar service.

## Serviço e controles

Reprodução em segundo plano é proprietária de uma sessão persistente de serviço. Declarar foreground service mediaPlayback e permissões apropriadas às versões; notificação de mídia segue a sessão. Validar quem pode controlar/browse, exposição do serviço não autoriza mutações arbitrárias. Swiping Activity não deve matar música ativa; após force-stop não prometer continuidade nem autostart proibido. Usuário controla retomada.

Audio focus: pedir antes tocar; loss pausa, transient pausa/retoma conforme política e intenção, duck só se autorizado. A chamada interrompe música conforme foco do sistema; evento noisy/unplug pausa sem tocar speaker automaticamente. Bluetooth/media buttons/lockscreen/control notification refletem estado do Rust; hardware volume e volume digital têm labels distintos. Respeitar restrições de início em background vigentes para target SDK.

## Arquivos e biblioteca

MediaStore para indexação autorizada; SAF/document picker para arquivos/pastas/providers, com grants persistentes quando disponíveis. Usar ContentResolver e file descriptors/document streams; content:// não é path de filesystem. Requests READ_MEDIA_AUDIO/permissions legacy conforme API e caso real; negar permissão não bloqueia rádio ou documentos selecionados permitidos. Não usar MANAGE_EXTERNAL_STORAGE para conveniência.

Adicionar android-content ao SourceDescriptor e resolver somente grants locais; playlists remotas não podem abrir content:// do dispositivo. Revogação/grant expirada marca item indisponível, não apaga playlist/rating. Escrita de tags/rename requer permissão do provider; se replace atômico não existe, guardar cópia/preview e explicar limitação antes alteração. Não prometer rename POSIX em qualquer document provider.

## Recursos, armazenamento e ciclo de vida

Config/banco/cache no espaço privado; credential material protegido com Android Keystore e cifragem apropriada, sem plaintext de refresh tokens. UI rotation/process recreation reconecta à sessão; process death salva checkpoint e permite restore, sem afirmar que executará eternamente sob qualquer fabricante. WorkManager para downloads/análise adiáveis, foreground apropriado quando necessário; replay histórico e ID de jobs preservados.

Reduzir visualização em background e desligar com tela apagada; wake locks só enquanto fluxo requer, com release em pausa/erro. Downloads respeitam metered/Wi-Fi/bateria/quota e quota do provider. Não pedir exemption de bateria indiscriminadamente. Sleep timer fecha playback; desligar telefone não é função oferecida. Alarm/wake exigem políticas/permissões específicas e não dependem de contornar o OS.

## Extensões e distribuição

Temas declarativos compartilhados com perfil phone/tablet e fallback builtin. Plugins control/metadata/provider dependem de runtime WASM validado em arm64, budgets, licença e regras de distribuição/loja. AOT/interpreter/JIT é decisão M0 com matriz de suporte, não garantia de Wasmtime arbitrário no telefone. Native helper externo/download de bibliotecas não é permitido no perfil Android1.0; módulos nativos first-party somente dentro do app assinado. Diferenças de capabilities são exibidas, sem erro silencioso.

APK/AAB com bibliotecas .so arm64, símbolos separados, native page-size compatibility e toolchain NDK/Gradle travados após prova. Validar requisitos atuais de target API/page size em cada release; o pacote não inventa autorização de Google Play. Android Auto browsing/controls é expansão de integração prevista após base MediaLibraryService, sem pedir compatibilidade com APK/skins Poweramp.

## Referências oficiais

[Background playback/MediaSessionService](https://developer.android.com/media/media3/session/background-playback), [audio focus](https://developer.android.com/media/optimize/audio-focus), [mídia em shared storage](https://developer.android.com/training/data-storage/shared/media), [AAudio](https://developer.android.com/ndk/guides/audio/aaudio/aaudio), [target API da Play](https://developer.android.com/google/play/requirements/target-sdk). Referências consultadas em 2026-10-14; validar contra SDK usado no release.
