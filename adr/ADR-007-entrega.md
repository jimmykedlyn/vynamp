# ADR-007 — colaboração, Gitflow e formatos de entrega

Versão 0.7.0 · 2026-10-14 · Configuração do projeto; player ainda não implementado.

Status: requisitos de configuração aceitos pelo usuário; operação remota/signing depende de ativação externa real.

Decisão: Superpowers obrigatório no Claude Code, instrução comumAGENTS e entrypointCLAUDE; uma worktree/owner porTASK e revisão técnica por subagente; Gitflowmain/develop com merge commits e PR; CI Gate único agregado. GitHubprivado jimmykedlyn/vynamp-mediaplayer informado pelo usuário, não verificado/pushado no ambiente.

Instaladores1.0 definidos: WindowsNSIS+ZIP, LinuxAppImage+DEB, macOSUniversal2DMG+ZIP, AndroidAPK+AAB. MSIX/Flatpak são adicionais futuros, não ambiguity do gateinicial. Preflight impede release antes app/hardware/licença/signing. Nativecodeplatform signatures e manifesto assinado protegem entrega; Linux usa signature do manifesto/hash.

Consequência: bootstrap pode rodar agora, player/installer test dependem de implementação. Rust/NDK/SDK pins resultam deM0 real. Não incluir .git/worktrees noZIP, pois o usuário moverá a pasta. Novos scopes/arquiteturas/instaladores exigem ADR e tasks.

Nota (2026-10-23, supera parcialmente a decisão original): o fluxo passou a agente único. Claude Code é o único autor e revisor (Opus orquestra e trata alto risco, Sonnet implementa e revisa, Haiku executa checks); o segundo agente foi removido do repositório e a revisão cruzada entre agentes deixou de existir. Merge e release continuam com o mantenedor humano. Registros históricos que citavam o agente anterior permanecem apenas como evidência da época; a cadeia documental viva não o menciona.
