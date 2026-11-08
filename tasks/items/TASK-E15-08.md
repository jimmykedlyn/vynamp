# TASK-E15-08 — Validar DMG/ZIP Universal2 e notarização

Versão0.7.0 · Estado planejado; infraestrutura preparada não significa aplicação/ativação externa completa.

Fase M3 / P0 · Owner engineering-delivery · Esforço inicial 3–10 dias úteis; reestimar.

## Dependências

- [TASK-E12-02](TASK-E12-02.md) — Entregar macOS Intel e Apple Silicon nativos na 1.0
- [TASK-E11-04](TASK-E11-04.md) — Completar CI e supply chain

## Entrega concreta

Slices Intel/Silicon, runtime inside-out signing e notary/staple.

[Spec](../../_arquivo/repo/docs/engineering/06-installers.md) · REQ-E15-08 em [matriz](../../product/03-requisitos.md).

## Critérios de aceite

Executar nativamente em ambos Macs; Gatekeeper/notary/staple/upgrade testados; helpers/libs suportam arquitetura prometida.

## Evidência

QA-012, QA-018, QA-021; registrar commit/digest, comando, ambiente/OS/ABI, outputs e limitações. Instalação de plugin/repo privado/assinatura requer ambiente real; não substituir por mocks/documentos. Quebrar em slices pequenas antes execução, implementar sucesso/erro/cancelamento, revisar pelo outro agente e preservar resultados no PR/docs/evidence.

## Identidade aprovada Vynamp C

Usar Vynamp.icns único para Intel/Silicon/Universal2, todos os papéis 1x/2x; Info.plist e Resources corretos, template menu bar e validação em ambas arquiteturas. [Guia canônico](../../design/04-marca.md). Este requisito complementar não altera IDs/estado nem dispensa o contrato acima.
