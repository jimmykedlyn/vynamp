# Catálogo de módulos DSP

Versão: 0.7.4 · Data: 2026-11-05 · Estado: especificação proposta. **Nenhum módulo está implementado no produto nativo.** No MVP web de referência (`vynamp-mvp-reference@0d9860b`, rev.38, fora deste repositório) parte dos módulos tem áudio real — inclusive Vinyl e Tube; é evidência de código, e o que foi verificado está na tabela abaixo (ver [Famílias v1.4](#famílias-v14-e-estado-no-protótipo-rev-15-relatado)).

Cada módulo informa: estado, prioridade/fase, objetivo e uso, parâmetros e faixas, posição na [cadeia real](03-dsp.md#cadeia-real-de-processamento), dependências, bypass, trade-offs, UX e testes. Faixas sem justificativa técnica ficam como **decisão aberta**, sem números inventados. Os testes `DSP-T-xx` estão na [matriz de testes DSP](../../quality/04-matriz-dsp.md).

**Fases** (adaptam a proposta do pedido aos marcos reais): **Fundação** = M1–M3 com TASK existente · **Fase 2 / Hi-Fi** · **Fase 3 / Avançado** · **Fase 4 / Ecossistema**. Um módulo **proposto** sem TASK não tem marco comprometido. O marco é definido quando a TASK for criada.

Regras comuns: todo nó tem bypass real com crossfade, smoothing de 10–50 ms, latência e tail declarados, parâmetros serializáveis e versionados, bypass seguro em NaN/instabilidade, e status ativo/inativo/bypass/incompatível/erro na UI ([painel DSP](../../design/06-painel-dsp.md)).

## Famílias v1.4 e estado no protótipo (rev. 15, relatado)

O painel DSP navega por **oito famílias**: Ganho/EQ, Dinâmica, Acústica, Estéreo, Tempo/formato, Criativos, Vitrola e Saída (DSP-UX-03; [painel DSP §8](../../design/06-painel-dsp.md#8-acesso-e-controles-dsp-ux)). As famílias são navegação, **não** switches de processamento nem um limite de oito efeitos, e não alteram a ordem da cadeia. Os capítulos 1 a 6 abaixo continuam sendo a organização textual da v1.3 (cinco blocos de módulos e extensibilidade); a tabela mapeia cada módulo à família de navegação. Medidores (Spectrum, pico/RMS, VU, LUFS, true peak) são uma **área própria, fora das famílias** ([painel DSP §2 e §6](../../design/06-painel-dsp.md#6-medidores-e-visualização)). O mapeamento de módulos sem família explícita na fonte é **proposto**, a confirmar com o design do painel.

Colunas do protótipo: **evidência relatada da referência externa rev. 14/15**, salvo onde a linha cita o código do pin rev.38 lido para este documento — nesse caso a afirmação é verificada, e a medição continua não executada. A referência cita 39 módulos e 198 campos verificados; são números do protótipo, não congelam o catálogo nativo nem excluem módulos exigidos aqui. Nativo: **nada implementado** em nenhuma linha; fases e prioridades de cada módulo seguem as seções 1 a 6. Um nó configurável não comprova processamento.

| Módulo (seção) | Família v1.4 | Protótipo rev. 15 (relatado) |
|---|---|---|
| Preamp / Input Gain · Ganho · Mixer (soma de rotas) | Ganho/EQ | áudio real; o mixer soma rotas e pode aumentar picos |
| Equalizadores gráficos 10/20/31/64 e paramétrico 32 (§1) | Ganho/EQ | áudio real **dentro do grafo** ([EQ §14](08-equalizers.md#14-mvp-htmlsvgjs--contrato-documental)) |
| Filtros passa-altas/baixas, shelves/bell, controles tonais (§1) | Ganho/EQ | áudio real; frequência acima de 0,45 × Fs bloqueia a ativação em vez de ser movida |
| Dynamic EQ, Loudness Compensation, AutoEQ, Bass Enhancer, Harmonic Exciter (§1) | Ganho/EQ (Bass Enhancer e Exciter: Criativos, a confirmar) | nó de configuração, indisponível para áudio |
| Convolution/FIR, Room Correction (§1); Spatial/HRTF (§3) | Acústica | configuração; importação de IR segue o fluxo anterior, sem aplicação no grafo |
| Compressor (§2) | Dinâmica | áudio real (compressor do navegador com makeup automático mais ganho Makeup; o nível de saída pode subir) |
| **Limiter amostral** (§2) | Dinâmica | áudio real **com limitações**: compressor 20:1 com knee 0. **Não é brickwall, não é True Peak, não mede dBTP e não garante proteção contra clipping.** Não substitui o True Peak Limiter exigido (§2 e [contrato do sinal](07-dsp-signal-contract.md)) |
| Multiband Compressor, Declipper, gate/expander (§2) | Dinâmica | nó de configuração, explicitamente indisponível para áudio |
| True Peak Limiter e Clipping Protection (§2) | Saída (proteção final; proposto) | nó de configuração, explicitamente indisponível para áudio; sem dBTP. Indicadores de clipping/true peak exibidos ficam na área Medidores |
| ReplayGain/R128 e Loudness Normalization/LUFS (§2) | Dinâmica (normalização de loudness; proposto) | nó de configuração, explicitamente indisponível para áudio; sem LUFS. A **medição** de loudness pertence à área Medidores |
| Stereo Width, Mid/Side, Balance/Pan/Mono, Phase Alignment e Channel Delay, Crossfeed (§3) | Estéreo | áudio real; Width com graves em mono ainda sem adaptador; pan com lei de −3 dB; fonte mono expandida para estéreo |
| Channel Matrix/Upmix/Downmix, Bass Management (§3) | Estéreo | sem adaptador relatado |
| Resampler/SRC, Time Stretch e Pitch Shift (§4) | Tempo/formato | sem adaptador relatado |
| Dither/Noise Shaping, Output Gain e Format Conversion (§4) | Saída | sem adaptador relatado; no protótipo a saída é a do navegador, antes do volume do player |
| Echo e Reverb (§4, criativos) | Criativos | áudio real; Echo com feedback limitado; Reverb com resposta ao impulso sintética e determinística, **não** é sala medida nem importação FIR |
| **Saturação suave** | Criativos | áudio real (tanh com oversampling 2×, drive e dry/wet). **Não equivale ao Tube Amplifier**; não afirmar que implementa uma válvula |
| LFO → Ganho | Criativos | controle real (modulação senoidal limitada a ±0,25 do ganho linear); sem ligação a portas de áudio |
| Spectrum e pico/RMS | área Medidores (fora das famílias; [painel DSP §6](../../design/06-painel-dsp.md#6-medidores-e-visualização)) | análise real: FFT de 1024 amostras, pico amostral e RMS em dBFS, antes do volume; não são LUFS nem dBTP |
| **Vinyl Simulator e Tube Amplifier** (§5) | Vitrola | design aprovado em 2026-10-23, incorporado como **módulo DSP interno** do app ([vitrola](10-vitrola.md) §8); no pin rev.38 têm **áudio real** — `class Vinyl` e `class Tube` em `mvp/advanced-dsp.js`, montados como `AudioWorkletNode` (`vynamp-advanced37`) por `mvp/graph-audio.js`, com `real=true` em `mvp/graph-model.js`; algoritmos e validação acústica continuam pendentes |

Regras que valem para o catálogo: se um nó indisponível estiver habilitado na rota ativa do protótipo, a cadeia não é ativada e a razão é exibida; o bypass explícito permite a rota sem alegar o efeito. Os módulos anteriores, fases e prioridades não mudam por causa do protótipo.

## 1. Ganho, equalização e correção tonal

### Preamp / Input Gain
- **Estado** planejado · Fundação M1 · TASK-E04-01.
- **Objetivo** ganho de entrada e reserva de headroom antes de boosts.
- **Parâmetros** −24..+12 dB. Preamp de RG para arquivos sem tag em separado.
- **Posição** depois de trim/RG, antes do EQ.
- **Bypass** 0 dB.
- **Trade-off** ganho negativo reduz o nível percebido; a sugestão automática só é aplicada com ação do usuário.
- **UX** slider vertical no EQ e leitura em dB.
- **Testes** EQ-T-12, DSP-T-03.

### Equalizadores gráficos 10/20/31/64 e paramétrico até 32 filtros
- **Estado** planejado (EQ-10 M1; EQ-20/31/PEQ32 M3; EQ-64 M5) · TASK-E04-01/04, E02-07, E12-10.
- **Especificação completa** [08-equalizers](08-equalizers.md). O MVP HTML/SVG/JS deve ter os cinco modos interativos agora.

### Dynamic EQ
- **Estado** proposto · Fase 3.
- **Objetivo** EQ que age só quando a banda passa de um nível (de-essing, controle de graves em picos).
- **Parâmetros** por banda: frequência, Q, ganho máximo, threshold (dBFS), ratio, attack/release (ms). Faixas: decisão aberta.
- **Posição** segmento por stream, depois do EQ estático.
- **Dependências** grafo E04-03, detector de nível.
- **Trade-off** modulação audível se attack/release forem curtos demais; CPU por banda.
- **UX** banda com indicador de atuação.
- **Testes** DSP-T-10.

### Loudness Compensation
- **Estado** previsto no [contrato](07-dsp-signal-contract.md) como opcional ("bass/treble/loudness compensation"); detalhe proposto · Fase 2.
- **Objetivo** compensação perceptual (curvas de igual audibilidade) em volume baixo. **Não é normalização** (RG/LUFS).
- **Parâmetros** nível de referência de audição e intensidade; faixas abertas.
- **Posição** depende do volume. Como o volume vem depois, o nó lê o ganho master como parâmetro, sem mudar de posição.
- **Trade-off** o Vynamp não conhece a pressão sonora real (o volume de hardware não é observável), então a curva é aproximada.
- **UX** aviso "aproximação".
- **Testes** DSP-T-10.

### AutoEQ / Headphone Correction
- **Estado** planejado · Fundação M3 · TASK-E04-09 (import manual com origem/licença).
- **Objetivo** correção por modelo de fone.
- **Parâmetros** conjunto de filtros paramétricos importado (até 32) + preamp do perfil.
- **Posição** nó próprio depois do EQ do usuário (não ocupa o slot do EQ).
- **Trade-off** perfis de terceiros variam por unidade e almofada; boosts exigem pré-ganho. Não há claim universal de correção.
- **UX** import explícito, provenance visível, nunca presumir o modelo.
- **Testes** DSP-T-05.

### Bass Enhancer
- **Estado** proposto · Fase 3.
- **Objetivo** reforço de graves por harmônicos (psicoacústico) ou shelf dinâmico.
- **Parâmetros** frequência de corte, intensidade, ganho de saída; abertos.
- **Posição** segmento por stream.
- **Trade-off** não é neutro; risco de clipping.
- **UX** selo "colorativo".
- **Testes** DSP-T-10.

### Harmonic Exciter
- **Estado** proposto · Fase 3.
- **Objetivo** adicionar harmônicos em agudos.
- **Parâmetros** frequência, drive, mix; abertos.
- **Posição** por stream.
- **Trade-off** aliasing (requer oversampling); fadiga auditiva.
- **UX** selo "colorativo", ganho compensado.
- **Testes** DSP-T-10.
- Não confundir com a válvula do [vitrola](10-vitrola.md).

### Convolution / FIR
- **Estado** proposto · Fase 2.
- **Objetivo** carregar IR/FIR (correção de sala, fone, fase linear).
- **Parâmetros** arquivo IR (WAV/FLAC próprio decodificado pela `vynamp-decode`), canais, Fs da IR, normalização, mix. Comprimento máximo: decisão aberta, com limite de tamanho por segurança.
- **Posição** nó de correção depois do EQ.
- **Dependências** convolução particionada (uniforme/não uniforme) e resampling da IR quando a Fs diverge.
- **Trade-off** latência = tamanho da partição; CPU/memória crescem com o comprimento.
- **UX** picker, metadados, validação e estado inválido; nunca fingir que a IR foi aplicada.
- **Testes** DSP-T-06.

### Room Correction
- **Estado** proposto · Fase 3 · **dependente de investigação**.
- **Objetivo** aplicar filtros medidos externamente (FIR/PEQ importados).
- **Posição/dependências** usa Convolution/PEQ. Não presume calibração automática nem microfone suportado.
- **UX** fluxo de import, sem medição embutida.
- **Testes** DSP-T-06.

## 2. Dinâmica e loudness

### ReplayGain (track/album) e R128
- **Estado** planejado · Fundação M1 · TASK-E04-01; análise em batch M3 · TASK-E08-03.
- **Parâmetros** modo Off/Track/Album, preamp com tag e sem tag (−24..+12 dB), peak guard.
- **Fallback** sem tag usa o preamp configurado; nenhum scan ao tocar. Opus R128/USAC não aplicam ganho duas vezes (QA-027).
- **Diferença** normalização por metadados (ganho estático) ≠ processamento dinâmico. ReplayGain 2.0 (base BS.1770): viável no analyzer E08-03; versão e padrão registrados.
- **Testes** DSP-T-07.

### Loudness Normalization (LUFS)
- **Estado** offline planejado (E08-03); em tempo real proposto · Fase 2 · dependente de investigação.
- **Objetivo** alvo em LUFS. Offline = escreve/usa metadados; em tempo real = medidor + ganho lento, que altera a dinâmica percebida e fica rotulado assim.
- **Testes** DSP-T-07.

### Compressor
- **Estado** planejado · M3 · TASK-E04-07.
- **Parâmetros** threshold, ratio, knee, attack, release, makeup, meter de redução de ganho. Faixas a fixar em E04-07.
- **Posição** por stream, reordenável.
- **Trade-off** pumping e perda de dinâmica.
- **Testes** DSP-T-08.

### Multiband Compressor
- **Estado** proposto · Fase 3.
- **Parâmetros** número de bandas, crossovers (Linkwitz-Riley proposto) e um compressor por banda.
- **Trade-off** fase e latência dos crossovers; artefatos nas transições.
- **Testes** DSP-T-08.

### True Peak Limiter (criativo) e proteção final true-peak
- **Estado** planejado · M3 · E04-07 (limiter reordenável) e **E17-07/E17-08** (proteção final fixa).
- **Parâmetros** ceiling em dBTP (padrão −1 dBTP), lookahead/latência exibidos, oversampling pinado no algoritmo/fixture.
- **Regra** proteção, não licença para elevar o ganho. Medido no PCM quantizado entregue (≤ ceiling +0,1 dB).
- **Testes** DSP-T-09, QA-027.

### Clipping Protection / indicadores
- **Estado** planejado (detecção por bloco, [engine](01-engine.md)).
- **Escopo** distinguir clip no processamento (float acima de 0 dBFS entre nós; sem hard-clip entre nós), na saída PCM (pós-quantização) e no hardware (não observável; não é afirmado).
- **UX** indicador por estágio.
- **Testes** DSP-T-03.

### Declipper (experimental)
- **Estado** proposto · Fase 3 · dependente de investigação.
- **Objetivo** tentativa de restaurar picos clipados.
- **UX** rótulo "experimental, resultado incerto"; desligado por padrão.
- **Testes** DSP-T-10.

## 3. Imagem estéreo, canais e espacialização

### Crossfeed
- **Estado** proposto · Fase 2.
- **Objetivo** mistura filtrada entre canais para fones.
- **Parâmetros** intensidade e frequência de corte (presets ao estilo Bauer/Meier como referência, sem claim de equivalência).
- **Posição** por stream, depois da correção.
- **Requisito** estéreo; mono/multicanal: indisponível com motivo.
- **Testes** DSP-T-11.

### Stereo Width
- **Estado** proposto · Fase 3.
- **Parâmetros** largura 0–200% (abaixo de 100% estreita, acima alarga).
- **Trade-off** mono-compatibilidade: medidor de correlação recomendado.
- **Testes** DSP-T-11.

### Mid/Side
- **Estado** proposto · Fase 3.
- **Objetivo** ganho e EQ separados para M e S.
- **Dependências** matriz M/S.
- **Testes** DSP-T-11.

### Balance / Pan / Mono
- **Estado** planejado · M3 · TASK-E04-07 (balance, mono, routing).
- **Parâmetros** balanço L/R; pan law (0, −3, −4,5, −6 dB) é decisão aberta.
- **Testes** DSP-T-11.

### Phase Alignment e Channel Delay
- **Estado** proposto · Fase 3.
- **Parâmetros** atraso por canal (amostras/ms) e inversão de polaridade.
- **Trade-off** cancelamento; compensação de latência entre canais.
- **Testes** DSP-T-11.

### Channel Matrix / Upmix / Downmix
- **Estado** planejado · M3 · TASK-E01-09 (mapeamento e downmix com headroom) e E04-07 (routing básico).
- **Regra** sem alegar formatos licenciados inexistentes; emulação surround rotulada (E12-10).
- **Testes** DSP-T-12.

### Bass Management
- **Estado** proposto · Fase 3.
- **Parâmetros** crossover, roteamento LFE/sub, delays.
- **Requisito** hardware multicanal real.
- **Testes** DSP-T-12.

### Spatial Audio / Binaural / HRTF
- **Estado** proposto · Fase 3 · dependente de investigação.
- **Objetivo** espacialização de estéreo/multicanal para fones.
- **Regra** sem head tracking (não há infraestrutura); suporte por plataforma declarado; renderers imersivos de codecs seguem E17-13..16.
- **Testes** DSP-T-12.

## 4. Tempo, conversão e finalização

### Resampler / SRC
- **Estado** planejado · M0 spike · TASK-E04-02.
- **Escopo** modos balanced/high, latência e CPU medidas, razão não inteira, estado entre blocos.
- **Posição** final única (ver conflito D-DSP-05 com o vitrola).
- **Nota** o resampler será uma nova biblioteca própria (nome a definir), decidido em D-DSP-08 e [ADR-011](../../adr/ADR-011-pilha-propria.md).
- **Testes** DSP-T-13.

### Time Stretch e Pitch Shift
- **Estado** planejado · M3 · TASK-E04-08 (tempo 0,5–2× com preservação de pitch; pitch ±12 semitons; velocidade independente).
- **Trade-off** transientes/artefatos e CPU. A música não herda a velocidade de podcast.
- **Testes** DSP-T-14.

### Dither e Noise Shaping
- **Estado** TPDF planejado (último estágio, só ao reduzir para inteiro; nunca em DSD/float/transparente). Noise shaping M5 opcional.
- **Regra** a `vynamp-output` não faz dither.
- **Testes** DSP-T-13.

### Output Gain e Format Conversion
- **Estado** planejado.
- **Escopo** ganho master antes do resampler. A conversão para o formato do dispositivo é feita pelo Vynamp ou pela biblioteca só com `conversion: Allow` (clamp determinístico, sem dither, reportado em `library_conversion`).
- **Testes** DSP-T-13.

### Efeitos criativos (reverb/echo/chorus/flanger/silence trim; voice remover M5)
- **Estado** planejado · M3 · TASK-E04-08; voice remover exploratório M5.
- **Regra** tails seguem a política de stop/seek/faixa.
- **Testes** DSP-T-14.

## 5. Emulações analógicas já planejadas

### Vinyl Simulator e Tube Amplifier Simulator (vitrola)
- **Estado** design aprovado em 2026-10-23; integração depois de E04-03 (M3 nas specs de skin). No pin rev.38 os dois **têm processamento real** num AudioWorklet (verificado no código de `mvp/advanced-dsp.js` e `mvp/graph-audio.js`); a rev. 15, que só tinha configuração, é histórico. Validação acústica: nada. Nativo: nada.
- **Especificação** [10-vitrola](10-vitrola.md). São dois módulos com identidade própria, opcionais e colorativos, com macro de intensidade e painel avançado.
- **Testes** VIT-T-01..10.

## 6. Extensibilidade

### Sistema de módulos internos
- **Estado** planejado · TASK-E04-03.
- **Escopo** registro de nós com ID/versão/parâmetros/metadados, ordem validada, serialização versionada e migração ([contrato de nó](03-dsp.md#contrato-conceitual-dos-nós)).

### Plugin Host
- **Estado** VST3 em worker isolado · M5 · TASK-E12-07, com um spike de DSP em WASM separado (sem WASM na callback sem prova de deadline).
- **LV2/CLAP** dependentes de investigação: licenças, SO e manutenção. Não há integração nativa simples presumida.
- **Requisitos** crash não derruba o player; plugin declara latência, canais e custo; circuit breaker (E07-04).

### Presets e compartilhamento
- **Estado** planejado.
- **Escopo** fábrica × usuário, import/export versionado, validação, limites de tamanho e migração (schema v2 pendente, [EQ §11](08-equalizers.md#11-modelo-de-dados-e-persistência)).
