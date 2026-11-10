# TASK-E11-09 — Preparar stable 1.0 e rollout beta

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E11-09` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E11-09` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E11-08`, `TASK-E10-10`, `TASK-E15-10` |
| Bloqueia | `TASK-E09-01` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | médio |

## Entrega

Release candidate, artifacts assinados, relatório de evidência e plano canary.

## Boundary

- **Não** constrói nem assina artefato nenhum. O instalador por plataforma é `TASK-E11-07` e a assinatura de binário, installer e APK é `TASK-E15-06`, `TASK-E15-07`, `TASK-E15-08` e `TASK-E15-09`; aqui só se confere o que eles produziram, e chamar `SignTool` nesta TASK criaria um segundo caminho de assinatura sem dono.
- **Não** gera o `manifest.json.minisig` nem abre o draft privado. Digest, manifesto minisign, environments e a aprovação de publicação são `TASK-E15-10`; esta TASK consome o conjunto assinado como entrada do corte.
- **Não** escreve guia de usuário, notices nem release notes. `TASK-E11-08` é dona da documentação de lançamento; aqui o problema conhecido é conferido como visível no material que ela publicou, e reescrevê-lo nesta TASK deixaria o texto sem revisor.
- **Não** fecha a matriz de evidência REQ/TASK/QA nem o relatório de problemas conhecidos. Ambos saem de `TASK-E10-10`; produzi-los aqui faria dois donos do mesmo documento e a matriz passaria a divergir do backlog.
- **Não** roda soak, matriz de hardware nem acessibilidade. Soak e matriz de hardware são `TASK-E10-08` e o fechamento multiplataforma é `TASK-E14-06`; um alvo ausente lá bloqueia o corte aqui em vez de ser compensado por teste local.
- **Não** decide catálogo, direitos, região ou custo por ouvinte-hora. Isso é `TASK-E09-01`, e o roadmap põe M4 depois do 1.0: trazer essa decisão para o corte travaria o stable por trabalho que não é pré-requisito dele.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e11-09-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E11-09-AC-1` | o SHA candidato a 1.0.0 com o digest de fonte de `cargo xtask release digest` e, anexados a `evidence/release/1.0.0.json`, a matriz de evidência M1–M3 de `TASK-E10-10` e o fechamento de `TASK-E14-06` | `cargo xtask release preflight --version 1.0.0` roda nesse SHA e a lista de gates de saída M3 de `delivery/01-roadmap.md` é conferida item por item contra a matriz anexada | Gates M3 passam |
| `TASK-E11-09-AC-2` | o relatório de problemas conhecidos que `TASK-E10-10` entrega, com as limitações de hardware por plataforma e as exceções de capability de `delivery/04-gates.md` | a página de release do 1.0.0 é montada a partir de `CHANGELOG.md` e desse relatório, e lida como fica publicada, sem edição fora do documento | problemas conhecidos visíveis |
| `TASK-E11-09-AC-3` | os oito artefatos que `release.rs` exige por `SUFFIXES` (windows-x64-setup.exe, windows-x64-portable.zip, linux-x64.deb, linux-x64.AppImage, macos-universal.dmg, macos-universal.zip, android-arm64.apk, android-arm64.aab) em `dist/release`, com `manifest.json` e `manifest.json.minisig` de `TASK-E15-10` | `cargo xtask release assemble --version 1.0.0` roda sobre esse diretório e em seguida `sha256sum -c SHA256SUMS` e `minisign -V -p packaging/update.pub` são executados sobre cada artefato | checksums/assinaturas conferidos |
| `TASK-E11-09-AC-4` | a triagem de bugs por severidade contra o SHA candidato, com o critério de zero bug P0/P1 conhecido no caminho crítico de `quality/01-estrategia-testes.md` | a triagem de release é fechada e cada P0/P1 ainda aberto é confrontado com esse critério antes do corte, incluindo os de dados corrompidos, vazamento de secret e stall grave de áudio | zero P0/P1 |
| `TASK-E11-09-AC-5` | uma janela de beta no canal beta, com feed de update separado do stable e os relatórios de usuário coletados durante a janela | cada item de feedback é classificado em P0/P1, bug menor ou pedido, e os P0/P1 são fechados ou viram problema conhecido visível antes do corte do stable | feedback beta tratado. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `cargo xtask release digest` sobre o SHA candidato, depois `cargo xtask release preflight --version 1.0.0`, depois `cargo xtask release assemble --version 1.0.0` sobre um `dist/release` com os oito nomes de `release::SUFFIXES`, fechando com `sha256sum -c SHA256SUMS` e `minisign -V -p packaging/update.pub -m dist/release/manifest.json -x dist/release/manifest.json.minisig` | `preflight` tem que imprimir `Preflight passed` e o `manifest.json` tem que listar exatamente 8 artefatos, cada um com `bytes` maior que 0 e `sha256` de 64 hex. Hoje o comando reprova na primeira guarda, com `Release disabled: actual player/build/hardware/signing acceptance missing`, porque `config/project.json` traz `application_ready: false` e `release_enabled: false`; também não existem `evidence/release/`, `THIRD_PARTY_NOTICES`, `packaging/update.pub`, `apps/android/gradlew` nem `tests/hardware/`, e `release assemble` exige `GITHUB_RUN_ID` e um `dist/release` que não existe neste clone. |
| Hardware | Windows 11 x64 é o único alvo presente neste ambiente, e sem DAC USB; Linux x64, macOS Intel, macOS Silicon e aparelho Android arm64 estão ausentes — `.github/workflows/quality.yml` exige runners self-hosted rotulados vynamp-hardware e vynamp-android-device que não estão provisionados (`TASK-E15-05`), então as cinco linhas de `platforms` em `evidence/release/1.0.0.json` não podem ser preenchidas como `passed` aqui | ausência é limitação registrada, não aprovação |

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
