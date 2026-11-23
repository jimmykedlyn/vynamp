# Instaladores: formatos, recipes e assinatura

## Visão geral

Define os formatos de instalação, receitas de build e requisitos de assinatura para o projeto Vynamp nas plataformas suportadas.

## Plataformas suportadas

- Windows x86_64 — NSIS e .zip portable
- Linux x86_64 — AppImage e .deb
- macOS Intel x86_64 e Apple Silicon arm64 — DMG e .zip Universal2
- Android arm64-v8a — APK e AAB
- iOS arm64 — .ipa

## Receitas

- Windows: NSIS + .zip, sem admin para playback, dados preservados, binários e instalador assinados, manifesto e hash corretos.
- Linux: AppImage/.deb em distro baseline limpa sem libs ausentes, licenças/SBOM auditados, manifesto assinado, instalação/upgrade preserva dados.
- macOS: DMG/.zip Universal2, Gatekeeper/notary/staple/upgrade testados, helpers/libs suportam arquitetura prometida.
- Android: APK/AAB com signer correspondente ao keystore, target SDK/NDK/ABIs validados, grants/data sobrevivem update.
- iOS: provisionamento e assinatura correspondente ao keystore.

## Assinatura

- Binários e instaladores devem ser assinados; manifesto/hash verificados.
- Nunca publicar stubs ou unstable sem assinatura.
- Manteredor aprova publicação.

## Critérios de aceitação

- Install/upgrade/uninstall sem admin (Windows) ou usuário padrão (Linux/macOS/Android).
- Dados preservados após atualização.
- Manifesto e hash corretos.
- Nunca publicar sem assinatura explícita.
- Produto público somente após varredura de segredos e ordem explícita "pode publicar".

## Histórico

- **2026-11-23**: Migração de specs do arquivo removido (_arquivo/) para a estrutura consolidada do engineering/.