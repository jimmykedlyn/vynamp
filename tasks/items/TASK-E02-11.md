# TASK-E02-11 — Congelar MVP de referência e extrair SVGs, ícones e temas

Versão: 0.7.1 · Data: 2026-11-03 · Estado: em execução; PR em revisão, sem aceite antecipado.

## Contrato da tarefa

| Campo | Valor |
|---|---|
| ID / requisito | TASK-E02-11 / REQ-E02-11 |
| Fase / prioridade | M0 / P0 |
| Estado | Em execução; sem aceite antecipado |
| Responsável funcional | design — pessoa a designar |
| Esforço provisório | 2–4 dias úteis |
| Risco | médio |

## Dependências

- Nenhuma. Usa o MVP rev.37 em `D:/Projects/vynamp-media-player/mvp` como fonte.

## Entrega concreta

Pacote assets/reference/mvp com MVP rev.37 congelado por hash, 12 temas resolvidos, 91 ícones, 16 medidores e SVGs renderizados, extraídos por ferramenta reprodutível.

Documento canônico: [referência do MVP](../../design/07-pin-mvp.md). [Matriz normativa](../../product/03-requisitos.md).

## Critérios de aceite

Cada SVG renderiza em resvg sem pixel de geometria divergente do Chrome (delta>48 fora do antialiasing de 1 px; texto registrado e resolvido em E02-12; recurso sem suporte no resvg vira requisito do carregador nativo); cada tema traz resolvidos todos os tokens --* que define em :root, e toda variável referenciada que o tema não define em :root consta de unsetVars com fallback, origem e temas afetados (valor por estado em E02-12); reextração reproduz os mesmos hashes.

## Verificação e evidência

QA-034 em [QA comum](../../quality/01-estrategia-testes.md). Registrar commit, comando, versão do Chrome e do resvg, hashes da fonte congelada e dos artefatos. Diferença acima da tolerância é falha, não limitação, salvo exceção de navegador registrada no [mapa de lacunas](../../design/07-pin-mvp.md).

## Execução e limites

Fidelidade total ao MVP: nenhuma lacuna aceita por conveniência. A ferramenta de extração roda num navegador real porque a fonte é web; o produto nativo não depende de navegador. Biblioteca nova necessária para fechar lacuna ganha repositório git próprio e modular. Atualizar backlog JSON e requisito junto de qualquer mudança.
