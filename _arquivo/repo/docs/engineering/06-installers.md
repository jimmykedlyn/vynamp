# Instaladores: formatos, recipes e assinatura

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

[Identidade C e exports](../design/04-brand-identity.md) obrigatórios em PE/NSIS/shortcut, desktop/AppImage/DEB, ICNS no bundle Universal2 e launcher/themed/notification Android. Pré-requisito branding_exports_ready e export-manifest íntegro; PNGs de referência não satisfazem o gate.
