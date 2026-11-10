# TASK-E10-02 — Testar domínio e propriedades de fila

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E10-02` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E10-02` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M1 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E01-03`, `TASK-E03-02`, `TASK-E10-01` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–8 dias úteis |
| Risco | médio |

## Entrega

Suite de state machine, comandos aleatórios, ordering e queue invariants.

## Boundary

- **Não** implementa a state machine: ela é o insumo que esta TASK exercita. O que
  se entrega é o gerador de comandos aleatórios e as invariantes que ele prova.
- **Não** mede desempenho. Sequência longa aqui é cobertura de estado, não throughput;
  medir é `TASK-E10-07`.
- **Não** faz fuzzing de parser, pacote ou RPC: isso é `TASK-E10-05`.
- **Não** toca em rede. Toda sequência é local e determinística pela seed registrada,
  porque a AC-1 exige que a seed acompanhe o caso que falha.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e10-02-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E10-02-AC-1` | um gerador de comandos aleatórios sobre a state machine, com a seed registrada | sequências longas rodam e cada evento observado carrega a geração que o produziu | Sequências aleatórias preservam generation e invariantes |
| `TASK-E10-02-AC-2` | as propriedades de fila e o gerador de comandos de AC-1 | as sequências aleatórias cobrem shuffle, history, repeat e stop-after | shuffle/history/repeat/stop-after são exercitados. |

## Verificação
| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `cargo test --package vynamp --locked` sobre a suíte de sequências aleatórias, com a seed impressa no relatório do caso que falha | a mesma seed reproduz a mesma sequência e o evento carrega a geração que o produziu — resultado lido do relatório, não "parece certo" |
| Hardware | nenhum: a state machine é determinística e roda em processo de usuário, sem dispositivo de áudio | ausência é limitação registrada, não aprovação |

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
