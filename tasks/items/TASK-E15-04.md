# TASK-E15-04 — Ativar workflows e CI Gate multiplataforma

Versão0.7.0 · Estado planejado; infraestrutura preparada não significa aplicação/ativação externa completa.

Fase M1 / P0 · Owner engineering-delivery · Esforço inicial 3–10 dias úteis; reestimar.

## Dependências

- [TASK-E15-03](TASK-E15-03.md) — Importar GitHub privado e ativar Gitflow

## Entrega concreta

CI docs/tooling e matriz Cargo/Android, required aggregate context. Entregue em Woodpecker ([ADR-010](../../docs/architecture/adr/ADR-010-woodpecker-only-ci.md)), substituído pela [ADR-014](../../docs/architecture/adr/ADR-014-public-github-actions-and-in-app-dsp.md): a TASK-E19-01 devolveu o CI ao GitHub Actions, onde macOS Intel e Apple Silicon passam a ser alvos verificados. A entrega histórica não aprovou macOS nem hardware.

[Spec](../../docs/engineering/04-ci-cd.md) · REQ-E15-04 em [matriz](../../docs/product/03-requirements.md).

## Critérios de aceite

Jobs ativos não mascaram falha; pins/actions/read permissions; skips sem fontes não aprovam player; cinco alvos cobertos.

## Evidência

QA-018, QA-021; registrar commit/digest, comando, ambiente/OS/ABI, outputs e limitações. Instalação de plugin/repo privado/assinatura requer ambiente real; não substituir por mocks/documentos. Quebrar em slices pequenas antes execução, implementar sucesso/erro/cancelamento, revisar pelo outro agente e preservar resultados no PR/docs/evidence.
