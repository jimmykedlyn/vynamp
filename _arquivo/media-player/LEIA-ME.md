# Vynamp 0.7.0 — início Windows

Pasta do projeto: D:\Projects\vynamp-media-player. PowerShell normal, sem elevar.

1. Primeira vez: `powershell -NoProfile -ExecutionPolicy Bypass -File .\INICIAR.ps1`
2. Abrir o agente: `powershell -NoProfile -ExecutionPolicy Bypass -File .\ABRIR-CLAUDE.ps1`

Claude é o único autor e revisor. Regras: AGENTS.md. Estado atual: STATE.md. Ordem: ROADMAP-rodadas.md.
Você só aprova e faz o merge commit dos PRs no GitHub e avisa "merge feito".

repo = checkout Git (worktrees em repo\.worktrees\claude); pacote = distribuição original; .cache = caches; logs = execuções; archive = histórico do fluxo antigo.
Rust, Git/gh e Claude devem estar no PATH. O CI roda em GitHub Actions (ADR-014); Docker Desktop não é mais necessário para o CI.
