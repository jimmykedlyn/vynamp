# TASK-E02-12 — Gerar gabarito visual e estratégia de fontes

Versão: 0.7.1 · Data: 2026-11-03 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E02-12 / REQ-E02-12 |
| Fase / prioridade | M0 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | design — pessoa a designar |
| Esforço provisório | 3–6 dias úteis |
| Risco | médio |

## Dependências

- [TASK-E02-11](TASK-E02-11.md) — Congelar MVP de referência e extrair SVGs, ícones e temas

## Entrega concreta

Capturas por tema, layout, largura e estado com dump de geometria e estilo por elemento e fontes efetivas por nó; estratégia de fontes por plataforma com licenças.

Documento canônico: [referência do MVP](../../docs/design/06-mvp-reference.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Todo estado alcançável do MVP tem captura e dump (geometria, estilo computado e custom properties --* por elemento, inclusive as de unsetVars), incluindo os estados cujo clique expira na varredura de TASK-E02-11 (`rendered/click-failures.json`: link de salto com foco, fila limpa), acionados por teclado ou script; cada família de fonte usada tem fonte nativa ou substituta licenciada com métricas comparadas; armazenamento dos PNG decidido.

## Verificação e evidência

QA-034 em [QA comum](../../docs/quality/01-test-strategy.md). Registrar commit, comando, versão do Chrome e do resvg, hashes da fonte congelada e dos artefatos. Diferença acima da tolerância é falha, não limitação, salvo exceção de navegador registrada no [mapa de lacunas](../../docs/design/06-mvp-reference.md).

## Execução e limites

Fidelidade total ao MVP: nenhuma lacuna aceita por conveniência. A ferramenta de extração roda num navegador real porque a fonte é web; o produto nativo não depende de navegador. Biblioteca nova necessária para fechar lacuna ganha repositório git próprio e modular. Atualizar backlog JSON e requisito junto de qualquer mudança.
