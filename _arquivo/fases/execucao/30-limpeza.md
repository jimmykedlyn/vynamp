# Passo 30 — limpeza

**Primeiro passo da FASE 3.** Aplica M4 e M6 no texto de que todo o resto depende,
cria `_arquivo/` e move para lá tudo que sai — **nada é apagado**.

Roda em `D:\Projects\vynamp-media-player\repo` (o git existente), antes de mover
qualquer coisa para `D:\Projects\vynamp`.

---

## 1. O que este passo faz

| # | Ação | Origem |
|---|---|---|
| 1 | Cria `_arquivo/INDEX.md` e a estrutura de destino | regra do mantenedor |
| 2 | Move para `_arquivo/repo/` os 83 arquivos "arquivar" do mapa | `00-mapa-destino.csv` |
| 3 | Aplica M4: 12 VUs → 16 VUs em 11 pontos | decisão M4 |
| 4 | Aplica M6: remove 100% Woodpecker de 72 arquivos (exceto `CHANGELOG.md`) | decisão M6 |
| 5 | Remove a regra com o JWT de `.claude/settings.local.json` | achado da FASE 1 |
| 6 | Corrige `.gitignore` (`*.log`, `VALIDATION.json`, `.cache/`) | achado 9 |
| 7 | Corrige a âncora quebrada em `tools/02:48` | órfão da FASE 1 |
| 8 | Registra o SHA `29e14e6` como último estado herdado | M11 |

## 2. O que este passo **não** faz

- Não apaga nada. `Remove-Item` só em `.cache/` e `target/` — caches de build, não
  arquivos de projeto.
- Não faz `git push`, `merge`, `tag` ou `gh repo edit`.
- Não muda visibilidade de repo nenhum.
- Não move arquivos para `D:\Projects\vynamp` — isso é o passo 31.
- Não toca `AUDIOSR-RUST-CPU.md` nem `SOURCE_SEPARATION_MODELS.md`: eles só saem
  **depois** do merge de `chore/claude-specs-ia-futura` (M8). Se o merge ainda não
  aconteceu, o script avisa e pula.

## 3. Pré-requisitos

- [ ] `repo/` em `develop` @ `29e14e6`, árvore limpa (`git status --short` vazio)
- [ ] Os 5 PRs mergeados (lista D2) — senão M8 não pode ser executado
- [ ] Nenhum worktree ativo sobre os arquivos movidos

## 4. Verificação ao fim

```bash
git -C repo status --short          # deve listar só movimentos e edições deste passo
grep -ri woodpecker repo/docs       # 0 resultados fora de CHANGELOG.md
grep -rn "12 VUs" repo/docs         # 0 resultados
ls _arquivo/INDEX.md                # existe e tem uma linha por arquivo movido
git -C repo diff --stat             # revisar antes de commitar
```

Resultado esperado: `grep -ri woodpecker` retorna só `CHANGELOG.md`; `12 VUs` retorna
zero; o `diff --stat` mostra os 83 arquivos como renomeações/movimentos, não como
deleções.

## 5. Commit

```
chore(specs): aplica M4 (16 VUs) e M6 (sem Woodpecker) e cria _arquivo

- M4: 12 -> 16 VUs em 11 pontos; VU-INT-06 = 12 skins x 16 VUs = 192
- M6: remove Woodpecker de 72 arquivos (CHANGELOG preservado)
- 83 arquivos movidos para _arquivo/repo/, com motivo em _arquivo/INDEX.md
- jogos de permissao: regra com JWT Woodpecker removida
- .gitignore: *.log, VALIDATION.json, .cache/
- SHA 29e14e6 registrado como ultimo estado herdado do vynamp-mediaplayer
```

## 6. Riscos

| Risco | Mitigação |
|---|---|
| O regex de M4 troca "12" onde não deve (ex.: 12 kHz) | o script casa só as formas `12 VUs`, `12 skins × 12 VUs`, `= 144`; imprime cada alteração para revisão |
| O regex de M6 estraga `CHANGELOG.md` | arquivo excluído por caminho explícito |
| Mover arquivo versionado perde histórico | usa `git mv`, não `Move-Item` |
| `.claude/settings.local.json` é recriado pelo app | a regra pode voltar; o token deve ser **revogado**, não só removido |

## 7. Depois deste passo

Rodar o 31 (`31-estrutura.ps1`), que move o repo para `D:\Projects\vynamp` e cria
a árvore de submódulos. **Parar e confirmar** antes de qualquer `git push`.
