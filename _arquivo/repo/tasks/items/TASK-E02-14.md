# TASK-E02-14 — Gerar gabarito de comportamento e preferências

Versão: 0.7.2 · Data: 2026-11-03 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E02-14 / REQ-E02-14 |
| Fase / prioridade | M0 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | design — pessoa a designar |
| Esforço provisório | 2–5 dias úteis |
| Risco | médio |

## Dependências

- [TASK-E02-11](TASK-E02-11.md) — Congelar MVP de referência e extrair SVGs, ícones e temas

## Entrega concreta

Roteiros de interação por teclado, ponteiro e toque com estado esperado e esquema das preferências persistidas com defaults e migração.

Referência visual: vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452 (rev.38), caminhos `mvp/...` e `out/...` conforme [referência do MVP](../../design/07-pin-mvp.md). Caminhos relevantes: `mvp/`.

Documento canônico: [referência do MVP](../../design/07-pin-mvp.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Cada controle do MVP tem roteiro com resultado verificável; o esquema cobre todas as chaves lidas ou escritas; nenhuma ação fica sem roteiro.

## Verificação e evidência

QA-034 em [QA comum](../../quality/01-estrategia-testes.md). Registrar commit, comando, versão do Chrome e do resvg, hashes da fonte congelada e dos artefatos. Diferença acima da tolerância é falha, não limitação, salvo exceção de navegador registrada no [mapa de lacunas](../../design/07-pin-mvp.md).

## Execução e limites

Fidelidade total ao MVP: nenhuma lacuna aceita por conveniência. A ferramenta de extração roda num navegador real porque a fonte é web; o produto nativo não depende de navegador. Biblioteca nova necessária para fechar lacuna ganha repositório git próprio e modular. Atualizar backlog JSON e requisito junto de qualquer mudança.
