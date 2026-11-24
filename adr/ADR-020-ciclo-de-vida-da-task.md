# ADR-020 — Ciclo de vida da TASK no Notion

- **Status:** Proposta
- **Data:** 2026-11-22
- **Emenda:** ADR-019 (trecho "não há propriedade Status; o andamento são as três caixas")
- **Decisor:** jimmykedlyn

## Contexto

A ADR-019 tornou a base TASKs do Notion a fonte única de andamento e representou esse
andamento com três caixas: `Plano aprovado`, `PR do módulo`, `Entregue`. Em 2026-11-22 a
base tinha 200 TASKs e seis com `Entregue`. Nesse uso, o modelo mostrou estes problemas:

1. **Combinações inválidas sem alarme.** `TASK-E01-05` e `TASK-E01-07` estão `Entregue` sem
   `PR do módulo`; `TASK-E04-05` e `TASK-E21-01` estão `Entregue` sem `Plano aprovado`.
   Nada diz qual caixa é a verdadeira.
2. **A fase mais longa não aparece.** Entre "plano aprovado" e "PR aberto" ficam
   implementação, checagem e revisão.
3. **Reserva invisível.** A claim vive em `.git/.../claims/` da máquina; outra sessão em
   outra máquina não a vê.
4. **Sem estados de exceção.** Não existe bloqueada, cancelada nem devolvida pelo revisor; e
   CI pendente não fica registrado em lugar nenhum, então uma sessão nova não sabe retomar.
5. **"Entregue" chega cedo demais para libs.** O merge na `develop` do módulo só aparece no
   umbrella depois do PR de repin, mas a TASK dependente já é liberada.
6. **Dois lugares de status.** O contrato pedia atualizar o `status` do `backlog.json` no
   PR de implementação, ao mesmo tempo que proibia status em arquivo. Na prática ninguém
   atualizou: as seis entregues continuam `planned` no backlog, e `TASK-E11-02` está
   `implemented` no backlog e pendente no Notion.

## Decisão

1. A base TASKs ganha a propriedade **Estado** (seleção), com os valores `Rascunho`,
   `Pronta`, `Reservada`, `Em implementação`, `Em revisão`, `PR aberto`, `Entregue`,
   `Integrada`, `Bloqueada`, `Cancelada`, e as transições do §5 do `AGENTS.md`.
2. Entram as propriedades de rastreio `Responsável`, `Branch`, `PR` (URL),
   `SHA revisado`, `Merge commit`, `Voltas` (número) e `Motivo`.
3. As três caixas saem depois da migração (§Migração). Até lá, `Estado` prevalece.
4. A dependência fica satisfeita em `Integrada`. Para TASK do módulo `vynamp` e de app,
   `Entregue` já é `Integrada`.
5. O plano é aprovado pelo **mantenedor**, que é como as páginas de Comandos já funcionam
   ("me mostre o plano antes de codar" → "plano aprovado"). O orquestrador só registra.
6. O `status` do `backlog.json` deixa de ter qualquer relação com andamento. Ele muda uma
   única vez, de `planned` para o valor estrito, no PR de spec que leva a TASK a `Pronta`.
   Os 14 `implemented` atuais ficam como histórico.
7. A ordem de execução é **Onda → Prio → TASK**, e a Onda segue o roadmap: 01–16 = marco
   1.0 (M0–M3), 17–24 = M4, 25–26 = M5, 27–30 = F6. Em 2026-11-22, 18 TASKs de M5 e F6
   foram movidas para as Ondas 25–30, porque as ondas antigas eram só a profundidade das
   dependências e punham trabalho pós-1.0 no começo da fila. `Ordem` é herança do
   `ORDER.md` (sequência de IDs) e não decide nada.
8. Revisor BLOQUEADO três vezes na mesma TASK a leva para `Bloqueada`, e quem decide é o
   mantenedor.

## Alternativas consideradas

- **Manter as três caixas e acrescentar `Bloqueada`/`Cancelada` como caixas.** Muda pouco,
  mas multiplica as combinações inválidas, que já são o problema número 1.
- **Caixas mais Estado convivendo para sempre.** Dá redundância visual, mas cria duas fontes
  dentro da própria fonte única.
- **Propriedade do tipo *Status* do Notion** em vez de seleção. Ela tem grupos (a fazer, em
  andamento, concluído), mas o MCP tem menos suporte de filtro e escrita para esse tipo que
  para seleção. Fica a seleção; reavaliar se o MCP passar a tratar *Status* igual.
- **Status de volta para o `backlog.json`.** Versionado e revisável, mas exige PR para cada
  mudança de estado; é o modelo que a ADR-019 abandonou.

## Consequências

Positivas:

- Uma sessão nova retoma qualquer TASK só pelo Notion.
- Dá para ver quem está com cada TASK de qualquer máquina.
- As dependências entre repos só se liberam quando o código está visível para quem consome.

Negativas:

- Mais propriedades para manter, e o orquestrador escreve no Notion em quase toda
  transição: mais chamadas de MCP e mais tokens.
- A claim continua local no `xtask`. Até o `xtask repo worktree` consultar ou gravar
  `Responsável`, a proteção entre máquinas depende do orquestrador olhar o Notion antes.
- `Estado` vira um campo livre que um humano pode pôr em qualquer valor. A consistência
  depende de revisão periódica (consulta em `engineering/notion.md`).
- O `check-tasks` precisa aceitar o valor estrito escolhido para `Pronta`. Isso tem que ser
  verificado antes de aceitar esta ADR.

## Migração

1. Criar as propriedades novas.
2. Preencher `Estado` a partir do que existe:
   - as seis `Entregue` → `Entregue`, ou `Integrada` se o PR de repin já foi mergeado;
   - TASK com spec sem `PENDENTE` → `Pronta`;
   - o resto → `Rascunho`.
3. Preencher `PR` e `Merge commit` das seis entregues pela consulta REST da página de
   Comandos de cada uma.
4. Resolver `TASK-E11-02`: ou ela é `Entregue` (e ganha os dados do PR), ou o
   `implemented` do backlog está errado e é corrigido num PR de spec.
5. Ajustar as visões ("A fazer" passa a ordenar por Onda → Prio → TASK) e a descrição da
   base (hoje cita o `ORDER.md`).
6. Remover as três caixas.
7. Atualizar `AGENTS.md`, `engineering/notion.md` e o modelo da página de Comandos no mesmo
   PR desta ADR.
