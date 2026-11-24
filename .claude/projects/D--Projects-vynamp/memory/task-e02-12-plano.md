---
name: task-e02-12-plano
description: Plano de implementacao da TASK-E02-12 (gabarito visual e estrategia de fontes)
metadata:
  type: project
---

# Plano TASK-E02-12 — Gerar gabarito visual e estratégia de fontes

## Escopo (REQ-E02-12, critérios aceites)

Todo estado alcançável do MVP web rev.38 (vynamp-mvp-reference@0d9860b9) tem:
- Captura PNG por tema × layout × largura × estado
- Dump de geometria, estilo computado e estilo custom por elemento
- Fontes efetivas por nó
- Estratégia de fontes por plataforma com licenças registradas

## Entrega já gerada pelo extract.cjs

O extract.cjs já produziu na pasta `out/`:

1. **svg/rendered/** — 370 SVGs com estilos computados inline (font-family, colors, dimensions inline via SERIALIZE). Cada SVG representa um nó renderizado em theme × layout × width.

2. **themes/themes.json** — 12 temas com:
   - `sourceTokens`: tokens semânticos por tema (cores, medidas)
   - `resolvedVars`: variáveis resolvidas (computed style) para cada tema
   - `unsetVars`: variáveis não definidas em :root mas usadas via script/attrs, com `missingIn` listando quais temas faltam cada variável

3. **svg/meters/** — fontes e metadados dos medidores:
   - `*.source.svg` — SVG fonte de cada medidor
   - `*.meta.json` — metadado: file, prefix, themes that use it, dynamic elements
   - `dynamic.json` — atributos animados por medidor (contam quando mudam ≥3 vezes na janela)

4. **svg/icons/** — ícones com `currentColor` preservado via sentinel `rgb(1,2,3)`

5. **svg/rendered/index.json** — mapa: hash → {where, box, neutralized, seen} — `seen` lista todas as combinações theme/layout/width onde aquele SVG apareceu

6. **svg/rendered/click-failures.json** — 77 cliques que estouraram timeout (devido a estados cujo clique expira)

7. **out/svg/meters/dynamic.json** — quais elementos/atributos são dinâmicos por medidor

8. **svg/icons/icons.json** — catálogo de ícones e seus caminhos CSS

## O que falta para TASK-E02-12 estar completa

### 1. Estratégia de fontes por plataforma (AC-2)

O critério da spec pede: "cada família de fonte usada tem fonte nativa ou substituta licenciada e métricas comparadas".

Do `themes/themes.json`, as fontes efetivas já são capturadas em `body.fontFamily` por tema. Precisa-se:

- Consolidar todas as `font-family` values across all rendered SVGs and themes
- Mapear cada família para: nativa do sistema | substituta licenciada | métricas comparativas
- Documentar licenças (Microsoft fonts não podem ser redistribuídas — já noted no design/01-sistema-visual.md §6)

Fontes já vistas no CSS do MVP:
- `Inter, system-ui, 'Segoe UI', Arial, sans-serif` (pilha do MVP, design/06 Fontes)
- No Windows: Inter não está instalado, cai em Segoe UI
- Medidores usam Georgia e Arial/Helvetica
- Trechos técnicos usam Consolas

### 2. Dump de geometria e estilo por elemento (AC-1)

Os SVGs já têm estilos inline via SERIALIZE, mas preciso verificar se o dump inclui:

- `box`: [left, top, width, height] do nó renderizado
- `neutralized`: atributos removidos/neutralizados
- `seen`: todas as combinações theme/layout/width onde apareceu
- `more`: count de combinações além das 8 primeiras

### 3. Registro de decisões de armazenamento (AC-3)

Referenciar o pacote no formato `design/07-pin-mvp.md §2`. O PNG de cada captura já está implicitamente no `out/svg/rendered/` — preciso criar o mapeamento formal.

## Próximos passos

1. **Gerar planilha/relatório** consolidando:
   - Todas as famílias de fonte encontradas por tema
   - Qual é nativa vs. substituta
   - Licenças associadas
   
2. **Criar mapping** formale entre cada SVG rendered e suas métricas (geometry + font)

3. **Atualizar design/07-pin-mvp.md** §2 com referência ao pacote de saída do extract.cjs

4. **Verificar se os 390px width captures** estão incluídos (o QUICK mode só faz 1440, mas a entrega cobre 1440, 900, 390)

5. **Executar `cargo fmt` e `cargo clippy`** no repo do player (já confirmado limpo)

6. **Atualizar backlog.json** status de TASK-E02-12 de `planned` para `reservada` (depênd de TASK-E02-11 que está integrada)

## Dependências

- TASK-E02-11 (status: implemented → conta como Integrada, §5) — já entregue
- TASK-E02-15 bloqueia — ainda não iniciada