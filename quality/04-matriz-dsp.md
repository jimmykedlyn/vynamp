# Matriz de testes DSP e requisitos não funcionais

Versão: 0.7.4 · Data: 2026-11-07 · Estado: testes **planejados** para o produto nativo; nenhum foi executado aqui. Execuções do protótipo web aparecem só como evidência relatada da referência externa rev. 14/15, não verificada neste repositório. O harness numérico é planejado em TASK-E10-03 (M2) e o relatório de qualidade em TASK-E04-10 (M3).

Os testes por modalidade de EQ estão em [equalizadores §16](../reference/audio/08-equalizers.md#16-testes-e-aceite-por-modalidade) (EQ-T-01..15) e os do vitrola em [vitrola §7](../reference/audio/10-vitrola.md#7-testes-de-integração-planejados) (VIT-T-01..10). Cenários agregados: QA-005, QA-027, QA-028, **QA-032** (EQ) e **QA-033** (vitrola) em [estratégia de testes](01-estrategia-testes.md).

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

Convenção das subseções 2.1 e 2.2: testes **planejados para o nativo** e testes **executados no protótipo** ficam em colunas separadas. A coluna do protótipo é evidência relatada da referência externa (Chrome/Windows, fixtures controladas; protótipo rev.14/15, histórico; fonte atual: vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452 rev.38; os arquivos `*-validation.json` e `interactive/tests/` não existem neste repositório) e não equivale a certificação de acessibilidade, medição de qualidade sonora, nem a teste da engine nativa. Nativo: nenhum teste executado.

### 2.1 Aceite de acesso e controles (DSP-UX-AC)

Origem: pedido v1.4 §9.1, complementa os testes técnicos anteriores. Requisitos cobertos: DSP-UX-01..21, exceto DSP-UX-09 (campo textual ou seletor), que não tem cenário próprio nesta matriz, em [painel DSP §8](../design/06-painel-dsp.md#8-acesso-e-controles-dsp-ux). IDs locais mapeáveis aos oficiais.

| ID | Cenário | Resultado exigido | Requisitos | Protótipo (relatado) | Nativo |
|---|---|---|---|---|---|
| DSP-UX-AC-01 | Abrir pelo cabeçalho e pelos quatro atalhos (EQ, Dinâmica, Estéreo, Vitrola). | Painel/editor correto, sem navegação profunda; estado compartilhado e retorno de foco ao fechar. | UX-01, 02, 05 | `audio-header-validation`, testes de retorno de foco | planejado |
| DSP-UX-AC-02 | Percorrer as oito categorias, pesquisar e alternar módulos. | Categoria e módulo selecionados identificáveis, destino visível, catálogo completo alcançável; busca anterior não esconde o atalho acionado. | UX-03, 04 | testes de categorias | planejado |
| DSP-UX-AC-03 | Ajustar knob por arraste vertical, Shift e teclado; ajustar slider e chave. | Valor, unidade, posição e estado efetivo coerentes; nomes acessíveis; sem depender somente de gesto. | UX-06..08, 11, 12, 13, 19 | `audio-controls-validation` | planejado |
| DSP-UX-AC-04 | Abrir edição exata, confirmar valor válido e cancelar outro ajuste. | Confirmação aplica uma vez; cancelamento mantém o valor anterior; foco retorna ao acionador. | UX-10, 12, 15 | `audio-controls-validation` | planejado |
| DSP-UX-AC-05 | Carregar preset válido com valor fora do passo de gesto (por exemplo −23,5 quando permitido). | Valor preservado no estado, no readout e na exportação; **não quantizado** ao renderizar ou trocar tema. | UX-14 | `audio-control-regressions` (regressão 1) | planejado |
| DSP-UX-AC-06 | Tentar crossover igual ou invertido e depois corrigir. | Rejeição mantém o editor aberto e o último valor válido; mensagem explica a relação; correção válida confirma e sincroniza os controles. **Valor rejeitado não é aceito visualmente.** | UX-15, 16 | `audio-control-regressions` (regressão 2) | planejado |
| DSP-UX-AC-07 | Duplo clique/reset acessível, bypass, reset de módulo e troca de preset. | Escopos distintos e comportamento documentado; nenhuma reinicialização indevida de outros módulos/modos. | UX-11, 17 | `audio-controls-validation` | planejado |
| DSP-UX-AC-08 | Recarregar e abrir o mesmo efeito por outro acesso. | Persistência e estado único preservados; sem cópias divergentes de parâmetros. | UX-02, 17 | `audio-controls-validation` (acessos) | planejado |
| DSP-UX-AC-09 | Navegar com teclado, texto ampliado, alto contraste e movimento reduzido nas resoluções suportadas. | Sem controles essenciais cortados; foco, valor e unidade legíveis; navegação responsiva utilizável. | UX-05, 12, 18, 19 | `audio-header-validation` (ampliação); leitor de tela não validado | planejado (leitores de tela na plataforma final) |
| DSP-UX-AC-10 | Percorrer EQ10/20/31/64 e PEQ32 após aplicar o design de controles. | Contagens e funções obrigatórias preservadas; sliders gráficos reais e editáveis, não substituídos por decoração. | UX-07; [EQ §14](../reference/audio/08-equalizers.md#14-mvp-htmlsvgjs--contrato-documental) | `v12-control-evidence`, `modular-editing-validation` | planejado |
| DSP-UX-AC-11 | Verificar rótulos de capacidade e cadeia de áudio. | A UI identifica DEMO, indisponível ou processamento comprovado; não alega som alterado só porque o knob se move. | UX-18, 21 | relatado, sem arquivo específico citado | planejado |
| DSP-UX-AC-12 | Testar reprodução, fila, waveform, 12 skins e 16 VUs e abertura/fechamento do DSP. | Sem regressão de reprodução ou layout; VUs originais estéreo e proporções preservados. | UX-20; VU-INT | `interactive-validation` | planejado |

As duas regressões permanecem explícitas e valem como critério de aceite no nativo: **(1) não quantizar preset válido pelo passo visual** (AC-05) e **(2) não aceitar visualmente um valor rejeitado pelo estado canônico** (AC-06; readout, controle, gráfico e estado persistido mostram o mesmo valor efetivo). Os 12 cenários são planejados para o nativo; a execução no protótipo não os cumpre.

### 2.2 Grafo modular e VU-INT

Origem: complemento rev. 15 (28/10/2026). Contratos: [DSP, grafo modular](../reference/audio/03-dsp.md#grafo-modular-protótipo--nativo) (o grafo é contrato do protótipo; reordenação livre nativa é decisão aberta, D-DSP-12), [painel DSP §9](../design/06-painel-dsp.md#9-fluxo-modular-grafo--referência-do-protótipo) e [§6.1 (VU-INT)](../design/06-painel-dsp.md#61-integração-visual-dos-vu-meters-vu-int). Os testes técnicos DSP-T-xx e EQ-T-xx permanecem; nenhuma linha afirma execução nativa.

| ID | Cenário | Resultado exigido | Protótipo (relatado) | Nativo |
|---|---|---|---|---|
| VU-INT-01 | Comparar byte a byte os 16 SVGs aprovados antes e depois do acabamento. | Arquivos inalterados: escalas, ponteiros, vidro, LEDs, marcações, coordenadas, proporções e identidade preservados. | `meter-finish-validation` | planejado |
| VU-INT-02 | Inspecionar a cópia DOM exibida em cada acabamento. | Só gabinete externo, junta/contorno e rótulos externos adaptados; nada apagado, recortado ou simplificado. | `meter-finish-validation` | planejado |
| VU-INT-03 | Verificar o host em todas as skins. | Sem moldura, sombra ou fundo de container concorrente; o host fornece dimensão, foco e interação. | `modular-visual-validation` | planejado |
| VU-INT-04 | Verificar cores de LEDs, escala e ponteiros; madeira e champagne. | Tokens semânticos da skin só na apresentação externa; LEDs, escala e ponteiros não recoloridos; madeira e champagne mantêm material próprio. | `meter-finish-validation`; comparativo de 72 casos (12 skins × 6 acabamentos) | planejado |
| VU-INT-05 | Tamanho natural pelo viewBox, Ampliar, clique para alternar e botão direito para os 16 modelos. | Ordem e funcionamento do menu inalterados pelo acabamento. | `interactive-validation` | planejado |
| VU-INT-06 | Percorrer 12 skins × 16 VUs (192 combinações), claro/escuro, mobile e restauração do original (seis acabamentos, na ordem do seletor do MVP: Integrado · gabinete compacto, sem moldura externa, moldura ultrafina, acabamento adaptativo à skin, embutido · profundidade discreta e original completo · sem recorte; preferência persistida em Aparência). | Todas legíveis, sem distorcer o desenho; ampliação para detalhes pequenos; reversão e persistência do acabamento. | `meter-finish-validation` (192), `modular-visual-validation` | planejado |
| DSP-GR-01 | Construir grafo: portas tipadas, saída→entrada do mesmo tipo, CV do LFO só na modulação do Ganho. | Rejeita ciclos, duplicatas, autoconexões, portas ausentes, valores inválidos e preset incompatível, com mensagem; nós Entrada/Saída não removidos nem duplicados. | `modular-ui-validation` | planejado |
| DSP-GR-02 | Rota topológica determinística; nó desconectado; cadeia habilitada sem caminho; fan-out e soma paralela. | Desconectado não processa nem mostra fluxo; sem caminho = silêncio, sem rota seca oculta; a soma exige headroom. | `modular-audio-validation`, `modular-live-validation` | planejado |
| DSP-GR-03 | Instâncias: duplicar nó e usar "Copiar ajuste do catálogo". | Parâmetros por instância, sem compartilhar por referência; o catálogo não controla instâncias silenciosamente. | `modular-ui-validation` | planejado |
| DSP-GR-04 | Mover, pan, zoom, recolher e trocar acabamento durante a reprodução. | Cadeia de áudio não reconstruída; ajustes comuns suavizados; mudança de topologia com transição curta e liberação das rotas antigas. | `modular-audio-validation`, `modular-live-validation` (12 trocas rápidas de bypass) | planejado |
| DSP-GR-05 | Capacidade real × configuração: EQ/PEQ no grafo, Compressor, Limiter amostral, Saturação, Vinyl/Tube, True Peak/LUFS. | Só famílias com adaptador processam; **Limiter amostral não é True Peak nem garante proteção**; **Saturação não é o Tube**; nó indisponível na rota ativa impede a ativação com motivo, e bypass explícito não alega o efeito. | `modular-audio-validation`, `modular-live-validation` (−12 dB medidos; sem fluxo falso em demo) | planejado |
| DSP-GR-06 | Preset: importar JSON versionado, A/B, histórico (até 60), reabrir. | Validação atômica antes de substituir; leitura atrasada não sobrescreve edição nova; reabertura e importação iniciam em bypass, sem reprodução automática; A/B de sessão sem compensação de loudness. | `modular-ui-validation`, `modular-review-validation` | planejado |
| DSP-GR-07 | Medição estéreo e animação de fluxo. | Pico = máximo L/R; RMS pela energia dos dois canais; oposição de fase não cancela a medição; animação só com áudio local tocando, cadeia habilitada e extremos na rota, sem bypass; DEMO nunca fabrica processamento. | `modular-stereo-meter-validation`, `modular-live-validation` | planejado |
| DSP-GR-08 | Limites do adaptador e acessibilidade do canvas. | Excesso de nós, conexões, tamanho de preset, biquads ou IR gera motivo explícito; alternativa sem arraste, teclado, Escape, foco, alto contraste, movimento reduzido, fonte 200% sem overflow; contraste dos fios ≥ 3:1. | `modular-layout-validation`, `modular-contrast-validation`, `modular-editing-validation` | planejado (limites do adaptador não são metas nativas; medir latência/CPU antes) |

## 3. Requisitos não funcionais verificáveis

| # | Requisito | Critério | Origem/estado |
|---|---|---|---|
| NFR-1 | Qualidade sonora | transparência sample-exact com DSP desligado; filtros ±0,2 dB; sem clipping evitável; coloração do vitrola intencional, controlável e totalmente reversível (VIT-T-01) | planejado |
| NFR-2 | Performance | callback p99.9 < 25% do período; reprodução estéreo 24/192 ≤ 5% de CPU total na máquina de referência; visualizer ≤ 10%; vitrola < 5% de um núcleo a 96 kHz (meta). Metas de EQ-64/PEQ32/FIR **só depois do benchmark** | [performance](02-performance.md); vitrola §8.5 |
| NFR-3 | Robustez | DSP-T-04/14/17; perda de dispositivo → pausa e posição preservada | planejado |
| NFR-4 | Compatibilidade | só formatos, canais e backends suportados ou explicitamente planejados ([saídas](../reference/audio/04-outputs-hires.md), `vynamp-output`) | planejado |
| NFR-5 | Segurança | presets/IR/plugins com limites de tamanho, schema e JSON pointer de erro; IR decodificada pela `vynamp-decode` com limites; plugins isolados ([ameaças](../security/01-threat-model.md)) | planejado |
| NFR-6 | Privacidade/offline | nenhum serviço remoto para DSP local; import de preset/AutoEQ é manual | planejado |
| NFR-7 | Observabilidade | contadores atômicos de clipping/underrun/latência; logs fora da RT, sem dados sensíveis | planejado |
| NFR-8 | Usabilidade | defaults seguros (EQ plano e desligado, vitrola desligado e sutil quando ligado), reset por módulo, desfazer | planejado/proposto |
