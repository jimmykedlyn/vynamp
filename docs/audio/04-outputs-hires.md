# Saídas, hi-res e modo transparente

Versão: 0.8.0 · Data: 2026-11-03 · Estado: especificação proposta, implementação não iniciada.

## Backend matrix

Windows M1: WASAPI shared via vynamp-output (plano 02 de vynamp-output, plano ainda não escrito); M3: WASAPI exclusive event-driven (Camada 2); ASIO fora do núcleo (adapter privado opcional futuro). Linux M1–M3: ALSA simples (Camada 1, plano 03), ALSA hw (Camada 2), PipeWire nativo e PulseAudio fallback (Camada 3); macOS Intel/Silicon M1–M3: CoreAudio e seleção de sample rate; Android M1–M3: AAudio (Camada 3, plano 04 de vynamp-output, plano ainda não escrito) com fallback negociado, audio focus e routes nativas. Ver [backends por plataforma](05-platform-backends.md). ASIO/ALSA/CoreAudio não garantem transparência sem negociação/teste.

Perfil por dispositivo: stable ID quando disponível, backend, sample rate policy, formato, canais, buffer, fallback, EQ e volume. Não persistir index numérico de enumeração. Dispositivos ausentes mantêm perfil para futura reconexão. Enumerar não bloqueia UI; tratamento explícito de driver que trava.

Policies: follow system (shared), follow source (exclusive), fixed rate escolhido, auto highest compatible somente por opção. Negociação apresenta taxa/formato reais; se rejeitados, pedir fallback ou falhar conforme preferência. Exclusive ocupa dispositivo e pode impedir outros apps; texto explica antes abrir. Bluetooth mostra a saída conhecida, não presume codec de transporte nem bit-perfect.

## Hi-res

Badge HI-RES SOURCE quando fonte lossless PCM >16 bits e/ou >48kHz; definição visível, sem insinuar melhoria audível. OUTPUT mostra negotiated rate/valid bits/channels. Lossy a 192k não vira lossless. FLAC24/192 em saída16/48 exibe resample/quantização. Sample container32 com valid bits24 não é origem32 bits real. Declarar “transporte de saída” quando não há acesso à transformação de driver/hardware.

## Transparente / bit-perfect

Condições: decode lossless exato, mesma taxa, mesmo mapa de canais e bits válidos compatíveis; DSP, RG, fades, limiter, resampling e volume digital desativados; unity volume; backend exclusivo/direto capaz; nenhum mixer do OS conhecido. Caminho inteiro preserva PCM32 e 24-bit packing. Visualização recebe cópia e é permitida. UI usa “Modo transparente ativo” com lista de condições; “bit-perfect verificado” apenas em combinação testada com captura bit a bit, nunca inferido só de exclusive.

Ao ligar EQ, recusar ou sair do modo mediante intenção explícita e atualizar label; não deixar selo incorreto. Ao abrir arquivo lossy, apresentar “processamento desativado; origem com perdas”, sem bit-perfect source claim. Hardware volume é distinguido de digital e não usado automaticamente se comportamento desconhecido.

## Resampling/dither

Resampler de alta qualidade: nova biblioteca própria de resampling (nome a definir; [ADR-011](../architecture/adr/ADR-011-own-audio-stack.md)), provada em M0 pela TASK-E04-02. Modos balanced/high; latência e CPU medidas; tratar razão não inteira e estado entre blocos. Dither TPDF somente ao reduzir precisão para saída inteira, último estágio, nunca DSD/float nem bypass transparente. Noise shaping opcional M5. Não elevar resolução para promover marketing.

## Falhas

DAC removido: pausar, flush, conservar posição e mostrar reconnect. Default device changed só migra se preferência autoriza; iniciar nova geração. Exclusive busy: oferecer shared sem aplicar silenciosamente. Sleep/resume renegocia sem duplicar áudio. Buffer options conservadoras, advanced low latency com indicação de underruns.

QA exige DAC USB real com 16/44.1,24/96,24/192 e formatos não suportados; loopback shared não prova caminho físico exclusivo. Testar troca de taxa, multichannel, driver ocupado e unplug. Hi-res é capability do conjunto, não atributo universal do aplicativo.
