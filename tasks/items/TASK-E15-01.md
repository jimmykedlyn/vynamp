# TASK-E15-01 — Configurar Codex/Claude e Superpowers obrigatório

Versão0.7.0 · Estado planejado; infraestrutura preparada não significa aplicação/ativação externa completa.

Fase M0 / P0 · Owner engineering-delivery · Esforço inicial 2–5 dias úteis; reestimar.

## Dependências

- Nenhuma.

## Entrega concreta

Entrypoints, settings, plugin lock e bootstrap/doctor nativos.

[Spec](../../docs/engineering/01-agents-superpowers.md) · REQ-E15-01 em [matriz](../../docs/product/03-requirements.md).

## Critérios de aceite

Ambos agentes confirmam plugin ativo e skill discovery em máquina compatível; configuração sozinha não conta como instalação.

## Evidência

QA-009; registrar commit/digest, comando, ambiente/OS/ABI, outputs e limitações. Instalação de plugin/repo privado/assinatura requer ambiente real; não substituir por mocks/documentos. Quebrar em slices pequenas antes execução, implementar sucesso/erro/cancelamento, revisar pelo outro agente e preservar resultados no PR/docs/evidence.
