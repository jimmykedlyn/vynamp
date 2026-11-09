# TASK-E19-01 — Migrar o CI do repo principal para GitHub Actions

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. Critério de aceite: **PENDENTE** de autoria —
> a coluna `Then` é o requisito aprovado, copiado literal; `Given` e `When` ainda
> não existem.

| Campo | Valor |
|---|---|
| ID | `TASK-E19-01` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E19-01` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M1 |
| Prioridade | P0 |
| Estado | in_progress |
| Depende de | `TASK-E15-03` |
| Bloqueia | `TASK-E19-02`, `TASK-E19-05`, `TASK-E19-06` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–10 dias úteis |
| Risco | alto |

## Entrega

ADR-014, workflows ci/release/quality em GitHub Actions com actions fixadas por SHA e gate único CI Gate, xtask de validação e release reescrito para Actions e remoção do Woodpecker.

## Boundary

<!-- PENDENTE: o que esta TASK não faz, mesmo parecendo relacionado. -->

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e19-01-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E19-01-AC-1` | qualquer workflow em `.github/workflows/` | `cargo xtask check-delivery` lê cada `uses:` | Actions fixadas por SHA |
| `TASK-E19-01-AC-2` | o job `gate` agregando `specs`, `tooling`, `submodules` e `arch` | uma dependência falha, é cancelada ou não existe | gate reprova dependência falha, cancelada ou ausente |
| `TASK-E19-01-AC-3` | o workflow de release | o ref do push não é uma tag `v*`, ou o preflight não passa | release só em tag v* e falha fechado |
| `TASK-E19-01-AC-4` | a matriz do job `tooling` | o CI roda em `macos-15-intel` e `macos-15` | macOS Intel e Apple Silicon viram alvos verificados |
| `TASK-E19-01-AC-5` | o histórico completo do repositório | uma varredura por padrões de segredo roda sobre todos os commits | nenhum segredo no histórico. |

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
