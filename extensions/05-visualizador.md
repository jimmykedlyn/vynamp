# Visualizadores — catálogo, runtime, presets e aceite

Versão 0.7.0 · Visualização já prevista; este contrato fecha detalhes operacionais.

## Cobertura

M1 básico spectrum/VU integrado; M2/M3 spectrum linear/log, waveform/oscilloscope, stereo VU/RMS/sample/true-peak, espectrograma, phase/correlation/goniometer e cenas/partículas discretas. Distinguem meters técnicos de efeitos decorativos; não apresentam FFT elétrica como acústica do ambiente. Window Hann, FFT 2048/4096 padrão e tamanho avançado validado, smoothing/hold/decay em unidades reais, canal/layout/tap escolhidos. Waveform do arquivo e visualizer realtime são dados/processos diferentes.

Tap selectable source/pre-EQ/post-EQ/final, default final processado quando disponível; units dBFS/dBTP, reference scale e Fs claros. No modo transparente, copiar PCM inteiro/frames em ring bounded e converter no worker, sem alterar a saída. Renderer/DSP helper nunca acessa ponteiro do buffer owned da callback, JNI/network/alloc/UI no RT ou backend clock como timer de GUI. Clock consumido + latency de tap corrige apresentação; frame stale de outra generation é descartado.

Observer ring ≤2 MiB e ≤250 ms, menor dos limites; não prometer duração constante em 384 kHz/8 canais. Overflow descarta frames visuais, não bloqueia áudio. Render a 30 fps por padrão, 60 fps opcional em foreground e 10 fps em economia; background/minimized suspend conforme perfil. Android screen-off/background zero rendering. Cada instância recebe orçamento agregado; fullscreen não abre segunda engine nem captura teclado indevidamente; Escape retorna com foco/posição preservados.

## Presets/extensões

[Scene schema](../schemas/visualizer-preset.schema.json) declarativo com ID/version/renderer family/tap/channel/FFT/motion/palette/parameters bounded. Built-in scenes e pacotes próprios não carregam GLSL/WGSL/JS/binários arbitrários em 1.0. Catálogo de efeitos aprovado/runtime estático; asset hash/licença, zip/path/bomb/grants como temas. Preset que menciona effect não instalado falha/fallback explícito; não tenta baixar shader ou código.

Plugins visualizer próprios M3 usam capacidades/versionamento e observer consumer fora da callback com bytes/fuel/deadline/CPU/GPU budgets; Wasmtime/Android runtime sujeito spike, não port Winamp DLL/MilkDrop/AVS. Sandbox de renderer/processo reduz risco, não imunidade a driver. Cena reconstruída/inspirada tem assets próprios. Advanced ecosystem e shaders customizados só por RFC/ADR futura, não requisito implícito na 1.0.

## UX e robustez

Renderer disponível, resolução, FFT máxima, cena, canal e custo são capabilities reais. Sem GPU, device loss, headless, RDP ou docking: fallback para meter CPU ou desativado com motivo; não interromper áudio. Textura com dimensão máxima 4096, pacote budget e decoded/GPU budget definidos antes implementação; orçamento default worker 64 MiB e assets 16 MiB, ajustado por target com prova. Não alocar em loop sem bound; shader builtin compilado/pinado; device-loss recria sem leaks.

Reduced motion/highcontrast/TalkBack/VoiceOver/NVDA/Orca: opção desligar, meters estáticos/lentos, labels/unidades/snapshot coalescido; evitar flashes>3/s. Nunca expor centenas de eventos acessíveis por segundo. Temas e identidade C não fazem legendas dependentes só de cor. Não pedir microfone/recording/display capture para visualizar áudio do próprio player.

## Gate QA-028

Mesma faixa/FFT/tap com gold numerical por Fs e layouts; ausência de GPU/device lost/resize/multi-monitor/fullscreen/altoDPI, cenas malformadas, família desconhecida, ciclos, budgets e múltiplos visualizers, seek/generation de playback, RDP, Android background/rotation/energia. Captura clocks/CPU/GPU/frame drop e soak de 8 h prova que renderer lento não causa underrun; material gráfico não é aceite de áudio ou acessibilidade.

## Integracoes de sistema e visualizacoes

Built-ins M2/M3: spectrum logarithmic, oscilloscope, stereo VU/peak, spectrogram e partículas discretas. Window FFT2048/4096, Hann, smoothing/persistence configuráveis; faixa de frequência e valores dB coerentes com sample rate. Não apresentar espectro como medição acústica do ambiente. PCM observe ring separado com drop de frames; renderer lento perde visual frames, não áudio.

Render wgpu opcional conforme toolkit; fallback CPU/bar graph e visualizer off. Reduzir quando janela background/minimized; reduced motion mantém meters estáticos ou atualização lenta. Fullscreen visualizer sai com Escape, não sequestra atalhos de playback. Scene presets JSON próprios, sem shader code arbitrary em 1.0; remix dos assets builtin permitido conforme licença escolhida.

Windows: SMTC/media keys, tray, taskbar progress, file associations por consentimento, open/enqueue context action opcional, global hotkeys e single-instance forwarding via IPC autenticado local. Linux M2–M3 MPRIS; macOS Intel/Silicon M2–M3 Now Playing/media keys; Android M1–M3 MediaSession/notificação/lockscreen/Bluetooth/focus. Discord rich presence/scrobblers opt-in e revoke, não enviar caminhos locais. O app não precisa admin para playback.

Remote control M4: desabilitado default; bind loopback, token por sessão, discovery e LAN só habilitados explicitamente com pairing, TLS quando LAN, rate limits. API local separada da provider API; só playback/queue via capability, sem filesystem geral/shell. Ações idempotentes request IDs. Mobile companion e Chromecast/UPnP/AirPlay M5 com capacidades negociadas, sem prometer hi-res/bit-perfect remoto.

M5 VST3 opcional via worker sandbox específico: scanner separado, plugin blacklist/timeout, latency compensation, device stability e licença SDK validada. VST3 não faz parte do formato `.vynplug`; ponte opcional, sem herdar compatibilidade histórica de Winamp.
