# TASK-E15-08 — Validar DMG/ZIP Universal2 e notarização

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E15-08` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E15-08` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M3 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E12-02`, `TASK-E11-04` |
| Bloqueia | `TASK-E15-10`, `TASK-E16-08` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–10 dias úteis |
| Risco | alto |

## Entrega

Slices Intel/Silicon, runtime inside-out signing e notary/staple.

## Boundary

- **Não** entrega a porta macOS: CoreAudio, Now Playing, Keychain e acessibilidade são de `TASK-E12-02`. Aqui se prova arquitetura, assinatura e notarização do bundle que ela produz.
- **Não** produz `packaging/icons/Vynamp.icns` nem o `export-manifest.json` final: `branding_exports_ready` é de `TASK-E14-06`, que fecha a matriz de evidência com os exports de marca, e `TASK-E02-11` é a dona dos ícones extraídos. A pasta tem só `README.md` e `export-manifest.example.json`, e `branding::exports` reprova antes do primeiro `codesign`; branding é pré-requisito declarado, não trabalho desta TASK.
- **Não** cria identidade Developer ID, chave de notary nem senha do `.p12`: `TASK-E15-10` organiza secrets e environments, e `Signing::provision` exige `MAC_P12_BASE64`, `MAC_P12_PASSWORD`, `MAC_NOTARY_KEY_ID`, `MAC_NOTARY_ISSUER` e `MAC_SIGN_IDENTITY`.
- **Não** cobre UTTypes, document types, Quick Actions no Finder nem callbacks de LaunchServices: `TASK-E16-05` é a dona. O `packaging/macos/Info.plist` daqui declara só bundle, versão e `LSMinimumSystemVersion`.
- **Não** declara helpers ou libraries de terceiros que não existam: `runtime-files.json` com `macos-universal: []` e `audit_complete: false` reprova em `dependencies()`, e quem inventaria libraries reais é `TASK-E11-04`. Rosetta também não substitui slice: a AC-1 é execução nativa em cada Mac.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e15-08-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E15-08-AC-1` | um Mac Intel x86_64 real e um Mac Apple Silicon arm64 real, ambos com macOS 13 ou superior, e o `vynamp-1.0.0-macos-universal.dmg` e `.zip` gerados por `lipo -create` dos slices `x86_64-apple-darwin` e `aarch64-apple-darwin` | o DMG é montado e o `Vynamp.app` é aberto e tocado em cada máquina, com `lipo -archs Contents/MacOS/vynamp` e `sysctl -n sysctl.proc_translated` conferidos para provar execução nativa em vez de traduzida | Executar nativamente em ambos Macs |
| `TASK-E15-08-AC-2` | o build já assinado inside-out e aceito pelo `notarytool`, com o bilhete grampeado, partindo de um Mac com o 1.0.0 instalado em `/Applications` e com o `.dmg` e o `.zip` recém-baixados | `spctl --assess --type execute -vv` e `xcrun stapler validate` rodam sem remover a quarentena à mão, e depois o 1.0.1 é instalado por cima do 1.0.0 copiando o `.app` do zip | Gatekeeper/notary/staple/upgrade testados |
| `TASK-E15-08-AC-3` | o bundle completo, com todo arquivo de `Contents/MacOS`, cada `.dylib` e `.so` e cada `.framework` listados em `packaging/runtime-files.json` para `macos-universal` | `lipo -info` e `lipo -verify_arch x86_64 arm64` rodam sobre cada um desses arquivos, e não só sobre o binário principal | helpers/libs suportam arquitetura prometida. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `cargo xtask package --platform macos-universal --version 1.0.0` num Mac com Developer ID, seguido de `lipo -archs` e `lipo -verify_arch x86_64 arm64`, `codesign --verify --deep --strict`, `spctl --assess --type execute -vv`, `xcrun stapler validate` e um ciclo install 1.0.0, upgrade 1.0.1 e `codesign -dv` do app resultante nos dois Macs | nada disso pode rodar aqui: `codesign`, `xcrun`, `lipo` e `hdiutil` não existem neste Windows, `packaging/icons/Vynamp.icns` não existe, `runtime-files.json` para `macos-universal` está vazio com `audit_complete: false` e `THIRD_PARTY_NOTICES` não existe, então `licenses()` falha antes do `lipo`. `package.rs` não tem `#[test]`. A evidência esperada é `lipo -archs` respondendo `x86_64 arm64` para o binário e para cada dylib listada, `spctl` com 'accepted' e origem 'Notarized Developer ID', `stapler validate` com código 0, e o upgrade preservando library e settings com sha256 idêntico. |
| Hardware | Mac Intel real e Mac Apple Silicon real com macOS 13 ou superior: ambos ausentes neste ambiente, que é Windows 11 sem `codesign`, `xcrun`, `lipo` ou `hdiutil`. Rosetta não substitui o Mac Intel e a AC-1 exige execução nativa em cada slice, então sem os dois aparelhos a TASK fica como limitação registrada, nunca como passed. | ausência é limitação registrada, não aprovação |

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
