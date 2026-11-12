# Passo 35 — GitHub

Cria os 7 repositórios novos, adiciona os 9 submódulos ao umbrella e — **por último e
com confirmação própria** — arquiva o `vynamp-mediaplayer`.

Este é o primeiro passo da FASE 3 que toca a rede. Tudo o que ele faz é criação e
leitura. Ele **não** mexe em visibilidade, proteção de branch, assinatura nem plano.

`-SemArquivar` pula a seção 9 (o arquivamento do `vynamp-mediaplayer`), que é a única
mudança de estado no GitHub deste passo e pode ser adiada.

---

## 1. O que existe hoje

| Repo | Existe | Estado |
|---|---|---|
| `vynamp-mediaplayer` | sim | privado, `develop` @ `29e14e6` — vira o conteúdo do umbrella |
| `vynamp-decode` | sim | privado, `feature/mpeg` @ `67bfac5`, árvore limpa (atualizado em 2026-11-07; o plano dizia `9fc1e4b` com 5 arquivos não commitados) |
| `vynamp-output` | sim | privado, `feature/core` @ `d0497b5` |
| `vynamp-mvp-reference` | sim | privado, `main` @ `741316f` |
| `vitrola` | sim | privado, 2 commits, 0 `.rs` — **não entra** (M10) |
| `vynamp` | **não** | a criar |
| `vynamp-dsp` | **não** | a criar |
| `vynamp-encode` | **não** | a criar |
| `vynamp-engine` | **não** | a criar |
| `vynamp-player` | **não** | a criar |
| `vynamp-converter` | **não** | a criar |
| `vynamp-restore` | **não** | a criar |

7 a criar, 3 mantidos, 1 a arquivar.

## 2. Decisões deste passo

| # | Decisão | Por quê |
|---|---|---|
| **D-GH-1** | Os 7 repos novos nascem **públicos**. | ~~Privados~~. Corrigido em 2026-11-08 pelo mantenedor: a ADR-014, que é a spec normativa, manda os repositórios nascerem públicos e já havia decidido isso em 2026-11-05 — o "privados" desta linha contradizia a ADR. `xtask/src/repo.rs` sempre exigiu `private == false`; era `config/github.json` que estava fora da regra. Ver D-GH-13. |
| **D-GH-2** | `vynamp` (umbrella) nasce com **histórico vazio**, e o conteúdo vem do `vynamp-mediaplayer`. | Decisão M11 do mantenedor. Os 109 commits e os PRs #1–#39 ficam no repo arquivado, legíveis. |
| **D-GH-3** | O primeiro push do umbrella é feito **do clone antigo**, não de um repo novo. | O `vynamp-mediaplayer` já tem o histórico e os specs. Criar o `vynamp` e copiar `.git` daria o mesmo resultado com mais passos; trocar o remote é o caminho mais curto e preserva `git mv` já feito nos passos 30/31. |
| **D-GH-4** | Submódulos entram apontando para **`develop`** primeiro, e só passam a **tag** depois do primeiro commit verde. | Um submódulo apontando para tag que não existe quebra `git clone` do umbrella. A ordem tag-first é circular: não há tag sem commit, não há commit sem repo. |
| **D-GH-5** | `vynamp-mediaplayer` é arquivado **no fim**, depois de tudo verde. | Repo arquivado é read-only. Arquivar antes deixa o projeto sem remote legível, e `gh repo unarchive` é a única volta. |
| **D-GH-6** | `reference/vynamp-mvp` é o **único** submódulo que aponta para um repo com branch `main`. | Os 8 repos de código têm `develop`; a referência tem `main` e não recebe tag (é pin por commit). |

| **D-GH-7** | Os 7 repos novos nascem **com um commit** (`develop`), não vazios. | `git submodule add` **falha** contra repositório sem nenhum commit: o clone do submódulo vem sem HEAD e o comando aborta com `unable to checkout submodule`. Medido, não suposto. |
| **D-GH-8** | Cada repo novo de módulo nasce com `Cargo.toml`, `Cargo.lock`, `src/lib.rs` (ou `src/main.rs`) e o workflow de CI do `padroes/`. | O CI usa `--locked`, e `cargo --locked` **falha sem `Cargo.lock`**: `cannot create the lock file ... because --locked was passed`. Repo sem lockfile tem o CI vermelho no primeiro run. |
| **D-GH-9** | `vynamp-decode` e `vynamp-output` **não** são repositórios vazios: entram pelo conteúdo que já têm. | O `develop` do decode tem 189 arquivos e **3 workflows do Woodpecker**; o `develop` do output tem 2 arquivos e **nenhum `Cargo.toml`** — o código real está em `feature/core`/`feature/mpeg`, que são branches de trabalho, não `develop`. |
| **D-GH-11** | Os **templates de CI vão só para os 7 repos novos**. `vynamp-decode` e `vynamp-output` não recebem `ci.yml` neste passo. | Instalar `ci-lib.yml` no `develop` do output — que não tem `Cargo.toml` — deixa o CI vermelho no primeiro run. A troca é a TASK-CI-01 (passo 37), um PR por repo, depois que `feature/mpeg` e `feature/core` estiverem em `develop`. |
| **D-GH-12** | O `Cargo.toml` semeado usa `edition = "2024"`, `rust-version = "1.99"`, `resolver = "3"`. | É o que `vynamp-decode`, `vynamp-output` e `xtask` já usam, e o que o `rust-toolchain.toml` (`1.99.0`) exige. Semear com `2021`/`1.83` faria o repo novo divergir do resto do projeto desde o primeiro commit. |
| **D-GH-10** | O `reference/vynamp-mvp` **entra** no `.gitmodules`, pinado por **commit** (`741316f`), não por tag. | "Fora do workspace" (em `01-arvore-final.md`) significa **fora do workspace do Cargo** — não é membro do build. Ele ainda é entrada do `.gitmodules`, e é o único com pin por commit: não tem branch `develop` e não recebe tag. |
| **D-GH-13** | A visibilidade é **pública** durante todo o passo, e a exigência "conteúdo antes da visibilidade" da ADR-014 fica como débito explícito da **TASK-E19-04**. | Os 6 repos de módulo semeados no passo 35 ficaram públicos sem `LICENSE`, `SECURITY.md` e `CONTRIBUTING.md`, que a ADR-014 §2 exige *antes* da abertura. A ordem real foi a inversa. Fechar isso aqui exigiria criar os 6 repos outra vez; a TASK-E19-04 é justamente a que dota cada repo novo desse conteúdo, e ainda está `planned`. Público→privado é irreversível na prática, então voltar atrás não é opção. |

### Sobre D-GH-2 e D-GH-3 juntos

O `vynamp` novo no GitHub nasce vazio. O conteúdo vem do `vynamp-mediaplayer`:
`git remote set-url origin https://github.com/jimmykedlyn/vynamp.git` e push. O
histórico **é** o do `vynamp-mediaplayer` — a diferença entre "repo novo" e "repo
renomeado" é que o GitHub trata o repo novo como sem issues/PRs antigos e sem
estrelas; o grafo de commits é o mesmo.

Isso é diferente de "umbrella com histórico vazio" do M11, e é uma correção que o
próprio plano exige: o plano dizia que o umbrella nasceria com histórico vazio, mas o
passo 31 **move** o repo inteiro (com `.git`) para `D:\Projects\vynamp`. Descartar
esse histórico jogaria fora 109 commits de decisões que as specs referenciam.

**Ruling (ledger da FASE 2):** o umbrella herda o histórico do `vynamp-mediaplayer` via
troca de remote, e não nasce vazio. *Custo se errado:* os PRs #1–#39 não aparecem no
repo novo, e as citações a PR nas specs precisam do repo explícito — que é o que o M11
já previa.

## 3. Ordem de execução

| # | Ação | Deixa o quê |
|---|---|---|
| 1 | `gh repo create` dos 7 repos, privados, com `--description` e sem `--push` | 7 repos vazios no GitHub |
| 2 | Semear os 7 repos com o conteúdo mínimo (D-GH-7, D-GH-8) e dar push | 7 repos com `develop` e `Cargo.lock` |
| 3 | `git remote set-url` do umbrella para `jimmykedlyn/vynamp` | origin trocado |
| 4 | Push da `develop` do umbrella | repo novo com o histórico |
| 5 | Criar `main` a partir de `develop` nos repos de código que ainda não têm | branch `main` existe |
| 6 | `git submodule add` dos 9 | `.gitmodules` com 9 entradas |
| 7 | Push dos submódulos e do umbrella | tudo remoto |
| 8 | Criar as tags `v0.1.0` nos 8 repos de código | tag de release existe |
| 9 | Apontar os submódulos para as tags | pin por tag |
| 10 | **Arquivar `vynamp-mediaplayer`** (confirmação no console) | repo archived |

O passo 2 vem **antes** do 6 por uma razão medida: `git submodule add` contra um
repositório sem nenhum commit falha com `fatal: unable to checkout submodule`. A
ordem "cria tudo vazio → adiciona submódulos" não executa.

O passo 8 é o que transforma "submódulo em branch" em "submódulo em tag", que é a
regra do umbrella. Um tag por repo, mesmo antes de qualquer código: `v0.1.0` marca
"estrutura criada, sem produto".

O passo 5 é `main` a partir de `develop`, e não o contrário: nos 3 repos mantidos
(`vynamp-decode`, `vynamp-output`, `vynamp-mvp-reference`) `main` já existe, e
`develop` já é ancestral de `main` no decode.

## 4. O que este passo **não** faz

- Não abre nenhum repo para público. `gh repo create --visibility private`.
- Não configura proteção de branch (é do mantenedor — `04-repos-e-remotes.md` §7).
- Não cria release, não tagga versão de produto, não assina nada.
- Não faz push de `main`, não faz merge, não cria PR.
- Não mexe em `vitrola`, em `vynamp-chatgpt-arquivo` nem no `vynamp-mvp-reference`
  além de ler o remote.

## 5. Regras do script

1. **Autenticação por `AMBIENTE.ps1`, de fora.** O script **verifica** que
   `D:\Projects\vynamp-media-player\AMBIENTE.ps1` existe, mas não o carrega: um
   dot-source dele redefiniria `TEMP`, `TMP` e `CARGO_HOME` do processo, e este
   passo só precisa de `gh`. Quem o carrega é você:

   ```bash
   pwsh -NoProfile -Command ". D:\Projects\vynamp-media-player\AMBIENTE.ps1; Invoke-VynampAuthenticatedAgent -Action { pwsh -NoProfile -File ./35-github.ps1 }"
   ```

   Nenhum token é lido, impresso ou gravado em arquivo.
2. **`gh` nunca recebe token por argumento.** `GH_TOKEN` vem do ambiente, dentro do
   bloco autenticado. A pré-checagem roda `gh auth status` **antes** de criar
   qualquer repo: sem credencial, o passo para com a receita acima, em vez de
   criar 4 repos e falhar depois.
3. **Falha de 403 não é tratada como "plano"** — o script pergunta se o repositório
   existe e se a credencial alcança a org, e diz qual das duas coisas foi.
4. **Idempotente.** Repo já existente é detectado e pulado; submódulo já adicionado
   é pulado; tag já existente é pulada.
5. **Arquivar exige digitação.** O passo 8 pergunta `pode arquivar` e só prossegue com
   a resposta literal. `git push`, `merge` e `tag` de produto não existem no script.

## 6. Verificação

```bash
gh repo list jimmykedlyn --limit 30 | grep vynamp
# esperado: 11 repos vynamp* (10 da estrutura + vynamp-mediaplayer arquivado)

cd D:\Projects\vynamp
git submodule status
# esperado: 9 linhas — 8 com (v0.1.0) e 1 com (741316f); nenhuma com prefixo '-'

git ls-remote --tags origin | grep v0.1.0
# esperado: 8 tags (a referência não recebe tag)
```

Resultado esperado: `git submodule status` mostra 9 entradas, 8 com ` (v0.1.0)` e a
referência com ` (741316f)`, nenhuma com prefixo `-`. Se alguma aparecer com `-`, o
submódulo está em commit que o remote não tem — o script avisa em vez de seguir.

**Ressalva honesta sobre o `vynamp-output`:** o `develop` dele não tem `Cargo.toml`,
então o `v0.1.0` só é tag de verdade depois que `feature/core` entrar em `develop`
(D2, item 2). Se isso ainda não tiver acontecido quando o passo 35 rodar, o script
marca o output como **sem tag** e o submódulo dele fica em `develop` — com o aviso
explícito, não com um pin fingido.

## 7. Commit

```
chore(repos): cria os 7 repos e liga os 9 submodulos (passo 35)

- vynamp, vynamp-dsp, vynamp-encode, vynamp-engine, vynamp-player,
  vynamp-converter, vynamp-restore criados como publicos (D-GH-1, D-GH-13)
- origin do umbrella passa a apontar para jimmykedlyn/vynamp (D-GH-3)
- 9 submodulos adicionados; 8 repos de codigo com tag v0.1.0 (D-GH-4)
- vynamp-mediaplayer arquivado por ultimo, sob confirmacao no console (D-GH-5)
- nenhuma mudanca de visibilidade, protecao ou assinatura
```

## 8. Risco

| Risco | Mitigação |
|---|---|
| Arquivar antes da hora e o projeto ficar sem remote legível | o arquivamento é o último passo e exige `pode arquivar` digitado |
| Submódulo em commit inexistente no remote | `git submodule status` com `-`; o script avisa e não prossegue |
| Token impresso em log | o script nunca lê nem escreve token; `gh` usa `GH_TOKEN` do ambiente |
| 403 tratado como limite de plano | o script distingue "repo não existe" de "sem permissão" e diz qual |
| Repo criado e abandoned por erro no meio | os 7 são vazios e privados; `gh repo delete` reverte |
