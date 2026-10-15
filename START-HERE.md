# Comece aqui — Vynamp 0.7.0

Raiz exigida: `D:\Projects\vynamp-media-player`. ZIP completo, com correções Windows consolidadas. Player ainda não implementado. [Passo a passo completo](docs/engineering/09-windows-restart.md).

## Preparar uma vez

Extraia a pasta `vynamp-media-player` do ZIP em `D:\Projects`. Abra PowerShell normal, sem administrador, e execute **na raiz externa do ZIP**:

```powershell
cd D:\Projects\vynamp-media-player
powershell -NoProfile -ExecutionPolicy Bypass -File .\INICIAR.ps1
```

O script prepara `repo`, preserva histórico GitHub e PRs, executa os checks e cria worktrees isoladas. Pacote original em `pacote`; caches/tmp em `.cache`; logs em `logs`; tudo dentro da raiz. Se faltar autenticação GitHub ou identidade Git, o script solicita os dados necessários. Não altera trust/configuração global do Git, não força push ou merge e não aceita tarefas sem evidências.

## Abrir os agentes

Em dois terminais normais nessa mesma raiz:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\ABRIR-CODEX.ps1
```

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\ABRIR-CLAUDE.ps1
```

Prompts enviados automaticamente. Codex executa E15-01; Claude revisa com índice próprio em detached. Passe o commit do Codex ao Claude; as sessões não conversam automaticamente. Após revisar/integrar E15-01/E15-02 e o PR inicial quando existir, distribuir tarefas independentes E11-01/Codex e E15-03/Claude em novas worktrees. Não pular dependências nem chamar marca/configuração de app implementado.

Rust/Git/gh/Codex/Claude e Superpowers devem estar instalados. Programas e perfis de autenticação/plugins existentes continuam em suas instalações; não são copiados ao repo. A regra de raiz única cobre os arquivos do projeto, builds, dependências Cargo, tmp/worktrees/logs. Não instalar Python.

[Tooling Rust](xtask/README.md) · [Agentes](docs/engineering/01-agents-superpowers.md) · [Estrutura](docs/engineering/08-folder-structure.md). GitHub: `jimmykedlyn/vynamp-mediaplayer`, privado, sem alteração de nome.
