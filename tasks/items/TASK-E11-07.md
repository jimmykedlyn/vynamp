# TASK-E11-07 — Entregar distribuição nativa por plataforma

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E11-07` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E11-07` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E11-04`, `TASK-E07-05`, `TASK-E11-06` |
| Bloqueia | `TASK-E11-08`, `TASK-E14-05` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | médio |

## Entrega

Windowsinstaller/ZIP, Linuxpackages, MacUniversal2 ou packages x64/arm64 e AndroidAPK/AAB, com signing/symbols.

## Boundary

- **Não** valida instalador, pacote e update consumindo os artefatos em cada alvo com evidência assinada: esse gate por alvo é `TASK-E14-05`, e aqui se entrega o artefato, não a matriz de aceite dele.
- **Não** implementa o helper nativo isolado, o sandbox de processo nem o RPC autenticado que o pacote precisa carregar: isso é `TASK-E07-05`.
- **Não** implementa a verificação de manifesto, canal, expiração e rollback que consome os artefatos: o updater assinado é `TASK-E11-06`, dependência direta desta TASK.
- **Não** escreve guia de usuário, notices, política de privacidade nem release notes: a documentação de lançamento é `TASK-E11-08`.
- **Não** gera as declarações OS e o manifest de capacidades a partir do build: isso é `TASK-E16-09`, e o pacote daqui apenas as inclui.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e11-07-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E11-07-AC-1` | os oito artefatos nomeados em SUFFIXES de xtask/src/release.rs (windows-x64-setup.exe, windows-x64-portable.zip, linux-x64.deb, linux-x64.AppImage, macos-universal.dmg, macos-universal.zip, android-arm64.apk, android-arm64.aab) reunidos em dist/release | cada artefato é instalado, atualizado para a versão seguinte e removido no alvo que lhe corresponde: VM limpa Windows 11, container Ubuntu LTS, Mac Intel e Mac Silicon reais e aparelho Android arm64 | Instalar/update/remove em cada alvo |
| `TASK-E11-07-AC-2` | um perfil com biblioteca indexada, playlists, favoritos, grants de pasta no Android e config de dispositivo, mais os artefatos das duas versões anteriores | o instalador roda sobre a instalação existente e depois o uninstall é executado no Windows, no Linux e no Android | preservar dados/grants |
| `TASK-E11-07-AC-3` | um DMG e um ZIP Universal2 montados com lipo sobre x86_64-apple-darwin e aarch64-apple-darwin, e um APK/AAB arm64 com as bibliotecas .so nativas e o targetSDK e o page size do release corrente | o bundle é inspecionado com lipo -info e codesign --verify, o DMG é submetido a notarização e o APK é verificado com apksigner verify e bundletool dump manifest | Mac slices nativas/notarização e Androidnative/targetSDK/page size checks |
| `TASK-E11-07-AC-4` | o mesmo build entregue nos cinco alvos de config/project.json — windows-x64, linux-x64, macos-intel, macos-silicon e android-arm64 —, com as interfaces de integração do host: chave HKCU de uninstall e atalho no Windows, .desktop e MIME no Linux, Info.plist e document types no Mac, intent filters e package no Android | o usuário instala, abre arquivo e desinstala pela interface padrão do sistema operacional, sem invocar a CLI do app | interfaces de distribuição específicas do host. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | cargo test -p xtask manifest_requires_exact_nonempty_set, que exercita artifact_manifest de xtask/src/release.rs e passa hoje exigindo o conjunto exato e não vazio (medido: 1 passed); cargo xtask release assemble não serve de oráculo ainda, porque baixa os artefatos com gh run download a partir de GITHUB_RUN_ID e o preflight falha antes com Release disabled. Hoje packaging/runtime-files.json tem audit_complete=false, packaging/update.pub e THIRD_PARTY_NOTICES não existem, apps/desktop e apps/android também não — ci detect devolve core=false e android=false | artifact_manifest aceita os oito nomes e recusa tanto um a menos quanto um a mais; lipo -info mostra as slices x86_64 e arm64 no binário principal e nos helpers, apksigner verify reporta um signer e o manifest não declara AAB como instalável. O que ainda não existe: instalador compilado, assinatura real, notarização e os smoke de install/update/uninstall em alvo nenhum |
| Hardware | parcial: Mac Intel e Mac Silicon reais não existem aqui, então slices Universal2, lipo -info, codesign e notarização ficam bloqueados, e não há aparelho Android arm64 nem emulador com o page size do release; Linux x64 tem alvo real via Docker 29.8.1 (verificado rodando alpine:3.22) e WSL Ubuntu, e o host Windows 11 serve de alvo Windows, embora não seja VM limpa — cross-build não substitui alvo, e o que falta se registra como bloqueio das AC-1, AC-2 e AC-3, não como ausência de todo alvo | ausência é limitação registrada, não aprovação |

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
