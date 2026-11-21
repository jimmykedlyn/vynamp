# STATE — onde o projeto está

Versão: 0.2.0 · Data: 2026-11-21 · Estado: retrato congelado; a fonte de status das
TASKs é a base de backlog do Notion (ver [AGENTS.md](AGENTS.md) §7).

Este arquivo responde uma pergunta só: **em que ponto o projeto estava quando o
status saiu do repositório**. Desde 2026-11-21 o status vivo de cada TASK mora no
Notion. O `tasks/backlog.json` continua versionado como **especificação** das
TASKs (título, módulo, requisitos, dependências, critério), e o campo `status` dele
ficou congelado nesta data: serve só para o `check-tasks` saber quanto rigor cobrar
de cada TASK. Quem leva uma TASK além de `planned` no Notion atualiza esse campo no
mesmo PR, porque é ele que faz o gate exigir o critério escrito. O
`cargo xtask docs-sync` deixou de existir, e a lista abaixo não é mais regenerada.

## Onde estão as coisas

| Pergunta | Documento |
|---|---|
| Qual o status de cada TASK | base de backlog do Notion |
| Em que ordem executar | campos `Depende de`, `Onda` e `Prio` da base do Notion |
| O que abrir depois | base do Notion; [`tasks/NEXT.md`](tasks/NEXT.md) é o retrato de 2026-11-21 |
| O que já foi entregue e com que prova | [`tasks/EVIDENCE.md`](tasks/EVIDENCE.md) e `evidence/PR/` |
| Por que a arquitetura é assim | [`adr/`](adr/) |
| O que o produto promete | [`product/03-requisitos.md`](product/03-requisitos.md) |

## O estado em 2026-11-21

| Estado | TASKs |
|---|---|
| `implemented` | 20 |
| `planned` | 195 |
| **total** | 215 |

Entregues, em ordem de ID:

- `TASK-CI-01` — Reescrever delivery() e github_actions() para a árvore do umbrella
- `TASK-CI-02` — Adicionar o job check-tasks ao CI do umbrella
- `TASK-E01-01` — Provar decode, gapless e precisão de samples
- `TASK-E01-02` — Provar callback e output clock
- `TASK-E01-04` — Entregar fonte local e decoder registry
- `TASK-E02-11` — Congelar MVP de referência e extrair SVGs, ícones e temas
- `TASK-E04-02` — Provar resampler e sample conversion
- `TASK-E10-01` — Criar corpus de testes licenciado
- `TASK-E11-01` — Criar workspace e baseline de engenharia
- `TASK-E11-02` — Definir fronteiras de segurança
- `TASK-E11-03` — Resolver licenças e inventário de dependências
- `TASK-E15-01` — Configurar Claude Code e Superpowers obrigatório
- `TASK-E15-02` — Validar worktrees, claims e handoff
- `TASK-E15-03` — Importar GitHub privado e ativar Gitflow
- `TASK-E15-04` — Ativar workflows e CI Gate multiplataforma
- `TASK-E19-01` — Migrar o CI do repo principal para GitHub Actions
- `TASK-E19-02` — Higiene, governança e README bilíngue do repo principal
- `TASK-E19-04` — Re-apontar as specs de Restore e Converter e criar os repos próprios
- `TASK-E19-05` — CI e README bilíngue nas libs Rust e no MVP de referência
- `TASK-E19-06` — Abrir os repositórios ao público e proteger as branches

## Pendências de processo que não são TASK

- **Ratificar ou reverter três critérios da `specs/decode.md` §5.** O commit `2227943`
  acrescentou `DECODE-AC-32` e `-33` e reescreveu `DECODE-AC-01` no mesmo commit dos
  testes, contra a regra "a spec muda antes do código". A violação está registrada na
  própria §5; desfazer exigiria reescrever história, então a decisão é do mantenedor.

- **Revisão em contexto limpo da `TASK-E01-04`: feita.** Veredito APROVADO COM
  RESSALVAS, sete achados, seis corrigidos e um registrado. As quatro tentativas
  anteriores foram interrompidas por um sinal inválido — arquivo de saída de subagente
  em 0 byte não indica travamento, e todos os desta sessão gravam assim.
