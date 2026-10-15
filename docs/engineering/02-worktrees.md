# Worktrees, ownership e recuperação

Versão 0.7.0 · 2026-10-14 · Configuração do projeto; player ainda não implementado.

Padrão local: `.worktrees/<agent>/<task-lowercase>-<slug>/`. Agent é codex ou claude. Branch `feature/<task>-<agent>-<slug>`; tipos fix/chore seguem mesmo padrão, base develop; hotfix usa main. Worktrees e `.superpowers` ignoradas. Nunca copiar `.git` para criar isolamento.

`cargo xtask repo worktree` valida TASK existente, slug seguro e gitignore; cria branch/worktree e claim atômico no Git common dir `.git/auralis/claims/<TASK>/owner.json`. O common dir é compartilhado entre worktrees; claim não vai ao GitHub e funciona para dois processos locais. Em máquinas distintas, ownership é registrado no issue/PR e evidência; lock local não protege todo servidor remoto.

Antes criar, detectar isolamento nativo: git-dir distinto de git-common-dir e não submodule significa worktree existente. Reutilizar essa worktree; doctor e baseline próprios. Nunca abrir Codex e Claude no mesmo diretório/index. Só um owner por TASK; dependências podem ser revisadas pelo outro agente em modo leitura.

Root checkout é bootstrap/sincronização; main/develop não são área de implementação. Atualizar refs antes nova TASK, sem modificar worktree de outro agente. Mudança de owner exige parar sessão, salvar/commitar trabalho, registrar handoff e transferir claim conscientemente; não apagar lock automaticamente por idade.

Após PR merge com merge commit, fetch/pull refs e `cargo xtask repo remove-worktree --agent <agent> --task <TASK>`: exige limpa e integrada em develop; hotfix também em main. Proibido remove --force, reset --hard, git clean -fdx e delete de trabalho não integrado. Mover a raiz com worktrees registradas exige remover/recriar ou git worktree repair com revisão; o pacote deve ser movido antes bootstrap.

Baseline: `cargo xtask ci docs` e testes do subsistema disponíveis. Falha preexistente é registrada e resolvida/decidida dentro do escopo autorizado; não atribuir à task sem prova. Caso app já forneça worktree, não lutar com o harness para impor outra localização.

Namespace Git histórico legado (`auralis/claims`) é preservado entre toolings 0.4 e 0.5 para que claims ativos não sejam ignorados na renomeação. Não remover claims para alterar branding.
