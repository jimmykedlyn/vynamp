# Decisões, hipóteses e pontos em aberto

Versão: 0.7.1 · Data: 2026-10-25 · Estado: especificação proposta, implementação não iniciada.

## Decisões fixadas pelo pedido

Produto independente em Rust; inspiração visual clássica com cores especificadas; temas e plugins próprios; streaming cliente agora e plataforma futura; ampla funcionalidade de áudio; incluir hi-res e recursos inspirados em AIMP e Poweramp. Não exigir compatibilidade Winamp.

## Hipóteses adotadas para execução

| ID | Decisão provisória | Razão / revisão |
|---|---|---|
| H-01 | Substituída: Windows/Linux/macOS Intel+Silicon/Android obrigatórios | Instrução do usuário; ADR-006; matriz/gates M0–M3 |
| H-02 | Resolvida: Vynamp, identidade C — Pulso | Escolha explícita do usuário; ADR-008; exports nativos antes M3 |
| H-03 | Sem conta para uso local | Reduzir dependência da plataforma |
| H-04 | Desktop UI Rust; Android shell Kotlin/Compose proposto, core Rust comum | Provar a11y/bridge/lifecycle em M0; não duplicar engine |
| H-05 | Desktop 1.0 gratuito sem anúncios no player | Política comercial ainda em aberto; não embutir billing no núcleo |
| H-06 | Temas/assets e implementação novos | Evitar acoplamento ao projeto anterior |
| H-07 | PCM até 384 kHz como alvo 1.0 | 768 kHz e DSD nativo em expansão, dependentes de hardware |
| H-08 | Web M4 com live e VOD | Não construir backend antes de validar cliente |

## Decisões pendentes com dono e prazo

UI final e backend exclusivo: responsável engenharia, M0, provas de acessibilidade/latência. Licença do código e marca: responsável produto, antes publicar M3. Encoder AAC/MP3, ASIO e FFmpeg: engenharia/revisão de distribuição, antes empacotar. Hosting/região/custos/CDN: plataforma, entrada M4. Catálogo próprio, royalties e territórios: produto/plataforma, antes ingest público. DRM e assinatura: nova RFC, não dependência de M1–M3. Datas dependem de equipe/hardware; estimativas por tarefa são faixas, não calendário prometido.

## Gestão de escopo

Adicionar requisito exige ID, fase, critérios mensuráveis e tarefa. Remover exige decisão explícita, atualização da matriz e changelog. “Todas as funcionalidades” significa cobertura da matriz acordada e do catálogo de recursos deste pacote, não afirmação de equivalência binária ou de cada opção histórica dos três produtos.

## Decisões DSP e equalizadores (2026-10-25)

Vêm da revisão documental do DSP (pedido "Documentação DSP — Equalizadores Obrigatórios" v1.3). Estado: **proposto**, até a aprovação do mantenedor no PR. Nenhuma altera código. Detalhes em [DSP](../audio/03-dsp.md), [equalizadores](../audio/08-equalizers.md), [catálogo](../audio/09-dsp-catalog.md) e [vitrola](../audio/10-vitrola.md).

| ID | Decisão | Razão / revisão |
|---|---|---|
| D-DSP-01 | Marcos do EQ: **PEQ nativo de 32 filtros continua M3** (TASK-E04-04); **gráfico nativo de 64 bandas e qualquer PEQ de 64 filtros continuam M5** (TASK-E12-10) até uma decisão documentada mudar isso; o **MVP HTML/SVG/JS cobre agora** a UI interativa de Gráfico 10/20/31/64 e Paramétrico até 32 | O pedido diz "64 bandas M5 permanece M5", mas em 03-dsp v0.7.0 o item M5 era o **paramétrico** de 64; os dois itens de 64 ficam em M5 e a UI do protótipo não muda o marco nativo |
| D-DSP-02 | Clássico e Avançado 10 são o mesmo estado (EQ-10); o clássico mostra ±12 dB e marca "fora da escala" quando o avançado passou disso, sem cortar o valor | Coerência entre painéis sem perda silenciosa |
| D-DSP-03 | Grades: EQ-20 em meia oitava `1000·2^((k−11)/2)` (22 Hz–16 kHz); EQ-31 em 1/3 de oitava ISO 266/IEC 61260-1 base 10; EQ-64 logarítmica `20·1000^(k/63)` (≈1/6,3 oitava, **não** 1/3); peaking de Q constante `Q=√(2^N)/(2^N−1)` | Faltava grade definida para 20 e 64; aprovar em E04-04/E12-10 |
| D-DSP-04 | Rótulo canônico do protótipo: `[DEMO — sem processamento de áudio neste modo]` (en: `[DEMO — no audio processing in this mode]`) | O SKIN-REQUIREMENTS-GPT v1.2 usa variantes (ver divergências) |
| D-DSP-05 | **Aberto**: o vitrola §8.1 pede "depois do resampler", mas o contrato 07 tem um único resampler final. A decisão vitrola fica preservada e o 07 não muda; opção A (taxa do stream) × B (resampler por stream, emenda do 07) | Precisa de decisão do mantenedor; afeta também mixer com Fs diferentes |
| D-DSP-06 | **Aberto**: crossfade com o vitrola ligado (candidato: desativar o crossfade com aviso) | A referência pressupõe disco contínuo/gapless |
| D-DSP-07 | O vitrola é hospedado por referência; os parâmetros "candidatos" do pedido que a referência não define ficam **a validar**; nos conflitos vale a referência; falta uma TASK de integração (M3, após E04-03) | Preservar o acordo anterior; criar a TASK num PR de backlog |
| D-DSP-08 | **Aberto**: os candidatos de resampler rubato/SoXR (de terceiros) em [saídas](../audio/04-outputs-hires.md) versus a preferência por bibliotecas próprias | Decidir no ADR de TASK-E04-02 |
| D-DSP-09 | Um único modo de EQ processa por vez; os outros ficam guardados. AutoEQ/headphone correction é outro nó, depois do EQ | Pipeline "gráfico OU paramétrico"; correção sem ocupar o EQ do usuário |
| D-DSP-10 | Ordem proposta no segmento por stream: EQ → correção → vitrola → dinâmica/imagem/criativos; reordenação livre não é prometida | Pedido v1.3 §3.5 compatível com vitrola §8.1 |

### Divergências encontradas

| Onde | O que diverge | Tratamento |
|---|---|---|
| Pedido v1.3 §"Requisito obrigatório" × `docs/audio/03-dsp.md` v0.7.0 §"EQ paramétrico" | "64 bandas M5" referia-se ao paramétrico, não ao gráfico | D-DSP-01 |
| `docs/audio/03-dsp.md` v0.7.0 §"EQ gráfico" | editor avançado só com 10/20/31, sem modo gráfico 64 | corrigido em 0.7.1 e [08](../audio/08-equalizers.md) |
| `tasks/items/TASK-E12-10.md` §Entrega | "EQ64" não diz se é gráfico ou paramétrico | interpretado como os dois (D-DSP-01); texto da TASK não alterado |
| `schemas/audio-preset.schema.json` v1 | sem `mode`, `enabled`, `slope`, nem modos gráficos | v2 pendente em TASK-E04-04 (este PR é só Markdown) |
| `SKIN-REQUIREMENTS-GPT-MVP-OBRIGATORIO-v1.2.md` §13.1 × §13.3.7 | rótulos `[DEMO — no actual audio processing]` e "Prévia interativa — o áudio não está sendo processado por este modo" diferem do rótulo do pedido v1.3 | D-DSP-04; arquivo não alterado |
| `SKIN-REQUIREMENTS-GPT-MVP-OBRIGATORIO-v1.2.md` §5.11 | presets vitrola Subtle/Warm/Rustic [proposal] × Clean/Warm/Vintage do pedido | a validar (D-DSP-07) |
| `SKIN-REQUIREMENTS-GPT-MVP-OBRIGATORIO-v1.2.md` §12.8 | lista módulos (Dynamic EQ, Bass Enhancer, Exciter, multibanda, declipper, crossfeed, M/S, HRTF…) ausentes do repositório | catalogados como **propostos** em [09](../audio/09-dsp-catalog.md) |
| `SKIN-REQUIREMENTS-CODEX.md` v1.0 §1.4 item 5, §5.9 (editor avançado e Android) e §6 | 64 bandas gráficas só como "reserved slot"; editor com 10/20/31; sem seletor de 5 modos | contradiz a UI obrigatória; corrigido para v1.1 fora do repositório (ver evidência do PR) |
| Pedido v1.3 §3.5.A "evitar ruído em pausa/silêncio" × vitrola §2 | a referência põe crackle nos silêncios entre faixas e o run-out em loop | vale a referência; pausa = a validar |
| Pedido v1.3 §3.5.B (Bias, Oversampling/Quality, seed) × vitrola §4/§8.7 | bias dinâmico interno, qualidade automática, semente automática | vale a referência; controles do usuário a validar |
| vitrola §8.1 × `docs/audio/07-dsp-signal-contract.md` §Caminho | "depois do resampler" × resampler único final | D-DSP-05 |
| Contexto informado × `ROADMAP-rodadas.md` (raiz) | não há linha vitrola; a ordem "após E04-03" está em `STATE.md` e no vitrola §6 | TASK de integração pendente (D-DSP-07) |
| `docs/audio/04-outputs-hires.md` §Backend matrix × design `vynamp-output` 2026-10-25 | "WASAPI shared via CPAL" e "ASIO opcional" × biblioteca própria substitui o CPAL; ASIO fora do núcleo | não editado (fora do escopo DSP); atualizar com TASK-E04-05/plano 05 da vynamp-output |
| `docs/audio/07-dsp-signal-contract.md` §Caminho | mixer soma streams antes do resampler final; fontes com Fs diferentes no crossfade não têm conversão definida | dependente de investigação (junto com D-DSP-05) |
