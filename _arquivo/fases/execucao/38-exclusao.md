# Passo 38 — exclusão definitiva

Último passo da FASE 3 e **o único que apaga do disco**. Tudo o que ele remove já
saiu do caminho nos passos 30–37: o projeto vive em `D:\Projects\vynamp`, o
histórico está no GitHub (`jimmykedlyn/vynamp` e o `vynamp-mediaplayer` arquivado)
e o `_arquivo/` do umbrella tem entrada para cada arquivo arquivado.

Ele começa perguntando `pode excluir` no console. Mesmo com a resposta certa, ele
abre **uma confirmação por grupo** (A, B, C). Nenhum grupo sai sem a palavra dele.
`Ctrl+C` em qualquer ponto é seguro: cada grupo é uma lista fechada de caminhos, e
parar entre dois `Remove-Item` deixa o disco num estado que o próximo run lê sem
erro.

---

## 1. O que existe no disco depois do passo 37

| Caminho | Tamanho | Vivo? |
|---|---|---|
| `D:\Projects\vynamp` | umbrella: `.git` herdado do repo antigo, 9 submódulos | **sim** — é o projeto |
| `…\vynamp-media-player\repo\` | 14 GB: **só** `target\` (3,1 GB), `.worktrees\` (11 GB) e `_arquivo\` vazio | não — o resto foi movido no 31; o `.git` foi junto |
| `…\vynamp-media-player\archive\` | 1 MB, 10 arquivos (histórico do fluxo antigo) | não |
| `…\vynamp-media-player\logs\` | 0 arquivos | não |
| `…\vynamp-media-player\.cache\` | 6,4 GB | **em parte** — só `.cache\cargo` é vivo (308 MB) |
| `D:\Projects\vynamp-chatgpt-arquivo\` | ~1 GB, 10.562 arquivos (M10: fora das specs) | não |
| `…\vynamp-media-player\AMBIENTE.ps1` | 3,4 KB | **sim** — único caminho de autenticação |
| `D:\Projects\_smoke37\` | 4,7 MB, teste do smoke | não |
| `D:\Projects\_vynamp-recuperacao-2026-11-06\` | 2 arquivos (patch da camada3 do decode) | **não decidido** — relatado, não tocado |

Os 10 `.md` da raiz externa e os dois do M8 não aparecem porque o passo 30 os moveu
para `_arquivo/raiz/` — e o 31 levou o `_arquivo/` junto com o resto do repo.

O `archive\` tem 10 arquivos, não 958 KB de `.md`: o peso é o
`GUIA-PROMPTS-CODEX-CLAUDE.md`, e ele foi para `_arquivo/` no passo 30 se estava
na lista do mapa — os que sobrarem aqui são o resto do fluxo antigo.

## 2. O que este passo faz

| Ordem | Grupo | O que | Confirmação |
|---|---|---|---|
| 0 | — | Lê o disco, mostra cada caminho com tamanho, exige `pode excluir` | **1** |
| A | Build e testes descartáveis | `repo\target`, `repo\.worktrees`, `_smoke37` | **2** |
| — | Bootstrap preservado | `.cache\cargo`, `import-completed.json`, `AMBIENTE.ps1`, a raiz externa inteira | **nunca** |
| B | Conteúdo não migrado | `archive\`, `logs\`, `vynamp-chatgpt-arquivo\`, `.cache\` exceto `cargo\` | **3** |
| C | Casca do repositório antigo | `repo\` vítrea (só `_arquivo\`) — **se `_arquivo\` estiver vazia** | **4** |

Cada caminho removido entra em `_arquivo/INDEX.md` do **umbrella**, com
`Destino = -` e passo `38`. O índice está versionado, então um clone novo sabe o
que desapareceu e por quê.

### Por que o bootstrap não está em nenhum grupo

`AMBIENTE.ps1` é o único caminho de autenticação do projeto:

```powershell
. D:\Projects\vynamp-media-player\AMBIENTE.ps1
Invoke-VynampAuthenticatedAgent -Action { … }
```

ele define `CARGO_HOME = .cache\cargo`, `FFMPEG = .cache\tools\...` e valida o
acesso ao `vynamp-mediaplayer`. Três scripts do `vynamp-decode`
(`tools/fixtures/generate*.ps1`) o carregam por caminho absoluto. Até existir um
`AMBIENTE.ps1` do umbrella — tarefa **TASK-CI-02**, emitida no passo 37 — ele fica
onde está. Os grupos A, B e C não o tocam, nem o `repo\` (o passo 31 não o moveu:
ele nunca esteve dentro do repo).

### Por que o grupo C quase nunca roda

O passo 31 moveu o conteúdo de `repo\` para o umbrella **com o `.git` dentro**, e o
passo 35 apontou esse mesmo história para `jimmykedlyn/vynamp` (D-GH-3). Se tudo
deu certo, `repo\` contém só `_arquivo\` vazio e `target\`/`.worktrees\`, que o
grupo A já apagou. O grupo C existe para o caso contrário: **o 31 falhou no meio**
e sobrou lá um `.git` ou um `_arquivo\` com conteúdo. Nesse caso o script
**recusa** apagar — `.git` sem remoto novo e `_arquivo\` com linhas são sinal de
que algo não foi migrado, e apagar seria perder a única cópia.

A checagem é um portão, não um aviso:

- se `repo\_arquivo\` existe e tem **algum arquivo** → grupo C recusado;
- se `repo\.git` existe e `git -C repo remote get-url origin` **não** é
  `https://github.com/jimmykedlyn/vynamp.git` → grupo C recusado.

Nos dois casos o resto do passo segue e o resumo diz o que ficou.

### `.cache\` — o que sai e o que fica

| Caminho | Tamanho | Grupo | Por quê |
|---|---|---|---|
| `tmp` | 2.010 MB | B | TEMP do bootstrap |
| `bootstrap-build` | 1.125 MB | B | `CARGO_TARGET_DIR`; recriado vazio no próximo bootstrap |
| `e15-01-verification-20261015` | 847 MB | B | evidência de verificação antiga |
| `tools` | 669 MB | B* | ffmpeg de dev. **Pulado** se `AMBIENTE.ps1` ainda o exportar |
| `task-e15-01-driver` | 632 MB | B | driver de teste antigo |
| `flac-test-files` | 477 MB | B* | corpus CC0 (IETF CELLAR). **Pulado** se `VYNAMP_FLAC_TEST_FILES` apontar para ele |
| `woodpecker` | 423 MB | B | CI morto (M6) |
| `cargo` | 308 MB | **nunca** | `CARGO_HOME` do bootstrap |
| `tests`, `doctor.txt`, `pr-task-e15-01-eac3221.md`, `wpw`, `import-completed.json` | < 2 MB | B* | resíduo; `import-completed.json` **fica** (recibo do `INICIAR.ps1`) |

`B*` = o caminho é pulado quando a variável do `AMBIENTE.ps1` o referencia. São
fixtures que o ambiente de dev usa; apagar quebra um teste que passa hoje.

## 3. O que **não** faz

- Não toca no GitHub (nenhum `gh`). O arquivamento do `vynamp-mediaplayer` é o
  passo 35 (D-GH-5).
- Não faz push, merge, tag, branch ou `git add`.
- Não apaga nada dentro de `D:\Projects\vynamp` — nem `_arquivo/`, nem submódulos.
- Não apaga `vitrola`, `vynamp-mvp-reference`, `vynamp-decode`, `vynamp-output`,
  `vynamp-fases` nem `_vynamp-recuperacao-2026-11-06`.
- Não remove `.cache\cargo` nem `import-completed.json`; se forem editados para
  dentro de um grupo, o script falha antes de qualquer `Remove-Item`.
- Não apaga caminho novo. A lista é fechada; pasta que não estiver nela é
  **relatada** e fica.

## 4. Como o script decide

```
$grupos = [ordered]@{
  A = repo\target, repo\.worktrees, _smoke37
  B = repo\archive, repo\logs, vynamp-chatgpt-arquivo, .cache\<8 itens da tabela>
  C = repo   (só se _arquivo vazia e .git ausente ou apontando para vynamp)
}
```

Três checagens antes de cada `Remove-Item` — qualquer falha **para o grupo**:

1. caminho sob `D:\Projects\vynamp-media-player\` **ou** `D:\Projects\_smoke37`
   (segunda checagem: exceção explícita, porque está fora da raiz externa);
2. o caminho pertence à lista do grupo e **não** está em `$protegidos`;
3. não é ancestor de `D:\Projects\vynamp` nem de outro repo `D:\Projects\vynamp*`.

## 5. Verificação

```powershell
Test-Path D:\Projects\vynamp-media-player\repo\target        # False
Test-Path D:\Projects\vynamp-media-player\repo\.worktrees    # False
Test-Path D:\Projects\vynamp-media-player\repo\.git          # garantido False pelo 31, ou C recusou
Test-Path D:\Projects\vynamp-media-player\archive            # False
Test-Path D:\Projects\vynamp-chatgpt-arquivo                 # False
Test-Path D:\Projects\vynamp-media-player\AMBIENTE.ps1       # True
Test-Path D:\Projects\vynamp-media-player\.cache\import-completed.json  # True
Test-Path D:\Projects\vynamp-media-player\.cache\cargo       # True
Test-Path D:\Projects\vynamp                                 # True
git -C D:\Projects\vynamp status --short                     # limpo
git -C D:\Projects\vynamp log --oneline -3                   # commit 38 no topo
```

## 6. Commit

Não há commit aqui: o índice muda depois que o disco mudou, e commitar disso não
desfaz nada — pior, o commit entraria no histórico antes de você conferir a
verificação acima. O script deixa `_arquivo/INDEX.md` modificado no umbrella; o
mantenedor revisa e commita:

```
chore(limpeza): exclusao definitiva da raiz externa (passo 38)

Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
```

O commit é local, no umbrella, sem push.

## 7. Riscos

| Risco | Mitigação |
|---|---|
| Apagar `_arquivo\` do umbrella | o grupo C só apaga o `_arquivo\` do repo antigo, e só se vazio; o do umbrella fica |
| Apagar algo que um script do decode ainda usa | `AMBIENTE.ps1`, `.cache\cargo`, `.cache\tools`, `flac-test-files` protegidos ou pulados pela variável |
| Perder trabalho não commitado nas worktrees | o passo 31 **já bloqueia** a migração se houver worktree ativa; se chegou aqui, elas foram removidas com os PRs mergeados |
| Excluir fora da raiz externa | checagem 1 e 3; `_smoke37` é a única exceção e está escrita no código |
| Sobra de tentativa anterior no umbrella | o grupo C recusa quando há conteúdo não migrado; a decisão fica com o mantenedor |
