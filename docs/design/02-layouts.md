# Layouts, geometria e telas

Versão: 0.7.2 · Data: 2026-10-28 · Estado: especificação proposta, implementação não iniciada.

## Coordenadas canônicas

Unidade DIP, origem no topo esquerdo da área útil. DPI é responsabilidade do renderer; layout não multiplica manualmente pixels. Modos: Classic Stack (padrão), Library Workspace e Mini. [Referência visual original](../../assets/layout-reference.html) e [desenho clássico](../../assets/classic-layout.svg).

### Classic Stack

Janela útil inicial 600×760; mínimo 480×660; padding externo 12. Largura de painel W = largura útil −24. Layout abaixo usa W=576. Header principal x12 y12 w576 h28; player x12 y40 w576 h184; EQ x12 y232 w576 h180; playlist x12 y420 w576 h328; gaps 8. EQ oculto devolve espaço à playlist. Biblioteca abre workspace, preservando playback e geometria anterior.

| Região player, origem local | Retângulo x/y/w/h | Conteúdo |
|---|---|---|
| Display | 12 / 12 / 352 / 78 | Tempo, título, espectro e qualidade |
| Capa opcional | 376 / 12 / 72 / 72 | Thumbnail original/licenciada |
| Ações de fonte | 460 / 12 / 104 / 32 | Local / Rádio / Biblioteca |
| Seek | 12 / 102 / 552 / 20 | Slider, hover timestamp, estado LIVE |
| Transporte | 12 / 140 / 264 / 32 | Anterior, play, pause, stop, próxima, abrir |
| Volume / pan | 288 / 140 / 164 / 32 | Volume + mute; pan em popup |
| Modos | 464 / 140 / 100 / 32 | Shuffle / repeat |

Player header: menu esquerda, nome central, minimizar/fechar direita. Display tempo ocupa 100 DIP; título ocupa resto, codec e resolução em segunda linha. Espectro inferior 28 DIP, não cobre texto. EQ: titlebar 24, toolbar 28, preamp e dez bandas ocupam 100, footer 16. Playlist: titlebar24, tabs28, lista flex, footer32 com adicionar/remover/ordenar e contagem/duração. EQ gráfico avançado abre editor maior; não comprimir 31 bandas no painel clássico.

Abaixo de 560 DIP, capa sai, botões de fonte viram menu, volume vai à segunda linha, player ganha 32 DIP e playlist perde 32. Layout mínimo ainda permite teclado e targets adequados. Snap docking a 12 DIP, soltar com Alt ignora snap; módulos destacáveis apenas em M3. Posições persistidas por monitor e limitadas à área visível na restauração.

### Library Workspace

Inicial 1280×800, mínimo 960×640. Topbar 56; sidebar 200; painel central flex ≥480; inspector direito 280 opcional; now-playing inferior 96. Sidebar: Biblioteca, Pastas, Playlists, Rádio, Podcasts, Favoritos, Downloads, Temas/Plugins. Search na topbar, breadcrumb e filtros no centro. Conteúdo alterna grid de álbuns, lista de faixas ou pastas. Inspector: capa, créditos, tags, letras e cadeia de áudio. Em largura <1152, inspector vira drawer; sidebar pode colapsar a 56.

### Mini

420×112 mínimo; título/tempo, play e próxima, volume/mute, expandir; seek de 8 DIP com área de interação24. Always-on-top opcional e indicador claro. Preferência “close to tray” desligada por padrão; se ligada, primeiro fechamento informa onde reabrir.

## Telas adicionais

Preferências 880×680 com navegação lateral: Reprodução, Saída, DSP, Biblioteca, Streaming, Aparência, Atalhos, Plugins, Privacidade, Avançado. EQ avançado 960×600 com seletor Gráfico 10/20/31/64/Paramétrico (até 32 filtros), gráfico, tabela de bandas e preset por dispositivo; no nativo, 64 aparece como "indisponível nesta versão (M5)" até TASK-E12-10, e no MVP HTML/SVG/JS os cinco modos são interativos ([equalizadores](../audio/08-equalizers.md), [painel DSP](05-dsp-panel.md)). Detalhes da faixa 800×640: informações/tags/capa/letras/capítulos. Gerenciador de jobs 800×520: progresso, erro, cancelar/retomar. Rádio: estações e URL; podcasts: feeds/episódios; temas: gallery/preview/instalação; plugins: status/permissões/versão.

## Estados de tela obrigatórios

Empty com CTA; Loading cancelável; Populated; Partial error com itens válidos mantidos; Offline sem apagar local; Unsupported com motivo; Permission denied; Conflict; Recovery. Dialogs focam primeira ação segura, Escape cancela, retorno de foco ao invocador. Não embutir stacktrace ou nome de crate no fluxo normal; detalhes técnicos em painel opcional.

## Letras sob o VU (2026-10-28)

Posição do componente resumido de letras (LYR-01..LYR-06, LYR-15); regras completas em [Letras: componente resumido sob o VU e fontes](../library/02-tags-lyrics-tools.md#letras-componente-resumido-sob-o-vu-e-fontes). Estado: especificado; protótipo externo relatado (revisão 14, não verificado aqui); nativo: nada.

- **Onde:** imediatamente abaixo do VU, no layout estendido aprovado de referência (os 16 VUs estéreo e seus [acabamentos](05-dsp-panel.md#61-integração-visual-dos-vu-meters-vu-int)), integrado à superfície da skin, sem moldura ou caixa adicional (LYR-01). Os layouts em DIP acima ainda não definem a região do VU estendido; a geometria exata do componente fica a definir com ela, e nenhuma coordenada é fixada aqui.
- **Três linhas:** anterior, atual (central) e próxima, uma linha visual cada, altura a partir de tokens e da escala de texto; referência do protótipo (relatada): 26 DIP no desktop, 30 DIP em tela estreita (LYR-02, LYR-03).
- **Sem reduzir o VU:** o componente adapta o espaço sem diminuir excessivamente o VU, distorcer suas proporções ou encobrir controles de reprodução; deve ser testado com texto ampliado e layouts compactos (LYR-AC-10).
- **Classic Stack e Library Workspace:** exibido onde o VU estendido for exibido; no Library Workspace, o inspector já lista "letras" e continua sendo o acesso ao texto completo.
- **Mini:** a regra de três linhas vale onde o componente é exibido; não é imposto no modo Mini quando faltar espaço (acesso à letra completa fica no painel).
- **Texto longo:** nenhuma quebra de grid nem rolagem horizontal da página; alternativa de leitura completa (LYR-03).
- **Independência:** ocultar a letra não oculta nem redimensiona o instrumento; trocar o visualizador não apaga a preferência de letra (LYR-15).
