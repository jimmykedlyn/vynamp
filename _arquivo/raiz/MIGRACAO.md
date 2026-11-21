Vou parar de usar o Paperclip e voltar a desenvolver o Vynamp direto com o Claude Code, usando multiagentes (subagentes). O backlog passará a ser a base de tarefas no Notion. Prepare os repositórios para essa transição.

ESCOPO
- Estou executando no Windows, dentro da pasta vynamp. Ela é a base de trabalho.
- Primeiro, identifique a estrutura:
  - Se a própria pasta for um repositório git (tem .git), trate-a como o repositório principal.
  - Procure também repositórios em subpastas (ex.: vynamp-engine, vynamp-player,
    vynamp-decode, vynamp-encode, vynamp-output).
  - Me mostre a lista de repositórios encontrados, com a branch atual e o remoto de cada um,
    antes de começar.
- Os repositórios já estão clonados, mas estão desatualizados.
- Trabalhe um repositório por vez e mostre o resultado de cada um antes de passar ao próximo.
- Não saia da pasta vynamp nem altere nada fora dela.

CUIDADOS NO WINDOWS
- Antes de tudo, verifique se git, gh, cargo e node estão disponíveis e se o `gh auth status`
  está ok. Se faltar algo, me avise em vez de instalar.
- Quebras de linha: verifique o `git config core.autocrlf`. Se o `git status` mostrar arquivos
  modificados só por diferença de CRLF/LF, NÃO commite isso: me mostre e proponha a correção
  (ex.: .gitattributes).
- Se algum script do repositório for só para Linux/macOS (bash, Makefile), não o reescreva;
  apenas informe e siga com o equivalente que funcionar no Windows.

ETAPA 1 — ATUALIZAR OS REPOSITÓRIOS LOCAIS
Para cada repositório:
1. Mostre a branch atual e o `git status`.
2. Se houver alterações não commitadas, não descarte nada: guarde com
   `git stash push -u -m "antes-de-sair-do-paperclip"` e me informe o que foi guardado.
3. `git fetch --all --prune --tags`
4. Antes de atualizar, verifique se existem commits locais que nunca foram enviados ao remoto
   (`git log --branches --not --remotes --oneline`). Liste-os e não os perca.
5. Vá para a branch padrão (`git checkout main` ou a branch padrão do repositório) e rode
   `git pull --ff-only`.
   - Se falhar porque a main local divergiu do remoto, pare e me mostre a divergência
     (`git log --oneline main...origin/main`). Não faça rebase, merge nem reset sem minha aprovação.
6. Se o repositório tiver submódulos: `git submodule update --init --recursive`.
7. Mostre quantos commits foram atualizados (ex.: "main avançou 47 commits") e um resumo do que
   mudou, agrupado por área.
8. Liste os PRs abertos (`gh pr list`) e as branches remotas criadas pelos agentes do Paperclip
   (ex.: task/TASK-*). Para cada uma, informe: o que contém, se o CI está verde e se já foi
   mergeada. Não feche PRs nem apague branches sem minha aprovação.

ETAPA 2 — LEVANTAR O QUE É ESPECÍFICO DO PAPERCLIP
Procure referências ao Paperclip e à forma de operar dele, em:
- CLAUDE.md, AGENTS.md, .claude/ (settings, skills, hooks, commands, agents), .mcp.json
- docs/, README, scripts, xtask, workflows do GitHub Actions
- variáveis de ambiente e arquivos de configuração
Inclua: heartbeat, wake/sleep, issues e status do Paperclip, orquestrador e agentes executores,
claim/handoff, o MCP paperclip-assigned e os apps da galeria, regras do tipo "deixe o Paperclip
acordar você", o fallback do gh gerenciado, o backlog.json e o `cargo xtask docs-sync`.

Para cada ocorrência, mostre: arquivo e linha, o que faz, e a classificação:
- REMOVER: só faz sentido no Paperclip.
- ADAPTAR: a ideia continua útil para o desenvolvimento com Claude Code, mas o texto precisa
  mudar. Exemplos: usar GH_TOKEN, não fazer loops de sleep, exigir evidência de CI no PR.
- MANTER: não depende do Paperclip.

PARE AQUI e me mostre o inventário antes de alterar qualquer coisa.

ETAPA 3 — LIMPEZA (só depois da minha aprovação)
- Crie uma branch `chore/remove-paperclip` em cada repositório.
- Aplique as remoções e adaptações aprovadas.
- Não quebre o CI. Se algum check (inclusive o CI Gate) depender do que for removido, ajuste o
  workflow e explique a mudança.
- Rode build, testes e lint localmente antes de commitar.
- Faça commits pequenos e descritivos.

ETAPA 4 — MIGRAR OS AGENTES DO PAPERCLIP PARA SUBAGENTES DO CLAUDE CODE
Hoje o Paperclip tem 5 agentes. Recrie-os no Claude Code:

| Agente        | Papel                                           | Modelo            | Thinking |
|---------------|-------------------------------------------------|-------------------|----------|
| orquestrador  | PM - orquestra as TASKs                         | claude-opus-5-5   | medium   |
| implementador | Engineer - implementação com TDD                | claude-opus-5-5   | high     |
| checador      | Roda fmt, clippy e testes e reporta             | claude-sonnet-5   | medium   |
| revisor       | QA - revisão com contexto limpo                 | claude-sonnet-5-5 | high     |
| integrador    | Integra PRs (CI verde, merge, atualizar status) | claude-sonnet-5-5 | medium   |

Sobre o thinking:
- Verifique (`claude --version` e a documentação de subagentes) se esta versão do Claude Code
  aceita definir o esforço/thinking no frontmatter de cada subagente. Se aceitar, configure
  por agente conforme a tabela.
- Se não aceitar, não invente campos: registre o nível desejado no AGENTS.md e me diga como
  aplicá-lo (ex.: configuração da sessão principal ou do 9router).
- O orquestrador é a sessão principal, então o thinking dele é o da própria sessão.

Passos:
1. Antes de escrever, leia as instruções atuais de cada agente (no repositório ou no que eu te
   passar do Paperclip) e preserve as regras úteis, removendo o que for específico do Paperclip
   (heartbeat, wake, issues, claim/handoff).
2. Crie ou reescreva o AGENTS.md na raiz com:
   - o papel, as responsabilidades e os limites de cada agente;
   - o fluxo de uma TASK: orquestrador lê a TASK no Notion → implementador (TDD) →
     checador (fmt/clippy/test) → revisor (contexto limpo, sem ver o raciocínio do implementador)
     → integrador (PR, CI verde, merge) → orquestrador atualiza o status no Notion;
   - o que cada agente NÃO pode fazer (ex.: revisor e checador não editam código;
     só o integrador faz merge; ninguém faz push --force);
   - regras comuns: Notion como fonte da verdade, GH_TOKEN, sem loops de sleep,
     evidência obrigatória no PR.
3. O orquestrador é a sessão principal do Claude Code (subagentes não chamam outros subagentes).
   Coloque as regras dele no AGENTS.md, não num arquivo de subagente.
4. Crie em .claude/agents/ um arquivo para cada um dos outros 4 (implementador.md, checador.md,
   revisor.md, integrador.md), com frontmatter (name, description, tools, model e, se suportado,
   o thinking) e as instruções do papel. Restrinja as ferramentas: checador e revisor sem
   Write/Edit; integrador com git/gh, mas sem editar código.
5. No CLAUDE.md, adicione `@AGENTS.md` para que as regras sejam carregadas sempre.
6. Me mostre os arquivos antes de commitar.

ETAPA 5 — DEIXAR PRONTO PARA INTEGRAR COM O BACKLOG NO NOTION
Não implemente a integração ainda; só prepare o terreno:
- No AGENTS.md, registre que a fonte da verdade das tarefas é a base de backlog do Notion, com
  os campos TASK, Título, Módulo, Fase, Onda, Prio, Depende de e Comandos (link para a página
  com os passos da tarefa).
- Descreva o fluxo esperado por tarefa: ler a TASK no Notion → conferir se as dependências estão
  concluídas → criar a branch task/<TASK-ID> → seguir o fluxo de agentes da Etapa 4 com base na
  página de Comandos → PR com evidência (CI verde, testes, resumo) → atualizar o status no Notion.
- Não recrie o backlog.json nem o docs-sync como fonte de tarefas.
- Liste o que falta para a integração funcionar (ex.: MCP do Notion configurado no Claude Code,
  ID da base, campo de status) e proponha os próximos passos.

ETAPA 6 — ENTREGA
- Abra um PR por repositório com as mudanças das Etapas 3, 4 e 5 e mostre o link.

REGRAS GERAIS
- Nunca use git push --force, git reset --hard, nem apague branches, arquivos, stashes ou PRs
  sem minha aprovação explícita.
- Se algo for ambíguo, pergunte em vez de supor.
- Ao final, entregue um resumo por repositório: o que foi atualizado, guardado em stash,
  removido, adaptado e criado, e o que ficou pendente.