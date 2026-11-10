# TASK-E11-02 — Definir fronteiras de segurança

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E11-02` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E11-02` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M0 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E11-01` |
| Bloqueia | `TASK-E06-02`, `TASK-E07-05`, `TASK-E11-05` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Threat model aplicado, defaults de limites, regras de IPC/pacotes e prova de sandbox.

## Boundary

- **Não** implementa keychain nem sandbox por plataforma. A AC-3 prova o isolamento
  do *helper* de privilégio; os adapters por host são `TASK-E14-02`.
- **Não** faz auditoria de dependência vulnerável nem SBOM: isso é `TASK-E11-04`.
  Aqui a superfície é segredo, limite de entrada e regra de IPC.
- **Não** aplica proteção de branch no GitHub: é decisão do mantenedor, e
  `TASK-E19-06` registra o que foi aplicado.
- **Não** promete ausência de vulnerabilidade. O que se entrega é threat model com
  controle e teste mapeados por entrada; o risco residual fica registrado, não zerado.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e11-02-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E11-02-AC-1` | o threat model com cada entrada de ameaça catalogada | cada entrada é confrontada com o controle e o teste que a cobre | Cada entrada tem controle e teste mapeados |
| `TASK-E11-02-AC-2` | o repositório e o CI no estado do commit | a varredura de segredos e a política de permissões de token rodam | segredos protegidos |
| `TASK-E11-02-AC-3` | o helper de privilégio e o host que o invoca | a prova de sandbox roda e o resultado é anexado à documentação do isolamento | isolamento de helper documentado com evidência. |

## Verificação
| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `cargo xtask check-secrets` sobre o histórico alcançável, mais leitura das permissões de token em cada `uses:` de `.github/workflows/` | `No credential shape found in the reachable history.` e cada action fixada por SHA de 40 hex com `permissions: contents: read` — leitura da API e do arquivo, não "parece certo" |
| Hardware | nenhum: a prova de sandbox do helper roda no runner hospedado, sem dispositivo de áudio | ausência é limitação registrada, não aprovação |

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
