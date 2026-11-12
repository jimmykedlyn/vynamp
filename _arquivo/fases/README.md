# vynamp-fases — FASE 2 (documentos de decisão)

Esta pasta **não é git**. Ela guarda o que a FASE 3 vai executar: os padrões, os
documentos de decisão, o contrato de agente e os 9 passos, cada um com o seu `.md`
(o que muda e por quê) e o seu `.ps1` (o que faz exatamente aquilo).

Nada aqui foi executado. Nenhum arquivo do projeto foi movido, apagado ou enviado.

---

## Ordem de leitura

| # | Arquivo | Para quê |
|---|---|---|
| 1 | `docs/03-contradicoes.md` | as 13 contradições de (b), o status de cada uma e onde ela é resolvida |
| 2 | `docs/01-arvore-final.md` | a árvore final do umbrella: 5 libs, 3 apps, `specs/`, `reference/` fora do build |
| 3 | `docs/04-repos-e-remotes.md` | os 10 repos: visibilidade, default branch, proteção, CI |
| 4 | `docs/02-fusao-specs.md` | o que cada um dos 107 `.md` vira depois de M3/M4/M5/M6/M8 |
| 5 | `docs/00-mapa-destino.csv` | uma linha por arquivo: origem, destino, motivo, passo |
| 6 | `agentes/AGENTS.md` + `agentes/CLAUDE.md` | o único contrato de agente (substitui os dois `AGENTS.md` antigos) |
| 7 | `padroes/` | `spec.md`, `plan.md`, `tasks.md`, `adr.md` + 3 templates de CI |
| 8 | `execucao/30..38` | os 9 passos, em ordem. Leia o `.md` antes de rodar o `.ps1` |

## Estrutura desta pasta

```
vynamp-fases\
├── README.md              ← este arquivo
├── padroes\
│   ├── spec.md  plan.md  tasks.md  adr.md
│   └── ci-lib.yml  ci-app.yml  ci-engine.yml
├── docs\
│   ├── 00-mapa-destino.csv
│   ├── 01-arvore-final.md
│   ├── 02-fusao-specs.md
│   ├── 03-contradicoes.md
│   └── 04-repos-e-remotes.md
├── agentes\
│   ├── AGENTS.md
│   └── CLAUDE.md
└── execucao\
    ├── 30-limpeza.md    + 30-limpeza.ps1
    ├── 31-estrutura.md  + 31-estrutura.ps1
    ├── ...              ...
    └── 38-exclusao.md   + 38-exclusao.ps1
```

## Os 9 passos

| Passo | O que faz | Toca o GitHub? | Apaga? |
|---|---|---|---|
| **30 limpeza** | Move para `_arquivo/` o que não migra; aplica M4 (16 VUs) e M6 (Woodpecker sai); remove o JWT do `settings.local.json`; corrige `.gitignore` | não | não (só move) |
| **31 estrutura** | `Move-Item` de `…\repo` para `D:\Projects\vynamp` (com o `.git`, **menos** `target/` e `.worktrees/`); `git mv` das specs pelo mapa; `Cargo.toml` vira workspace virtual; README novo | não | não |
| **32 agentes** | `AGENTS.md` + `CLAUDE.md` únicos na raiz do umbrella; os dois antigos vão para `_arquivo/` | não | não |
| **33 subagentes** | `.claude/agents/*.md`: tiering Opus/Sonnet/Haiku do `AGENTS.md` | não | não |
| **34 CI** | `.github/workflows/` do `padroes/`; remove Woodpecker por inteiro | não | não |
| **35 github** | Cria os 7 repos novos (privados), troca o origin do umbrella, 9 submódulos, tags `v0.1.0`, arquiva o `vynamp-mediaplayer` **por último** | **sim** | não |
| **36 gitflow** | `main`/`develop`/`feature/*`, proteções que o `gh` consegue declarar, `.gitattributes` | sim | não |
| **37 tasks** | Reescreve as TASKs no formato SDD (Given/When/Then, `Boundary`), um commit por repo | sim (branches, sem merge) | não |
| **38 exclusão** | **O único que apaga.** Grupos A (build), B (não migrado), C (casca do repo antigo) | não | **sim**, sob `pode excluir` |

O passo 35 depende dos 5 PRs da lista D2 mergeados. O 31 depende do 30 commitado.
O 38 depende do 35 (umbrella com `.git` e `_arquivo/INDEX.md`).

## Regras que todo `.ps1` obedece

Por construção, não por disciplina:

- **Nada é apagado direto.** Todo arquivo de projeto que sai vai antes para
  `_arquivo/<caminho original>` com uma linha em `_arquivo/INDEX.md`
  (`| Data | Origem | Destino | Motivo | Passo |`). O índice é idempotente por
  (Data, Origem, Passo): rodar de novo não duplica linha.
- **O passo 38 é o único destrutivo**, e só depois de `pode excluir` digitado no
  console. A lista de caminhos é fechada; qualquer caminho fora dela, ou que seja
  `D:\Projects\vynamp`, outro repo `vynamp*`, `vitrola`, `AMBIENTE.ps1` ou
  `.cache\cargo`, é recusado com erro antes de qualquer `Remove-Item`.
- **Nenhum `git push`, `merge`, `tag` ou `gh repo edit` nos passos 30–34.** Só
  leitura de git e escrita local.
- **Merge é seu.** O 35 empurra branches e cria tags de estrutura; não mergeia
  nada, não muda visibilidade nem proteção (ADR-014 + TASK-E19-06).
- **Nenhum token é lido, impresso ou gravado.** GitHub via `AMBIENTE.ps1` +
  `Invoke-VynampAuthenticatedAgent`; `gh` pega `GH_TOKEN` do ambiente.
- **Idempotentes e param em erro** (`$ErrorActionPreference = 'Stop'`), com
  `-WhatIf` pelo `ShouldProcess` onde há escrita.
- **PowerShell 7+ (`pwsh`).** UTF-8 sem BOM na saída; compatibilidade com 5.1
  abandonada de propósito.

## O que a FASE 3 vai fazer

Na ordem: **3.0** limpeza → **3.1** estrutura → **3.2** agentes → **3.3**
subagentes → **3.4** CI → **3.5** GitHub → **3.6** gitflow → **3.7** tasks →
**3.8** exclusão definitiva. Um passo por vez, um commit convencional por passo,
parando em cada etapa que toque visibilidade ou remoto.

A FASE 3 **não começa** enquanto os 5 PRs da lista D2 não estiverem mergeados:
sem eles, `ADR-014`, `ADR-015`, `tools/03`, a dissolução da vitrola e os 5
arquivos do decode não estão em `develop`, e mover levaria specs sem essas peças.

## Rulings registrados

Decisões tomadas nesta fase, para o caso de alguém comparar com M11 e estranhar:

| # | Ruling | Custo se errado |
|---|---|---|
| **D-GH-2/3** | O umbrella **herda o histórico** do `vynamp-mediaplayer` via troca de remote, em vez de nascer vazio (M11). O passo 31 move o `.git` de qualquer forma; descartá-lo jogaria fora 109 commits que as specs citam. | Os PRs #1–#39 não aparecem no repo novo; as citações a PR precisam do repo explícito. |
| **D-GH-7/8/9/11/12** | Os 7 repos novos nascem **com um commit** (`develop`, `Cargo.toml`, `Cargo.lock`, `src/`); `decode` e `output` entram pelo conteúdo que já têm; CI vai só para os 7 novos. | `submodule add` falha em repo vazio; `--locked` falha sem lockfile. |
| **31 não move build** | `target/` e `.worktrees/` ficam para trás e o 38 apaga. | Nenhum: não são versionados. |
