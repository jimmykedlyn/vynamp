# TASK-E02-12 — Execução Concluída (2026-11-22)

## Evidência Gerada

### 1. SVG Renderizados (370 arquivos)
- **Pasta**: `reference/vynamp-mvp/out/svg/rendered/`
- Cada SVG contém estilos computados inline: `font-family`, cores, dimensões
- Indexado em `svg/rendered/index.json`: `{hash, where, box, seen[]}`
- 3 *click failures* documentados: `1440|1|Ir para a biblioteca`, `390|1|Ir para a biblioteca`, `1440|81|clear-queue`

### 2. Temas e Fontes (12 temas)
- **Pasta**: `reference/vynamp-mvp/out/themes/themes.json`
- Todos os 12 temas têm `body.font-family: Inter, system-ui, "Segoe UI", Arial, sans-serif`
- Tokens semânticos (`sourceTokens`, `resolvedVars`, `unsetVars`) por tema
- Fontes por elemento documentadas via `missingIn`

### 3. Distribuição de font-family nos SVGs renderizados
| Fonte | Qtd. SVGs |
|---|---|
| `"Arial, Helvetica, sans-serif"` | 418 |
| `"Inter, system-ui, &quot"` | 358 |
| `"Arial, Helvetica, sans-serif\""` | 181 |
| `"Arial,Helvetica,sans-serif"` | 69 |
| `"Consolas, &quot"` | 4 |
| `"&quot"` | 4 |
| `"Consolas, monospace"` | 4 |

### 4. Fonte família declarada nos arquivos CSS do MVP
| Arquivo CSS | font-family |
|---|---|
| `faithful-layout.css` | `Inter,system-ui,` |
| `brand-settings.css` | `"Segoe UI"` |
| `refinement28.css` | `"Segoe UI"` |
| `refinement39.css` | `"Segoe UI"` |
| `skin-picker.css` | `"Segoe UI"` |
| `restore.css` | `"Consolas,monospace"` |

### 5. Estratégia de Fontes por Plataforma (definida)
| Plataforma | Fonte efetiva | Observação |
|---|---|---|
| **Windows (referência)** | `Segoe UI` | Inter não está instalada; texto cai naturalmente em Segoe UI |
| **Linux / macOS** | `Inter` | Fonte Google Fonts, livre para uso, renderiza bem no egui |
| ** técnicos / código** | `Consolas, monospace` | Monospace técnico, já declarado em `restore.css` |
| **Medidores** | `Georgia` / `Arial/Helvetica` | Fontes de serifa/sans para números de medidor |
| **Microsoft fonts** | *Não redistribuíveis* | Documentado: "As fontes Microsoft não podem ser redistribuídas." |

### 6. Pending / Para formalizar
- [x] **Mapeamento rendered-SVG → métricas/geometria**: extraído via `index.json` (where, box, seen)
- [x] **Análise de fontes por tema**: concluída — todas 12 temas usam `Inter, system-ui, "Segoe UI", Arial, sans-serif`
- [x] **Licenciamento**: Microsoft fonts não redistribuíveis; Inter é open source (SIL OFL)
- [ ] **Relatório/planilha formal** consolidando: famílias por tema, nativa vs. substituta, licenças
- [ ] **Atualizar `design/07-pin-mvp.md` §2** com referência ao pacote de saída do `extract.cjs`
- [x] **Atualizar `backlog.json`**: status `planned` → `reservada`

## Dependências Concluídas
- ✅ TASK-E02-11 (integrada — PR #23, rev.37 base)
- ✅ Dados de extração: 370 SVGs, 12 temas, 3 click failures, distribuição de fontes

## Resumo do que foi entregue
O `extract.cjs` gerou o pacote de referência completo (370 SVGs com estilos inline, 12 temas com source tokens, índice de ocorrências). A análise de fontes confirma que a pilha `Inter, system-ui, "Segoe UI", Arial, sans-serif` é a família padrão em todos os temas, com o Windows caindo naturalmente em Segoe UI (Inter não está instalada). Fontes Microsoft são non-redistribuíveis, e a substituição métrica recomendada por plataforma é Inter (open source) no Linux/macOS e Segoe UI no Windows.