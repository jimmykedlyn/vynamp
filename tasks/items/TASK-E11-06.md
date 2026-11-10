# TASK-E11-06 — Implementar updater assinado

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E11-06` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E11-06` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E11-04`, `TASK-E11-05` |
| Bloqueia | `TASK-E10-09`, `TASK-E11-07` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | médio |

## Entrega

Verificação de metadados/artefatos, canais, expiry, rotação e rollback.

## Boundary

- **Não** constrói nem assina os artefatos por plataforma: NSIS, DEB, AppImage, DMG/ZIP Universal2 e APK/AAB são `TASK-E11-07`, e aqui se verifica o que ele produz, não como é produzido.
- **Não** roda os drills de kill, migration e update rollback nem o teste de vazamento em export e log: essa evidência agregada é `TASK-E10-09`.
- **Não** é o CI e a supply chain que emitem os artefatos e os hashes: jobs, pins por SHA e SBOM são `TASK-E11-04`, e o updater consome o resultado deles.
- **Não** implementa o armazenamento de credencial nem a redaction dos logs de diagnóstico: keychain e retenção são `TASK-E11-05`, dependência direta desta TASK.
- **Não** faz o rollout canary, não corta stable 1.0 e não decide canal: isso é `TASK-E11-09`, depois de o updater existir.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e11-06-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E11-06-AC-1` | três bundles do canal beta num fixture de update: um com o payload do manifesto alterado depois da assinatura, um com validade expirada no manifesto e um já registrado como aplicado no estado local | o updater verifica cada bundle contra a chave pública pinned antes de qualquer escrita no diretório de instalação | Rejeitar tamper/stale/replay |
| `TASK-E11-06-AC-2` | um bundle aceito na verificação, com o diretório de staging vazio, o binário em uso e os oito nomes de artefato de SUFFIXES em xtask/src/release.rs | o updater baixa o bundle, verifica o manifesto e o usuário confirma aplicar | staging antes aplicar |
| `TASK-E11-06-AC-3` | playback ativo de uma faixa local e uma gravação de rádio em curso, com o bundle já verificado e pronto para trocar binários | o usuário confirma a atualização enquanto o áudio está tocando e a gravação está gravando | não interromper playback/gravação automaticamente |
| `TASK-E11-06-AC-4` | um banco SQLite da biblioteca na versão N, com a migração N+1 empacotada no update e um backup consistente tomado pelo owner antes da troca | o update é aplicado e o app reabre o banco pelo processo de migração | DB preservado. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | um fixture de quatro manifestos em tests/fixtures/update/ (tamper, expirado, replay, válido) assinado por uma chave minisign de teste, conduzido pelo caminho de publish-draft de xtask/src/release.rs; hoje esse arquivo só monta o conjunto exato de artefatos e assina manifest.json, e packaging/update.pub não existe — o próprio preflight falha com Missing release prerequisite: packaging/update.pub | os três manifestos inválidos são recusados com motivo distinto (assinatura, expiração, replay) e nenhum byte é escrito em staging; o válido passa e o swap acontece só depois da verificação. O que ainda não existe: rejeição de expiração e replay — release.rs hoje não as implementa —, staging atômico com health check e rollback, e o checksum do DB igual antes e depois da migração |
| Hardware | nenhum: verificação de assinatura, expiração e staging são operações de arquivo e criptografia, e o updater roda na máquina onde o app está instalado; nenhum dispositivo de áudio é aberto, e o mac e o Android não entram porque os artefatos deles são escopo de TASK-E11-07 | ausência é limitação registrada, não aprovação |

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
