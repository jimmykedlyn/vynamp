# TASK-E10-01 — Criar corpus de testes licenciado

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. Critério de aceite: **PENDENTE** de autoria —
> a coluna `Then` é o requisito aprovado, copiado literal; `Given` e `When` ainda
> não existem.

| Campo | Valor |
|---|---|
| ID | `TASK-E10-01` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E10-01` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M0 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E11-01` |
| Bloqueia | `TASK-E10-02`, `TASK-E10-03`, `TASK-E10-04`, `TASK-E10-05` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Fixtures geradas de tons/sweep/gapless e corrupções, com manifests de hashes e licenças.

## Boundary

<!-- PENDENTE: o que esta TASK não faz, mesmo parecendo relacionado. -->

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e10-01-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E10-01-AC-1` | as fixtures geradas e o manifesto com o hash de cada arquivo | o job de CI confere o manifesto contra os arquivos versionados | CI valida hashes |
| `TASK-E10-01-AC-2` | cada fixture do corpus com sua entrada no manifesto | o oráculo de frames, tags e ranges é escrito por fixture | oráculos de frames/tags/ranges definidos |
| `TASK-E10-01-AC-3` | a suíte que consome o corpus | todo stream remoto é procurado na suíte e o CI roda sem rede | testes não dependem de rádios públicas instáveis. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | PENDENTE | comparação numérica, não "parece certo" |
| Hardware | PENDENTE | ausência é limitação registrada, não aprovação |

## DoD — pronto para review

- [ ] Todos os critérios de aceite com evidência
- [ ] `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` limpos
- [ ] `cargo test` verde; testes ignorados listados com motivo
- [ ] Spec do módulo atualizada se a implementação revelou algo novo
- [ ] `backlog.json` com o estado correto
- [ ] Revisão feita por contexto limpo, não pelo implementador
- [ ] PR aberto com template preenchido; CI Gate verde no SHA final
## Notas de execução

Fechada em 2026-11-09 por medição contra o corpus que está no repositório, não por
leitura da spec. O corpus vive em `libs/decode/tests/fixtures/` e o manifesto é
conferido pela própria suíte, não por um job separado.

| AC | Verificação executada | Resultado |
|---|---|---|
| AC-1 | `cargo test --package vynamp-decode --test oracle_pcm --locked` com um byte de `tests/fixtures/pcm/sine-s16le.wav` invertido, depois `git checkout --` no arquivo | **exit 101**, `manifest_files_decode_to_the_source` panica em `oracle_pcm.rs:144` com o sha256 esperado contra o adulterado. Restaurado, `git status` limpo. Sem esse teste, o hash no manifesto seria decoração |
| AC-2 | `oracle_pcm.rs`, `oracle_flac.rs`, `oracle_mpeg.rs` | cada linha do manifesto é lida e confere três oráculos por fixture: **sha256 do arquivo**, **sha256 da referência** (`mpeg`), e a **decodificação contra o WAV/f32 de referência**. `oracle_mpeg.rs:143` ainda recusa qualquer arquivo no diretório que o manifesto não nomeie — corpus e manifesto não podem divergir em silêncio |
| AC-3 | `cargo test --workspace --locked --offline` com `HTTP_PROXY`/`HTTPS_PROXY`/`ALL_PROXY` apontados para `127.0.0.1:9` (porta morta) | **47 suítes `ok`, 0 falhas, exit 0**. Nenhuma URL `http(s)` em `crates/*/tests/` nem em `tools/`. O único caminho que busca arquivos remotos é `conformance_flac.rs`, que só o faz com `VYNAMP_FLAC_TEST_FILES` explícito e cai no corpus versionado sem ela |

**Onde está a verificação de hash, e onde não está.** O `Then` do AC-1 diz "o job de
CI confere o manifesto". Medido: **nenhum job faz isso**. `grep -nE
"manifest|hash|sha256"` em `libs/decode/.github/workflows/ci.yml` não retorna nada, e
o job equivalente do umbrella só usa `--manifest-path` do Cargo. Quem confere é
`cargo test`, que roda no CI do decode. O comportamento exigido está provado — um
manifesto que não confere quebra o build — mas por um teste e não por um passo de
workflow. Registro isso porque o critério nomeia "job", e a leitura literal dele é
falsa.

**Cobertura do corpus, que o deliverable pede e o critério não enumera.**
`pcm/` 13 arquivos gerados por `tools/fixtures/generate.ps1` com FFmpeg (u8..f64,
6 canais, RF64, W64, AIFF, AIFC); `flac/` 25 arquivos mais 9 referências WAV,
cobrindo `cholesky`/`levinson`/`fixed`/`leftside`/`rightside`/`midside`, níveis 0/5/8/12
e blocksize 192/4096/16384; `flac-conformance/` 19 subset + 11 faulty do repositório
`ietf-wg-cellar/flac-test-files` em `aa7b0c6` (CC0 1.0, `SOURCE.txt` versionado);
`mpeg/` 67 arquivos Layer II/III com a referência `.f32` de cada um.

**O que esta TASK não provou:** nenhuma fixture de *sweep* ou de par *gapless*
existe ainda — o corpus cobre tons e variantes de codec, não os pares gapless que o
deliverable pede. Não há corrupção gerada (só as `faulty/` de FLAC, que vêm de
terceiro). Nenhum áudio foi tocado; nenhuma flag de prontidão mudou.
