# Glossário e convenções

Versão 0.7.0.

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

IDs são estáveis e não mudam com renomeação de arquivo. “Alvo”, “gate” e “proposta” nunca significam resultado já medido. Dados nos previews são fictícios. O nome Vynamp é provisório.
