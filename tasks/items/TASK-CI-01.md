# TASK-CI-01 — Reescrever delivery() e github_actions() para a árvore do umbrella

> Originada de `34-ci.md` §7: "Isso vira **TASK-CI-01** no passo 37". Enquanto ela
> não fechar, `cargo xtask check-delivery` fica vermelho por motivo conhecido.

| Campo | Valor |
|---|---|
| ID | `TASK-CI-01` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | n/a — engenharia, não requisito de produto |
| Critérios da spec | n/a — TASK transversal do umbrella |
| Fase | M1 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | passo 34 (concluído) |
| Bloqueia | nenhuma |
| Modelo executor | Sonnet |
| Esforço | 2–4 dias úteis |
| Risco | médio |

## Entrega

`xtask/src/validate.rs` com `delivery()` e `github_actions()` reescritos para a árvore
do umbrella: validam spec, `xtask`, pins de submódulo e regras de arquitetura, e nada
de empacotamento. `cargo xtask check-delivery` verde com os submódulos do passo 31.

## Boundary

Não reescreve o job de CI (isso é `34-ci.ps1`) nem cria `check-tasks` (TASK-CI-02).
Não toca `check-docs`, que já foi corrigido no passo 34.

## Antes de começar

- [ ] Passo 34 executado e `cargo xtask check-docs` verde
- [ ] Worktree própria criada
- [ ] `cargo test --package xtask` verde antes da mudança (baseline)

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-CI-01-AC-1` | o umbrella sem `docs/` nem `packaging/` | `cargo xtask check-delivery` roda | valida spec, `xtask`, pins de submódulo e regras de arquitetura; nenhuma checagem de empacotamento |
| `TASK-CI-01-AC-2` | os fixtures de `validate.rs:587-730` reescritos | `cargo test --package xtask` roda | os 3 testes de workflow passam com o `ci.yml` do passo 34 |
| `TASK-CI-01-AC-3` | os submódulos do passo 31 no estado semeado | `cargo xtask check-delivery` roda | verde, sem falso positivo de pin |
| `TASK-CI-01-AC-4` | um workflow com `uses:` sem SHA | `check-delivery` roda | reprova e nomeia o arquivo e a linha |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test --package xtask` | verde no CI Gate |
| Gate real | `cargo xtask check-delivery` | verde |
| Oráculo independente | fixture adversária com `uses:` sem SHA | reprovação com arquivo e linha |

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
