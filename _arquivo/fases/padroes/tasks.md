# TEMPLATE — TASK

> Substitui `repo/tasks/TASK-TEMPLATE.md` (0.7.0), que exige Given/When/Then em prosa
> e nenhuma das 191 tarefas cumpre. Aqui o critério é campo de tabela, não parágrafo.
>
> Arquivo: `<repo>/tasks/items/TASK-<EPIC>-<NN>.md`.
> O `backlog.json` do umbrella continua sendo a fonte de status — o Markdown não.

---

# TASK-<EPIC>-<NN> — <título com resultado concreto>

| Campo | Valor |
|---|---|
| ID | `TASK-<EPIC>-<NN>` |
| Módulo dono | `vynamp-<nome>` (exatamente um) |
| Repo | `jimmykedlyn/vynamp-<nome>` |
| REQ coberto | `REQ-E01-01` (um ou mais) |
| Critérios da spec | `<MOD>-AC-01`, `<MOD>-AC-02` |
| Fase | M0–M5 |
| Prioridade | P0 / P1 / P2 |
| Estado | planned / in_progress / implemented / verified / accepted / blocked |
| Depende de | `TASK-...` (IDs, não links soltos) |
| Bloqueia | `TASK-...` |
| Modelo executor | Opus (alto risco: áudio/RT, concorrência, segurança, release) / Sonnet |
| Esforço | X–Y dias úteis (estimativa, não promessa) |
| Risco | alto / médio / baixo |

## Entrega

O artefato concreto que passa a existir. Não é "implementar X": é o que se pode
apontar e verificar.

## Boundary

O que esta TASK **não** faz, mesmo que pareça relacionado. Impede que a tarefa
cresça durante a execução.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-<id>-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

Obrigatório: pelo menos um, em Given/When/Then, observável de fora.
Erro e cancelamento entram como linha própria quando aplicável.

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-<EPIC>-<NN>-AC-1` | ... | ... | ... |
| `TASK-<EPIC>-<NN>-AC-2` | ... | ... | ... |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test -p <crate> <filtro>` | verde no run do CI |
| Oráculo independente | ... | comparação numérica, não "parece certo" |
| Hardware | ... | ausência de hardware é limitação, não aprovação |

Evidência vai em `docs/evidence/PR/<task>/` e no corpo do PR. Não em painel, não em
handoff.

## DoD — pronto para review

- [ ] Todos os critérios de aceite com evidência
- [ ] `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` limpos
- [ ] `cargo test` verde; testes ignorados listados com motivo
- [ ] Spec do módulo atualizada se a implementação revelou algo novo
- [ ] `backlog.json` com o estado correto
- [ ] Revisão feita por contexto limpo, não pelo implementador
- [ ] PR aberto com template preenchido; CI Gate verde no SHA final

## Notas de execução

Preenchido durante a execução, não antes. Decisões tomadas, desvios do plano,
limitações encontradas.
