# Ordem de execução das TASKs

Versão: 0.1.0 · Data: 2026-11-13 · Estado: **gerado** a partir de
[`backlog.json`](backlog.json) por `cargo xtask docs-sync`.

Este arquivo não se edita à mão. A ordem é a topológica das dependências do
backlog: uma TASK entra na onda `n` quando todas as suas dependências estão
em ondas anteriores. A onda diz o que **pode** começar, não o que **deve**
começar primeiro -- dentro de uma onda a ordem é fase, prioridade e ID, e a
escolha entre elas é do mantenedor. O status de cada uma vive no backlog, e
`cargo xtask check-docs` reprova este arquivo quando ele diverge de lá.

15 TASKs entregues, 200 planejadas, 25 ondas.

## Onda 1 — 19 TASK(s)

| ID | Título | Módulo | Fase | Prio | Depende de |
|---|---|---|---|---|---|
| `TASK-E02-01` | Validar toolkit Rust com UI clássica acessível | `vynamp-player` | M0 | P0 | `TASK-E11-01` |
| `TASK-E02-12` | Gerar gabarito visual e estratégia de fontes | `vynamp-player` | M0 | P0 | `TASK-E02-11` |
| `TASK-E02-13` | Gerar gabarito de movimento, medidores e ondas | `vynamp-player` | M0 | P0 | `TASK-E02-11` |
| `TASK-E02-14` | Gerar gabarito de comportamento e preferências | `vynamp-player` | M0 | P0 | `TASK-E02-11` |
| `TASK-E07-01` | Provar sandbox WASM e capabilities | `vynamp-player` | M0 | P0 | `TASK-E11-01` |
| `TASK-E10-01` | Criar corpus de testes licenciado | `vynamp` | M0 | P0 | `TASK-E11-01` |
| `TASK-E11-02` | Definir fronteiras de segurança | `vynamp` | M0 | P0 | `TASK-E11-01` |
| `TASK-E13-01` | Provar Android arm64, bridge Rust e output | `vynamp-player` | M0 | P0 | `TASK-E11-01`, `TASK-E01-01`, `TASK-E01-02` |
| `TASK-E14-01` | Provar os cinco targets e features portáveis | `vynamp` | M0 | P0 | `TASK-E11-01` |
| `TASK-E15-01` | Configurar Claude Code e Superpowers obrigatório | `vynamp` | M0 | P0 | — |
| `TASK-E16-01` | Provar envelope de ativação e protocolo comum | `vynamp-player` | M0 | P0 | `TASK-E11-01` |
| `TASK-E01-03` | Implementar state machine e generation cancellation | `vynamp-engine` | M1 | P0 | `TASK-E01-01`, `TASK-E01-02` |
| `TASK-E03-01` | Implementar schema SQLite e migrations seguras | `vynamp-player` | M1 | P0 | `TASK-E11-01` |
| `TASK-E15-04` | Ativar workflows e CI Gate multiplataforma | `vynamp` | M1 | P0 | `TASK-E15-03` |
| `TASK-E01-05` | Implementar mixer, drain e transições básicas | `vynamp-engine` | M1 | P1 | `TASK-E01-04` |
| `TASK-E01-07` | Adicionar codecs modernos e capítulos | `vynamp-decode` | M2 | P1 | `TASK-E01-04` |
| `TASK-E21-01` | Entregar registry de encoders e disponibilidade por build | `vynamp-encode` | M3 | P0 | — |
| `TASK-E04-05` | Entregar saída exclusiva e device profiles | `vynamp-output` | M3 | P1 | `TASK-E01-02`, `TASK-E04-02` |
| `TASK-E20-01` | Provar iOS arm64, bridge Rust e CoreAudio | `vynamp-player` | F6 | P0 | `TASK-E11-01`, `TASK-E01-01`, `TASK-E01-02` |

## Onda 2 — 29 TASK(s)

| ID | Título | Módulo | Fase | Prio | Depende de |
|---|---|---|---|---|---|
| `TASK-E02-15` | Mapear lacunas web para nativo por plataforma | `vynamp-player` | M0 | P0 | `TASK-E02-12`, `TASK-E02-13`, `TASK-E02-14` |
| `TASK-E15-02` | Validar worktrees, claims e handoff | `vynamp` | M0 | P0 | `TASK-E15-01` |
| `TASK-E17-01` | Fixar catálogo por binding, capacidades e corpus | `vynamp-decode` | M0 | P0 | `TASK-E11-01`, `TASK-E14-01` |
| `TASK-E02-02` | Implementar tokens e componentes semânticos | `vynamp-player` | M1 | P0 | `TASK-E02-01`, `TASK-E02-11` |
| `TASK-E03-02` | Implementar queue independente e shuffle | `vynamp-player` | M1 | P0 | `TASK-E03-01`, `TASK-E01-03` |
| `TASK-E04-01` | Implementar EQ10, ganho e ReplayGain playback | `vynamp-engine` | M1 | P0 | `TASK-E01-05` |
| `TASK-E13-02` | Entregar serviço e MediaSession com engine Rust | `vynamp-player` | M1 | P0 | `TASK-E13-01`, `TASK-E01-03` |
| `TASK-E14-02` | Entregar ports de arquivos, credenciais e sistema | `vynamp` | M1 | P0 | `TASK-E14-01`, `TASK-E01-03` |
| `TASK-E15-05` | Integrar testes e evidência operacional | `vynamp` | M1 | P0 | `TASK-E11-01`, `TASK-E15-04` |
| `TASK-E16-02` | Implementar owner único por perfil e IPC autorizado | `vynamp-player` | M1 | P0 | `TASK-E16-01`, `TASK-E01-03` |
| `TASK-E11-04` | Completar CI e supply chain | `vynamp` | M1 | P1 | `TASK-E11-01`, `TASK-E11-03`, `TASK-E14-01`, `TASK-E15-04` |
| `TASK-E03-03` | Implementar import incremental e roots offline | `vynamp-player` | M2 | P0 | `TASK-E03-01`, `TASK-E01-07` |
| `TASK-E05-01` | Implementar SourceDescriptor e provider simulado | `vynamp-player` | M2 | P0 | `TASK-E01-03` |
| `TASK-E08-01` | Implementar jobs persistidos | `vynamp-engine` | M2 | P0 | `TASK-E03-01` |
| `TASK-E17-02` | Implementar trim, gain e timeline por codec | `vynamp-decode` | M2 | P0 | `TASK-E01-07` |
| `TASK-E01-06` | Implementar gapless sample-accurate | `vynamp-engine` | M2 | P1 | `TASK-E01-05` |
| `TASK-E11-05` | Implementar config, logs e keychain | `vynamp` | M2 | P1 | `TASK-E11-02`, `TASK-E03-01` |
| `TASK-E21-03` | Declarar priming e padding do encoder no contêiner | `vynamp-encode` | M3 | P0 | `TASK-E21-01`, `TASK-E01-07` |
| `TASK-E21-10` | Validar o plano contra a matriz de formatos | `vynamp-encode` | M3 | P0 | `TASK-E21-01` |
| `TASK-E01-08` | Completar codecs desktop e large files | `vynamp-decode` | M3 | P1 | `TASK-E01-07` |
| `TASK-E21-02` | Recusar MP3 ausente com erro tipado e zero byte escrito | `vynamp-encode` | M3 | P1 | `TASK-E21-01`, `TASK-E11-03` |
| `TASK-E21-04` | Recusar resample, remix e mudança de profundidade implícitos | `vynamp-encode` | M3 | P1 | `TASK-E21-01` |
| `TASK-E21-06` | Interromper a sessão por `CancelToken` sem escrever mais nada | `vynamp-encode` | M3 | P1 | `TASK-E21-01` |
| `TASK-E21-07` | Devolver erro de I/O e disco cheio sem pânico | `vynamp-encode` | M3 | P1 | `TASK-E21-01` |
| `TASK-E21-09` | Rotular toda saída lossy como lossy no resumo | `vynamp-encode` | M3 | P1 | `TASK-E21-01` |
| `TASK-E21-15` | Ser a fonte única do rótulo lossy e da extensão | `vynamp-encode` | M3 | P1 | `TASK-E21-01` |
| `TASK-E21-08` | Reembutir metadados e relatar o que foi descartado | `vynamp-encode` | M3 | P2 | `TASK-E21-01` |
| `TASK-E21-16` | Não oferecer saída DSD | `vynamp-encode` | M3 | P2 | `TASK-E21-01` |
| `TASK-E20-02` | Entregar AVAudioSession e playback em background | `vynamp-player` | F6 | P0 | `TASK-E20-01`, `TASK-E01-03` |

## Onda 3 — 25 TASK(s)

| ID | Título | Módulo | Fase | Prio | Depende de |
|---|---|---|---|---|---|
| `TASK-E02-03` | Construir Classic Stack e transporte | `vynamp-player` | M1 | P0 | `TASK-E02-02`, `TASK-E01-03` |
| `TASK-E02-16` | Implementar harness de fidelidade com egui_kittest | `vynamp-player` | M1 | P0 | `TASK-E02-15`, `TASK-E02-01` |
| `TASK-E10-02` | Testar domínio e propriedades de fila | `vynamp` | M1 | P0 | `TASK-E01-03`, `TASK-E03-02`, `TASK-E10-01` |
| `TASK-E13-03` | Entregar audio focus e rotas Android | `vynamp-player` | M1 | P0 | `TASK-E13-02`, `TASK-E01-05` |
| `TASK-E13-05` | Entregar interface phone e navegação acessível | `vynamp-player` | M1 | P0 | `TASK-E13-02`, `TASK-E02-02` |
| `TASK-E04-03` | Construir grafo DSP e transições sem glitches | `vynamp-engine` | M2 | P0 | `TASK-E04-01`, `TASK-E04-02` |
| `TASK-E05-02` | Entregar HTTP Range e reprodução progressiva | `vynamp-player` | M2 | P0 | `TASK-E05-01`, `TASK-E01-07` |
| `TASK-E06-01` | Definir schema e validador de temas | `vynamp-player` | M2 | P0 | `TASK-E02-02` |
| `TASK-E13-04` | Entregar MediaStore, SAF e permissões | `vynamp-player` | M2 | P0 | `TASK-E13-02`, `TASK-E03-03` |
| `TASK-E16-09` | Gerar declarações OS a partir do build verificado | `vynamp-player` | M2 | P0 | `TASK-E17-01`, `TASK-E16-01` |
| `TASK-E17-11` | Implementar política de proxy e rede isolada | `vynamp-player` | M2 | P0 | `TASK-E05-01` |
| `TASK-E03-04` | Implementar leitura de tags e capas segura | `vynamp-player` | M2 | P1 | `TASK-E03-03` |
| `TASK-E08-02` | Entregar conversor em lote e validação pós-escrita | `vynamp-converter` | M3 | P0 | `TASK-E08-01`, `TASK-E01-07`, `TASK-E11-03` |
| `TASK-E08-03` | Entregar análise de loudness, ReplayGain e BPM | `vynamp-player` | M3 | P0 | `TASK-E08-01`, `TASK-E04-01` |
| `TASK-E17-03` | Completar bindings de containers e seleção de áudio | `vynamp-decode` | M3 | P0 | `TASK-E01-08` |
| `TASK-E17-04` | Avaliar e integrar xHE-AAC com capacidade honesta | `vynamp-decode` | M3 | P0 | `TASK-E17-01`, `TASK-E01-07` |
| `TASK-E17-05` | Certificar lossless, híbridos e PCM especiais | `vynamp-decode` | M3 | P0 | `TASK-E01-08` |
| `TASK-E01-09` | Entregar channel mapping e DSD para PCM | `vynamp-decode` | M3 | P1 | `TASK-E01-08`, `TASK-E04-02` |
| `TASK-E08-05` | Entregar sleep timer e scheduler | `vynamp-player` | M3 | P1 | `TASK-E08-01`, `TASK-E01-03` |
| `TASK-E11-06` | Implementar updater assinado | `vynamp` | M3 | P1 | `TASK-E11-04`, `TASK-E11-05` |
| `TASK-E21-11` | Exigir confirmação na redução de profundidade | `vynamp-encode` | M3 | P1 | `TASK-E21-10` |
| `TASK-E21-12` | Resample só com confirmação e resampler registrado | `vynamp-encode` | M3 | P1 | `TASK-E21-10` |
| `TASK-E21-13` | Exigir ganho declarado na troca mono e estéreo | `vynamp-encode` | M3 | P1 | `TASK-E21-10` |
| `TASK-E21-14` | Impedir que WAV float 32 vire inteiro sem pedido | `vynamp-encode` | M3 | P1 | `TASK-E21-10` |
| `TASK-E20-03` | Entregar shell SwiftUI touch-first e acessibilidade | `vynamp-player` | F6 | P0 | `TASK-E20-01`, `TASK-E02-02` |

## Onda 4 — 23 TASK(s)

| ID | Título | Módulo | Fase | Prio | Depende de |
|---|---|---|---|---|---|
| `TASK-E02-04` | Entregar playlist UI e keyboard workflow | `vynamp-player` | M1 | P1 | `TASK-E02-03`, `TASK-E03-02` |
| `TASK-E06-02` | Entregar instalação atômica de temas | `vynamp-player` | M2 | P0 | `TASK-E06-01`, `TASK-E11-02` |
| `TASK-E10-03` | Criar harness numérico de áudio | `vynamp` | M2 | P0 | `TASK-E04-03`, `TASK-E10-01` |
| `TASK-E13-07` | Entregar recovery de processo e controle de energia | `vynamp-player` | M2 | P0 | `TASK-E13-03`, `TASK-E03-02` |
| `TASK-E14-03` | Entregar paridade de presets e schema de temas | `vynamp` | M2 | P0 | `TASK-E14-02`, `TASK-E06-01` |
| `TASK-E16-04` | Entregar MIME, extensões de shell e abertura externa no Linux | `vynamp-player` | M2 | P0 | `TASK-E16-02`, `TASK-E16-09`, `TASK-E14-02` |
| `TASK-E16-05` | Entregar document types, Quick Actions e Finder nos dois Macs | `vynamp-player` | M2 | P0 | `TASK-E16-02`, `TASK-E16-09`, `TASK-E14-02` |
| `TASK-E16-06` | Entregar VIEW e Share seguros no Android | `vynamp-player` | M2 | P0 | `TASK-E16-01`, `TASK-E16-09`, `TASK-E13-04`, `TASK-E13-05` |
| `TASK-E03-05` | Entregar FTS, filtros e views de biblioteca | `vynamp-player` | M2 | P1 | `TASK-E03-04` |
| `TASK-E03-06` | Entregar playlists import/export e CUE | `vynamp-player` | M2 | P1 | `TASK-E03-02`, `TASK-E03-04` |
| `TASK-E05-04` | Entregar buffers, cache e downloads | `vynamp-player` | M2 | P1 | `TASK-E05-02` |
| `TASK-E06-05` | Entregar meters e espectro | `vynamp-player` | M2 | P1 | `TASK-E01-05`, `TASK-E02-03` |
| `TASK-E17-12` | Publicar manifest real de capacidades por artefato | `vynamp` | M3 | P0 | `TASK-E17-01`, `TASK-E16-09`, `TASK-E15-04` |
| `TASK-E18-02` | Entregar Vynamp Converter e a lib de encoders | `vynamp-converter` | M3 | P0 | `TASK-E08-01`, `TASK-E01-07`, `TASK-E08-02`, `TASK-E11-03` |
| `TASK-E19-03` | Dissolver o vitrola no DSP interno do app | `vynamp-engine` | M3 | P0 | `TASK-E04-03` |
| `TASK-E03-08` | Implementar escrita de tags e capítulos em lote | `vynamp-player` | M3 | P1 | `TASK-E03-04`, `TASK-E08-01` |
| `TASK-E03-09` | Entregar letras, bookmarks e histórico | `vynamp-player` | M3 | P1 | `TASK-E03-04`, `TASK-E03-02` |
| `TASK-E04-04` | Entregar EQ paramétrico e presets | `vynamp-engine` | M3 | P1 | `TASK-E04-03` |
| `TASK-E04-06` | Entregar modo transparente e diagnósticos hi-res | `vynamp-output` | M3 | P1 | `TASK-E04-05`, `TASK-E04-03` |
| `TASK-E17-13` | Expandir MPEG-H com renderer e licença | `vynamp-decode` | M5 | P1 | `TASK-E17-01`, `TASK-E17-03` |
| `TASK-E17-14` | Expandir AC-4 e apresentações de áudio | `vynamp-decode` | M5 | P1 | `TASK-E17-01`, `TASK-E17-03` |
| `TASK-E17-15` | Expandir IAMF como modelo e renderer | `vynamp-decode` | M5 | P1 | `TASK-E17-01`, `TASK-E17-03` |
| `TASK-E20-05` | Adaptar DSP e equalização ao layout e ao orçamento iOS | `vynamp-player` | F6 | P1 | `TASK-E20-03`, `TASK-E04-01` |

## Onda 5 — 15 TASK(s)

| ID | Título | Módulo | Fase | Prio | Depende de |
|---|---|---|---|---|---|
| `TASK-E02-05` | Construir Library Workspace e inspector | `vynamp-player` | M2 | P1 | `TASK-E02-04`, `TASK-E03-05` |
| `TASK-E05-05` | Entregar feeds e episódios de podcasts | `vynamp-player` | M2 | P1 | `TASK-E05-04`, `TASK-E03-04` |
| `TASK-E10-07` | Instrumentar e medir performance | `vynamp` | M2 | P1 | `TASK-E01-05`, `TASK-E03-05`, `TASK-E10-03` |
| `TASK-E07-02` | Definir pacote e SDK de plugins v1 | `vynamp-player` | M3 | P0 | `TASK-E07-01`, `TASK-E06-02` |
| `TASK-E17-06` | Certificar tags técnicas, capítulos e sidecars | `vynamp-player` | M3 | P0 | `TASK-E03-08` |
| `TASK-E19-07` | Integrar o módulo DSP vitrola no grafo do player | `vynamp-engine` | M3 | P0 | `TASK-E19-03`, `TASK-E04-03` |
| `TASK-E21-05` | Provar bit-exatidão dos formatos lossless | `vynamp-encode` | M3 | P0 | `TASK-E21-01`, `TASK-E10-03` |
| `TASK-E01-10` | Endurecer engine e documentação dos limites | `vynamp-engine` | M3 | P1 | `TASK-E01-06`, `TASK-E01-09`, `TASK-E04-06` |
| `TASK-E03-07` | Implementar smart playlists e templates | `vynamp-player` | M3 | P1 | `TASK-E03-05`, `TASK-E03-06` |
| `TASK-E04-07` | Entregar dynamics, tone e channel controls | `vynamp-engine` | M3 | P1 | `TASK-E04-04` |
| `TASK-E04-09` | Entregar perfis e headphone preset import | `vynamp-output` | M3 | P1 | `TASK-E04-04`, `TASK-E04-05` |
| `TASK-E05-06` | Entregar HLS sob demanda | `vynamp-player` | M3 | P1 | `TASK-E05-04`, `TASK-E01-07` |
| `TASK-E06-06` | Entregar visualizações avançadas | `vynamp-player` | M3 | P1 | `TASK-E06-05` |
| `TASK-E08-04` | Entregar organização de arquivos e dedupe | `vynamp-player` | M3 | P1 | `TASK-E08-01`, `TASK-E03-05` |
| `TASK-E12-06` | Entregar CD playback e ripping | `vynamp-decode` | M5 | P2 | `TASK-E08-02`, `TASK-E03-08` |

## Onda 6 — 12 TASK(s)

| ID | Título | Módulo | Fase | Prio | Depende de |
|---|---|---|---|---|---|
| `TASK-E02-06` | Entregar Mini, preferências e estados de erro | `vynamp-player` | M2 | P1 | `TASK-E02-05` |
| `TASK-E07-03` | Implementar broker de permissões | `vynamp-player` | M3 | P0 | `TASK-E07-02`, `TASK-E05-02` |
| `TASK-E12-01` | Entregar Linux nativo e MPRIS na 1.0 | `vynamp-player` | M3 | P0 | `TASK-E14-02`, `TASK-E02-05` |
| `TASK-E12-02` | Entregar macOS Intel e Apple Silicon nativos na 1.0 | `vynamp-player` | M3 | P0 | `TASK-E14-02`, `TASK-E02-05` |
| `TASK-E15-06` | Validar NSIS/portable e assinatura Windows | `vynamp` | M3 | P0 | `TASK-E01-10`, `TASK-E11-04` |
| `TASK-E17-07` | Fixar proteção true-peak depois do processamento | `vynamp-engine` | M3 | P0 | `TASK-E04-03`, `TASK-E04-07` |
| `TASK-E17-09` | Entregar presets e taps limitados de visualização | `vynamp-player` | M3 | P0 | `TASK-E06-06`, `TASK-E07-01` |
| `TASK-E03-10` | Completar recovery, export e large-library gate | `vynamp-player` | M3 | P1 | `TASK-E03-07`, `TASK-E03-08`, `TASK-E03-09` |
| `TASK-E04-08` | Entregar tempo, pitch e efeitos criativos | `vynamp-engine` | M3 | P1 | `TASK-E04-03`, `TASK-E04-07` |
| `TASK-E05-07` | Entregar HLS live, ABR e timeshift | `vynamp-player` | M3 | P1 | `TASK-E05-06` |
| `TASK-E07-05` | Implementar helper nativo isolado | `vynamp-player` | M3 | P1 | `TASK-E07-02`, `TASK-E11-02` |
| `TASK-E20-04` | Entregar acesso a arquivos e biblioteca sob sandbox | `vynamp-player` | F6 | P1 | `TASK-E20-02`, `TASK-E02-05` |

## Onda 7 — 18 TASK(s)

| ID | Título | Módulo | Fase | Prio | Depende de |
|---|---|---|---|---|---|
| `TASK-E05-03` | Entregar rádio Icecast/SHOUTcast e ICY | `vynamp-player` | M2 | P0 | `TASK-E05-02`, `TASK-E02-06` |
| `TASK-E06-03` | Implementar layouts declarativos e bindings | `vynamp-player` | M2 | P0 | `TASK-E06-01`, `TASK-E02-06` |
| `TASK-E08-06` | Integrar controles do Windows | `vynamp-player` | M2 | P1 | `TASK-E02-06`, `TASK-E01-03` |
| `TASK-E14-04` | Executar corpus e outputs por alvo | `vynamp` | M3 | P0 | `TASK-E12-01`, `TASK-E12-02`, `TASK-E13-07`, `TASK-E04-06` |
| `TASK-E15-07` | Validar AppImage/DEB e dependências Linux | `vynamp` | M3 | P0 | `TASK-E12-01`, `TASK-E11-04` |
| `TASK-E15-08` | Validar DMG/ZIP Universal2 e notarização | `vynamp` | M3 | P0 | `TASK-E12-02`, `TASK-E11-04` |
| `TASK-E17-08` | Medir teto no PCM final com oráculo independente | `vynamp` | M3 | P0 | `TASK-E17-07`, `TASK-E10-03` |
| `TASK-E17-19` | Expor capacidades Bluetooth e headroom por rota | `vynamp-output` | M3 | P0 | `TASK-E13-03`, `TASK-E14-02`, `TASK-E04-08` |
| `TASK-E02-07` | Integrar cadeia de áudio e EQ avançado na UI | `vynamp-player` | M3 | P1 | `TASK-E02-06`, `TASK-E04-04`, `TASK-E04-06` |
| `TASK-E02-08` | Entregar docking e persistência multi-monitor | `vynamp-player` | M3 | P1 | `TASK-E02-06` |
| `TASK-E04-10` | Validar qualidade numérica e hardware matrix | `vynamp` | M3 | P1 | `TASK-E04-06`, `TASK-E04-07`, `TASK-E04-08`, `TASK-E04-09` |
| `TASK-E07-04` | Implementar lifecycle e circuit breaker | `vynamp-player` | M3 | P1 | `TASK-E07-03` |
| `TASK-E08-08` | Entregar scrobblers e presence | `vynamp-player` | M3 | P1 | `TASK-E03-09`, `TASK-E07-03` |
| `TASK-E10-05` | Fuzz parsers, pacotes e RPC | `vynamp` | M3 | P1 | `TASK-E06-02`, `TASK-E07-03`, `TASK-E10-01` |
| `TASK-E11-07` | Entregar distribuição nativa por plataforma | `vynamp` | M3 | P1 | `TASK-E11-04`, `TASK-E07-05`, `TASK-E11-06` |
| `TASK-E12-03` | Validar ARM64 adicional Windows/Linux e helpers | `vynamp` | M5 | P0 | `TASK-E12-01`, `TASK-E12-02` |
| `TASK-E12-05` | Entregar trackers, MIDI e codecs raros | `vynamp-decode` | M5 | P2 | `TASK-E01-08`, `TASK-E07-05` |
| `TASK-E20-06` | Fechar assinatura, TestFlight e aceite iOS | `vynamp-player` | F6 | P0 | `TASK-E20-02`, `TASK-E20-03`, `TASK-E20-04`, `TASK-E20-05` |

## Onda 8 — 17 TASK(s)

| ID | Título | Módulo | Fase | Prio | Depende de |
|---|---|---|---|---|---|
| `TASK-E13-06` | Integrar streaming, podcasts e downloads Android | `vynamp-player` | M2 | P0 | `TASK-E13-03`, `TASK-E13-04`, `TASK-E05-03`, `TASK-E05-05` |
| `TASK-E16-03` | Entregar associação, menu de contexto e pacotes de ícones no Windows | `vynamp-player` | M2 | P0 | `TASK-E16-02`, `TASK-E16-09`, `TASK-E08-06` |
| `TASK-E06-04` | Produzir três temas originais | `vynamp-player` | M2 | P1 | `TASK-E06-03`, `TASK-E06-02` |
| `TASK-E10-04` | Criar servidor de streaming adversarial | `vynamp` | M2 | P1 | `TASK-E05-03`, `TASK-E10-01` |
| `TASK-E16-08` | Validar portable, backup e limpeza de integração | `vynamp-player` | M3 | P0 | `TASK-E16-02`, `TASK-E15-06`, `TASK-E15-07`, `TASK-E15-08` |
| `TASK-E17-10` | Certificar visualizers, fallback e energia mobile | `vynamp-player` | M3 | P0 | `TASK-E17-09`, `TASK-E13-05`, `TASK-E14-04` |
| `TASK-E02-09` | Completar localização e testes visuais | `vynamp-player` | M3 | P1 | `TASK-E02-07`, `TASK-E02-08` |
| `TASK-E05-08` | Entregar gravação de rádios | `vynamp-player` | M3 | P1 | `TASK-E05-03`, `TASK-E08-02` |
| `TASK-E07-06` | Entregar templates e plugins de exemplo | `vynamp-player` | M3 | P1 | `TASK-E07-04` |
| `TASK-E07-07` | Entregar gerenciador de plugins | `vynamp-player` | M3 | P1 | `TASK-E07-04`, `TASK-E07-05`, `TASK-E02-06` |
| `TASK-E08-07` | Entregar CLI e IPC de instância única | `vynamp-player` | M3 | P1 | `TASK-E01-03`, `TASK-E08-06` |
| `TASK-E10-08` | Executar soak e matriz de hardware | `vynamp` | M3 | P1 | `TASK-E04-10`, `TASK-E01-10`, `TASK-E10-07`, `TASK-E14-04` |
| `TASK-E10-09` | Validar upgrades, recovery e privacidade | `vynamp` | M3 | P1 | `TASK-E03-10`, `TASK-E11-06`, `TASK-E10-05` |
| `TASK-E17-16` | Certificar áudio cinematográfico e passthrough | `vynamp-decode` | M5 | P1 | `TASK-E17-03`, `TASK-E04-10` |
| `TASK-E17-17` | Expandir speech, AAC low-delay e legados | `vynamp-decode` | M5 | P1 | `TASK-E01-08`, `TASK-E12-05` |
| `TASK-E12-04` | Entregar DSD nativo/DoP e PCM 768 kHz | `vynamp-output` | M5 | P2 | `TASK-E04-10`, `TASK-E01-09` |
| `TASK-E12-10` | Completar EQ64 e escala avançada | `vynamp-engine` | M5 | P2 | `TASK-E04-10`, `TASK-E03-10`, `TASK-E12-03` |

## Onda 9 — 7 TASK(s)

| ID | Título | Módulo | Fase | Prio | Depende de |
|---|---|---|---|---|---|
| `TASK-E10-06` | Automatizar semântica e regressão visual | `vynamp` | M2 | P1 | `TASK-E02-06`, `TASK-E06-04` |
| `TASK-E16-07` | Certificar ativação e associações em cinco alvos | `vynamp-player` | M3 | P0 | `TASK-E16-03`, `TASK-E16-04`, `TASK-E16-05`, `TASK-E16-06` |
| `TASK-E18-01` | Entregar Vynamp Restore como ferramenta desktop de restauração | `vynamp-restore` | M3 | P0 | `TASK-E08-01`, `TASK-E01-07`, `TASK-E16-03`, `TASK-E16-04`, `TASK-E16-05` |
| `TASK-E05-09` | Endurecer rede e diagnóstico de streaming | `vynamp-player` | M3 | P1 | `TASK-E05-07`, `TASK-E05-08` |
| `TASK-E06-07` | Entregar editor e preview de temas | `vynamp-player` | M3 | P1 | `TASK-E06-04`, `TASK-E06-06` |
| `TASK-E07-08` | Entregar documentação e harness do SDK | `vynamp-player` | M3 | P1 | `TASK-E07-06`, `TASK-E07-07` |
| `TASK-E17-18` | Certificar expansão MIDI, trackers e DSD especial | `vynamp` | M5 | P1 | `TASK-E12-05`, `TASK-E12-04` |

## Onda 10 — 5 TASK(s)

| ID | Título | Módulo | Fase | Prio | Depende de |
|---|---|---|---|---|---|
| `TASK-E13-08` | Entregar temas e plugins mobile | `vynamp-player` | M3 | P0 | `TASK-E13-05`, `TASK-E06-04`, `TASK-E07-08` |
| `TASK-E17-20` | Fechar gate combinado de codecs, DSP e visualizers | `vynamp` | M3 | P0 | `TASK-E17-02`, `TASK-E17-03`, `TASK-E17-04`, `TASK-E17-05`, `TASK-E17-06`, `TASK-E17-08`, `TASK-E17-10`, `TASK-E17-11`, `TASK-E17-12`, `TASK-E17-19`, `TASK-E16-07` |
| `TASK-E02-10` | Validar experiência desktop ponta a ponta | `vynamp-player` | M3 | P1 | `TASK-E02-09`, `TASK-E06-07`, `TASK-E07-07` |
| `TASK-E06-08` | Entregar galeria local e recovery visual | `vynamp-player` | M3 | P1 | `TASK-E06-07` |
| `TASK-E07-10` | Auditar isolamento das extensões | `vynamp-player` | M3 | P1 | `TASK-E07-08`, `TASK-E10-05` |

## Onda 11 — 4 TASK(s)

| ID | Título | Módulo | Fase | Prio | Depende de |
|---|---|---|---|---|---|
| `TASK-E13-09` | Entregar tablet/foldable e distribuição Android | `vynamp-player` | M3 | P0 | `TASK-E13-06`, `TASK-E13-07`, `TASK-E13-08`, `TASK-E11-04` |
| `TASK-E06-10` | Validar fidelidade visual dos temas | `vynamp-player` | M3 | P1 | `TASK-E06-08`, `TASK-E02-09` |
| `TASK-E11-08` | Preparar documentação de lançamento | `vynamp` | M3 | P1 | `TASK-E11-07`, `TASK-E06-08`, `TASK-E07-08` |
| `TASK-E12-07` | Entregar VST3 e investigar DSP de terceiros | `vynamp-engine` | M5 | P2 | `TASK-E07-10`, `TASK-E04-10` |

## Onda 12 — 3 TASK(s)

| ID | Título | Módulo | Fase | Prio | Depende de |
|---|---|---|---|---|---|
| `TASK-E13-10` | Fechar aceite Android 1.0 em hardware | `vynamp-player` | M3 | P0 | `TASK-E13-09`, `TASK-E04-10`, `TASK-E05-09` |
| `TASK-E14-05` | Validar packaging e update em cada alvo | `vynamp` | M3 | P0 | `TASK-E11-07`, `TASK-E12-01`, `TASK-E12-02`, `TASK-E13-09` |
| `TASK-E15-09` | Validar APK/AAB, SDK/NDK e identidade | `vynamp` | M3 | P0 | `TASK-E13-09`, `TASK-E11-04` |

## Onda 13 — 1 TASK(s)

| ID | Título | Módulo | Fase | Prio | Depende de |
|---|---|---|---|---|---|
| `TASK-E14-06` | Fechar release 1.0 multiplataforma | `vynamp` | M3 | P0 | `TASK-E14-04`, `TASK-E14-05`, `TASK-E13-10`, `TASK-E10-08`, `TASK-E16-07`, `TASK-E16-08`, `TASK-E17-12`, `TASK-E17-20` |

## Onda 14 — 1 TASK(s)

| ID | Título | Módulo | Fase | Prio | Depende de |
|---|---|---|---|---|---|
| `TASK-E10-10` | Fechar gates e cobertura desktop | `vynamp` | M3 | P1 | `TASK-E10-08`, `TASK-E10-09`, `TASK-E02-10`, `TASK-E07-10`, `TASK-E14-06` |

## Onda 15 — 1 TASK(s)

| ID | Título | Módulo | Fase | Prio | Depende de |
|---|---|---|---|---|---|
| `TASK-E15-10` | Fechar release assinado e draft privado | `vynamp` | M3 | P0 | `TASK-E15-06`, `TASK-E15-07`, `TASK-E15-08`, `TASK-E15-09`, `TASK-E10-10` |

## Onda 16 — 1 TASK(s)

| ID | Título | Módulo | Fase | Prio | Depende de |
|---|---|---|---|---|---|
| `TASK-E11-09` | Preparar stable 1.0 e rollout beta | `vynamp` | M3 | P1 | `TASK-E11-08`, `TASK-E10-10`, `TASK-E15-10` |

## Onda 17 — 1 TASK(s)

| ID | Título | Módulo | Fase | Prio | Depende de |
|---|---|---|---|---|---|
| `TASK-E09-01` | Definir catálogo, direitos, escala e custos | `vynamp` | M4 | P0 | `TASK-E05-09`, `TASK-E11-09` |

## Onda 18 — 2 TASK(s)

| ID | Título | Módulo | Fase | Prio | Depende de |
|---|---|---|---|---|---|
| `TASK-E09-02` | Materializar OpenAPI v1 e contract tests | `vynamp` | M4 | P0 | `TASK-E09-01`, `TASK-E05-01` |
| `TASK-E11-10` | Preparar operação da plataforma | `vynamp` | M4 | P1 | `TASK-E09-01` |

## Onda 19 — 2 TASK(s)

| ID | Título | Módulo | Fase | Prio | Depende de |
|---|---|---|---|---|---|
| `TASK-E09-03` | Implementar identidade e autorização | `vynamp` | M4 | P0 | `TASK-E09-02` |
| `TASK-E05-10` | Integrar provider API, offline e direitos | `vynamp-player` | M4 | P1 | `TASK-E05-09`, `TASK-E09-02` |

## Onda 20 — 4 TASK(s)

| ID | Título | Módulo | Fase | Prio | Depende de |
|---|---|---|---|---|---|
| `TASK-E16-10` | Entregar deep links e callbacks do provider | `vynamp-player` | M4 | P0 | `TASK-E16-01`, `TASK-E09-03` |
| `TASK-E08-09` | Entregar audiobooks e progresso offline | `vynamp-player` | M4 | P1 | `TASK-E05-10`, `TASK-E03-09` |
| `TASK-E08-10` | Entregar controle remoto autenticado | `vynamp-player` | M4 | P1 | `TASK-E07-03`, `TASK-E08-07`, `TASK-E09-03` |
| `TASK-E09-04` | Implementar catálogo e administração editorial | `vynamp` | M4 | P1 | `TASK-E09-03` |

## Onda 21 — 3 TASK(s)

| ID | Título | Módulo | Fase | Prio | Depende de |
|---|---|---|---|---|---|
| `TASK-E09-05` | Implementar ingest e transcode isolados | `vynamp` | M4 | P1 | `TASK-E09-04`, `TASK-E11-03` |
| `TASK-E09-08` | Entregar sincronização de dados do ouvinte | `vynamp` | M4 | P1 | `TASK-E09-03`, `TASK-E09-04`, `TASK-E03-10` |
| `TASK-E09-09` | Entregar registry de temas e plugins | `vynamp` | M4 | P1 | `TASK-E09-04`, `TASK-E07-08`, `TASK-E06-08` |

## Onda 22 — 3 TASK(s)

| ID | Título | Módulo | Fase | Prio | Depende de |
|---|---|---|---|---|---|
| `TASK-E06-09` | Preparar publicação de temas | `vynamp-player` | M4 | P1 | `TASK-E06-08`, `TASK-E09-09` |
| `TASK-E07-09` | Preparar registry e revogação de plugins | `vynamp-player` | M4 | P1 | `TASK-E07-08`, `TASK-E09-09` |
| `TASK-E09-06` | Entregar streaming VOD e CDN | `vynamp` | M4 | P1 | `TASK-E09-05` |

## Onda 23 — 1 TASK(s)

| ID | Título | Módulo | Fase | Prio | Depende de |
|---|---|---|---|---|---|
| `TASK-E09-07` | Entregar ingest live e programação de rádio | `vynamp` | M4 | P1 | `TASK-E09-06` |

## Onda 24 — 1 TASK(s)

| ID | Título | Módulo | Fase | Prio | Depende de |
|---|---|---|---|---|---|
| `TASK-E09-10` | Validar operação, carga e recuperação | `vynamp` | M4 | P1 | `TASK-E09-06`, `TASK-E09-07`, `TASK-E09-08`, `TASK-E09-09`, `TASK-E11-10` |

## Onda 25 — 2 TASK(s)

| ID | Título | Módulo | Fase | Prio | Depende de |
|---|---|---|---|---|---|
| `TASK-E12-08` | Entregar DASH, WebDAV e clouds | `vynamp-player` | M5 | P2 | `TASK-E05-10`, `TASK-E09-10` |
| `TASK-E12-09` | Entregar casting e companion | `vynamp-player` | M5 | P2 | `TASK-E08-10`, `TASK-E09-10` |

