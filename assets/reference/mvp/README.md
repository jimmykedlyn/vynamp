# Pacote de referência do MVP

Dados extraídos do MVP web rev.37 congelado. A UI nativa é medida contra eles (QA-034). Regras, tolerâncias e fases: [referência do MVP](../../../docs/design/06-mvp-reference.md). Não editar à mão: regenerar com o repositório `vynamp-mvp-reference` e registrar o novo hash em `SOURCE.json`.

| Caminho | Conteúdo |
|---|---|
| `SOURCE.json` | Hash da árvore do MVP, commit da ferramenta, versões de Chrome/Playwright/resvg e sha256 de cada arquivo deste pacote |
| `themes.json` | 12 temas com tokens de origem e custom properties resolvidas; `unsetVars`: variáveis fora de `:root`, com fallback e origem |
| `icons/` | 91 ícones SVG, `icons.json` (nomes, aliases, ícones CSS) e `usage.json` |
| `meters/` | 16 medidores (`<prefixo>/source.svg` + `meta.json`) e `dynamic.json` |
| `rendered/` | SVGs distintos renderizados pela UI e `index.json` com os estados onde aparecem |
| `svg-check.json` | Comparação resvg × Chrome por arquivo |
