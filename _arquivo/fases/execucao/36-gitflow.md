# Passo 36 — gitflow

Instala o gitflow nos 10 repos com código: hooks locais que bloqueiam commit em
`main`/`develop`, `.gitignore` de worktree, e as branches `feature/*` já existentes
deixadas como estão. É o passo que transforma o "gitflow" em regra executável.

---

## 1. O que existe hoje

| Repo | Hooks | `core.hooksPath` | `.worktrees/` ignorado |
|---|---|---|---|
| `vynamp` (umbrella, hoje `vynamp-mediaplayer`) | `.githooks/pre-commit`, `.githooks/pre-push` | `.githooks` | sim (`.gitignore:1`) |
| `vynamp-decode` | **nenhum** | não | não |
| `vynamp-output` | **nenhum** | não | não |
| `vynamp-dsp`, `-encode`, `-engine` | não existem ainda | — | — |
| `vynamp-player`, `-converter`, `-restore` | não existem ainda | — | — |

O umbrella já tem o gitflow instalado, mas instalado num repo **com um layout só** (o
workspace). Os repos de módulo nascem no passo 35 com um layout diferente: cada um
compila sozinho, e o `xtask` que carrega a regra de branch **não está lá** — ele
vive no umbrella.

Esse é o ponto central deste passo.

## 2. O problema: o hook do umbrella depende do `xtask` que o umbrella tem

O `pre-commit` do umbrella é:

```sh
cargo xtask guard-branch
```

E `guard-branch` (em `xtask/src/repo.rs:216`) faz exatamente o que o gitflow pede:

```rust
ensure!(
    !b.is_empty() && b != "main" && b != "develop",
    "Commit blocked on main/develop/detached HEAD"
);
ensure!(Regex::new(r"^(?:(?:feature|fix|chore|hotfix)/task-e\d{2}-\d{2}-claude-...
```

Dois problemas para os repos de módulo:

1. **`cargo xtask` não existe neles.** `xtask` é membro do workspace do umbrella.
   Num repo de módulo, `cargo xtask` é "no such command".
2. **O regex exige `task-eNN-NN-claude-<slug>`.** Uma branch `feature/task-e19-07-x`
   criada por outro nome passa; uma `feature/audio-resampler` não. A regra é boa
   (branch de TASK tem ID rastreável), mas ela é do umbrella e precisa continuar valendo
   nos repos de módulo.

**Decisão:** os repos de módulo recebem o **mesmo hook, portado**, com a guarda
embutida — sem `cargo xtask`, sem dependência do umbrella. O hook é um shell de
~15 linhas que faz a mesma checagem em POSIX. Manter as duas implementações em
sincronia é um custo; o preço de não ter a guarda no repo onde o commit acontece é
maior, porque o guard do umbrella só roda quando o commit acontece no umbrella.

## 3. Decisões deste passo

| # | Decisão | Por quê |
|---|---|---|
| **D-GF-1** | Os 8 repos de módulo recebem `.githooks/pre-commit` **autocontido**, com a regra de branch embutida em shell. | `cargo xtask` não existe no repo de módulo. Um hook que chama um binário ausente simplesmente não roda — e hook que não roda parece proteção. |
| **D-GF-2** | A regex do guard é a **mesma** do `xtask/src/repo.rs:225`, copiada literalmente. | Divergir entre as duas cria branch que passa num repo e é bloqueada no outro. A regra é uma só; a implementação é duplicada. |
| **D-GF-3** | `core.hooksPath = .githooks` é configurado em **todos** os 10 repos, e o `pre-commit` é `chmod +x`. | Sem `core.hooksPath`, o `.githooks/` fica inerte. Sem bit de execução, o git ignora o hook. São as duas metades do mesmo requisito. |
| **D-GF-4** | O `pre-push` (hoje `cargo xtask ci docs`) **não** é portado. | Ele valida o índice de docs do umbrella. Num repo de módulo não há índice de docs; a validação equivalente é o CI do GitHub, que já roda `fmt`/`clippy`/`test`. |
| **D-GF-5** | O `.gitignore` de todo repo de módulo ganha `.worktrees/` e `target/`. | O `xtask repo worktree` cria `.worktrees/<agente>/`; sem a entrada, o worktree entra no commit. |
| **D-GF-6** | **Nenhuma branch é criada, movida ou apagada.** As branches de feature que já existem (`feature/mpeg`, `feature/core`, `feature/task-e19-03-claude-dissolve-vitrola`…) ficam como estão. | Merge é do mantenedor. O passo 36 instala a regra; não aplica a regra ao histórico alheio. |
| **D-GF-7** | **Nenhuma proteção de branch é configurada no GitHub.** O passo deixa um relatório do estado atual. | Proteção de branch é decisão do mantenedor (`04-repos-e-remotes.md` §7). `gh api -X PUT .../protection` é exatamente a chamada proibida. |
| **D-GF-8** | O script **não** roda `git branch --delete` nem `git push --delete` nas branches órfãs. | 7 branches `dependabot/*` paradas no umbrella e várias `feature/*` são lixo, mas limpeza de branch remoto é push — proibido ao agente. O relatório lista as candidatas. |

## 4. O hook

```sh
#!/bin/sh
# Regra de Gitflow local: commit so em branch de trabalho.
#
# Portao em shell, sem dependencia do umbrella: 'cargo xtask guard-branch'
# nao existe num repo de modulo, e um hook que falha em rodar nao protege
# nada. A regex e a mesma de xtask/src/repo.rs:225.
set -eu

branch="$(git branch --show-current)"

if [ -z "$branch" ]; then
    echo "Commit bloqueado: HEAD destacado. Crie uma branch de trabalho." >&2
    exit 1
fi

if [ "$branch" = "main" ] || [ "$branch" = "develop" ]; then
    echo "Commit bloqueado em $branch. Trabalhe em feature/*, fix/*, chore/* ou hotfix/*." >&2
    exit 1
fi

if ! echo "$branch" | grep -Eq '^(feature|fix|chore|hotfix)/task-e[0-9]{2}-[0-9]{2}-claude-[a-z0-9]+(-[a-z0-9]+)*$|^chore/claude-[a-z0-9]+(-[a-z0-9]+)*$|^release/[0-9]+\.[0-9]+\.[0-9]+([.-][a-z0-9.-]+)?$'; then
    echo "Commit bloqueado: branch fora do Gitflow: $branch" >&2
    echo "Use feature/task-<id>-<slug>, fix/..., chore/..., hotfix/... ou release/X.Y.Z." >&2
    exit 1
fi
```

O `grep -E` é o mesmo regex do Rust, escrito em POSIX ERE.

**Por que `git branch --show-current` e não `git rev-parse --abbrev-ref HEAD`:** o
`rev-parse` **falha com exit 128** em branch que ainda não tem commit ("ambiguous
argument 'HEAD'"), e com `set -eu` o hook morre antes de imprimir a mensagem — o
commit é bloqueado, mas o motivo não aparece. Pior: num repo recém-criado, o primeiro
commit nunca passaria. `git branch --show-current` responde `develop` no branch
não-nascido (exit 0) e devolve **string vazia** em HEAD destacado, que é exatamente
o que o `if ( -z )` precisa. Medido nos dois casos.

O comportamento do hook, medido: `develop` bloqueia (inclusive sem commit),
`feature/task-e01-01-claude-teste` passa, `feature/resampler` bloqueia,
`chore/claude-limpeza` passa, detached bloqueia.

## 5. O que este passo **não** faz

- Não configura proteção de branch no GitHub (D-GF-7).
- Não cria, renomeia, apaga ou faz push de branch (D-GF-6, D-GF-8).
- Não mergeia nada. Não tageia nada.
- Não altera visibilidade, assinatura ou plano.
- Não mexe em `vynamp-mvp-reference`: é read-only, sem hooks, sem branch de trabalho.

## 6. Verificação

```bash
# 1. Todo repo de modulo tem hook Path configurado
for r in dsp decode encode output engine player converter restore; do
  git -C D:/Projects/vynamp-$r config core.hooksPath   # esperado: .githooks
done

# 2. O hook bloqueia commit em develop
git -C D:/Projects/vynamp-dsp switch develop
git -C D:/Projects/vynamp-dsp commit --allow-empty -m teste   # esperado: bloqueado

# 3. O hook aceita branch de TASK valida
git -C D:/Projects/vynamp-dsp switch -c feature/task-e01-01-teste
git -C D:/Projects/vynamp-dsp commit --allow-empty -m teste   # esperado: passa

# 4. O hook bloqueia branch sem ID de TASK
git -C D:/Projects/vynamp-dsp switch -c feature/resampler
git -C D:/Projects/vynamp-dsp commit --allow-empty -m teste   # esperado: bloqueado
```

Os testes 2–4 rodam em repos **de teste**, nunca nos repos reais: um `--allow-empty`
em `develop` de verdade cria lixo que o mantenedor teria de limpar.

O script **não** roda os testes 2–4: os efeitos colaterais de um hook testado no repo
errado (commit vazio, branch órfã) custam mais que o que o teste prova. Ele verifica a
**presença, o conteúdo e o bit de execução** do hook, e o `core.hooksPath`. O
comportamento foi provado num repo descartável, com os 6 casos acima, antes de este
documento ser escrito.

## 7. Commit

```
chore(gitflow): instala a regra de branch nos repos de modulo (passo 36)

- .githooks/pre-commit autocontido em shell, sem dependencia do umbrella (D-GF-1)
- regex de branch identica a xtask/src/repo.rs:225 (D-GF-2)
- core.hooksPath = .githooks em todos os repos de codigo (D-GF-3)
- .gitignore ganha .worktrees/ e target/ (D-GF-5)
- nenhuma branch criada, movida ou apagada (D-GF-6)
- nenhuma protecao de branch configurada no GitHub (D-GF-7)
```

## 8. Risco

| Risco | Mitigação |
|---|---|
| Hook divergir do `xtask` e bloquear branch legítima | a regex é cópia literal; a divergência aparece no primeiro commit bloqueado, e a correção é uma linha em dois lugares |
| Hook que não roda por falta de bit de execução | o script verifica e corrige o bit; o commit de setup valida que o hook dispara |
| `cargo xtask guard-branch` continuar só no umbrella | o umbrella mantém o `xtask`; o port em shell cobre os módulos. Os dois checam a mesma regra |
| Limpeza de branch órfã vir "de graça" | D-GF-8: o script só **relata**. Apagar branch remoto é push |
|和保护配置 | D-GF-7: nenhum `PUT` em `branches/*/protection`. O relatório é só leitura |
