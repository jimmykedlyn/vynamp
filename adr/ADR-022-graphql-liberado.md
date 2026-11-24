# ADR-022 — API GraphQL do GitHub liberada

- **Status:** Aceita
- **Data:** 2026-11-24
- **Decisor:** jimmykedlyn

## Contexto

O contrato (`AGENTS.md`, seção "API do GitHub") e a definição do integrador proibiam
GraphQL: "REST apenas. GraphQL é instável neste ambiente". A proibição nunca teve ADR nem
incidente registrado; ficou como regra de contrato.

Ela tem custo:

1. **GitHub Projects (v2) só tem API GraphQL.** Sem ela, o andamento das TASKs não pode
   morar num Project, que é o substituto planejado para a base do Notion.
2. **`gh pr …` usa GraphQL por baixo.** A regra obrigava a reescrever em `gh api` REST
   operações que o `gh` já faz, e a própria definição do integrador manda usar
   `gh pr merge --match-head-commit`, que é GraphQL — a regra já era contraditória.

## Decisão

1. GraphQL é permitido, tanto por `gh pr …`/`gh issue …` quanto por `gh api graphql`.
2. REST continua permitido; cada operação usa a API que a faz melhor.
3. Continua valendo: credencial só do `gh auth login` do mantenedor (keyring) ou de
   `GH_TOKEN` já exportado; token nunca como argumento, em log, PR ou evidência.
4. Falha de API (GraphQL ou REST) é registrada com o erro real; não se mascara nem se
   repete às cegas.

## Alternativas consideradas

- **Manter a proibição.** Impede o GitHub Projects e mantém a contradição com
  `gh pr merge`.
- **Liberar só para Projects.** Regra mais difícil de seguir e de revisar, sem ganho de
  segurança: o risco de instabilidade é o mesmo em qualquer chamada GraphQL.

## Consequências

Positivas:

- O andamento das TASKs pode ir para um GitHub Project.
- Fim da contradição entre o contrato e o fluxo do integrador.

Negativas:

- **Se a instabilidade que motivou a regra for real,** ela volta a aparecer, agora sem
  rota de fuga escrita. Mitigação: o item 4 da decisão; se a falha se repetir, nova ADR.
- **Rate limit diferente.** GraphQL conta pontos por custo da consulta, não por chamada;
  consulta grande em Project pode esgotar a cota antes do esperado.
- O token precisa do escopo `project` para escrever em Projects (`gh auth refresh -s
  project`), um privilégio a mais na credencial do mantenedor.
