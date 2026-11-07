# Claude Code — Vynamp

Leia e siga [AGENTS.md](AGENTS.md); ele é a fonte única do contrato, não duplique regras aqui. Claude Code é o único autor e revisor (Opus orquestra/alto risco, Sonnet implementa e revisa, Haiku executa checks). Superpowers é obrigatório: instalar e verificar antes de implementar. Use worktree `claude/<task>-<slug>`, branch própria e um único owner por TASK; nenhuma sessão divide o index Git.

Comece por [SPEC.md](SPEC.md) e tarefa selecionada. Plano em docs/superpowers/plans, evidências em docs/evidence. Não editar main/develop, não mergear/publicar releases automaticamente. Validar testes/CI, criar PR para develop conforme Gitflow. Configuração do plugin em .claude/settings.json habilita mas não instala sozinha; bootstrap local é obrigatório.
