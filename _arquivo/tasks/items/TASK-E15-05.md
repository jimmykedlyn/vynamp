# TASK-E15-05 — Integrar testes e evidência operacional

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E15-05` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E15-05` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M1 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E11-01`, `TASK-E15-04` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–10 dias úteis |
| Risco | alto |

## Entrega

Unit/integration/numerical/stream/UI/hardware/fuzz e protocolo de relatórios.

## Boundary

- **Não** substitui o gate por relatório. O relatório é o **protocolo** de evidência;
  quem reprova continua sendo o job, e o `gate` é de `TASK-E19-01`.
- **Não** transforma falta de hardware em aprovação. A AC-3 exige que o resultado seja
  publicado como ausência declarada; executar em hardware real por alvo é
  `TASK-E14-04`.
- **Não** junta validação de plugin e de instalador no job de tooling: a AC-4 exige
  separação por tipo, e o job que faz essa separação é `TASK-E15-04`.
- **Não** aceita fixture sem manifesto. A AC-2 exige hash e licença conferidos; fixture
  que existe sem manifesto verificado é decoração, e gerar esse manifesto é
  `TASK-E10-01`.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e15-05-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E15-05-AC-1` | cada TASK executada e o risco que ela declara | o protocolo de relatório de testes é preenchido | Tests reais/comandos/oráculos por risco |
| `TASK-E15-05-AC-2` | cada fixture usada por um teste | o manifesto é conferido quanto a licença e hash | fixture licence/hash |
| `TASK-E15-05-AC-3` | um teste que exige hardware indisponível no CI | o resultado é publicado | hardware ausente não passa |
| `TASK-E15-05-AC-4` | as validações de plugin e as de instalador | os jobs são separados por tipo | plugin/installer validations separados de tooling. |

## Verificação
| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | o manifesto de cada fixture, confrontado com o hash do arquivo e com a licença, mais o protocolo de relatório preenchido TASK a TASK | cada fixture usada por teste tem hash e licença conferidos no manifesto — leitura do manifesto, não "parece certo" |
| Hardware | a AC-3 exige que um teste que depende de hardware indisponível publique o resultado como ausência, sem passar | ausência é limitação registrada, não aprovação |

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
