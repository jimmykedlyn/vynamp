# TASK-E09-08 — Entregar sincronização de dados do ouvinte

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E09-08` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E09-08` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M4 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E09-03`, `TASK-E09-04`, `TASK-E03-10` |
| Bloqueia | `TASK-E09-10` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 8–20 dias úteis |
| Risco | alto |

## Entrega

Outbox, changelog, cursores, revisões, tombstones e UI de conflitos.

## Boundary

- **Não** implementa o schema SQLite, as migrations nem a fila de reprodução local. `TASK-E03-01` e `TASK-E03-02` são donas disso; a revisão daqui é de objeto sincronizado, e versionar a tabela local daqui faria dois donos da mesma migration.
- **Não** implementa OIDC/PKCE, roles, isolamento por conta nem revogação de token. `TASK-E09-03` é a dona e é dependência desta TASK; o 409 daqui é de revisão de objeto, não de sessão, e checar escopo de conta aqui duplicaria a autorização.
- **Não** implementa catálogo, busca nem publicação editorial. Isso é `TASK-E09-04`, outra dependência; playlists e favoritos são do ouvinte, e misturar os dois domínios faria o change log carregar objeto de catálogo.
- **Não** define o contrato OpenAPI de sync, o cursor opaco nem os enums de erro. `TASK-E09-02` é a dona; aqui se implementa contra o contrato, e inventar o formato do cursor nesta TASK faria o mock provider e a API divergirem.
- **Não** implementa recovery, export nem o gate de biblioteca de 100 mil itens. `TASK-E03-10` é a dona e é dependência desta TASK; o conflito daqui é entre cliente e servidor, não o restore local depois de crash.
- **Não** decide a política de conflito de progresso de audiobook. `TASK-E08-09` é a dona do progresso offline e da retomada explícita, e a spec proíbe o máximo sempre vence; o sync daqui entrega o evento e a revisão, e escolher o vencedor aqui contrariaria aquela TASK.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e09-08-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E09-08-AC-1` | uma playlist com IDs de entrada estáveis e1 a e4 na revisão 7 no servidor e na revisão 7 no cliente, o cliente offline move e4 para a posição 1 e remove e2, e enquanto isso outro cliente publica a revisão 8 movendo e3 para a posição 2 | o outbox do cliente offline volta e envia o lote com `base_revision: 7` para `POST /v1/sync/operations`, e a playlist resultante é lida pelo changelog e pela API | Merge offline de reorder/remove preserva intenção |
| `TASK-E09-08-AC-2` | o mesmo objeto com a edição local do cliente offline sobre a revisão 7, e o servidor já na revisão 9 com uma alteração do mesmo objeto vinda de outro dispositivo | o cliente submete o lote com `base_revision: 7` e recebe 409, e a UI de conflitos é aberta mostrando o valor local e o valor remoto do objeto | 409 preserva edição |
| `TASK-E09-08-AC-3` | uma requisição de sync com `Idempotency-Key` fixa cujo primeiro envio teve timeout de leitura depois de o servidor gravar o changelog | o cliente reenvia o mesmo corpo com a mesma chave, o processo é reiniciado e o outbox reenvia uma terceira vez, e as três respostas são comparadas com o estado do changelog | operação repetida idempotente |
| `TASK-E09-08-AC-4` | uma biblioteca local com `/home/jimmy/Música/Faixa.flac` e `D:/Musicas/Faixa.flac` nos metadados, e o cliente na configuração padrão, sem opt-in de compartilhamento de caminho | playlists, favoritos, bookmarks e progresso são enfileirados e o lote de sync é enviado ao servidor, que o processa e grava no changelog | paths locais não sobem por padrão. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | dois lotes de sync autorados à mão sobre a fixture de playlist e1 a e4 de `tests/fixtures/library/` (pasta inexistente hoje: `tests/fixtures/` só tem `README.md`), enviados com `curl` para `POST /v1/sync/operations`, o segundo com o mesmo corpo e a mesma `Idempotency-Key`, mais um `GET /v1/sync/changes` a partir do cursor anterior e um lote com o campo `path` apontando para `D:/Musicas/Faixa.flac` | a evidência tem que ser: a playlist resultante com e4 na posição 1, e2 como tombstone e e3 na posição 2, isto é 3 entradas vivas mais 1 tombstone, e revisão 10 sem reordenar o que o outro cliente não tocou; o reenvio devolvendo corpo idêntico byte a byte e a contagem do changelog subindo 1, não 3; o 409 trazendo o objeto atual com a revisão 9 e o rascunho local preservado na UI com os dois valores visíveis; e o lote com caminho recusado com 422 e código estável, com 0 ocorrência de `D:/Musicas` ou `Musica` na tabela de changelog e no log. Nada disso existe hoje: não há rota de sync, não há tabela de changelog e não há servidor — 168 arquivos `.rs` no umbrella e nenhum é HTTP, sem `axum`, `sqlx` ou `tokio` em `Cargo.toml` algum, e `cargo test` na raiz roda só `xtask`.
| Hardware | nenhum: as provas são JSON, HTTP e tabela de changelog e nenhum teste abre dispositivo de áudio; os dois clientes são processos no mesmo Windows, então não falta aparelho Android nem iOS, o que falta é o servidor — `postgres:18-alpine` está no cache Docker local, mas não há API nenhuma para subir com ele | ausência é limitação registrada, não aprovação |

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
