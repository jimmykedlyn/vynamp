# Pull request — TASK-E19-03

## O que muda

O `vitrola` deixa de ser biblioteca em repositório próprio e passa a ser **módulo DSP interno do app**, por decisão do mantenedor registrada na [ADR-014](../../../../docs/architecture/adr/ADR-014-public-github-actions-and-in-app-dsp.md): *"os Tube e Vinil são recursos do DSP dentro da aplicação, não são libs e nem repositórios externos"*.

O design aprovado em 2026-10-23 (77 linhas, 9 seções) é preservado por **incorporação**, não por reimplementação: entra em [10-vitrola.md](../../../../docs/audio/10-vitrola.md) como §8.1–8.9, e o texto literal de origem fica em [vitrola-design.md](../../../../docs/evidence/PR/task-e19-03/vitrola-design.md) para que a alegação seja auditável em vez de uma promessa.

Isto **não implementa** nada: vinil e válvula continuam sem código. A integração no grafo é a [TASK-E19-07](../../../../tasks/items/TASK-E19-07.md), que depende de E04-03.

## A descoberta

O clone local estava **um commit à frente** do remoto:

| Ref | SHA | Estado |
|---|---|---|
| `origin/develop`, `origin/main` | `3d44c68` | publicado, ainda diz `agulha` (nome antigo da lib de decode) |
| `develop` local | `75c4ca2` | **não publicado**, renomeia três ocorrências para `vynamp-decode` |

Um arquivo, três linhas. Arquivar sem preservar o commit local deixaria sobrevivente justamente o texto com o nome errado — por isso a cópia da evidência é a de `75c4ca2`.

## Arquivos

19 modificados + 2 novos. Os principais:

| Arquivo | Mudança |
|---|---|
| [docs/audio/10-vitrola.md](../../../../docs/audio/10-vitrola.md) | 0.8.0; passa a ser **fonte única** do design; §1 reescrita; ordens das seções corrigidas; §6 aponta a TASK-E19-07 |
| [ADR-011](../../../../docs/architecture/adr/ADR-011-own-audio-stack.md) | 0.1.1; linha do inventário deixa de ser repositório com SHA |
| [docs/GLOSSARY.md](../../../../docs/GLOSSARY.md) | "biblioteca própria" → módulo DSP interno |
| [docs/product/05-decisions.md](../../../../docs/product/05-decisions.md) | citações `vitrola §N` re-apontadas para §8.x |
| [tasks/items/TASK-E19-03.md](../../../../tasks/items/TASK-E19-03.md) | 0.9.0 com a situação por critério de aceite |
| 13 outros docs | catálogo DSP, contrato de sinal, paridade, licenças, roadmap, riscos, MVP reference, INDEX, CHANGELOG |

Nenhuma alteração de código, dependência, workflow ou backlog.

## O que não foi feito, e por quê

- **O arquivamento do repositório remoto é do mantenedor.** `gh repo archive jimmykedlyn/vitrola` está prescrito na TASK, mas visibilidade e ciclo de vida de repositório são decisão do mantenedor ([AGENTS.md](../../../../AGENTS.md), ADR-014). É reversível por `gh repo unarchive`. **Logo, o critério "repositório arquivado" continua pendente.**
- **O clone local não foi removido.** É a única cópia do commit `75c4ca2`; só sai depois que este PR estiver no `develop`.
- **A matriz VU-INT-06 não foi tocada** — é escopo da E02-07.

## Verificação

| Comando | Resultado |
|---|---|
| `cargo xtask check-docs` | `passed`, `errors: []` — 358 arquivos, 191 requisitos, 191 tarefas, 19 épicos, spec 0.7.0 |
| `cargo xtask check-delivery` | `passed`, `errors: []`, `workflow_files: 3` |
| `grep -rn "jimmykedlyn/vitrola" docs/ tasks/ config/ AGENTS.md` | 15 ocorrências, **nenhuma em documento normativo**: 8 em `docs/evidence/` (origem) e 7 nos dois arquivos de TASK como o comando de arquivamento a executar (5 na E19-03, 2 na E19-06) |
| `gh repo view jimmykedlyn/vitrola --json isArchived` | **não executado** — é do mantenedor; o valor é `false` hoje |

## Riscos e limites

- **A revisão independente está em andamento neste SHA.** No primeiro ciclo ela não rodou: o plano semanal estourou em 2026-11-05 (HTTP 429) e subagentes de revisão morriam sem relatório; a conferência foi feita pelo próprio autor, declarada em vez de escondida. O item de checklist só muda quando o relatório de contexto limpo chegar — este corpo não antecipa o veredito.
- **A cópia literal é um `git show` do clone local**, não uma validação de que `75c4ca2` é o design correto — o design foi aprovado em 2026-10-23 e o commit posterior só renomeia.
- **Nada foi exercitado em runtime**: não há código para exercitar.
- **Este PR não altera visibilidade, proteções, assinatura nem plano** — isso é da TASK-E19-06.

## Checklist

- [x] Worktree e owner próprios.
- [x] `cargo xtask check-docs` e `check-delivery` passam.
- [x] Docs/contracts/notices atualizados; nenhuma credencial no diff.
- [x] Design original preservado de forma auditável (cópia literal versionada).
- [ ] CI Gate (GitHub Actions) — aguardando o run deste SHA.
- [ ] Revisão técnica independente — em andamento.
- [ ] Arquivamento do repositório remoto — **do mantenedor**.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
