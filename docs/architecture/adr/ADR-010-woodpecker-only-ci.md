# ADR-010 — CI/CD somente pelo Woodpecker

> **Substituída pela [ADR-014](ADR-014-public-github-actions-and-in-app-dsp.md) em 2026-11-05.** O mantenedor tornou os repositórios públicos, o que torna os runners hospedados do GitHub gratuitos e devolve a branch protection ao plano gratuito — as duas razões que sustentavam esta ADR. O texto abaixo é preservado como histórico da decisão; não é mais normativo.

Data: 2026-10-19. Estado: substituída pela ADR-014.

## Contexto

O GitHub Actions do repositório privado está bloqueado por billing desde o PR15; nenhum job inicia. O piloto Woodpecker da TASK-E15-04 provou, com webhooks reais, sucesso/falha/cancelamento/agente ausente nos agentes Linux (Docker) e Windows (Local). Em 2026-10-19 o mantenedor decidiu: CI/CD só pelo Woodpecker, sem GitHub Actions pago; macOS fica sem agente por enquanto; migração no PR #16; workflows Actions removidos.

## Decisão

- `.github/workflows` é removido (conteúdo preservado no histórico Git) e `cargo xtask check-delivery` falha se ele voltar. Dependabot mantém só `cargo`.
- Workflows em `.woodpecker/`: `linux` (docs, clippy, testes, core `x86_64-unknown-linux-gnu`), `windows` (idem, `x86_64-pc-windows-msvc`), `android` (agente Linux; sem fontes Android só reporta ausência), `gate` e `release`.
- O servidor publica o workflow `gate` como status GitHub `CI Gate`, o contexto exigido por `config/project.json` e `repo protect`; os demais aparecem como `woodpecker/<evento>/<workflow>`. `gate` depende de `linux`, `windows` e `android` e só roda se os três tiverem sucesso: falha, cancelamento ou agente ausente nunca produzem `CI Gate` success.
- macOS Intel e Apple Silicon: **não verificados** até existir agente Mac. O gate imprime isso; ele não aprova macOS, e o critério "cinco alvos cobertos" da TASK-E15-04 continua parcial.
- Release em tag `v*`: `cargo xtask release preflight` (tag conferida via `CI_COMMIT_TAG`) seguido de um step que falha de propósito. Build/assinatura/publicação distribuídos (agente Mac, transporte de artefatos entre agentes, secrets de signing no Woodpecker) não foram migrados; `release assemble` ainda depende de artefatos do Actions e não é usado.
- Hardware/quality: sem workflow; `cargo xtask quality-gate` roda manualmente nas máquinas de hardware até a TASK-E15-05 registrar agentes de hardware.
- Segurança mantida do desenho E15-04: túnel só HTTP, gRPC local, `WOODPECKER_OPEN=false`, repo não trusted, PR de fork exige aprovação, imagens Docker pinadas por digest, sem `privileged`/`volumes` em steps.

## Consequências

- Não há custo de minutos GitHub; a disponibilidade do CI depende do notebook (Docker Desktop + agente Windows ligados, ver [operação](../../../infra/ci/woodpecker/README.md)).
- Branch protection continua indisponível no plano atual (HTTP 403); `CI Gate` é publicado, mas não obrigatório até o mantenedor resolver o plano.
- Workflow `gate` pulado (dependência falhou/cancelada) pode deixar o status `CI Gate` em `pending`; isso nunca é success.
- Reverter para Actions exige novo ADR e alteração de `check-delivery`.
