# Reinício Windows consolidado

Pedido: ZIP completo corrigido, reinício simples e todos os arquivos de projeto dentro de D:\Projects\vynamp-media-player. Decisões anteriores de produto e Superpowers preservadas; execução inline autorizada pelo pedido. Sem app/release fictício ou force push.

## Plano

- [x] Consolidar paths externos Windows e caches de checks sem colisão com executable, incluindo alias de diretório.
- [x] Preparar worktree Codex de bootstrap e Claude reviewer detached automaticamente, com claim e índice exclusivos, preservando histórico/import PR.
- [x] Incluir INICIAR/ABRIR-CODEX/ABRIR-CLAUDE PowerShell como wrappers do tooling Rust; pacote e repo irmãos dentro da raiz; caches/tmp/logs locais; nunca elevar ou configurar trust global.
- [x] Atualizar versão 0.7.0, START-HERE, prompts, estrutura e evidências; preservar conteúdo funcional/brand e fases.
- [x] TDD, suite/checks, revisão independente, ZIP completo e validação de cada hash em cópia extraída.

## Aceite

Fluxos empty/nonempty Git isolam Codex/Claude sem branchesdevelop ausentes ou sobrescrever trabalho. Caminhos Git ordinários e UNC normalizados só nos argumentos externos. Checks não recompilam driver em execução. Todos os arquivos do projeto cabem na raiz indicada; programas já instalados permanecem suas instalações existentes. Scripts falham com mensagens reais sem marcar bootstrap concluído. GitHub/Windows nativo/hardware/Claude runtime não serão declarados testados no Linux.
