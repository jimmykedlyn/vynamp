# TASK-E15-02 — Validar worktrees, claims e handoff

Versão0.7.0 · Estado planejado; infraestrutura preparada não significa aplicação/ativação externa completa.

Fase M0 / P0 · Owner engineering-delivery · Esforço inicial 2–5 dias úteis; reestimar.

## Dependências

- [TASK-E15-01](TASK-E15-01.md) — Configurar Claude Code e Superpowers obrigatório

## Entrega concreta

Scripts e regras de isolamento, claim atômico e remoção segura.

[Spec](../../docs/engineering/02-worktrees.md) · REQ-E15-02 em [matriz](../../docs/product/03-requirements.md).

## Critérios de aceite

Índices independentes, task exclusiva, paths válidos, worktree suja/unmerged preservada; hotfix só remove após main e develop.

## Evidência

QA-009, QA-012; registrar commit/digest, comando, ambiente/OS/ABI, outputs e limitações. Instalação de plugin/repo privado/assinatura requer ambiente real; não substituir por mocks/documentos. Quebrar em slices pequenas antes execução, implementar sucesso/erro/cancelamento, revisar pelo outro agente e preservar resultados no PR/docs/evidence.

## Execução E15-02

[Plano em slices](../../_arquivo/repo/docs/superpowers/plans/2026-10-16-e15-02-isolation.md) e [evidência do autor](../../docs/evidence/PR/task-e15-02/README.md). A validação local do tooling não constitui aceite da TASK: revisão Claude do SHA exato, CI remoto e integração pelo mantenedor são etapas separadas; produto/hardware/release continuam fora deste aceite.
