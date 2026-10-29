# Matriz de testes DSP e requisitos não funcionais

Versão: 0.7.1 · Data: 2026-10-25 · Estado: testes **planejados**; nenhum foi executado. O harness numérico é planejado em TASK-E10-03 (M2) e o relatório de qualidade em TASK-E04-10 (M3).

Os testes por modalidade de EQ estão em [equalizadores §16](../audio/08-equalizers.md#16-testes-e-aceite-por-modalidade) (EQ-T-01..15) e os do vitrola em [vitrola §7](../audio/10-vitrola.md#7-testes-de-integração-planejados) (VIT-T-01..10). Cenários agregados: QA-005, QA-027, QA-028, **QA-032** (EQ) e **QA-033** (vitrola) em [estratégia de testes](01-test-strategy.md).

## 1. Pipeline e módulos

| ID | Objetivo | Método | Evidência esperada | Critério de aprovação |
|---|---|---|---|---|
| DSP-T-01 | Bypass não muda o sinal | todos os nós em bypass; PCM pseudoaleatório 16/24/32 bits e float | diff de amostras | float: idêntico após o crossfade; transparente: bit a bit no caminho inteiro |
| DSP-T-02 | Sem clicks/pops | toggles, presets e automação rápida (100 mudanças/s) sobre tom e ruído rosa | detector de descontinuidade (E10-03) + gravação | nenhum degrau acima do limiar; sem zipper |
| DSP-T-03 | Headroom e clipping por estágio | EQ +15 dB em várias bandas, vitrola, compressor makeup | medidores por estágio | clipping indicado no estágio certo; proteção final ≤ ceiling +0,1 dBTP (QA-027) |
| DSP-T-04 | Troca de Fs, canais e formato | 44,1↔48/96/192/384 kHz, mono/estéreo/5.1 no meio da fila | log de `prepare` + captura | sem NaN, sem stall; nós incompatíveis com motivo; posição correta |
| DSP-T-05 | AutoEQ/import | perfis válidos, inválidos, de versão futura e gigantes | log de validação | inválido mantém o anterior; origem/licença exibidas |
| DSP-T-06 | FIR/IR | IR válida, grande demais, canais errados, Fs diferente, corrompida | log + medição | rejeições claras; latência declarada = medida; nunca "aplicada" se falhou |
| DSP-T-07 | ReplayGain/loudness | faixas com/sem tag, track/album, Opus R128 | ganho aplicado vs. esperado | ganho correto uma única vez; LUFS × dB não misturados |
| DSP-T-08 | Compressor/multibanda | tons em degrau, ruído rosa | curva estática + envelopes | threshold/ratio/attack/release dentro da tolerância do algoritmo |
| DSP-T-09 | True-peak | fixtures com overs inter-sample | oráculo independente no PCM final (E17-08) | ≤ −1 dBTP +0,1 dB incluindo dither |
| DSP-T-10 | Módulos colorativos propostos | sweep/tom antes e depois | espectro + LUFS | efeito rotulado; ganho compensado quando prometido |
| DSP-T-11 | Imagem estéreo | sinais L-only, R-only, M, S, correlacionados | correlação/goniômetro | mono-compatibilidade declarada; delays exatos em amostras |
| DSP-T-12 | Matriz/multicanal | 5.1/7.1 identificáveis por canal | captura por canal | mapeamento correto; downmix com headroom |
| DSP-T-13 | Resampling, conversão e dither | sweep até Nyquist, tons de baixo nível, 16/24/32 | espectro de aliasing/ruído | aliasing e ruído dentro das metas do ADR de E04-02; dither só no estágio final |
| DSP-T-14 | Tempo/pitch/criativos | fala e música, tails, seek/stop | captura | latência indicada; tails seguem a política; sem NaN |
| DSP-T-15 | Bit-perfect × DSP | ligar EQ/vitrola em transparente; DSD/DoP | estado da UI + `PathProperties` | pede para sair do modo; nenhum selo incorreto; DSP desligado em DSD |
| DSP-T-16 | Presets e restauração | preset corrompido, versão incompatível, migração v1→v2 | log + estado | restauração segura; nada é aplicado parcialmente |
| DSP-T-17 | Robustez e soak | 8 h, seek rápido, troca/perda de dispositivo, CPU elevada | contadores de underrun e callback | zero underrun causado pelo app; p99.9 da callback < 25% do período |
| DSP-T-18 | Visualizações coerentes | o mesmo sinal em VU/peak/true peak/LUFS | leituras | valores distintos e corretos por definição (QA-028) |
| DSP-T-19 | Plugins (M5) | plugin travado/crash | log | bypass/circuit breaker; o player continua (E12-07) |

## 2. MVP HTML/SVG/JS

Matriz PASS/FAIL/BLOCKED obrigatória para o protótipo (contagem de controles, persistência ao recarregar, troca de tema, teclado, navegação responsiva e rótulos `[DEMO]`). Detalhe em EQ-T-01/02/08/09/13/14. Mockup estático não é aceito; "não verificado" deixa a entrega INCOMPLETA.

## 3. Requisitos não funcionais verificáveis

| # | Requisito | Critério | Origem/estado |
|---|---|---|---|
| NFR-1 | Qualidade sonora | transparência sample-exact com DSP desligado; filtros ±0,2 dB; sem clipping evitável; coloração do vitrola intencional, controlável e totalmente reversível (VIT-T-01) | planejado |
| NFR-2 | Performance | callback p99.9 < 25% do período; reprodução estéreo 24/192 ≤ 5% de CPU total na máquina de referência; visualizer ≤ 10%; vitrola < 5% de um núcleo a 96 kHz (meta). Metas de EQ-64/PEQ32/FIR **só depois do benchmark** | [performance](02-performance-observability.md); vitrola §5 |
| NFR-3 | Robustez | DSP-T-04/14/17; perda de dispositivo → pausa e posição preservada | planejado |
| NFR-4 | Compatibilidade | só formatos, canais e backends suportados ou explicitamente planejados ([saídas](../audio/04-outputs-hires.md), `vynamp-output`) | planejado |
| NFR-5 | Segurança | presets/IR/plugins com limites de tamanho, schema e JSON pointer de erro; IR decodificada pela `vynamp-decode` com limites; plugins isolados ([ameaças](../security/01-threat-model.md)) | planejado |
| NFR-6 | Privacidade/offline | nenhum serviço remoto para DSP local; import de preset/AutoEQ é manual | planejado |
| NFR-7 | Observabilidade | contadores atômicos de clipping/underrun/latência; logs fora da RT, sem dados sensíveis | planejado |
| NFR-8 | Usabilidade | defaults seguros (EQ plano e desligado, vitrola desligado e sutil quando ligado), reset por módulo, desfazer | planejado/proposto |
