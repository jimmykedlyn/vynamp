# TASK-E15-04 — Ativar workflows e CI Gate multiplataforma

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E15-04` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E15-04` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M1 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E15-03` |
| Bloqueia | `TASK-E11-04`, `TASK-E15-05`, `TASK-E17-12` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–10 dias úteis |
| Risco | alto |

## Entrega

CI docs/tooling e matriz Cargo/Android, required aggregate context.

## Boundary

- **Não** faz a agregação do gate. O job `gate` e o `needs:` são `TASK-E19-01`; aqui
  se entregam os jobs e o contexto agregado que vira required.
- **Não** deixa job ativo mascarar falha. A AC-1 é literalmente essa regra: um job que
  falha enquanto outro ainda corre não é aprovação.
- **Não** aprova o player por skip. A AC-3 exige que skip por falta de hardware ou de
  fonte **não** aprove o player, e o agregado publica o estado final.
- **Não** cobre iOS. A AC-4 confronta os workflows com a matriz dos cinco alvos; iOS é
  FASE 6 e entra depois do gate.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e15-04-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E15-04-AC-1` | um job que falha e o agregado CI Gate que depende dele | a matriz roda até o fim | Jobs ativos não mascaram falha |
| `TASK-E15-04-AC-2` | todos os workflows no estado do commit | as actions, os pins por SHA e as permissões de token são conferidos | pins/actions/read permissions |
| `TASK-E15-04-AC-3` | um job do player que pula por falta de hardware ou de fonte | o agregado publica o estado final | skips sem fontes não aprovam player |
| `TASK-E15-04-AC-4` | a matriz de alvos obrigatórios da spec de plataformas | os workflows são confrontados com a matriz | cinco alvos cobertos. |

## Verificação
| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `cargo xtask check-delivery` sobre os workflows, confrontando cada `uses:` e cada permissão com a matriz de alvos | todo `uses:` com SHA de 40 hex, `permissions: contents: read`, e cada um dos cinco alvos presente em um job — leitura do workflow, não "parece certo" |
| Hardware | nenhum: o gate agregador roda em runner hospedado; a matriz de hardware é o `quality.yml` do umbrella, separado | ausência é limitação registrada, não aprovação |

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
