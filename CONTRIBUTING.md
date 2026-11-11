# Contributing / Contribuições

Versão 0.10.1 · 2026-11-10 · Estado: contrato vigente; o player ainda não está implementado.

## English

### Before you start

Pick a `TASK-*` from [`tasks/backlog.json`](tasks/backlog.json) and open its spec
in [`tasks/items/`](tasks/items/). Read the requirement it traces to in the
[matrix](product/03-requisitos.md), and any ADR that constrains it. If the
task does not exist, the change is not ready to be written — the backlog is the
unit of work, not the diff.

Read [`AGENTS.md`](AGENTS.md) (the working contract), [Gitflow](engineering/03-gitflow.md)
and [CI and gates](engineering/04-ci.md).

### The change

One branch per task, from `develop`, named
`feature|fix|chore/task-e<NN>-<NN>-claude-<slug>` — for example
`feature/task-e19-02-claude-governance-bilingual-readme`. The name is enforced:
`.githooks/pre-commit` runs `cargo xtask guard-branch`, which rejects any commit
on a branch that does not match (`xtask/src/repo.rs`).
Small commits, Conventional Commits (`feat`, `fix`, `test`, `docs`, `chore`,
`build`, `ci`). No force push to a shared branch, no squash, no rebase merge —
merge commits only, to keep ancestry and worktree cleanup reliable.

Then run, in this order:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo xtask check-docs
cargo xtask check-tasks
cargo xtask check-authoring
cargo xtask check-delivery
```

`check-docs` requires every Markdown file to have a `# ` title and an entry in
[`INDEX.md`](INDEX.md). `.claude/` is out of scope: files under it are agent
definitions that open with a YAML frontmatter block, not prose, and they are
configuration rather than specification.

`check-tasks` covers the backlog and the TASK records. `check-authoring` applies
the authoring rules to the same Markdown, and its blocking rule is that a
`Boundary` bullet must name who owns the scope it excludes — a TASK, an ADR or a
document that exists on disk; a bullet with no owner is not verifiable, so the
next task repeats the gap. `check-delivery` validates the agent definitions the
other two skip — the four frontmatter keys, and a `model:` from
`opus|sonnet|haiku` — and the workflows: every `uses:` pinned by full commit SHA,
the `CI Gate` job present and fail-closed.

### The pull request

Open against `develop` with a body that states: **source** (task, requirement,
ADR), **what changed**, **commands and their real output**, and **limitations**
— what you did not run, what remains unverified, and any skipped check. A PR
that claims more than it demonstrated is a defect in this project, not a
marketing choice.

The required status is `CI Gate`. A green gate does not mean the player works:
this repository has no application to build (D-CI-1), so the gate proves the
specs, the task records, the `xtask` tooling, the submodule pins and the
dependency-direction rules — nothing about audio, UI or a binary.

### Rules of evidence

- Do not report a test as executed unless you executed it, and quote the output.
- Do not report a target as verified if the job covering it was skipped.
- Do not commit music, personal data, tokens or signed artefacts.
- Do not introduce code or assets from Winamp or any other player.
- Do not add a rule in two places; state it once and link to it.

### Review and merge

Every change is reviewed in a clean context, by someone other than the author of
the change. Only the maintainer merges, tags and publishes. Approval of the
technical review is not approval of a GitHub identity, and vice versa.

### Licence

Contributions are accepted under `MIT OR Apache-2.0` unless you state otherwise.
See [LEGAL.md](LEGAL.md). Security problems go to
[GitHub Security Advisories](https://github.com/jimmykedlyn/vynamp/security/advisories/new)
— never to a public issue. See [SECURITY.md](SECURITY.md).

## Português

### Antes de começar

Escolha uma `TASK-*` em [`tasks/backlog.json`](tasks/backlog.json) e abra a spec
em [`tasks/items/`](tasks/items/). Leia o requisito correspondente na
[matriz](product/03-requisitos.md) e qualquer ADR que restrinja a
mudança. Se a tarefa não existe, a mudança não está pronta para ser escrita — a
unidade de trabalho é o backlog, não o diff.

Leia [`AGENTS.md`](AGENTS.md) (o contrato de trabalho), o
[Gitflow](engineering/03-gitflow.md) e o
[CI e gates](engineering/04-ci.md).

### A mudança

Uma branch por tarefa, a partir de `develop`, nomeada
`feature|fix|chore/task-e<NN>-<NN>-claude-<slug>` — por exemplo
`feature/task-e19-02-claude-governance-bilingual-readme`. O nome é imposto:
`.githooks/pre-commit` roda `cargo xtask guard-branch`, que recusa qualquer commit
em branch fora do padrão (`xtask/src/repo.rs`). Commits pequenos, Conventional Commits
(`feat`, `fix`, `test`, `docs`, `chore`, `build`, `ci`). Sem force push em branch
compartilhada, sem squash, sem rebase merge — somente merge commit, para
preservar ancestry e a limpeza confiável das worktrees.

Depois rode, nesta ordem:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo xtask check-docs
cargo xtask check-tasks
cargo xtask check-authoring
cargo xtask check-delivery
```

`check-docs` exige que todo arquivo Markdown tenha título `# ` e entrada no
[`INDEX.md`](INDEX.md). `.claude/` fica fora: o que está ali são definições de
agente, que começam com bloco de frontmatter YAML, não prosa, e são configuração,
não especificação.

`check-tasks` cobre o backlog e os registros de TASK. `check-authoring` aplica as
regras de autoria ao mesmo Markdown, e sua regra bloqueante é que um bullet de
`Boundary` nomeie quem é dono do escopo que exclui — uma TASK, uma ADR ou um
documento que exista em disco; bullet sem dono não é verificável, e a TASK seguinte
repete a lacuna. `check-delivery` valida as definições de agente que os outros dois
pulam — as quatro chaves do frontmatter, e `model:` de `opus|sonnet|haiku` — e os
workflows: todo `uses:` fixado por SHA completo de commit, o job `CI Gate` presente
e fail-closed.

### O pull request

Abra contra `develop` com um corpo que declare: **fonte** (tarefa, requisito,
ADR), **o que mudou**, **comandos e a saída real** e **limitações** — o que você
não rodou, o que segue não verificado e qualquer check pulado. Um PR que afirma
mais do que demonstrou é defeito neste projeto, não escolha de marketing.

O status exigido é `CI Gate`. Gate verde não significa que o player funciona:
este repositório não tem aplicação para construir (D-CI-1), então o gate prova
as specs, os registros de tarefa, o tooling do `xtask`, os pins dos submódulos e
as regras de direção de dependência — nada sobre áudio, UI ou binário.

### Regras de evidência

- Não relate um teste como executado se você não o executou; cole a saída.
- Não relate um alvo como verificado se o job que o cobre foi pulado.
- Não commite música, dados pessoais, tokens nem artefatos assinados.
- Não introduza código ou assets do Winamp nem de outro player.
- Não duplique uma regra em dois lugares; declare uma vez e linke.

### Revisão e merge

Toda mudança é revisada em contexto limpo, por alguém que não é o autor da
mudança. Só o mantenedor mergeia, cria tag e publica. A aprovação da revisão
técnica não equivale à aprovação de uma identidade GitHub, e vice-versa.

### Licença

Contribuições são aceitas sob `MIT OR Apache-2.0` salvo declaração em contrário.
Ver [LEGAL.md](LEGAL.md). Problemas de segurança vão para
[GitHub Security Advisories](https://github.com/jimmykedlyn/vynamp/security/advisories/new)
— nunca para issue pública. Ver [SECURITY.md](SECURITY.md).
