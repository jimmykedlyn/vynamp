# Evidência — atualização Woodpecker 3.19.0 → 3.20.1

2026-11-04 · Windows 11 Pro 10.0.26300, PowerShell 7, Docker Desktop. Escopo: servidor, agente Linux (imagens) e agente/CLI Windows (binários). plugin-git (2.10.1) e cloudflared (2026.10.1) inalterados.

## Fontes e pins

- Release: https://github.com/woodpecker-ci/woodpecker/releases/tag/v3.20.1 (3.20.0 evitada: o próprio release avisa sobre breaking change não intencional, #7398, corrigido na 3.20.1). Mudanças de segurança relevantes da 3.20.0: token do agente removido do ambiente do processo (#7412) e quebras de linha YAML citadas na substituição de ambiente (#7283).
- Digests de índice OCI multi-arch (mesmo tipo do pin anterior, conferido com `docker buildx imagetools inspect`):
  - server `sha256:0f31560e29d67ec4a06d6eeee4bac38d2a2697a6a741b8a0db3998180f3a3517`
  - agent `sha256:a7d7299a13add0b3936fa0ac9ed23507fe283a7fc7d9596c4289d7e8206541eb`
- Binários Windows: SHA256 local idêntico ao `checksums.txt` oficial do release (agent `b98db20f…f4cb`, cli `8982eac8…2412`).

## Execução

1. `scripts/Test-Preflight.ps1` → `preflight passed`.
2. `scripts/Stop-Woodpecker.ps1`; backup de `/var/lib/woodpecker` (SQLite 6.287.360 bytes) em `D:\Projects\vynamp-media-player\.cache\woodpecker\backups\data-v3.19.0-20261104` (fora do Git).
3. `scripts/Install-Tools.ps1` → hashes verificados; `scripts/Start-Woodpecker.ps1` → containers `healthy`.

## Resultado

- `GET /version` → `3.20.1`; log do servidor: `starting Woodpecker server with version '3.20.1'`, sem erros de migração.
- `GET /api/agents`: `vynamp-windows` v3.20.1 (windows/amd64) e `vynamp-linux` (linux/amd64) conectados.
- O agente Linux se reporta como `next-3f1bb0bd0d`, mas a imagem em execução tem o digest pinado, label `org.opencontainers.image.version=3.20.1` e revisão `3f1bb0bd0db28e410ad797034d75780116122c91`, que é exatamente o commit da tag `v3.20.1`. É uma peculiaridade do build upstream, não um desvio de versão.

## Limitações

- Rollback: restaurar o backup no volume `vynamp-woodpecker-data` e reverter os pins; migrações do banco não são reversíveis sem o backup.
- O pipeline deste PR no servidor atualizado é a validação end-to-end.
