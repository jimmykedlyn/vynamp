# Gitflow e envio para o GitHub privado

Versão 0.7.0 · 2026-10-14 · Configuração do projeto; player ainda não implementado.

Repositório escolhido: jimmykedlyn/vynamp-mediaplayer, privado segundo usuário. Acesso não verificado neste ambiente; scripts confirmam a privacidade pela API antes import. Nenhum push ou mudança remota executado nesta preparação.

| Branch | Base / destino | Regra |
|---|---|---|
| main | histórico estável | Somente PR release/hotfix, tag SemVer depois CI/gates |
| develop | integração | Feature/fix/chore entram por PR e merge commit |
| feature/fix/chore/task-agent-slug | develop → develop | Um owner, testes, plano, revisão técnica pelo outro agente |
| release/x.y.z | develop → main; depois develop | Freeze, version/changelog/evidence, correções limitadas |
| hotfix/task-agent-slug | main → main; depois develop | Corrigir produção, preservando ambas linhas de histórico |

Merge commit é obrigatório; squash/rebase merge desativados para preservar ancestry e cleanup confiável das worktrees. Commits Conventional Commits (`feat`, `fix`, `test`, `docs`, `chore`, `build`, `ci`). Nenhum force push em main/develop; feature compartilhada não é reescrita sem coordenação. Não commitar secrets/instaladores/binários pesados.

Publicar feature: `git push -u origin <branch>`, `gh pr create --base develop --body-file <arquivo>`; comparar diff/task/CI antes merge. Aprovação técnica por Codex/Claude não equivale a aprovação por outra identidade GitHub. `--reviews 1` exige pessoa/conta autorizada diferente do autor; solo pode usar0 e manter revisão/evidência técnica, com merge pelo mantenedor.

Import inicial preserva histórico via cargo xtask github-import. O importador automatiza o bootstrap quando o repositório está vazio. Alternativa manual: clonar para pasta nova, copiar todos arquivos do pacote exceto .git/.worktrees/.superpowers/target/dist; `git symbolic-ref HEAD refs/heads/main`, `git add --all`, `git commit -m "chore: bootstrap Vynamp"`, `git branch develop`, `git push -u origin main`, `git push -u origin develop`. Esse primeiro commit é exceção de bootstrap, não licença para desenvolver diretamente em main.

Depois, `repo.py protect` configura GitHub: CI Gate obrigatório/strict, dismiss stale reviews, conversations resolvidas, sem force push/deletion, admins sujeitos às regras; default branch develop. Se plano/permissão privada não suportar a API, falhar e registrar pendência, não dizer que ficou protegido. GitHub Environments release-signing/release-publish/hardware-validation usam responsáveis/proteções quando disponíveis.

Tags `vX.Y.Z` e `vX.Y.Z-beta.N` apontam a commit aprovado de main. Pipeline cria draft privado, assinatura/verificação/manifest/SBOM e evidência revisados antes publish pelo mantenedor. A plataforma web futura tem deploy separado M4, não é acionada ao subir spec/player.

[Branch protection](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches).
