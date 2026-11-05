# Evidência TASK-E19-01 — CI do repo principal em GitHub Actions

Versão 0.8.1 · 2026-11-05 · Autor Claude (branch `feature/task-e19-01-claude-public-github-actions`, base develop `a381d27`). Documento, gate local e configuração de CI; **não** é aceite de app, hardware nem release.

## Escopo entregue

| Item | Arquivo |
|---|---|
| Workflow de CI com gate `CI Gate` | `.github/workflows/ci.yml` |
| Release por tag `v*`, fail-closed | `.github/workflows/release.yml` |
| Hardware manual, opt-in explícito | `.github/workflows/quality.yml` |
| Validador invertido (exige Actions, proíbe Woodpecker) | `xtask/src/validate.rs` |
| Release valida tag por `GITHUB_REF`/`GITHUB_REF_NAME` | `xtask/src/release.rs` |
| `repo audit`/`repo protect` exigem repo público | `xtask/src/repo.rs` |
| Visibilidade esperada `public` | `config/github.json` |
| Dependabot volta a atualizar actions | `.github/dependabot.yml` |
| Woodpecker removido | `.woodpecker/`, `infra/ci/` |
| Docs normativos reescritos | `docs/engineering/{03-gitflow-github,04-ci-cd}.md`, `ADR-009`, `ADR-010`, `ADR-011`, `AGENTS.md`, PR template, `docs/INDEX.md` |

## Desenho dos workflows

`ci.yml` dispara em PR para `develop`, push em `develop`/`main` e manual. Push em branch de feature não dispara — o PR da mesma branch cobre o diff. Concurrency cancela execuções antigas da mesma ref.

Jobs: `docs` (docs/schema/backlog/delivery/fmt/clippy dos tooling + outputs `core`/`android` de `cargo xtask ci detect`), `tooling` (clippy do workspace em `windows-2022`, `ubuntu-22.04`, `macos-15-intel`, `macos-15`), `core` (matriz de quatro alvos desktop, `x86_64-pc-windows-msvc`, `x86_64-unknown-linux-gnu`, `x86_64-apple-darwin`, `aarch64-apple-darwin`, condicionada a `needs.docs.outputs.core == 'true'`), `android` (`ubuntu-24.04`, condicionada ao output), `gate`.

`gate` tem `name: CI Gate`, `if: always()` e `needs: [docs, tooling, core, android]`; as checagens fazem `test "${{ needs.<job>.result }}" = success` e só conferem `core`/`android` quando o `detect` diz que existem. Dependência falha, cancelada ou ausente reprova — não há caminho para gate verde sem as dependências reais.

`release.yml` dispara só em `push` de tag `v*`: `preflight` (checkout com `fetch-depth: 0`, `cargo xtask tag-version`, `cargo xtask release preflight`) e `publish` com `exit 1`. A matriz de quatro installers do commit `247b424` **não** foi restaurada: depende de artefatos entre agentes, secrets de signing e caminho de build macOS arm64 que o preflight ainda recusa.

`quality.yml` é `workflow_dispatch` com input booleano `confirm_hardware_runners`, default `false`. Com `false` o job `preflight` registra que os runners `vynamp-hardware`/`vynamp-android-device` (TASK-E15-05) ainda não existem e a matriz é pulada, em vez de ficar em fila.

Todas as `uses:` são `actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683`. Nenhum `uses:` por tag.

## Comandos e resultados reais

Ambiente: Windows 11 Pro 10.0.26300, Rust 1.99.0 (rustup), git 2.47.1.windows.2.

| Comando | Resultado |
|---|---|
| `cargo test --workspace --locked` | 86 passed (2 suites) |
| `cargo fmt --all --check` | limpo |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | No issues found |
| `cargo xtask check-docs` | `"result": "passed"`, `"errors": []`, 347 arquivos markdown |
| `cargo xtask check-delivery` | `"result": "passed"`, `"errors": []`, `"workflow_files": 3` |
| `cargo xtask ci detect` | `{"android":"false","core":"false"}` |
| `grep -rn "uses:" .github/workflows/` | 6 referências, todas com SHA de 40 hex |

### Execução real no GitHub

O workflow rodou no PR #30, em `7565e12` (evento `pull_request`), e concluiu com sucesso. Run [42096253664](https://github.com/jimmykedlyn/vynamp-mediaplayer/actions/runs/42096253664). A primeira execução verde foi em `36d0e02` (run `42089094392`); o commit seguinte apenas corrigiu esta evidência.

| Check | Resultado | Duração |
|---|---|---|
| `docs` | pass | 1m31s |
| `tooling (windows-2022)` | pass | 1m37s |
| `tooling (ubuntu-22.04)` | pass | 55s |
| `tooling (macos-15-intel)` | pass | 1m40s |
| `tooling (macos-15)` | pass | 22s |
| **`CI Gate`** | **pass** | 4s |
| `core` | skipped | — |
| `android` | skipped | — |

Isso corrige a afirmação anterior desta evidência de que nenhum workflow teria rodado. O repositório é **privado** e o Actions executou normalmente — o bloqueio de billing registrado em 2026-10-18/23 (runs `39171754196`, `39979729441` etc., que falhavam sem executar nenhum step) não se repete hoje. A razão da mudança não foi determinada: pode ser consumo de cota do plano, ciclo de reset ou configuração da conta. **Não** se deve tratar "repo privado" como suficiente para prever bloqueio, nem como suficiente para prever que roda.

## Limitações — o que este PR não prova

- `core` e `android` ficam **pulados**: `cargo xtask ci detect` devolve `false`/`false` porque o workspace contém só `xtask`. Skip declarado não significa que o player passou testes, e o `CI Gate` verde não exercita build de áudio, app desktop nem Android.
- O gate publicado como status `CI Gate` já existe e passou, mas `config/github.json` continua `protection_status: pending_plan`: a branch protection **é o que exige** esse status, e segue indisponível no repositório privado. Sem ela o check é informativo, não bloqueante.
- `release.yml` falha de propósito em `publish`; nenhum artefato assinado é produzido. O caminho de tag nunca foi executado.
- `quality.yml` nunca rodou — depende de runners self-hosted inexistentes.
- Hardware segue sem cobertura de CI — depende da TASK-E15-05.
- macOS Intel/Apple Silicon executaram `tooling` com sucesso, mas o job `core` neles está pulado; a cobertura macOS de código de produto continua não exercitada.

## Controles de segurança preservados

- `permissions: contents: read` no CI e no release; o único job com permissão para publicar não existe ainda.
- `persist-credentials: false` em todo checkout.
- `release.yml` referencia secrets de signing apenas por **nome**; nenhum valor, nenhuma chave privada em workflow, artefato ou log.
- O validador recusa `.woodpecker/` e qualquer `uses:` sem SHA completo, para que a inversão da ADR-010 não reabra a porta a tag flutuante.
- `git log --all -p` do repositório não revela token, PAT, chave AWS ou chave privada (scan de 2026-11-05, sem achado além de um falso positivo no próprio detector em `infra/ci/`, agora removido).

## Pendente

- Abertura dos repositórios e reaplicação de branch protection: TASK-E19-06.
- Licença, governança, SECURITY.md e README bilíngue: TASK-E19-02.
- Remoção do Woodpecker local (Docker Desktop, agente Windows, Cloudflare Tunnel) fora do repositório: operacional, do mantenedor.
