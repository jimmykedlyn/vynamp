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
| Estado | implemented |
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

Executada em 2026-11-09; evidência em
[`evidence/PR/task-ci-01/`](../../evidence/PR/task-ci-01/README.md).

- **`AC-1`, `AC-2` e `AC-3` já estavam atendidos pelo passo 34.** Esta TASK ficou
  `planned` no backlog enquanto o código já estava em `develop`. Verificado contra o
  código, não presumido: `check-delivery` verde, 11 testes `github_actions_*` e
  nenhuma checagem de empacotamento no caminho.
- **`AC-4` não estava atendido, e a diferença é material.** A mensagem era
  `Unpinned action <ref> in <stem>`: o *file stem* (`ci`, não `ci.yml`) e **sem
  linha** — o AC pede arquivo e linha. Corrigido com `action_ref_locations()`, que
  carrega o caminho YAML de cada `uses:`, e `uses_lines()`, que casa a n-ésima
  referência com a n-ésima linha do documento.
- **Por que a linha não pode vir da busca pelo texto da ação.** O `ci.yml` repete
  `actions/checkout@<sha>` em cinco jobs; procurar por texto dá a todas as
  ocorrências a linha da primeira, que é o oposto do que o AC pede. Um teste exige
  que cada ocorrência repetida receba a sua própria linha.
- **`uses:` também aparece como primeiro item de lista** (`- uses: …`), não só como
  chave de mapeamento. A varredura de linhas cobre as duas formas, e foi um teste
  vermelho que expôs isso.
