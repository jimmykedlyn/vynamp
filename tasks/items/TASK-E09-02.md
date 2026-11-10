# TASK-E09-02 — Materializar OpenAPI v1 e contract tests

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E09-02` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E09-02` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M4 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E09-01`, `TASK-E05-01` |
| Bloqueia | `TASK-E05-10`, `TASK-E09-03` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 8–20 dias úteis |
| Risco | alto |

## Entrega

OpenAPI de catálogo/resolve/sync/stations, erros e testkit do provider.

## Boundary

- **Não** implementa nenhum dos dois lados que o contrato descreve. O provider mock e o `SourceDescriptor` são `TASK-E05-01`, e a API Axum/PostgreSQL com busca e publicação é `TASK-E09-04`; aqui se escreve e se prova a forma, não o comportamento por trás dela.
- **Não** implementa identidade, PKCE, keychain ou revogação. `TASK-E09-03` é dona disso; esta TASK declara que existe um escopo por operação e um `bearerAuth`, e nada além, porque um teste de contrato que já falasse com um IdP estaria provando autenticação e não contrato.
- **Não** decide territórios, janelas de disponibilidade nem custo por ouvinte-hora. O conteúdo de `Rights` vem da RFC de `TASK-E09-01`, que é dependência desta; o contrato apenas dá forma aos campos que ela definiu, e inventar política aqui deixaria a RFC sem autoridade.
- **Não** entrega upload assinado, quarentena, probe ou workers de transcode, apesar de o arquivo já listar `/v1/admin/assets/upload`, `/v1/admin/assets/{id}/finalize` e `/v1/admin/jobs/{id}`. O pipeline é `TASK-E09-05`; aquelas operações ficam congeladas no papel até lá.
- **Não** mede carga, latência ou disponibilidade. Contract tests são semântica, não performance: as 1000 sessões e o p95 de 250 ms excluindo CDN são `TASK-E09-10`, e um contrato verde não diz nada sobre SLO.
- **Não** implementa o adapter do cliente que consome o contrato, com renovação de URL e restrições. Isso é `TASK-E05-10`, bloqueada por esta; o testkit daqui serve para ela consumir, não para substituí-la.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e09-02-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E09-02-AC-1` | o `schemas/openapi-platform.json` no estado de hoje: OpenAPI 3.1.0, 25 paths, 26 operações, 31 schemas em `components` todos com `additionalProperties: false`, 11 `enum` e zero chaves `example` ou `examples` no arquivo inteiro | cada operação passa a ter exemplo de request e de response e um validador no `xtask` confere cada exemplo contra o schema da própria operação, reprovando o exemplo que violar `required`, `enum` ou `additionalProperties: false` | Exemplos validam |
| `TASK-E09-02-AC-2` | o provider mock do desktop de `TASK-E05-01`, que hoje não existe (o `apps/player/src/main.rs` tem 3 linhas e um único `println!`), e um servidor real ainda não implementado | a mesma suíte de contract tests roda contra o mock e contra o servidor e compara, por operação, o status HTTP, o `code` do Problem e a forma do JSON, reprovando qualquer divergência entre os dois lados | mock e real compartilham contrato |
| `TASK-E09-02-AC-3` | os 5 parâmetros `cursor` de query, as 13 operações que exigem o header `Idempotency-Key`, os 26 valores de `x-required-scope` e os 11 `enum` do arquivo, mais um cliente que envia valor de enum que o contrato não conhece | o contrato fixa cursor opaco com expiração, `limit` de 1 a 100, a janela de retenção da Idempotency-Key e o escopo exigido por operação, e os contract tests enviam cursor expirado, a mesma key com corpo diferente e enum desconhecido em campo obrigatório | cursor/idempotência/escopos e enums desconhecidos são definidos. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `./target/debug/xtask.exe check-docs` no umbrella mais o validador novo de exemplos por operação dentro do `xtask`, e `cargo test --package xtask --locked`, que hoje soma 126 testes (90 unitários em `xtask/src/`, dos quais 34 em `validate.rs`, e 36 de integração em `xtask/tests/tooling.rs`) | O que existe hoje é validação estrutural, e ela passa: `check-docs` confere `openapi == "3.1.0"`, que todo `$ref` resolve (`api_refs`), que toda operação tem `responses`, que os parâmetros de path batem com o template da URL, que os 26 `operationId` são únicos e que cada um dos 31 schemas passa no meta-schema do `jsonschema`, imprimindo `openapi_paths: 25`, `operation_count: 26`, `errors: []`, `result: passed`. Ela não valida exemplo nenhum, porque o arquivo tem zero chaves `example` — AC-1 parte de uma cobertura de 0 de 26 operações. Também não há teste cruzando `x-required-scope` com a lista de escopos de `architecture/04-contratos.md`: o arquivo usa `account.self`, `registry.read`, `admin.catalog`, `admin.ingest` e `admin.registry` e não tem o `admin.edit` que aquela página cita, divergência que o gate atual não vê. O cursor dos 5 GETs é `type: string` sem `description` nem expiração declarada. Para AC-2 não existe o segundo lado: `apps/player/src/main.rs` tem 3 linhas e o `Cargo.lock` tem 165 pacotes sem nenhum `axum`, `sqlx`, `reqwest` ou `tower`. A evidência a produzir é a matriz operação por operação com exemplo válido e um caso adversarial por operação, e a lista de divergências mock contra servidor vazia nos três campos comparados. |
| Hardware | nenhum: contrato e exemplos são validados em processo por `cargo test --package xtask` e pelos contract tests contra o mock rodando na mesma máquina, sem servidor publicar rede; nenhum teste abre placa de áudio, Android, iOS ou macOS e a única peça externa seria o mock, que é código | ausência é limitação registrada, não aprovação |

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
