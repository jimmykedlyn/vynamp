# Evidência TASK-E19-01 — CI do repo principal em GitHub Actions

Versão 0.8.0 · 2026-11-05 · Autor Claude (branch `feature/task-e19-01-claude-public-github-actions`, base develop `a381d27`). Documento, gate local e configuração de CI; **não** é aceite de app, hardware nem release.

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

## Limitações — o que este PR não prova

- **Nenhum workflow rodou no GitHub.** A abertura para público é a TASK-E19-06 e, até ela, o CI do repositório privado não inicia por billing. Os jobs estão validados por estrutura (parse YAML + regras do validador), não por execução real.
- `core` e `android` ficam **pulados**: `cargo xtask ci detect` devolve `false`/`false` porque o workspace contém só `xtask`. Skip declarado não significa que o player passou testes.
- O gate real publicado como status `CI Gate` só existe depois do primeiro `pull_request` executado no remoto. `config/github.json` continua `protection_status: pending_plan` porque a branch protection segue indisponível no repositório privado.
- `release.yml` falha de propósito em `publish`; nenhum artefato assinado é produzido.
- Hardware segue sem cobertura de CI — depende da TASK-E15-05.
- macOS Intel/Apple Silicon entram como alvos **verificáveis** pelo desenho; a verificação só existe após uma execução real no Actions.

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
