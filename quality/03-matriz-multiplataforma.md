# Matriz de validação e release multiplataforma

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Alvos e evidência independente

Windows 11 x64; Linux x64 com Ubuntu LTS/Fedora e Wayland/X11; macOS x86_64 real; macOS arm64 real; Android arm64 físico e emuladorx64 para automação. CI cross-compile não equivale a funcionamento nativo. Capturas/benchmarks registram OS/hardware/driver/ABI/build/fixture. Rodar corpus compartilhado, não alterar oráculos para fazer um alvo passar.

M0: toolchains, dependencies, native outputs, bridges e runtime/tema/a11y feasibility. M1: abrir arquivo, tocar, pause/seek/stop, fila e EQ básico em todos. M2: library/metadata/stream/podcast e permissões adaptadas. M3: packages, DSP/formats/hi-res negotiated, migrations e qualidade/release por alvo. Version1.0 agregada exige todos; só pode reduzir plataforma com autorização explícita de mudança de escopo.

## Cenários adicionais

| QA | Teste | Aceite |
|---|---|---|
| QA-018 | Linux/macOS portabilidade | Install nativo, paths/keychain/notifications/output reais; duas arquiteturas Mac sem Rosetta obrigatório |
| QA-019 | Android playback/lifecycle/focus | Notificação/lockscreen/BT; tela apagada; foco e noisy; rotation não reinicia áudio; process death permite recovery |
| QA-020 | Android arquivos/UX/energia | SAF/MediaStore/revoke/deny; TalkBack/text200%; tablet/foldable; jobs/caches e bateria sem wake lock vazando |
| QA-021 | Matriz de build/assinatura/ABI | Artifacts por alvo, Universal2/native .so, requisitos Play/NDK/pages, sandbox/portais e rollback compatível |

## Hardware e performance

Reusar desktop baseline em máquina real equivalente, publicar valores por OS. Android: aparelho arm64 de médio porte com6GiB, Androidbaseline e versão recente, alto-falante/fone/BT e DACUSB quando validado; tablet grande e emulator apenas para UI/automation. Screen-off playback2h musical e rádio; zero underrun atribuível ao app no teste controlado; visualização zero em background; restore após kill não faz autoplay proibido.

Energia: medir incremento contra baseline de reprodução de referência no mesmo aparelho, volume/rota/rede/brilho/temperatura; percent bateria sem protocolo não é gate. Definir budget M0 baseado em medidas. Android memoria idle alvo≤100MiB e playback24/96≤200MiB de processo agregado incluindo runtime/shell, com caches configurados; metas propostas a testar. Bass/DSP completo não promete mesmo CPU budget em CPU desktop e telefone.

Install tests: Windowsuser scope; DEB/MIME/uninstall e AppImage/libs/integração explícita; Flatpak extra futuro; macOS code signing/notarization e cada slice; AndroidAPK/AAB/PACKAGE update/migration, storage grants, Android Keystore e store policy atual. Não aprovar plugins externos/JIT Android sem gate de runtime/distribuição. Falta de hardware limita claim, nunca vira teste passed.
