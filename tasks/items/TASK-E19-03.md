# TASK-E19-03 — Dissolver o vitrola no DSP interno do app

Versão 0.9.0 · Estado **implementado na documentação**; o arquivamento do repositório remoto é do mantenedor. Dissolver o repositório não equivale a implementar os efeitos.

Fase M3 / P0 · Owner audio-dsp · Esforço inicial 1–3 dias úteis; reestimar.

## Dependências

- [TASK-E04-03](TASK-E04-03.md) — Construir grafo DSP e transições sem glitches

## Entrega concreta

Design aprovado do vitrola incorporado como módulo DSP interno, referências ao repositório externo removidas e repositório vitrola arquivado de forma reversível.

Decisão do mantenedor (2026-11-05): *"os Tube e Vinil são recursos do DSP dentro da aplicação, não são libs e nem repositórios externos"*. O design aprovado (77 linhas, 9 seções, redigido em 2026-10-23) é preservado por **incorporação** — não por reimplementação — dentro de `docs/audio/10-vitrola.md`. As referências que apontavam para `jimmykedlyn/vitrola` passam a apontar para o módulo interno. O repositório remoto **será arquivado** pelo mantenedor, o que é reversível (`gh repo unarchive`); o clone local só é removido depois que o design estiver no repositório principal, porque ele é a única cópia.

Escopo: `docs/audio/10-vitrola.md` (§1 reescrita + seção "Design aprovado"), `docs/architecture/adr/ADR-011-own-audio-stack.md`, `docs/GLOSSARY.md`, `docs/audio/09-dsp-catalog.md`, `docs/design/06-mvp-reference.md`, `docs/audio/07-dsp-signal-contract.md`, `docs/audio/03-dsp.md`, `docs/security/02-privacy-licensing.md`, `docs/product/06-feature-parity.md`, `docs/delivery/03-risks-open-questions.md`; `docs/evidence/PR/task-e19-03/vitrola-design.md` com o conteúdo local; `gh repo archive jimmykedlyn/vitrola`.

Boundary: não implementa os efeitos (a integração é TASK-E19-07); não **deleta** o repositório `vitrola` — arquiva, preservando histórico e SHAs citados; não reescreve histórico.

## Critérios de aceite

Nenhum documento normativo cita o repositório externo do vitrola; a spec descreve módulo DSP interno preservando VIT-T-01..10; o design local não se perde; o repositório remoto fica arquivado e reversível.

Detalhamento: `grep -rn "jimmykedlyn/vitrola" docs/` retorna só as ocorrências de evidência, que registram a origem; nenhuma em documento normativo e `docs/audio/10-vitrola.md` descreve o vinil e a válvula como módulos DSP internos do app, preservando a tabela VIT-T-01..10; `docs/evidence/PR/task-e19-03/vitrola-design.md` contém o design aprovado integral; `gh repo view jimmykedlyn/vitrola --json isArchived` retorna `true`.

[Spec](../../docs/audio/10-vitrola.md) · REQ-E19-03 em [matriz](../../docs/product/03-requirements.md).

## Situação em 2026-11-05

Entregue neste PR, em documentação:

| Critério de aceite | Estado |
|---|---|
| Nenhum documento normativo cita `jimmykedlyn/vitrola` | feito — o nome só sobrevive em `tasks/items/TASK-E19-03.md` e `TASK-E19-06.md`, como o comando de arquivamento a executar, e na evidência, que registra a origem |
| A spec descreve módulo DSP interno preservando VIT-T-01..10 | feito — [10-vitrola](../../docs/audio/10-vitrola.md) 0.8.0, §8.1–8.9 com o design integral; a tabela de testes continua na §7 |
| O design local não se perde | feito — `75c4ca2` (um commit à frente do publicado) copiado literalmente para [`docs/evidence/PR/task-e19-03/vitrola-design.md`](../../docs/evidence/PR/task-e19-03/vitrola-design.md) |
| O repositório remoto fica arquivado e reversível | **pendente — é do mantenedor**: `gh repo archive jimmykedlyn/vitrola`. Reversível por `gh repo unarchive`. O clone local de `vitrola` só deve ser removido depois que este PR estiver no `develop` |

Descoberta ao executar: o clone local estava **um commit à frente** do remoto. `75c4ca2` renomeia `agulha` para `vynamp-decode`; o remoto ainda dizia `agulha`. A cópia preservada é a local, e é por isso que ela é a única versão correta a guardar.

A pendência da §6 antiga — "criar uma TASK de integração" — está resolvida: é a [TASK-E19-07](TASK-E19-07.md), criada no épico E19.

## Evidência

QA-033, QA-005; registrar commit/digest, comando, ambiente/OS/ABI, outputs e limitações. Quebrar em slices pequenas antes execução, implementar sucesso/erro/cancelamento, revisar pelo outro agente e preservar resultados no PR/docs/evidence.
