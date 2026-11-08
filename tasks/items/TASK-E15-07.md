# TASK-E15-07 — Validar AppImage/DEB e dependências Linux

Versão0.7.0 · Estado planejado; infraestrutura preparada não significa aplicação/ativação externa completa.

Fase M3 / P0 · Owner engineering-delivery · Esforço inicial 3–10 dias úteis; reestimar.

## Dependências

- [TASK-E12-01](TASK-E12-01.md) — Entregar Linux nativo e MPRIS na 1.0
- [TASK-E11-04](TASK-E11-04.md) — Completar CI e supply chain

## Entrega concreta

AppDir/DEB, tool hash, libs/notices e manifesto assinado.

[Spec](../../_arquivo/repo/docs/engineering/06-installers.md) · REQ-E15-07 em [matriz](../../product/03-requisitos.md).

## Critérios de aceite

Rodar em distro baseline limpa sem libs ausentes; licenças/SBOM auditados; manifesto assinado e instalação/upgrade preservam dados.

## Evidência

QA-012, QA-018, QA-021; registrar commit/digest, comando, ambiente/OS/ABI, outputs e limitações. Instalação de plugin/repo privado/assinatura requer ambiente real; não substituir por mocks/documentos. Quebrar em slices pequenas antes execução, implementar sucesso/erro/cancelamento, revisar pelo outro agente e preservar resultados no PR/docs/evidence.

## Identidade aprovada Vynamp C

Exportar SVG C de paths sanitizado e PNGs hicolor nos tamanhos normativos; desktop Icon=vynamp, AppImage/DEB não devem incluir o antigo template Auralis. [Guia canônico](../../design/04-marca.md). Este requisito complementar não altera IDs/estado nem dispensa o contrato acima.
