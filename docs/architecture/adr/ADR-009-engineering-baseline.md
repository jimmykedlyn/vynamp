# ADR-009 — workspace mínimo e baseline de engenharia

Data: 2026-10-18. Estado: registro do baseline existente; revisão da TASK-E11-01 pendente.

## Contexto

A [TASK-E11-01](../../../tasks/items/TASK-E11-01.md) exige repositório independente, workspace mínimo, toolchain pin, ADRs, CI inicial e paths de fixtures. O bootstrap e a E15-02 já integraram workspace, lockfile resolvido e CI de tooling. Recriá-los duplicaria a infraestrutura e poderia sugerir que existe um player.

## Decisão

Reutilizar `Cargo.toml` com membro/default-member `xtask`, resolver 3 e `Cargo.lock` versionado. Rust 1.99.0 em `rust-toolchain.toml` é o pin existente; `config/toolchains.json` deve continuar refletindo esse pin. O comando de baseline é `cargo build --workspace --locked`; `cargo xtask ci docs` valida docs/config, fmt, clippy e testes reais de tooling. A resolução de dependências é a do Cargo.lock existente, sem lockfile escrito manualmente ou atualização incidental.

O workspace ainda não inclui crates/apps de áudio/UI. Os diretórios com README de contratos não são membros compiláveis. A adição de código real ocorre nas respectivas tarefas, incluindo-o explicitamente no workspace e ativando seus gates; não criar executáveis placeholder para tornar um job verde.

O CI inicial executava tooling em Windows, Linux, macOS Intel e Apple Silicon no GitHub Actions, com `CI Gate` agregado; desde 2026-10-19 o CI é somente Woodpecker ([ADR-010](ADR-010-woodpecker-only-ci.md)), com macOS não verificado. Core/Android permanecem condicionais à existência de app real. Pin/ABI/SDK/NDK, hardware, licença final, inventário runtime, assinatura e release exigem seus gates próprios; build de xtask não os aprova. Proteções GitHub/default branch pertencem à TASK-E15-03.

Fixtures persistentes usam [tests/fixtures](../../../tests/fixtures/README.md). Fixtures Git do tooling continuam temporárias/isoladas em `xtask/tests/tooling.rs`. O contrato exige origem, licença/permissão, SHA-256 e oráculo independente antes de introduzir corpus; esta decisão não adiciona corpus nem aprova codecs.

## Consequências e verificação

- [ADR-001](ADR-001-independent-product.md) mantém o produto independente, sem código/assets Winamp. Esta entrega adiciona documentação, sem código/assets de terceiros; não é auditoria legal de toda dependência transitiva.
- `cargo metadata --locked --no-deps --format-version 1` comprova membros/targets e `cargo xtask ci detect` distingue tooling de aplicativo.
- Lockfile/pins/CI existentes permanecem intactos; novos membros/dependências precisam de mudança revisada nas tarefas correspondentes.
- [Evidência E11-01](../../evidence/PR/task-e11-01/README.md) registra ambiente, comandos, resultados e limites; aceite depende de revisão do SHA exato, CI e integração pelo mantenedor.
