# Engine de áudio e invariantes

Versão: 0.7.2 · Data: 2026-11-07 · Estado: especificação proposta, implementação não iniciada. Contrato do relógio apresentado, do ring SPSC, da drenagem e a limitação conhecida do viés de latência absorvidos do spike da TASK-E01-02 ([evidência](../../evidence/PR/task-e01-02/README.md)).

## Pipeline

Fonte → demux → decode → trim/gain intrínseco declarado → ReplayGain/preamp → DSP por stream → mixer/crossfade → ganho master → resampler final → proteção true-peak → conversão/dither → backend. [Contrato canônico do sinal](07-dsp-signal-contract.md); diagrama e contrato de nós em [DSP](03-dsp.md#cadeia-real-de-processamento). Visualização observa cópia limitada fora da callback. Modo transparente usa inteiros sem transformações, separado do grafo DSP float.

Domain types: SampleRate(u32), FrameCount(u64), ChannelLayout, MediaTime em ticks racionais; não usar f32 ou milissegundos truncados como relógio principal. Posição apresentada vem dos frames efetivamente consumidos/latência do dispositivo, não bytes baixados. Decode mantém formato e precisão original; DSP padrão f64 nos filtros e acumuladores, buffers f32 onde benchmark provar adequação; preservar PCM32 inteiro no modo transparente. “Pipeline 64-bit” não será marketing se partes relevantes são f32.

## Threads

Control thread recebe comandos; workers fazem I/O/demux/decode; audio callback só consome buffers prontos; UI usa snapshots. Nada de mutex bloqueante, heap allocation, filesystem, DNS, HTTP, SQLite, logs síncronos ou extensões WASM dentro do callback. SPSC ring pré-alocado; parâmetros enviados por troca atômica/snapshot. Grafo compilado fora do callback, swap em fronteira de bloco com descarte fora da thread real-time.

O ring SPSC é pré-alocado na construção — a armazenagem de amostras é uma única alocação e o caminho de produção/consumo não aloca mais nada; o produtor publica amostras com store Release do contador de escrita depois de gravá-las, e o consumidor libera slots com store Release do contador de leitura depois de lê-los — cada lado só toca slots que o outro entregou, e nenhum dos dois espera. Contadores monotônicos em `u64`.

O clock apresentado é publicado pela própria callback por seqlock de escritor único: leitores (UI, controle) repetem a leitura enquanto a escrita está em curso, e a callback nunca espera. `presented = max(anterior, conteúdo escrito antes desta callback − latência_reported_do_primeiro_frame_do_buffer)`. A latência informada pelo backend é o atraso até o **primeiro** frame do buffer que está sendo escrito, então ela se mede contra o conteúdo entregue nas callbacks **anteriores** — medi-la contra o conteúdo já acrescentado superestima a posição em um buffer do dispositivo. `presented` é monotônico, nunca ultrapassa `content_frames`, e silêncio de underrun ou drenagem não o avança.

A fórmula supõe que a fila do dispositivo contém **só** conteúdo. Silêncio de underrun ou de drenagem enfileirado também compõe a latência reportada, então `presented` fica subestimado em no máximo `latency_frames`: após um underrun atrasa até a latência inteira e volta a acompanhar quando o conteúdo flui; na drenagem estabiliza em `content_frames − latency_frames` sem alcançar `content_frames`. Nunca superestima e nunca regride. A correção depende do dreno completo da [TASK-E01-05](../../libs/engine/tasks/items/TASK-E01-05.md); medição em [evidência](../../evidence/PR/task-e01-02/README.md).

Cada reprodução tem generation ID; seek/troca incrementa geração e cancela produtor anterior. Dados de geração velha não entram no ring. EOF só termina após drenar buffers e saída. Underrun zera região restante sem repetir amostras e publica contador; fade de recuperação evita click quando houver espaço seguro. A callback lê a marca de fim de stream **antes** de medir o disponível no ring: depois que o produtor sinaliza fim, toda amostra já foi publicada, então leitura curta é drenagem e não underrun — sem essa ordem, o fim de toda reprodução viraria underrun.

## Estados

Idle → Opening → Buffering → Playing ↔ Paused; Playing/Paused → Seeking → Buffering/Playing; qualquer estado ativo → Stopping → Idle. Erros recuperáveis → Reconnecting ou DeviceLost; terminal → Error. Cancellation é idempotente. Stop em Opening cancela toda operação. Paused conserva posição; live usa timeshift quando habilitado ou retoma live edge, com label explícito. Fechamento espera shutdown limitado e não trava indefinidamente.

## Gapless e transições

Predecode da próxima faixa pelo menos 2 s quando recurso e memória permitem. Remover encoder delay/padding somente com metadados confiáveis; não usar “silence remover” para fingir gapless. Entre faixas de mesmo formato/dispositivo, nenhuma inserção/remoção de frames na sequência de referência. Troca de sample rate em exclusivo pode exigir reabertura e quebra de gapless; mostrar limitação. Crossfade 0–12 s, equal-power padrão, separado para avanço manual/automático; desligado por padrão. Gapless e crossfade são políticas mutuamente exclusivas na mesma fronteira.

## Qualidade

Clipping detectado por bloco, não corrigido silenciosamente no modo transparente. Float NaN/Inf de decoder/DSP invalida bloco e desativa nó, com diagnóstico. Sample rate/channels de arquivo não confiável devem ser validados antes alocação. Mono/stereo/5.1/7.1 com matriz explícita; downmix inclui headroom, não soma ilimitada. Não apresentar emulação surround como canais reais.

## Metas

Em máquina de referência definida em QA: abrir arquivo local p95≤350ms com cache quente, seek local p95≤150ms, comando play/pause→ação p95≤80ms. Reprodução estéreo PCM/FLAC24/192 por 8h sem underrun causado pelo app. Callback p99.9 <25% do período negociado sob teste de carga. Números são gates a medir; falha gera ajuste de design ou ADR, nunca aprovação fictícia.
