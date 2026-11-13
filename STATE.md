# STATE — onde o projeto está

Versão: 0.1.0 · Data: 2026-11-13 · Estado: resumo de leitura; a fonte de status é
[`tasks/backlog.json`](tasks/backlog.json).

Este arquivo responde uma pergunta só: **em que ponto o projeto está agora**. Ele não
decide nada e não guarda status — quem guarda é o backlog, que é versionado e viaja com
o clone. Quando os dois divergirem, o backlog ganha e este arquivo está errado.

A parte que muda a cada TASK entregue está no bloco gerado abaixo, e
`cargo xtask check-docs` reprova o arquivo quando ele diverge do backlog. Para
reescrevê-lo: `cargo xtask docs-sync`.

## Onde estão as coisas

| Pergunta | Documento |
|---|---|
| Qual o status de cada TASK | [`tasks/backlog.json`](tasks/backlog.json) |
| Em que ordem executar | [`tasks/ORDER.md`](tasks/ORDER.md) — gerado |
| O que abrir depois | [`tasks/NEXT.md`](tasks/NEXT.md) |
| O que já foi entregue e com que prova | [`tasks/EVIDENCE.md`](tasks/EVIDENCE.md) e `evidence/PR/` |
| Por que a arquitetura é assim | [`adr/`](adr/) |
| O que o produto promete | [`product/03-requisitos.md`](product/03-requisitos.md) |

## Regras que valem para este arquivo

1. **Nada de data nem SHA.** Data envelhece no dia seguinte e SHA muda a cada merge. Um
   resumo que carrega os dois vira trabalho de manutenção diária e mente no intervalo.
   O que muda por TASK é a lista de entregues, e é só ela que entra no bloco gerado.
2. **Prosa é humana, bloco é gerado.** O texto fora dos marcadores é escrito à mão; o
   que está entre `<!-- BEGIN GENERATED -->` e `<!-- END GENERATED -->` é saída de
   `docs-sync`, e editar à mão ali é perder a edição no próximo run.
3. **Este arquivo não bloqueia nada.** Ele é leitura. Se sumir, o clone continua
   completo, porque o backlog tem tudo.

## O estado

<!-- BEGIN GENERATED: state -->
| Estado | TASKs |
|---|---|
| `implemented` | 15 |
| `planned` | 200 |
| **total** | 215 |

Entregues, em ordem de ID:

- `TASK-CI-01` — Reescrever delivery() e github_actions() para a árvore do umbrella
- `TASK-CI-02` — Adicionar o job check-tasks ao CI do umbrella
- `TASK-E01-01` — Provar decode, gapless e precisão de samples
- `TASK-E01-02` — Provar callback e output clock
- `TASK-E01-04` — Entregar fonte local e decoder registry
- `TASK-E02-11` — Congelar MVP de referência e extrair SVGs, ícones e temas
- `TASK-E04-02` — Provar resampler e sample conversion
- `TASK-E11-01` — Criar workspace e baseline de engenharia
- `TASK-E11-03` — Resolver licenças e inventário de dependências
- `TASK-E15-03` — Importar GitHub privado e ativar Gitflow
- `TASK-E19-01` — Migrar o CI do repo principal para GitHub Actions
- `TASK-E19-02` — Higiene, governança e README bilíngue do repo principal
- `TASK-E19-04` — Re-apontar as specs de Restore e Converter e criar os repos próprios
- `TASK-E19-05` — CI e README bilíngue nas libs Rust e no MVP de referência
- `TASK-E19-06` — Abrir os repositórios ao público e proteger as branches
<!-- END GENERATED: state -->

## Pendências de processo que não são TASK

- **Revisão em contexto limpo da `TASK-E01-04`.** O subagente `revisor` falhou duas
  vezes sem devolver saída e a revisão saiu da sessão do implementador. O código está
  em `develop` (`6b83a8b`); fechar o registro exige revisão nova, não edição de
  histórico.
