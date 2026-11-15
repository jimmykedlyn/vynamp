# Modelo de ameaças e controles

Versão: 0.8.1 · Data: 2026-11-15 · Estado: threat model aplicado (TASK-E11-02); código de produto ainda não iniciado.

Assets: arquivos do usuário, biblioteca/ratings, credentials, integridade do áudio, conta/provider, pacotes, updater. Trust boundaries: arquivo de mídia, metadado/cover, URL/manifest, pacote/theme/plugin, helper, web API e ingest admin. Local user media é entrada não confiável para parsers, não autorização para executar código.

## Como ler as colunas de teste

As duas colunas finais respondem perguntas diferentes e as duas são necessárias:

- `Cenário adversarial` é o ataque concreto que a TASK dona tem de cobrir. É o registro
  do que precisa ser provado; `TASK-E06-02` (zip) e `TASK-E07-05` (sandbox) dependem
  desta lista. Ela não muda quando um teste nasce — muda quando o ataque muda.
- `Teste que existe hoje` foi **medida** contra o tree no commit desta revisão, não
  herdada da versão anterior. Cada linha traz um de dois valores:
  - `arquivo:linha` de um teste que roda hoje — nome da função de teste entre crases.
  - **sem teste**, com a TASK dona do teste que falta. "Sem teste" é registro, não
    pendência escondida: a entrada continua no modelo e o controle continua obrigatório,
    mas ninguém pode afirmar que ele está provado.

As TASKs de módulo estão em submódulo. Um clone sem
`git submodule update --init --recursive` não tem esses arquivos, e a leitura da coluna
passa a ser documental. É a razão de o job `specs` do `ci.yml` buscar submódulos.

| Entrada / ameaça | Controle obrigatório | Cenário adversarial | Teste que existe hoje |
|---|---|---|---|
| Mídia/tag/capa malformada | budgets, parser seguro, native helper isolado | fuzz/corpus, oversized/decompression bomb | **Parcial.** `libs/decode/crates/vynamp-decode-core/src/limits.rs:117` `channels`, `:131` `sample_rate`, `:142` `frames_bytes`, `:156` `default_metadata_limit_is_64_mib`; `libs/decode/crates/vynamp-decode-core/tests/id3.rs:38` `oversized_tag_hits_metadata_limit`; `libs/decode/crates/vynamp-decode-flac/tests/metadata.rs:180` `metadata_over_limit`, `:204` `block_headers_count_toward_the_metadata_limit`, `:232` `the_number_of_metadata_blocks_is_capped`, `:424` `byte_flip_and_truncation_smoke`; `libs/decode/crates/vynamp-decode-flac/tests/decode.rs:464` `rejects_malformed_subframes`, `:628` `garbage_never_panics`; `libs/decode/crates/vynamp-decode-flac/tests/demux.rs:611` `byte_flip_smoke_never_panics`, `:666` `truncation_at_frame_boundaries`; `libs/decode/crates/vynamp-decode-riff/tests/riff.rs:316` `channel_limit`, `:333` `frame_limit`, `:500` `huge_chunk_size_is_eof`, `:525` `truncated_headers`, `:568` `byte_flip_and_truncation_smoke`; `libs/decode/crates/vynamp-decode-aiff/tests/aiff.rs:687` `byte_flip_and_truncation_smoke`; `libs/decode/crates/vynamp-decode-mp3/tests/mp1.rs:269` e `mp2.rs:374` `garbage_never_panics`. **Sem teste:** decompression bomb (corpus/fuzz é `TASK-E10-05`) e capa — o bloco `PICTURE` é hoje só `RawBlock` não interpretado (`libs/decode/crates/vynamp-decode-flac/src/metadata.rs:69`), então não há parser de imagem a atacar nem teste a escrever até existir um |
| Playlist/HLS/redirect SSRF | scheme/domain policy, DNS+redirect checks | loopback, link-local, rebound, file URI | **sem teste.** Nenhum cliente HTTP existe no tree; a dona é `TASK-E05-09`, com a política de proxy em `TASK-E17-11` |
| Pacote zip traversal/bomb | canonical path, limits, staging | ../, absolute, symlink, huge ratio | **sem teste.** Nenhum leitor de pacote existe; a dona é `TASK-E06-02`, sobre o validador de `TASK-E06-01` |
| Plugin loop/rede/arquivo indevido | fuel/deadline/memory/cap broker | trap, denied calls, flood | **sem teste.** Sem runtime de plugin; dona `TASK-E07-01`, com broker em `TASK-E07-03` e lifecycle em `TASK-E07-04` |
| Native helper compromise | sandbox OS, mínimo handles, IPC auth | child crash, malicious RPC, escape review | **sem teste, e sem helper.** Medido: não há helper de privilégio nem processo filho no tree (ver §Isolamento de helper) |
| Token/URL em logs | keychain e redaction estrutural | secret fixture em export/log | **sem teste de redaction.** O que existe é a varredura de segredo no histórico (`cargo xtask check-secrets`, `xtask/src/secrets.rs`), que é outra coisa: ela acha credencial commitada, não credencial impressa em log. Redaction é `TASK-E11-05` |
| Update adulterado/downgrade | assinatura+hash+metadata expirável | invalid key, replay, rollback attack | **sem teste.** Updater inexistente; dona `TASK-E11-06` |
| DB/write interrompido | atomic write/transaction/backups | kill durante save/migrate | **sem teste.** Nenhum schema ou escrita de biblioteca existe; dona `TASK-E03-01`, com escrita atômica de configuração em `TASK-E11-05` |
| Web ingest remoto/upload | SSRF, content probe, quotas, workers | invalid body/foreign tenant | **sem teste.** Sem backend; dona `TASK-E09-05` |
| Sync cross-user | authz por objeto, tenant/account scope | IDOR, replay, stale revision | **sem teste.** Sem sync; dona `TASK-E09-08` |

## Defaults de limite

Onde há budget hoje, ele é o `Limits` de `vynamp-decode-core`, aplicado a toda entrada
não confiável de mídia. Os valores medidos em `limits.rs:23`:

| Campo | Default | Efeito |
|---|---|---|
| `max_bytes` | 64 GiB | teto de leitura acumulada |
| `max_frames` | 66 355 200 000 | teto de frames |
| `max_channels` | 32 | recusa acima; zero é `Invalid` |
| `max_sample_rate` | 768 000 Hz | recusa acima; zero é `Invalid` |
| `max_metadata_bytes` | 64 MiB | teto do metadado, contado com os cabeçalhos de bloco (`metadata.rs:204`) |
| `max_depth` | 16 | teto de aninhamento |

O default é o piso, não a política final: cada chamador pode apertar. O que o teste
prova é que o limite **reprova** acima do teto e **aceita** exatamente no teto, nos dois
sentidos (`metadata.rs:180`, `:204`).

## Segurança não bloqueante

Mídia local e rádio não exigem conta. Consentimentos aparecem só quando ação acessa externo, instala extensão ou grava/exclui. Sem prompts genéricos toda execução. Pacotes builtin auditados não precisam repetir permissões. Revoke de plugin cancela requests e encerra handles; revogar rede não mata playback local.

## Política de token no CI

Prova por leitura dos arquivos, não por confiança. No commit desta revisão valem as duas
condições abaixo, e ambas são verificadas por `cargo xtask check-delivery`:

1. Todo workflow declara `permissions: contents: read` no topo. O token default do GitHub
   não é seguro por omissão — em repositório antigo ele nasce read/write.
2. Nenhuma chave de permissão, no bloco de workflow nem no de job, tem valor diferente de
   `read` ou `none`. Um `permissions:` de job substitui o do workflow inteiro, então a
   linha do topo sozinha não prova nada sobre o job que roda; e ler só `contents` provaria
   uma chave e chamaria isso de política — `id-token: write` troca OIDC por credencial de
   nuvem e `packages: write` publica artefato.

**Onde o gate roda hoje: no `pre-push` local, ainda não em CI.** O
`.githooks/pre-push` chama `cargo xtask ci docs`, que inclui `check-delivery`; nenhum
workflow de `.github/workflows/` chama `check-delivery` nem `ci docs`. Consequência
medida: um clone sem `setup.sh` — e todo runner de CI — não tem o gate, então apagar
`permissions:` passa verde no CI e só reprova na máquina de quem instalou o hook. Quem
fecha essa lacuna é `TASK-E15-04` (job de CI para o gate multiplataforma, commit
`8d11700`, branch `feature/task-e15-04-claude-ci-gate-multiplataforma`), que **ainda não
está em `develop`**. Até lá, "portão no CI" não é afirmável.

O gate está em `xtask/src/validate.rs` (`github_actions`), com os testes
`github_actions_requires_the_read_only_token_policy`,
`github_actions_rejects_a_job_that_widens_the_token`,
`github_actions_rejects_any_wide_permission_key` e
`github_actions_accepts_permissions_named_none`. Sem ele, apagar o bloco é uma linha
silenciosa: o workflow continua rodando e nada diz que o token alargou.

Toda action é fixada por SHA de 40 hex, conferido pelo mesmo gate
(`github_actions_rejects_unpinned_action`). A varredura de segredo roda no job `tooling`
do `ci.yml` sobre o histórico alcançável, com `git fetch --unshallow` antes
(`github_actions_requires_the_secret_scan_on_full_history`), e o resultado medido nesta
revisão foi `No credential shape found in the reachable history.` — essa, sim, roda em
CI.

## Isolamento de helper

**Estado medido: não há helper.** O critério AC-3 fala do helper de privilégio e do host
que o invoca; nenhum dos dois existe. O comando que provou a ausência:

```
$ grep -rniE '\bhelper\b|ipc|named pipe|unix socket' --include=*.rs --include=*.toml \
    libs apps xtask | grep -v target
```

As cinco ocorrências são comentário e nome de função de teste — nenhuma é processo,
socket ou transporte (`apps/*/src/main.rs`, os três, são um `println!` de versão).

Consequência, registrada em vez de mascarada: **AC-3 não está satisfeito e não pode
estar** enquanto o helper não existir. O que fecha o AC é a implementação do helper, que
é `TASK-E07-05` (sandbox OS, RPC autenticado, memória compartilhada limitada), sobre os
adapters por host de `TASK-E14-02`. Esta TASK define a fronteira e registra a ausência;
implementar o helper aqui seria fazer trabalho de duas TASKs de outro módulo.

A prova de sandbox que o AC-3 pede — filho que cai, RPC malicioso, revisão de escape —
não pode rodar sem o processo existir. O relatório de ausência é a evidência honesta
disponível; "sandbox provado" não é afirmável.

## Updater

Feed HTTPS + assinatura de manifesto e artefato com key rotation, channel, version, target, hash,size,expires,min_supported e revoked builds. Validar antes executar instalador; downloads staged, rollback se launch health check falha. Config/schema compatibility verificada; não downgrade DB sem backup. Offline mantém versão atual. Um algoritmo improvisado de assinatura não será criado; usar esquema/runtime estabelecido e auditado. Implementação em `TASK-E11-06`; hoje não há teste porque não há updater.

## Riscos residuais

O risco residual **não** é zerado por esta revisão. O que mudou foi a honestidade do
registro: das 10 entradas, uma tem cobertura parcial com testes medidos e nove estão sem
teste, cada uma com TASK dona nomeada. Um threat model que declarasse as dez cobertas
seria falso — o produto não tem cliente HTTP, empacotador, runtime de plugin, helper,
updater, banco nem backend.

A TASK que fecha a lacuna de cobertura como um todo é `TASK-E10-05` (fuzz targets,
corpus adversarial, testes de limites e cobertura do threat model). Enquanto ela não
rodar, "cobertura do threat model" é uma linha de matriz, não uma medida.

## Limites

Sandbox reduz impacto, não prova ausência de vulnerabilidade. Bit-perfect e permissões não implicam direitos sobre conteúdo. Native/codec dependencies entram em SBOM e triagem de CVEs (auditoria em `TASK-E11-04`); crítico explorável bloqueia release, exceptions documentadas com owner/expiry. Security review focused nos parsers, IPC e update antes de 1.0.
