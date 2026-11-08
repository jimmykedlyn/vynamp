# Backends multiplataforma e negociação

Versão: 0.8.1 · Data: 2026-11-03 · Estado: especificação proposta, implementação não iniciada.

## Obrigação M1–M3

Todos os alvos têm backend nativo desde a slice M1. Os backends vêm da vynamp-output ([ADR-011](../../adr/ADR-011-pilha-propria.md)): Windows shared WASAPI; Linux ALSA simples na Camada 1 e PipeWire/PulseAudio com escolha por disponibilidade na Camada 3; macOS CoreAudio em Intel e Silicon (compilado, não verificado sem agente Mac); Android AAudio (Camada 3) com fallback explícito. Além do core, ainda não há API de backend; não universalizar um backend se faltam clock/route/format capabilities.

Backends enumeram dispositivos/rotas por API de OS, negociam formato e mantêm clock/drain/error. Android AudioManager e bridge tratam routes/focus; AAudio sozinho não fornece todas integrações. Quando exclusive/direct não disponível, informar saída shared real. Transparência bit a bit é teste por conjunto, não consequência de pedir flag de exclusive.

| Recurso | Windows | Linux | macOS Intel/Silicon | Android |
|---|---|---|---|---|
| Shared playback | WASAPI | PipeWire/PulseAudio | CoreAudio | AAudio (vynamp-output, Camada 3) |
| Direct/exclusive | WASAPI exclusive (Camada 2); ASIO fora do núcleo | ALSA direto opcional | Controle de dispositivo/taxa/hog quando aplicável | Requisição de sharing/rota conforme hardware; não promete bypass total |
| Route changes | Device events | Server/device events | CoreAudio callbacks | AudioManager/device callbacks e noisy/focus |
| Hi-res | Negociação DAC/driver | Servidor/ALSA e hardware | CoreAudio/DAC | OS/ROM/route/DAC; PCM24/192 não presumido |
| Volume | Digital ou device explícito | Mixer/device explícito | Digital/device explícito | Media stream hardware volume distinto de preamp/digital |

## Android

API output selecionada em M0 por medições do AAudio direto da vynamp-output (Camada 3; Oboe e OpenSL ES fora da pilha, [ADR-011](../../adr/ADR-011-pilha-propria.md)). Shared é default seguro; low-latency/exclusive só se dispositivo realmente permite. Consultar taxa, channels, format e sharing obtidos ao abrir. Tratamento de disconnect recria stream fora callback; sem JNI/logs/locks no audio callback. Não escolher o maior sample rate sem considerar fonte/energia.

USB DAC via rota do sistema na 1.0 quando suportado. Driver USB exclusivo próprio requer spike e novo requisito se necessário para alvo hi-res, não pressuposto da paridade. Bluetooth codec/link details só aparecem quando OS expõe informação; não presumir LDAC/aptX nem qualidade lossless a partir de sample rate app.

Origem hi-res é comum; saída hi-res é capability. Se fonte24/192 toca saída48k, UI mostra transformação, inclusive quando OS faz parte do processamento. DSP e output policy compartilham lógica; ausência de acesso a caminho físico impede claim bit-perfect. DSD→PCM é opção M3 apenas para perfis testados; native DSD ficaM5.

## Testes

Mesmo corpus lossless/gapless/DSP nas cinco configurações. Windows DACbusy/unplug; Linux servidor ausente/restart e X11/Wayland; macOS rate change/aggregate/default e duas arquiteturas; Android focus/noisy/Bluetooth/USB/screen-off/disconnect. Metas de latência/energia são por classe de dispositivo. Nenhum backend pode recortar fim de faixa, tocar buffers antigos ou elevar volume ao mudar rota.
