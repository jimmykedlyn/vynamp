# TASK-E19-02 — Higiene, governança e README bilíngue do repo principal

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. Critério de aceite: **PENDENTE** de autoria —
> a coluna `Then` é o requisito aprovado, copiado literal; `Given` e `When` ainda
> não existem.

| Campo | Valor |
|---|---|
| ID | `TASK-E19-02` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E19-02` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M1 |
| Prioridade | P0 |
| Estado | in_progress |
| Depende de | `TASK-E19-01` |
| Bloqueia | `TASK-E19-06` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | médio |

## Entrega

LICENSE dual, LEGAL, CODE_OF_CONDUCT, GOVERNANCE, templates de issue, SECURITY.md com canal de advisory e README bilíngue EN/PT, com docs de CI e Gitflow reescritos para o repo público.

## Boundary

<!-- PENDENTE: o que esta TASK não faz, mesmo parecendo relacionado. -->

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e19-02-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E19-02-AC-1` | a raiz do repositório | o `README.md` é aberto | README bilíngue EN/PT na raiz |
| `TASK-E19-02-AC-2` | o `SECURITY.md` e o recurso de relato privado do repositório | os dois são comparados | SECURITY.md aponta para GitHub Security Advisories e não inventa contato |
| `TASK-E19-02-AC-3` | a raiz do repositório | os arquivos de licença são lidos | LICENSE dual presente |
| `TASK-E19-02-AC-4` | os documentos de governança e as specs | cada afirmação de capacidade ou de alvo é confrontada com a evidência | docs não afirmam capacidade ou alvo não verificado. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | PENDENTE | comparação numérica, não "parece certo" |
| Hardware | PENDENTE | ausência é limitação registrada, não aprovação |

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
