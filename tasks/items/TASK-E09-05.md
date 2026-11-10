# TASK-E09-05 — Implementar ingest e transcode isolados

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E09-05` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E09-05` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M4 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E09-04`, `TASK-E11-03` |
| Bloqueia | `TASK-E09-06` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 8–20 dias úteis |
| Risco | alto |

## Entrega

Upload assinado, quarantine/probe/rights, outbox e workers de renditions.

## Boundary

- **Não** define catálogo, perfis de rendition, direitos por região nem orçamento de banda. Esse RFC e `TASK-E09-01`, quatro dependências acima desta (`E09-01` -> `E09-02` -> `E09-03` -> `E09-04`); aqui se executa o perfil que ele aprovou, e escolher o perfil dentro do worker faria o pipeline decidir política de distribuição.
- **Não** entrega resolve, URL assinada de delivery nem CDN. Isso é `TASK-E09-06`; o master privado desta TASK é o objeto de armazenamento, não o caminho de reprodução, e servir bytes daqui criaria um segundo caminho de acesso sem expiração.
- **Não** implementa ingest live, packager HLS nem grade de programação. `TASK-E09-07` é dona do ingest SRT/RTMP e do HLS ao vivo; esta TASK é do pipeline de arquivo já completo, e reaproveitar o worker de VOD para live amarraria a latência do rádio a um job de transcode.
- **Não** implementa a API de catálogo, a busca nem a publicação editorial. `TASK-E09-04` é dona do Axum/PostgreSQL de catálogo e da UI de admin; o endpoint de upload assinado daqui é o único ponto de escrita de mídia, e trazer o CRUD de asset para cá duplicaria o schema.
- **Não** decide se o build de FFmpeg com esses encoders é distribuível. O inventário de licença e de feature é `TASK-E11-03`, que bloqueia esta TASK; o worker consome o binário cuja licença já foi registrada, e resolver licença aqui seria decidir distribuição dentro de um worker.
- **Não** roda o teste de 1.000 sessões nem o drill de restore do bucket. O teste de carga é `TASK-E09-10` e o drill com IaC, PITR e runbooks é `TASK-E11-10`; aqui a fila e os workers só precisam ser corretos e idempotentes, e medir escala nesta TASK daria baseline a partir de um pipeline sem perfil fechado.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e09-05-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E09-05-AC-1` | dois POST de upload assinado com o mesmo SHA-256 de um WAV de 48 kHz/24 bits de 60 s e o mesmo `profile_version=1`, e o worker de rendition reiniciado no meio do primeiro job com o lease expirado e a mensagem ainda na fila | o relay do outbox publica as duas mensagens e os workers consomem as duas, a segunda depois do restart, e o resumo do job é lido em `jobs` e em `renditions` | Jobs duplicados idempotentes |
| `TASK-E09-05-AC-2` | três entradas de um cliente não confiável: um corpo de 12 GiB anunciado em `Content-Length` contra o máximo de 10.737.418.240 bytes declarado em `size_bytes` do `UploadRequest`, um WAV renomeado para `.flac` com MIME declarado `audio/flac`, e uma URL de ingest externo que resolve para `127.0.0.1`, resolve para `169.254.169.254` e responde 302 para `file:///etc/passwd` | as três atravessam o endpoint de criação de asset e o probe de ingest, e a política de destino é aplicada antes de qualquer conexão de saída | limites de mídia e SSRF aplicados |
| `TASK-E09-05-AC-3` | um asset em `ready` cujo master lossless está no bucket privado, e um token de CDN de curta duração emitido para um ouvinte desse asset | o GET no object key do master é feito sem credencial, com a URL assinada de delivery e com a credencial interna do worker, e as três respostas são comparadas | master privado |
| `TASK-E09-05-AC-4` | três assets no mesmo pipeline: um com probe `failed`, um com rights validation `denied` e um com probe `verified` e rights `granted`, todos com a tabela de transições permitidas carregada | os workers concluem cada job e a transição para `ready` é tentada nos três, com o estado e o motivo da recusa gravados em `audit` | ready somente após validações. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | subir `postgres:18-alpine` (é a única imagem de banco no cache Docker deste ambiente; não há MinIO) e rodar `cargo test -p vynamp-platform-ingest --test ingest` com a fixture: dois uploads do mesmo SHA-256 e mesmo `profile_version=1`, corpo de 12 GiB acima do teto de 10.737.418.240 bytes, WAV renomeado para `.flac`, as três URLs de ingest externo (loopback, link-local, 302 para `file:///etc/passwd`) e três `curl -sI` no object key do master (anônimo, URL assinada de delivery, credencial do worker) | a evidência tem que ser: uma linha em `jobs` e o mesmo conjunto de linhas em `renditions` depois das duas mensagens duplicadas, garantido por chave única em hash mais `profile_version`; 413 para o corpo de 12 GiB, 415 para o WAV com MIME de FLAC e 422 com `ssrf_denied` para as três URLs, com zero conexão de saída no log de egress; 403 anônimo, 403 com a URL de delivery e 200 só com a credencial do worker; e `ready` inalcançável com `probe != verified` ou `rights != granted`. Nada disso existe hoje: o umbrella tem 168 arquivos `.rs` e nenhum é servidor — não há `axum`, `sqlx`, `tokio` nem `reqwest` em nenhum `Cargo.toml` —, `cargo test` na raiz roda só o pacote `xtask` (1.018 linhas em `xtask/tests/tooling.rs`), `tests/fixtures/` tem apenas `README.md`, o único exemplo que escreve arquivo de áudio (`gen_source_sine`, em `libs/decode/crates/vynamp-decode-testkit/examples/`) escreve 4.800 frames, 100 ms, e FFmpeg/ffprobe não estão no PATH deste Windows.
| Hardware | nenhum: as provas são de fila, HTTP e object storage e nenhum teste abre dispositivo de áudio; o que falta aqui é ferramenta, não hardware — FFmpeg/ffprobe não estão no PATH e não há MinIO nem imagem com FFmpeg no cache Docker local: as únicas imagens de banco e fila ali são `postgres:18-alpine` e `redis:8-alpine` | ausência é limitação registrada, não aprovação |

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
