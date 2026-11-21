# TASK-E10-05 — Fuzz parsers, pacotes e RPC

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E10-05` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E10-05` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E06-02`, `TASK-E07-03`, `TASK-E10-01` |
| Bloqueia | `TASK-E07-10`, `TASK-E10-09` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | médio |

## Entrega

Fuzz targets, corpus adversarial, testes de limites e cobertura do threat model.

## Boundary

- **Não** implementa o broker de permissões nem persiste grants: `TASK-E07-03` é a dependência e a dona. O fuzz só prova que a recusa acontece e com qual motivo; conceder ou revogar grant aqui duplicaria a autoridade de acesso.
- **Não** implementa o validador ZIP de tema, o staging nem o rollback: `TASK-E06-02` é a dependência. O corpus adversarial de tema entra pelo validador dela, e corrigir o validador dentro desta TASK misturaria achado de fuzz com a entrega que ele audita.
- **Não** faz a revisão de segurança do isolamento das extensões nem os testes de falha durante reprodução: `TASK-E07-10` é a TASK que esta bloqueia. Um alvo de fuzz sem crash não é revisão de sandbox.
- **Não** executa os drills de kill, migração e rollback de update, nem o teste de vazamento de token em log ou export: `TASK-E10-09` é quem faz, e esta TASK a bloqueia.
- **Não** fixa os defaults de limites do threat model: `TASK-E11-02` define budgets, regras de IPC/pacotes e a prova de sandbox. Aqui os limites entram por parâmetro do alvo e o valor medido é registrado, não decidido.
- **Não** faz soak de 8 h nem matriz de hardware: `TASK-E10-08` cobre isso. Um alvo de fuzz com orçamento de minutos não substitui soak e não mede underrun.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e10-05-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E10-05-AC-1` | o corpus adversarial: os 10 arquivos `.flac` de `libs/decode/tests/fixtures/flac-conformance/faulty/` (a seleção commitada não inclui o `10 - ` do upstream, que `conformance_flac.rs` pula), os manifestos pcm/flac/mpeg truncados em cada byte de fronteira, um ZIP de tema com proporção 1000:1, 200 arquivos e um caminho `../`, um manifest HLS com 20 mil entradas e aninhamento 4, e um payload RPC declarando 4 GiB de corpo com 12 bytes enviados | cada entrada roda em alvo de fuzz com teto de memória declarado e watchdog de tempo por execução, por um orçamento fixo de 600 s por alvo, o mesmo de `-max_total_time=600` | Mídia/imagem/HLS/ZIP/RPC inválidos não causam OOM/hang |
| `TASK-E10-05-AC-2` | o broker de `TASK-E07-03` com um grant de `network.domains` que não inclui o host pedido, e manifests cujas URLs filhas apontam para 127.0.0.1, 169.254.169.254, `file:///etc/passwd`, um redirect 302 para host fora da allowlist e um domínio que resolve para loopback | os casos de limite submetem esses pedidos e filhos ao broker e ao parser de manifest e registram a recusa com o motivo e o offset | permissões negativas e URLs filhas cobertas. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | alvos de fuzz com `cargo fuzz run parse_media corpus/ -- -max_total_time=600 -rss_limit_mb=1024` e os pares `theme_zip` e `plugin_rpc`, com corpus inicial montado a partir de `libs/decode/tests/fixtures/flac-conformance/faulty/` e dos manifestos truncados, e cada crash minimizado versionado com sha256 no manifesto | Nada de fuzz existe: `find . -type d -name 'fuzz*'` não acha diretório, nenhum `Cargo.toml` cita `arbitrary`, `libfuzzer-sys` ou `proptest`, `cargo-fuzz` não está no PATH desta máquina (`~/.cargo/bin` tem cargo, rustfmt, clippy, miri e rls, não cargo-fuzz) e a toolchain fixada em `rust-toolchain.toml` é 1.99.0 com profile minimal, sem nightly. A prova mais próxima hoje vem de dois testes: `libs/decode/crates/vynamp-decode/tests/conformance_flac.rs` rejeita em `Mode::Strict` os arquivos de `faulty/` exigindo pelo menos 5 e pulando o `10 - `; `libs/decode/crates/vynamp-decode-flac/tests/resilient_decode.rs` corrompe em memória os próprios `sine-16-stereo.flac` e `sine-24-stereo.flac` e espera `Error::Limit(LimitKind::Frames)` na linha 169. Os limites vêm de `libs/decode/crates/vynamp-decode-core/src/limits.rs`, com default `max_bytes: 64 << 30` e `max_depth: 16`. A evidência alvo: zero OOM com RSS abaixo de 1 GiB, zero execução acima de 10 s de parede, e toda recusa nomeando a variante de `LimitKind` ou o `Error::Invalid` com offset. |
| Hardware | nenhum: os alvos consomem bytes em processo de usuário; nenhum caso abre dispositivo de áudio, tela ou GPU, e a rede é o fixture local de `TASK-E10-04`, com loopback e DNS controlados pelo próprio teste | ausência é limitação registrada, não aprovação |

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
