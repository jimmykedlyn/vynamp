# ADR-023 — Andamento das TASKs em GitHub Issues e Projects

- **Status:** Aceita
- **Data:** 2026-11-24
- **Substitui:** ADR-019 (trecho "backlog no Notion"); emenda a ADR-020 (o ciclo de vida
  continua, muda o lugar onde ele é gravado)
- **Decisor:** jimmykedlyn

## Contexto

Desde a ADR-019 o andamento das TASKs vive na base TASKs do Notion, e a ADR-020 deu a ela
os estados e os campos de rastreio. Esse arranjo tem custos:

1. **Fora do GitHub.** Branch, PR, SHA e merge commit nascem no GitHub e são copiados à mão
   para o Notion. A cópia diverge (`engineering/10-estado-transicao.md` lista TASKs
   `Entregue` sem PR e sem plano).
2. **Só o orquestrador alcança.** O acesso é pelo MCP do Notion; subagentes, CI e o
   mantenedor no celular não leem o mesmo dado pelo mesmo caminho.
3. **Sem vínculo com o PR.** Um PR não fecha nem referencia a página da TASK.

A ADR-022 liberou o GraphQL, que é a única API do GitHub Projects (v2).

## Decisão

1. **Uma Issue por TASK** no repositório `jimmykedlyn/vynamp`, título
   `TASK-EII-NN — <título>`, com os rótulos `task`, `módulo:<módulo>`, `fase:<fase>` e
   `prio:<prio>`. O corpo aponta para a entrada no `tasks/backlog.json` e para o arquivo em
   `tasks/items/`; não copia o critério.
2. **Um Project (v2) "Vynamp — TASKs"** do usuário `jimmykedlyn` reúne as Issues e guarda
   o andamento nos campos `Estado`, `Onda`, `Responsável`, `Branch`, `PR`, `SHA revisado`,
   `Merge commit`, `Voltas` e `Motivo`, com os mesmos valores e regras da ADR-020.
3. **A spec continua no `tasks/backlog.json`** (título, módulo, fase, prio, dependências,
   critério) e o `xtask` segue validando contra ele. A Issue espelha; quem manda é o
   backlog.
4. O PR de implementação referencia a Issue (`Refs #N`); o fechamento da Issue acompanha
   o estado `Integrada` ou `Cancelada`, não o merge do PR.
5. A base TASKs do Notion fica congelada como histórico. Ninguém mais lê nem escreve nela
   como fonte de andamento.
6. A migração cria só as Issues das TASKs `planned`; as `implemented` contam como
   `Integrada` sem Issue. O estado inicial vem da spec (`Pronta` se o critério está
   completo, senão `Rascunho`) e é ajustado pelo que existe no GitHub (branch ou PR da
   TASK nos repos de módulo). A `Onda` vem do export da base do Notion; sem export, é
   calculada pela faixa do marco e pela profundidade das dependências.

## Alternativas consideradas

- **Manter o Notion.** Sem migração, mas mantém a cópia manual e o acesso só pelo MCP.
- **Só Issues, com estado em rótulos.** Dispensa GraphQL, mas rótulos não são exclusivos:
  uma Issue pode ficar com dois estados ao mesmo tempo, o problema que a ADR-020 resolveu.
- **Andamento no `backlog.json`.** Versionado, mas exige PR por transição; é o modelo que
  a ADR-019 abandonou.

## Consequências

Positivas:

- Rastreio e andamento no mesmo lugar do código; `gh` lê e escreve os dois.
- PR e Issue se referenciam; a consulta de consistência vira uma chamada GraphQL.

Negativas:

- Project (v2) exige o escopo `project` no token do mantenedor (`gh auth refresh -s
  project`).
- São ~200 Issues novas no repositório público: o andamento passa a ser público.
- `Onda` depende do export do Notion ou de um cálculo que pode divergir do que o
  mantenedor ajustou à mão; a divergência é corrigida no Project depois.
- A claim local do `xtask` continua sem consultar `Responsável`; a proteção entre máquinas
  segue dependendo do orquestrador olhar o Project antes de reservar.
