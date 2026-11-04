# TASK-E02-16 — Implementar harness de fidelidade com egui_kittest

Versão: 0.7.2 · Data: 2026-11-03 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E02-16 / REQ-E02-16 |
| Fase / prioridade | M1 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | design — pessoa a designar |
| Esforço provisório | 4–8 dias úteis |
| Risco | médio |

## Dependências

- [TASK-E02-15](TASK-E02-15.md) — Mapear lacunas web para nativo por plataforma
- [TASK-E02-01](TASK-E02-01.md) — Validar toolkit Rust com UI clássica acessível

## Entrega concreta

Crate de testes que renderiza telas nativas e as compara com os gabaritos visual, de movimento e de comportamento do MVP, executada localmente.

Referência visual: vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452 (rev.38), caminhos `mvp/...` e `out/...` conforme [referência do MVP](../../docs/design/06-mvp-reference.md). Caminhos relevantes: `out/svg/rendered/`, `out/check/report.json`.

Documento canônico: [referência do MVP](../../docs/design/06-mvp-reference.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

O harness falha quando uma tela ou estado excede a tolerância e aponta o elemento divergente; roda apenas localmente, na máquina do mantenedor, com o clone de vynamp-mvp-reference no SHA 0d9860b901510021bcca4c31a25014373ea7c452; o CI Gate o marca como não executado.

## Verificação e evidência

QA-034 em [QA comum](../../docs/quality/01-test-strategy.md). Registrar commit, comando, versão do Chrome e do resvg, hashes da fonte congelada e dos artefatos. Diferença acima da tolerância é falha, não limitação, salvo exceção de navegador registrada no [mapa de lacunas](../../docs/design/06-mvp-reference.md).

## Execução e limites

Fidelidade total ao MVP: nenhuma lacuna aceita por conveniência. A ferramenta de extração roda num navegador real porque a fonte é web; o produto nativo não depende de navegador. Biblioteca nova necessária para fechar lacuna ganha repositório git próprio e modular. Atualizar backlog JSON e requisito junto de qualquer mudança.
