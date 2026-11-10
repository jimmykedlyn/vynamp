# TASK-E09-07 — Entregar ingest live e programação de rádio

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E09-07` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E09-07` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M4 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E09-06` |
| Bloqueia | `TASK-E09-10` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 8–20 dias úteis |
| Risco | alto |

## Entrega

Ingest autenticado, packager HLS, grade, now-playing e failover.

## Boundary

- **Não** implementa o parser HLS, a janela live, o ABR por throughput nem o timeshift do cliente. Isso é `TASK-E05-07`, e o parser VOD é `TASK-E05-06`; aqui se publica playlist e segmentos, e implementar o cliente aqui faria o servidor depender do próprio player.
- **Não** entrega resolve VOD, URL assinada de rendition nem CDN de catálogo. Isso é `TASK-E09-06`, que bloqueia esta TASK; o HLS daqui é de rádio ao vivo, sem renditions por faixa, e reaproveitar a rota de resolve misturaria direito de catálogo com grade.
- **Não** implementa o CRUD de estação, a revisão editorial nem o audit de publicação. `TASK-E09-04` é dona do catálogo e do admin; a grade desta TASK consome a revisão que ele publica, e criar editor de estação aqui duplicaria o schema.
- **Não** roda o teste de 1.000 sessões nem mede SLO de startup ou custo de CDN. `TASK-E09-10` é dona do gate de carga; medir escala dentro do packager daria número sem baseline de operação.
- **Não** provisiona ingest redundante, alarme externo, runbook de failover nem IaC. Isso é `TASK-E11-10`; aqui só se prova que a falha de source gera alerta e filler, e o canal de escalação é dele.
- **Não** decide direito de retransmissão, região da rádio nem filler permitido por contrato. A decisão é de `TASK-E09-01`; esta TASK exige que o filler seja autorizado pelo operador e registra quem autorizou, sem fixar a política.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e09-07-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E09-07-AC-1` | um encoder publicando 48 kHz estéreo AAC-LC 128k por SRT autenticado na mesma máquina, e o packager configurado com segmentos de 4 s e janela live de 6 segmentos | três clientes independentes abrem a master playlist — `ffprobe` na URL, `hls.js` em Chromium headless e VLC — e cada um toca 60 s a partir do live edge sem configuração extra | Cliente HLS interoperável |
| `TASK-E09-07-AC-2` | uma estação no ar com o detector de silêncio em 2 s, um filler cujo uso foi autorizado pelo operador com registro de quem autorizou, e o encoder cai no meio do programa | o source para de publicar, o detector dispara e o packager continua escrevendo a mesma janela live durante 30 s, com o feed de now-playing e o canal de alerta observados | falha de source gera alerta/filler autorizado |
| `TASK-E09-07-AC-3` | duas estações com grade em `America/New_York`: uma com programa de 01:30 a 02:30 no dia da volta do horário de verão (2026-11-01) e outra no dia da entrada (2026-03-08), com o horário ambíguo de 01:30 com offset declarado na própria grade | a grade das duas estações é materializada em slots e a timeline de publicação do dia é montada e conferida contra o relógio de parede | grade considera DST |
| `TASK-E09-07-AC-4` | um packager com `EXT-X-PROGRAM-DATE-TIME` por segmento, dois reconnects do encoder que incrementam `EXT-X-DISCONTINUITY-SEQUENCE`, e um cliente que volta 30 s no DVR e retorna ao live edge | a timeline dos segmentos é exportada e o áudio de cada segmento é decodificado e comparado com o do segmento correspondente na janela anterior ao reconnect | timeline não duplica áudio. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | um encoder sintético `ffmpeg -re -f lavfi -i sine=frequency=1000:sample_rate=48000 -c:a aac -b:a 128k` publicando por SRT/RTMP em `localhost` (FFmpeg não está no PATH deste Windows), três clientes (`ffprobe`, `hls.js` em Chromium headless, VLC) e os dois dias de DST de `America/New_York` em 2026, mais um kill do processo do encoder no meio do programa | a evidência tem que ser: master só com tags RFC 8216, `#EXT-X-TARGETDURATION` entre 4 e 6, `#EXT-X-MEDIA-SEQUENCE` subindo de 1 em 1 e `#EXT-X-DISCONTINUITY-SEQUENCE` monotônica, e os três clientes abrindo a URL sem extensão nenhuma; na queda, um alerta com timestamp e a janela seguindo com filler marcado como autorizado, sem buraco na sequência; na grade, 2026-03-08 01:30 a 02:30 virando 30 min reais em 1 slot, e 2026-11-01 01:30 resolvido pelo offset declarado, sem dois slots com o mesmo ID; e na timeline, `EXT-X-PROGRAM-DATE-TIME` monotônico com 0 SHA-256 de segmento repetido na faixa do reconnect. Nada disso existe hoje: não há packager, nenhum `.m3u8` no repositório, nenhum código de ingest, e `tests/fixtures/` só tem `README.md` reservando `tests/fixtures/network/`; `cargo test` na raiz roda apenas `xtask`. Também faltam os três clientes do AC-1: VLC e `ffprobe` não estão instalados e `hls.js` não é dependência do MVP, que só traz `playwright@1.64.1`; o Chromium do Playwright está em `%LOCALAPPDATA%\ms-playwright\chromium-1243`, mas dois dos três clientes não abrem nada hoje.
| Hardware | nenhum: o source é um encoder sintético `lavfi` na própria máquina, então nenhum dispositivo de áudio, placa de captura ou aparelho móvel é aberto; a ausência é de instrumento — FFmpeg com SRT/RTMP e um servidor de ingest não estão instalados neste Windows; o único FFmpeg da máquina é o do Playwright, com `--disable-everything`, sem encoder AAC e sem protocolo SRT ou RTMP | ausência é limitação registrada, não aprovação |

## DoD — pronto para review

- [ ] Todos os critérios de aceite com evidência
- [ ] `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` limpos
- [ ] `cargo test` verde; testes ignorados listados com motivo
- [ ] Spec do módulo atualizada se a implementação revelou algo novo
- [ ] `backlog.json` com o estado correto
- [ ] Revisão feita por contexto limpo, não pelo implementador
- [ ] PR aberto com template preenchido; CI Gate verde no SHA final
## Notas de execução

Preenchido durante a execução, não antes.
