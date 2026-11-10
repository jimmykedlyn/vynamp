# Roadmap e gates de entrega

Versão: 0.7.2 · Data: 2026-10-28 · Estado: especificação proposta, implementação não iniciada.

| Fase | Entrega | Gate de saída |
|---|---|---|
| M0 | Provas de áudio, UI, plugins e contratos | ADRs aprovados com evidência, não só hello-world |
| M1 | Alpha local nos cinco alvos: playback/queue/EQ10/UI por host/output nativo | Abrir/tocar/seek/pause/stop, formato básico, recuperação e callback disciplina |
| M2 | Beta multiplataforma: biblioteca/tags read, formatos modernos, rádios/podcasts/temas |100k, HTTP/ICY, packages próprios e a11y básica |
| M3 | Versão1.0 Windows/Linux/macOS Intel+Silicon/Android: DSP/hi-res/HLS/tools/plugins por capability | Hardware/soak/security/distribuição/a11y em cada alvo, gate TASK-E14-06 e matriz M1–M3 completa |
| M4 | Provider platform: catálogo/VOD/live/auth/sync/registry | rights/authz/cost/load/restore e mock contract parity |
| M5 | ARM adicional Windows/Linux, DSD nativo/768k, casting/VST/CD/legados | Gates de expansão por capability; não posterga Linux/macOS/Android obrigatórios. Specs Android fechadas em M0/M1 e consolidadas. |

## Ordem crítica

M0 repo/fixtures → decode/output clock → state/queue → UI classic → biblioteca → network/radio → temas/plugins → advanced output/DSP → HLS/tools → release readiness. Contrato provider mock pode avançar depois network antes M4. Plataforma real depende de cliente capaz e ingest rights definidos.

## Organização da equipe

Trilhas independentes: engine/output, product/UI, library/tools, streaming, extension/security e platform futura. Contratos congelados em reunião de design; integração por thin vertical slices. Dependência de tarefa indica bloqueio real, não necessidade de executar todas em fila. Tarefas em [backlog](../tasks/README.md) com IDs/aceite/estimativa; sem atribuição a pessoas inexistentes.

## Estimativa

Cada tarefa tem uma faixa provisória de esforço por fase (2–5 dias em spikes, maior em entregas complexas); spikes e tarefas complexas explicitam investigação. Não somar faixas como calendário: equipe, dependências, plataformas e validação hardware mudam duração. Reestimar apósM0 e após primeira vertical slice. Não afirmar que toda matriz será entregue em poucas semanas.

## Definição de completo

Código revisado, criteria da tarefa demonstrados, teste apropriado, docs/contracts atualizados, erro/cancelamento/acessibilidade considerados, licença/assets registrados e artifact reproduzível. M1 não é M3; disabled placeholder e label “em breve” não contam como funcionalidade implementada.

## Roadmap DSP e equalizadores

Adiciona ao roadmap acima as fases do pedido DSP v1.3, mapeadas aos marcos reais. Nada está implementado. O MVP HTML/SVG/JS é um protótipo de UI: a coluna dele **não** antecipa marcos nativos. Detalhes: [DSP](../reference/audio/03-dsp.md), [equalizadores](../reference/audio/08-equalizers.md), [catálogo](../reference/audio/09-dsp-catalog.md), [vitrola](../reference/audio/10-vitrola.md) e [decisões](../product/05-decisoes.md#decisões-dsp-e-equalizadores-2026-10-25).

| Fase DSP | Item | Marco nativo · TASK | MVP HTML/SVG/JS | Estado |
|---|---|---|---|---|
| Fundação | Arquitetura DSP modular (ordem fixa por stream), smoothing, bypass, latência | M2 · E04-03 | UI de cadeia e estados | planejado |
| Fundação | Preamp, **EQ-10** clássico, ReplayGain | M1 · E04-01 | obrigatório e interativo | planejado |
| Fundação | **EQ-20, EQ-31, EQ-PEQ32**, presets | M3 · E04-04 (UI E02-07) | obrigatório e interativo | planejado |
| Fundação | **EQ-64 gráfico nativo** (e PEQ de 64 filtros) | **M5** · E12-10 | **obrigatório e interativo agora (64 controles reais)** | planejado no nativo (M5) |
| Fundação | Medição de clipping, peak/VU/FFT básicos | M2 · E06-05 | todos os modos, com `[DEMO]` quando não medem | planejado |
| Fundação | Proteção final true-peak | M3 · E17-07/08 | indicador honesto | planejado |
| Fundação | Resampler, dither | M0 · E04-02 | config + estados | planejado |
| Fundação | Modo transparente e diagnósticos | M3 · E04-06 | diálogo "Sair do modo transparente?" | planejado |
| **Efeitos já planejados** | **Vinyl Simulator + Tube Amplifier Simulator (vitrola — módulo DSP interno)** | após `vynamp-decode` Camada 1 + **E04-03**; integração em **TASK-E19-07** | controles completos, `[DEMO]` sem modelo de sinal | design aprovado 2026-10-23, incorporado em 2026-11-05; prioridade preservada |
| Fase 2 / Hi-Fi | Compressor, limiter criativo, tone, balance/mono | M3 · E04-07 | config `[DEMO]` | planejado |
| Fase 2 / Hi-Fi | AutoEQ/import, perfis por dispositivo | M3 · E04-09 | import/validação | planejado |
| Fase 2 / Hi-Fi | Tempo/pitch/criativos | M3 · E04-08 | config | planejado |
| Fase 2 / Hi-Fi | Loudness/LUFS (analyzer em batch) | M3 · E08-03 | medidor `[DEMO]` | planejado |
| Fase 2 / Hi-Fi | Crossfeed, Convolution/FIR, loudness compensation em tempo real, true-peak meter em tempo real | sem TASK | config `[DEMO]` | proposto |
| Fase 3 / Avançado | Dynamic EQ, multibanda, M/S, width, delay/fase, bass management, room correction, HRTF, declipper, bass enhancer, exciter | sem TASK (candidatos M5 ou RFC) | config `[DEMO]` | proposto |
| Fase 3 / Avançado | Editor multicanal do EQ, surround rotulado | M5 · E12-10 | prévia rotulada | planejado |
| Fase 4 / Ecossistema | Host VST3 isolado, spike WASM DSP | M5 · E12-07 | prévia `[PLANNED]` | planejado |
| Fase 4 / Ecossistema | LV2/CLAP, compartilhamento de presets | sem TASK | — | dependente de investigação |

Critérios de avanço: E04-03 aprovado (grafo, swap e crossfade) → EQ-20/31/PEQ32 e vitrola. E04-10 (matriz numérica e de hardware) antes de qualquer item M5. Benchmark de CPU/latência publicado antes de aceitar o EQ-64 nativo. Riscos principais: CPU de cascatas longas e FIR, conflito D-DSP-05 da posição do vitrola, schema de preset v2 e implementação do módulo (TASK-E19-07).
