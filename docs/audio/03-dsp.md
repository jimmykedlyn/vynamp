# DSP, equalização e presets

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## EQ gráfico

Painel clássico com 10 centros: 31/62/125/250/500/1k/2k/4k/8k/16k Hz, ganho ±12 dB, preamp −24..+12 dB, Reset/On/Auto. Editor avançado: modos 10/20/31 bandas, ganhos ±15 dB; frequências acima de Nyquist indisponíveis, sem reajuste oculto. Curva calculada a partir dos filtros reais, não desenho decorativo.

## EQ paramétrico

Até 32 bandas M3; 64 M5 após benchmark. Tipos peaking, low/high shelf, low/high pass, notch e band pass; frequência 10 Hz até min(40 kHz,0.45×Fs), ganho −24..+24 dB quando aplicável, Q0.1..18. Slopes por topologia suportada e exibida. Canal both/left/right; multichannel editor em M5. Ajustes interpolados 10–50 ms para não produzir zipper noise; coeficientes calculados fora da callback, estabilidade verificada.

## Cadeia

Ordem padrão: decode → trim/gain intrínseco declarado → ReplayGain/preamp → EQ/tone/efeitos por stream → mixer/crossfade → ganho master digital → resampler final → proteção true-peak final → quantização/dither/conversão → backend. O [contrato do sinal](07-dsp-signal-contract.md) define a posição fixa da proteção final e a medição no PCM entregue ao backend. Não permitir boost, plugin, mix ou resampling depois dela. Limiter criativo reordenável é outro nó. Nós desligados são bypass real; plugins declaram latência, canais e custo. Compensar caminhos paralelos e impedir ciclos.

Limiter true-peak opcional, teto −1 dBTP padrão, lookahead anunciado; aumento de latência visível. Soft clipping não substitui true-peak. Compressor threshold/ratio/attack/release/knee/makeup e meter de redução. Normalização por ReplayGain track/album, peak guard configurável, preamp para arquivos sem tag; nenhum scan ao tocar. Analyzer de loudness batch com padrão e versão registrados; LUFS não se mistura com RG dB sem conversão especificada.

M3: balance/pan, mono, channel routing básico, fades, ajuste tempo0.5–2x com preservação de pitch, pitch±12 semitons, velocidade independente; reverb/echo/chorus/flanger e silence trim opcional. Efeitos avançados podem ser módulos first-party com mesmos contratos; bypass seguro em falha. Voice remover é efeito exploratório M5 e não separação profissional garantida.

## Presets

Formato JSON próprio versionado, UUID, nome, autor, sample-rate-independent parameters, ordem de nós, checksum e provenance. Precedência: escolha manual da sessão > faixa > gênero > dispositivo > global. Transferir preset não acessa rede automaticamente. AutoEQ/headphone correction é import explícito, origem/licença identificadas, nunca presumir modelo do fone. Ao trocar DAC, lembrar perfil; não elevar volume em silêncio automaticamente.

## Aceite

Medir resposta dos filtros com impulse/sweep, tolerância ±0.2dB para curva-alvo nos pontos definidos, estabilidade perto de Nyquist, ausência NaN e amplificação involuntária. EQ flat bypass sample-exact no caminho transparente quando desligado. Meter true-peak contra fixture, tolerância0.1dB. Testar alteração rápida, taxa nova, canais e restore. O teste de ouvido complementa testes numéricos; não substitui.
