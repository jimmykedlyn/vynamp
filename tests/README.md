# Organização de testes

Versão 0.7.0 · 2026-10-14 · Configuração do projeto; player ainda não implementado.

Os testes executáveis da automação estão em xtask/tests/tooling.rs e módulos unitários de xtask/src. Execute `cargo test --package xtask --locked`. Criar futuros unit/integration/fixtures/audio/ui/hardware/security/performance conforme matriz QA. tests/hardware/Cargo.toml deverá integrar harness Rust real ao workspace antes de quality workflow aceitar. Fixtures exigem licença/hash/oráculo; emulador não substitui hardware. O player ainda não foi implementado.
