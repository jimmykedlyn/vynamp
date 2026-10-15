# Codex, Claude Code e Superpowers obrigatório

Versão 0.7.0 · 2026-10-14 · Configuração do projeto; player ainda não implementado.

AGENTS.md é fonte comum; CLAUDE.md referencia esse contrato. `.claude/settings.json` habilita Superpowers no escopo do projeto; `.codex/config.toml` define sandbox/approval sem credentials/modelos globais. Plugin é instalado pelo manager nativo de cada ferramenta, não apenas por copiar Markdown.

Após instalação, executar `cargo xtask doctor --agent both`. Doctor consulta discovery nativo e exige versão ativa explícita; formato desconhecido falha, sem inventar marcador. Instalar Codex em `/plugins`, buscar superpowers e Install Plugin. Claude: `/plugin install superpowers@claude-plugins-official` ou `cargo xtask install --agent claude`. Reiniciar sessões e confirmar using-superpowers. Nenhuma instalação local do usuário é inferida a partir dos settings ou deste documento.

Pipeline de trabalho obrigatório: usar using-superpowers; conferir spec/ADR; brainstorming apenas para decisão nova não coberta pela spec; writing-plans para task; isolamento conforme using-git-worktrees; TDD para comportamento; systematic-debugging em falhas; verification-before-completion; revisão do outro agente e finishing-a-development-branch. O pedido de configuração já autoriza a preparação; não reabrir aprovações de decisões já dadas. Regras explícitas do usuário prevalecem sobre skills.

Planos em docs/superpowers/plans com arquivo, interface, testes/oráculo e dependências concretas. Evidence com commit/test/hardware; ledger temporário em `.superpowers` (ignored) e resumo preservado no PR/doc evidence. Codex implementa/Claude revisa ou vice-versa; reviewer usa outra sessão e diff/commit, não edita o checkout do autor. Dois agentes não recebem permissões administrativas automáticas nem credenciais diferentes inventadas.

Claude cloud pode não carregar plugins declarados no projeto; verificar runtime antes tarefa e usar ambiente local/self-hosted compatível. Skills baixadas não equivalem a hooks do plugin funcionando. Se discovery JSON mudar, doctor falha até adaptar parser/lock com prova. Atualização de Superpowers requer PR, release notes e teste de carregamento de ambos; não pin de branch main fingindo versão estável.

Fontes: [Superpowers oficial](https://github.com/obra/superpowers), [Claude plugin management](https://code.claude.com/docs/en/discover-plugins), [Codex plugins](https://learn.chatgpt.com/docs/plugins). Não usar instruções antigas de `.codex/INSTALL.md` que atualmente retornam404.
