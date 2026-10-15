# Visualizações e integrações de sistema

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

Built-ins M2/M3: spectrum logarithmic, oscilloscope, stereo VU/peak, spectrogram e partículas discretas. Window FFT2048/4096, Hann, smoothing/persistence configuráveis; faixa de frequência e valores dB coerentes com sample rate. Não apresentar espectro como medição acústica do ambiente. PCM observe ring separado com drop de frames; renderer lento perde visual frames, não áudio.

Render wgpu opcional conforme toolkit; fallback CPU/bar graph e visualizer off. Reduzir quando janela background/minimized; reduced motion mantém meters estáticos ou atualização lenta. Fullscreen visualizer sai com Escape, não sequestra atalhos de playback. Scene presets JSON próprios, sem shader code arbitrary em 1.0; remix dos assets builtin permitido conforme licença escolhida.

Windows: SMTC/media keys, tray, taskbar progress, file associations por consentimento, open/enqueue context action opcional, global hotkeys e single-instance forwarding via IPC autenticado local. Linux M2–M3 MPRIS; macOS Intel/Silicon M2–M3 Now Playing/media keys; Android M1–M3 MediaSession/notificação/lockscreen/Bluetooth/focus. Discord rich presence/scrobblers opt-in e revoke, não enviar caminhos locais. O app não precisa admin para playback.

Remote control M4: desabilitado default; bind loopback, token por sessão, discovery e LAN só habilitados explicitamente com pairing, TLS quando LAN, rate limits. API local separada da provider API; só playback/queue via capability, sem filesystem geral/shell. Ações idempotentes request IDs. Mobile companion e Chromecast/UPnP/AirPlay M5 com capacidades negociadas, sem prometer hi-res/bit-perfect remoto.

M5 VST3 opcional via worker sandbox específico: scanner separado, plugin blacklist/timeout, latency compensation, device stability e licença SDK validada. VST3 não faz parte do formato `.vynplug`; ponte opcional, sem herdar compatibilidade histórica de Winamp.
