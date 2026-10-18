# Woodpecker pilot — operação local

Versão 0.7.0 · 2026-10-18 · TASK-E15-04, piloto Windows/Linux; não substitui o CI Gate multiplataforma. Desenho: [E15-04](../../../docs/superpowers/specs/2026-10-18-e15-04-woodpecker-cloudflare-design.md) · [plano](../../../docs/superpowers/plans/2026-10-18-e15-04-woodpecker.md).

## Componentes

- `compose.yaml`: servidor Woodpecker 3.19.0 (HTTP `127.0.0.1:8000`, gRPC `127.0.0.1:9000`, SQLite no volume `vynamp-woodpecker-data`), agente Linux/Docker (`host=vynamp-linux`) e cloudflared (profile `tunnel`).
- Agente Windows nativo, backend Local (`host=vynamp-windows`), iniciado por `scripts/Start-Woodpecker.ps1` com ambiente sem tokens da sessão interativa.
- Túnel nomeado `vynamp-woodpecker` → `https://ci-cd-vynamp.jkca.lat` → somente `http://woodpecker-server:8000`; fallback 404; gRPC nunca exposto.
- Pins e hashes em `versions.json`; `scripts/Install-Tools.ps1` baixa e verifica os binários Windows em `D:\Projects\vynamp-media-player\.cache\woodpecker\bin` sem alterar instalações do sistema; cloudflared é a instalação global winget 2026.10.1, verificada por hash.
- Pipelines em `.woodpecker/linux.yaml` e `.woodpecker/windows.yaml`; status GitHub `woodpecker-pilot/...`.

## Runtime fora do Git

`D:\Projects\vynamp-media-player\.cache\woodpecker\`: `bin`, `downloads`, `secrets` (ACL só do usuário: `server.env`, `agent.env`, `cloudflared\`), `logs`, `windows-agent` (workspace, tmp, `CARGO_HOME` do CI).

## Primeira ativação

1. `pwsh -File infra\ci\woodpecker\scripts\Install-Tools.ps1`
2. Mantenedor: `cloudflared tunnel login` com o cloudflared global (winget 2026.10.1; navegador) e depois `scripts\Initialize-Tunnel.ps1`.
3. Mantenedor: GitHub OAuth App com homepage `https://ci-cd-vynamp.jkca.lat` e callback `https://ci-cd-vynamp.jkca.lat/authorize`; depois `scripts\Initialize-Secrets.ps1` (prompt local oculto).
4. `scripts\Start-Woodpecker.ps1`; login na UI; ativar o repo; aprovação obrigatória para forks; repo não trusted.

## Rotina

- Iniciar: `scripts\Start-Woodpecker.ps1` · Parar: `scripts\Stop-Woodpecker.ps1` (não remove volumes).
- Checagem: `scripts\Test-Preflight.ps1` e `-SelfTest`.
- Backup do SQLite: parar o servidor e copiar o volume inteiro; não copiar arquivo ativo com WAL.

## Limitações conhecidas

- Backend Local não isola steps: herdam o ambiente do agente, inclusive `WOODPECKER_AGENT_SECRET` (limitação documentada pelo Woodpecker). Por isso só código confiável do repo privado roda no agente Windows.
- Steps PowerShell exigem `$PSNativeCommandUseErrorActionPreference = $true` como primeiro comando; sem ele a falha de comando nativo intermediário é mascarada (provado em teste local).
- MacIntel, MacSilicon, Android e hardware continuam pendentes; workflows GitHub Actions permanecem até migração revisada.
