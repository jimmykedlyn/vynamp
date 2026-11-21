# TASK-E14-05 — Validar packaging e update em cada alvo

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E14-05` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E14-05` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M3 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E11-07`, `TASK-E12-01`, `TASK-E12-02`, `TASK-E13-09` |
| Bloqueia | `TASK-E14-06` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | alto |

## Entrega

Installer/packages/Universal 2/APK-AAB, migrations, dependencies nativas e SBOM.

## Boundary

- **Não** constrói nem assina os pacotes de cada plataforma. NSIS e ZIP portable, DEB e AppImage, DMG e ZIP Universal2, APK e AAB saem de `TASK-E11-07`; aqui se valida o que ele produziu, em máquina limpa, não como foi produzido nem com qual recipe.
- **Não** implementa a verificação do manifesto de update nem o staging. Assinatura, expiração, replay, swap atômico e rollback são `TASK-E11-06`, e esta TASK só executa o update que ele entrega.
- **Não** emite o SBOM nem a supply chain. Jobs de CI, pins por SHA, audit e geração de SBOM são `TASK-E11-04`; o SBOM aqui é anexo conferido do release, e hoje o xtask não tem gerador nenhum.
- **Não** roda os drills de migration, backup e recovery da biblioteca. Esses cenários são `TASK-E10-09` (QA-012) e o schema SQLite que as migrations alteram é `TASK-E03-01`; o que se confere aqui é que a migration empacotada roda, não que o recovery funciona.
- **Não** implementa as portas Linux, macOS e Android. PipeWire, MPRIS e Orca são `TASK-E12-01`, CoreAudio e Now Playing são `TASK-E12-02` e o AAB com ABIs e NDK é `TASK-E13-09`; um installer quebrado aqui pode ser bug da porta e continua sendo deles. A identidade do assinante do APK e do AAB, a ABI e a compatibilidade de page size medidas no artefato são `TASK-E15-09`, e aqui o checklist só confere que a prova dela foi publicada — medir de novo daria dois pareceres sobre o mesmo SHA-256.
- **Não** fecha a matriz agregada de release nem decide reduzir escopo de plataforma. A evidência agregada dos cinco alvos, a acessibilidade e o bug P0/P1 são `TASK-E14-06`, que esta TASK bloqueia.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e14-05-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E14-05-AC-1` | os oito artefatos nomeados em SUFFIXES de xtask/src/release.rs, windows-x64-setup.exe, windows-x64-portable.zip, linux-x64.deb, linux-x64.AppImage, macos-universal.dmg, macos-universal.zip, android-arm64.apk e android-arm64.aab, gerados a partir de uma tag, com os certificados que xtask/src/signing.rs provisiona pelas variáveis WIN_PFX_BASE64, MAC_P12_BASE64 e ANDROID_KEYSTORE_PATH | cada artefato é instalado numa máquina limpa do seu alvo, atualizado para a versão seguinte, aberto uma vez e removido, com a assinatura verificada pelo verificador nativo do alvo antes da instalação | Artifacts instalam/assinam/atualizam |
| `TASK-E14-05-AC-2` | um perfil instalado na versão N-1, com biblioteca indexada, playlists, config de dispositivo, os MimeType de packaging/linux/vynamp.desktop registrados no Linux, o handler user-scope do Windows e os grants de pasta SAF e MediaStore concedidos no Android | o pacote da versão N é aplicado sobre esse perfil e a lista de grants, associações de arquivo e permissões de sistema é comparada item a item com a fotografada antes da atualização | permissões preservadas |
| `TASK-E14-05-AC-3` | o release candidato montado por cargo xtask release assemble, com DMG e ZIP Universal2 assinados, DEB e AppImage, APK e AAB assinados, e o checklist do canal stable aberto por alvo | o checklist de release é executado: sandbox e xdg-desktop-portal no DEB e no AppImage, ticket de notarização e as duas slices no DMG e no ZIP, e target SDK e native page size do .so arm64 no APK e no AAB | portais Linux, notarização macOS e native pages Android conferidos no release. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | cargo xtask release preflight --version 1.0.0 seguido de cargo xtask release assemble --version 1.0.0 numa checkout da tag, mais a instalação de cada um dos 8 artefatos em VM limpa do seu alvo; hoje o preflight para na primeira guarda de xtask/src/release.rs com Release disabled, porque config/project.json traz release_enabled e application_ready em false, e as guardas seguintes também caem: branding_exports_ready false reprova em branding::exports antes da lista de arquivos, product_version 0.1.0-dev não bate com a tag, e quatro dos oito pré-requisitos de arquivo não existem, THIRD_PARTY_NOTICES, apps/android/gradlew, apps/android/gradlew.bat e packaging/update.pub, além de evidence/release/1.0.0.json ausente — e packaging/runtime-files.json tem audit_complete false com as listas de windows-x64, linux-x64 e macos-universal vazias | preflight sai 0 e assemble produz exatamente os 8 nomes de SUFFIXES, com checksum e assinatura de cada um; cada artefato instala, atualiza de N-1 para N e remove na sua VM limpa, e a atualização deixa a lista de grants, associações e config com diferença 0; o DMG passa spctl assess e o ticket de notarização está grampeado, com lipo -archs devolvendo x86_64 e arm64 nas duas slices; o APK passa zipalign -c -P 16 e declara target SDK dentro da política vigente; o SBOM anexado cobre as dependências nativas listadas em packaging/runtime-files.json já preenchido — hoje esse arquivo tem as três listas vazias, o xtask não gera SBOM em lugar nenhum e a pasta evidence/release nem existe |
| Hardware | Windows 11 x64, esta máquina: o ZIP portable sai, mas o setup.exe NSIS não, porque makensis não está no PATH nem no fallback de xtask/src/package.rs em C:/Program Files (x86)/NSIS/makensis.exe; faltam VM Linux limpa, já que só há WSL2 Ubuntu parado e sem sessão gráfica para xdg-desktop-portal, Mac Intel e Mac Silicon reais para spctl e lipo, aparelho Android arm64 para APK, AAB e page size de 16 KB, e os certificados WIN_PFX_BASE64, MAC_P12_BASE64 e ANDROID_KEYSTORE_PATH, que não estão definidos nesta sessão — ausência registrada, não aprovação | ausência é limitação registrada, não aprovação |

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
