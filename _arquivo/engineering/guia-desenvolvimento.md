# Guia de desenvolvimento — passo a passo

Versão 0.8.0 · 2026-11-11

> **Sobre este arquivo.** Ele substitui o antigo "Reinício Windows" (bootstrap 0.7), que
> ficava em `D:\Projects\vynamp-media-player`. Essa pasta foi apagada em 2026-11-11 e o
> histórico está em `_arquivo/media-player/`. O nome do arquivo foi mantido para não
> quebrar links.

---

## Visão geral

O fluxo inteiro, do zero até a entrega:

| # | O que fazer | Quando | Seção |
|---|---|---|---|
| 1 | Preparar a máquina | Uma vez por máquina ou depois de um clone novo | [§2](#2-preparar-a-máquina) |
| 2 | Escolher a TASK | Sempre que for começar algo | [§3](#3-escolher-a-task) |
| 3a | Implementar uma TASK de **módulo** (lib ou app) | O caso mais comum | [§4](#4-task-de-módulo-o-caso-comum) |
| 3b | Implementar uma TASK do **umbrella** (doc, `xtask`, CI) | Quando o entregável não é código de módulo | [§5](#5-task-do-próprio-umbrella) |
| 4 | Rodar os gates e abrir o PR do umbrella | Antes de todo PR no umbrella | [§6](#6-gates-antes-do-pr-do-umbrella) |
| 5 | Atualizar a máquina depois do merge | Depois de cada merge | [§7](#7-depois-do-merge) |

As regras que valem em qualquer etapa estão em [§8](#8-regras-que-nunca-mudam) e os
erros mais comuns em [§9](#9-problemas-comuns).

### Termos usados neste guia

- **Umbrella**: o repositório central, em `D:\Projects\vynamp`. Guarda docs, specs,
  TASKs e evidências, e aponta para os submódulos.
- **Submódulo**: cada biblioteca e cada app é um repositório Git próprio, montado dentro
  do umbrella.
- **Repinar**: no umbrella, apontar um submódulo para um commit novo (normalmente o merge
  commit do PR do módulo).
- **Gate**: verificação automática que precisa passar antes de abrir o PR.
- **Evidência**: log da suíte + `EVIDENCE.md`, guardados em `evidence\PR\<task>\`.

---

## 1. Onde fica cada coisa

Tudo acontece em **`D:\Projects\vynamp`**. Não existe outra raiz.

| Caminho | O que é | Para onde vai o PR |
|---|---|---|
| `D:\Projects\vynamp` | Umbrella: docs, specs, `tasks\`, `evidence\`, `xtask` | `jimmykedlyn/vynamp` |
| `libs\decode`, `libs\output`, `libs\dsp`, `libs\encode`, `libs\engine` | Bibliotecas (submódulos) | Repo do módulo (`vynamp-decode`, …) |
| `apps\player`, `apps\converter`, `apps\restore` | Aplicativos (submódulos) | Repo do app (`vynamp-player`, …) |
| `reference\vynamp-mvp` | MVP web de referência — **somente leitura** | Nenhum |
| `_arquivo\` | Histórico (STATE.md, fases, media-player) | Só para registrar |

---

## 2. Preparar a máquina

Use um **PowerShell normal** (sem administrador).

### 2.1 Baixar o código e instalar os hooks

```powershell
cd D:\Projects\vynamp
git switch develop
git pull --ff-only                         # atualiza sem criar merge local
git submodule update --init --recursive    # baixa libs\ e apps\
sh setup.sh                                # instala os hooks do umbrella
# instala os hooks de cada submódulo que tiver setup.sh:
git submodule foreach --quiet 'if [ -f setup.sh ]; then sh setup.sh >/dev/null && echo "$name ok"; fi'
```

O que os hooks fazem: o `pre-commit` **recusa** commit nas branches `main` e `develop` e
recusa branches com nome fora do padrão (ver [§4.4](#44-nomes-de-branch)).

### 2.2 Login no GitHub

```powershell
gh auth login
```

Basta fazer uma vez; a credencial fica guardada no keyring do sistema.

### 2.3 Pasta temporária (obrigatório antes de qualquer `cargo`)

Os testes do `xtask` criam repositórios Git temporários. Se a pasta TEMP estiver
**dentro** de um repositório, eles falham com `Nested repository refused`. Por isso,
aponte o TEMP para um disco com espaço e **fora de qualquer repositório**:

```powershell
$env:TEMP = 'D:\Temp\vynamp'; $env:TMP = $env:TEMP
New-Item -ItemType Directory -Force $env:TEMP | Out-Null
```

> ⚠️ Essas variáveis valem só para a janela atual do PowerShell. Ao abrir outra janela,
> rode de novo.

Depois, confira se está tudo certo:

```powershell
cargo xtask doctor     # diagnóstico do ambiente
cargo xtask ci docs
```

### 2.4 ffmpeg (opcional — só para quem mexe em fixtures)

O ffmpeg é usado **apenas em desenvolvimento**, como oráculo: para gerar fixtures e
comparar resultados de decode. A cópia antiga ficava na pasta apagada, então é preciso
baixar de novo:

1. Baixe o ffmpeg para `D:\Projects\vynamp\.cache\tools\ffmpeg\` (essa pasta é ignorada
   pelo Git).
2. Exponha o caminho:

   ```powershell
   $env:FFMPEG = 'D:\Projects\vynamp\.cache\tools\ffmpeg\bin\ffmpeg.exe'
   ```

Os scripts em `libs\decode\tools\fixtures\` usam `$env:FFMPEG` ou o parâmetro `-FFmpeg`.

---

## 3. Escolher a TASK

1. Abra `tasks\NEXT.md`: ele diz qual TASK pegar.
   O status oficial de todas fica em `tasks\backlog.json`.
2. Leia a TASK:
   - TASKs do umbrella ficam em `tasks\items\`;
   - TASKs de módulo ficam dentro do submódulo
     (ex.: `libs\decode\tasks\items\TASK-E01-01.md`).
3. Veja onde ela vive e siga a seção certa:
   - entregável é código de uma lib ou app → [§4](#4-task-de-módulo-o-caso-comum);
   - entregável é doc, `xtask` ou CI do umbrella → [§5](#5-task-do-próprio-umbrella).

> **Hoje a próxima é `TASK-E11-02`** — fronteiras de segurança — no umbrella. Para
> código de módulo, a primeira da fila é `TASK-E01-03`, state machine e generation
> cancellation, em `libs\engine`.

**Para pedir ao Claude:** abra o Claude Code em `D:\Projects\vynamp` e escreva
`execute a TASK-E11-02`. As regras que ele segue estão em `AGENTS.md`.

---

## 4. TASK de módulo (o caso comum)

O código vive **no repositório do submódulo**. O umbrella só é atualizado no fim, para
apontar para o código novo (repinar).

São três etapas: **A** implementar no módulo → **B** fazer o merge no módulo →
**C** repinar no umbrella.

### 4.1 Etapa A — implementar no submódulo

Exemplo com `libs\decode`:

```powershell
cd D:\Projects\vynamp\libs\decode
git switch develop
git pull --ff-only
git switch -c feature/task-e01-01-claude-<slug>

# ... implementar com TDD (teste primeiro, depois o código) ...

cargo fmt --check                                              # formatação
cargo clippy --workspace --all-targets --locked -- -D warnings # lint, sem warnings
cargo test --workspace --locked                                # testes
git push -u origin HEAD
```

### 4.2 Etapa B — PR e merge no módulo

1. Abra o PR no repo do módulo (`jimmykedlyn/vynamp-decode`) com base **`develop`**.
2. Espere o CI do módulo ficar verde.
3. Faça o merge com **merge commit** (nunca squash, nunca rebase).

### 4.3 Etapa C — repinar no umbrella

1. No umbrella, crie a branch `chore/claude-repin-decode-<slug>`.
2. Atualize o submódulo para o **merge commit** da etapa B.
3. Atualize o status em `tasks\backlog.json`.
4. Grave a evidência em `evidence\PR\task-e01-01\`.
5. Rode os gates ([§6](#6-gates-antes-do-pr-do-umbrella)).
6. Abra o PR em `jimmykedlyn/vynamp`.

### 4.4 Nomes de branch

O hook recusa qualquer nome fora destes padrões:

| Padrão | Exemplo |
|---|---|
| `{feature,fix,chore,hotfix}/task-eNN-NN-claude-<slug>` | `feature/task-e01-01-claude-gapless` |
| `chore/claude-<slug>` | `chore/claude-repin-decode-gapless` |
| `release/X.Y.Z` | `release/0.9.0` |

---

## 5. TASK do próprio umbrella

Para TASKs cujo entregável é doc, `xtask` ou CI do umbrella, trabalhe numa **worktree
isolada**, criada a partir do checkout central:

```powershell
cd D:\Projects\vynamp

# 1. criar a worktree (--kind é opcional: fix, chore ou hotfix)
cargo xtask repo worktree --agent claude --task TASK-EXX-NN --slug <slug> [--kind fix|chore|hotfix]

# 2. trabalhar em .worktrees\claude\task-exx-nn-<slug>\

# 3. ao terminar, remover a worktree
cargo xtask repo remove-worktree --agent claude --task TASK-EXX-NN
```

> A worktree **não pode** ser criada dentro de um submódulo. Para TASK de módulo, use
> [§4](#4-task-de-módulo-o-caso-comum).

---

## 6. Gates antes do PR do umbrella

Rode todos e só abra o PR se passarem:

```powershell
cargo xtask check-docs
cargo xtask check-tasks
cargo xtask check-authoring
cargo xtask check-delivery
cargo xtask check-secrets
```

- No GitHub, o check agregado **CI Gate** precisa passar.
- A branch `develop` tem `enforce_admins`: nem administrador consegue pular as regras.
- A evidência fica em `evidence\PR\<task>\` (log da suíte + `EVIDENCE.md`).

---

## 7. Depois do merge

1. Atualize a máquina:

   ```powershell
   cd D:\Projects\vynamp
   git switch develop
   git fetch origin
   git merge --ff-only origin/develop
   git submodule update --init --recursive
   ```

2. Confira o PR real no GitHub antes de seguir: estado, branch base e merge commit.
3. Se o PR envolveu uma **decisão de projeto**, registre o que mudou em
   `_arquivo\media-player\STATE.md`.

---

## 8. Regras que nunca mudam

- ❌ Nunca commitar direto em `main` ou `develop` (o hook bloqueia).
- ❌ Nunca usar `git reset --hard`, `git clean -fdx`, `--no-verify` ou force push.
- ❌ Nunca passar token como argumento do `gh`.
- ❌ Nunca alterar `reference\vynamp-mvp` (é somente leitura).
- ✅ Merge sempre com **merge commit** (nada de squash ou rebase).
- ✅ API do GitHub: só REST.
- ✅ TEMP sempre fora de qualquer repositório Git antes de rodar `cargo`.

---

## 9. Problemas comuns

| Sintoma | Causa provável | O que fazer |
|---|---|---|
| `Nested repository refused` nos testes do `xtask` | TEMP aponta para dentro de um repositório | Refazer [§2.3](#23-pasta-temporária-obrigatório-antes-de-qualquer-cargo) |
| O commit é recusado pelo hook | Você está em `main`/`develop` ou o nome da branch está fora do padrão | Criar branch com nome válido ([§4.4](#44-nomes-de-branch)) |
| A worktree não é criada | Você está dentro de um submódulo | Para módulo, usar [§4](#4-task-de-módulo-o-caso-comum) |
| Scripts de fixture não acham o ffmpeg | `$env:FFMPEG` não definido nesta janela | Refazer [§2.4](#24-ffmpeg-opcional--só-para-quem-mexe-em-fixtures) |
