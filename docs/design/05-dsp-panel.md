# Painel Audio DSP, equalizador e medidores — UI/UX

Versão: 0.7.1 · Data: 2026-10-25 · Estado: especificação proposta. Não há UI DSP implementada nem no nativo nem no MVP HTML/SVG/JS.

Base: [visual](01-visual-system.md), [layouts](02-layouts.md), [acessibilidade](03-accessibility-i18n.md), [DSP](../audio/03-dsp.md), [equalizadores](../audio/08-equalizers.md), [vitrola](../audio/10-vitrola.md) e [visualizadores](../extensions/04-visualizer-contract.md). A UI nativa da cadeia e do EQ avançado é planejada em TASK-E02-07 (M3).

## 1. Princípios

- **Divulgação progressiva** (Hick): a visão **Básica** mostra preset de uso, On/Off geral, EQ e intensidades. A visão **Avançada** mostra parâmetros completos, roteamento, latências e medição.
- **Consistência** (Jakob): o EQ clássico mantém a metáfora de 10 sliders verticais do player clássico. Os demais módulos usam os mesmos sliders, toggles e menus do Vynamp.
- **Fitts**: alvos ≥ 24 DIP no desktop e ≥ 48 dp no Android, mesmo no editor de 64 bandas (rolagem/zoom em vez de encolher).
- **Gestalt**: os módulos são agrupados por categoria e a cadeia aparece como uma linha de nós na ordem real.
- **Feedback**: todo ajuste reflete valor, curva e estado na hora; o áudio muda em até 50 ms.
- **Honestidade**: "processado" × "transparente" × "DEMO" nunca se confundem.

## 2. Estrutura do painel Audio DSP

| Área | Conteúdo | Estado |
|---|---|---|
| Cabeçalho | toggle **DSP geral** (bypass global com crossfade), preset de uso, A/B, desfazer/refazer, perfil do dispositivo atual | proposto |
| Presets de uso | **Original/Neutro** (DSP desligado), **Fones**, **Caixas estéreo**, **Home Theater** (só se o caminho multicanal for suportado; senão, indisponível com motivo), personalizados | proposto |
| Navegação | Ganho & EQ · Correção · Loudness & dinâmica · Estéreo & canais · Tempo & formato · **Vitrola** (Vinil, Válvula) · Medidores | proposto |
| Cadeia | inspector linear da [cadeia real](../audio/03-dsp.md#cadeia-real-de-processamento): nome, on/bypass (ícone + texto), parâmetros-chave e latência declarada. Arrastar para reordenar **só** nos nós reordenáveis e validados | planejado (E02-07); reordenação: proposta |
| Rodapé | headroom estimado, indicador de clipping por estágio, latência total, CPU do DSP, buffer/underruns | proposto |

Estados de cada módulo (ícone + texto, nunca só cor): **Ativo** · **Inativo** · **Bypass** · **Incompatível** (motivo: transparente, DSD, multicanal, Fs) · **Erro** (bypass seguro + diagnóstico) · **DEMO** (somente no MVP) · **Indisponível nesta versão** (marco).

## 3. Equalizador

- **Classic compacto** (576×180): preamp, 10 bandas, On/Off, Auto, Reset, Presets ▾ e mini curva calculada.
- **Editor avançado** (960×600): seletor **Gráfico 10 · Gráfico 20 · Gráfico 31 · Gráfico 64 · Paramétrico (até 32 filtros)**, gráfico grande (log de 10 Hz a min(40 kHz; 0,45×Fs), eixo em dB), curva prevista e curva medida separadas, pontos arrastáveis, tabela de bandas/filtros, preamp, presets por modo, reset, bypass, aviso de headroom/clipping e contador "n/32" no paramétrico. Especificação completa: [equalizadores §12](../audio/08-equalizers.md#12-uiux-do-equalizador).
- **64 bandas**: 64 sliders reais com rolagem horizontal, zoom e minimapa. Não pode haver 10 sliders reais e o resto decorativo.
- **Android**: 10 bandas com rolagem horizontal explícita; 20/31/64 e paramétrico em editor de tela cheia.

## 4. Cards Vinil e Válvula

Dois cards independentes, conforme [vitrola §5](../audio/10-vitrola.md#5-ux-hospedeira):

- **Vinil**: intensidade, ruído/estalos/rotação progressivos, rpm, rituais (com aviso de que alteram a duração), lado/disco atual em texto, desgaste e "Comprar disco novo". Nenhum ruído com o card desligado.
- **Válvula**: aquecimento (Aquecendo… / Pronto), perfil, saturação, sag e transformador. Qualidade de oversampling automática, mostrada como informação.

Cada card tem bypass próprio, presets, A/B com loudness igualado, restaurar padrões e o selo **"altera o áudio intencionalmente"**. É proibido ilustrar toca-discos ou válvulas.

## 5. Mensagens e fluxos

| Situação | Mensagem (pt-BR) | Ação |
|---|---|---|
| Ligar DSP em transparente | "Sair do modo transparente? O processamento será ativado e o selo removido." | Confirmar / Cancelar |
| Boost sem headroom | "A curva soma +X dB. Risco de clipping." | "Ajustar preamp (−X dB)" / Ignorar |
| Clipping detectado | "Clipping em <estágio>" (indicador persistente por 2 s) | abrir medidores |
| Latência alta | "Latência do DSP: N ms (FIR/lookahead)" | ver nós |
| Banda acima de Nyquist | "Indisponível: F > limite de 0,45 × Fs" | — |
| Módulo incompatível | "Indisponível: <motivo>" | link para a causa |
| Preset inválido | "Preset não carregado: <campo> inválido. Preset anterior mantido." | detalhes |
| vitrola desativado | "Vitrola desativado: caminho bit-perfect / DSD nativo / transparente" | "Sair do modo transparente" |
| MVP sem áudio | `[DEMO — sem processamento de áudio neste modo]` | — |

Fluxos principais: (1) primeira execução com EQ plano e desligado; (2) escolher o preset de uso "Fones" → EQ + AutoEQ importado + crossfeed (quando existir) → A/B; (3) abrir o editor avançado → trocar para 64 → ajustar → salvar preset do modo; (4) ligar o vitrola → intensidade sutil → painel avançado.

## 6. Medidores e visualização

Instrumentos de exibição **não são efeitos DSP** e nunca alteram a saída. As convenções dos VU meters existentes (SKIN-REQUIREMENTS §5.13: troca de modo por clique/V, 0 VU = −18 dBFS ajustável, slot de 28 DIP) ficam preservadas. Contrato técnico: [visualizadores](../extensions/04-visualizer-contract.md) (TASK-E06-05/06, E17-09).

| Medidor | Definição / metodologia | Estado |
|---|---|---|
| Spectrum FFT | janela Hann, FFT 2048/4096 padrão, smoothing/hold/decay em unidades reais, escala log/linear | planejado |
| VU (balístico) | integração ~300 ms, 0 VU = −18 dBFS (−14/−20 opcionais). **Não** é peak | planejado; valores [proposal] da skin |
| Peak / PPM | amplitude de sample peak com hold | planejado |
| True Peak (dBTP) | oversampling ≥ 4× conforme ITU-R BS.1770 (algoritmo pinado na fixture); medido no tap final | planejado |
| Loudness (LUFS) | momentary 400 ms, short-term 3 s, integrated com gating (BS.1770 / EBU R128), LRA (EBU Tech 3342) | planejado (analyzer E08-03); medidor em tempo real: proposto |
| Osciloscópio, goniômetro/vectorscope, correlação | do tap escolhido | planejado (E06-05/06) |
| Curva do EQ/FIR | calculada dos filtros reais; "prevista" × "medida" | planejado |
| Redução de ganho, CPU, buffer, latência | contadores RT coletados fora da callback | planejado |

- **Pontos de medição**: source / pre-EQ / post-EQ / **final (padrão)**. A unidade e a Fs aparecem no tooltip. Medidores pré/pós por módulo (ex.: pré/pós-vitrola) são **propostos**.
- **Taxa de UI**: 30 fps padrão, 60 opcional, 10 em economia; suspenso quando minimizado ou em background (Android: zero render). Reduced motion: estático com números.
- **Coerência**: VU, peak, true peak e LUFS **não** mostram o mesmo número com rótulos diferentes.
- Orçamento: visualizer ≤ 10% de CPU total agregado ([performance](../quality/02-performance-observability.md)).

## 7. Acessibilidade

Teclado completo (Tab entre grupos, setas/Home/End nos sliders), leitores de tela com nome, valor e unidade, contraste conforme os tokens, estados sem depender da cor, tooltips com unidade, sem eventos acessíveis por quadro, foco que nunca some ao trocar de modo e 200% de zoom sem perder controles.
