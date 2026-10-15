# Performance, métricas e diagnósticos

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Budgets de referência

Cold launch p95≤1.5s/SSD até UI utilizável; warm≤700ms. Memória idle≤120MiB RSS sem plugins runtime/GPU cache pesado, playback FLAC24/192≤250MiB com caches configurados; reportar diferença entre RAM privada/GPU/shared. CPU playback stereo24/192≤5% total na máquina referência, visualizer≤10% total agregado; medir core utilization e workload, não comparar % de SO diferentes sem normalizar. Binary size/download targets registrados M0 após native backends; não inventar tamanho final.

Library100k: search warmp95≤100ms, lista virtualizada interactionp95≤50ms, scan inicial throughput medido sem atrapalhar áudio. UI stable60fps foreground sem renderizar cada row/album; visualizer30fps default. Callback p99.9<25%deadline, zero blocking/alloc, underruns0 em soak8h condição controlada. Startup HTTP audio p95≤2s em50Mbps/20ms; HLS≤3s com segmentos4s prontos e janela adequada; rádio server pode impor latência externa, registrar separadamente.

## Métricas locais

Startup phase timing, command latency, decode throughput, ring occupancy, callback deadline misses, underrun/rebuffer, source bitrate, live delay, cache hit, DB latency, job throughput e plugin failures. Counters RT enviados por atomics e coletados fora callback. Histograma pequeno com overhead medido; tracing RT não escreve strings/logs. Frame-rate não é indicador de áudio.

Painel diagnóstico: source descriptor sanitizado, decoder/backend versions, chain, negotiation, CPU/ring e últimas falhas. Export `.vyndiag` zip com config redacted, device IDs hash opcional, errors e metrics; sem mídia/capas/paths completos por padrão. Usuário seleciona incluir dados adicionais.

## Bench protocol

Build Release, toolchain/native versions, commit, OS/driver/hardware, dataset hashes, energia/thermal state, processo aquecido e runs≥30 para p95 de operações; callback histogram≥1M invocations ou soak. Testes cold/warm separados. Baseline guardado em JSON; regressão>10% abre investigação, não rollback automático de mudança necessária. Metas só viram “atingidas” após relatório reproduzível.
