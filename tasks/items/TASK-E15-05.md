# TASK-E15-05 — Integrar testes e evidência operacional

Versão0.7.0 · Estado planejado; infraestrutura preparada não significa aplicação/ativação externa completa.

Fase M1 / P0 · Owner engineering-delivery · Esforço inicial 3–10 dias úteis; reestimar.

## Dependências

- [TASK-E11-01](TASK-E11-01.md) — Criar workspace e baseline de engenharia
- [TASK-E15-04](TASK-E15-04.md) — Ativar workflows e CI Gate multiplataforma

## Entrega concreta

Unit/integration/numerical/stream/UI/hardware/fuzz e protocolo de relatórios.

[Spec](../../engineering/05-testes.md) · REQ-E15-05 em [matriz](../../product/03-requisitos.md).

## Critérios de aceite

Tests reais/comandos/oráculos por risco; fixture licence/hash; hardware ausente não passa; plugin/installer validations separados de tooling.

## Evidência

QA-001, QA-002, QA-009, QA-021; registrar commit/digest, comando, ambiente/OS/ABI, outputs e limitações. Instalação de plugin/repo privado/assinatura requer ambiente real; não substituir por mocks/documentos. Quebrar em slices pequenas antes execução, implementar sucesso/erro/cancelamento, revisar pelo outro agente e preservar resultados no PR/docs/evidence.
