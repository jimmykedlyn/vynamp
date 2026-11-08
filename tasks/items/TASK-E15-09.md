# TASK-E15-09 — Validar APK/AAB, SDK/NDK e identidade

Versão0.7.0 · Estado planejado; infraestrutura preparada não significa aplicação/ativação externa completa.

Fase M3 / P0 · Owner engineering-delivery · Esforço inicial 3–10 dias úteis; reestimar.

## Dependências

- [TASK-E13-09](TASK-E13-09.md) — Entregar tablet/foldable e distribuição Android
- [TASK-E11-04](TASK-E11-04.md) — Completar CI e supply chain

## Entrega concreta

Android release signing, native dependencies/page-size e Play readiness.

[Spec](../../_arquivo/repo/docs/engineering/06-installers.md) · REQ-E15-09 em [matriz](../../product/03-requisitos.md).

## Critérios de aceite

APK/AAB signer corresponde ao keystore; target SDK/NDK/ABIs validados, grants/data sobrevivem update; testar aparelho arm64.

## Evidência

QA-019, QA-020, QA-021; registrar commit/digest, comando, ambiente/OS/ABI, outputs e limitações. Instalação de plugin/repo privado/assinatura requer ambiente real; não substituir por mocks/documentos. Quebrar em slices pequenas antes execução, implementar sucesso/erro/cancelamento, revisar pelo outro agente e preservar resultados no PR/docs/evidence.

## Identidade aprovada Vynamp C

Exportar foreground/background/monochrome, launchers round/default v26/v33, fallbacks por densidade e notificação. Validar safe circle 66dp, OEM masks, themed/loja e acessibilidade. [Guia canônico](../../design/04-marca.md). Este requisito complementar não altera IDs/estado nem dispensa o contrato acima.
