# Project Tooling Implementation Plan

Versão 0.4.0 · 2026-10-14 · Configuração do projeto; player ainda não implementado.

> For agentic workers: REQUIRED SUB-SKILL: use superpowers:executing-plans task-by-task; subagent-driven-development quando autorizado. Respeitar autorização já dada para preparar configuração, sem pedir aprovação repetida. Não mergear/publicar fora do escopo.

**Goal:** configurar colaboração dos dois agentes, isolamento, Gitflow e entrega multiplataforma com gates honestos.

**Architecture:** automação de responsabilidade pequena (migração atual em Rust/xtask) e workflows declarativos; stages distinguem spec/tooling de app implementado. Native plugin manager verifica Superpowers real. Claims no Git common dir impedem owners duplicados locais.

**Tech Stack:** Rust/xtask, Git/gh, GitHub Actions, NSIS/SignTool, appimagetool/dpkg, Apple codesign/notary/lipo e AndroidGradle/signing. Toolchains app ficam M0.

**Spec:** [engenharia](../../engineering/01-agents-superpowers.md), [Gitflow](../../engineering/03-gitflow-github.md), [CI](../../engineering/04-ci-cd.md).

## Global Constraints

Windows/Linux/MacIntel/MacSilicon/Android obrigatórios1.0; Superpowers obrigatório; um owner porTASK; código sem secrets; main/develop porPR; pacote atual sem player e sem assinatura real fornecida. Preservar repositório privado existente.

## Review Focus

Owners duplicados no common dir; paths/slugs maliciosos; worktree dirty/unmerged; plugin declarado mas não instalado; release sem todos os alvos/hardware/artifacts. Tests tooling cobrem Git/gate; discovery/plugin e assinatura exigem ambiente real.

## Tarefas

- [x] Testes falhando de init/isolation/claims/cleanup antes cargo xtask repo; implementar e passar xtask/tests/tooling.rs.
- [x] Testes falhando de gate/artifacts antes do gate de release; implementar bloqueios e passar testes em xtask/src/release.rs.
- [x] Configurar entrypoints AGENTS/CLAUDE, configs e native plugin bootstrap; instalação local permanece externa e não é marcada concluída.
- [x] Criar CI/release/quality e scripts de package/signing com fail-closed gates; assinaturas/instaladores reais permanecem dependentes do app.
- [x] Validar pacote, YAML/JSON/links/backlog e todos tooling tests; gerar evidência de preparação.
- [ ] Importar repo privado por branch/PR em máquina autenticada; integrar e configurar protections/environments, sem force push.

Interfaces: repo.py init/worktree/remove/protect, agents.py install/doctor, ci.py docs/detect/core/android, release.py preflight/digest/build/assemble/publish-draft. Commands de execução e resultados esperados em START-HERE e docs de engenharia. Não transformar tarefas de infraestrutura pendentes em testes passed do player.

Plano original da preparação 0.3.0, substituído na automação pelo [plano Rust](2026-10-14-rust-tooling.md). Checkmarks anteriores registram tooling original, não aprovação do player.
