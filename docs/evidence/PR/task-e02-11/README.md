# TASK-E02-11 — MVP de referência congelado e extração

Autor: Claude, 2026-11-03. Base `develop` `b5974f4`; branch `feature/task-e02-11-claude-mvp-reference`. Regras e tolerâncias: [referência do MVP](../../../design/06-mvp-reference.md). Pacote: [assets/reference/mvp](../../../../assets/reference/mvp/README.md), proveniência por arquivo em `SOURCE.json`.

## Fonte e ambiente

| Item | Valor |
|---|---|
| MVP | rev.37, 203 arquivos, árvore `1a8902c9b5d3b1e148d829d25c6defee93ed492a654da95dcb7641c4097fa71a` |
| Ferramenta | `vynamp-mvp-reference` (repositório git próprio, privado; commit em `SOURCE.json`) |
| Ambiente | Windows 11, Node 24.15, Playwright 1.64.1, Google Chrome 156.0.8078.79, resvg 0.49.0 |
| Comandos | `npm run serve`, `npm run extract`, `npm run check-svg`, `npm run verify`, `node tools/package.cjs <repo>/assets/reference/mvp` |

## Resultado por critério

| Critério | Resultado |
|---|---|
| Extração | 12 temas, 91 ícones, 16 medidores (9 selecionáveis, com dinâmica; 7 só como asset), 286 SVGs renderizados distintos |
| SVG resvg × Chrome, geometria = 0 px | 387 de 393 passam direto; todos os 6 restantes estão explicados e nenhum é diferença aceita. 5 são lacunas do resvg que viram requisito do carregador nativo e caem para 0 px com a normalização (`normalizedPass`). (a) `meters/10-arco-hifi-champagne/source.svg` usa `pathLength` (3880 px). O mantenedor não usa mais esse medidor, mas ele fica no pacote porque os knobs usam o mesmo `pathLength`. (b) 4 recortes da camada de fios do grafo DSP (`02f634db6c4c58f9`, `4719decdf855bbbf`, `c936e75101185a73`, `ef8bd2333c30c6fe`, 22 px) usam `filter:blur(3px)` de CSS num fio reto: o resvg corta o brilho nas pontas. O sexto é o arquivo sem tinta abaixo |
| Fundo de comparação | Todo arquivo é comparado sobre branco e sobre preto, e o relatório guarda o pior (`bg`). Em 7 arquivos o pior é o preto (tinta branca ou brilho claro) |
| Sem tinta (`empty`) | 1 arquivo, que não passa: `rendered/b74e8795efe095c4.svg`. É o medidor 02 no estado `meter-off`, em que o `<svg>` tem `visibility:hidden`. Está vazio de fato, e a geometria desse medidor é comparada nos estados visíveis |
| Temas com tokens `--*` resolvidos | 12 temas; 83–84 custom properties resolvidas em `:root` por tema, nenhuma com `var()` pendente. 22 variáveis referenciadas não são definidas em `:root` por algum tema (vêm de CSS de elemento ou de script; `--wave-spectrum-right-color`, por exemplo, falta em 8 temas) e estão em `unsetVars` com fallback, origem e temas afetados (`missingIn`); o valor por estado entra no dump de TASK-E02-12 |
| Reextração reproduz os mesmos hashes | `npm run verify` (segunda extração completa em diretório temporário, comparada byte a byte com `out/`, `check/` excluído): 415 arquivos, 0 diferenças |

## Cliques da varredura que expiram

`rendered/click-failures.json` lista 3 cliques que não completam em 1,2 s: o link "Ir para a biblioteca" (em 1440 e 390 px; é `.skip`, fora da tela em `top:-80px` até receber foco) e `#clear-queue` em 1440 px. Foram os mesmos nas duas extrações deste PR (o arquivo entra na comparação do `verify`); como dependem de tempo-limite, uma máquina sob carga pode acusar diferença aí, e o `verify` a mostra. O estado com foco do link e a fila limpa entram no dump de estados de TASK-E02-12, que aciona por teclado e por script.

## Achados que mudaram a ferramenta

- **Defs externos:** gradientes e filtros definidos em outro `<svg>` da página são copiados para `<defs>`. Sem isso, o Chrome descartava o elemento e os pivôs do medidor vintage divergiam em 869 px.
- **`url()` sem aspas:** o resvg ignora `filter:url("#id")` em `style`. O serializador escreve `url(#id)` e as sombras do medidor industrial passaram de 71 px para 0.
- **Métrica de borda:** o teste de vizinhança 3×3 por faixa mín–máx por canal elimina o antialiasing deslocado até 1 px sem esconder diferença de geometria.
- **Canvas > 16 Mpx:** a camada de fios do grafo DSP (12000×12000) é recortada à caixa pintada + 16 px, o que evita falta de memória no canvas.
- **Valores animados:** a primeira dupla extração divergiu em 50 arquivos. `meta.json`/`dynamic.json` guardavam valores amostrados no tempo (ângulo de agulha, `currentTime`) e os SVGs renderizados gravavam o atributo animado no instante da captura. Agora só se registra *quais* atributos animam, e o SVG é gravado em repouso (`neutralized` no índice). A chave de elemento animado usa o caminho a partir do id mais próximo (`#…-band-L-00 > g`), e não só a última etiqueta (`g`), que atingiria todo grupo.
- **Corrida da waveform:** depois disso, ainda divergiam 14 arquivos e depois 1. A onda é decodificada de forma assíncrona e desenhada uma vez (`#wave` `data-source` vai de `loading` a `decoded`) ao carregar a faixa e a cada clique que seleciona outra. A captura às vezes pegava a onda provisória. O extrator agora espera `data-source` sair de `loading` após carregar e após cada clique. A dinâmica só conta como animado o atributo que muda 3 vezes ou mais na janela de 3 s; uma escrita única e tardia não é animação.
- **Tinta branca:** o svgcheck compunha tudo sobre branco e marcava como vazios os SVGs de tinta branca. Agora compara sobre os dois fundos e fica com o pior (`bg`). Isso revelou o corte do `blur()` nas pontas dos fios, que só aparece sobre preto. Arquivo vazio não passa.
- **`serve.cjs`:** com diretório relativo, o servidor respondia 404 a tudo (comparava caminho relativo com absoluto). Corrigido; escuta só em loopback, a travessia `..` continua bloqueada e escape malformado responde 404.

## Pendências para outras TASKs

- Texto: a diferença de glifos (DirectWrite × resvg) fica registrada por arquivo em `svg-check.json` (`strictPct`, `shiftPx` com texto) e é resolvida em TASK-E02-12 com a estratégia de fontes.
- `pathLength` e `blur()` de CSS (região do filtro) viram requisitos do carregador SVG nativo (TASK-E02-15 confirma o renderizador).
- Os 3 cliques expirados acima: estados cobertos em TASK-E02-12.
