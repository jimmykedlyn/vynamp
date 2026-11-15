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
  controle e teste mapeados por entrada; o risco residual fica registrado, não zerado. A revisão que classifica achados é `TASK-E07-10`.

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

Executado em 2026-11-15. O estado abaixo foi medido, não herdado do plano.

### AC-1 — controle e teste por entrada

`security/01-threat-model.md` v0.8.0 ganhou a coluna `Teste que existe hoje`, com
`arquivo:linha` e nome da função para cada teste que roda. As 23 referências citadas
foram conferidas uma a uma contra o tree (`sed -n <linha>p` devolveu a `fn` esperada em
todas) e a suíte do `vynamp-decode` rodou verde: **517 passed, 0 failed, 0 ignored**.

Resultado por entrada: **1 de 10 com cobertura parcial** (mídia/tag/capa — budgets e
parsers com testes reais; bomb e capa sem teste, com dona nomeada) e **9 de 10 sem
teste**, cada uma com a TASK dona do teste que falta. Nenhuma entrada foi declarada
coberta sem teste: o produto não tem cliente HTTP, empacotador, runtime de plugin,
helper, updater, banco nem backend, então não há o que atacar.

### AC-2 — segredos e política de token

| Verificação | Comando | Resultado |
|---|---|---|
| Varredura de segredo no histórico | `cargo xtask check-secrets` | `No credential shape found in the reachable history.`, exit 0 |
| Token read-only por workflow | leitura de `.github/workflows/*.yml` | `ci.yml:13` e `quality.yml:15` declaram `permissions: contents: read`; nenhum job sobrescreve |
| Action fixada por SHA | `grep -n "uses:" .github/workflows/*.yml` | 7 ocorrências, todas `actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1` (40 hex) |

A política de token era **afirmada e não verificada**: nada em CI reprovava a remoção do
bloco `permissions:`. Foi fechada com TDD — dois testes vermelhos primeiro
(`github_actions_requires_the_read_only_token_policy`,
`github_actions_rejects_a_job_that_widens_the_token`), depois o gate em
`xtask/src/validate.rs` (`github_actions`). O gate cobre o bloco de workflow **e** o de
job, porque um `permissions:` de job substitui o do workflow inteiro.

### AC-3 — isolamento de helper: **não satisfeito**

O helper de privilégio e o host que o invoca **não existem**. Comando que provou:

```
$ grep -rniE '\bhelper\b|ipc|named pipe|unix socket' --include=*.rs --include=*.toml \
    libs apps xtask | grep -v target
```

Quatro ocorrências, todas comentário ou nome de função de teste. `apps/player/src/main.rs`
é um `println!` de versão. Sem processo, não há prova de sandbox a rodar: filho que cai,
RPC malicioso e revisão de escape precisam do binário.

Registrado como ausência medida, não como pendência disfarçada. O que fecha o AC é
`TASK-E07-05` (sandbox OS, RPC autenticado), sobre os adapters de `TASK-E14-02` — ambas
fora do Boundary desta TASK. Por isso o estado no backlog **continua `planned`**: dois
critérios de três estão entregues, e marcar `implemented` seria declarar pronto sem
evidência.

### Comandos e resultado

| Comando | Resultado |
|---|---|
| `cargo test --package xtask --bin xtask` | 120 passed, 0 failed |
| `cargo fmt --all -- --check` | limpo |
| `cargo clippy --package xtask --all-targets --locked -- -D warnings` | limpo |
| `cargo xtask check-docs` | `"result": "passed"`, 215 TASKs |
| `cargo xtask check-tasks` | `"result": "passed"` |
| `cargo xtask check-authoring` | `"result": "passed"`, 0 blocking |
| `cargo xtask check-delivery` | `"result": "passed"`, 2 workflows |
| `cargo test --workspace` em `libs/decode` | 517 passed, 0 failed, 0 ignored |

### Limitações registradas

- `cargo xtask ci docs` **falha neste runner** por motivo ambiental, não do repositório:
  o clone do harness tem `origin` = `https://github.com/jimmykedlyn/vynamp` (sem `.git`)
  e `config/github.json` declara `…/vynamp.git`. Com o `origin` alinhado ao arquivo,
  `check-delivery` passa com `"errors": []`. O `origin` foi restaurado ao valor original
  depois da medição; a divergência é do ambiente, não do commit.
- Os submódulos estavam vazios no início da execução. Foram inicializados
  (`git submodule update --init --recursive`) para que a coluna de teste do AC-1 fosse
  medida contra código real em vez de inferida. Nenhum arquivo de submódulo foi editado.
- Nenhum hardware, áudio real ou dispositivo foi tocado. Nenhuma prova de sandbox foi
  executada, porque não há helper.
