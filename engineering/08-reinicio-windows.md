# Guia de desenvolvimento — passo a passo

Versão 0.8.0 · 2026-11-11 · Substitui o antigo "Reinício Windows" (bootstrap 0.7 em
`D:\Projects\vynamp-media-player`, pasta apagada em 2026-11-11; histórico em
`_arquivo/media-player/`). O nome do arquivo ficou para não quebrar links.

## 1. Pasta

Tudo acontece em **`D:\Projects\vynamp`** (o umbrella). Não existe outra raiz.

| Caminho | O que é | Onde vai o PR |
|---|---|---|
| `D:\Projects\vynamp` | umbrella: docs, specs, `tasks/`, `evidence/`, `xtask` | `jimmykedlyn/vynamp` |
| `libs\decode`, `libs\output`, `libs\dsp`, `libs\encode`, `libs\engine` | bibliotecas (submódulos) | repo do módulo (`vynamp-decode`, …) |
| `apps\player`, `apps\converter`, `apps\restore` | aplicativos (submódulos) | repo do app (`vynamp-player`, …) |
| `reference\vynamp-mvp` | MVP web de referência, **read-only** | nenhum |
| `_arquivo\` | histórico (STATE.md, fases, media-player) | só para registrar |

## 2. Preparar (uma vez por máquina, ou depois de um clone novo)

PowerShell normal, sem administrador:

```powershell
cd D:\Projects\vynamp
git switch develop
git pull --ff-only
git submodule update --init --recursive
sh setup.sh
git submodule foreach --quiet 'if [ -f setup.sh ]; then sh setup.sh >/dev/null && echo "$name ok"; fi'
```

`setup.sh` instala os hooks (`pre-commit` recusa commit em `main`/`develop` e nome de
branch fora do padrão). Credencial do GitHub: `gh auth login` uma vez; ela fica no
keyring. Nunca passar token como argumento de `gh`. API do GitHub só REST.

Antes de qualquer `cargo`, apontar o temporário para um disco com espaço e **fora
de qualquer repositório Git** (os testes do `xtask` criam repos temporários e recusam
TEMP dentro de um repo: "Nested repository refused"):

```powershell
$env:TEMP = 'D:\Temp\vynamp'; $env:TMP = $env:TEMP
New-Item -ItemType Directory -Force $env:TEMP | Out-Null
cargo xtask doctor
cargo xtask ci docs
```

**ffmpeg (oráculo de dev):** só para gerar fixtures e comparar decode. A cópia antiga
morava na pasta apagada; baixar de novo para `D:\Projects\vynamp\.cache\tools\ffmpeg\`
(ignorado pelo Git) e expor com
`$env:FFMPEG = 'D:\Projects\vynamp\.cache\tools\ffmpeg\bin\ffmpeg.exe'`.
Os scripts em `libs\decode\tools\fixtures\` leem `$env:FFMPEG` ou `-FFmpeg`.

## 3. Escolher a TASK

1. `tasks\NEXT.md` diz qual abrir; `tasks\backlog.json` tem o status oficial.
2. Ler a TASK. As do umbrella ficam em `tasks\items\`; as de módulo ficam no
   submódulo (ex.: `libs\decode\tasks\items\TASK-E01-01.md`).
3. Hoje a próxima é **`TASK-E11-02`** (fronteiras de segurança), no umbrella; para
   código de módulo, a fila é `TASK-E01-03` no `libs\engine`.

Para pedir ao Claude: abrir o Claude Code em `D:\Projects\vynamp` e escrever
`execute a TASK-E11-02`. As regras que ele segue estão em `AGENTS.md`.

## 4. TASK de módulo (o caso comum)

O código vive no repo do submódulo; o umbrella só repina.

```powershell
cd D:\Projects\vynamp\libs\decode
git switch develop
git pull --ff-only
git switch -c feature/task-e01-01-claude-<slug>
# ... implementar com TDD ...
cargo fmt --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
git push -u origin HEAD
```

1. PR no repo do módulo (`jimmykedlyn/vynamp-decode`), base `develop`.
2. CI do módulo verde → merge com **merge commit** (nunca squash/rebase).
3. No umbrella, branch `chore/claude-repin-decode-<slug>`: atualizar o submódulo
   para o merge commit, atualizar `tasks\backlog.json` e a evidência em
   `evidence\PR\task-e01-01\`, rodar os gates (§6), PR em `jimmykedlyn/vynamp`.

Padrão de nome de branch (o hook recusa o resto):
`{feature,fix,chore,hotfix}/task-eNN-NN-claude-<slug>`, `chore/claude-<slug>` ou
`release/X.Y.Z`.

## 5. TASK do próprio umbrella

Para TASKs cujo entregável é doc, `xtask` ou CI do umbrella, usar uma worktree
isolada, criada do checkout central:

```powershell
cd D:\Projects\vynamp
cargo xtask repo worktree --agent claude --task TASK-EXX-NN --slug <slug> [--kind fix|chore|hotfix]
# trabalha em .worktrees\claude\task-exx-nn-<slug>\
cargo xtask repo remove-worktree --agent claude --task TASK-EXX-NN
```

A worktree recusa ser criada dentro de um submódulo; para módulo, use §4.

## 6. Gates antes do PR do umbrella

```powershell
cargo xtask check-docs
cargo xtask check-tasks
cargo xtask check-authoring
cargo xtask check-delivery
cargo xtask check-secrets
```

No GitHub o agregado **CI Gate** precisa passar; `develop` tem `enforce_admins`.
Evidência fica em `evidence\PR\<task>\` (log da suíte + `EVIDENCE.md`).

## 7. Depois do merge

```powershell
cd D:\Projects\vynamp
git switch develop
git fetch origin
git merge --ff-only origin/develop
git submodule update --init --recursive
```

Verificar o PR real (estado, base, merge commit) antes de seguir. Registrar o que
mudou em `_arquivo\media-player\STATE.md` quando for decisão de projeto. Nunca usar
`git reset --hard`, `git clean -fdx`, `--no-verify` ou force push.
