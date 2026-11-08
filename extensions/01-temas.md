# Temas próprios e editor

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

Formato `.vyntheme`: ZIP com manifest.json, tokens.json, layouts/*.json, assets/ e license.txt. Não carrega JS, shaders arbitrários, executáveis, DLLs, WASM ou fontes remotas. Asset budget20MiB unpacked, max200 files, images≤4096px por dimensão e decode budget64MiB; zip ratio/path traversal/symlink checks; manifest≤256KiB. SVG é sanitizado, sem scripts/external refs, ou rasterizado em worker.

Manifest: schema_version1, id reverse-domain, version SemVer, name, author, license, min_app_version, layouts e assets; hashes SHA256 e assinatura opcional v1. Instalação em staging, validação completa, commit atômico por versão, restore anterior se renderer falha. Tema pode alterar cor, geometria declarativa dentro constraints e assets, nunca audio/network/security settings.

Bindings allowlisted: playback.state/title/time, volume, EQ summary, queue count; actions play/pause/seek/open-url/show-library. Não aceitar bindings arbitrários a Rust code. Layout nodes panel/text/icon/button/slider/list/image/visualizer, constraints min/max/anchor/gap, sem loops/expressões ilimitadas. Engine resolve cycles como erro. Tema define Classic/Library/Mini ou herda fallback; não esconde stop/mute/acessibilidade.

Editor M3: modificar tokens, posição/constraints, import de assets, preview em DPI/densidade/states, validar contraste e export. Undo, preview sem afetar tema ativo e errors por JSON pointer. Starter templates Classic Graphite/Midnight Modern/Copper originais. O gallery local é M3; registry online M4. Tema compartilhado carrega licença/autor; screenshots gerados pela aplicação.

Safe mode restaura tema builtin e ignora pacotes user sem apagar. Conflito de ID exige mostrar versão/autor, não overwrite silencioso. Unknown fields opcionais podem ser ignorados com aviso; unknown schema major rejeitado. [Contrato e exemplo](../schemas/README.md).

Perfil Android: layouts phone/tablet opcionais com fallback builtin obrigatório, tokens comuns e role mapping para Compose. Geometria desktop não é reduzida para simular touch. Tema válido desktop pode herdar mobile builtin e customizar tokens, sem esconder comandos/a11y. Layouts executáveis continuam proibidos.
