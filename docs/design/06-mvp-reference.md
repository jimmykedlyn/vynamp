# Referência do MVP web para a UI nativa

Versão: 0.8.0 · Data: 2026-11-03 · Estado: referência atual rev.38 em `vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452`; TASK-E02-11 integrada (PR #23, feita sobre a rev.37, histórico); TASK-E02-12..16 planejadas e apontadas para a rev.38.

Base: [visual](01-visual-system.md), [layouts](02-layouts.md), [marca](04-brand-identity.md), [painel DSP](05-dsp-panel.md), [estratégia de QA](../quality/01-test-strategy.md) (QA-034).

## 1. Regra

O MVP web rev.38 (`vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452`) é a especificação visual e de comportamento da UI nativa (egui/eframe, sem navegador) em Windows, Linux, macOS (Intel e Apple Silicon) e Android; iOS está fora da 1.0 (ver [visão e escopo](../product/01-vision-scope.md)). Nenhum arquivo do MVP é copiado para este repositório: as specs citam URL, SHA fixo e caminho dentro do repositório de referência. A UI nativa reproduz o MVP **sem lacunas**: SVGs, cores, temas, tipografia, animações, medidores, ondas, interações e preferências. O código do MVP não é código de produto.

Só fica fora o que é intrínseco ao navegador (motor DOM/CSS, a API Web Audio em si). O **comportamento observável** dessas APIs é medido e reimplementado; por exemplo, as constantes e a resposta do `AnalyserNode` viram gabarito de TASK-E02-13. Uma biblioteca escrita para fechar uma lacuna ganha repositório git próprio e modular, no padrão de vynamp-decode, vynamp-output e vitrola.

## 2. Fonte congelada e ferramenta

| Item | Valor |
|---|---|
| Repositório | https://github.com/jimmykedlyn/vynamp-mvp-reference (privado), branch `main` |
| Pin | `vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452` |
| Congelamento | rev.38 congelada no commit `0a6342c` ("Freeze MVP rev.38: all 16 meters selectable"); `0d9860b` ("Make mvp/ the only, editable copy of the MVP") é o pin |
| Fonte | `mvp/`: 203 arquivos, 2.118.012 bytes (~2,1 MB) |
| Hash de conteúdo de `mvp/` | `f00f766045edf4c1032c9a30c5826803c090f960ac9377d815c020079f398bba` (sha256 das linhas `<sha256> *./<caminho>` ordenadas por caminho, cada uma terminada em LF, formato `sha256sum --binary`), calculado sobre os blobs do commit (checkout limpo; uma cópia de trabalho com CRLF dá outro valor) |
| Git tree ids (outra definição, não misturar com o hash acima) | `mvp/` = `f85fd66247ecdb4cd649b45804f51ab2a140bf9a`, `out/` = `1d9f9615fd372d9ed6d39a1fa54be18b96440b4a` (`git rev-parse 0d9860b:mvp` e `:out`) |
| Bytes | `.gitattributes` `* -text`: `mvp/` e `out/` são hasheados byte a byte, sem conversão de fim de linha |
| Ferramentas | `tools/extract.cjs`, `tools/svgcheck.cjs`, `tools/verify.cjs`, `tools/serve.cjs`, `tools/package.cjs` |
| Ambiente | Node ≥ 20, Google Chrome e resvg 0.49.0 (README do repositório de referência); Playwright 1.64.1 (`package.json`). Rev.37, histórico: extraída com Node 24.15, Chrome 156.0.8078.79, Windows 11 |
| Proveniência | URL do repositório + SHA do commit + hash de conteúdo/tree. Não existe mais `SOURCE.json` |

Pendência fora deste repositório: no pin `0d9860b`, `tools/package.cjs` e o README de `vynamp-mvp-reference` ainda copiam `out/` para `assets/reference/mvp/` e escrevem `SOURCE.json`. Isso deve ser removido lá (follow-up no repositório de referência); este repositório não aceita esse pacote.

A ferramenta roda num navegador real porque a fonte é web. Ela não entra no produto nem no build Rust.

## 3. Pacote de referência

Nenhum arquivo do MVP fica neste repositório. O pacote é gerado e mantido em `vynamp-mvp-reference` (`out/`); caminhos relativos à raiz desse repositório no pin acima:

| Caminho | Conteúdo | Consumidor |
|---|---|---|
| `out/themes/themes.json` | 12 temas (chaves `generator`, `themes`, `unsetVars`): tokens de origem, todas as custom properties `--*` resolvidas, `colorScheme`, raio, medidor padrão, estilos de `body` e atributos de `<html>`; `unsetVars` lista as variáveis referenciadas que `:root` não define, com fallback do código e origem (CSS de elemento ou script), cujo valor por estado vem do dump de TASK-E02-12 | TASK-E02-02 (loader de temas) |
| `out/svg/icons/` | 91 ícones standalone (grade 24×24, traço 1,5, `currentColor`), 9 aliases, 4 ícones CSS, `icons.json` e `usage.json` (uso por tela) | TASK-E02-02, E02-03 |
| `out/svg/meters/<16 diretórios>/` e `out/svg/meters/dynamic.json` | 16 medidores: SVG fonte, metadados (tipo, canais, escala dB/ângulo, pivôs, temas que o citam, selecionável) e atributos dinâmicos. Na rev.38 os 16 são selecionáveis e montados | TASK-E02-07, E02-13 |
| `out/svg/rendered/` | Cada SVG distinto que a UI desenha, serializado standalone com estilos computados embutidos, e o índice de estados onde aparece; inclui `click-failures.json` | TASK-E02-12, E02-16 |
| `out/check/report.json` (`out/check/diff/` e `out/check/resvg/` são saídas locais do svgcheck, não versionadas) | Comparação resvg × Chrome de cada SVG (substitui o antigo `svg-check.json`) | QA-034 |

A reextração é verificada por `npm run verify`, que extrai de novo para um diretório temporário e compara byte a byte com `out/` (rev.38: 499 arquivos, 0 diferem); cliques que expiram ficam em `out/svg/rendered/click-failures.json`, para que uma diferença de cobertura apareça na comparação.

Regras de serialização: atributos animados por script são gravados em repouso (valor do SVG fonte do medidor, ou removidos quando não há) e listados em `neutralized` no índice, para que o arquivo não dependa do instante da captura; `out/svg/meters/dynamic.json` registra só quais elementos e atributos animam, e as curvas ficam com TASK-E02-13; `width`/`height` não entram no hash; propriedades herdadas só são escritas quando diferem do pai; `currentColor` é preservado por cor sentinela; `class` é removido.

## 4. Fases

| Fase | Tarefa | Entrega |
|---|---|---|
| 0–1 | [TASK-E02-11](../../tasks/items/TASK-E02-11.md) | Congelamento, SVGs, ícones, medidores e temas (integrada, histórico: feita sobre a rev.37; a rev.38 é a fonte atual) |
| 2 | [TASK-E02-12](../../tasks/items/TASK-E02-12.md) | Capturas por estado, dump de geometria/estilo, fontes efetivas e estratégia de fontes |
| 3 | [TASK-E02-13](../../tasks/items/TASK-E02-13.md) | Transições, séries rAF sob sinais determinísticos, balística de medidores, ondas |
| 4 | [TASK-E02-14](../../tasks/items/TASK-E02-14.md) | Roteiros de interação e esquema de preferências |
| 5 | [TASK-E02-15](../../tasks/items/TASK-E02-15.md) | Mapa de lacunas web→nativo por plataforma |
| 6 | [TASK-E02-16](../../tasks/items/TASK-E02-16.md) | Harness egui_kittest, executado só localmente na máquina do mantenedor, com o clone de `vynamp-mvp-reference` no SHA fixado; o CI Gate o marca como não executado |

## 5. Tolerância de SVG

O renderizador nativo de referência é o resvg (Rust puro, `usvg` + `tiny-skia`); a escolha do renderizador SVG da UI é confirmada em TASK-E02-15 e, se for outro, a verificação é refeita com ele. Cada SVG é renderizado com zoom 2 no Chrome e no resvg e composto sobre branco e sobre preto (tinta branca sobre transparente só aparece sobre preto); vale o fundo com a maior diferença, registrado em `bg`. Um pixel diverge quando algum canal difere mais de 48.

| Métrica | Definição | Uso |
|---|---|---|
| estrita | pixels divergentes / pixels de tinta | registro |
| deslocamento | divergentes cujo valor, por canal (±48), cai fora da faixa mín–máx da vizinhança 3×3 da outra renderização, nos dois sentidos | registro |
| geometria | deslocamento com todo `<text>` removido dos dois lados | critério |

- **Critério:** geometria = 0 px em todo SVG. A diferença tolerada é só antialiasing de borda deslocado até 1 px, com qualquer cobertura.
- **Canvas maiores que 16 Mpx** (camada de fios do grafo DSP, 12000×12000) são recortados à caixa pintada + 16 px antes da comparação; o arquivo do pacote não muda.
- **Serialização compatível:** `url("#id")` computado é escrito sem aspas (o resvg ignora `filter:url("…")` em `style`), e gradientes/filtros definidos em outro SVG da página são copiados para `<defs>`.
- **Sem tinta:** arquivo sem pixel de tinta nas duas renderizações, nem sobre branco nem sobre preto, não foi comparado; o svgcheck o marca `empty`, ele não passa e a evidência explica cada um.
- **Texto:** a diferença de rasterização de glifos (DirectWrite × resvg) fica registrada por arquivo e é resolvida em TASK-E02-12, junto da estratégia de fontes; não é aceita como lacuna.
- **Recursos que o resvg não implementa ou trata diferente** são marcados em `gaps` e viram requisito do carregador SVG nativo:
  - `pathLength`: reescalar `stroke-dasharray`/`stroke-dashoffset` pelo comprimento real do caminho;
  - `blurFn` (`filter:blur(Npx)` de CSS, só no brilho dos fios do grafo DSP): o resvg deriva a região do filtro da caixa do elemento, de altura 0 num fio reto, e corta o brilho nas pontas; o carregador converte para `feGaussianBlur` com região em `userSpaceOnUse` = caixa com traço + 3σ + 1, como o Chrome pinta. Forma que a normalização não cobre (`blur()` em regra de `<style>`, outra unidade, várias funções no mesmo `filter`) fica em `unhandledGaps` e reprova.

  O svgcheck compara o resvg sobre o arquivo normalizado com o Chrome sobre o original e registra `normalizedBlobPx`/`normalizedPass`; o arquivo só é aceito com geometria = 0 px nessa comparação. O SVG fonte permanece intacto; nenhum SVG de `out/svg/rendered/` contém `pathLength` (verificado por grep); ele aparece no SVG fonte do medidor 10, e o serializador o normaliza se aparecer em `rendered/`.

Resultados do check-svg na rev.38: 477 arquivos verificados, 471 passam, 5 passam só após a normalização (lacunas) e 1 está vazio.

- `pathLength`: `out/svg/meters/10-arco-hifi-champagne/source.svg`.
- `blurFn`: quatro arquivos de `out/svg/rendered/` (`02f634db6c4c58f9`, `4719decdf855bbbf`, `c936e75101185a73`, `ef8bd2333c30c6fe`).
- Vazio: `out/svg/rendered/b74e8795efe095c4.svg` (sem tinta sobre branco nem sobre preto); a evidência precisa explicar o caso e a explicação está pendente para TASK-E02-12.

Evidência histórica (rev.37): [evidência TASK-E02-11](../evidence/PR/task-e02-11/README.md).

## 6. Fontes

A pilha do MVP é `Inter, system-ui, "Segoe UI", Arial, sans-serif`. No Windows de referência, Inter não está instalada e o texto cai em Segoe UI. Os medidores usam Georgia e Arial/Helvetica; trechos técnicos usam Consolas. As fontes Microsoft não podem ser redistribuídas. TASK-E02-12 captura a fonte efetiva de cada nó e decide, com licença registrada, a fonte do sistema ou a substituta métrica compatível por plataforma.

## 7. Exceções de navegador

| Recurso do MVP | Tratamento nativo |
|---|---|
| API Web Audio (`AudioContext`, `AnalyserNode`) | Motor próprio (E01/E04); constantes e resposta medidas em TASK-E02-13 |
| Motor de layout DOM/CSS | Layout egui reproduz geometria do dump de TASK-E02-12 |
| `localStorage` | Preferências nativas com o esquema de TASK-E02-14 |
| `requestAnimationFrame`/transições CSS | Animação egui reproduzindo curvas de TASK-E02-13 |

TASK-E02-15 completa esta tabela para todos os recursos usados.
