# Glossário e convenções

Versão 0.8.1 · 2026-11-07 (acabamento de VU passa a seis valores, contagem verificada no código do pin rev.38; termos de integração com o shell, pacotes de ícones e ferramentas auxiliares adicionados; termos de motor de restauração, defeito, severidade, região e projeto de restauração; termos de controles DSP, grafo modular e VU; termos de letras; pilha de áudio própria; MVP rev.38).

| Termo | Significado nesta spec |
|---|---|
| Frame | Um sample por canal no mesmo instante; não confundir com bytes ou pacote de codec |
| Sample rate | Frames por segundo; taxa negociada real pode diferir da origem |
| Valid bits | Bits significativos, separados do tamanho do container |
| PCM | Áudio já decodificado em amostras, inteiro ou float |
| Lossless / lossy | Sem perdas / com perdas em relação à entrada do encoder, não afirmação sobre master original |
| Hi-res source | Definição do produto: lossless PCM >16 bits e/ou >48 kHz |
| Transparente | Sem transformação no caminho definido; prova bit-perfect depende do conjunto testado |
| Gapless | Continuidade de frames com tratamento correto de delay/padding, não remover silêncio musical |
| Crossfade | Sobreposição intencional de faixas, diferente de gapless |
| ReplayGain | Metadados de ajuste de loudness e peak, não normalização destrutiva do arquivo |
| dBFS / dBTP / LUFS | Nível digital / true peak / loudness integrado; unidades não intercambiáveis |
| DIP | Unidade independente de DPI do sistema |
| SourceDescriptor | Contrato de origem/transporte/capacidades/restrições, separado da UI |
| Provider | Adapter de catálogo/resolve/auth; não codec nem saída de áudio |
| Generation | Identidade da execução de playback; cancela buffers/eventos antigos |
| Queue / playlist | Sequência ativa / coleção editável; não a mesma entidade |
| Capability | Operação suportada ou permissão concedida, sempre contextual |
| WASM / WIT | Sandbox de código / descrição de interfaces component model |
| VOD / live / DVR | Conteúdo sob demanda / ao vivo / janela de retrocesso |
| HLS / ICY | Streaming segmentado HTTP / metadata de rádio intercalada |
| P0/P1/P2 | Prioridade dentro da fase; não datas |
| M0–M5 | Provas, alpha, beta, 1.0, plataforma e expansão |
| ADR / RFC | Registro de decisão / proposta de mudança com alternativas e impacto |
| DSP / nó | Processamento digital de sinal / módulo do grafo com prepare, process, reset e latência declarada |
| Bypass | Nó desligado sem alterar o sinal (bypass real), com crossfade na transição |
| Smoothing | Interpolação de parâmetro (10–50 ms) para evitar zipper noise e cliques |
| Headroom | Margem até 0 dBFS (ou até o ceiling) antes de clipar |
| EQ gráfico / PEQ | Bandas de centro fixo com ganho / filtros com tipo, frequência, ganho, Q e canal livres |
| EQ-10/20/31/64, EQ-PEQ32 | IDs das cinco modalidades de [equalizadores](../reference/audio/08-equalizers.md) |
| Q / 1/3 de oitava | Seletividade do filtro (centro/largura) / série de centros espaçados por 2^(1/3) (ISO 266); 64 bandas **não** é 1/3 de oitava |
| Nyquist / 0,45×Fs | Metade da taxa de amostragem / limite superior prático do editor de EQ |
| Oversampling | Processar em taxa multiplicada para reduzir aliasing de não linearidades ou medir true peak |
| Wow / flutter | Flutuação lenta / rápida de velocidade e afinação simulada pelo vitrola |
| vitrola | Módulo DSP **interno** do app que simula vinil e amplificador valvulado; não é biblioteca nem repositório próprio ([spec](../reference/audio/10-vitrola.md), [ADR-014](../adr/ADR-014-publico-dsp-interno.md)) |
| vynamp-decode / vynamp-output | Bibliotecas próprias de decodificação / saída de áudio; a saída nunca aplica ganho, dither ou resampling ([ADR-011](../adr/ADR-011-pilha-propria.md)) |
| Lib própria de resampling | Futura biblioteca própria de conversão de taxa de amostragem (nome e repositório a definir); decisão D-DSP-08 |
| MVP HTML/SVG/JS | Protótipo interativo de UI, rev.38 em `vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452` (repositório privado, fora deste); `[DEMO]` marca o que não processa áudio; não é a engine nativa ([referência do MVP](../design/07-pin-mvp.md)) |
| Knob | Controle circular de parâmetro contínuo, ajustado por gesto vertical; ver [painel DSP](../design/06-painel-dsp.md#8-acesso-e-controles-dsp-ux) |
| Readout | Valor numérico com unidade junto ao knob ou slider; clicável para abrir o editor exato |
| Grafo modular | Vista de nós e conexões do DSP; no protótipo, referência relatada, no nativo, nada implementado ([painel DSP §9](../design/06-painel-dsp.md#9-fluxo-modular-grafo--referência-do-protótipo)) |
| Porta CV | Porta de controle que liga uma saída de modulação (ex.: LFO) à entrada de modulação de um nó; não carrega áudio nem MIDI |
| Acabamento de VU | Apresentação do gabinete externo dos VU meters, em seis valores, na ordem do seletor do MVP: Integrado · gabinete compacto (padrão), Sem moldura externa, Moldura ultrafina, Acabamento adaptativo à skin, Embutido · profundidade discreta e Original completo · sem recorte; não altera o SVG nem é efeito DSP ([VU-INT](../design/06-painel-dsp.md#61-integração-visual-dos-vu-meters-vu-int)) |
| Letra sincronizada | Letra com timestamps (LRC, SYLT em milissegundos) que permite escolher a linha atual pela posição de reprodução; texto sem tempos nunca é tratado assim ([letras](../library/02-tags-letras.md#letras-componente-resumido-sob-o-vu-e-fontes)) |
| LRC | Arquivo de letra com timestamps por linha, múltiplos tempos por linha e offset; pode ser importado explicitamente para uma faixa |
| Componente resumido de letra | Exibição de três linhas (anterior, atual, próxima) imediatamente abaixo do VU; não substitui o painel completo de letras |
| Fonte de letra | Origem da letra mostrada: LRC local importado, metadados embutidos ou DEMO; a ordem de escolha é LRC importado, depois metadados |
| Letra desligada × instrumental | Desligada é preferência global de exibição; instrumental/sem letra é marca reversível por faixa, nunca deduzida da ausência de tags |
| Offset de letra | Deslocamento temporal aplicado à seleção da linha (tag de offset do LRC e ajuste −/+100 ms do painel completo) |
| Sparse package (pacote esparso) | Identidade de pacote MSIX concedida a um executável Win32 instalado fora do contêiner; habilita o menu de contexto moderno do Windows 11 sem MSIX obrigatório ([ADR-012](../adr/ADR-012-integracao-shell.md)) |
| IExplorerCommand / IExplorerCommandEnumerator | Interfaces COM do shell moderno do Windows 11 que expõem um comando de menu de contexto e seu submenu enumerado |
| IShellItemArray | Coleção COM de itens selecionados, recebida em lote pelo comando de contexto; base do batching atômico (1/2/20/100 itens em uma única ativação) |
| ProgID / HKCU | Identificador de classe de arquivo registrada por usuário (`HKEY_CURRENT_USER\Software\Classes`); base do menu clássico "Mostrar mais opções" |
| ServiceMenu | Extensão de menu de contexto do KDE Dolphin instalada como arquivo `.desktop` em `~/.local/share/kio/servicemenus/` |
| Desktop Action | Ação de launcher declarada em `vynamp.desktop` (`Exec=... %U`); aparece no menu do launcher, não garante menu de contexto no gerenciador |
| Quick Actions | Menu de serviços do Finder no macOS, declarado pela chave `NSServices` do `Info.plist` |
| Pacote de ícones de arquivo | Conjunto de ícones por família (Documento, Disco, Sinal) aplicado aos 17 formatos de áudio; no Windows/Linux altera o shell, no macOS/Android só a UI interna |
| Vynamp Restore / Vynamp Converter | Ferramentas desktop executáveis separadas (`vynamp-restore`, `vynamp-converter`) para restauração acústica e transcodificação em lote ([ADR-012](../adr/ADR-012-integracao-shell.md)) |
| Motor de restauração | Biblioteca interna que executa a cadeia normativa de DSP de restauração; o app `vynamp-restore` é a casca e o motor é reaproveitável ([ADR-013](../adr/ADR-013-restore.md)) |
| Cadeia de restauração | Sequência fixa e auditável de 11 estágios DSP aplicada ao arquivo de origem; a ordem é normativa e não configurável ([spec do Restore](../reference/tools/01-vynamp-restore.md#3-cadeia-de-restauração)) |
| Defeito | Falha acústica detectada no sinal (clipe, zumbido, chiado, clique, trinca, rumble, offset DC, banda limitada), com severidade, evidência numérica e correção sugerida |
| Severidade | Classificação `mild` / `moderate` / `severe` de um defeito, atribuída pelo detector a partir da evidência medida |
| Região | Trecho do arquivo tratado de forma diferente (ganho e fade), com limites `start`/`end`; `kind: between` é o tipo suportado na 1.0 (`overlay` foi placeholder no MVP e está fora de escopo) |
| Projeto de restauração | Arquivo `.vynamp-restore` (JSON versionado) com cadeia, parâmetros, regiões e presets; guarda as decisões de um projeto, não o áudio processado |
| Saída restaurada | Arquivo gravado pelo Restore, sempre derivado e nunca substituto da origem; a escrita é atômica e validada após a gravação ([spec do Restore](../reference/tools/01-vynamp-restore.md#8-exportação)) |
| `vynamp-encode` | Biblioteca própria de codificadores, única fonte de encoders do produto; `std`-only, repositório privado, criada no PR5 ([spec do Converter](../reference/tools/02-vynamp-converter.md#1-escopo)) |

IDs são estáveis e não mudam com renomeação de arquivo. “Alvo”, “gate” e “proposta” nunca significam resultado já medido. Dados nos previews são fictícios. O nome Vynamp é provisório.
