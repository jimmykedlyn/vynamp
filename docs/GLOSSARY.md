# Glossário e convenções

Versão 0.7.1 · 2026-10-25 (termos DSP adicionados).

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
| EQ-10/20/31/64, EQ-PEQ32 | IDs das cinco modalidades de [equalizadores](audio/08-equalizers.md) |
| Q / 1/3 de oitava | Seletividade do filtro (centro/largura) / série de centros espaçados por 2^(1/3) (ISO 266); 64 bandas **não** é 1/3 de oitava |
| Nyquist / 0,45×Fs | Metade da taxa de amostragem / limite superior prático do editor de EQ |
| Oversampling | Processar em taxa multiplicada para reduzir aliasing de não linearidades ou medir true peak |
| Wow / flutter | Flutuação lenta / rápida de velocidade e afinação simulada pelo vitrola |
| vitrola | Biblioteca própria de simulação de vinil e amplificador valvulado hospedada no DSP ([spec](audio/10-vitrola.md)) |
| vynamp-decode / vynamp-output | Bibliotecas próprias de decodificação / saída de áudio; a saída nunca aplica ganho, dither ou resampling |
| MVP HTML/SVG/JS | Protótipo interativo de UI; `[DEMO]` marca o que não processa áudio; não é a engine nativa |

IDs são estáveis e não mudam com renomeação de arquivo. “Alvo”, “gate” e “proposta” nunca significam resultado já medido. Dados nos previews são fictícios. O nome Vynamp é provisório.
