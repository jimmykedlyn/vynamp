# Host Android

Versão 0.7.0 · 2026-10-14 · Configuração do projeto; player ainda não implementado.

Implementar app Gradle com wrapper, módulo app e bridge Rust nativa na TASK-E13. Package outputs app/build/outputs/apk/release/*.apk e bundle/release/*.aab. Aplicar signing template após AGP pinado e somente para builds release com secrets, evitando quebrar debug sem credentials. NDK/SDK/build-tools/toolchain em config/toolchains.json; build Rust cdylib integrado ao Gradle, sem biblioteca fictícia. Core/media/service/focus/SAF conforme specAndroid.
