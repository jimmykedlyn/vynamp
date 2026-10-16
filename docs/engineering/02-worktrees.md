# Worktrees, ownership e recuperação

Versão 0.7.0 · 2026-10-14 · Configuração do projeto; player ainda não implementado.

Padrão local: `.worktrees/<agent>/<task-lowercase>-<slug>/`. Agent é codex ou claude. Branch `feature/<task>-<agent>-<slug>`; tipos fix/chore seguem mesmo padrão, base develop; hotfix usa main. Worktrees e `.superpowers` ignoradas. Nunca copiar `.git` para criar isolamento.

`cargo xtask repo worktree` valida TASK existente, slug seguro e gitignore; cria branch/worktree e claim atômico no Git common dir `.git/auralis/claims/<TASK>/owner.json`. O common dir é compartilhado entre worktrees; claim não vai ao GitHub e funciona para dois processos locais. Em máquinas distintas, ownership é registrado no issue/PR e evidência; lock local não protege todo servidor remoto.

Antes criar, detectar isolamento nativo: git-dir distinto de git-common-dir e não submodule significa worktree existente. Reutilizar essa worktree; doctor e baseline próprios. Nunca abrir Codex e Claude no mesmo diretório/index. Só um owner por TASK; dependências podem ser revisadas pelo outro agente em modo leitura.

Root checkout é bootstrap/sincronização; main/develop não são área de implementação. Atualizar refs antes nova TASK, sem modificar worktree de outro agente. Mudança de owner exige parar sessão, salvar/commitar trabalho, registrar handoff e transferir claim conscientemente; não apagar lock automaticamente por idade.

Após PR merge com merge commit, fetch/pull refs e `cargo xtask repo remove-worktree --agent <agent> --task <TASK>`: exige limpa e integrada em develop; hotfix também em main. Proibido remove --force, reset --hard, git clean -fdx e delete de trabalho não integrado. Mover a raiz com worktrees registradas exige remover/recriar ou git worktree repair com revisão; o pacote deve ser movido antes bootstrap.

Execute remoção somente no checkout central. O comando confere TASK/owner, branch/base e path padrão da claim, destino sem symlink, common Git dir e branch atual da worktree. HEAD detached ou branch trocada bloqueiam remoção mesmo se a branch antiga estiver integrada. A ancestry verificada é a do HEAD real; todos os checks precedem `git worktree remove` sem force. Claim/branch só são removidas após sucesso dos passos Git. Falha intermediária preserva o que restou e exige inspeção, sem cleanup destrutivo automático.

Além das bases obrigatórias develop/main, o preflight verifica o critério de merge de `git branch -d`: upstream resolvido da branch, ou HEAD do checkout central quando não há upstream resolvido. Se esse destino não contém o HEAD da worktree, recusa antes de remover path/branch/claim, com `Branch deletion not merged into upstream/checkout HEAD; removal refused`. Por exemplo, uma feature integrada só em develop e sem upstream é preservada quando o central está em main; após escolher develop no central, o retry normal funciona. Se houver upstream, confira/corrija sua configuração conscientemente, pois Git o prefere ao HEAD central. O comando não troca checkout/upstream automaticamente nem remove refs com force. Esse preflight não torna atômicos erros de I/O/locks ou interrupções entre as mutações.

## Interrupção e handoff

Criação reserva a TASK com `create_dir` atômico antes de registrar owner/criar worktree. Dois processos concorrentes locais não podem assumir a mesma TASK. Falha recuperável antes de uma worktree existir libera apenas a claim que essa invocação criou; destino preexistente e branch colidente são preservados. Se o processo for encerrado depois da reserva, pode restar claim incompleta: ela continua bloqueando outro owner, inclusive sem owner.json. Idade do diretório não autoriza takeover.

Ao encontrar claim incompleta ou divergência de branch/path/base, pare as sessões envolvidas e preserve os dados. No checkout central, confira `git worktree list --porcelain`, refs/HEAD e owner.json; em cada worktree envolvida, confira status e índice. Registre a causa e combine recuperação com o owner/mantenedor. Não edite a claim para contornar a validação, não transfira ownership implicitamente e não repita remoção manual com force. Mudanças simultâneas de checkout/refs/arquivos durante remoção não são suportadas: interrompa as sessões antes do cleanup; a claim de TASK não é um lock de todas as operações Git.

Handoff deve informar TASK/owner, pasta exata, branch, common dir/claim/índice, SHA base e HEAD completos, PR real ou pendência, arquivos ainda não commitados, comandos/resultados/ambiente/limitações e próximo prompt pronto. Autor e revisor registram seus próprios checks separadamente. O guia externo do projeto recebe painel e bloco REVISAR atualizados a cada entrega; duas ferramentas não equivalem a duas identidades GitHub.

Baseline: `cargo xtask ci docs` e testes do subsistema disponíveis. Falha preexistente é registrada e resolvida/decidida dentro do escopo autorizado; não atribuir à task sem prova. Caso app já forneça worktree, não lutar com o harness para impor outra localização.

Namespace Git histórico legado (`auralis/claims`) é preservado entre toolings 0.4 e 0.5 para que claims ativos não sejam ignorados na renomeação. Não remover claims para alterar branding.
