# TASK-E15-03 — Importar GitHub privado e ativar Gitflow

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E15-03` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E15-03` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M0 |
| Prioridade | P0 |
| Estado | implemented |
| Depende de | `TASK-E15-02` |
| Bloqueia | `TASK-E15-04`, `TASK-E19-01` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Importação por branch/PR ou bootstrap vazio, proteções e default develop.

## Boundary

- **Não** decide a visibilidade do repositório: é decisão do mantenedor, e quem a
  executa é `TASK-E19-06`, que abre os repositórios ao público pela ADR-014. A AC-1 mede
  a **privacidade real pela API**, e o repositório é público hoje por instrução
  explícita dele — ler o estado de hoje como se fosse a entrega inverteria a ordem
  dos fatos.
- **Não** reimporta o repositório: o import é `TASK-E15-01`. A AC-2 compara o
  histórico com a origem, e reimportar reescreveria a raiz.
- **Não** trata 403 de plano como bug: o plano da conta é um limite real do GitHub, e
  a pendência é registrada, não mascarada; quem a fecha é `TASK-E19-06`.
- **Não** protege `vynamp-mvp-reference`: ele é referência congelada, pinada por
  commit, sem `develop` e sem workflow, e quem decide o conjunto protegido é
  `TASK-E19-06`.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e15-03-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E15-03-AC-1` | o repositório publicado em `jimmykedlyn/vynamp` | a API do GitHub responde a visibilidade | Privacidade real confirmada |
| `TASK-E15-03-AC-2` | o repositório importado de `vynamp-mediaplayer` | o histórico é comparado com a origem | histórico preservado |
| `TASK-E15-03-AC-3` | `main` e `develop` protegidas | alguém tenta force push | sem force push |
| `TASK-E15-03-AC-4` | os dez repositórios, depois da abertura ao público | `cargo xtask repo audit --repo <owner/name>` roda em cada um | CI Gate/proteções realmente aplicados ou pendência registrada. |

## Verificação
| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `gh api repos/jimmykedlyn/<repo>` para visibilidade, `git rev-list --max-parents=0` para a raiz, e `gh api .../branches/<branch>/protection` para a proteção | `private=false` lido da API; a mesma raiz `247b424` em `origin/develop` e `origin/main`; `force=false`, `del=false`, `pr=true`, `strict=true`, `ctx=["CI Gate"]` nas duas branches — leitura da API e do Git, não "parece certo" |
| Hardware | nenhum: visibilidade, histórico e proteção de branch não dependem de dispositivo de áudio | ausência é limitação registrada, não aprovação |

## DoD — pronto para review

- [ ] Todos os critérios de aceite com evidência
- [ ] `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` limpos
- [ ] `cargo test` verde; testes ignorados listados com motivo
- [ ] Spec do módulo atualizada se a implementação revelou algo novo
- [ ] `backlog.json` com o estado correto
- [ ] Revisão feita por contexto limpo, não pelo implementador
- [ ] PR aberto com template preenchido; CI Gate verde no SHA final
## Notas de execução

Fechada em 2026-11-09. Esta é a TASK que mais exige cuidado ao fechar, porque duas
cláusulas foram satisfeitas em condições que mudaram depois — e "passou" sem essa
distinção esconderia justamente o que o critério queria medir.

| AC | Verificação executada | Resultado |
|---|---|---|
| AC-1 | `gh api repos/jimmykedlyn/vynamp` | `private=false`, `visibility=public` |
| AC-2 | `git rev-list --max-parents=0 origin/develop` vs `origin/main` | ambos `247b424` — mesma raiz importada, nenhuma reimportação |
| AC-3 | `gh api repos/jimmykedlyn/vynamp/branches/{main,develop}/protection` | `force=false`, `del=false`, `pr=true`, `strict=true`, `ctx=["CI Gate"]` nas duas |
| AC-4 | `cargo xtask repo audit --repo <owner/name>` nos dez repositórios | 9 com `pending: []`; `vynamp-mvp-reference` com três pendências **registradas** |

**AC-1 — "privacidade real confirmada" precisa de leitura literal.** O critério
não diz "privado para sempre": diz que a API responde a visibilidade e que a
privacidade era real. Era, em 2026-10-18, quando a evidência registrou
`private=true` lido da API antes e depois. Hoje o repositório é **público por
decisão do mantenedor** ("é para deixar público mesmo, eu mudei essa regra"), e a
API responde `private=false`. A cláusula foi cumprida no tempo dela; ler o estado
de hoje como se fosse a entrega inverteria a ordem dos fatos.

**AC-2 — histórico preservado.** A raiz `247b424` de `origin/main` é a mesma de
`origin/develop`, e `main` é ancestral de `develop` (`git merge-base
--is-ancestor`, exit 0). Nenhum commit do import original foi reescrito; o
`develop` tem 197 commits sobre essa raiz.

**AC-3 — "sem force push".** A proteção impede force push em `main` e `develop`, e
é isso que o critério mede. O log de atividade do repositório **tem** oito eventos
`force_push`, e todos são ou de um `dependabot[bot]` reescrevendo a própria branch,
ou de `chore/claude-e19-01-secret-scan` em 2026-11-09 — a branch de feature onde um
blob com forma de chave privada precisou sair do alcance do histórico. Nenhum
deles toca branch protegida, e nenhum push forçado foi feito nesta TASK.

**AC-4 — a pendência do `vynamp-mvp-reference` é registrada, não esquecida.** Ele
não tem `develop` e não tem workflow, então exigir `CI Gate` ali deixaria todo PR
`blocked` para sempre. A decisão está em
[`evidence/PR/task-e19-06/README.md`](../../evidence/PR/task-e19-06/README.md),
seção "Por que `vynamp-mvp-reference` ficou de fora da proteção" — que é
exatamente o "ou pendência registrada" do `Then`.

**A pendência do AC-4 aberta na evidência original foi fechada depois.** O `403` de
branch protection registrado em 2026-10-18 ("Upgrade to GitHub Pro or make this
repository public") deixou de existir quando os repositórios foram abertos; a
proteção foi aplicada nos dez pela TASK-E19-06 (2026-11-08), e é o que as leituras
acima confirmam.
