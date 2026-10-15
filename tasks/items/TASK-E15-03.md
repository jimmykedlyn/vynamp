# TASK-E15-03 — Importar GitHub privado e ativar Gitflow

Versão0.7.0 · Estado planejado; infraestrutura preparada não significa aplicação/ativação externa completa.

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
