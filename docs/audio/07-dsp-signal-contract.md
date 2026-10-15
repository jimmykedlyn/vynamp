# Contrato do sinal, headroom e proteção final true-peak

Versão 0.7.0 · Corrige cadeia da spec 0.5; não é motor DSP implementado.

## Caminho processado canônico

Fonte/demux/decode → trim e gain codec intrínseco declarado → ReplayGain/preamp/headroom → EQ/tone/efeitos/routing por stream → mix/crossfade → ganho master digital → resampler na taxa de saída → proteção final true-peak → quantização/dither/conversão → backend. Medidores source/pre-EQ/post-EQ/final têm tap/units/Fs explicitados. Alterações na ordem criativa respeitam proteção final fixa. Não permitir ganho, plugin, mix ou resampling depois da proteção final; dither/conversão são únicos estágios posteriores e entram no teste do teto final.

Limiter criativo/compressor reordenável é diferente da proteção final de saída. Usuário pode desligar proteção e escolher headroom manual, com clipping meter/estado claro; não mostrar protegido com nó final bypass. Core usa float com headroom em processamento e nunca hard-clip entre nós como tratamento padrão. Limiter não corrige overflow/NaN de filtros instáveis: detectar, bypass seguro/erro e preservar áudio quando possível. Snapshot de grafo por generation, coefficients fora callback e transições de 10–50 ms; sem allocations/locks/log/I/O/JNI.

## Proteção

True-peak ceiling default −1 dBTP, lookahead/custo/latência exibidos; sample-peak limiter/soft clipping recebem outro nome. Oversampling/interpolação/bandwidth/canal-link/attack/release e Fs real são pinados no algoritmo/fixture M0, não inferidos de um slider. Teto medido no PCM quantizado entregue ao backend com estimador de referência independente: alvo −1 dBTP, máximo alvo +0.1 dB incluindo quantização/dither nos bit-depths suportados. Se não cumprir, diminuir ceiling interno/margem ou rejeitar perfil; não aumentar tolerância para esconder overs.

Ganho master normalmente 0..1, preamp/boost explícitos antes da proteção. Controle hardware do DAC/OS não é gain do grafo; o Vynamp não pode garantir pressão sonora física ou transformações de driver. Headroom assistido pode sugerir preamp para EQ/routing/crossfade e explicar aproximação; não reduzir volume sem opção ou aplicar dois sistemas de normalização. Crossfade de fontes correlacionadas é somado antes da proteção; curva equal-power/linear escolhida e duração/gapless/skip policy explícitas.

Rate change/oversampling latency integrados no clock consumido e compensação; seek/reset drena lookahead de generation anterior, pause/resume não repete buffers. Tail de reverb/echo e fade seguem política por stop/seek/track, nunca acumulam indefinidamente. Devices/Fs/channels incompatíveis rejeitam preset ou adaptam com aviso identificável; não mover bandas acima Nyquist sem intenção.

## EQ e presets

Conservar gráfico 10/20/31 e paramétrico 32 M3/64 M5. Biquad/topology/Q/peaking/shelves/cuts documentados por algoritmo; ganhos/Q/frequências limitados, import error por JSON pointer, coeficientes estáveis perto Nyquist. Presets gráficos e paramétricos são namespaces distintos, conversion preview sem alegar equivalência exata; undo/reset/AB comparação e autosave opt-in, device/genre/track precedence preservada. Curva usa filtros reais. AutoEQ/headphone correction é import com origem/licença, pré-ganho documentado, sem claim universal de correção perfeita.

Compressor/limiter: threshold/ratio/knee/attack/release/makeup/reduction meter e unidades. Bass/treble/loudness compensation opcionais dentro do grafo, não global OS EQ; pitch/tempo não duplicam parâmetros e mostram transformações. Testes no telefone incluem CPU/energia, clipping/headroom, Fs/route changes e limites de ganho. Nenhum DVC/bypass proprietário ou EQ de outros aplicativos é anunciado.

## Transparente e passthrough

Modo transparente lossless inteiro bypassa RG/preamp/EQ/efeitos/mix/master digital/resampler/dither/limiters e mantém unity/canais/Fs/validbits; visualizer observa cópia e não transforma caminho. Ao ligar processamento, pedir sair do modo e retirar selo; não combinar proteção ativa com bit-perfect. Codec com gain/DRC não desativável impede claim neutro. Passthrough codificado M5 não é PCM bit-perfect verificado nem compatível com DSP/normalização/visualizer PCM sem decode separado autorizado.

## QA-027

Impulse/sweep, EQ extremo+preamp, fontes correlacionadas em crossfade, tempos/pitch/routing, sample rates 44.1↔48/192/384, multichannel link, rapid automation, denormal/NaN, silence→peak→silence, seeks/stop/tails, quantização 16/24/32 com/sem dither. Comparar sample/true-peak no buffer entregue ao backend e captura adequada quando disponível; áudio já passa depois de todos os boosts. Teste metadata Opus/USAC/RG não aplica gain duas vezes. CPU/latência/lookahead e origem do estimador no relatório; ouvido complementa, não substitui oracle.
