# Modelo de ameaças e controles

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

Assets: arquivos do usuário, biblioteca/ratings, credentials, integridade do áudio, conta/provider, pacotes, updater. Trust boundaries: arquivo de mídia, metadado/cover, URL/manifest, pacote/theme/plugin, helper, web API e ingest admin. Local user media é entrada não confiável para parsers, não autorização para executar código.

| Entrada / ameaça | Controle obrigatório | Teste |
|---|---|---|
| Mídia/tag/capa malformada | budgets, parser seguro, native helper isolado | fuzz/corpus, oversized/decompression bomb |
| Playlist/HLS/redirect SSRF | scheme/domain policy, DNS+redirect checks | loopback, link-local, rebound, file URI |
| Pacote zip traversal/bomb | canonical path, limits, staging | ../, absolute, symlink, huge ratio |
| Plugin loop/rede/arquivo indevido | fuel/deadline/memory/cap broker | trap, denied calls, flood |
| Native helper compromise | sandbox OS, mínimo handles, IPC auth | child crash, malicious RPC, escape review |
| Token/URL em logs | keychain e redaction estrutural | secret fixture em export/log |
| Update adulterado/downgrade | assinatura+hash+metadata expirável | invalid key, replay, rollback attack |
| DB/write interrompido | atomic write/transaction/backups | kill durante save/migrate |
| Web ingest remoto/upload | SSRF, content probe, quotas, workers | invalid body/foreign tenant |
| Sync cross-user | authz por objeto, tenant/account scope | IDOR, replay, stale revision |

## Segurança não bloqueante

Mídia local e rádio não exigem conta. Consentimentos aparecem só quando ação acessa externo, instala extensão ou grava/exclui. Sem prompts genéricos toda execução. Pacotes builtin auditados não precisam repetir permissões. Revoke de plugin cancela requests e encerra handles; revogar rede não mata playback local.

## Updater

Feed HTTPS + assinatura de manifesto e artefato com key rotation, channel, version, target, hash,size,expires,min_supported e revoked builds. Validar antes executar instalador; downloads staged, rollback se launch health check falha. Config/schema compatibility verificada; não downgrade DB sem backup. Offline mantém versão atual. Um algoritmo improvisado de assinatura não será criado; usar esquema/runtime estabelecido e auditado.

## Limites

Sandbox reduz impacto, não prova ausência de vulnerabilidade. Bit-perfect e permissões não implicam direitos sobre conteúdo. Native/codec dependencies entram em SBOM e triagem de CVEs; crítico explorável bloqueia release, exceptions documentadas com owner/expiry. Security review focused nos parsers, IPC e update antes1.0.
