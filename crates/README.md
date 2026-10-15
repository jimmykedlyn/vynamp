# Workspace Rust: contrato de implementação

Versão 0.7.0 · 2026-10-14 · Configuração do projeto; player ainda não implementado.

O Cargo workspace raiz, toolchain 1.99.0 e Cargo.lock já existem para xtask. TASK-E11-01 deve adicionar crates/apps reais e resolver lockfile após provas M0. Não duplicar nem remover xtask. CI do player exige apps/desktop/Cargo.toml com package vynamp-desktop e src/main.rs reais; Android exige wrappers Unix/Windows e app/build.gradle.kts. Somente o workspace da automação não ativa esses jobs. Núcleo atende cinco targets; bridge Android adapta e não duplica queue/DSP.
