# TASK-E09-04 — Implementar catálogo e administração editorial

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E09-04` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E09-04` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M4 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E09-03` |
| Bloqueia | `TASK-E09-05`, `TASK-E09-08`, `TASK-E09-09` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 8–20 dias úteis |
| Risco | alto |

## Entrega

API Axum/PostgreSQL, busca, estações, publicação e UI admin auditável.

## Boundary

- **Não** implementa upload, quarentena, probe, transcode ou o estado do asset que a publicação exige. O pipeline com outbox e workers é `TASK-E09-05`, bloqueada por esta; aqui se lê o `asset_id` e se recusa publicar asset que não esteja pronto, sem ser dono de como ele ficou pronto.
- **Não** entrega o resolve, o token assinado nem o comportamento de CDN, apesar de o catálogo alimentar os dois. `TASK-E09-06` é a dona do delivery; um `GET /v1/catalog/tracks` correto não prova que a URL de playback expira nem que rights bloqueia resolve.
- **Não** implementa autenticação, emissão de token, PKCE, revogação nem o isolamento de conta que o catálogo pressupõe. `TASK-E09-03` é dependência desta e entrega a identidade; esta TASK consome o sujeito e a role que ele afirma, e reimplementar a decisão de token aqui criaria dois pontos de autorização.
- **Não** sincroniza biblioteca, playlist, favorito, bookmark ou progresso do ouvinte. O changelog, os cursores de sync e a UI de conflitos são `TASK-E09-08`, bloqueada por esta; o cursor de paginação do catálogo é outro cursor, com outra semântica, e compartilhar os dois faria o cliente tratar catálogo como biblioteca.
- **Não** entrega o registry de temas e plugins, com publisher, assinatura e revogação de versão. É `TASK-E09-09`, também bloqueado por esta; as operações `/v1/admin/packages` aparecem no mesmo contrato por conveniência de arquivo, não por dono.
- **Não** faz busca na biblioteca local do player nem indexação de arquivos em disco: FTS, filtros e views da biblioteca do desktop são `TASK-E03-05`. O catálogo publicado é do servidor; busca local é da biblioteca do player, com outro índice e sem conta, e a busca desta TASK é só a consulta ao catálogo publicado.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e09-04-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E09-04-AC-1` | um PostgreSQL de teste semeado com 250 faixas publicadas em que várias compartilham a mesma `revision`, e um `limit` de 50 | o cliente percorre as páginas com o `next_cursor` opaco devolvido até `next_cursor` vir nulo, enquanto entre a página 2 e a 3 uma faixa é publicada, uma é despublicada e uma tem o título alterado, e o teste compara a união das páginas com o conjunto de IDs semeados | Paginação estável |
| `TASK-E09-04-AC-2` | três faixas em rascunho, uma pronta mas nunca publicada e duas publicadas, com um token de role listener e os UUIDs de todas conhecidos | o listener lista o catálogo com busca vazia e com termo que casa só com faixa não publicada, e pede `GET /v1/catalog/tracks/{id}` para o ID de cada faixa não publicada | conteúdo não publicado invisível ao ouvinte |
| `TASK-E09-04-AC-3` | quatro contas, uma por role (listener, editor, operator, admin), cada uma com token válido, e o conjunto de operações que o contrato marca com `catalog.read`, `admin.catalog` e `admin.ingest` | cada uma das quatro contas chama cada uma dessas operações e o teste tabula o status devolvido por role e por operação, com os mesmos IDs e o mesmo corpo em todas as chamadas | roles distintas |
| `TASK-E09-04-AC-4` | um token de editor e uma faixa que passou por criação, um PATCH com `base_revision` corrente e um PATCH com `base_revision` já superada, seguido de um pedido de publicação | o teste lê o registro de auditoria depois das quatro operações e confere ator, ID do objeto, revisão resultante, timestamp e resultado de cada uma, incluindo a recusada por revisão antiga | operações editoriais têm audit trail. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | queries de busca, paginação e auditoria rodadas contra PostgreSQL de teste com 250 faixas semeadas (`docker run postgres` funciona aqui, `docker version` responde 29.8.1; `psql` não está no PATH), mais o `EXPLAIN` do plano da consulta paginada conferindo o índice e o `ORDER BY` com desempate explícito, e a contagem de linhas de auditoria por operação | Não existe backend nenhum hoje: 0 arquivos `.sql` no repo, nenhum dos 23 `Cargo.toml` é de servidor, e o `Cargo.lock` tem 165 pacotes sem `axum`, `sqlx` ou `tokio`. No contrato, `TrackPage` e `StationPage` têm `items` e `next_cursor` e o parâmetro `cursor` é `type: string` sem `description`, então AC-1 não tem hoje nem a definição de cursor nem um servidor que o emita. AC-2 não tem onde ser expresso: a palavra `published` aparece 0 vezes em `schemas/openapi-platform.json` e `Track` e `Station` não têm campo de estado ou data de publicação, o que significa que o contrato atual descreveria o catálogo não publicado como catálogo. Para AC-3 existe só a matéria-prima: `x-required-scope` separa `catalog.read` (4 operações), `admin.catalog` (5) e `admin.ingest` (3), e nenhum teste cruza escopo com role. Para AC-4 existe a operação `POST /v1/admin/tracks/{id}/publish` e nada mais: não há tabela, evento nem schema de audit no repo, e `markdown_disagreements` de `check-docs` só compara texto do backlog com o markdown. A evidência a produzir: duas passadas de paginação com as mesmas 250 linhas devolvendo os mesmos 250 IDs em ordens compatíveis, uma faixa nova aparecendo só depois da página 3, as quatro não publicadas ausentes das duas respostas e do 404, a matriz role por operação com os status esperados e uma linha de auditoria por operação aceita mais uma para a recusada. |
| Hardware | nenhum: catálogo e administração editorial são API e banco, e a prova roda em processo mais um container PostgreSQL nesta máquina; nenhum arquivo de mídia é decodificado por esta TASK e não há placa de áudio, Android, iOS ou macOS envolvidos | ausência é limitação registrada, não aprovação |

## DoD — pronto para review

- [ ] Todos os critérios de aceite com evidência
- [ ] `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` limpos
- [ ] `cargo test` verde; testes ignorados listados com motivo
- [ ] Spec do módulo atualizada se a implementação revelou algo novo
- [ ] `backlog.json` com o estado correto
- [ ] Revisão feita por contexto limpo, não pelo implementador
- [ ] PR aberto com template preenchido; CI Gate verde no SHA final
## Notas de execução

Preenchido durante a execução, não antes.
