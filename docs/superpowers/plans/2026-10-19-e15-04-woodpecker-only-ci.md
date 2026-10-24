# E15-04 — CI/CD somente Woodpecker (adendo ao plano 2026-10-18)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans. Steps use checkbox (`- [ ]`) syntax.

**Goal:** tornar o Woodpecker o único CI/CD do repositório, com status `CI Gate` publicado por ele, e remover os workflows GitHub Actions pagos.

**Architecture:** workflows `.woodpecker/{linux,windows,android,gate,release}.yaml` nos agentes existentes; o servidor formata o contexto do workflow `gate` como `CI Gate`; `cargo xtask check-delivery` passa a validar `.woodpecker/` e a recusar `.github/workflows`.

**Tech Stack:** Woodpecker 3.19.0, Rust 1.99.0/xtask, PowerShell 7.

**Spec:** decisão do mantenedor em 2026-10-19 (CI/CD só pelo Woodpecker, sem GitHub Actions pago; macOS sem agente fica "não verificado"; migração no PR #16; workflows removidos) registrada em [ADR-010](../../architecture/adr/ADR-010-woodpecker-only-ci.md); desenho base em [spec E15-04](../specs/2026-10-18-e15-04-woodpecker-cloudflare-design.md).

## Global Constraints

- Imagens de steps Docker pinadas por tag + `@sha256:`; Windows Local usa `image: pwsh`.
- Nenhum step com `privileged`, `volumes` ou `network_mode`; repo continua não trusted.
- Contexto obrigatório continua `CI Gate` (`config/project.json`, `repo protect`).
- macOS Intel/Silicon: não verificado; gate imprime isso e nunca alega aprovação macOS.
- Release fail-closed: sem agente Mac nem transporte de artefatos entre agentes, a pipeline de tag termina em falha após o preflight.
- Sem alterar plano/gastos/configurações do GitHub; sem merge/release/force push.

## Review Focus

- Falha em linux, windows ou android deve impedir `CI Gate` success (gate não roda → nunca success).
- Imagem sem digest ou workflow GitHub reintroduzido deve falhar `check-delivery`.
- Tag `v*` não pode produzir pipeline verde enquanto build/publicação distribuídos não existirem.
- `evaluate`/eventos: PR de fork continua exigindo aprovação.
- Status antigos `woodpecker-pilot/*` deixam de ser publicados para novos commits.

---

### Task 1: validação de CI no xtask

**Files:** Modify `xtask/src/validate.rs`, `xtask/src/release.rs`.

- [x] Testes `woodpecker_config_*` em `validate.rs` (fixture tempdir): config válida passa; imagem sem digest, `.github/workflows` presente, gate sem dependência android e step `privileged` falham. Rodar: FAIL (função ausente).
- [x] Implementar `fn woodpecker(root: &Path, c: &mut Checks) -> Result<usize>` e chamá-la em `delivery`. Rodar: PASS.
- [x] Testes `version_from_tag` e `woodpecker_tag_check` em `release.rs`; implementar; `tag-version` lê `CI_COMMIT_TAG` e imprime a versão; preflight verifica tag quando `CI=woodpecker`.
- [x] `cargo test --package xtask --locked` verde; commit.

### Task 2: workflows e servidor

**Files:** Create `.woodpecker/{android,gate,release}.yaml`; modify `.woodpecker/{linux,windows}.yaml`, `infra/ci/woodpecker/compose.yaml`; delete `.github/workflows/*`, entrada `github-actions` de `.github/dependabot.yml`.

- [x] `WOODPECKER_STATUS_CONTEXT=woodpecker` e `WOODPECKER_STATUS_CONTEXT_FORMAT` que publica `CI Gate` para o workflow `gate` e `woodpecker/<workflow>` para os demais.
- [x] `woodpecker-cli lint` nos cinco arquivos; `cargo xtask check-delivery` exit 0; preflight `-SelfTest` 9/9.
- [x] Recriar o servidor (`compose up -d`), dados preservados; commit e push.

### Task 3: docs e evidência real

- [x] ADR-010; atualizar 04-ci-cd, 03-gitflow, ADR-009 (nota), AGENTS.md, PR template, TASK-E15-04, README infra, config/github.json, INDEX.
- [x] Evidência real: push success com `CI Gate` success no GitHub; branch descartável com falha → `CI Gate` não success; tag não é criada (release só verificado por lint/preflight local). Resultado: #9 push e #10 PR de `71f7086` success com `CI Gate` success; #11 `fda34b0` failure com `CI Gate` pending/skipped ([evidência](../../evidence/PR/task-e15-04/README.md)).
- [x] Atualizar PR #16 e bloco REVISAR do guia.
