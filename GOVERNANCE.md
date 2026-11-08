# Governance / Governança

Versão 0.9.0 · 2026-11-05 · Descreve o modelo que existe hoje; não promete estrutura futura.

## Current model / Modelo atual

Vynamp is a **single-maintainer project**. There is no foundation, no committee,
no steering group and no voting body. Decisions are made by the maintainer,
`@jimmykedlyn`, who is also the only person who merges, tags and publishes.

Vynamp é um **projeto de mantenedor único**. Não há fundação, comitê, grupo
diretor nem votação. As decisões são do mantenedor, `@jimmykedlyn`, que é também
a única pessoa que mergeia, cria tags e publica.

The project is written and reviewed by an AI agent (Claude Code) under that
maintainer's direction. The agent is the author **and** the reviewer of every
task, with model tiering and a clean-context review pass; that arrangement is
recorded in [`AGENTS.md`](AGENTS.md) and is not a substitute for human approval
of a merge.

O projeto é escrito e revisado por um agente de IA (Claude Code) sob a direção
desse mantenedor. O agente é autor **e** revisor de toda tarefa, com
escalonamento por modelo e uma passagem de revisão em contexto limpo; esse
arranjo está em [`AGENTS.md`](AGENTS.md) e não substitui a aprovação humana de
um merge.

## How decisions are made / Como as decisões são tomadas

| Kind / Tipo | Where it is recorded / Onde fica registrado |
|---|---|
| Architecture, technology, irreversible choice / Arquitetura, tecnologia, escolha irreversível | ADR in [`adr`](adr) |
| Product or policy decision, including licence and security channel / Decisão de produto ou política | [Decisions register](product/05-decisoes.md) (D-* IDs) |
| What will be built, in what order / O que será construído e em que ordem | Backlog [`tasks/backlog.json`](tasks/backlog.json) and [requirement matrix](product/03-requisitos.md) |
| A single unit of work / Uma unidade de trabalho | TASK in [`tasks/items/`](tasks/items/) |
| Day-to-day rule for whoever writes code / Regra do dia a dia de quem escreve | [`AGENTS.md`](AGENTS.md) and [`CONTRIBUTING.md`](CONTRIBUTING.md) |

An ADR is superseded, never deleted: a reversed decision keeps its file with a
marker pointing at the ADR that replaced it, so the reasoning survives. The same
applies to a decision row — it is amended, not erased.

Uma ADR é substituída, nunca apagada: a decisão revertida mantém o arquivo com
uma marca apontando para a ADR que a substituiu, para o raciocínio sobreviver. O
mesmo vale para uma linha do registro de decisões — ela é emendada, não apagada.

## Contributions / Contribuições

External contributions are welcome and are reviewed by the same process and the
same gates as the maintainer's own work: `CI Gate` green, `cargo xtask
check-docs` and `check-delivery` passing, a PR that states source, task, command,
result and limitations. A contribution is accepted under the project licence
(`MIT OR Apache-2.0`) unless stated otherwise. See
[`CONTRIBUTING.md`](CONTRIBUTING.md).

Contribuições externas são bem-vindas e passam pela mesma revisão e pelos mesmos
gates do trabalho do próprio mantenedor: `CI Gate` verde, `cargo xtask
check-docs` e `check-delivery` passando, e um PR que declare fonte, tarefa,
comando, resultado e limitações. A contribuição é aceita sob a licença do
projeto (`MIT OR Apache-2.0`) salvo declaração em contrário. Ver
[`CONTRIBUTING.md`](CONTRIBUTING.md).

## What this project does not promise / O que este projeto não promete

- **No response SLA**, on issues, PRs or security reports. / Sem SLA de resposta.
- **No roadmap date**. Estimates in the backlog are ranges, not a calendar. /
  Sem data de roadmap; estimativas são faixas, não calendário.
- **No support contract**. There is no paid tier, no LTS branch and no
  backport policy yet. / Sem contrato de suporte, tier pago, branch LTS ou
  política de backport.
- **No trademark grant.** The name and brand assets belong to the maintainer;
  the code licence does not license the marks. / A licença do código não licencia
  as marcas.

## Changing this document / Alterar este documento

A change to the governance model itself requires an ADR, because it changes who
may decide — not just what is decided. Minor wording fixes do not.

Mudar o modelo de governança exige uma ADR, porque muda **quem** decide, não
apenas o que é decidido. Correções de redação não exigem.
