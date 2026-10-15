# Rust Tooling Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [x]`) syntax for tracking.

**Goal:** Substituir Python por Rust na automação entregue.
**Architecture:** Workspace com crate xtask modular. Operações Git/CI/assinatura via subprocessos tipados, parsing por crates, gates sem dependência de código do player.
**Tech Stack:** Rust/rustup/Cargo, clap, serde, jsonschema, serde_yaml, quick-xml, sha2, tempfile, walkdir, zip.
**Spec:** [desenho](../specs/2026-10-14-rust-tooling-design.md).

## Global Constraints

Windows/Linux/macOS Intel/macOS Silicon/Android permanecem obrigatórios. application_ready e release_enabled permanecem false. Sem Python em comandos ativos. Sem forçar Git, inventar evidências ou divulgar segredos. Executar inline conforme o trabalho autorizado; distribuição sem .git preservada.

## Review Focus

- Workspace apenas xtask não ativa CI do player: teste de detecção.
- Paths com espaços e traversal: testes de slug, claims e relatórios.
- Hotfix requer ambos históricos: teste de remoção após integração só em develop.
- Plugins com shape desconhecido não contam como ativos: teste de discovery.
- Manifest completo e evidência não dispensam hash e tamanho: testes de artefatos/digest.

### Task 1: CLI, Git e discovery

**Files:** Cargo.toml, Cargo.lock, rust-toolchain.toml, .cargo/config.toml, xtask/src/{main,util,repo,agents}.rs, xtask/tests/tooling.rs.
**Interfaces:** Context(root), run/capture com argumentos separados; repo::init/worktree/remove; agents::doctor.
- [x] Escrever testes Git/discovery e executar observando falha antes da implementação.
- [x] Implementar CLI e operações preservando claims/gates; executar testes, fmt e clippy.

### Task 2: Validação e CI

**Files:** xtask/src/{validate,ci}.rs e testes.
**Interfaces:** validate::spec/delivery; ci::detect/docs/core/android/quality.
- [x] Testar schemas inválidos, DAG/código ausente e produzir falha real.
- [x] Implementar schemas Draft2020-12, links/DAG/OpenAPI/XML/contraste e CI.
- [x] Executar validação da distribuição e regressões.

### Task 3: Import, instaladores e releases

**Files:** xtask/src/{github,release,package,signing}.rs e testes.
**Interfaces:** release::preflight/source_digest/artifact_manifest; package::build; github::import.
- [x] Testar release bloqueada, artefatos inválidos e digest alterado.
- [x] Implementar importação privada sem descartar histórico, assinaturas efêmeras e receitas nativas.
- [x] Verificar compilação; registrar ausência de certificados/hardware/execução remota.

### Task 4: Migração completa da entrega

**Files:** hooks, workflows, dependabot, configs, START-HERE, documentação/index, VALIDATION/ARTIFACTS/evidência.
- [x] Remover Python e atualizar comandos, estrutura e CI sem ativar player.
- [x] Executar fmt, clippy, testes e docs; conferir links e gates negativos.
- [x] Empacotar v0.4.0 com inventário e SHA256; entregar próximos comandos.

## Execução e limitações

Implementação inline em cópia isolada de distribuição sem .git; nenhum histórico remoto criado/alterado. Testes locais em Linux: 23. Revisão independente Codex encontrou e confirmou correções de LF, credenciais sintéticas, overlay legado, membership do app e symlinks. GitHub/Claude/plugin installation/hardware/assinatura/app não executados. Evidência detalhada em docs/evidence/rust-tooling.json.
