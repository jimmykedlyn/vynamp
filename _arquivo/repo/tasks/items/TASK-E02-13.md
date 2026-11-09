# TASK-E02-13 — Gerar gabarito de movimento, medidores e ondas

Versão: 0.7.2 · Data: 2026-11-03 · Estado: especificação proposta, implementação não iniciada.

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

Referência visual: vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452 (rev.38), caminhos `mvp/...` e `out/...` conforme [referência do MVP](../../design/07-pin-mvp.md). Caminhos relevantes: `out/svg/meters/dynamic.json`, `mvp/`.

Documento canônico: [referência do MVP](../../design/07-pin-mvp.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Cada animação do MVP tem duração, easing e keyframes ou série temporal; medidores e ondas respondem a degrau, seno e rajada com tolerância documentada; Web Audio entra só como comportamento reimplementável.

## Verificação e evidência

QA-034 em [QA comum](../../quality/01-estrategia-testes.md). Registrar commit, comando, versão do Chrome e do resvg, hashes da fonte congelada e dos artefatos. Diferença acima da tolerância é falha, não limitação, salvo exceção de navegador registrada no [mapa de lacunas](../../design/07-pin-mvp.md).

## Execução e limites

Fidelidade total ao MVP: nenhuma lacuna aceita por conveniência. A ferramenta de extração roda num navegador real porque a fonte é web; o produto nativo não depende de navegador. Biblioteca nova necessária para fechar lacuna ganha repositório git próprio e modular. Atualizar backlog JSON e requisito junto de qualquer mudança.
