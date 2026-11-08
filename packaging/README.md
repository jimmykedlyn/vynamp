# Packaging e gates

Versão 0.7.0 · 2026-10-14 · Configuração do projeto; player ainda não implementado.

Windows NSIS+portable, Linux AppImage+DEB, MacUniversal2 DMG+ZIP, AndroidAPK+AAB. xtask/src/package.rs usa recipes locais e fail-closed preflight. Arquivos são templates/contratos, não prova de installer gerado. Runtime inventory e tools lock precisam ser completados com outputs reais. update.pub conterá chave pública de manifest signing; chave privada nunca entra aqui. Ver _arquivo/repo/docs/engineering/06-installers.md.
