# Base TASKs no Notion

Referência operacional do §8 do `AGENTS.md`. Base **TASKs** em *Vynamp — Guia de
desenvolvimento / Ordem de execução*, data source
`collection://4172db34-4753-811b-98bb-000bb999ffcc`.

## Esquema (após a ADR-020)

| Propriedade | Tipo | Origem | Observação |
|---|---|---|---|
| TASK | título | spec | `TASK-EII-NN` |
| Título | texto | spec | igual ao `backlog.json` |
| Módulo | seleção | spec | um dos 9 repos |
| Fase | seleção | spec | `M0`…`M5`, `F6`; marco do roadmap |
| Onda | seleção | spec | `Onda 01`…`Onda 30`; **ordem de execução**, em faixas por marco (abaixo) |
| Prio | seleção | spec | `P0`, `P1`, `P2`; desempate dentro da onda |
| Depende de | texto | spec | ids separados por vírgula; `—` = nenhuma |
| Ordem | número | legado | posição no antigo `ORDER.md`; não usar para decidir |
| **Estado** | seleção | orquestrador | `AGENTS.md` §5 |
| **Responsável** | texto | orquestrador | `claude@<máquina>/<sessão>` |
| **Branch** | texto | orquestrador | nome exato da branch |
| **PR** | URL | orquestrador | PR do módulo |
| **SHA revisado** | texto | orquestrador | 40 caracteres |
| **Merge commit** | texto | orquestrador | 40 caracteres |
| **Voltas** | número | orquestrador | BLOQUEADOs do revisor |
| **Motivo** | texto | orquestrador | obrigatório em Bloqueada/Cancelada |

Saem após a migração: `Plano aprovado`, `PR do módulo`, `Entregue`.

Os campos de origem "spec" só mudam junto com o `backlog.json`, num PR de spec. O Notion
espelha esses campos; quem manda neles é o backlog.

## Visões

| Visão | Filtro | Ordenação | Agrupamento |
|---|---|---|---|
| Próximas | `Estado = Pronta` | Onda ↑, Prio ↑, TASK ↑ | — |
| Em curso | `Estado` ∈ {Reservada, Em implementação, Em revisão, PR aberto} | Estado, TASK | Responsável |
| Bloqueadas | `Estado = Bloqueada` | TASK | — |
| Por onda | — | Prio ↑, TASK ↑ | Onda |
| Por módulo | — | Onda ↑, TASK ↑ | Módulo (quadro) |

A visão "A fazer" de hoje agrupa por Onda e, dentro de cada onda, ordena por **Fase** antes
de Prio; o contrato diz Onda → Prio. Trocar a ordenação para Prio ↑, TASK ↑.

A página *Ordem de execução corrigida* (filha de *Ordem de execução*) tem uma visão
vinculada da base, "Por onda (ordem corrigida)", com a regra e as decisões pendentes.

## Faixas de Onda

A Onda segue o roadmap. Toda TASK fica na faixa do seu marco e numa onda maior que a de
cada uma das suas dependências.

| Ondas | Marco | Fases |
|---|---|---|
| 01–16 | 1.0 | M0, M1, M2, M3 |
| 17–24 | M4 (plataforma) | M4 |
| 25–26 | M5 (expansões) | M5 |
| 27–30 | F6 (iOS) | F6 |

Não começar TASK de um marco enquanto houver TASK pronta de um marco anterior. Faixa cheia
ganha ondas novas no fim dela, e as faixas seguintes andam junto, num PR de spec.

Até 2026-11-21 as ondas eram só a profundidade das dependências, e isso punha trabalho de
M5 e F6 nas primeiras ondas. Em 2026-11-22, 18 TASKs foram movidas (tabela em
`engineering/estado-transicao.md`).

## Consultas de consistência

O orquestrador roda isto ao começar uma sessão. Qualquer linha que volte é corrigida antes
de pegar TASK nova.

```sql
-- Estado avançado sem rastreio
SELECT "TASK","Estado" FROM "collection://4172db34-4753-811b-98bb-000bb999ffcc"
WHERE ("Estado" IN ('PR aberto','Entregue','Integrada') AND ("PR" IS NULL OR "SHA revisado" IS NULL))
   OR ("Estado" IN ('Entregue','Integrada') AND "Merge commit" IS NULL)
   OR ("Estado" IN ('Reservada','Em implementação','Em revisão','PR aberto') AND ("Responsável" IS NULL OR "Branch" IS NULL))
   OR ("Estado" IN ('Bloqueada','Cancelada') AND "Motivo" IS NULL);
```

Dependências em estado incompatível (o SQL do MCP não tem CTE recursiva) ficam como
conferência do orquestrador, id por id, ao reservar.

## Modelo da página (Comandos)

Todas as 200 páginas seguem o mesmo modelo. A versão nova troca "confira no backlog" por
"confira no Notion" e separa o PR de repin.

````markdown
**Início**

```powershell
$env:TEMP = 'D:\Temp\vynamp'; $env:TMP = $env:TEMP
New-Item -ItemType Directory -Force $env:TEMP | Out-Null
cd D:\Projects\vynamp
git switch develop
git pull --ff-only
git submodule update --init --recursive
claude '<TASK>: início. Leia a TASK no Notion e confira as dependências (Integrada). Reserve, crie a worktree em <caminho do módulo> e me mostre o plano antes de codar.'
```

**Entrega**

```txt
<TASK>: plano aprovado. Siga o fluxo do AGENTS.md §6.3 até o merge em develop; depois abra o repin no umbrella em chore/claude-repin-<módulo>-<slug>.
```

**Conferir** (PowerShell 7)

```powershell
gh api "repos/jimmykedlyn/<repo>/pulls?state=all&head=jimmykedlyn:<branch>" --jq '.[] | "PR #\(.number): \(if .merged_at then "MERGEADO \(.merge_commit_sha)" else .state end)"'
gh api "repos/jimmykedlyn/vynamp/pulls?state=all&head=jimmykedlyn:chore/claude-repin-<módulo>-<slug>" --jq '.[] | "Repin #\(.number): \(if .merged_at then "MERGEADO" else .state end)"'
```
````
