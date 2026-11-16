# Evidência TASK-E15-04 — Workflows e CI Gate multiplataforma

Versão 0.1.0 · 2026-11-15 · Autor Claude (branch
`feature/task-e15-04-claude-ci-gate-multiplataforma`, base `develop` `310a0ad`).
Documento, gate local e configuração de CI; **não** é aceite de app, hardware nem
release.

## Escopo entregue

| Item | Arquivo |
|---|---|
| Job `delivery` roda `check-delivery` no CI | `.github/workflows/ci.yml` |
| Job `android` cobre o quinto alvo sem construir app | `.github/workflows/ci.yml` |
| Matriz `tooling` ganha `windows-2022` | `.github/workflows/ci.yml` |
| `arch` reprova fonte ausente em vez de sair com 0 | `.github/workflows/ci.yml` |
| Agregado `Hardware Gate` no workflow de hardware | `.github/workflows/quality.yml` |
| Regras novas em `github_actions` | `xtask/src/validate.rs` |
| `same_remote` normaliza a URL do `origin` | `xtask/src/validate.rs` |
| Spec atualizada | `engineering/04-ci.md` |

## Critérios de aceite

| AC | Como foi pago | Teste |
|---|---|---|
| AC-1 jobs ativos não mascaram falha | todo job do `ci.yml` tem de estar no `needs:` do `gate`; o gate tem de ler o `result` de cada dependência; a matriz não pode cancelar as pernas restantes | `github_actions_rejects_a_job_outside_the_aggregate`, `github_actions_requires_the_gate_to_read_every_dependency_result`, `github_actions_requires_the_matrix_to_run_to_the_end` |
| AC-2 pins/actions/read permissions | `check-delivery` passa a rodar em job próprio; `permissions: contents: read` e `persist-credentials: false` viram regra | `github_actions_requires_check_delivery_in_ci`, `github_actions_requires_the_read_only_token_policy`, `github_actions_rejects_a_job_that_widens_the_token`, `github_actions_requires_persist_credentials_false` |
| AC-3 skips sem fontes não aprovam | gate compara contra `success`, nunca contra `failure`; `arch` reprova fonte ausente; `Hardware Gate` reprova matriz pulada | `github_actions_rejects_a_gate_that_accepts_skipped_jobs`, `quality_workflow_publishes_a_verdict_that_rejects_skips` |
| AC-4 cinco alvos cobertos | quatro imagens na matriz `tooling` + job `android` | `github_actions_requires_the_five_mandatory_targets` |

## Decisão de escopo: Android sem build

O umbrella não constrói app (D-CI-1, `AGENTS.md` §1.2 regra 1). O job `android` não é
um build: confirma que o alvo Rust `aarch64-linux-android` existe no toolchain pinado e
que `cargo xtask ci detect` diz a verdade sobre haver app para construir. O build
Android de verdade roda no CI do repo do app. A alternativa — pôr um `cargo xtask ci
android` aqui — seria reprovada pelo próprio `check-delivery` e abriria um segundo lugar
onde o produto prova a si mesmo.

## Achado durante a execução

`check-delivery` comparava o `origin` com `==` contra a URL de `config/github.json`, que
leva `.git`. O `actions/checkout` grava o remoto **sem** o sufixo, então o job `delivery`
reprovaria em todo runner enquanto passava numa estação cujo remoto foi clonado à mão. A
comparação passou a ser sobre a identidade do repositório (`same_remote`), com teste que
prova que repositório diferente continua reprovando.

## Comandos e resultado

| Comando | Resultado |
|---|---|
| `cargo fmt --all` | limpo |
| `cargo clippy --package xtask --all-targets --locked -- -D warnings` | limpo |
| `cargo test --package xtask --locked` | 129 + 36 testes, 0 falhas, 0 ignorados |
| `cargo xtask check-delivery` | `{"errors":[],"result":"passed","workflow_files":2}` |
| `cargo xtask ci docs` | `check-docs` e `check-delivery` `passed`; fmt, clippy e testes verdes |

## Sobreposição com TASK-E11-02

A regra de `permissions: contents: read` também está sendo escrita por outra sessão em
`TASK-E11-02` (fronteiras de segurança), como trabalho não commitado no checkout
principal. Esta TASK paga o AC-2 com a mesma regra porque o critério a exige aqui; na
integração, uma das duas cópias sai. O texto da mensagem de erro difere de propósito
(`TASK-E15-04 AC-2` contra `TASK-E11-02 AC-2`) para que a duplicata seja visível no
conflito em vez de se esconder num merge silencioso.

## Limitações

- Nenhum workflow foi executado no GitHub Actions a partir deste ambiente. O push da
  branch e a abertura do PR usam `GITHUB_TOKEN_REPO` (write medido por sonda de ref), mas
  a execução dos jobs acontece só no runner do GitHub. O CI Gate vale pelo SHA final no
  PR, não por esta evidência local.
- Os runners `vynamp-hardware`/`vynamp-android-device` continuam ausentes (TASK-E15-05).
  O `Hardware Gate` agora reprova o run nesse estado, em vez de terminar verde.
- `windows-2022` e `macos-15` não foram executados localmente; a matriz é afirmação de
  configuração, verificada por leitura do YAML, não por execução.

## Correção pós-merge do `Estado`

O PR #67 entrou como `a2f7e40` e o CI Gate ficou verde nos 12 checks. Um defeito
sobreviveu ao merge, e o gate que devia tê-lo pego não existia.

`tasks/items/TASK-E15-04.md` foi mergeado com `| Estado | planned |` enquanto
`tasks/backlog.json` já dizia `implemented`. O próprio cabeçalho do arquivo afirma que
os campos de estado vêm do backlog, e nada conferia: `check-tasks` comparava dono e
critérios de aceite, nunca o `Estado`. O arquivo que um leitor abre para saber o estado
da TASK mentia, e os 12 checks verdes não tinham como notar.

A regra nova compara o `Estado` do Markdown com o `status` do backlog, e só para as
TASKs do umbrella -- o arquivo de um módulo mora no repo do módulo, e corrigi-lo exige
commit lá e repin aqui. Reprovar o umbrella por um arquivo que ele não pode consertar
deixaria o CI vermelho sem saída.

**Verificação.** Com a regra e sem a correção do arquivo, `cargo xtask check-tasks`
responde `{"errors":["TASK-E15-04 is implemented in backlog, planned in Markdown"],
"result":"failed"}`. Com a correção, `passed` em 215 tasks.

**Achado que a regra deixou visível e que não é desta TASK:** `TASK-E01-04`
(`vynamp-decode`, pinado em `9f26d7c`) tem a mesma divergência -- `planned` no arquivo,
`implemented` no backlog do umbrella. Fica registrado, não corrigido: é arquivo de outro
módulo, e o conserto é uma TASK lá mais um repin aqui.
