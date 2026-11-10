# TASK-E09-06 — Entregar streaming VOD e CDN

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E09-06` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E09-06` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M4 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E09-05` |
| Bloqueia | `TASK-E09-07`, `TASK-E09-10` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 8–20 dias úteis |
| Risco | alto |

## Entrega

Resolve assinado para FLAC/Opus progressivo e HLS AAC com direitos e expiração.

## Boundary

- **Não** transcodifica nem produz rendition. Master, quarantine, probe e workers são `TASK-E09-05`, que bloqueia esta TASK; aqui só se resolve o que já está em `ready`, e gerar HLS na rota de resolve poria transcode no caminho de latência do play.
- **Não** decide a matriz de renditions, os direitos por região nem o custo por ouvinte-hora. É `TASK-E09-01`; esta TASK obedece ao rights que ele fixou, e mudar perfil aqui mudaria um orçamento que ninguém renegociou.
- **Não** constrói o packager live, a grade nem o now-playing. `TASK-E09-07` é dona do HLS ao vivo e do failover; o HLS desta TASK é VOD com playlist finita, e misturar os dois faria o mesmo código servir janela deslizante e playlist fechada.
- **Não** implementa o parser HLS nem a negociação de qualidade do lado do cliente. Parser VOD é `TASK-E05-06` e janela live, ABR e timeshift são `TASK-E05-07`; aqui se emite a playlist, e assumir o parser do player faria o servidor depender de um cliente específico.
- **Não** implementa download offline nem a renovação de URL no player. Adapter de provider, offline e direitos no cliente são `TASK-E05-10`; a URL renovável daqui é contrato de servidor, e o retry do cliente é outra camada.
- **Não** define o schema OpenAPI de resolve, os enums de erro nem o mock provider. Contrato é `TASK-E09-02`; esta TASK implementa contra o contrato publicado, e inventar campo aqui faz o mock e a API divergirem no primeiro contract test.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e09-06-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E09-06-AC-1` | uma faixa com master FLAC 96 kHz/24 bits, rendition Opus 160k progressiva e HLS AAC-LC 128k e 256k já publicadas, e um cliente que declara qualidade `lossless` com `Accept: audio/flac` e pede `Range: bytes=65536-131071` | o resolve é chamado, o GET parcial é feito na URL devolvida e em seguida um segundo resolve é chamado com qualidade `economy` e `Accept: audio/ogg` para a mesma faixa | Range/qualidade negociados |
| `TASK-E09-06-AC-2` | uma URL de resolve com TTL de 300 s, consumida por uma sessão em reprodução aos 290 s, com o bearer da API guardado no cliente e nunca enviado à CDN | o cliente pede renovação com o mesmo `session_id` e o mesmo bearer enquanto o áudio continua, e continua o GET na URL nova depois dos 300 s, quando a URL antiga é reusada | token renovável |
| `TASK-E09-06-AC-3` | um asset com rights `granted` para a região BR, `denied` para US e download offline `denied`, e um segundo asset cujo direito é revogado com o token de sessão do listener ainda válido | o resolve é chamado com listener em BR, com listener em US, e outra vez com o listener de BR depois da revogação do direito, comparando corpo e código das três respostas | rights negados bloqueiam resolve |
| `TASK-E09-06-AC-4` | um asset hi-res cujo único perfil é FLAC lossless progressivo e a matriz de prova cliente/browser/CDN de HLS lossless vazia, que `streaming/03-plataforma.md` exige antes de servir HLS lossless | um cliente web que declara `Accept: application/vnd.apple.mpegurl` pede o resolve desse asset, e um segundo cliente com o mesmo Accept pede o resolve de um asset que tem AAC-LC HLS disponível | não forçar lossless HLS sem prova. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | origin local com as renditions de uma faixa (um FLAC 96/24 e um HLS AAC-LC com segmentos de 6 s) servida em `localhost`, mais quatro `curl`: `-H 'Range: bytes=65536-131071'` no FLAC, o mesmo Range no `.m3u8` e no `.ts`, `-H 'Accept: audio/flac'` contra `-H 'Accept: audio/ogg'`, e um GET com a URL antiga depois do TTL | a evidência tem que ser 206 com `Content-Range: bytes 65536-131071/<tamanho>` e 65.536 bytes idênticos ao `dd skip=65536 count=65536` do arquivo local, e zero 206 em `.m3u8`; o mesmo asset resolvido para três URLs distintas e não intercambiáveis, com taxa e codec batendo com o perfil pedido; 200 na renovação aos 290 s e 403 na URL antiga aos 301 s; 403 com `rights_denied` para o listener US e para o asset revogado, sem object key no corpo; e nenhuma variante HLS acima de 48 kHz, apesar do master de 96 kHz, e nenhum `.m3u8` lossless para o cliente web hi-res, só FLAC progressivo ou 422 com motivo. Nada disso existe hoje: não há endpoint nem origin — nenhum `Cargo.toml` do umbrella traz `axum` ou `reqwest`, `cargo test` na raiz roda só `xtask`, e não há `.m3u8` nem segmento no repositório; `tests/fixtures/` tem apenas `README.md`, que só reserva `tests/fixtures/network/`. O único FFmpeg presente na máquina é o do Playwright (`%LOCALAPPDATA%\ms-playwright\ffmpeg-1011\ffmpeg-win64.exe`), compilado com `--disable-everything`, sem encoder AAC, FLAC ou Opus, então nem os segmentos HLS nem o FLAC 96/24 saem dele.
| Hardware | nenhum: as provas são HTTP Range, TTL de URL e rights e nenhum teste abre dispositivo de áudio; falta instrumento, não hardware — FFmpeg/ffprobe não estão no PATH deste Windows para gerar os segmentos, e não há imagem de proxy nem de object storage no cache Docker local | ausência é limitação registrada, não aprovação |

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
