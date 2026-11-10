# TASK-E10-10 — Fechar gates e cobertura desktop

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E10-10` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E10-10` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E10-08`, `TASK-E10-09`, `TASK-E02-10`, `TASK-E07-10`, `TASK-E14-06` |
| Bloqueia | `TASK-E11-09`, `TASK-E15-10` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | médio |

## Entrega

Matriz de evidência REQ/TASK/QA e relatório de problemas conhecidos.

## Boundary

- **Não** executa o soak de 8 h, o desplugue de DAC nem a captura transparente. É `TASK-E10-08`, que esta TASK consome; rodar de novo aqui faria o fechamento do gate depender de mais 8 h de máquina e o relatório citaria dois números para o mesmo cenário.
- **Não** executa os drills de kill, migration, update e vazamento de token. É `TASK-E10-09`; aqui só se lê o resultado e se decide se ele basta para o requisito, e executar o drill de novo transformaria a leitura da evidência em reautoria dela.
- **Não** agrega a matriz dos cinco alvos nem substitui o aceite por plataforma. É `TASK-E14-06`; reduzir a 1.0 à cobertura desktop é exatamente a confusão que aquela TASK existe para impedir, e o gate daqui é o de desktop.
- **Não** valida as jornadas desktop, a acessibilidade nem o ajuste de interação. É `TASK-E02-10`; ela entrega o relatório de jornadas que esta matriz cita, e refazer as jornadas aqui duplicaria o escopo de QA-010.
- **Não** audita o isolamento das extensões nem as permissões de plugin. É `TASK-E07-10`; QA-009 entra na matriz como linha herdada, com o veredito dela, não como trabalho desta TASK.
- **Não** prepara o release candidate, os artefatos assinados nem o plano canary. É `TASK-E11-09`, que esta TASK bloqueia; fechar o gate de M3 não autoriza publicar, e `release_enabled` continua false em `config/project.json`.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e10-10-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E10-10-AC-1` | os 197 requisitos de `product/03-requisitos.md`, 191 deles com fase M0 a M5 e 139 em M1 a M3 (22 M1, 37 M2 e 80 M3), e as 199 TASKs do backlog com o campo de estado, hoje 11 em implemented e 188 em planned | cada requisito M1 a M3 é confrontado com a evidência anexada, e o que não tem evidência recebe um registro de mudança de escopo nomeando requisito, motivo e quem aprovou | Aceitar todos os requisitos M1–M3 ou registrar mudança explícita de escopo |
| `TASK-E10-10-AC-2` | o relatório de problemas conhecidos montado a partir das TASKs de fase M1 a M3 e dos relatórios de `TASK-E10-08`, `TASK-E10-09`, `TASK-E02-10`, `TASK-E07-10` e `TASK-E14-06`, com uma severidade atribuída a cada problema | o relatório é triado e a contagem de P0 e P1 conhecidos no caminho crítico é conferida antes de declarar o gate fechado | zero P0/P1 |
| `TASK-E10-10-AC-3` | a matriz de evidência com as linhas de Windows x64, Linux x64, macOS Intel, macOS Silicon, Android arm64 e iOS arm64, em que só Windows tem hardware nesta máquina | o fechamento do gate é tentado e cada linha sem hardware real é conferida no relatório para ver se está marcada como não executada com o motivo, em vez de passed | ausência de hardware não vira teste aprovado. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `cargo xtask check-tasks` sobre a matriz de evidência REQ/TASK/QA escrita em `evidence/`, seguido de `cargo xtask quality-gate` e de `cargo xtask release preflight --version 1.0.0` para mostrar o que o gate ainda recusa | Hoje `check-tasks` responde resultado passed com lista de erros vazia e 199 tasks, e passa com a matriz inexistente, então sozinho ele não prova cobertura; `quality-gate` sai com `Hardware gate blocked: application not implemented` e exit 1, e `release preflight` com `Release disabled: actual player/build/hardware/signing acceptance missing` e exit 1. Matriz REQ/TASK/QA não existe: `evidence/` tem 107 arquivos em 26 diretórios de PR, só `task-e10-01` no épico E10, e `evidence/completeness-audit.json` audita a spec, não a ligação requisito-evidência. Com 188 TASKs em planned e 139 requisitos em M1 a M3, a matriz nasce com 188 linhas sem evidência, e as linhas de Linux, macOS Intel, macOS Silicon, Android e iOS nascem sem hardware. |
| Hardware | nenhum: o gate compara documento, backlog e matriz e não abre dispositivo; o que falta não é hardware deste host para executar a TASK, e sim as evidências de Linux x64, dos dois macOS, Android arm64 e iOS arm64, que entram na matriz como não executadas porque esta máquina tem só Windows 11 x64 | ausência é limitação registrada, não aprovação |

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
