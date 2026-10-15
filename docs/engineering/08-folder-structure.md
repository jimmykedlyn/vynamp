# Estrutura da pasta e responsabilidade

Versão 0.7.0 · 2026-10-14 · Configuração do projeto; player ainda não implementado.

```text
D:\Projects\vynamp-media-player\
├── INICIAR.ps1 / AMBIENTE.ps1
├── ABRIR-CODEX.ps1 / ABRIR-CLAUDE.ps1
├── PROMPT-CODEX.txt / PROMPT-CLAUDE.txt / LEIA-ME.md
├── pacote\                # spec e tooling originais do ZIP
├── repo\                  # checkout Git preservando histórico
│   ├── .git\
│   ├── .worktrees\codex\task-e15-01-bootstrap\
│   ├── .worktrees\claude\review-bootstrap\
│   ├── target\xtask-ci\   # cache dos checks isolado do driver
│   └── dist\              # artefatos futuros, ignored
├── .cache\cargo\          # dependências Cargo do projeto
├── .cache\bootstrap-build\
├── .cache\tmp\
└── logs\
```

Mover todos os arquivos do ZIP incluindo hidden antes gerar .git/worktrees. Não versionar .worktrees, dist, credentials ou caches; git claims ficam no common dir e não são copiados para outra máquina. crates/apps têm README de contrato; não contêm player/stubs que fingem funcionalidades. Não há .git no ZIP; importar ao repo preservando o histórico com START-HERE.

Revisão 0.6: `config/format-catalog.json`, `docs/product/08-completeness-audit.md`, `docs/research/03-completeness-sources.md`, `tasks/epics/E16.md` e `E17.md`; backlog de 176 TASK/REQ. Nenhuma pasta precisa ser criada manualmente antes de extrair; incluir arquivos ocultos.

A pasta `pacote` contém os mesmos docs/crates/apps/config/xtask do projeto; o importador copia para `repo`. Usar [passo a passo Windows](09-windows-restart.md). A raiz única exigida substitui o exemplo anterior com checkout fora da pasta.
