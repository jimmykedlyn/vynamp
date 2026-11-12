# Os 10 repositórios

Contagem final: **1 umbrella + 5 libs + 3 apps + 1 referência = 10**.
Criados nesta reorganização: 7. Mantidos: 3. Arquivado: 1. Fora das specs: 2.

Todos em `github.com/jimmykedlyn/`, org do mantenedor.

---

## 1. Tabela

| # | Repo | Tipo | Existe? | Visibilidade | Branch padrão | CI | Submódulo em |
|---|---|---|---|---|---|---|---|
| 1 | `vynamp` | umbrella | **criar** | privado → público na E19-06 | `develop` | Gate completo | — |
| 2 | `vynamp-dsp` | lib | **criar** | privado → público | `develop` | fmt + clippy + test | `libs/dsp` |
| 3 | `vynamp-decode` | lib | existe | privado → público | `develop` | fmt + clippy + test | `libs/decode` |
| 4 | `vynamp-encode` | lib | **criar** | privado → público | `develop` | fmt + clippy + test | `libs/encode` |
| 5 | `vynamp-output` | lib | existe | privado → público | `develop` | fmt + clippy + test | `libs/output` |
| 6 | `vynamp-engine` | lib | **criar** | privado → público | `develop` | fmt + clippy + test | `libs/engine` |
| 7 | `vynamp-player` | app | **criar** | privado → público | `develop` | Gate + build por alvo | `apps/player` |
| 8 | `vynamp-converter` | app | **criar** | privado → público | `develop` | Gate + build por alvo | `apps/converter` |
| 9 | `vynamp-restore` | app | **criar** | privado → público | `develop` | Gate + build por alvo | `apps/restore` |
| 10 | `vynamp-mvp-reference` | referência | existe | **privado, sempre** | `main` | nenhum | `reference/vynamp-mvp` (pin por commit) |

Fora da contagem e fora das specs novas:

| Repo | Situação |
|---|---|
| `vynamp-mediaplayer` | **arquivar** após o primeiro commit verde do umbrella. Segue legível, read-only. |
| `vitrola` | fica como está. 2 commits, 0 `.rs`. Fora de todas as specs (M10). |

---

## 2. Visibilidade — decisão do mantenedor, não do agente

A `ADR-014` determina repositórios públicos para que o CI Gate do GitHub Actions
funcione sem 403 (C9). Mas **quem muda visibilidade é o mantenedor**, na
TASK-E19-06, com a varredura de segredos apresentada antes e a palavra
"pode publicar" registrada.

Regra do agente, sem exceção:

> O agente não altera visibilidade, proteção de branch, assinatura de commit nem
> plano do GitHub por conta própria. `gh repo edit --visibility` só roda se o
> mantenedor pedir explicitamente, na hora, depois de ver a varredura.

### Varredura de segredos — estado atual

| Escopo | Árvore de trabalho | Histórico | Veredito |
|---|---|---|---|
| `repo/` | 0 | 0 (todas as revs) | limpo |
| `vynamp-decode` | 0 | 0 | limpo |
| `vynamp-output` | 0 | 0 | limpo |
| `vynamp-mvp-reference` | 0 | 58 hits, todos `-----BEGIN CERTIFICATE` em `playwright-core/lib/coreBundle.js` | falso positivo (CA do Playwright) |
| `.claude/settings.local.json` (raiz, não versionado) | **1 JWT Woodpecker literal** | não versionado | **achado real** |

**Antes de qualquer repo novo ficar público:**

1. Revogar o JWT Woodpecker no serviço (morto, mas o token existe).
2. Remover a regra de `.claude/settings.local.json` (passo 30) e garantir que o
   arquivo **não** é copiado para o umbrella.
3. Repetir a varredura em cada repo novo **depois** do primeiro commit: os 7 novos
   nascem vazios, então o risco é o que for copiado da raiz.
4. Apresentar o resultado e esperar "pode publicar".

---

## 3. Estrutura comum a todo repo de módulo

```
vynamp-<nome>/
├── AGENTS.md              uma linha: "o contrato é o do umbrella" + link
├── README.md              o que é o módulo, como compilar isolado
├── Cargo.toml             independente, sem [workspace] do umbrella
├── rust-toolchain.toml
├── .gitignore
├── .github/workflows/ci.yml
├── specs/
│   ├── <modulo>.md        a spec do módulo (template padroes/spec.md)
│   └── plans/             planos derivados
├── tasks/items/           só as TASKs deste módulo
├── docs/evidence/PR/      evidência por task
└── src/                   o código
```

**Cada repo compila sozinho.** `git clone` + `cargo test` tem de funcionar sem o
umbrella. É a razão de M9 existir: uma lib que exige o workspace do umbrella não é
uma lib independente.

### `AGENTS.md` de módulo

```markdown
# AGENTS.md

O contrato do agente é o do umbrella: https://github.com/jimmykedlyn/vynamp/blob/develop/AGENTS.md

Específico deste módulo:
- <regra local, se houver>
```

Não repete arquitetura, DDD, SDD nem fluxo — esses são iguais em todo lugar. Se uma
regra local precisar contrariar o umbrella, o problema é no umbrella, e a correção
vai lá.

---

## 4. Gitflow

Igual em todos os 10 repos:

```
main      ← release, protegida, tag de versão
develop   ← integração, protegida, branch padrão
feature/* fix/* chore/* docs/*   ← trabalho
```

| Regra | Valor |
|---|---|
| Merge em `develop` | merge commit pelo GitHub, feito pelo mantenedor |
| Merge em `main` | release, com tag `vX.Y.Z` |
| Force push | proibido, inclusive em feature branch própria |
| Histórico | nunca reescrito |
| Conventional Commits | obrigatório |
| Branch de TASK | `feature/task-<id>-<slug>` |
| Worktree | `cargo xtask repo worktree task-<id>-<slug>` |

O umbrella fixa submódulos em **tag de release**, nunca em branch. Atualizar um
submódulo é uma mudança deliberada, com commit próprio.

---

## 5. CI por repo

### Repos de lib (5)

```yaml
# .github/workflows/ci.yml
on: [push, pull_request]
jobs:
  gate:
    runs-on: ubuntu-latest
    steps:
      - checkout
      - cargo fmt --check
      - cargo clippy --all-targets -- -D warnings
      - cargo test
      - cargo machete
```

Zero testes passa: o repo nasce verde e continua verde. Teste de referência que
depende de fixture grande fica atrás de `#[ignore]` com motivo escrito e variável de
ambiente documentada (M7) — e o CI ganha um job que provê o fixture.

### Repos de app (3)

Gate acima + build por alvo da matriz de suporte. Android entra com cross-build;
macOS Intel e Silicon exigem runner nativo, não emulador.

### Umbrella (1)

Gate + verificação de arquitetura (as 5 regras de `01-arvore-final.md` §3) +
validação do índice de docs + dependabot.

### Referência (1)

Nenhum. É read-only e não entra em build.

---

## 6. Ordem de criação (passo 35)

| # | Ação | Pré-requisito |
|---|---|---|
| 1 | Criar `vynamp` (privado) | nenhum |
| 2 | Criar os 5 repos de lib (privados) | nenhum |
| 3 | Criar os 3 repos de app (privados) | nenhum |
| 4 | Semear os 7 repos com `Cargo.toml`, `Cargo.lock`, `src/` e CI — **não podem ficar vazios** | 1–3 |
| 4b | Submódulos no umbrella apontando para `develop` inicialmente | 4 |
| 5 | Primeiro commit verde do umbrella | 4b |
| 6 | Submódulos passam a apontar para tags | 5 |
| 7 | **Arquivar `vynamp-mediaplayer`** | 5 e 6 |

O passo 7 é o único irreversível sem `gh repo unarchive`, e ganha confirmação própria
no console.

**Por que o passo 4 existe.** `git submodule add` contra um repositório sem nenhum
commit falha com `fatal: unable to checkout submodule`. E `cargo --locked` — que o
CI usa — falha sem `Cargo.lock`: `cannot create the lock file ... because --locked
was passed`. Repo vazio e repo sem lockfile quebram na mesma hora, por isso nenhum
dos 7 nasce vazio.

**Os 3 repos mantidos** (`vynamp-decode`, `vynamp-output`, `vynamp-mvp-reference`)
não passam por 1–4: eles já existem e entram pelo conteúdo que têm — com uma ressalva
medida: o `develop` do **output** tem 2 arquivos e **nenhum `Cargo.toml`**; o código
está em `feature/core`. O submódulo do output é adicionado apontando para `develop`,
como os outros, e o passo 7 fixa o pin — mas o pin do output só vira `v0.1.0` de
verdade depois que `feature/core` entrar em `develop` pelo PR dele (D2, item 2). Até
lá o submódulo do output fica em `develop`, e o script do passo 35 diz isso em vez de
fingir que a tag existe.

---

## 7. Proteções de branch — a definir pelo mantenedor

O agente **não configura** isto. Registrado para quando o mantenedor quiser:

| Repo | `develop` | `main` |
|---|---|---|
| umbrella | PR + CI Gate + 1 review | idem + não fast-forward |
| libs/apps | PR + CI Gate | idem + tag obrigatória |

Enquanto o CI Gate é o único portão automatizado, `CI ausente/pendente/falho ≠ pronto
para merge` continua valendo por decisão do agente, não por configuração do GitHub.
