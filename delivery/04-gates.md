# Manifest de capacidades e gates de distribuição

Versão 0.7.0 · Preparação documental; aplicativo ainda não implementado.

## Artefatos obrigatórios na 1.0

| Plataforma | Artefatos |
|---|---|
| Windows x64 | NSIS EXE e portable ZIP |
| Linux x64 | AppImage e DEB |
| macOS | DMG e ZIP Universal2, ambos com Intel e Silicon nativos |
| Android arm64 | APK e AAB |

São oito artefatos obrigatórios e cinco alvos de teste. MSIX, Flatpak, PKG e pacotes Mac separados são extras futuros; não substituem os obrigatórios. Stores têm assinatura e atualização próprias. Cada Mac exige relatório nativo independente; emulator não substitui Android físico.

## Manifest por artefato

Gerar product/version/platform/arch/minOS, sourceDigest, backend/features/configuração, inventário nativo, licenças/notices, capabilities de codecs, integração OS, visualizers, DSP e saída. Vincular evidências, fixtures e hashes ao artefato efetivamente distribuído. Catálogo planejado não alimenta declarações OS diretamente. Associação exige binding/parser/demux/decode verificados, disponíveis e licenciados; import de playlist usa capability própria. Liste desconhecido/indisponível com motivo. Um decoder listado pelo FFmpeg ou feature all-codecs não é evidência de playback.

Assinaturas, revogação, staging, health checks, rollback e compatibilidade de DB seguem os gates existentes. Installer/updater auditam bibliotecas realmente incluídas. Branding nativo e declarações de arquivo precisam de hashes e provas; não liberar flags manualmente para passar.

## Portable e concorrência

Um owner pelo armazenamento canônico do perfil, mesmo entre sessões, aliases, instalado e portable; updater respeita lock e não encerra reprodução/gravação sem escolha do usuário. Perfis distintos permitem instâncias explícitas. Portable/AppImage só registram integração por ação explícita. Uninstall remove apenas registros ainda pertencentes à instalação e preserva dados/defaults de terceiros. [Contrato de ativação](../platforms/04-ativacao-ownership.md).

## Gates adicionais M3

TASK-E16-07/08 e TASK-E17-12/20 integram TASK-E14-06. Formatos required comuns exigem verificação nos cinco alvos; conditional exige spike e limitação documentada, sem alegação de suporte quando indisponível. QA-022–031 cobre ativação, grants, trim/DRC/canais, sinal final, visualizers e proxy. Recursos M4/M5 não bloqueiam os requisitos comuns M3.

Documento, schema ou teste de tooling não aprova áudio ou integração nativa. application_ready, release_enabled e branding_exports_ready permanecem false neste pacote. Não gerar executáveis, instaladores ou ícones nativos fictícios.

## Instaladores: formatos, recipes e assinatura

Versão 0.7.0 · 2026-10-14 · Configuração do projeto; player ainda não implementado.

Receitas em xtask/src/package.rs e packaging/. Só rodam quando preflight aprova app real/licença/toolchain/evidências. Esta entrega não contém executável, app Android ou instalador compilado. Atualização0.3 fixa NSIS como installer Windows preferido; MSIX/Flatpak opcionais futuros não bloqueiam os formatos definidos abaixo.

| Plataforma | Saídas1.0 | Construção / validação |
|---|---|---|
| Windowsx64 | EXE NSIS user scope + ZIP portable | Cargo→runtime audit→SignTool bin/DLL→makensis→SignTool installer |
| Linuxx64 | AppImage + DEB | AppDir/desktop/icon/libs auditadas→appimagetool por URL/hash pin→dpkg-deb |
| macOSIntel+Silicon | DMG + ZIP Universal2 | build x64/arm64→lipo verify→bundle→codesign inside-out→notarize/staple→DMG/ZIP |
| Androidarm64 | APK + AAB | Gradle/Rust bridge com SDK/NDK pins→release signing→apksigner/jarsigner |

Nome `vynamp-<version>-<platform>-<type>.<ext>`; conjunto completo exigido em release.py, não publicar só Windows e chamar multiplataforma. Mac helpers/libs também devem ser Universal2 ou resolved por architecture, não apenas main binary. Android signing.gradle.kts é template a aplicar no app implementado, evitando senha em arquivos Gradle. Não trata AAB como APK instalável diretamente.

packaging/runtime-files.json deve mapear libs/helpers reais e passar audit_complete. Linux dependencies/control precisam fechar dependências transientes reais, não apenas libasound; AppImage não é instalação de system services. Windows dados ficam fora INSTDIR; uninstall não apaga library/settings/downloads. IconSVG original inclui provisório, branding/licença final antes distribuir.

Linux usa distribuição com checksums dentro de manifesto assinado (minisign), não inventa assinatura Authenticode/Apple. Todos artifacts recebem SHA256; manifest.json recebe assinatura e chave pública pinned no app/updater. Windowsbin/installer, Macapp/DMG e AndroidAPK/AAB têm assinaturas da plataforma além desse manifesto. Certificates/keys importados temporariamente por signing.py e removidos ao final.

Disponibilizar SBOM/notices/símbolos separados, release notes e evidência antes publicação. package builders ainda requerem toolchain/application e licenças reais; testes deste pacote validam bloqueios/contratos, não concluem installer smoke/signing tests.

[SignTool](https://learn.microsoft.com/en-us/windows/win32/seccrypto/signtool), [notarização Apple](https://developer.apple.com/documentation/security/notarizing-macos-software-before-distribution).

## Ícones Vynamp

[Identidade C e exports](../design/04-marca.md) obrigatórios em PE/NSIS/shortcut, desktop/AppImage/DEB, ICNS no bundle Universal2 e launcher/themed/notification Android. Pré-requisito branding_exports_ready e export-manifest íntegro; PNGs de referência não satisfazem o gate.
