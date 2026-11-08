# Pull request — TASK-E19-03

## O que muda

O `vitrola` deixa de ser biblioteca em repositório próprio e passa a ser **módulo DSP interno do app**, por decisão do mantenedor registrada na [ADR-014](../../../adr/ADR-014-publico-dsp-interno.md): *"os Tube e Vinil são recursos do DSP dentro da aplicação, não são libs e nem repositórios externos"*.

O design aprovado em 2026-10-23 (77 linhas, 9 seções) é preservado por **incorporação**, não por reimplementação: entra em [10-vitrola.md](../../../reference/audio/10-vitrola.md) como §8.1–8.9, e o texto literal de origem fica em [vitrola-design.md](vitrola-design.md) para que a alegação seja auditável em vez de uma promessa.

Isto **não implementa** nada: vinil e válvula continuam sem código. A integração no grafo é a [TASK-E19-07](../../../tasks/items/TASK-E19-07.md), que depende de E04-03.

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
| [docs/audio/10-vitrola.md](../../../reference/audio/10-vitrola.md) | 0.8.0; passa a ser **fonte única** do design; §1 reescrita; ordens das seções corrigidas; §6 aponta a TASK-E19-07 |
| [ADR-011](../../../adr/ADR-011-pilha-propria.md) | 0.1.1; linha do inventário deixa de ser repositório com SHA |
| [docs/GLOSSARY.md](../../../architecture/GLOSSARY.md) | "biblioteca própria" → módulo DSP interno |
| [docs/product/05-decisions.md](../../../product/05-decisoes.md) | citações `vitrola §N` re-apontadas para §8.x |
| [tasks/items/TASK-E19-03.md](../../../tasks/items/TASK-E19-03.md) | 0.9.0 com a situação por critério de aceite |
| 13 outros docs | catálogo DSP, contrato de sinal, paridade, licenças, roadmap, riscos, MVP reference, INDEX, CHANGELOG |

Nenhuma alteração de código, dependência, workflow ou backlog.

## O que não foi feito, e por quê

- **O arquivamento do repositório remoto é do mantenedor.** `gh repo archive jimmykedlyn/vitrola` está prescrito na TASK, mas visibilidade e ciclo de vida de repositório são decisão do mantenedor ([AGENTS.md](../../../AGENTS.md), ADR-014). É reversível por `gh repo unarchive`. **Logo, o critério "repositório arquivado" continua pendente.**
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

- **A revisão independente rodou e aprovou com ressalvas; todas foram aplicadas.** No primeiro ciclo ela não rodou — o plano semanal estourou em 2026-11-05 (HTTP 429) e os subagentes de revisão morriam sem relatório, então a conferência ficou com o próprio autor, declarada em vez de escondida. No segundo ciclo um revisor de contexto limpo (Sonnet, nunca o implementador) devolveu **APROVADO COM RESSALVAS** com 9 achados, todos de precisão factual e de link, nenhum de escopo. Ele conferiu de forma independente que a cópia literal do design é byte-idêntica a `git show 75c4ca2:` (sha256 `dfa9b12b…`), que a tabela VIT-T-01..10 está íntegra, que não há alegação de implementação e que a string de aceite é idêntica nos três arquivos exigidos. As 9 correções estão em `1352f1f`: o texto que afirmava o arquivamento do repositório como fato (é pendência do mantenedor), um link de seção com slug errado, "na biblioteca" → "no módulo", o D-DSP-07 que ainda dizia "TASK de integração pendente" (a TASK-E19-07 existe), três referências `vitrola §3/§4` na numeração antiga e duas contagens erradas na própria evidência.
- **A cópia literal é um `git show` do clone local**, não uma validação de que `75c4ca2` é o design correto — o design foi aprovado em 2026-10-23 e o commit posterior só renomeia.
- **Nada foi exercitado em runtime**: não há código para exercitar.
- **Este PR não altera visibilidade, proteções, assinatura nem plano** — isso é da TASK-E19-06.

## Checklist

- [x] Worktree e owner próprios.
- [x] `cargo xtask check-docs` e `check-delivery` passam.
- [x] Docs/contracts/notices atualizados; nenhuma credencial no diff.
- [x] Design original preservado de forma auditável (cópia literal versionada).
- [x] Revisão técnica independente por contexto limpo — aprovado com ressalvas, todas aplicadas em `1352f1f`.
- [ ] CI Gate (GitHub Actions) — o run de `1352f1f` é o que vale.
- [ ] Arquivamento do repositório remoto — **do mantenedor**.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
