# Evidência E15-03 / QA-012 — GitHub privado e Gitflow

Versão 0.7.0 · 2026-10-18 · Autor Claude · Branch `chore/task-e15-03-claude-gitflow` · Base `edfb1ade703f9cee2029d88bd5e26f39d12902a7`. Resultados do autor; a revisão do Codex é registrada à parte. Capturas feitas antes do commit (HEAD = base) não contêm o SHA final.

[Plano](../../../superpowers/plans/2026-10-18-e15-03-gitflow.md) · [Spec Gitflow](../../../engineering/03-gitflow-github.md) · [TASK](../../../../tasks/items/TASK-E15-03.md)

## Aceite

| Critério | Resultado | Fonte |
|---|---|---|
| Privacidade real confirmada | Passou: `private=true`, `visibility=private` pela API, antes e depois | [audit-before.json](audit-before.json), [audit-after.json](audit-after.json) |
| Histórico preservado | Passou: import da E15-01 reutilizado, sem reimportação; `origin/main` (247b424, raiz) ancestral de `origin/develop` (edfb1ad), 11 commits | [history.txt](history.txt) |
| Sem force push | Passou: API de atividade `activity_type=force_push` retorna 0 eventos; nenhum push forçado nesta TASK | [history.txt](history.txt) |
| Default develop e merge commit | Aplicado: `default_branch=develop`, squash/rebase desativados, exclusão de branch após merge | [protect.txt](protect.txt), [audit-after.json](audit-after.json) |
| CI Gate/proteções | **Pendência registrada, não aplicado:** branch protection e rulesets retornam HTTP 403 (“Upgrade to GitHub Pro or make this repository public”) | [audit-after.txt](audit-after.txt), [history.txt](history.txt) |

## Comandos e resultados

Ambiente em [environment.txt](environment.txt): Windows 11 build 26300, PowerShell 7.4.20, Git 2.47.1.windows.2, gh 2.102.0, Rust/Cargo 1.99.0, `AMBIENTE.ps1`, target próprio `target/xtask-driver`.

1. Baseline antes de editar: `cargo xtask doctor --agent both` exit 0 (Superpowers confirmado nos dois gerenciadores); `cargo xtask ci docs` exit 0.
2. RED: testes `repo::github_policy_tests` não compilavam (`plan_blocks_protection`/`audit_report` inexistentes). GREEN: 5 testes passaram após a implementação.
3. `cargo xtask repo audit --repo jimmykedlyn/vynamp-mediaplayer` antes da mudança: exit 1, quatro pendências (default `main`, squash/rebase permitidos, proteção de main e develop `unavailable_plan`).
4. `cargo xtask repo protect --repo jimmykedlyn/vynamp-mediaplayer --reviews 0`: privacidade confirmada antes do PATCH; PATCH das configurações exit 0; PUT da proteção HTTP 403; comando exit 1 com mensagem `PENDING` explícita. `--reviews 0` porque há uma única conta mantenedora e admins ficam sujeitos às regras.
5. Auditoria depois: exit 1, restam apenas as duas pendências de proteção.
6. `cargo xtask ci docs` final exit 0: 34 unitários (29 existentes + 5 novos) e 39 de integração, docs/config/fmt/clippy passaram ([ci-docs-final.txt](ci-docs-final.txt); baseline com 29+39 em [ci-docs-baseline.txt](ci-docs-baseline.txt)); `doctor` repetido em [doctor.txt](doctor.txt).

## Mudanças no tooling

- `repo protect` confirma `private=true` antes de alterar o repositório e transforma o 403 de plano em erro `PENDING`; não imprime mais “Protection applied” sem sucesso real.
- `repo audit` (somente leitura) relata privacidade, default, métodos de merge e proteção por branch (`applied`/`incomplete`/`unavailable_plan`/`absent`); falha enquanto houver pendência.
- Dependabot passa a mirar `develop`.

## Observações e limitações

- Antes da troca de default, os sete PRs Dependabot abertos (#1–#7) miravam `main`; depois do PATCH a listagem mostra `base=develop`. A mudança de base não foi feita pelo Claude; nenhum desses PRs foi fechado ou integrado.
- Secret scanning e push protection aparecem `disabled` na resposta da API; não fazem parte desta TASK.
- Environments de release (`release-signing`, `release-publish`, `hardware-validation`) não existem (0) e continuam fora do escopo.
- **Pendência do mantenedor:** habilitar proteção exige plano que suporte proteção em repositório privado; tornar público não é aceito. Depois, reexecutar `repo protect --reviews 0` e `repo audit` até exit 0. Até lá, main/develop aceitam push direto pelo GitHub; os hooks locais e o merge manual com CI Gate verde são a única barreira.

## Correções após revisão Codex de 2783f6f (2026-10-18)

Revisão Codex de `2783f6f99bb4376d91298eb8bc0512e7a420ed54` solicitou correções; resultados abaixo são do autor, ambiente igual ao acima.

- **F1/P2:** `repo audit` só considerava `applied` CI Gate strict, admins e bloqueio de force push/deletion. Agora exige também `required_pull_request_reviews` presente com `dismiss_stale_reviews=true` e `required_conversation_resolution.enabled=true`; campos ausentes falham fechado (`incomplete`). `required_approving_review_count=0` (conta única) continua aceito, mas o PR segue obrigatório.
- **F2/P3:** `delete_branch_on_merge` diferente de `true` (inclusive ausente) vira pendência.
- RED: `cargo test -p xtask --bin xtask github_policy_tests` exit 101, 6 passaram e 4 falharam (PR ausente/nulo, stale review, conversas, delete_branch) ([tdd-fix-red.txt](tdd-fix-red.txt)). GREEN: exit 0, 10/10 ([tdd-fix-green.txt](tdd-fix-green.txt)). O teste de review_count 0 já passava e protege o modo solo.
- `repo audit` real, somente leitura, depois da correção: exit 1 com as mesmas duas pendências de plano (main/develop `unavailable_plan`); privacidade, default, merge commit e `delete_branch_on_merge=true` conferem ([audit-fix.txt](audit-fix.txt), [audit-fix.json](audit-fix.json)). `repo protect` não foi reexecutado.
- `git diff --check`: capturas `ci-docs-baseline.txt`, `ci-docs-final.txt` (linha vazia extra no EOF) e `history.txt` (espaço final) normalizadas só em espaços; conteúdo factual inalterado.
- `origin/develop` (merge do PR11, `98a1a73`) incorporado por merge na branch, sem rebase/force; `VALIDATION.json` regenerado por `cargo xtask ci docs`.
