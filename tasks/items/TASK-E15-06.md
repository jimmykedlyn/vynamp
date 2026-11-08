# TASK-E15-06 — Validar NSIS/portable e assinatura Windows

Versão0.7.0 · Estado planejado; infraestrutura preparada não significa aplicação/ativação externa completa.

Fase M3 / P0 · Owner engineering-delivery · Esforço inicial 3–10 dias úteis; reestimar.

## Dependências

- [TASK-E01-10](TASK-E01-10.md) — Endurecer engine e documentação dos limites
- [TASK-E11-04](TASK-E11-04.md) — Completar CI e supply chain

## Entrega concreta

EXE user scope + ZIP, runtime inventory e SignTool.

[Spec](../../_arquivo/repo/docs/engineering/06-installers.md) · REQ-E15-06 em [matriz](../../product/03-requisitos.md).

## Critérios de aceite

Install/upgrade/uninstall sem admin para playback; dados preservados; binaries/installer assinados, manifesto/hash correto.

## Evidência

QA-012, QA-021; registrar commit/digest, comando, ambiente/OS/ABI, outputs e limitações. Instalação de plugin/repo privado/assinatura requer ambiente real; não substituir por mocks/documentos. Quebrar em slices pequenas antes execução, implementar sucesso/erro/cancelamento, revisar pelo outro agente e preservar resultados no PR/docs/evidence.

## Identidade aprovada Vynamp C

Exportar/inspecionar ICO multi-size C, embutir no executável e NSIS, shortcuts/taskbar/tray e DPI; assinar executável/installer reais. branding_exports_ready exige exports/report/hash verificados. [Guia canônico](../../design/04-marca.md). Este requisito complementar não altera IDs/estado nem dispensa o contrato acima.
