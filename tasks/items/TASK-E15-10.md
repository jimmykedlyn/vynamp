# TASK-E15-10 — Fechar release assinado e draft privado

Versão0.7.0 · Estado planejado; infraestrutura preparada não significa aplicação/ativação externa completa.

Fase M3 / P0 · Owner engineering-delivery · Esforço inicial 3–10 dias úteis; reestimar.

## Dependências

- [TASK-E15-06](TASK-E15-06.md) — Validar NSIS/portable e assinatura Windows
- [TASK-E15-07](TASK-E15-07.md) — Validar AppImage/DEB e dependências Linux
- [TASK-E15-08](TASK-E15-08.md) — Validar DMG/ZIP Universal2 e notarização
- [TASK-E15-09](TASK-E15-09.md) — Validar APK/AAB, SDK/NDK e identidade
- [TASK-E10-10](TASK-E10-10.md) — Fechar gates e cobertura desktop

## Entrega concreta

Evidence digest, gates, manifest minisign, secrets/environments e draft.

[Spec](../../docs/engineering/07-secrets-activation.md) · REQ-E15-10 em [matriz](../../docs/product/03-requirements.md).

## Critérios de aceite

Conjunto completo de artifacts reais/assinados; source digest e relatórios revisados; nunca publicar stubs/unsigned stable; mantenedor aprova publicação.

## Evidência

QA-012, QA-021; registrar commit/digest, comando, ambiente/OS/ABI, outputs e limitações. Instalação de plugin/repo privado/assinatura requer ambiente real; não substituir por mocks/documentos. Quebrar em slices pequenas antes execução, implementar sucesso/erro/cancelamento, revisar pelo outro agente e preservar resultados no PR/docs/evidence.
