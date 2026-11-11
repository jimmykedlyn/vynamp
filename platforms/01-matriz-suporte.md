# Plataformas obrigatórias e paridade

Versão: 0.8.0 · Data: 2026-11-11 · Estado: especificação proposta, implementação não iniciada.

## Compromisso do produto

Windows, Linux, macOS Intel, macOS Apple Silicon e Android são plataformas de primeira classe desde M0. M1 exige uma slice local nativa em cada alvo; M2 exige biblioteca/rede e UX adequada; M3/1.0 só é aceita após os gates de todos os alvos. Uma alpha pode ser disponibilizada por alvo, mas não conta como lançamento multiplataforma completo.

| Plataforma obrigatória | Arquitetura / baseline proposto | Reprodução e integração | Distribuição M3 |
|---|---|---|---|
| Windows | Windows 11 x86_64 | WASAPI, SMTC, keychain OS, tray | Installer user scope e ZIP |
| Linux | x86_64; Ubuntu 22.04+ e Fedora estável como referências | PipeWire/PulseAudio, ALSA opcional, MPRIS, Secret Service | AppImage + DEB obrigatórios; Flatpak futuro opcional |
| macOS Intel | x86_64, macOS 13+ | CoreAudio, Now Playing, VoiceOver, Keychain | DMG + ZIP Universal2 assinados e notarizados |
| macOS Apple Silicon | arm64, macOS 13+ | Mesmo núcleo e APIs, execução nativa | Mesmo DMG + ZIP Universal2 com slices Intel e Silicon nativas |
| Android | Android 10/API 29+, arm64-v8a em aparelhos reais | AAudio (vynamp-output), MediaSession/foreground playback, MediaStore/SAF, Keystore | APK assinado e AAB; target SDK conforme política vigente de publicação |

Baselines são decisões técnicas propostas, revisadas em M0 conforme toolchain e testes. x86_64 Android é alvo de emulador/CI, não substituto de arm64 real. Mac Intel não depende de Rosetta; Silicon exige slice nativa. Linux/Windows ARM64 e Android 32-bit são expansão separada, não requisito implícito.

## Paridade funcional

Obrigatórios nos cinco alvos: formatos comuns e registry de perfis, play/pause/seek/queue, gapless, RG/EQ, presets, biblioteca/tags suportadas, temas próprios, streaming/radio/HLS/podcasts, downloads, privacidade, cancelamento e recovery. Compartilhar algoritmos e contratos, não copiar a geometria desktop para a tela do telefone.

Recursos específicos são capacidades explícitas: WASAPI/ASIO Windows, ALSA direto Linux, CoreAudio macOS, audio focus e notificações Android. Desktop docking/tray/global hotkeys/CLI não se aplicam ao telefone; há equivalentes por navegação/MediaSession/ações de notificação. ASIO, VST3, CD, scheduler de desligamento e plugin nativo externo não são exigidos no Android. DSD nativo e 768 kHz permanecem M5 por hardware, independentemente do OS.

Nenhum alvo ganha silêncio, formato omitido ou feature desabilitada apenas por falta de porta: registrar erro/capacidade e tarefa. M3 exige paridade de formatos comuns via decoder compartilhado ou adapter licenciado testado. Formatos raros M5 seguem matriz de capacidades por alvo.

**A exceção declarada são as duas ferramentas desktop.** `vynamp-restore` e `vynamp-converter` não existem nos builds Android e iOS: são escopo decidido pelo mantenedor em 2026-11-03 e registrado em [ADR-012 §1](../adr/ADR-012-integracao-shell.md#1-arquitetura-de-integração-por-plataforma) e [D-MOB-01](../product/05-decisoes.md#decisões-de-mobile-2026-11-11), não falta de porta. O player mobile mantém a paridade de reprodução, DSP e formatos comuns; detalhes em [Android §Escopo do app](02-android.md#escopo-do-app-só-o-player) e [iOS §1](05-ios.md).

**O painel DSP é adaptado ao toque, não reduzido.** A frase "não copiar a geometria desktop" vale literalmente para ele: alvos de 48 dp no Android e 44 pt no iOS, 10 bandas em rolagem horizontal e 20/31/64/paramétrico em tela cheia, mantendo os mesmos limites de ajuste e o mesmo orçamento de qualidade ([painel DSP §10](../design/06-painel-dsp.md#10-layout-no-toque-android-e-ios), [D-MOB-02](../product/05-decisoes.md#decisões-de-mobile-2026-11-11)).

## Gate por alvo

Build/install/launch nativos; arquivo e rádio reais; pause/seek/queue; metadados/presets; acessibilidade (Narrator/NVDA, Orca, VoiceOver ou TalkBack); suspensão/retomada e troca de dispositivo; migrations; update/distribuição; arquivos/credenciais conforme OS. macOS tem dois relatórios reais independentes. Android requer screen-off, audio focus, unplug/Bluetooth e process recreation, não apenas emulador.

O release gate agregado depende de TASK-E14-06, Linux TASK-E12-01, macOS TASK-E12-02 e Android TASK-E13-10. Provas de áudio exclusivas continuam dependentes de equipamento; “multiplataforma” não significa que todo OS exponha WASAPI ou ASIO.
