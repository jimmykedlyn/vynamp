# Referência do MVP web para a UI nativa

Versão: 0.7.1 · Data: 2026-11-03 · Estado: TASK-E02-11 em execução (PR em revisão); TASK-E02-12..16 planejadas.

Base: [visual](01-visual-system.md), [layouts](02-layouts.md), [marca](04-brand-identity.md), [painel DSP](05-dsp-panel.md), [estratégia de QA](../quality/01-test-strategy.md) (QA-034).

## 1. Regra

O MVP web rev.37 é a especificação visual e de comportamento da UI nativa (egui/eframe, sem navegador) em Windows, Linux, macOS, iOS e Android. A UI nativa reproduz o MVP **sem lacunas**: SVGs, cores, temas, tipografia, animações, medidores, ondas, interações e preferências. O código do MVP não é código de produto.

Só fica fora o que é intrínseco ao navegador (motor DOM/CSS, a API Web Audio em si). O **comportamento observável** dessas APIs é medido e reimplementado; por exemplo, as constantes e a resposta do `AnalyserNode` viram gabarito de TASK-E02-13. Uma biblioteca escrita para fechar uma lacuna ganha repositório git próprio e modular, no padrão de vynamp-decode, vynamp-output e vitrola.

## 2. Fonte congelada e ferramenta

| Item | Valor |
|---|---|
| Fonte | MVP rev.37, apenas os arquivos carregados pelo app (2,4 MB, 203 arquivos) |
| Hash da árvore | `1a8902c9b5d3b1e148d829d25c6defee93ed492a654da95dcb7641c4097fa71a` (sha256 da concatenação das linhas ordenadas `<sha256> *./<caminho>`, cada uma terminada em LF, formato `sha256sum --binary`) |
| Repositório da ferramenta | `vynamp-mvp-reference` (privado): `mvp/` congelado, `tools/extract.cjs`, `tools/svgcheck.cjs`, `tools/verify.cjs`, `tools/serve.cjs`, `tools/package.cjs` |
| Ambiente da extração | Node 24.15, Playwright 1.64.1, Google Chrome 156.0.8078.79, resvg 0.49.0, Windows 11 |
| Proveniência por arquivo | [SOURCE.json](../../assets/reference/mvp/SOURCE.json) |

A ferramenta roda num navegador real porque a fonte é web. Ela não entra no produto nem no build Rust.

## 3. Pacote no repositório

O pacote fica em [assets/reference/mvp](../../assets/reference/mvp/README.md):

| Caminho | Conteúdo | Consumidor |
|---|---|---|
| `themes.json` | 12 temas: tokens de origem, todas as custom properties `--*` resolvidas, `colorScheme`, raio, medidor padrão, estilos de `body` e atributos de `<html>`; `unsetVars` lista as variáveis referenciadas que `:root` não define, com fallback do código e origem (CSS de elemento ou script), cujo valor por estado vem do dump de TASK-E02-12 | TASK-E02-02 (loader de temas) |
| `icons/` | 91 ícones standalone (grade 24×24, traço 1,5, `currentColor`), aliases, ícones CSS e uso por tela | TASK-E02-02, E02-03 |
| `meters/` | 16 medidores: SVG fonte, metadados (tipo, canais, escala dB/ângulo, pivôs, temas que o citam, selecionável) e atributos dinâmicos. O rev.37 só monta os 9 selecionáveis (`player.js` troca qualquer outro índice salvo pelo primeiro e o campo `meter` do tema não troca o medidor); os outros 7 não têm comportamento observável e entram só como asset | TASK-E02-07, E02-13 |
| `rendered/` | Cada SVG distinto que a UI desenha, serializado standalone com estilos computados embutidos, e o índice de estados onde aparece | TASK-E02-12, E02-16 |
| `svg-check.json` | Comparação resvg × Chrome de cada SVG | QA-034 |

A reextração é verificada por `npm run verify`, que extrai de novo para um diretório temporário e compara byte a byte com `out/`; cliques que expiram ficam em `rendered/click-failures.json`, para que uma diferença de cobertura apareça na comparação.

Regras de serialização: atributos animados por script são gravados em repouso (valor do SVG fonte do medidor, ou removidos quando não há) e listados em `neutralized` no índice, para que o arquivo não dependa do instante da captura; `meters/dynamic.json` registra só quais elementos e atributos animam, e as curvas ficam com TASK-E02-13; `width`/`height` não entram no hash; propriedades herdadas só são escritas quando diferem do pai; `currentColor` é preservado por cor sentinela; `class` é removido.

## 4. Fases

| Fase | Tarefa | Entrega |
|---|---|---|
| 0–1 | [TASK-E02-11](../../tasks/items/TASK-E02-11.md) | Congelamento, SVGs, ícones, medidores e temas |
| 2 | [TASK-E02-12](../../tasks/items/TASK-E02-12.md) | Capturas por estado, dump de geometria/estilo, fontes efetivas e estratégia de fontes |
| 3 | [TASK-E02-13](../../tasks/items/TASK-E02-13.md) | Transições, séries rAF sob sinais determinísticos, balística de medidores, ondas |
| 4 | [TASK-E02-14](../../tasks/items/TASK-E02-14.md) | Roteiros de interação e esquema de preferências |
| 5 | [TASK-E02-15](../../tasks/items/TASK-E02-15.md) | Mapa de lacunas web→nativo por plataforma |
| 6 | [TASK-E02-16](../../tasks/items/TASK-E02-16.md) | Harness egui_kittest no CI |

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

  O svgcheck compara o resvg sobre o arquivo normalizado com o Chrome sobre o original e registra `normalizedBlobPx`/`normalizedPass`; o arquivo só é aceito com geometria = 0 px nessa comparação. O SVG fonte permanece intacto; nenhum SVG de `rendered/` usa `pathLength` (o medidor 10 não é montado), e o serializador o normaliza se aparecer.

Resultados: [evidência TASK-E02-11](../evidence/PR/task-e02-11/README.md).

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
