# TASK-E11-04 — Completar CI e supply chain

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E11-04` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E11-04` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M1 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E11-01`, `TASK-E11-03`, `TASK-E14-01`, `TASK-E15-04` |
| Bloqueia | `TASK-E11-06`, `TASK-E11-07`, `TASK-E13-09`, `TASK-E15-06`, `TASK-E15-07`, `TASK-E15-08`, `TASK-E15-09` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–8 dias úteis |
| Risco | médio |

## Entrega

CI Rust e hosts Windows x64/Linux x64/macOS x64+arm64/Android arm64, native dependencies, audits, SBOM e artifacts.

## Boundary

- **Não** substitui hardware por cross-build. A AC-2 é literal: cross-build não
  substitui hardware, e o resultado do cross-build é publicado **como cross-build**. A prova em hardware por alvo é `TASK-E14-04`.
- **Não** constrói o player. Os jobs do umbrella compilam o que o umbrella é dono:
  specs, ferramental, pins de submódulo e regras de arquitetura. A primeira tela do app é `TASK-E02-01`.
- **Não** inventa versão de NDK, Gradle ou action: a AC-3 exige que a versão
  **conferida** seja a mesma do commit, e o pin por SHA é o que torna isso verificável.
  A versão vem da matriz de toolchains de `TASK-E14-01`, dona da ABI de cada alvo.
- **Não** emite SBOM de binário publicado: o inventário aqui é de toolchain e de
  artefato de CI. O SBOM do distribuído é `TASK-E11-08`, nos notices de lançamento.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e11-04-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E11-04-AC-1` | a matriz de targets da spec de plataformas e o workspace com lockfile | o CI compila e roda o smoke de cada target | Todos os targets obrigatórios compilam e executam smoke apropriado |
| `TASK-E11-04-AC-2` | um target compilado por cross-build sem hardware correspondente disponível | o resultado do job é publicado no relatório de evidência | cross-build não substitui hardware |
| `TASK-E11-04-AC-3` | o workflow e o lockfile no estado do commit | as versões de NDK, Gradle e actions, os hashes e o inventário de secrets são conferidos | features/hashes/NDK/Gradle travados e secrets isolados. |

## Verificação
| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `Cargo.lock` e a matriz de `config/project.json`, confrontados com os jobs de `.github/workflows/ci.yml` e com os pins por SHA | cada um dos cinco alvos aparece em um job e cada `uses:` é SHA de 40 hex — leitura do workflow e do lockfile, não "parece certo" |
| Hardware | a matriz compila e roda smoke por target; os alvos sem hardware correspondente publicam o cross-build identificado como tal | ausência é limitação registrada, não aprovação |

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
