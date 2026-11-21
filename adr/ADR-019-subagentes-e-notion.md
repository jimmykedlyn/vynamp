# ADR-019 — subagentes do Claude Code e backlog no Notion

Versão: 0.1.0 · Data: 2026-11-21 · Estado: decisão do mantenedor, tomada em 2026-11-21
antes do código que a implementa.

Status: aprovada pelo mantenedor na mesma data. Substitui a nota de 2026-10-23 da
[ADR-007](ADR-007-entrega.md) (tiers de modelo) e a §1–§2 da
[ADR-018](ADR-018-entrega-automatica.md) (merge pelo `cargo xtask deliver`). O resto das
duas continua valendo.

## Contexto

Entre 2026-11-14 e 2026-11-21 o projeto foi operado por um control plane externo
(Paperclip), com issues próprias (`JKC-*`), heartbeat e agentes que acordavam por fila.
O mantenedor decidiu voltar a desenvolver direto no Claude Code, com a sessão principal
orquestrando subagentes, e mover a fonte de status das TASKs para uma base de backlog no
Notion. Três decisões anteriores colidem com isso:

1. `AGENTS.md` §4.7–4.8: `tasks/backlog.json` é a fonte única de status, e `docs-sync`
   reescreve `STATE.md`, `tasks/NEXT.md` e `tasks/ORDER.md` a partir dele.
2. ADR-007 (nota de 2026-10-23): Opus orquestra, Sonnet implementa e revisa, Haiku checa.
3. ADR-018: o único caminho de merge do agente é o `cargo xtask deliver`, que nunca chegou
   a `develop` (o código está numa branch local não publicada).

## Decisão

### 1. Fonte de status: Notion

O status vivo de cada TASK mora na base de backlog do Notion, com os campos `TASK`,
`Título`, `Módulo`, `Fase`, `Onda`, `Prio`, `Depende de` e `Comandos` (link para a página
com os passos), mais um campo de status. `tasks/backlog.json` continua versionado como
**especificação**: título, módulo, requisitos, dependências e critério. O campo `status`
dele fica congelado em 2026-11-21 e só define o rigor do `check-tasks` (TASK `planned`
pode ter `Given`/`When` em `PENDENTE`; qualquer outra não). Quem leva uma TASK além de
`planned` atualiza esse campo no mesmo PR.

`cargo xtask docs-sync` sai. `STATE.md` e `tasks/NEXT.md` viram retrato congelado;
`tasks/ORDER.md` vai para `_arquivo/raiz/`.

### 2. Papéis e modelos

| Agente | Onde vive | Modelo | Effort |
|---|---|---|---|
| orquestrador | sessão principal (regras no `AGENTS.md`) | `claude-opus-5-5` | medium |
| implementador | `.claude/agents/implementador.md` | `claude-opus-5-5` | high |
| checador | `.claude/agents/checador.md` | `claude-sonnet-5-5` | medium |
| revisor | `.claude/agents/revisor.md` | `claude-sonnet-5-5` | high |
| integrador | `.claude/agents/integrador.md` | `claude-sonnet-5-5` | medium |

O orquestrador é a sessão principal porque subagentes não chamam subagentes. A tabela
original do mantenedor pedia `claude-sonnet-5` para o checador; o ID não existe, e foi
trocado por `claude-sonnet-5-5`.

### 3. Merge pelo integrador

O subagente `integrador` é o único agente que mergeia. As condições são as da ADR-018 §1,
sem o requisito de o PR ter sido aberto pelo `deliver`: base `develop`, head SHA igual ao
revisado (`--match-head-commit`), todos os checks em `SUCCESS` incluindo `CI Gate`,
`mergeStateStatus` `CLEAN`, merge commit. Merge em `main`, tag, release, visibilidade e
proteção de branch continuam sendo do mantenedor.

### 4. Branch de TASK

O `guard-branch` passa a aceitar `task/TASK-EII-NN`, além das formas anteriores.

## Alternativas

- **Manter `backlog.json` como fonte e sincronizar com o Notion.** Rejeitada: duas fontes
  vivas divergem, e o sincronizador seria outro control plane.
- **Remover `backlog.json` e os gates `check-tasks`/`check-authoring`.** Rejeitada pelo
  mantenedor: reescreve cerca de 50 usos no xtask e tira dois jobs do CI Gate sem ganho
  para o fluxo novo.
- **Integrador só abre o PR; o mantenedor mergeia.** Não escolhida: o mantenedor pediu
  explicitamente que o integrador faça o merge.

## Consequências

- **O status deixa de viajar com o clone.** Quem clona sem acesso ao Notion vê o retrato de
  2026-11-21, não o estado atual.
- **O `check-tasks` passa a depender de alguém lembrar de atualizar o `status` congelado.**
  Esquecer não deixa o gate vermelho; deixa-o frouxo para aquela TASK.
- **`cargo xtask repo worktree --task` ainda valida contra `backlog.json`.** TASK criada só
  no Notion é recusada até entrar no backlog.
- **O merge volta a ter agente no laço sem um runner testado.** As condições da §3 são
  conferidas pelo integrador via `gh`, não por código com teste; é disciplina, não gate.
- **Os IDs `JKC-*` citados em specs e no threat model apontam para um tracker
  descontinuado.** Ficam como registro histórico.
