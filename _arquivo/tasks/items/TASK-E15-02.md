# TASK-E15-02 — Validar worktrees, claims e handoff

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E15-02` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E15-02` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M0 |
| Prioridade | P0 |
| Estado | implemented |
| Depende de | `TASK-E15-01` |
| Bloqueia | `TASK-E11-01`, `TASK-E15-03` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Scripts e regras de isolamento, claim atômico e remoção segura.

## Boundary

- **Não** implementa o worktree: `cargo xtask repo worktree` cria, e esse comando é do
  workspace de `TASK-E11-01`. Esta TASK entrega as **regras** de isolamento, claim e
  remoção segura.
- **Não** remove worktree com mudança não commitada: quem executa a remoção é o
  `cargo xtask repo remove-worktree` do workspace de `TASK-E11-01`. A AC-1 exige que a worktree
  suja/unmerged seja **preservada**; perder trabalho na remoção é exatamente o que a
  regra impede.
- **Não** libera hotfix antes de `main` **e** `develop`: a AC-2 fixa essa ordem, e o
  fluxo de hotfix é ativado por `TASK-E15-03`, dona do Gitflow.
- **Não** substitui o claim atômico do Git common dir, que é o mecanismo real de
  exclusividade por TASK: ele é criado pelo `cargo xtask repo worktree` do workspace de
  `TASK-E11-01`.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e15-02-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E15-02-AC-1` | duas worktrees do mesmo repo, com um claim já tomado por uma TASK e a segunda worktree apontando para o mesmo diretório | a segunda worktree tenta tomar a mesma TASK e depois a remoção da worktree suja é pedida | Índices independentes, task exclusiva, paths válidos, worktree suja/unmerged preservada |
| `TASK-E15-02-AC-2` | uma worktree de hotfix com o merge feito apenas em main | a remoção da worktree é pedida | hotfix só remove após main e develop. |

## Verificação
| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | duas worktrees do mesmo repo disputando o mesmo claim, e depois pedido de remoção de uma worktree suja | a segunda worktree é recusada e a worktree suja é preservada — leitura da recusa, não "parece certo" |
| Hardware | nenhum: isolamento de worktree e claim são de filesystem e índice Git, sem dispositivo de áudio | ausência é limitação registrada, não aprovação |

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

### 2026-11-15 — verificação e registro

A implementação já existia em `develop` desde o commit `247b424` (tooling inicial), mas a
TASK nunca teve o estado atualizado. O mantenedor decidiu registrar como entregue em
2026-11-15, após a evidência abaixo.

**AC-1** — claim exclusivo e preservação de worktree suja/unmerged. Testes que provam, no
checkout central (`310a0ad`), todos verdes:

```
cargo test --package xtask --locked -- \
    concurrent_task_claim_has_one_winner \
    dirty_worktree_is_preserved unmerged_worktree_is_preserved
# 3 passed; 0 failed
```

Implementação correspondente em `xtask/src/repo.rs`: o claim é tomado com
`fs::create_dir` atômico (a segunda sessão recebe `Task already claimed`), e a remoção
preserva a worktree quando o claim está inconsistente, quando a branch diverge do claim ou
quando o HEAD está detached.

**AC-2** — hotfix só remove após `main` **e** `develop`. Testes:

```
cargo test --package xtask --locked -- \
    hotfix_requires_both_main_and_develop hotfix_merged_only_to_main_is_preserved
# 2 passed; 0 failed
```

Suíte inteira do `xtask` no mesmo commit: `118 passed` (unittests) + `36 passed`
(`tests/tooling.rs`), zero falhas.

**Limitação registrada:** sem push/PR neste runner, não há CI Gate pelo SHA. A evidência é
local e os testes são os mesmos que o CI executa.
