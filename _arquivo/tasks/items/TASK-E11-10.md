# TASK-E11-10 — Preparar operação da plataforma

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E11-10` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E11-10` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M4 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E09-01` |
| Bloqueia | `TASK-E09-10` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 8–20 dias úteis |
| Risco | médio |

## Entrega

IaC, secrets, DB/PITR, métricas de CDN/custo e runbooks de restore/rollback.

## Boundary

- **Não** define catálogo, direitos, regiões nem orçamento de bandwidth. Esse RFC é `TASK-E09-01`; aqui se provisiona a infraestrutura que ele escolheu, e inverter a ordem faria a IaC decidir escala em vez de executá-la.
- **Não** executa o teste de 1.000 sessões nem mede p95 ou custo por ouvinte-hora. A validação de operação, carga e recuperação é `TASK-E09-10`, que roda depois desta; esta TASK publica a meta RPO/RTO, os alarmes e os runbooks que aquele teste avalia.
- **Não** implementa as rotas de catálogo, resolve ou sync, nem a autorização por objeto. Contrato é `TASK-E09-02` e roles são `TASK-E09-03`; a migração expand/contract daqui é de schema operacional, não de contrato de API.
- **Não** entrega ingest, transcode nem URL assinada. Pipeline e delivery são `TASK-E09-05` e `TASK-E09-06`; as métricas de CDN e ingest desta TASK são observabilidade sobre eles, não a implementação deles.
- **Não** implementa keychain do cliente nem redaction de log local. O keychain do cliente é `TASK-E09-03` e os adapters por host são `TASK-E14-02`; o redaction é `TASK-E11-05`, que cobre config e log local. A prova de métrica sem PII daqui olha o export do servidor e da CDN, não o log do cliente, e as duas cobrem vazamentos por caminhos diferentes.
- **Não** implementa updater nem rollback de atualização no cliente. Verificação de metadados, expiry e rollback do updater são `TASK-E11-06`; o rollback desta TASK é da API e do schema do banco, e confundir os dois faria o rollback de servidor depender de um release de desktop.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e11-10-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E11-10-AC-1` | a IaC aplicada num ambiente de referência com PostgreSQL 16 em `archive_mode=on` e PITR apontando para um bucket S3-compatible, com o backup diário mais recente e o WAL disponível | um drill de restore é executado a partir do backup e do WAL até um timestamp de 10 min antes da falha simulada, cronometrado do início da indisponibilidade até o primeiro 200 da API | Drill avalia RPO 15 min/RTO 4 h |
| `TASK-E11-10-AC-2` | o conjunto de métricas de CDN, ingest e custo com os labels declarados no runbook, e uma fixture de requisição que carrega `/home/jimmy/Music/faixa.flac`, `teste@exemplo.com` e um token sintético `ghp_...` no path | a fixture atravessa o middleware de métricas e a série exportada é confrontada com a lista de labels proibidos do runbook | métricas sem PII |
| `TASK-E11-10-AC-3` | a API na versão N com clientes N em uso, e a versão N+1 com a migração expand já aplicada (coluna aditiva, sem drop) e o contrato de `TASK-E09-02` inalterado | o deploy N+1 é revertido para N com o banco no estado expandido e a suíte de contract tests roda contra N | rollback compatível |
| `TASK-E11-10-AC-4` | os runbooks de restore, rollback e rotação de tokens publicados, e a lista de alarmes de budget e de CDN com o custo por ouvinte-hora registrado | o registro de responsável operacional, com nome e canal de escalação, é conferido contra cada alarme e cada runbook | responsável operacional designado. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | um PostgreSQL 16 com `archive_mode=on` e `restore_command` no bucket, o runbook de restore e três fixtures: um dump diário, uma cadeia de WAL com 10 min de atraso e uma requisição de métrica contendo `/home/jimmy/Music/faixa.flac`, `teste@exemplo.com` e um `ghp_` sintético; o drill é `pg_basebackup` mais `recovery_target_time = now() - 10min`, cronometrado por script, e o segundo é o export da série raspada confrontado com a lista de labels proibidos | o relatório do drill tem que mostrar perda de dados de no máximo 15 min e a API respondendo dentro de 4 h, com timestamp de início e fim no próprio relatório, e a série exportada tem que ter zero ocorrência dos três padrões. O fixture do token não pode ter o formato completo de 36 caracteres depois de `ghp_`, senão `cargo xtask check-secrets` reprova o próprio blob da fixture em `.github/workflows/ci.yml`. Nada disso está no repositório: não há IaC (nenhum `.tf`, nenhum manifesto de deploy), nenhum subcomando de banco nos 14 de `xtask/src/main.rs`, nenhum runbook, e `cargo test` no umbrella roda só o pacote `xtask` — 1.018 linhas em `xtask/tests/tooling.rs` — que não abre banco nem rede. |
| Hardware | nenhum: as provas são de banco, fila, CDN e export de métricas e nenhuma abre dispositivo de áudio, DAC ou aparelho móvel. PostgreSQL 16.1 existe nesta máquina — o serviço `postgresql-x64-16` está em execução e `C:/Program Files/PostgreSQL/16/bin/psql.exe` responde 16.1 — mas o `postgresql.conf` padrão traz `archive_mode` e `archive_command` comentados, então não há PITR, e nenhum cliente de bucket S3-compatible (`aws`, `minio`, `mc`) está instalado; o que falta provisionar é a configuração de arquivamento e o bucket, não o banco | ausência é limitação registrada, não aprovação |

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
