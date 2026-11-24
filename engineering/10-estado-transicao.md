# Estado de transição

Retrato datado. Este arquivo é o lugar de "como está hoje"; o `AGENTS.md` só guarda regra.
Cada item tem um critério de saída: cumprido, o item sai daqui.

**Desde 2026-11-24 o andamento está no GitHub** ([ADR-023](../adr/ADR-023-andamento-no-github.md)).
As seções 1–3 retratam a base do Notion antes da migração. Os itens marcados "Notion" na
seção 2 são resolvidos pela migração, que deriva o estado do GitHub (seção 6).

**Conferido em 2026-11-22**, lendo a base TASKs do Notion (pelo MCP) e os 21 índices de
épico (`E01.md`…`E21.md`).

## 1. Números

| Fonte | TASKs | Observação |
|---|---|---|
| `tasks/backlog.json` | 215 | 213 nos épicos + `TASK-CI-01/02` |
| Índices de épico | 213 | 14 `implemented`, 199 `planned` |
| Notion | 200 | 6 com `Entregue` |

As 13 TASKs dos épicos que faltam no Notion são todas `implemented` (`E01-01/02/04`,
`E02-11`, `E04-02`, `E11-01/03`, `E15-03`, `E19-01/02/04/05/06`). Somadas a `CI-01/02`, dão
as 15 que a migração deixou de fora de propósito. Módulo e fase batem entre Notion e
épicos em todas as 200.

Distribuição por módulo (épicos): `vynamp-player` 98, `vynamp` 57, `vynamp-decode` 17,
`vynamp-encode` 16, `vynamp-engine` 15, `vynamp-output` 6, `vynamp-converter` 2,
`vynamp-dsp` 1, `vynamp-restore` 1.

## 2. Inconsistências a corrigir

| # | Onde | Achado | Saída |
|---|---|---|---|
| 1 | Notion | `TASK-E01-05` e `TASK-E01-07` têm `Entregue` sem `PR do módulo` | preencher `PR`/`Merge commit` pela consulta de Comandos |
| 2 | Notion | `TASK-E04-05` e `TASK-E21-01` têm `Entregue` sem `Plano aprovado` | registrar na migração (ADR-020); plano não é retroativo |
| 3 | Notion × backlog | `TASK-E11-02` (`vynamp`, M0) está `implemented` no backlog e pendente no Notion | decidir qual está certo; corrigir o outro |
| 4 | backlog | as 6 entregues continuam `planned` | some com a ADR-020 (status deixa de ser andamento) |
| 5 | Notion | não se sabe se as 5 entregues de lib já têm repin no umbrella | rodar o "Conferir" de cada uma; marcar `Integrada` ou abrir o repin |
| 7 | backlog | se o `backlog.json` guarda a Onda, ele ainda tem a Onda antiga das 18 TASKs movidas (seção 3) | atualizar no mesmo PR de spec |
| 10 | Épicos | a coluna "Estado" mostra `planned` para TASK já entregue | tirar a coluna dos índices, ou gerá-la a partir do Project |

(No item 5 são cinco TASKs de lib: `E01-03`, `E01-05` em engine; `E01-07` em decode;
`E04-05` em output; `E21-01` em encode. A `E02-01` é do player, que não tem repin.)

## 3. Ordem de execução — feito em 2026-11-22

As ondas antigas eram só a profundidade das dependências: todas as 200 TASKs estavam na
onda mais cedo possível, sem olhar a Fase. Isso punha o iOS inteiro (F6) nas Ondas 01–07 e
12 TASKs de M5 entre as Ondas 04 e 11, antes da release 1.0 (Onda 13). No Notion:

- as faixas de Onda passaram a seguir o roadmap (`engineering/notion.md`, "Faixas de Onda");
- 18 TASKs mudaram de Onda;
- a descrição da base deixou de citar o `ORDER.md`;
- a página *Ordem de execução corrigida* foi criada, com uma visão vinculada da base.

| TASK | Fase | Onda antes | Onda agora |
|---|---|---|---|
| `TASK-E12-03` | M5 | 07 | 25 |
| `TASK-E12-04` | M5 | 08 | 25 |
| `TASK-E12-05` | M5 | 07 | 25 |
| `TASK-E12-06` | M5 | 05 | 25 |
| `TASK-E12-07` | M5 | 11 | 25 |
| `TASK-E17-13` | M5 | 04 | 25 |
| `TASK-E17-14` | M5 | 04 | 25 |
| `TASK-E17-15` | M5 | 04 | 25 |
| `TASK-E17-16` | M5 | 08 | 25 |
| `TASK-E12-10` | M5 | 08 | 26 |
| `TASK-E17-17` | M5 | 08 | 26 |
| `TASK-E17-18` | M5 | 09 | 26 |
| `TASK-E20-01` | F6 | 01 | 27 |
| `TASK-E20-02` | F6 | 02 | 28 |
| `TASK-E20-03` | F6 | 03 | 28 |
| `TASK-E20-04` | F6 | 06 | 29 |
| `TASK-E20-05` | F6 | 04 | 29 |
| `TASK-E20-06` | F6 | 07 | 30 |

Conferido por script: nessa ordem, nenhuma TASK vem antes de uma dependência sua, e
nenhuma depende de TASK de fase posterior.

Pendências da mesma análise:

- **Caminho crítico da 1.0.** A cadeia mais longa até a `TASK-E11-09` (stable 1.0) tem 17
  TASKs e passa pelo SDK de plugins: `E02-02` → `E06-01` → `E06-02` → `E07-02` → `E07-03`
  → `E07-04` → `E07-06`/`E07-07` → `E07-08` → `E13-08` → `E13-09` → `E13-10`/`E14-05` →
  `E14-06` → `E10-10` → `E15-10` → `E11-09`. Seis delas são P1 (`E07-04`, `E07-06`,
  `E07-07`, `E07-08`, `E10-10`, `E11-09`). Saída: virar P0.
- **`TASK-E15-01`** (Claude Code e Superpowers) é pré-requisito do fluxo de agentes, mas é a
  10ª da Onda 01. Saída: ir primeiro.
- **`TASK-E10-10`** provavelmente deveria depender de `E10-02`, `E10-04` e `E10-06`, os
  únicos testes do épico E10 que ficam de fora dela.
- **38 TASKs de M0 a M3 não são exigidas por nenhum gate da 1.0** (`E14-06`, `E10-10`,
  `E15-10`, `E11-09`). São elas: conversor, restore e encoders (`E18-01`, `E18-02`,
  `E21-02` a `E21-16`); vitrola (`E19-03`, `E19-07`); fidelidade visual (`E02-12` a
  `E02-16`, `E06-10`, `E10-06`); processo (`E15-01`, `E15-02`, `E15-05`); extras do player
  (`E08-03`, `E08-04`, `E08-05`, `E08-07`, `E08-08`, `E14-03`); e `E10-02`, `E10-04`.
  Saída: ou ganham gate de release próprio, ou entram nas dependências de `TASK-E14-06`.

## 4. Dívidas de arquitetura conhecidas

- **Submódulos pinados em commit da `develop`, não em tag.** Saída: primeira tag de cada
  lib e repin do umbrella para ela.
- **Job `arch` do CI do umbrella não existe.** Até ele existir, a tabela do §1.2 é revisada
  no PR. Saída: o job roda `cargo tree`, `grep` e `cargo build --manifest-path` contra a
  tabela.
- **Workspace do Cargo do umbrella tem só `xtask`.**
- **`xtask repo worktree` e `check-tasks` validam contra o `backlog.json`, e a claim é
  local.** Saída: o `xtask` passa a gravar ou consultar `Responsável` no Project do GitHub (exige ADR).

## 5. Perguntas abertas para o mantenedor

1. **Onde mora o mixer?** O §1.4 põe `Mixer` e `Resampler` no `vynamp-dsp`, mas
   `TASK-E01-05` ("Implementar mixer, drain e transições básicas") é do `vynamp-engine`, e
   `TASK-E21-12` fala em "resampler registrado" no `vynamp-encode`. O `vynamp-dsp` tem uma
   única TASK, já entregue (`E04-02`). Se as primitivas estão nascendo fora do `dsp`, o
   argumento do §1.4 (player e conversor fazendo a mesma conta) não se sustenta. Ou as TASKs
   são de orquestração e chamam o `dsp`, e convém dizer isso na spec; ou falta um épico de
   `dsp`.
2. **57 TASKs no umbrella.** O umbrella tem só `xtask` no workspace. Vale confirmar que
   essas TASKs são de spec, QA, gate ou CI, e não código de produto parado no repo errado.
3. **`TASK-E11-02`**: entregue ou não (item 3 acima)?
4. **Conversor e restore entram na 1.0?** Decide o que fazer com as 38 TASKs da seção 3.

## 6. Migração Notion → GitHub (ADR-023)

- **Escopo `project` no token.** O Project (v2) exige `gh auth refresh -s project` pelo
  mantenedor. Saída: `gh auth status` mostra o escopo.
- **Issues e Project não existem ainda.** `cargo xtask migrate-tasks` sem
  `--apply` só gera o plano (`target/migracao-tasks-plano.json`) e só cria com `--apply`, depois do "pode criar" do mantenedor.
  É idempotente: Issue cujo título já começa com o id da TASK não é recriada, e campo já
  preenchido no Project não é regravado (prova: `cargo test --manifest-path xtask/Cargo.toml tasks_github`). Saída: uma
  Issue por TASK `planned`, todas no Project, com `Estado` e `Onda` preenchidos.
- **Onda sem export do Notion.** Sem export (`--waves export.csv`), o comando calcula a Onda pela faixa do marco e
  pela profundidade das dependências; as 18 TASKs ajustadas à mão em 2026-11-22 podem
  divergir. Saída: mantenedor confere a visão por onda no Project.
- **READMEs de módulo ainda citam o Notion** — resolvido em 2026-11-24: vynamp-player#24,
  vynamp-decode#33 e vynamp-engine#20 mergeados em `develop`; `libs/engine` repinado no
  merge commit `ac2f4d2`. O player é app, sem repin.
- **`libs/decode` pinado fora de `develop`.** O umbrella aponta para `4ce2eed`
  (`chore(tasks): E01-07 split into E01-07/11/12/13/14`), que só existe na branch
  `chore/claude-e01-split` do vynamp-decode, sem PR. Repinar no `develop` do decode
  (`a6883d7`, com o #33) apaga os arquivos das TASK-E01-11 a 14 e reprova o
  `check-docs`. Saída: PR dessa branch no vynamp-decode e repin no merge commit.
- **Rastreio incompleto nas TASKs migradas.** A migração não conhece `SHA revisado` nem
  `Responsável`: TASK `Entregue`/`Integrada` sai sem `SHA revisado`, e PR aberto ou
  branch sem PR sai `Bloqueada` com `Motivo`. Exceção aceita só para o que a migração
  criou. Saída: mantenedor decide cada `Bloqueada` da migração.
- **Base TASKs do Notion congelada.** Não é mais lida nem escrita; fica como histórico.
