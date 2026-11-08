# Vynamp — regras do agente (Claude, dono único)

Claude é autor E revisor de todo o projeto. Estado vivo: `STATE.md` (ler primeiro, manter ≤ 2 KB). Ordem das TASKs: `ROADMAP-rodadas.md`. Spec de cada TASK: `repo/tasks/items/TASK-*.md`. Histórico antigo: `archive/` (não ler salvo necessidade pontual — é grande).

## Segurança (inegociável)
- Visibilidade é decisão do mantenedor, decidida na ADR-014 e executada pela TASK-E19-06 — o agente não altera visibilidade, proteções, assinatura nem plano por conta própria.
- Sem merge, tag, release, force push ou reescrita de histórico. Merge é do mantenedor (merge commit pelo GitHub).
- Sem bypass de hooks. CI ausente/pendente/falho ≠ pronto para merge. CI Gate vem do GitHub Actions (ADR-014).
- Não tratar todo 403 como limite de plano; diagnosticar autenticação.
- Nunca imprimir tokens/credenciais. GitHub via `. AMBIENTE.ps1` + `Invoke-VynampAuthenticatedAgent -Action { ... }`.

## Fluxo por TASK
1. Opus (orquestrador) lê a spec da TASK e escreve plano curto.
2. Worktree própria `repo/.worktrees/claude/task-<id>-<slug>` via `cargo xtask repo worktree`.
3. Implementação por subagente **Sonnet** (TDD). Alto risco (áudio/tempo real E01/E02, concorrência, segurança, release) → **Opus**.
4. Checks por subagente **Haiku**, que devolve só resumo passou/falhou.
5. Revisão por subagente de contexto limpo, nunca o implementador: Sonnet; alto risco → Opus.
6. Publicar branch, criar/atualizar PR (template, `--body-file`, sem duplicar), conferir CI Gate do SHA final.
7. Atualizar `STATE.md` e entregar ao usuário: URL do PR, SHA revisado, base, status revisão/CI e "aprove e faça o merge commit pelo GitHub".

Até 2–3 TASKs independentes em paralelo; serializar as que tocam `Cargo.toml`/`Cargo.lock`/índices de docs.

## Após "merge feito"
Verificar PR via gh (MERGED, base, mergeCommit), `git fetch origin`, fast-forward do develop no checkout central limpo, confirmar HEAD=origin/develop, remover worktree/branch da TASK integrada, atualizar `STATE.md` e começar a próxima TASK.

## Economia de tokens
Não registrar painéis/handoffs longos. Evidência vai no commit (`docs/evidence/PR/<task>/`) e no corpo do PR; `STATE.md` só aponta. Subagentes retornam resumos curtos, não dumps.
