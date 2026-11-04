# TASK-E02-15 — Mapear lacunas web para nativo por plataforma

Versão: 0.7.2 · Data: 2026-11-03 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E02-15 / REQ-E02-15 |
| Fase / prioridade | M0 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | design — pessoa a designar |
| Esforço provisório | 2–4 dias úteis |
| Risco | médio |

## Dependências

- [TASK-E02-12](TASK-E02-12.md) — Gerar gabarito visual e estratégia de fontes
- [TASK-E02-13](TASK-E02-13.md) — Gerar gabarito de movimento, medidores e ondas
- [TASK-E02-14](TASK-E02-14.md) — Gerar gabarito de comportamento e preferências

## Entrega concreta

Mapa de cada recurso web usado pelo MVP para o equivalente nativo em Windows, Linux, macOS e Android (iOS fora da 1.0), com biblioteca própria planejada onde faltar.

Referência visual: vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452 (rev.38), caminhos `mvp/...` e `out/...` conforme [referência do MVP](../../docs/design/06-mvp-reference.md). Caminhos relevantes: `mvp/`, `out/`.

Documento canônico: [referência do MVP](../../docs/design/06-mvp-reference.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Zero lacunas sem plano; exceções só para APIs intrínsecas do navegador com comportamento reimplementado; toda biblioteca nova tem repositório git próprio e modular.

## Verificação e evidência

QA-034 em [QA comum](../../docs/quality/01-test-strategy.md). Registrar commit, comando, versão do Chrome e do resvg, hashes da fonte congelada e dos artefatos. Diferença acima da tolerância é falha, não limitação, salvo exceção de navegador registrada no [mapa de lacunas](../../docs/design/06-mvp-reference.md).

## Execução e limites

Fidelidade total ao MVP: nenhuma lacuna aceita por conveniência. A ferramenta de extração roda num navegador real porque a fonte é web; o produto nativo não depende de navegador. Biblioteca nova necessária para fechar lacuna ganha repositório git próprio e modular. Atualizar backlog JSON e requisito junto de qualquer mudança.
