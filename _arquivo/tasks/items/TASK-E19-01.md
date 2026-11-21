# TASK-E19-01 — Migrar o CI do repo principal para GitHub Actions

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E19-01` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E19-01` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M1 |
| Prioridade | P0 |
| Estado | implemented |
| Depende de | `TASK-E15-03` |
| Bloqueia | `TASK-E19-02`, `TASK-E19-05`, `TASK-E19-06` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–10 dias úteis |
| Risco | alto |

## Entrega

ADR-014, workflows ci/release/quality em GitHub Actions com actions fixadas por SHA e gate único CI Gate, xtask de validação e release reescrito para Actions e remoção do Woodpecker.

## Boundary

- **Não** publica release. `release.yml` não mora aqui: o umbrella não tem binário, e
  um workflow de release neste repositório só existiria para falhar no próprio
  preflight. O arquivo está em `_arquivo/workflows/release.yml`, fora da árvore que o
  CI lê, e o release assinado é `TASK-E15-10`.
- **Não** usa `macos-latest`. A imagem troca de arquitetura conforme o GitHub
  rotaciona, e um alvo que se move não é alvo fixado: são `macos-15-intel` e
  `macos-15`, a matriz que `TASK-E14-01` congela alvo a alvo.
- **Não** exige runner pago além do que o plano comporta. A matriz de cinco alvos por
  PR já esbarrou na cota do plano Free, e isso está registrado na
  [ADR-014](../../adr/ADR-014-publico-dsp-interno.md).
- **Não** migra a documentação de CI para o repositório privado; a ADR-014 registra a
  migração.

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
| AC-5, varredura de segredo | `cargo xtask check-secrets` | `No credential shape found in the reachable history.` sobre todo blob de toda ref |
| AC-4, matriz macOS | `cargo xtask check-delivery` | reprova se `macos-15-intel` ou `macos-15` sair da matriz do `tooling`, ou se `macos-latest` entrar |
| Oráculo independente | `cargo xtask check-delivery` para os pins, `cargo test --package xtask --locked github_actions` para a semântica do gate, e a API de execuções para a matriz | 15 testes verdes, incluindo `rejects_gate_missing_dependency`, `rejects_gate_skipping_on_failure` e `rejects_gate_ignoring_results`; `Tooling (macos-15-intel)` e `Tooling (macos-15)` `success` no mesmo run — lido do relatório, não "parece certo" |
| Hardware | nenhum: o job `tooling` roda fmt, clippy e os testes do xtask, que são host-agnostic | ausência é limitação registrada, não aprovação |

## DoD — pronto para review

- [ ] Todos os critérios de aceite com evidência
- [ ] `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` limpos
- [ ] `cargo test` verde; testes ignorados listados com motivo
- [ ] Spec do módulo atualizada se a implementação revelou algo novo
- [ ] `backlog.json` com o estado correto
- [ ] Revisão feita por contexto limpo, não pelo implementador
- [ ] PR aberto com template preenchido; CI Gate verde no SHA final
## Notas de execução

Fechada em 2026-11-09. Quatro dos cinco critérios fecham contra o estado atual; o
quinto (AC-3) fecha por **decisão registrada que revogou o objeto do critério**, e
isso está explicado abaixo em vez de escondido.

| AC | Verificação executada | Resultado |
|---|---|---|
| AC-1 | `cargo xtask check-delivery` | `"result": "passed"`, `workflow_files: 2`, todas as `uses:` com 40 hex |
| AC-2 | `cargo test --package xtask --locked github_actions` | 15 testes verdes, incluindo `rejects_gate_missing_dependency`, `rejects_gate_skipping_on_failure` e `rejects_gate_ignoring_results` |
| AC-3 | `ls .github/workflows/` + `check-delivery` + `archived_release` | **sem `release.yml`** — ver abaixo |
| AC-4 | `gh api .../actions/runs/42677998586/jobs` | `Tooling (macos-15-intel)` success e `Tooling (macos-15)` success no mesmo run |
| AC-5 | `cargo xtask check-secrets` | `No credential shape found in the reachable history.`, 3.184 blobs |

**AC-2 — "ausente" é o caso que ninguém testa.** O critério exige reprovar também
dependência que **não existe**. `github_actions_rejects_gate_missing_dependency`
tira `arch` e `tasks` do `needs:` e exige reprovação; sem esse teste, um `needs:`
encurtado deixaria o gate verde sobre um conjunto menor de checagens. A matriz
real, lida do run `42677998586` em `5c0b82a`, tem os oito jobs e `CI Gate`
`success`.

**AC-3 — o critério perdeu seu objeto, e a remoção foi deliberada.** O
`release.yml` **não está mais** em `.github/workflows/`: o passo 34 o arquivou em
`_arquivo/workflows/release.yml` pela decisão **D-CI-3** ("o umbrella não tem
binário; um workflow de release aqui só existiria para falhar no preflight").
`check-delivery` hoje **reprova** o retorno do arquivo à árvore
(`github_actions_rejects_release_workflow_in_the_umbrella`) e, ao mesmo tempo,
continua validando o arquivo arquivado (`archived_release`: trigger só `push` de tag
`v*`, roda `release preflight`, mantém `exit 1`). Ou seja: a propriedade que o AC-3
protegia — "release só em tag `v*` e falha fechado" — continua imposta sobre o
artefato herdado que o app vai usar, e agora também é imposta a ausência do arquivo
onde ele não deve estar. O AC-3 foi escrito em 2026-11-05, antes da decisão que o
tornou obsoleto; fechar a TASK como se o workflow existisse seria falso, e deixá-la
aberta seria manter aberto um critério que a arquitetura já resolveu.

**AC-4 — macOS Intel e Apple Silicon como alvos verificados.** A matriz do job
`tooling` é `[ubuntu-22.04, macos-15-intel, macos-15]`, e `check-delivery` reprova
tanto a remoção de qualquer um dos dois quanto a entrada de `macos-latest` (que
troca de arquitetura quando o GitHub rotaciona a imagem). A verificação não é o
YAML: é o run `42677998586`, com os dois jobs de Mac concluídos `success`.

**AC-5 — segredo no histórico.** `check-secrets` lê todo blob de toda ref alcançável
(3.184 blobs, 4,2 s) e roda no job `tooling` sobre um checkout `--unshallow`. Um
blob com forma de chave privada **existiu** na branch de trabalho — era o fixture
literal do próprio teste do scanner — e foi retirado do alcance por
`--force-with-lease` antes do merge.
