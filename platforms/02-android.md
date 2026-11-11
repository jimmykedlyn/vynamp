# Android: shell nativo e núcleo Rust

Versão: 0.8.0 · Data: 2026-11-11 · Estado: especificação proposta, implementação não iniciada.
Baseado em: [ADR-006](../adr/ADR-006-multiplataforma.md), [ADR-012](../adr/ADR-012-integracao-shell.md), [design/05-mobile-tablet.md](../design/05-mobile-tablet.md), [design/06-painel-dsp.md](../design/06-painel-dsp.md).

## Escopo do app: só o player

O build Android entrega **apenas o player**. Os apps separados `apps/restore` e `apps/converter` **não existem** neste alvo: `vynamp-restore` e `vynamp-converter` são ferramentas de desktop e não têm binário, tela, intent-filter nem serviço no APK/AAB.

A decisão é do mantenedor, de 2026-11-03, e está registrada em [ADR-012 §1](../adr/ADR-012-integracao-shell.md#1-arquitetura-de-integração-por-plataforma), onde a linha Android declara só dois verbos — **Reproduzir** e **Enfileirar no Vynamp** —, contra os sete verbos de desktop que incluem Converter e Restaurar. A mesma decisão aparece em [Restore §11](../reference/tools/01-vynamp-restore.md#11-matriz-de-plataformas) e na §1 deste documento da [matriz de suporte](01-matriz-suporte.md#paridade-funcional).

O que isso **não** significa:

- **Não é falta de porta.** A §"Nenhum alvo ganha silêncio" da [matriz de suporte](01-matriz-suporte.md#paridade-funcional) proíbe esconder capacidade por ausência de trabalho; aqui a ausência é escopo decidido, com dono e data, e a ADR-012 é a fonte.
- **Não remove capacidade de áudio do player.** O decodificador, o motor, o DSP, o resampler e o dither são os mesmos; só os dois executáveis de ferramenta ficam fora.
- **Não fecha a porta para sempre.** Trazer Restore ou Converter ao Android exige decisão nova registrada em [05-decisões](../product/05-decisoes.md) e atualização da ADR-012, não uma TASK avulsa.

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

## Painel DSP no toque

O painel de [design/06-painel-dsp.md](../design/06-painel-dsp.md) é o mesmo produto no telefone, mas a **geometria é adaptada ao toque**, não encolhida do desktop. A regra vem da [matriz de suporte §paridade](01-matriz-suporte.md#paridade-funcional) — "compartilhar algoritmos e contratos, não copiar a geometria desktop para a tela do telefone" — e o layout phone base é o de [design/05-mobile-tablet.md](../design/05-mobile-tablet.md#phone).

| Área do painel DSP | Desktop | Android |
|---|---|---|
| Alvos de toque | ≥ 24 DIP | **≥ 48 dp** em toda a cadeia, inclusive no editor de 64 bandas (DSP Fitts, [painel §1](../design/06-painel-dsp.md#1-princípios)) |
| Equalizador clássico | 10 sliders verticais compactos | 10 bandas em página com **rolagem horizontal explícita**, labels fixos, presets e preamp visíveis ([05-mobile §Phone](../design/05-mobile-tablet.md#phone)) |
| Gráfico 20/31/64 e paramétrico | editor 960×600 em janela | **editor de tela cheia**, com tabela de bandas/filtros e alternativa numérica ao arraste |
| Cabeçalho e presets | barra fixa no topo do painel | app bar + presets em *bottom sheet*; bypass geral com o mesmo crossfade |
| Cadeia e rodapé | inspector linear em coluna lateral | inspector em sheet; latência/CPU/underrun em linha resumida, expansível |
| Medidores | VUs lado a lado na área própria | VUs empilhados ou em carrossel; o espaço do instrumento não é reduzido por controles ([VU-INT-05](../design/06-painel-dsp.md#61-integração-visual-dos-vu-meters-vu-int)) |
| Taxa de UI | 30 fps padrão, 60 opcional | 30 fps padrão e **zero render** com o app em background ou tela apagada ([painel §6](../design/06-painel-dsp.md#6-medidores-e-visualização)) |

O que **não** muda com o toque: os limites de ajuste de [painel §8.5](../design/06-painel-dsp.md#85-limites-do-ajuste) (Classic 10, gráficos 10/20/31/64, paramétrico até 32 filtros), os estados de módulo, o A/B, o desfazer e a regra de que nenhuma UI escreve direto no grafo RT — a alteração chega ao áudio com smoothing, em até 50 ms.

**Sem Restore e Converter neste painel.** O painel DSP do Android controla só a cadeia do player; não há botão, menu ou fluxo que abra restauração ou transcodificação (ver a §"Escopo do app" acima).

## Recursos, armazenamento e ciclo de vida

Config/banco/cache no espaço privado; credential material protegido com Android Keystore e cifragem apropriada, sem plaintext de refresh tokens. UI rotation/process recreation reconecta à sessão; process death salva checkpoint e permite restore, sem afirmar que executará eternamente sob qualquer fabricante. WorkManager para downloads/análise adiáveis, foreground apropriado quando necessário; replay histórico e ID de jobs preservados.

Reduzir visualização em background e desligar com tela apagada; wake locks só enquanto fluxo requer, com release em pausa/erro. Downloads respeitam metered/Wi-Fi/bateria/quota e quota do provider. Não pedir exemption de bateria indiscriminadamente. Sleep timer fecha playback; desligar telefone não é função oferecida. Alarm/wake exigem políticas/permissões específicas e não dependem de contornar o OS.

## Extensões e distribuição

Temas declarativos compartilhados com perfil phone/tablet e fallback builtin. Plugins control/metadata/provider dependem de runtime WASM validado em arm64, budgets, licença e regras de distribuição/loja. AOT/interpreter/JIT é decisão M0 com matriz de suporte, não garantia de Wasmtime arbitrário no telefone. Native helper externo/download de bibliotecas não é permitido no perfil Android1.0; módulos nativos first-party somente dentro do app assinado. Diferenças de capabilities são exibidas, sem erro silencioso.

APK/AAB com bibliotecas .so arm64, símbolos separados, native page-size compatibility e toolchain NDK/Gradle travados após prova. Validar requisitos atuais de target API/page size em cada release; o pacote não inventa autorização de Google Play. Android Auto browsing/controls é expansão de integração prevista após base MediaLibraryService, sem pedir compatibilidade com APK/skins Poweramp.

## Critérios de aceite de plataforma (Android)

Estes critérios não substituem os das TASKs que implementam cada área; eles fixam o que **esta spec de plataforma** exige antes de um aceite Android 1.0. Cada um é verificável sem hardware de áudio, exceto onde indicado.

| ID | Given | When | Then | Coberto por |
|---|---|---|---|---|
| `ANDROID-AC-1` | o APK/AAB de release | o conteúdo do pacote é inspecionado | **não** existe binário, activity, service nem intent-filter de `vynamp-restore` ou `vynamp-converter`; só os dois verbos de reprodução estão declarados | esta spec §"Escopo do app"; [ADR-012 §1](../adr/ADR-012-integracao-shell.md#1-arquitetura-de-integração-por-plataforma) |
| `ANDROID-AC-2` | um telefone na densidade mais alta suportada | a árvore de layout do painel DSP é medida | todo alvo de toque tem ≥ 48 dp, inclusive no editor de 64 bandas | [painel §1](../design/06-painel-dsp.md#1-princípios); [TASK-E13-05](../apps/player/tasks/items/TASK-E13-05.md) |
| `ANDROID-AC-3` | a tela do equalizador em viewport de telefone | ela é percorrida com TalkBack e com fonte do sistema em 200% | as 10 bandas são alcançáveis por rolagem horizontal explícita, cada uma anuncia nome, valor, unidade e limites, e a fonte em 200% faz reflow sem esconder ação | [05-mobile §Phone](../design/05-mobile-tablet.md#phone); [TASK-E13-05](../apps/player/tasks/items/TASK-E13-05.md) |
| `ANDROID-AC-4` | gráfico 20/31/64 ou paramétrico aberto | o editor é aberto em viewport de telefone | o editor ocupa a tela cheia e oferece alternativa numérica ao arraste | [painel §3](../design/06-painel-dsp.md#3-equalizador) |
| `ANDROID-AC-5` | a reprodução ativa com o painel DSP aberto | o app vai para background e a tela apaga | o render dos medidores cai a zero e o áudio continua sem underrun causado pelo app | [painel §6](../design/06-painel-dsp.md#6-medidores-e-visualização); [TASK-E13-07](../apps/player/tasks/items/TASK-E13-07.md) |

Nenhum destes critérios está verificado: não há Android SDK neste ambiente e nenhum aparelho real foi usado. Ausência de prova é limitação registrada, não aprovação.

## Referências oficiais

[Background playback/MediaSessionService](https://developer.android.com/media/media3/session/background-playback), [audio focus](https://developer.android.com/media/optimize/audio-focus), [mídia em shared storage](https://developer.android.com/training/data-storage/shared/media), [AAudio](https://developer.android.com/ndk/guides/audio/aaudio/aaudio), [target API da Play](https://developer.android.com/google/play/requirements/target-sdk). Referências consultadas em 2026-10-14; validar contra SDK usado no release.
