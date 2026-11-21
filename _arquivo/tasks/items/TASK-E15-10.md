# TASK-E15-10 — Fechar release assinado e draft privado

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E15-10` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E15-10` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M3 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E15-06`, `TASK-E15-07`, `TASK-E15-08`, `TASK-E15-09`, `TASK-E10-10` |
| Bloqueia | `TASK-E11-09` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–10 dias úteis |
| Risco | alto |

## Entrega

Evidence digest, gates, manifest minisign, secrets/environments e draft.

## Boundary

- **Não** constrói nem conserta o instalador de cada plataforma: NSIS/portable com SignTool é `TASK-E15-06`, AppImage/DEB é `TASK-E15-07`, DMG/ZIP Universal2 com notarização é `TASK-E15-08` e APK/AAB com keystore é `TASK-E15-09`. Esta TASK recebe os oito arquivos prontos e assinados; reabrir a assinatura aqui criaria um quinto dono do mesmo certificado.
- **Não** monta a matriz de evidência REQ/TASK/QA nem o relatório de problemas conhecidos. Isso é `TASK-E10-10`; aqui apenas se consome `evidence/release/<versao>.json`, e regerar a matriz neste passo duplicaria a fonte de verdade dos gates de desktop.
- **Não** decide rollout beta, canary ou o corte da stable 1.0. `TASK-E11-09` depende desta TASK e é quem faz release candidate, plano canary e relatório de evidência; antecipar o plano de rollout aqui inverteria a dependência.
- **Não** implementa updater assinado, canais dev/nightly/beta/stable, expiry, rotação de chave ou rollback. `TASK-E11-06` é o dono; aqui só se assina o manifest e se confere que a chave pública em `packaging/update.pub` valida a assinatura.
- **Não** gera SBOM, `THIRD_PARTY_NOTICES` nem inventário de dependências. `TASK-E11-03` resolve licenças e `TASK-E11-04` cobre supply chain e audits; o preflight só reprova pela ausência desses arquivos, não os produz.
- **Não** aplica proteção de branch ou tag nova. `TASK-E15-03` entregou as proteções de main/develop e `TASK-E19-06` as aplicou nos dez repositórios com `--reviews 0`; reaplicar proteção neste passo desfaria essa decisão. Aqui só se registram os environments release-signing e release-publish.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e15-10-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E15-10-AC-1` | uma run de release no GitHub Actions cujo GITHUB_RUN_ID é conhecido e cujos artifacts no padrão release-* contêm exatamente os oito nomes vynamp-0.1.0-windows-x64-setup.exe, vynamp-0.1.0-windows-x64-portable.zip, vynamp-0.1.0-linux-x64.deb, vynamp-0.1.0-linux-x64.AppImage, vynamp-0.1.0-macos-universal.dmg, vynamp-0.1.0-macos-universal.zip, vynamp-0.1.0-android-arm64.apk e vynamp-0.1.0-android-arm64.aab, cada um assinado pela ferramenta da sua plataforma, maior que 0 byte e arquivo regular, e um clone com dist/release vazio | cargo xtask release assemble --version 0.1.0 roda com GITHUB_ACTIONS=true, GITHUB_REF em refs/tags/v0.1.0, GITHUB_RUN_ID da run e o preflight aprovado, e depois cargo xtask release publish-draft --version 0.1.0 roda com MINISIGN_SECRET_KEY_BASE64 do environment release-publish | Conjunto completo de artifacts reais/assinados |
| `TASK-E15-10-AC-2` | evidence/release/0.1.0.json com source_digest idêntico ao de cargo xtask release digest, reviewers exatamente [claude], e as cinco plataformas windows-x64, linux-x64, macos-intel, macos-silicon e android-arm64 com status passed, campo hardware não vazio, caminho de relatório existente e sha256 do relatório conferido | cargo xtask release preflight --version 0.1.0 roda com GITHUB_ACTIONS=true e GITHUB_REF=refs/tags/v0.1.0, e cargo xtask release digest roda de novo após editar um arquivo em xtask, packaging, config ou Cargo.lock | source digest e relatórios revisados |
| `TASK-E15-10-AC-3` | dist/release com um artefato de 0 byte, ou um vynamp-0.1.0-stub.exe fora do conjunto esperado, ou MINISIGN_SECRET_KEY_BASE64 de uma chave cuja pública não é a gravada em packaging/update.pub | cargo xtask release publish-draft --version 0.1.0 tenta criar o release a partir desse diretório | nunca publicar stubs/unsigned stable |
| `TASK-E15-10-AC-4` | draft privado v0.1.0 criado por gh release create --draft no repositório jimmykedlyn/vynamp, com os oito artifacts, manifest.json, SHA256SUMS e manifest.json.minisig anexados, e o environment release-publish restrito ao mantenedor com aprovação exigida | o mantenedor, única conta com aprovação no environment release-publish, confere SHA256SUMS, manifest.json e a assinatura minisign e roda gh release edit v0.1.0 --draft=false para converter o draft em release publicado | mantenedor aprova publicação. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `cargo xtask release preflight --version 0.1.0` seguido de `cargo xtask release publish-draft --version 0.1.0` numa árvore com `evidence/release/0.1.0.json` e `dist/release` reais, complementado por `cargo test --package xtask --test tooling release` e `cargo xtask check-delivery` | Hoje o preflight falha na primeira cláusula: `config/project.json` tem `release_enabled`, `application_ready` e `branding_exports_ready` em false e a mensagem é 'Release disabled: actual player/build/hardware/signing acceptance missing' (`xtask/src/release.rs:74-77`); com as flags ligadas o primeiro bloqueio é o branding, como prova o teste `release_blocks_pending_native_branding_exports` em `xtask/tests/tooling.rs` (`branding_exports_ready` em false e `packaging/icons/export-manifest.json` inexistente, só o `.example.json` no repo), e, passado o branding, ainda caem 'Tag/product version mismatch' (`product_version` em `0.1.0-dev`) e 'Actual desktop player absent' (`ci::detect` procura `apps/desktop`, que não existe no umbrella; o player mora em `apps/player` com o pacote `vynamp_player`). Faltam também `THIRD_PARTY_NOTICES`, `apps/android/gradlew`, `packaging/update.pub` e `evidence/release/`, não há `dist/release` e nenhum workflow em `.github/workflows/` publica artifacts `release-*`, então o `assemble` não tem run de onde baixar. Quando existir, a evidência é: `manifest.json` com exatamente 8 entradas em `artifacts` (as 8 de `release.rs::SUFFIXES`), todas com `bytes` maior que 0 e `sha256`; `SHA256SUMS` com as mesmas 8 linhas; `manifest.json.minisig` aceito por `minisign -V -p packaging/update.pub -m manifest.json -x manifest.json.minisig` — o `publish-draft` reescreve `manifest.json` e `SHA256SUMS` e re-assina com `MINISIGN_SECRET_KEY_BASE64` antes de verificar (`release.rs:252-290`), então um `.minisig` desatualizado é substituído e não rejeitado, e a recusa real é a chave cuja pública difere de `packaging/update.pub`; e o release remoto em estado draft. Nada disso existe hoje, `minisign` não está no PATH desta máquina e o teste `manifest_requires_exact_nonempty_set` cobre só um diretório temporário com bytes 'signed fixture, not player'. |
| Hardware | nenhum: nenhum passo deste fluxo abre dispositivo de áudio; os cinco alvos chegam como relatório e JSON de `evidence/release/<versao>.json` produzidos por TASK-E15-06 a E15-09, e este ambiente é Windows x64 sem `minisign` no PATH, sem certificado PFX, sem conta Apple Developer e sem keystore Android, o que bloqueia a assinatura real até o mantenedor prover as credenciais dos environments release-signing e release-publish | ausência é limitação registrada, não aprovação |

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
