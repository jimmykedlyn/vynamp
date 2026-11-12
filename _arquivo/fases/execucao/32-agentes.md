# Passo 32 — agentes

Consolida as **duas autoridades de agente** (C10) em uma só, aplicando M5.

---

## 1. O problema que este passo resolve

Hoje existem dois contratos de agente com regras que se contradizem:

| Arquivo | Tamanho | Problema |
|---|---|---|
| `D:\Projects\vynamp-media-player\AGENTS.md` (raiz, fora do git) | 2,2 K | manda ler `STATE.md` e `ROADMAP-rodadas.md` (este último arquivado) |
| `repo/AGENTS.md` (versionado) | 4,1 K | manda ler `START-HERE.md` e `SPEC.md` (o primeiro arquivado) |

Os dois divergem sobre CI Gate e proteção de branch. Um agente que lê os dois recebe
instruções incompatíveis; um que lê só um perde metade. Nenhum dos dois descreve a
arquitetura nova (M3), a regra de isolamento, `vynamp-dsp` (M9) nem o fluxo SDD real.

## 2. O que este passo faz

| # | Ação |
|---|---|
| 1 | Escreve `AGENTS.md` na raiz do umbrella, com as 6 seções do contrato único |
| 2 | Escreve `CLAUDE.md` de 4 linhas: título + `@AGENTS.md` |
| 3 | Move `repo/AGENTS.md` e `repo/CLAUDE.md` para `_arquivo/repo/` com motivo |
| 4 | Move o `AGENTS.md` da raiz externa para `_arquivo/raiz/` |
| 5 | Escreve o `AGENTS.md` de uma linha em cada repo de módulo (no passo 35, quando os repos existirem) |
| 6 | ~~Atualiza `docs/engineering/01-agents-superpowers.md`~~ — **não existe mais**: o passo 31 arquivou `docs/engineering/` inteiro; este item vira no-op verificado |

## 3. Conteúdo do `AGENTS.md` único

Vem de `D:\Projects\vynamp-fases\agentes\AGENTS.md`, copiado literalmente. Seções:

| § | Seção | O que fixa |
|---|---|---|
| 1 | **Arquitetura** | 10 repos, direção de dependência, primitivos atravessam, `vynamp-dsp` é primitivas, `reference/` read-only, decisão estrutural vira ADR |
| 2 | **DDD** | contexto por módulo, nenhum tipo de domínio atravessa repo, uma trait não é contrato universal, `vynamp-core` não existe |
| 3 | **Clean code** | fmt/clippy/machete como portão, erro é dado, `unwrap` justificado, `unsafe` com `// SAFETY:`, docs em `///` |
| 4 | **SDD** | spec → plan → tasks → código → review; nenhuma TASK sem Given/When/Then; TDD; evidência ou não aconteceu |
| 5 | **Fluxo** | Opus orquestra, Sonnet implementa, Haiku checa, review em contexto limpo; gitflow; economia de tokens |
| 6 | **Proibições** | force push, merge pelo agente, alterar visibilidade, bypass de hook, imprimir token, apagar direto, PR com teste falhando |

## 4. O que **não** muda

- O agente continua sem poder fazer merge, tag ou release.
- Visibilidade, proteção de branch e merge continuam sendo do mantenedor.
- `.claude/settings.local.json` não é versionado e não vira parte do contrato.

## 5. Verificação

```bash
ls D:\Projects\vynamp\AGENTS.md D:\Projects\vynamp\CLAUDE.md
wc -l D:\Projects\vynamp\CLAUDE.md     # 5 linhas
grep -c "^## " D:\Projects\vynamp\AGENTS.md   # 6 seções
ls D:\Projects\vynamp\repo\AGENTS.md   # nao existe mais
```

Resultado esperado: um único `AGENTS.md` com 6 seções; `CLAUDE.md` só importa.

## 6. Commit

```
docs(agentes): consolida os dois AGENTS.md em um contrato unico (M5)

- AGENTS.md do umbrella: Arquitetura, DDD, Clean code, SDD, Fluxo, Proibicoes
- CLAUDE.md reduzido a @AGENTS.md
- AGENTS.md e CLAUDE.md do repo e o AGENTS.md da raiz externa -> _arquivo/
- fixa M3 (isolamento), M9 (vynamp-dsp) e a regra de primitivos
```

## 7. Risco

| Risco | Mitigação |
|---|---|
| Perder uma regra que só existia no `AGENTS.md` antigo | o passo 32 compara os dois arquivos e imprime as linhas que **não** aparecem no novo, para revisão antes de arquivar |
| `CLAUDE.md` com `@AGENTS.md` não resolver | é a sintaxe de import do Claude Code; verificar abrindo uma sessão no umbrella depois |
