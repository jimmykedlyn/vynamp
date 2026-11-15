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
| Estado | implemented |
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

`TASK-E11-02-AC-3` foi **movido** para `TASK-E07-05` por decisão do mantenedor em
**2026-11-15** (`optionIds: ["split"]`). Esta TASK fecha com AC-1 e AC-2. A razão da
divisão: o helper não existe, e a prova de sandbox não roda sem o processo — implementá-lo
aqui seria trabalho de duas TASKs de outro módulo.

O texto integral do critério movido, para que não se perca entre as duas TASKs:

> `TASK-E07-05-AC-5` (recebido de `TASK-E11-02-AC-3`) — **Given** o helper de privilégio
> e o host que o invoca, **When** a prova de sandbox roda e o resultado é anexado à
> documentação do isolamento, **Then** isolamento de helper documentado com evidência.
> É a evidência que fecha o `AC-3` de `TASK-E11-02`.

**Pendência de destino:** essa linha ainda **não** está em
`apps/player/tasks/items/TASK-E07-05.md`. O arquivo pertence ao repo `vynamp-player`,
outro módulo dono, e esta TASK não o edita (`AGENTS.md` §4.2). Quem leva o critério para
lá é uma TASK do repo do player; até isso acontecer, `E07-05` é implementável contra os
próprios quatro ACs sem que nenhum peça a evidência de isolamento.

**Efeito no `acceptance` do REQ.** O `acceptance` de `REQ-E11-02` no backlog e em
`product/03-requisitos.md` continua com as três cláusulas, incluindo "isolamento de
helper documentado com evidência". O texto **não** foi reescrito: é requisito aprovado, e
mexer nele exige decisão registrada. O que o estado `implemented` afirma é o escopo
desta TASK — AC-1 e AC-2 verdes —, e a terceira cláusula do REQ fica registrada como
fechada por `TASK-E07-05`, não por aqui.

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

`security/01-threat-model.md` v0.8.3 ganhou a coluna `Teste que existe hoje`, com
`arquivo:linha` e nome da função para cada teste que roda. A coluna carrega **22
referências `arquivo:linha`**, todas distintas: 21 resolvem para a `fn` de teste nomeada
ao lado (`sed -n <linha>p`) e a vigésima segunda, `…/src/metadata.rs:69`, para
`pub struct RawBlock` — é a citação de código que sustenta o "capa não tem parser a
atacar", não uma referência a teste. O documento inteiro tem **35 ocorrências / 33
distintas** (`grep -oE '([A-Za-z0-9_./-]+\.rs)?:[0-9]+(-[0-9]+)?' | sort | uniq -c`), porque
as tabelas de `Limits` repetem `:204` e `:424`. A suíte do `vynamp-decode` rodou verde:
**517 passed, 0 failed, 3 ignored**.

A coluna de controle carrega três afirmações que a revisão derrubou e que foram
corrigidas contra o tree, não por leitura da versão anterior: `max_depth` **não é
aplicado** por demuxer nenhum (`libs/decode/specs/decode.md:339-341`), `max_bytes` é
teto do tamanho **declarado** e não de leitura acumulada, e o dono de `IDOR` é
`TASK-E09-03`, não `E09-08` — cuja Boundary diz isso na própria linha.

Os três ignorados, com motivo, como o `AGENTS.md` §4.6 exige:

| Teste | Motivo declarado |
|---|---|
| `blackbox::intensity_probe_gains` (`vynamp-decode-mp3/src/layer3/tests/blackbox.rs:489`) | `writes and analyses scratch files for the black-box comparison` |
| `blackbox::write_blackbox_streams` (`vynamp-decode-mp3/src/layer3/tests/blackbox.rs:285`) | `writes scratch files for the black-box comparison` |
| doctest de `vynamp-decode-testkit/src/alloc.rs:8` | `ignore` — exemplo de `#[global_allocator]`, não compila como doctest |

Resultado por entrada: **1 de 10 com cobertura parcial** (mídia/tag/capa — budgets e
parsers com testes reais; bomb e capa sem teste, com dona nomeada) e **9 de 10 sem
teste**, cada uma com a TASK dona do teste que falta. Nenhuma entrada foi declarada
coberta sem teste: o produto não tem cliente HTTP, empacotador, runtime de plugin,
helper, updater, banco nem backend, então não há o que atacar.

### AC-2 — segredos e política de token

| Verificação | Comando | Resultado |
|---|---|---|
| Varredura de segredo no histórico | `cargo xtask check-secrets` | `No credential shape found in the reachable history.`, exit 0 |
| Token read-only por workflow | leitura de `.github/workflows/*.yml` | `ci.yml:13` e `quality.yml:15` declaram `permissions: contents: read`; nenhum job sobrescreve, e nenhuma chave tem valor fora de `read`/`none` |
| Action fixada por SHA | `grep -n "uses:" .github/workflows/*.yml` | 7 ocorrências, todas `actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1` (40 hex) |

A política de token era **afirmada e não verificada**: nada reprovava a remoção do bloco
`permissions:`. Foi fechada com TDD — testes vermelhos primeiro, depois o gate em
`xtask/src/validate.rs` (`github_actions`). O gate cobre o bloco de workflow **e** o de
job, porque um `permissions:` de job substitui o do workflow inteiro; e rejeita
**qualquer** chave com valor diferente de `read`/`none`, não só `contents` — a primeira
versão lia só `contents`, e `id-token: write` passava. `github_actions_rejects_any_wide_permission_key`
ficou vermelho antes da regra (`got []`) e verde depois;
`github_actions_accepts_permissions_named_none` guarda o outro lado.

A primeira correção exigia `contents: read` também no bloco de **job**, o que reprovava
um job que **estreita** a permissão (`permissions: {}` ou `contents: none`) com a
mensagem "widens the token". `none` é mais estreito que `read`, e a política declarada é
"`read` ou `none`" — o gate punha vermelho no `pre-push` quem endurecia a configuração, e
a saída óbvia era voltar para `read`. Corrigido com TDD:
`github_actions_accepts_a_job_that_narrows_the_token` ficou vermelho antes da regra
(`Job ci/specs widens the token`) e verde depois. O laço de job continua load-bearing: com
ele desligado, `github_actions_rejects_a_job_that_widens_the_token` e
`github_actions_rejects_any_wide_permission_key` ficam vermelhos.

**Onde o gate roda:** no `pre-push` local (`.githooks/pre-push` → `cargo xtask ci docs`
→ `check-delivery`). **Não** roda em workflow nenhum de `.github/workflows/` neste
commit — medido com `grep -rn "check-delivery\|ci docs" .github/workflows/`. Quem fecha
é `TASK-E15-04` (`8d11700`), ainda fora de `develop`.

**As formas abreviadas passavam.** O laço de job percorria
`perms.as_object().into_iter().flatten()`; `permissions: write-all` e `read-all` não são
um mapa, então `as_object()` devolvia `None`, o iterador ficava vazio e **nenhum `check`
rodava** — `write-all` num job passava com zero erro. Era regressão em relação a
`d126e19`, onde o mesmo fixture reprovava. Fechado com TDD:
`github_actions_rejects_the_shorthand_permission_forms` ficou vermelho antes da regra
(`a job block with permissions: write-all must fail the gate, got []`) e verde depois,
cobrindo `write-all`, `read-all` e a forma entre aspas, no bloco de workflow e no de job.
A regra virou `check_permissions`, usada pelas duas posições. Mutação: desligar só o ramo
de string deixa o teste vermelho de novo.

### AC-3 — isolamento de helper: **movido para `TASK-E07-05`**

O helper de privilégio e o host que o invoca **não existem**. Comandos que provaram:

```
$ grep -rniE '\bhelper\b|ipc|named pipe|unix socket' --include=*.rs --include=*.toml \
    libs apps xtask | grep -v target
$ grep -rn 'Command::new' libs/*/crates libs/*/src apps/*/src
```

O primeiro devolve cinco ocorrências, todas comentário ou nome de função de teste. O
segundo devolve zero em `libs/` e `apps/`: nenhum código de produto cria processo filho.
Os três `apps/*/src/main.rs` são um `println!` de versão. Sem processo, não há prova de
sandbox a rodar: filho que cai, RPC malicioso e revisão de escape precisam do binário.

**Decisão do mantenedor em 2026-11-15** (`optionIds: ["split"]`): o AC-3 foi movido para
`TASK-E07-05` (sandbox OS, RPC autenticado), sobre os adapters de `TASK-E14-02`. Esta
TASK fecha com AC-1 e AC-2, e o estado no backlog vai a `implemented`.

**O critério recebido ainda não está no arquivo de `E07-05`.** Medido:
`grep -c "AC-5" apps/player/tasks/items/TASK-E07-05.md` devolve **0**, e o arquivo tem
quatro ACs. O texto integral está na seção "Critérios de aceite" acima, e a pendência de
destino está registrada logo abaixo dela. Quem leva a linha para o repo do player é uma
TASK daquele repo — decisão do mantenedor em 2026-11-15 (`optionIds: ["new_task"]`).

### Comandos e resultado

| Comando | Resultado |
|---|---|
| `cargo test --package xtask --locked` | 124 + 36 passed, 0 failed |
| `cargo fmt --all -- --check` | limpo |
| `cargo clippy --package xtask --all-targets --locked -- -D warnings` | limpo |
| `cargo xtask check-docs` | `"result": "passed"`, 215 TASKs |
| `cargo xtask check-tasks` | `"result": "passed"` |
| `cargo xtask check-authoring` | `"result": "passed"`, 0 blocking |
| `cargo xtask check-delivery` | `"result": "passed"`, 2 workflows |
| `cargo test --workspace` em `libs/decode` | 517 passed, 0 failed, 3 ignored (listados acima) |

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
