# TASK-E02-13 — Gerar gabarito de movimento, medidores e ondas

Versão: 0.7.1 · Data: 2026-11-03 · Estado: especificação proposta, implementação não iniciada.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E02-13 / REQ-E02-13 |
| Fase / prioridade | M0 / P0 |
| Estado | Planejada; não implementada |
| Responsável funcional | design — pessoa a designar |
| Esforço provisório | 3–6 dias úteis |
| Risco | alto |

## Dependências

- [TASK-E02-11](TASK-E02-11.md) — Congelar MVP de referência e extrair SVGs, ícones e temas

## Entrega concreta

Transições CSS, séries rAF sob sinais determinísticos, constantes de AnalyserNode e balística e estilos de onda em formato consumível pelo Rust.

Documento canônico: [referência do MVP](../../docs/design/06-mvp-reference.md). [Matriz normativa](../../docs/product/03-requirements.md).

## Critérios de aceite

Cada animação do MVP tem duração, easing e keyframes ou série temporal; medidores e ondas respondem a degrau, seno e rajada com tolerância documentada; Web Audio entra só como comportamento reimplementável.

## Verificação e evidência

QA-034 em [QA comum](../../docs/quality/01-test-strategy.md). Registrar commit, comando, versão do Chrome e do resvg, hashes da fonte congelada e dos artefatos. Diferença acima da tolerância é falha, não limitação, salvo exceção de navegador registrada no [mapa de lacunas](../../docs/design/06-mvp-reference.md).

## Execução e limites

Fidelidade total ao MVP: nenhuma lacuna aceita por conveniência. A ferramenta de extração roda num navegador real porque a fonte é web; o produto nativo não depende de navegador. Biblioteca nova necessária para fechar lacuna ganha repositório git próprio e modular. Atualizar backlog JSON e requisito junto de qualquer mudança.
