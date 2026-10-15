# Plugins próprios, SDK e isolamento

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Tipos e execução

Provider, metadata/lyrics, library utility, visualizer, control integration e decoder externo. WASM/Wasmtime para control/metadata/providers com capabilities; decoder pesado/processo nativo helper com RPC e shared memory limitado; DSP real-time first-party compilado e auditado em 1.0. DSP terceiro WASM não roda diretamente na callback; M5 pesquisa worker com latência e watchdog. Não expor ABI Rust instável nem carregar DLL arbitrária no processo principal.

Formato `.vynplug`: ZIP manifest + module.wasm ou helper declarado + license + checksums + signature. Nativos opcionais são classe de risco distinta e requerem consentimento explícito, OS sandbox e assinatura. Entitlements least privilege; assinatura prova origem/integridade, não segurança. Marketplace não substitui sandbox.

Manifest: id/version/sdk_version/kind, entrypoint, permissions, resource_limits, supported_platforms, homepage/license, sha256 por arquivo. SDK interface version1, WIT para WASM e RPC versionado para helper; independentemente versionado do app. Major incompatível bloqueia load; versões de dependência permitidas sem download transitivo invisível.

Permissions: network.domains, files.read.selected_roots, files.write.selected_output, library.read, library.modify, playback.control, metadata.read, system.notifications. Sem env vars, processo/shell, filesystem pleno ou loopback automático. Host implementa rede/arquivo por broker; não conceder WASI network genérico. Prompt com purpose/domain, grants revogáveis e exportáveis sem token.

## Budgets iniciais

WASM memory64MiB por plugin, execução por chamada com fuel/epoch interruption configurado e deadline500ms em metadata; provider I/O async via host pode ter timeout10s sem ocupar worker compute. max10 calls concorrentes por plugin, payload1MiB, rate100events/s; cooldown e circuit breaker após3 falhas/60s. Valores por classe podem mudar via ADR e benchmark. CPU limit não se prova apenas por timer externo: configurar interrupção real no runtime.

Plugin init/register/start/event/stop/unload é lifecycle explícito, cleanup no host mesmo após trap. Estado serializado em namespace isolado, migrations versionadas; crashes não tocam banco livremente. Visualizer recebe PCM/FFT limitado downsampled e desenha comandos/scene allowlisted, nunca device access. Meter feed tem autorização de áudio mas não network por default.

## SDK e distribuição

Gerador de template Rust→WASM, docs de API, três exemplos: spectrum view, lyrics local e radio provider mock. Test harness offline simula host/grants/failures. Package validator, signing tool e compatibility report. Registry futuro tem publisher verification, immutable versions, revocation e rollback. Instalação manual funciona sem conta; “trusted” não equivale a “sem permissões”.

Aceite: malicious zip, infinite loop, memory grow, denied permission, domain redirect, stale SDK, event flood, worker crash e revoke during request. Playback local continua; plugin failure aparece no manager com diagnóstico e safe action. Não prometer compatibilidade VST/Winamp nem segurança absoluta.

Perfil Android1.0: validar runtimeWASM, AOT/interpreter/JIT, memory e políticas de loja no gateM0. Não usar downloaded native helper externo; módulos nativos first-party embarcados noAPK. SDK mantém portas comuns e enumera capacidades/targets. Media/files broker usa SAF/ContentResolver, não paths irrestritos. Assinatura/registry não ignora restrições do OS.
