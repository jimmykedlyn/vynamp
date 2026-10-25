# Claude Code e Superpowers obrigatório

Versão 0.7.0 · 2026-10-14 · Configuração do projeto; player ainda não implementado.

AGENTS.md é a fonte única do contrato; CLAUDE.md referencia esse contrato. Fluxo de agente único: Claude Code é o único autor e revisor (Opus orquestra e trata alto risco, Sonnet implementa e revisa, Haiku executa checks); merge e release ficam com o mantenedor. `.claude/settings.json` habilita Superpowers no escopo do projeto. O plugin é instalado pelo manager nativo do Claude, não apenas por copiar Markdown.

Após instalação, executar `cargo xtask doctor`. Doctor consulta o discovery nativo e exige a versão ativa explícita (`expected_versions.claude` no lock); entradas de outros projetos não contam. Formato desconhecido falha, sem inventar marcador. Instalar com `/plugin install superpowers@claude-plugins-official` ou `cargo xtask install`. Reiniciar a sessão e confirmar using-superpowers. Nenhuma instalação local do usuário é inferida a partir dos settings ou deste documento.

Pipeline de trabalho obrigatório: usar using-superpowers; conferir spec/ADR; brainstorming apenas para decisão nova não coberta pela spec; writing-plans para task; isolamento conforme using-git-worktrees; TDD para comportamento; systematic-debugging em falhas; verification-before-completion; revisão técnica por subagente em contexto limpo e finishing-a-development-branch. O pedido de configuração já autoriza a preparação; não reabrir aprovações de decisões já dadas. Regras explícitas do usuário prevalecem sobre skills.

Planos em docs/superpowers/plans com arquivo, interface, testes/oráculo e dependências concretas. Evidence com commit/test/hardware; ledger temporário em `.superpowers` (ignored) e resumo preservado no PR/doc evidence. O revisor usa outra sessão/subagente e diff/commit. Nenhuma permissão administrativa ou credencial é inventada.

Claude cloud pode não carregar plugins declarados no projeto; verificar runtime antes da tarefa e usar ambiente local/self-hosted compatível. Skills baixadas não equivalem a hooks do plugin funcionando. Se o discovery JSON mudar, doctor falha até adaptar parser/lock com prova. Atualização de Superpowers requer PR, release notes e teste de carregamento; não pin de branch main fingindo versão estável.

Pin vigente: Superpowers `7.0.0` no Claude, com [saída real preservada](../evidence/task-e15-01-claude-manager-7.0.0.json). O lock do Codex foi removido em 2026-10-23 junto com o agente; registros históricos que o citam permanecem como evidência da época.

Fontes: [Superpowers oficial](https://github.com/obra/superpowers), [Claude plugin management](https://code.claude.com/docs/en/discover-plugins).
