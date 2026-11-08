# Evidência — chore/claude-specs-vynamp-restore (PR4)

Data: 2026-11-05 · Autor: Claude Opus 5.5 (Orquestrador) · Revisor: subagente Opus (contexto limpo).

Escopo: especificação normativa do **Vynamp Restore** (ferramenta desktop dedicada) e preparação do **Vynamp Converter**, com ADR, épico E18, matriz normativa e backlog. **Nenhuma linha de código de restauração, conversão ou encoder foi implementada ou executada** — esta pasta registra apenas o que foi verificado localmente.

## Ambiente

| Item | Valor |
|---|---|
| SO | Windows 11 Pro 10.0.26300 |
| Rust | 1.99.0 |
| Worktree | `repo/.worktrees/claude/specs-vynamp-restore` |
| Branch | `chore/claude-specs-vynamp-restore` |
| Base | `chore/claude-specs-shell-integration` (64b7487) |
| Referência factual | MVP web rev.38 em `vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452` |

## Entregas

| Artefato | Conteúdo |
|---|---|
| `docs/architecture/adr/ADR-013-vynamp-restore.md` | Decisão: Restore como processo separado com motor de restauração reaproveitável; escopo do que **não** é restauração; correção das incoerências do MVP |
| `docs/tools/01-vynamp-restore.md` | Spec normativa do `vynamp-restore`: CLI, cadeia de 11 estágios DSP com equações e faixas de parâmetro, 12 detectores de defeito, regiões, A/B, lote, metadados, exportação determinística, projeto `.vynamp-restore`, matriz de plataformas |
| `docs/tools/02-vynamp-converter.md` | Escopo preparatório do `vynamp-converter`: matriz de formatos, rótulagem lossless/lossy, fila de jobs, fronteira da lib `vynamp-encode` |
| `tasks/epics/E18.md` | Épico E18 — Ferramentas de arquivo: Restore e Converter |
| `tasks/items/TASK-E18-01.md`, `TASK-E18-02.md` | Contratos das duas tarefas |
| `docs/product/03-requirements.md` | REQ-E18-01 e REQ-E18-02 (184 requisitos/tarefas em 18 épicos) |
| `tasks/backlog.json` | Entradas TASK-E18-01 e TASK-E18-02 com paridade verbatim de `title`/`deliverable`/`acceptance` |
| `docs/INDEX.md`, `docs/GLOSSARY.md`, `docs/product/06-feature-parity.md`, `CHANGELOG.md` | Índice, glossário, catálogo de funcionalidades e histórico |

## Verificações locais

| Comando | Resultado |
|---|---|
| `cargo xtask check-docs` | `result: passed`, 0 erros, 338 arquivos Markdown, 184 tarefas, 184 requisitos, 18 épicos, 68 entradas de catálogo |
| `cargo xtask check-delivery` | `result: passed`, 0 erros |
| `cargo test -p xtask` | 85 aprovados, 0 falhas |
| Hook de pre-push (`xtask ci docs` + testes de `xtask` e `tests/tooling.rs`) | Aprovado — 51 + 34 testes, 0 falhas |

## Fidelidade ao MVP rev.38 e correções

O inventário factual (algoritmos, faixas de parâmetro, diagnósticos, presets) vem do MVP rev.38 fixado por hash. Três incoerências do protótipo foram corrigidas na versão nativa e **declaradas** em [ADR-013 §4](../../../adr/ADR-013-restore.md):

1. **De-crackle** — o módulo `mid/side` do MVP é código morto; o de-crackle realmente executado é o modelo AR/L-SAR, que é o que a spec normativa fixa (ordem P=48, 2 passagens por canal).
2. **Denoise Wiener** — inalcançável no MVP (sem UI e descartado pelo state store); declarado **fora de escopo** da 1.0 em vez de silenciosamente ausente.
3. **Região `kind: overlay`** — placeholder ("AI separation pending") no MVP; a 1.0 suporta apenas regiões de ganho/fade (`kind: between`).
4. **Detector de cliques** — `restore-click-detector.js` roda no worker e escreve `report.clickDetection`, mas nenhum consumidor da UI lê o campo; a 1.0 mantém só o detector `click` do diagnóstico.

Nenhum número de parâmetro foi inventado: todos vêm do código do MVP no pin.

## Revisão independente

Subagente Opus com contexto limpo (alto risco: define arquitetura de uma ferramenta de áudio) auditou:

- `docs/architecture/adr/ADR-013-vynamp-restore.md`
- `docs/tools/01-vynamp-restore.md` e `docs/tools/02-vynamp-converter.md`
- `tasks/epics/E18.md`, `tasks/items/TASK-E18-01.md`, `TASK-E18-02.md` e `tasks/backlog.json`
- `docs/product/03-requirements.md`, `docs/INDEX.md`, `docs/GLOSSARY.md`, `docs/product/06-feature-parity.md` e `CHANGELOG.md`

**Veredito e achados:** ver a seção de revisão ao final deste arquivo.

## Revisão independente e achados aplicados

Subagente Opus com contexto limpo, sobre o commit `0bd9258` (base `64b7487`): veredito **APROVADO COM RESSALVAS**, com 9 achados. Todos os 9 foram **verificados um a um contra o código do MVP rev.38** antes de serem aplicados — nenhum foi aceito deforma cega — e todos foram corrigidos neste branch. Onde o achado propunha divergência do protótipo, a decisão foi **manter a paridade e declarar a ressalva**, nunca divergir em silêncio.

| # | Achado | Decisão aplicada |
|---|---|---|
| 1 | **[BLOQUEANTE]** Ordem de exportação com `--overwrite` perdia a origem: o `rename` substituía o original **antes** da validação, então uma falha de validação deixava o usuário sem saída e sem original | Ordem invertida **nas duas specs**: temporário no mesmo volume → `fsync` → **validação pós-escrita no temporário** → `rename`. Em falha, o temporário é removido e a origem permanece intacta. Digitação "meso volume" → "mesmo volume" corrigida |
| 2 | **[BLOQUEANTE]** `band_limit` atribuía corte de banda a codec *lossless* (FLAC/ALAC/WAV não cortam banda) e omitia o `deartifact: 0.5` do MVP | Atribuição corrigida para compressão **com perda** (MP3/AAC) e correção alinhada ao MVP: `bwe: 0.5` + `bweCutoff` + `deartifact: 0.5` (`restore-diagnostics.js:759`) |
| 3 | **[IMPORTANTE]** Ordem acústica invertida: DC e de-click depois do AR e da STFT | Ordem do MVP **mantida** (paridade), mas o §3.1 agora **declara** que ela é artefato de partição do `this.pre` do JS e não decisão acústica, registra o viés `+ μ²` na autocorrelação e a corrupção do frame AR por cliques, e declara que mover DC/de-click antes do estágio 1 exigiria divergência explícita em ADR-013 e revalidação com fixtures novas |
| 4 | **[IMPORTANTE]** Passa-baixa Kaiser travado em 18 kHz destrói banda de hi-res (96/192 kHz), sem parâmetro de corte | Parâmetro `decrackleCutoff` (8000–22000 Hz, padrão 18000) exposto na §3.2, com regra `min(corte, 0.4·fs)`; comportamento declarado na §3.2, com orientação para elevar o corte ou desligar o de-crackle em masterizações que precisem preservar a banda |
| 5 | **[IMPORTANTE]** Quarta divergência não declarada: os 5 presets dependiam do denoise Wiener removido e o aviso de 8 kHz citava "redução de ruído alta" | Tabela de `tools` exatos dos 5 presets na §3.3; aviso de 8 kHz re-chaveado para zumbido ativo / passa-baixa agressivo. Além disso, verificado no MVP que `restore-click-detector.js` produz `report.clickDetection` sem nenhum consumidor da UI — declarado como **quarta** divergência na ADR-013 §4 e nota na §4.2 da spec |
| 6 | **[IMPORTANTE]** `clipping` com correção estática `declip: −2`, desacordo com o cálculo dinâmico do MVP | Corrigido para `declip: <headroomDB>`, com `headroomDB = −min(12, max(1, ⌈pico dBFS + 1⌉))` e `decrackle: 0.15` acrescentado em `moderate`/`severe` (`restore-diagnostics.js:389,612`). O valor fixo `−2` permanece, como no MVP, só no detector `true_peak` (`:626`) |
| 7 | **[MENOR]** Justificativa acústica indefensável para a precedência do gate sobre a anti-saturação | Reescrita: o gate vem antes para que a atenuação de ganho do declip não descalibre a detecção de envelope do limiar de ruído |
| 8 | **[MENOR]** Detecção de crepitação indefinida em mono e surround | Especificado: mono opera no canal único, estéreo usa mid/side, surround (≥ 3 canais) opera **por canal**, sem par side, com contagem somada |
| 9 | **[MENOR]** IDs de QA divergentes entre TASKs, specs e `backlog.json` | Quatro fontes alinhadas em `QA-015, QA-016, QA-023, QA-024, QA-031` — o Converter também é app de instância única com IPC, então QA-024 (owner/instance lock) aplica-se aos dois |

Verificações reexecutadas **após** as correções: `cargo xtask check-docs` → `passed`, 0 erros, 338 Markdown; `cargo xtask check-delivery` → `passed`, 0 erros; `cargo test -p xtask` → 85 aprovados, 0 falhas.

## Limitações

- **CI Gate (Woodpecker) não verificado** — a instrução vigente do mantenedor é ignorar o CI/CD neste momento; o mantenedor confere o CI Gate do SHA final antes do merge.
- **Nenhuma restauração, conversão ou codificação foi executada.** A validação é documental e estrutural.
- **macOS não verificado** (sem agente Mac neste ambiente) e **hardware de saída de áudio não testado**.
- O **PR5** (execução: lib `vynamp-encode`, fila de jobs, artefatos de release) está fora deste PR; este entrega o escopo fechado.
- A especificação do Converter é **preparatória**: os documentos de contrato final de `vynamp-encode` e a evidência de execução ficam no PR5.

## Artefato

O corpo do PR está versionado em [pr-body.md](pr-body.md).
