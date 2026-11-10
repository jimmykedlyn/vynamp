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
| Estado | planned |
| Depende de | `TASK-E15-01` |
| Bloqueia | `TASK-E11-01`, `TASK-E15-03` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Scripts e regras de isolamento, claim atômico e remoção segura.

## Boundary

- **Não** implementa o worktree: `cargo xtask repo worktree` cria. Esta TASK entrega
  as **regras** de isolamento, claim e remoção segura.
- **Não** remove worktree com mudança não commitada. A AC-1 exige que a worktree
  suja/unmerged seja **preservada**; perder trabalho na remoção é exatamente o que a
  regra impede.
- **Não** libera hotfix antes de `main` **e** `develop`: a AC-2 fixa essa ordem.
- **Não** substitui o claim atômico do Git common dir, que é o mecanismo real de
  exclusividade por TASK.

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
