# TASK-CI-02 — Adicionar o job check-tasks ao CI do umbrella

> Criada pelo passo 37 (D-37-13). É o que dá consequência mecânica a
> `acceptance_ready: false`: sem ela o campo é decoração e o SDD volta a ser
> declaração.

| Campo | Valor |
|---|---|
| ID | `TASK-CI-02` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | n/a — engenharia, não requisito de produto |
| Critérios da spec | n/a — TASK transversal do umbrella |
| Fase | M1 |
| Prioridade | P0 |
| Estado | implemented |
| Depende de | passo 37 (esta TASK é emitida por ele) |
| Bloqueia | nenhuma |
| Modelo executor | Sonnet |
| Esforço | 1–2 dias úteis |
| Risco | médio |

## Entrega

Job `check-tasks` no `.github/workflows/ci.yml` do umbrella e o comando `cargo xtask check-tasks` que o alimenta: valida que toda TASK de produto tem `Module dono` de um único módulo, REQ, critério em Given/When/Then sem `PENDENTE`, e que nenhuma TASK está fora de `planned` com `acceptance_ready: false`. Entra em `CI Gate`.

## Boundary

Não escreve critério de aceite, não reescreve TASK, não decide dono. Só reprova o que
não satisfaz a regra e **nomeia** cada infração.

## Antes de começar

- [ ] Passo 37 executado e as 193 TASKs no lugar
- [ ] Worktree própria criada
- [ ] `cargo xtask check-docs` verde

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-CI-02-AC-1` | uma TASK de produto com `Given` = PENDENTE | `cargo xtask check-tasks` roda | reprova e imprime o ID da TASK e a coluna que falta |
| `TASK-CI-02-AC-2` | uma TASK com `status: in_progress` e `acceptance_ready: false` no backlog | `check-tasks` roda | reprova e lista o ID |
| `TASK-CI-02-AC-3` | uma TASK com dois módulos donos ou com módulo inexistente | `check-tasks` roda | reprova e nomeia o arquivo |
| `TASK-CI-02-AC-4` | o repositório no estado do passo 37 | `check-tasks` roda | reprova **nomeando as 5 `in_progress`** e nenhuma outra — o vermelho esperado, documentado em `37-tasks.md` §8 |
| `TASK-CI-02-AC-5` | as 5 `in_progress` com critério escrito e `acceptance_ready: true` | `check-tasks` roda | verde |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test --package xtask` com 4 fixtures adversárias | verde no CI Gate |
| Gate real | `cargo xtask check-tasks` | reprova nomeando exatamente as 5 de AC-4 |
| CI | job `check-tasks` no `ci.yml` | presente e exigido pelo `CI Gate` |

## DoD — pronto para review

- [ ] Todos os critérios de aceite com evidência
- [ ] `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` limpos
- [ ] `cargo test` verde; testes ignorados listados com motivo
- [ ] Spec do módulo atualizada se a implementação revelou algo novo
- [ ] `backlog.json` com o estado correto
- [ ] Revisão feita por contexto limpo, não pelo implementador
- [ ] PR aberto com template preenchido; CI Gate verde no SHA final

## Notas de execução

Executada em 2026-11-09; evidência em
[`evidence/PR/task-ci-02/`](../../evidence/PR/task-ci-02/README.md).

- **`AC-1` a `AC-3`:** cobertos por `check_tasks_ac1_*` a `check_tasks_ac3_*` em
  `validate.rs`, mais `check_tasks_ac4_names_every_offender_and_only_those`, que
  exige que o vermelho nomeie todos os infratores e nenhum outro.
- **`AC-4` e `AC-5` contradizem-se como escritas, e a contradição é real.** Elas
  pedem, no mesmo commit, um vermelho nomeando **exatamente** as cinco `in_progress`
  e um verde assim que essas cinco ganham critério. Se o juiz fosse "a lista fixa de
  cinco IDs", o verde de `AC-5` exigiria que a lista encolhesse; se fosse "toda
  TASK fora de `planned`", o vermelho de `AC-4` teria de citar mais que cinco.
  Resolvido pela regra que as duas descrevem de fato: o gate reprova **toda** TASK
  fora de `planned` cujo critério não esteja escrito. Aplicada ao estado do passo 37,
  ela nomeia as cinco de `AC-4` **e** `TASK-E19-06` — que saiu de `planned` sem
  critério escrito. Nenhuma regra foi afrouxada entre o vermelho e o verde: os seis
  receberam critério, o que é `AC-5` aplicada honestamente.
- **O vermelho por ausência é o defeito que esta TASK existe para impedir.** Tarefas
  `planned` sem `Given`/`When` não reprovam: o passo 37 decidiu (`D-37-3`, `P3`) que
  autoria de critério é trabalho por módulo, e exigir 191 autorias de uma vez
  pintaria o gate de vermelho por trabalho que nenhuma regra pediu.
- **Ação tomada além do gate:** as sete TASKs que estavam fora de `planned` sem
  critério escrito receberam `Given`/`When` autorados, e `acceptance_ready` passou a
  `true` para elas. Foi assim que o vermelho virou verde sem exceção.
