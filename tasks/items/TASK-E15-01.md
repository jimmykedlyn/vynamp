# TASK-E15-01 — Configurar Claude Code e Superpowers obrigatório

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E15-01` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E15-01` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M0 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | nenhuma |
| Bloqueia | `TASK-E15-02` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Entrypoints, settings, plugin lock e bootstrap/doctor nativos.

## Boundary

- **Não** instala o plugin: `cargo xtask install` instala. A AC-2 exige que a
  **configuração sozinha** não conte como instalação, e é o doctor que tem de reprovar.
  Instalar é ato do operador sobre o manager nativo (ADR-007), sem dono no backlog.
- **Não** prova skill discovery. O `doctor` confirma o plugin manager e a versão; o
  "confirm skill discovery in session" é ele mesmo dizendo que discovery não é o que
  ele mede. A AC-1 fecha essa parte na sessão, não no binário, e essa prova manual em
  sessão não tem dono no backlog: a autoridade é `AGENTS.md` §5.
- **Não** roda em máquina incompatível: o doctor é quem decide a compatibilidade, e a
  matriz de toolchains e ABIs de cada alvo é `TASK-E14-01`.
- **Não** escreve o entrypoint do app. O player é repo próprio; o entrypoint dele é
  `TASK-E02-01`, e aqui é o entrypoint do ferramental de engenharia.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e15-01-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E15-01-AC-1` | uma máquina compatível com o entrypoint do projeto instalado | o bootstrap e o doctor do projeto rodam | Ambos agentes confirmam plugin ativo e skill discovery em máquina compatível |
| `TASK-E15-01-AC-2` | os arquivos de configuração versionados e um diretório de plugins vazio | o doctor roda sem o plugin instalado | configuração sozinha não conta como instalação. |

## Verificação
| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `./target/debug/xtask.exe doctor` no root real, e de novo com `--root` apontando para um root cujo `config/superpowers.lock.json` espera versão falsa | no root real **exit 0** (`claude: plugin manager confirmed Superpowers`); no root adulterado **exit 1** (`Superpowers active/version not confirmed`) — a segunda execução é a que prova a AC-2, e as capturas estão em `evidence/PR/task-e15-01/` |
| Hardware | nenhum: bootstrap e doctor de ferramental não tocam dispositivo de áudio | ausência é limitação registrada, não aprovação |

## DoD — pronto para review

- [ ] Todos os critérios de aceite com evidência
- [ ] `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` limpos
- [ ] `cargo test` verde; testes ignorados listados com motivo
- [ ] Spec do módulo atualizada se a implementação revelou algo novo
- [ ] `backlog.json` com o estado correto
- [ ] Revisão feita por contexto limpo, não pelo implementador
- [ ] PR aberto com template preenchido; CI Gate verde no SHA final
## Notas de execução

Verificada em 2026-11-09 contra o binário construído do commit `b6213b4`, com um
root real e um root adulterado. A TASK **fecha com uma decisão registrada** sobre o
texto do AC-1, que nomeia um agente que não existe mais.

| AC | Verificação executada | Resultado |
|---|---|---|
| AC-1 | `./target/debug/xtask.exe doctor` no root real | **exit 0**, `claude: plugin manager confirmed Superpowers; confirm skill discovery in session`. Confirma manager e versão; o "confirm skill discovery in session" é o próprio doctor dizendo que discovery não é o que ele prova |
| AC-2 | o mesmo `doctor --root C:/Users/jimmy/.claude/tmp/e15-01-root`, com `config/superpowers.lock.json` esperando `expected_versions.claude = 9.9.9` | **exit 1**, `claude: Superpowers active/version not confirmed; verify native /plugin (Claude)`. Configuração sozinha não passa: o veredito vem do `plugin list --json` do manager, não do arquivo versionado |

Capturas em `evidence/PR/task-e15-01/claude-verify-20261109.txt`.

### Decisão: o AC-1 fala de dois agentes, o produto tem um

O `Then` do AC-1 é literalmente "**Ambos** agentes confirmam plugin ativo e skill
discovery em máquina compatível". Isso descrevia um mundo com dois agentes. Esse mundo foi encerrado por decisão
do mantenedor no commit `aa7ed34`, que está em `develop`, e
`doctor --agent` hoje aceita **só** `claude`:

```
--agent <AGENT>  [default: claude] [possible values: claude]
```

Não reescrevo o `Then` (é o requisito aprovado, copiado do backlog) nem finjo que
dois agentes foram confirmados. A leitura que vale: o requisito era *provar que a
ferramenta distingue plugin instalado de configuração versionada, para cada agente
suportado*, e o conjunto de agentes suportados passou a ser `{claude}` por decisão
posterior e explícita. Com um agente, "ambos" é satisfeito por vacuidade do segundo —
e a substância (as duas provas de AC-1 e AC-2) está medida nos dois sentidos.

**Custo se essa leitura estiver errada:** se o projeto voltar a ter um segundo
agente, esta TASK precisa ser reaberta, porque a matriz de dois-agentes nunca foi
provada contra dois binários reais — só contra um.

### O que continua não provado

"Skill discovery" não é verificado pelo doctor, só recomendado na mensagem de saída;
a prova de que uma skill é descoberta e executada numa sessão nova continua sendo
manual. O relatório histórico de 2026-10-16 (`evidence/task-e15-01-qa-009.json`)
registrava dois managers; hoje ele é história, não estado. Nenhuma flag de
prontidão mudou.
