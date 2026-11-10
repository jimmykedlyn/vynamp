# TASK-E19-04 — Re-apontar as specs de Restore e Converter e criar os repos próprios

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E19-04` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E19-04` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M3 |
| Prioridade | P0 |
| Estado | implemented |
| Depende de | `TASK-E18-01`, `TASK-E18-02` |
| Bloqueia | `TASK-E19-06` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–6 dias úteis |
| Risco | médio |

## Entrega

Specs de Restore e Converter re-apontadas para repositórios próprios, com a fonte normativa mantida no repo principal e os repos vynamp-encode, vynamp-converter e vynamp-restore criados públicos.

## Boundary

- **Não** autora §3 a §10 das specs de módulo. O esqueleto de `specs/` criado no
  passo 37 continua esqueleto: esta TASK aponta os repos e os dota de governança,
  não escreve projeto de API (isso é `TASK-E18-01`/`TASK-E18-02`).
- **Não** move o texto normativo para os repos de módulo. A fonte continua em
  `reference/tools/`; num repo de módulo mora código, não uma segunda cópia da norma.
- **Não** mexe em visibilidade dos outros seis repos nem em branch protection —
  isso é `TASK-E19-06`, já executada.
- **Não** implementa encoder, conversor ou restaurador. Os três repos ficam em
  `Status: specification` no README.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e19-04-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E19-04-AC-1` | as specs canônicas `reference/tools/01-vynamp-restore.md` e `reference/tools/02-vynamp-converter.md`, e nenhuma outra | cada URL `github.com/jimmykedlyn/vynamp-*` citada nelas é requisitada na API REST | As specs apontam para URLs reais dos repos novos |
| `TASK-E19-04-AC-2` | os `specs/` de `vynamp-encode`, `vynamp-converter` e `vynamp-restore` e os documentos canônicos do umbrella | cada afirmação normativa de um `specs/*.md` de módulo é procurada no documento canônico correspondente | a fonte normativa continua única no repo principal |
| `TASK-E19-04-AC-3` | os repositórios `jimmykedlyn/vynamp-encode`, `vynamp-converter` e `vynamp-restore` | `visibility` e a árvore da raiz de cada um são lidos na API REST | os três repos existem, públicos, com README bilíngue, LICENSE e SECURITY.md. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `gh api repos/jimmykedlyn/<repo>` e `.../contents/` | `visibility=public` e os arquivos existindo, lidos da API — não do clone local |
| Hardware | não se aplica | a TASK é de repositório e documentação; ausência de hardware não aprova nem reprova |

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
[`evidence/PR/task-e19-04/`](../../evidence/PR/task-e19-04/README.md).

- **`AC-1` estava violado, e de forma mais grave que "falta um link".**
  `reference/tools/02-vynamp-converter.md` descrevia `vynamp-encode` como
  "repositório privado" — falso desde a `TASK-E19-06`. Nenhuma das duas specs
  canônicas citava URL de repositório nenhuma. As URLs agora estão nos dois
  documentos e foram lidas na API REST.
- **`AC-2` estava violado pelos próprios esqueletos.** As três
  `specs/README.md` de módulo abriam com "Normativas: o código segue a spec, não o
  contrário", o que põe o repo de módulo como fonte. Corrigido: elas declaram que a
  norma vive no repositório principal e apontam o documento canônico. O cabeçalho
  de cada `specs/*.md` ganhou a mesma advertência.
- **`vynamp-encode` não tem documento canônico próprio.** Sua norma está em
  `02-vynamp-converter.md` §1 (motor de codificação); o `specs/README.md` do
  módulo aponta para lá em vez de inventar um documento novo.
- **`vynamp-fases` não é repositório.** `GET repos/jimmykedlyn/vynamp-fases`
  devolve 404: é material de planejamento local. A referência a
  `01-arvore-final.md` nos esqueletos é textual, sem link, para não criar link
  quebrado.
- **`AC-3` dependia de conteúdo que ainda não estava no remoto.** Os arquivos de
  governança existiam só no working tree dos submódulos; entraram nos PRs deste
  passo. `visibility=public` foi lido da API, não presumido.
- **Pendência que esta TASK não fecha:** §3 a §6 e §8 a §10 das specs de módulo
  continuam `PENDENTE`. É autoria de spec por módulo, escopo de
  `TASK-E18-01`/`TASK-E18-02`, não desta TASK.
