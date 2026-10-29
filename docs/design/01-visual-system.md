# Identidade visual, cores e componentes

Versão: 0.7.1 · Data: 2026-10-28 · Estado: especificação proposta, implementação não iniciada.

## Direção visual

Classic Graphite: gabinete grafite/azul escuro, detalhes metálicos frios, display preto com verde-lima, medidores âmbar e ciano. Três módulos empilhados, barras de título curtas, botões de transporte quadrados, trilhos compactos e playlist densa. Fidelidade à sensação e hierarquia clássica; todas as ilustrações, ícones, geometria e tokens são próprios. Valores abaixo são proposta Vynamp, não amostragem certificada de um skin Winamp.

| Token | Hex | Uso |
|---|---|---|
| background | #11131B | Área externa |
| chassis | #252936 | Painéis |
| panel-raised | #333A4D | Botões e cabeçalhos |
| panel-inset | #191D2A | Reentrâncias |
| display | #080D0B | Tempo, espectro e lista clássica |
| border-light | #677086 | Borda iluminada superior/esquerda |
| border-dark | #090C14 | Borda inferior/direita |
| text-primary | #F1F3F7 | Texto de controles |
| text-secondary | #B8C0CF | Metadados e labels |
| text-muted | #8D97AB | Desabilitado/informação secundária |
| lime | #B8F35A | Tempo e item tocando |
| cyan | #66D9EF | Foco e seleção de ferramenta |
| amber | #FFBF69 | RMS/VU e ações de edição |
| danger | #FF6B7A | Erro/clip |
| selection | #32453A | Linha selecionada |

Tokens semânticos não podem ser substituídos por RGB arbitrário nos componentes. [JSON canônico](../../assets/design-tokens.json). Tema Midnight Modern usa mesma família, menos bevel e display maior; Copper usa âmbar como accent. Todos mantêm nomes, estados e layout acessível. Nenhum tema embarcado copia skins existentes.

## Tipografia e dimensões

UI: system sans ou Noto Sans com licença incluída; metadados 13–14 DIP, ações 14, seção 16, título 18. Display: fonte monoespaçada própria ou fonte livre revisada, 30 DIP para tempo, 13 para codec. Letras digitais são opção visual, nunca única representação acessível.

Grid 4 DIP; padding 8/12/16; gaps entre módulos 8; raio classic 2/modern 8; outline 1; bevel 1 interno. Densidade Compacta usa linha 24 e botões 28; Confortável usa linha 36 e botões 40. Alvos mínimos desktop 24×24 DIP; modo touch 44×44. Não diminuir abaixo do mínimo para imitar pixels antigos.

## Estados

Normal: raised; hover: borda cyan e luminosidade +8%; pressed: inset; checked: ponto lime + label acessível; disabled: text-muted e ausência de efeito hover, sem depender só de opacidade. Focus-visible: anel cyan 2 DIP com gap 2. Erro: ícone/label além de danger. Playing: ícone triangular + lime, seleção não confunde com faixa atual. Loading: label de ação e spinner limitado.

## Movimento e renderização

Hover 80 ms; expansão 160 ms; modais 120 ms; sem animação bloqueante. Reduced motion desliga transições, marquee e visualizador animado. 60 Hz para UI interativa, visualização 30 Hz padrão, 60 opcional, 10 em economia. Janela minimizada suspende desenho; áudio continua. Não usar flashes >3/s nem animações de alto contraste automáticas.

## Aceite visual

Screenshots de referência em 100/125/150/200% DPI, ambos temas e todos os estados; diferença aprovada por revisão humana além de regressão por imagem. Texto nunca corta controle essencial em PT-BR/EN; títulos longos usam elipse e tooltip, marquee opcional só ao hover. Contraste de texto normal ≥4.5:1; texto grande e indicadores essenciais ≥3:1; medir pares reais, especialmente estados selected/disabled. Preview não é prova de conformidade.

## Knobs de controle DSP

Contrato de controles em [painel DSP §8](05-dsp-panel.md#8-acesso-e-controles-dsp-ux) (DSP-UX-06..10, 20). Tokens de knob a consolidar nos tokens existentes; valores abaixo são referência visual vinda da referência externa rev. 14 (relatada, não verificada neste repositório), não exigência de tamanho fixo em toda plataforma.

| Token | Referência | Regra |
|---|---|---|
| knob-size | 88×88 DIP | área do controle; o alvo mínimo da plataforma (24 DIP desktop, 48 dp touch/Android, conforme [painel DSP §1](05-dsp-panel.md#1-princípios) e REQ-E13-05; a referência externa não define tamanho de toque) prevalece se maior |
| knob-arc | 270° | varredura do arco de valor; posição normalizada não é o valor serializado |
| knob-track / knob-fill / knob-pointer | **proposto**: tokens de panel-inset, amber ou cyan e text-primary | cor do arco nunca é a única indicação de valor; rótulo, unidade e readout sempre presentes |
| knob-readout | **proposto**: text-secondary, fonte de dados do display | valor clicável abaixo ou junto do knob, abre o editor exato |

Sem bordas ou caixas decorativas por parâmetro, sem cores novas competindo com a música; foco usa o anel cyan padrão; movimento reduzido remove a animação do ponteiro sem mudar o valor. Sliders dos equalizadores gráficos não viram knobs.

## Acabamento de VU meters

Três acabamentos de apresentação dos 12 VUs estéreo: **Original**, **Metal adaptativo** e **Integrado ao painel** (padrão na referência externa, relatado, e **proposto** como padrão nativo; D-DSP-18). Usam tokens semânticos da superfície da skin (panel-raised, border-light, border-dark) para o gabinete externo; madeira e champagne mantêm seus materiais. Os SVGs aprovados permanecem inalterados e LEDs, escala e ponteiros nunca são recoloridos. Contraste de texto e indicadores segue "Aceite visual". Contrato VU-INT: [painel DSP §6.1](05-dsp-panel.md#61-integração-visual-dos-vu-meters-vu-int). Protótipo: relatado e não verificado aqui; nativo: nada implementado.

## Marca aprovada

Vynamp, conceito C — Pulso. [Guia de identidade](04-brand-identity.md) e assets/branding são canônicos para logo/ícones. Marca âmbar/grafite/branco é separada dos tokens semânticos desta interface; lime/cyan/display não são alterados pela marca.
