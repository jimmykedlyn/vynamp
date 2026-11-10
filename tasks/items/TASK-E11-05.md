# TASK-E11-05 — Implementar config, logs e keychain

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E11-05` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E11-05` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M2 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E11-02`, `TASK-E03-01` |
| Bloqueia | `TASK-E11-06` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–12 dias úteis |
| Risco | médio |

## Entrega

Schema/migrations de configuração, escrita atômica, redaction e retenção.

## Boundary

- **Não** implementa o schema SQLite da biblioteca: WAL, write actor e migrations de DB são `TASK-E03-01`, e o config daqui é arquivo versionado separado do banco.
- **Não** implementa os adapters de keychain por host (Windows Credential Manager, Linux Secret Service, macOS Keychain, Android Keystore): o contrato e a falha fechada quando o vault não existe são `TASK-E14-02`.
- **Não** define o contrato de perfil portable e de owner único entre instalado e portable: lock de perfil e identidade canônica do armazenamento são `TASK-E16-08`, e aqui só se consome o caminho de perfil que ele define.
- **Não** implementa a tela de preferências que expõe retenção e telemetria ao usuário: a UI e os estados de erro são `TASK-E02-06`.
- **Não** confunde telemetria com analytics de produto: aqui só se prova o valor inicial do flag de envio, conforme a política de `security/02-privacidade-licencas.md`. A finalidade dos dados e o texto de privacidade publicados são `TASK-E11-08`, e a prova de que nada vaza em export ou log é `TASK-E10-09`; nenhuma das duas cabe neste schema.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e11-05-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E11-05-AC-1` | um log de diagnóstico gerado com um token de sessão, uma URL com credencial na query e um path de usuário sob %APPDATA%, e o conjunto de padrões de redaction ligado | o diagnóstico sanitizado é exportado e o arquivo de config é regravado pelo caminho de escrita atômica | Fixtures de token/URL/path redacted |
| `TASK-E11-05-AC-2` | o perfil portable apontado para uma pasta em mídia removível, com o vault do OS indisponível e o modo portable escolhido explicitamente | a sessão guarda a credencial de uma conta e o processo é encerrado, e depois o arquivo de config do perfil é aberto como texto plano | portable não grava secrets plaintext |
| `TASK-E11-05-AC-3` | uma gravação de config em curso com o arquivo temporário já escrito e o rename ainda não feito, e o processo morto por kill deixando o temporário no diretório do perfil | o app reabre o perfil e lê a config | crash permite recovery |
| `TASK-E11-05-AC-4` | uma instalação nova, sem arquivo de config prévio e sem consentimento de crash report ou telemetria registrado | o app sobe pela primeira vez e consulta o estado de telemetria antes de qualquer envio | telemetria desligada por padrão. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | uma fixture nova em tests/fixtures/config/ com três entradas (token em header, URL com credencial na query, path dentro de %APPDATA%) e um teste de integração no crate de config do app; hoje o umbrella tem 168 arquivos .rs e nenhum crate de config — só o xtask —, então a fixture e o teste ainda não existem | cada linha do diagnóstico exportado traz <redacted> no lugar do token, da query e do path, e a busca pelo literal do token no config portable devolve 0 ocorrências; a config sobrevivente ao kill é a última consistente, não o temporário, e a retenção fica em 20 MiB e 7 dias conforme security/02-privacidade-licencas.md, com telemetria em estado off em instalação nova |
| Hardware | nenhum: config, log e redaction são exercitados em processo de usuário, sem abrir dispositivo de áudio; o vault OS é o Windows Credential Manager nesta máquina e a ausência dele — Linux Secret Service e macOS Keychain não existem aqui — é justamente o caso portable que a AC-2 cobre | ausência é limitação registrada, não aprovação |

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
