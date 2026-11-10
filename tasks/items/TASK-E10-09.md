# TASK-E10-09 — Validar upgrades, recovery e privacidade

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E10-09` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E10-09` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E03-10`, `TASK-E11-06`, `TASK-E10-05` |
| Bloqueia | `TASK-E10-10` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | médio |

## Entrega

Drills de kill/migration/update rollback e testes de vazamento em export/log.

## Boundary

- **Não** implementa a assinatura, o expiry, a rotação de chave nem o staging do updater. É `TASK-E11-06`, dependência desta TASK; aqui se ataca o que ele verificou, e reescrever a verificação aqui faria o teste e o objeto testado serem o mesmo código.
- **Não** implementa schema de configuração, migration, escrita atômica, redaction nem retenção de log. É `TASK-E11-05`; esta TASK usa a redaction e falha se ela vazar, e implementá-la aqui permitiria o teste passar por construção.
- **Não** entrega crash restore, backup e export da fila nem o relatório de 100k. É `TASK-E03-10`, também dependência; a AC-1 mede a preservação do que ele restaurou.
- **Não** faz fuzz de parser, de pacote ou de RPC contra entrada hostil. É `TASK-E10-05`; replay de update é uma forma nomeada e assinada, não uma entrada arbitrária de parser.
- **Não** instala nem atualiza em Linux, macOS, Android ou iOS. É `TASK-E14-05`, e o artefato assinado de release é `TASK-E15-10`; o drill daqui é local, em Windows x64.
- **Não** reescreve o schema SQLite nem a atomicidade das migrations. É `TASK-E03-01`; a AC-1 depende dela e só confere que os dados sobreviveram, sem abrir a caixa da migration.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e10-09-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E10-09-AC-1` | um perfil com dados que não se reconstroem (contagem de reproduções, ratings, bookmarks e a fila salva) em SQLite com migration pendente, mais uma cópia do arquivo e das tabelas exportadas tirada antes do drill | o processo é morto à força no meio da escrita da migration, o perfil é reaberto e o conteúdo das tabelas é comparado com a cópia por hash | Dados irreproduzíveis preservados |
| `TASK-E10-09-AC-2` | um manifesto de update assinado com chave de teste e o artefato correspondente, mais duas variantes: a mesma assinatura com um byte alterado no artefato e um manifesto antigo, ainda válido, apresentado de novo depois de já aplicado | as três entradas são apresentadas ao updater de `TASK-E11-06`, que verifica assinatura, versão e janela de validade em cada uma e devolve um veredito por entrada | update adulterado/replay rejeitado |
| `TASK-E10-09-AC-3` | uma fixture com token de teste conhecido no campo de credencial de rádio e no cabeçalho de uma requisição, mais o log rotativo e o export `.vyndiag` produzidos depois de usar essa credencial, com a opção de incluir dados adicionais do `.vyndiag` ligada numa execução e desligada na outra | o log e o conteúdo descompactado do `.vyndiag` são varridos pelo token literal, pela forma decodificada e pela forma truncada, e o resultado da varredura entra no relatório | token de teste não aparece em logs/diagnósticos. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | um script de drill sobre um perfil temporário que dispara a migration, mata o processo no meio da escrita, reabre e compara as tabelas exportadas por `sha256sum` com a cópia, mais `cargo xtask check-secrets` e uma varredura `grep -r` do token de teste no log rotativo e no `.vyndiag` descompactado | Hoje a única peça executável é `cargo xtask check-secrets`, que responde `No credential shape found in the reachable history.` com exit 0 e varre blobs de git por forma de credencial — não lê log de execução nem export. Não há objeto para atacar: nenhum `.rs` contém `redact`, nenhum contém `vyndiag`, `packaging/update.pub` não existe e `minisign` aparece só em `xtask/src/release.rs` nas linhas 266 e 280, no caminho de publicação. `cargo xtask release preflight --version 0.1.0` falha antes de qualquer verificação de update, com `Release disabled: actual player/build/hardware/signing acceptance missing`, e `cargo xtask check-tasks` reporta 199 TASKs com 188 em planned, então o updater e o export da AC-3 ainda não existem para provar nada sobre eles. |
| Hardware | nenhum: os drills operam sobre arquivo SQLite temporário, log rotativo e artefato assinado com chave de teste no disco, sem abrir dispositivo de áudio nem sair da máquina | ausência é limitação registrada, não aprovação |

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
