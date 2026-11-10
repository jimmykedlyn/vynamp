# TASK-E09-03 — Implementar identidade e autorização

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E09-03` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E09-03` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M4 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E09-02` |
| Bloqueia | `TASK-E08-10`, `TASK-E09-04`, `TASK-E09-08`, `TASK-E16-10` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 8–20 dias úteis |
| Risco | alto |

## Entrega

OIDC/PKCE, keychain do cliente, roles, isolamento por conta e revogação.

## Boundary

- **Não** define a forma dos endpoints nem a tabela de escopos por operação. O contrato e os contract tests são `TASK-E09-02`, dependência desta; aqui se faz valer o que ele declarou, e redefinir rota ou escopo aqui deixaria duas versões do mesmo contrato.
- **Não** registra as rotas HTTP próprias, a verificação HTTPS do host de callback nem o tratamento de deep link que trazem o retorno do OAuth. Isso é `TASK-E16-10`, bloqueada por esta; abrir um handler de HTTP global no player para o callback contradiria o contrato de deep link e criaria dois lugares que aceitam URL externa.
- **Não** entrega o controle remoto autenticado, com pairing, TLS em LAN e rotação de token do servidor local. É `TASK-E08-10`, também bloqueada por esta; o token que ele emite é outro, com outro público, e reaproveitar aqui o access token da plataforma faria o controle remoto valer como credencial de catálogo.
- **Não** constrói o vault de configuração, o schema de settings, a redaction de log nem a retenção que guardam o refresh token. `TASK-E11-05` é a dona do keychain e do redaction; esta TASK define o que precisa ser guardado e exige que o token não apareça em log, não o mecanismo de armazenamento.
- **Não** decide direitos, territórios ou janela de disponibilidade, nem bloqueia mídia por território. Autenticar e autorizar por conta é diferente de checar direito: a negação por rights no resolve é `TASK-E09-06`, e a política de território é `TASK-E09-01`.
- **Não** produz o audit trail editorial nem o registro de quem publicou o quê. As operações editoriais auditáveis são `TASK-E09-04`, bloqueada por esta; o log de autenticação e o audit de domínio são registros distintos e misturá-los apagaria a distinção entre quem entrou e quem alterou.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e09-03-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E09-03-AC-1` | duas contas A e B, cada uma com playlist, bookmark e job próprios com UUIDs conhecidos, e um access token válido da conta A | a conta A envia, em cada endpoint que aceita ID no path, requisições com o ID de um objeto da conta B, com token válido e corpo sintaticamente aceitável | Negar IDOR/acesso entre contas |
| `TASK-E09-03-AC-2` | o cliente desktop sem sessão, com issuer OIDC e client ID lidos de configuração confiável e não de URL de estação, e o navegador do sistema disponível para o authorize | o usuário dispara o login, o cliente abre o navegador do sistema com `code_challenge` S256 e `state`, recebe o callback em loopback, confere o `state` e troca o code por tokens no endpoint de token | login pelo browser |
| `TASK-E09-03-AC-3` | um resolve bem-sucedido que devolve `refresh_reference` opaca e um locator de mídia assinado em host de CDN diferente do host da API, com o access token da API vivo na memória do cliente | o cliente baixa os bytes do locator pela CDN e o teste inspeciona os headers da requisição de mídia e a URL usada, inclusive depois de um renew que trocou o locator | não enviar bearer à CDN |
| `TASK-E09-03-AC-4` | um perfil com sessão ativa, refresh token no keychain do SO, cache privado de catálogo e resolve, biblioteca local apontando para arquivos de mídia que estão no disco e preferências de UI não privadas | o usuário faz logout e o cliente revoga o refresh token no servidor, apaga o token do keychain e limpa o cache privado | logout limpa apenas dados privados. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | contract tests de IDOR e de auth contra o mock em processo e contra o servidor de teste com PostgreSQL em container (`docker version` responde 29.8.1 nesta máquina; `psql` não está no PATH, então o banco sobe por container), mais a captura dos headers e da URL da requisição de bytes à CDN e a leitura do keychain do SO antes e depois do logout | Não existe nada disso hoje, e a evidência tem que dizer isso em vez de herdar um verde: o `Cargo.lock` tem 165 pacotes e zero ocorrências de `keyring`, `oidc`, `oauth2`, `openidconnect` ou `jsonwebtoken`; `apps/player/src/main.rs` são 3 linhas com um `println!` e nenhum cliente HTTP; não há servidor nenhum no repo (0 arquivos `.sql` e nenhum dos 23 `Cargo.toml` é de backend). O contrato declara só `bearerAuth` como `http`/`bearer` com `bearerFormat: OAuth2 access token`, e nada sobre issuer, PKCE, refresh ou keychain. O que o teste tem que mostrar quando existir: A pedindo objeto de B recebe 404 e não 403, para não confirmar existência; o authorize carrega `code_challenge_method=S256` e `state` conferido, com callback recusado quando o `state` diverge; a requisição de bytes à CDN não tem header `Authorization` e a única credencial é a query assinada do locator, antes e depois do renew; e depois do logout o refresh token antigo é rejeitado no servidor, o keychain não devolve mais token e o cache privado está vazio enquanto o arquivo de mídia local continua abrindo normalmente. |
| Hardware | não há hardware específico nem placa de áudio envolvida; o que falta é software e serviço: um issuer OIDC de teste não existe, o crate de keychain não está no `Cargo.lock` (0 pacotes `keyring`) e não há cliente nem servidor implementados. O navegador do sistema para o authorize existe neste Windows 11, e não há Android, iOS ou macOS no escopo | ausência é limitação registrada, não aprovação |

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
