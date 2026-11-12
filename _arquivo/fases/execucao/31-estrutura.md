# Passo 31 — estrutura

Move o repositório para `D:\Projects\vynamp` e reorganiza a árvore com o destino de
cada arquivo do mapa. **Ainda sem submódulos** — eles precisam dos remotes criados no
passo 35.

---

## 1. Pré-requisitos

- [ ] Passo 30 commitado e limpo
- [ ] Os 5 PRs mergeados (lista D2) — o mapa assume `ADR-014`, `ADR-015`, `tools/03`,
      a dissolução da vitrola e os 5 arquivos do decode em `develop`
- [ ] `D:\Projects\vynamp` **vazio** ou inexistente

## 2. O que este passo faz

| # | Ação |
|---|---|
| 1 | `Move-Item` do conteúdo de `…\repo` para `D:\Projects\vynamp`, **menos** `target/` e `.worktrees/` |
| 2 | `git mv` cada arquivo do mapa para o destino "manter"/"reescrever"/"renomear" |
| 3 | Aplica as fusões reais e leva os 17 arquivos que o passo 30 deixou vivos ao destino de cada um |
| 3b | Reescreve os links relativos com o mapa origem→destino (1 940 quebram sem isso) |
| 4 | Cria `libs/` e `apps/` vazios com `.gitkeep` (populados no 35) |
| 5 | Cria `reference/` e põe lá as 13 specs de áudio/ferramentas |
| 6 | Reorganiza `Cargo.toml` como workspace virtual |
| 6b | Corrige os caminhos `docs/` **fora** dos `.md`: 5 literais no Rust, `backlog.json`, `config/`, `assets/` e `.github/` |
| 7 | Reescreve `README.md` do umbrella |

## 3. O que **não** faz

- Não cria submódulo (`git submodule add` é o passo 35, depois dos remotes).
- Não cria repo no GitHub.
- Não faz push, merge nem tag.
- Não apaga nada: o que não tem destino no mapa já foi para `_arquivo/` no passo 30.
- **Não move build.** `target/` (3,1 GB) e `.worktrees/` (11 GB) ficam em
  `…\vynamp-media-player\repo\` e o passo 38 os apaga. Movê-los custaria duas vezes
  o mesmo tempo e o mesmo espaço para o mesmo fim — e nenhum dos dois é versionado,
  então o `.git` que viaja para o umbrella não perde nada com a ausência deles.

## 4. Mapa de movimento por diretório

```
repo/                                   →  D:\Projects\vynamp\
├── docs/INDEX.md                       →  INDEX.md
├── docs/GLOSSARY.md                    →  architecture/GLOSSARY.md
├── docs/architecture/*.md              →  architecture/*.md
├── docs/architecture/adr/*.md          →  adr/*.md
├── docs/product/*.md                   →  product/*.md
├── docs/delivery/*.md                  →  delivery/*.md
├── docs/design/*.md                    →  design/*.md
├── docs/engineering/*.md               →  engineering/*.md
├── docs/extensions/*.md                →  extensions/*.md
├── docs/library/*.md                   →  library/*.md
├── docs/platforms/*.md                 →  platforms/*.md
├── docs/quality/*.md                   →  quality/*.md
├── docs/security/*.md                  →  security/*.md
├── docs/streaming/*.md                 →  streaming/*.md
├── docs/evidence/**                    →  evidence/**
├── tasks/**                            →  tasks/** (itens reescritos no 37)
├── xtask/  config/  schemas/  packaging/  assets/  tests/
│                                       →  idem (mesmo caminho)
├── .github/**                          →  idem (reescrito no 34)
└── dotfiles (.editorconfig, .gitignore, .cargo/, .githooks/, .claude/)
                                        →  idem
```

`docs/superpowers/` já saiu no passo 30. **`docs/audio/` e `docs/tools/` não**: o passo 30
os deixou vivos de propósito, porque é este passo que lê o conteúdo deles. Os 17 arquivos
(`13` de áudio e ferramentas + `engineering/01`, `engineering/06`, `extensions/03`,
`quality/04`, `streaming/04`) são tratados na seção 5.

**`docs/` deixa de existir como diretório** — nenhum caminho `docs/` sobrevive ao passo.
O que sobra dele é `docs/INDEX.md`, que vira `INDEX.md` na raiz e precisa ter todas as
entradas reescritas (ver seção 6).

## 5. Os 17 arquivos que o passo 30 deixou vivos

Medido seção por seção em 2026-11-07. A premissa original do plano — "5 fusões, o resto
é duplicata" — só se sustenta em **um** dos cinco casos.

### 5.1 A única fusão real

| Origem | Destino | O que é duplicata |
|---|---|---|
| `docs/quality/04-integration-codecs-dsp.md` | `quality/01-estrategia-testes.md` | **Só as 12 linhas** `QA-022..QA-033` — conferidas byte a byte contra a tabela de `quality/01`. |

As **três seções de prosa** do mesmo arquivo (`Entrada pelos sistemas`, `Corpus de áudio`,
`DSP, render e rede`, 2 227 caracteres) são **únicas** e **entram junto** em
`quality/01-estrategia-testes.md`, depois da tabela de QA. Sem isso, o conteúdo some.

### 5.2 Quatro arquivos que NÃO são duplicata

Medi a interseção, não a impressão:

| Arquivo | O plano dizia | Medido | Destino |
|---|---|---|---|
| `docs/audio/05-platform-backends.md` | "9 linhas idênticas a 04" | **0 linhas comuns**, similaridade de sentença `0,00`. É o único arquivo que descreve Android/AAudio, obrigação M1–M3 e a matriz de backend por SO. | `evidence/` no umbrella? **Não.** Vira spec de `libs/output` no passo 35, junto com `audio/04`. Fica em `_arquivo/repo/docs/audio/` com o motivo, e o passo 35 lê de lá. |
| `docs/engineering/06-installers.md` | "mesmos instaladores que delivery/04" | `delivery/04` tem 5 seções de **manifesto**; a tabela de formatos/recipes/assinatura do 06 (2 618 chars) é única | `delivery/04-gates.md` §Instaladores — **acrescentado**, não substituído |
| `docs/streaming/04-api-semantics.md` | "endpoints duplicados com architecture/04" | Os endpoints **estão** em `architecture/04 §Provider API v1`; as 6 seções de semântica (5 878 chars) são únicas | `architecture/04-contratos.md` §Provider — **acrescentado** |
| `docs/extensions/03-visualizations-integrations.md` | "subconjunto de 04-visualizer-contract" | **113 dos 162 termos são exclusivos do 03** — SMTC/MPRIS/MediaSession, Discord, VST3, remote control | `extensions/05-visualizador.md` §Integrações de sistema — **acrescentado** |

### 5.3 Os 13 de `docs/audio/` e `docs/tools/` vão para `reference/`

Nenhum é absorvido neste passo, e **nenhum vai para `_arquivo/`**. Vão para
`reference/audio/` e `reference/tools/`.

O motivo é medido, não estético: **247 links markdown vivos, em 71 arquivos**, apontam para
esses 13 documentos (`product/03-requisitos.md` 41, `product/05-decisoes.md` 27,
`CHANGELOG.md` 26, `INDEX.md` 17, `design/05-dsp-panel.md` 11). O `cargo xtask check-docs`
do `xtask/src/validate.rs:152` exige que **todo** alvo relativo exista em disco, e `_arquivo/`
é excluído do walk (`util.rs:161`). Arquivá-los deixaria 247 links apontando para caminho
inexistente e o CI Gate reprovaria no primeiro push — e a única saída seria apagar 247 links
de rastreabilidade.

`reference/` resolve os dois lados: é versionado, está em `exclude` do `Cargo.toml` (§6),
então não entra no build, e é justamente onde o passo 35/37 vai buscar a spec congelada de
cada módulo. Os links continuam válidos e nada se perde.

| Arquivo | Vira spec de | Passo |
|---|---|---|
| `audio/01, 03, 07, 08, 09, 10` | `libs/engine` (`engine`, `pipeline`, `dsp`, `equalizadores`, `catalogo-dsp`) | 35/37 |
| `audio/02, 06` | `libs/decode` + `libs/encode` | 35/37 |
| `audio/04`, `audio/05` | `libs/output` (`output`, `backends`) | 35/37 |
| `tools/01`, `tools/03` | `apps/restore` (`restore`, `ia`) | 35/37 |
| `tools/02` | `apps/converter` (`converter`) | 35/37 |
| `engineering/01-agents-superpowers.md` | absorvido por `AGENTS.md` §5 | 32 |

`reference/` deixa de ser só o pin do MVP e passa a abrigar, também, spec congelada de
módulo. O `reference/LEIA-ME.md` criado pelo passo registra isso e o passo que esvazia cada
arquivo.

### 5.4 Regra do script

O script lê o conteúdo do arquivo **antes** de movê-lo para `_arquivo/`. Se o passo 30 já
rodou e o arquivo está em `_arquivo/repo/`, o script lê de lá e avisa que a fusão foi feita
por leitura do arquivado. O que ele **nunca** faz é fundir sem que o conteúdo de origem
tenha sido copiado — arquivo fundido sem cópia é perda silenciosa.

## 6. `Cargo.toml` do umbrella

```toml
[workspace]
resolver = "3"
members = ["xtask"]
default-members = ["xtask"]

# libs/* e apps/* NAO sao membros: sao repositorios independentes em submódulos.
# Cada um tem seu proprio Cargo.toml e compila sozinho (ver AGENTS.md §1.2).
exclude = ["libs/*", "apps/*", "reference/*"]
```

**Sem `[workspace.package]` nem `[workspace.lints]`.** O `xtask/Cargo.toml` declara
`edition`, `rust-version`, `license` e as dependências por extenso, sem nenhum
`.workspace = true`. Herança de workspace que ninguém herda é configuração morta: passa
no `cargo metadata` e não faz nada. Se no futuro um membro do umbrella quisher herdar,
a seção entra **junto** com o primeiro `.workspace = true` — não antes.

Isso é o ponto de M9: se `libs/engine` fosse membro do workspace do umbrella, ele
deixaria de poder ser construído sozinho — e a independência que justifica a lib
desapareceria.

## 7. Verificação

```bash
git -C D:\Projects\vynamp status --short
git -C D:\Projects\vynamp log --oneline -3
ls D:\Projects\vynamp            # architecture product delivery design
                                 # engineering extensions library platforms
                                 # quality security streaming evidence tasks
                                 # libs apps reference xtask
cargo metadata --no-deps --format-version 1   # sem erro; so xtask
grep -rn "docs/audio\|docs/tools" D:\Projects\vynamp --include=*.md \
     --exclude-dir=_arquivo                   # 0 resultados
cargo xtask check-docs                        # 0 links quebrados
```

Resultado esperado: nenhum caminho `docs/` restante fora de `_arquivo/`; `cargo metadata`
funciona com `xtask` como único membro; `check-docs` verde.

**O subcomando é `check-docs`, não `validate`.** Não existe `cargo xtask validate` —
o CLI tem `check-docs`, `check-delivery`, `guard-branch`, `quality-gate`, `doctor`,
`install` e os grupos `repo`/`ci`/`release` (`xtask/src/main.rs:189`). `cargo xtask ci docs`
roda `check-docs` + `check-delivery` + fmt + clippy + test, e é o que o CI Gate usa.

### 7.1 Por que `check-docs` importa aqui

Medi a movimentação antes de escrevê-la: **1 940 links relativos quebram em 261 arquivos**
se o `git mv` for feito sem reescrever link. Os piores são `product/03-requisitos.md` (418),
`INDEX.md` (322), `README.md` (38) e `SPEC.md` (23). `check-docs` reprova nisso, então o
commit do passo 31 não pode ser verificado sem a reescrita — ela faz parte do passo, não é
polimento posterior.

## 8. Commit

```
refactor(estrutura): move para vynamp/ e reorganiza as specs no umbrella

- repo/ -> D:\Projects\vynamp; docs/ deixa de existir como diretorio
- git mv por arquivo; historico preservado
- 1 fusao real (QA-022..033 de quality/04) e 3 acrescimos de conteudo unico
  (installers -> delivery/04, semantica da API -> architecture/04,
  integracoes de sistema -> extensions/05)
- 1940 links relativos reescritos pelo mapa origem->destino
- Cargo.toml do umbrella vira workspace virtual; libs/ e apps/ excluidos
- README.md e INDEX.md reescritos para o umbrella
- verificado por cargo xtask ci docs
```

## 9. Riscos

| Risco | Mitigação |
|---|---|
| `git mv` de arquivo rastreado perde histórico | sempre `git mv`, nunca `Move-Item` em arquivo versionado |
| Link relativo quebra ao mover | o script reescreve os 1 940 pelo mapa; `cargo xtask check-docs` confirma 0 |
| Fusão perde conteúdo único | só funde o que foi medido como duplicata; o resto é acrescentado, não substituído (§5.2) |
| `D:\Projects\vynamp` já tem conteúdo | o script aborta se não estiver vazio |
| Worktree ativa impede mover o repo | `git worktree list` é checado antes |
| Worktree remota não empurrada seria perdida | o script exige árvore limpa **e** `git rev-list --count origin/<branch>..<branch>` = 0 |
