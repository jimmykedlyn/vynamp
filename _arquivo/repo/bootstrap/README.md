# Templates de início Windows

Versão 0.7.0. O bundler publica cópias destes wrappers na raiz externa `vynamp-media-player`, com fontes em `pacote`. Execute os wrappers externos conforme [guia](../docs/engineering/09-windows-restart.md), não nesta pasta de templates.

Wrappers apenas orquestram o tooling Rust e iniciam CLIs existentes. Cargo/cache/tmp/logs locais; sem configuração global de trust/Git ou remoções. Identidade Git é solicitada quando necessária e gravada somente no checkout. Autenticação GitHub acontece antes do transcript. Prompts enviados como argumento único, sem newlines, compatíveis com shims Windows. Não há Python obrigatório nem script que declare app/codec/hardware pronto.

Atalhos usam target/driver exclusivo por worktree; hooks usam target/xtask-hooks próprio; checks usam target/xtask-ci. Cache de downloads Cargo é compartilhado apenas dentro da raiz. Nenhum build de outra sessão recompila o mesmo driver em execução.
