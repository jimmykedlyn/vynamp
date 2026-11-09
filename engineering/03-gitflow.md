# Gitflow e envio para o GitHub público

Versão 0.9.0 · 2026-11-08 · Configuração do projeto; player ainda não implementado.

Repositório: jimmykedlyn/vynamp. Estado remoto verificado na TASK-E19-06 (2026-11-08): default `develop`, somente merge commit, squash e rebase desativados, exclusão de branch após merge, dez repositórios públicos com `main` e `develop` protegidas exigindo o contexto `CI Gate` em modo strict, conversas resolvidas, PR obrigatório com dismiss de revisão obsoleta, administradores sujeitos às regras, e force push e exclusão proibidos. A [ADR-014](../adr/ADR-014-publico-dsp-interno.md) decidiu a abertura; a TASK-E19-06 a executou, junto com o relato privado de vulnerabilidade habilitado em cada repositório. A prova é `cargo xtask repo audit --repo <owner/name>`, que lê a API e falha enquanto houver pendência — o `403` de branch protection era limite do plano privado e deixou de existir em repositório público.

| Branch | Base / destino | Regra |
|---|---|---|
| main | histórico estável | Somente PR release/hotfix, tag SemVer depois CI/gates |
| develop | integração | Feature/fix/chore entram por PR e merge commit |
| feature/fix/chore/task-agent-slug | develop → develop | Um owner, testes, plano, revisão técnica pelo outro agente |
| release/x.y.z | develop → main; depois develop | Freeze, version/changelog/evidence, correções limitadas |
| hotfix/task-agent-slug | main → main; depois develop | Corrigir produção, preservando ambas linhas de histórico |

Merge commit é obrigatório; squash/rebase merge desativados para preservar ancestry e cleanup confiável das worktrees. Commits Conventional Commits (`feat`, `fix`, `test`, `docs`, `chore`, `build`, `ci`). Nenhum force push em main/develop; feature compartilhada não é reescrita sem coordenação. Não commitar secrets/instaladores/binários pesados.

Publicar feature: `git push -u origin <branch>`, `gh pr create --base develop --body-file <arquivo>`; comparar diff/task/CI antes merge. Aprovação técnica do Claude não equivale a aprovação por outra identidade GitHub. `--reviews 1` exige pessoa/conta autorizada diferente do autor; solo pode usar 0 e manter revisão/evidência técnica, com merge pelo mantenedor.

Import inicial preserva histórico via cargo xtask github-import. O importador automatiza o bootstrap quando o repositório está vazio. Alternativa manual: clonar para pasta nova, copiar todos arquivos do pacote exceto .git/.worktrees/.superpowers/target/dist; `git symbolic-ref HEAD refs/heads/main`, `git add --all`, `git commit -m "chore: bootstrap Vynamp"`, `git branch develop`, `git push -u origin main`, `git push -u origin develop`. Esse primeiro commit é exceção de bootstrap, não licença para desenvolver diretamente em main.

Depois, `cargo xtask repo protect` configura GitHub (confirma que o repositório é público antes de alterar qualquer coisa): CI Gate obrigatório/strict, dismiss stale reviews, conversações resolvidas, sem force push/deletion, admins sujeitos às regras; default branch develop. Enquanto o repositório não estiver público, o comando falha com `PENDING` em vez de alterar o remoto, e `cargo xtask repo audit` relata o estado real, falhando enquanto houver pendência: só considera proteção aplicada com CI Gate strict, PR obrigatório com dismiss stale reviews, conversas resolvidas, admins e bloqueio de force push/deletion (dado ausente conta como pendência), e também cobra `delete_branch_on_merge=true`. Com uma única conta, `--reviews 1` mais admins sujeitos às regras impediria o próprio mantenedor de integrar; por isso os dez repositórios foram protegidos com `--reviews 0` (TASK-E19-06, 2026-11-08), e é esse o valor em `config/github.json`. O status `CI Gate` vem do job `gate` do GitHub Actions ([ADR-014](../adr/ADR-014-publico-dsp-interno.md)); secrets de signing/publicação ficam nos environments `release-signing`/`release-publish` quando o release for migrado.

Tags `vX.Y.Z` e `vX.Y.Z-beta.N` apontam a commit aprovado de main. O workflow `release.yml` roda o preflight e, enquanto build/publicação não forem migrados, termina em falha fechada; depois da migração cria draft privado, assinatura/verificação/manifest/SBOM e evidência revisados antes publish pelo mantenedor. A plataforma web futura tem deploy separado M4, não é acionada ao subir spec/player.

[Branch protection](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches).
