# TASK-E15-03 — Importar GitHub privado e ativar Gitflow

Versão0.7.0 · Atualização 2026-10-18 · Estado: em execução; privacidade, histórico e configurações do repositório verificados; proteções de branch pendentes por plano GitHub, sem aceite antecipado.

Fase M0 / P0 · Owner engineering-delivery · Esforço inicial 2–5 dias úteis; reestimar.

## Dependências

- [TASK-E15-02](TASK-E15-02.md) — Validar worktrees, claims e handoff

## Entrega concreta

Importação por branch/PR ou bootstrap vazio, proteções e default develop.

[Spec](../../docs/engineering/03-gitflow-github.md) · REQ-E15-03 em [matriz](../../docs/product/03-requirements.md).

## Critérios de aceite

Privacidade real confirmada; histórico preservado; sem force push; CI Gate/proteções realmente aplicados ou pendência registrada.

## Evidência

QA-012; registrar commit/digest, comando, ambiente/OS/ABI, outputs e limitações. Instalação de plugin/repo privado/assinatura requer ambiente real; não substituir por mocks/documentos. Quebrar em slices pequenas antes execução, implementar sucesso/erro/cancelamento, revisar pelo outro agente e preservar resultados no PR/docs/evidence.

## Execução E15-03

[Plano](../../docs/superpowers/plans/2026-10-18-e15-03-gitflow.md) e [evidência do autor](../../docs/evidence/PR/task-e15-03/README.md). Histórico importado pela E15-01 preservado, sem reimportação nem force push. Aplicados em 2026-10-18: default `develop`, somente merge commit, exclusão de branch após merge e Dependabot mirando `develop`. Branch protection/rulesets retornam HTTP 403 no plano atual do repositório privado: CI Gate obrigatório e bloqueio de force push/deleção **não** estão aplicados e permanecem pendência do mantenedor. Revisão Codex do SHA exato, CI e merge pelo mantenedor são gates separados.
