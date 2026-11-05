# Evidência E15-04 — Woodpecker + Cloudflare Tunnel (CI/CD somente Woodpecker)

Versão 0.7.0 · 2026-10-18/19 · Autor Claude (branch `chore/task-e15-04-claude-woodpecker`, base develop `8024bdc2363d9d9b489a3b0d553fb59d2b127703`). Revisão técnica: Codex, pendente. Pipelines #1–#8 são do piloto Windows/Linux (2026-10-18); #9–#11 são da migração Woodpecker-only ([ADR-010](../../../architecture/adr/ADR-010-woodpecker-only-ci.md), 2026-10-19). macOS Intel/Apple Silicon: **não verificado** (sem agente Mac); não é aceite de app, hardware nem release.

## Ambiente

Notebook Windows 11 Pro 10.0.26300, PowerShell 7.4.20, git 2.47.1.windows.2, Rust 1.99.0 (rustup), Docker Desktop (engine 29.8.1, Compose v5.5.1, WSL2 `docker-desktop`). Woodpecker server/agents 3.19.0, plugin-git 2.10.1, cloudflared 2026.10.1 (global winget, atualizado de 2025.8.1 a pedido do mantenedor; container com digest pinado no túnel). Pins completos em `infra/ci/woodpecker/versions.json` (removido pela ADR-014; o arquivo segue no histórico Git). Hostname `https://ci-cd-vynamp.jkca.lat`, túnel nomeado `vynamp-woodpecker` (`c423877c-88bf-4bda-a0a0-a2561cf08c59`); túneis preexistentes da conta não foram alterados.

## Resultados reais (Woodpecker instalado, webhook GitHub)

| # | Evento / SHA | Resultado | Status GitHub | Arquivos |
|---|---|---|---|---|
| 1 | push `72094aa8e2a19ddceaacab0f443cb7586d8de2c6` (esta branch) | success: linux e windows, `ci docs`, clippy xtask, `cargo test --workspace`, detect (core ausente, declarado) | `woodpecker-pilot/push/linux` e `/windows` success; combinado success | `pipeline-1*` |
| 2 | push `05f47394270a3ceaad0424a6c8a9ee4da63505a8` (branch descartável `chore/e15-04-pilot-fail`, violação rustfmt deliberada) | failure: `tooling` exit 1 nos dois agentes | failure nos dois contextos | `pipeline-2*` |
| 3 | manual `72094aa`, agente Windows parado | linux success; windows **pending sem agente** até cancelamento → pipeline killed, windows skipped | windows ficou `pending` (nunca success) | `pipeline-3*` |
| 4 | manual `72094aa`, cancelado com `tooling` em execução nos dois | killed nos dois; nenhum processo cargo/rustc nem container de step restante | `error` nos dois contextos | `pipeline-4*` |
| 5 | push `9507e1eec88412305e70b7c2c89d9f8e1b0c2f6f` | success linux e windows | `woodpecker-pilot/push/*` success | `pipeline-5*` |
| 6 | pull_request PR #16, `ref=refs/pull/16/merge`; `git rev-parse HEAD` = `9507e1e` (head, não merge commit) | success linux e windows | `woodpecker-pilot/pr/*` success; GitHub Actions do mesmo PR falhou por billing (job não iniciado) | `pipeline-6*` |
| 9 | push `71f708680d73f5598562873fe534867ccd60debc` (migração Woodpecker-only) | success: android, linux, windows e gate (gate imprime `NOT VERIFIED - macOS`) | `woodpecker/push/{android,linux,windows}` success; `CI Gate` success (link #9/2) | `pipeline-9*` |
| 10 | pull_request PR #16, `ref=refs/pull/16/merge`; HEAD testado `71f7086` | success: android, linux, windows e gate | `woodpecker/pr/*` success; `CI Gate` success (link final #10/2, sobrescreveu #9/2); combinado success | `pipeline-10*`, `github-status-71f7086.json` |
| 11 | push `fda34b023271ba57e432e9d8ac000f7da86b77c8` (branch descartável `chore/e15-04-gate-fail` a partir de `71f7086`, violação rustfmt deliberada em `xtask/src/main.rs`) | failure: linux e windows `tooling` exit 1 (`ci docs`), android success, gate **skipped** (não executou) | `CI Gate` **pending** "Pipeline was skipped" (nunca success); linux/windows failure; combinado failure. Branch removida após a coleta | `pipeline-11*`, `github-status-fda34b0.json` |

Comandos nos steps: ver `.woodpecker/` (`linux`, `windows`, `android`, `gate`, `release`; removido pela ADR-014, o conteúdo segue no histórico Git). Pipelines #1–#8 publicavam `woodpecker-pilot/<evento>/<workflow>`; desde #9 o servidor usa `WOODPECKER_STATUS_CONTEXT_FORMAT` e publica o workflow `gate` como `CI Gate` e os demais como `woodpecker/<evento>/<workflow>`. Logs de #9–#11 exportados pela API local do servidor (token só em `cli.env`); log do gate de #11 não existe (step não iniciou). Cada log de tooling imprime `tested sha=<CI_COMMIT_SHA> ref=<CI_COMMIT_REF>` e `git rev-parse HEAD`.

## Outras verificações

- Webhook sem assinatura, com assinatura falsa e com `access_token` inválido via túnel: HTTP 400 nos três (`failure to parse token from hook`); nenhuma pipeline criada.
- API anônima via túnel (`/api/repos/1`, `/pipelines`, `/logs`, `/api/user`): 401.
- `https://ci-cd-vynamp.jkca.lat/healthz` 204; porta 9000 pelo hostname: sem resposta (gRPC não exposto). Portas publicadas só em 127.0.0.1.
- Repo no Woodpecker: private, não trusted (network/volumes/security false), `require_approval=forks`, `WOODPECKER_OPEN=false`, admin/owner `jimmykedlyn`.
- Persistência: `Stop-Woodpecker.ps1` + `Start-Woodpecker.ps1` (todos os containers parados) → repo ativo, pipelines #1–#4 preservadas, agentes reconectados, túnel 204.
- Exit code PowerShell (local, `woodpecker-cli exec`): sem `$PSNativeCommandUseErrorActionPreference` um `cmd /c exit 7` intermediário é mascarado (exit 0); com ele, step falha (exit 1). Por isso é o primeiro comando do pipeline Windows.
- Execução local prévia (`woodpecker-cli exec`, clones descartáveis, `b5bee8c`): Windows success após mover workspace para caminho curto (primeira tentativa falhou com `LNK1104` por MAX_PATH); Linux success.
- Revisão final do autor: no Linux `cargo xtask ci detect | tee` mascarava falha do detect (`sh -e` sem pipefail; provado na imagem rust pinada: pipe exit 0, redirecionamento exit 1). Corrigido para redirecionamento. No Windows a forma com pipe falha corretamente (provado com `woodpecker-cli exec`).
- Segredos: `server.env`/`agent.env`/`cli.env`/credenciais do túnel só em `D:\Projects\vynamp-media-player\.cache\woodpecker\secrets` (ACL só do usuário); preflight varre arquivos versionados e estes logs sem achados.

## Limitações e pendências

- Login de um usuário GitHub não autorizado não foi testado (sem segunda conta); proteção por `WOODPECKER_OPEN=false` + `WOODPECKER_REPO_OWNERS` configurada, não comprovada com recusa real.
- Após cancelar uma pipeline com workflow ainda pendente (#3), o contexto Windows permaneceu `pending` no GitHub (Woodpecker não publicou estado final do workflow skipped). Não vira success, mas não é atualizado.
- Backend Local não isola steps: herdam o ambiente do agente, incluindo `WOODPECKER_AGENT_SECRET`. Tokens da sessão interativa são removidos antes de iniciar o agente.
- Agente Windows e stack não iniciam sozinhos após reboot do Windows (sem serviço/tarefa agendada); Compose usa `restart: unless-stopped` enquanto o Docker Desktop estiver ativo.
- `CI Gate` é um único contexto por SHA, compartilhado pelos eventos push e pull_request: vale o último estado publicado. No #10 o gate do PR sobrescreveu o do push; se um evento posterior falhar, o gate pulado publica `pending` (provado no #11) e o contexto deixa de ser success. Push e PR testam o mesmo HEAD (`refs/pull/N/merge` faz checkout do head, não do merge commit).
- Status do commit de evidência final (que só altera docs) é registrado no handoff do guia/PR, não neste arquivo, para não mudar o SHA revisado.
- Release (`.woodpecker/release.yaml`, tag `v*`): só lint/preflight local; nenhuma tag criada. Fail-closed por desenho: build/assinatura/publicação distribuídos não migrados.
- Branch protection indisponível no plano atual do repositório privado: `CI Gate` é publicado mas não é obrigatório para merge.
- macOS Intel/Apple Silicon: não verificado (sem agente). Android: o workflow roda, mas sem fontes Android só declara ausência (não aprova app). Hardware: quality-gate manual até E15-05.
- GitHub Actions removido (`.github/workflows/` e Dependabot `github-actions`); `check-delivery` recusa reintrodução.
