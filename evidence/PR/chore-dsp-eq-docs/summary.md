# Evidência chore-dsp-eq-docs

Versão 0.7.1 · Data: 2026-10-28

## Escopo

Branch `chore/claude-dsp-eq-docs`, três commits (dd22d48 DSP v1.3, abf7274 DSP v1.4, 9399ca4 letras) mais um commit de correções da revisão Opus. Três blocos integrados:

- **(a) DSP v1.3** — entrada de 2026-10-25 em [CHANGELOG](../../../CHANGELOG.md).
- **(b) DSP v1.4** — DSP-UX-01..21, DSP-UX-AC-01..12, VU-INT-01..06, grafo modular e decisões D-DSP-11..18 ([product/05](../../../product/05-decisoes.md)).
- **(c) Letras** — LYR-01..15, LYR-AC-01..12, D-LYR-01..09, seção [library/02](../../../library/02-tags-letras.md#letras-componente-resumido-sob-o-vu-e-fontes) e [TASK-E03-09](../../../apps/player/tasks/items/TASK-E03-09.md).

Fora do git, na raiz do projeto: SKIN-REQUIREMENTS-GPT-MVP 1.3 e CODEX 1.2.

O protótipo HTML/SVG/JS é referência externa (rev. 14/15): evidência relatada, não verificada neste repositório. Nada nativo implementado.

## Conformidade EQ-10 / EQ-20 / EQ-31 / EQ-64 / PEQ32

Critério: Prompt_Claude_Documentacao_DSP_Equalizadores_Obrigatorios_v1.4.md §11 (fora do git). Fontes: [08-equalizers §1, §4–§7, §10 e §14](../../../reference/audio/08-equalizers.md). Estado: **COMPLETA** pelo §11 (cada linha tem caminho/seção, estado no MVP visual, marco nativo e testes planejados). A ausência de validação relatada do protótipo para EQ-10/20/31 é fato registrado em Pendências, não omissão.

| Modo | Especificado (doc/seção) | Estado no MVP visual ([08 §15](../../../reference/audio/08-equalizers.md#15-roadmap-mvp-visual--engine-nativa)) | Protótipo implementado (relatado) | Protótipo validado (relatado) | Nativo / fase (hoje: nada) | Testes planejados ([08 §16](../../../reference/audio/08-equalizers.md#16-testes-e-aceite-por-modalidade)) |
|---|---|---|---|---|---|---|
| EQ-10 | [§1](../../../reference/audio/08-equalizers.md#1-matriz-mandatória), [§4](../../../reference/audio/08-equalizers.md#4-eq-10--clássico-e-avançado-10) | Obrigatório agora | Relatado: EQ10 em centros clássicos no grafo; adaptação de áudio web no editor anterior ([§14](../../../reference/audio/08-equalizers.md#status-da-revisão-15-2026-10-28)) | Sem validação relatada específica do EQ-10 | Clássico M1 (TASK-E04-01); avançado M3 (TASK-E02-07) | EQ-T-01..06, 08..10, 12..14 ([§16](../../../reference/audio/08-equalizers.md#16-testes-e-aceite-por-modalidade)) |
| EQ-20 | [§1](../../../reference/audio/08-equalizers.md#1-matriz-mandatória), [§5](../../../reference/audio/08-equalizers.md#5-eq-20--meia-oitava) | Obrigatório agora | Relatado: biquads peaking em série; distribuição logarítmica 20 Hz–20 kHz ([§14](../../../reference/audio/08-equalizers.md#status-da-revisão-15-2026-10-28)) | Sem validação relatada específica do EQ-20 | M3 (TASK-E04-04) | EQ-T-01..06, 08..10, 12..14 ([§16](../../../reference/audio/08-equalizers.md#16-testes-e-aceite-por-modalidade)) |
| EQ-31 | [§1](../../../reference/audio/08-equalizers.md#1-matriz-mandatória), [§6](../../../reference/audio/08-equalizers.md#6-eq-31--13-de-oitava) | Obrigatório agora | Relatado: 1/3 de oitava no grafo ([§14](../../../reference/audio/08-equalizers.md#status-da-revisão-15-2026-10-28)) | Sem validação relatada específica do EQ-31 | M3 (TASK-E04-04) | EQ-T-01..06, 08..10, 12..14 ([§16](../../../reference/audio/08-equalizers.md#16-testes-e-aceite-por-modalidade)) |
| EQ-64 | [§1](../../../reference/audio/08-equalizers.md#1-matriz-mandatória), [§7](../../../reference/audio/08-equalizers.md#7-eq-64--logarítmica-densa-não-é-13-de-oitava) | Obrigatório agora (64 controles reais) | Relatado: 64 controles reais no grafo, com áudio real | Relatado: render offline com EQ64 e WAV estéreo (`modular-audio-validation`, `modular-live-validation`); edição de 64 bandas (`modular-editing-validation`) | M5 (TASK-E12-10) | EQ-T-01..06, 08..10, 12..15 ([§16](../../../reference/audio/08-equalizers.md#16-testes-e-aceite-por-modalidade)) |
| PEQ32 | [§1](../../../reference/audio/08-equalizers.md#1-matriz-mandatória), [§10](../../../reference/audio/08-equalizers.md#10-eq-peq32--paramétrico-até-32-filtros) | Obrigatório agora (≤ 32 filtros) | Relatado: 32 filtros reais (peaking, shelves, passes, notch, bandpass); sem seletor de slope | Relatado: PEQ por canal em render offline; 32 filtros (`modular-audio-validation`, `modular-editing-validation`) | M3 (TASK-E04-04); 64 filtros em M5 | EQ-T-01, 04..10, 12..15 (EQ-T-07 específico) ([§16](../../../reference/audio/08-equalizers.md#16-testes-e-aceite-por-modalidade)) |

Testes planejados: IDs por modo na coluna acima (a coluna "Modos" de 08 §16 define o escopo); ver também [matriz §2.2](../../../quality/04-matriz-dsp.md#22-grafo-modular-e-vu-int).

## Mapa de IDs → doc/seção

Contagens encontradas em `docs/` e `tasks/`: DSP-UX 21; DSP-UX-AC 12; VU-INT 6; LYR 15; LYR-AC 12. Extras: D-DSP-01..18 (11..18 são novos) e D-LYR 9.

| Família | Contagem | Doc/seção | Link |
|---|---|---|---|
| DSP-UX-01..05 | 5 | design/05 §8.1 Acesso visível e navegação por famílias | [§8.1](../../../design/06-painel-dsp.md#81-acesso-visível-e-navegação-por-famílias) |
| DSP-UX-06..10 | 5 | design/05 §8.2 Escolha do controle conforme o parâmetro | [§8.2](../../../design/06-painel-dsp.md#82-escolha-do-controle-conforme-o-parâmetro) |
| DSP-UX-11..17 | 7 | design/05 §8.3 Interação, precisão e validação | [§8.3](../../../design/06-painel-dsp.md#83-interação-precisão-e-validação) |
| DSP-UX-18..21 | 4 | design/05 §8.4 Hierarquia, acessibilidade e feedback | [§8.4](../../../design/06-painel-dsp.md#84-hierarquia-acessibilidade-e-feedback) |
| DSP-UX-AC-01..12 | 12 | quality/05 §2.1 Aceite de acesso e controles | [§2.1](../../../quality/04-matriz-dsp.md#21-aceite-de-acesso-e-controles-dsp-ux-ac) |
| VU-INT-01..06 | 6 | design/05 §6.1 Integração visual dos VU meters; testes em quality/05 §2.2 | [§6.1](../../../design/06-painel-dsp.md#61-integração-visual-dos-vu-meters-vu-int), [§2.2](../../../quality/04-matriz-dsp.md#22-grafo-modular-e-vu-int) |
| Grafo modular (sem ID) | — | design/05 §9; audio/03 "Grafo modular: protótipo × nativo" | [§9](../../../design/06-painel-dsp.md#9-fluxo-modular-grafo--referência-do-protótipo), [03-dsp](../../../reference/audio/03-dsp.md#grafo-modular-protótipo--nativo) |
| LYR-01..06 | 6 | library/02 Componente resumido sob o VU | [seção](../../../library/02-tags-letras.md#componente-resumido-sob-o-vu) |
| LYR-07..10 | 4 | library/02 Fontes, metadados e sincronização | [seção](../../../library/02-tags-letras.md#fontes-metadados-e-sincronização) |
| LYR-11..15 | 5 | library/02 Estados, persistência e concorrência | [seção](../../../library/02-tags-letras.md#estados-persistência-e-concorrência) |
| LYR-AC-01..12 | 12 | library/02 Critérios de aceite | [seção](../../../library/02-tags-letras.md#critérios-de-aceite) |
| D-DSP-11..18 | 8 | product/05 Decisões DSP e equalizadores (2026-10-25) | [seção](../../../product/05-decisoes.md#decisões-dsp-e-equalizadores-2026-10-25) |
| D-LYR-01..09 | 9 | product/05 Decisões de letras (2026-10-28) | [seção](../../../product/05-decisoes.md#decisões-de-letras-2026-10-28) |

## Pendências

- PR para develop (Gitflow): pendente de abertura.
- O mapa de IDs DSP-UX/VU-INT/D-DSP-11..18 deste resumo entrou no commit de letras (9399ca4) porque o resumo é único.
- CI Gate (Woodpecker): pendente, não executado para esta branch.
- Ausência de validação relatada específica para EQ-10, EQ-20 e EQ-31 no protótipo: fato registrado, não omissão da documentação; não corrigível só pela documentação.
- Decisões abertas em product/05: D-DSP-05, D-DSP-06, D-DSP-08, D-DSP-12.
- Divergência de versão (esperada): cada documento sobe um patch por edição, por isso 0.7.1, 0.7.2 e 0.7.3 coexistem. Cabeçalhos atuais dos arquivos modificados: 0.7.3 em docs/GLOSSARY.md e docs/product/05-decisions.md; 0.7.2 em docs/audio/03-dsp.md, 08-equalizers.md, 09-dsp-catalog.md, 10-vitrola.md, docs/design/02-layouts.md, docs/design/05-dsp-panel.md e docs/quality/05-dsp-test-matrix.md; 0.7.1 em CHANGELOG.md, docs/INDEX.md, este resumo, docs/audio/01-engine.md, docs/audio/07-dsp-signal-contract.md, docs/delivery/01-roadmap.md, docs/delivery/03-risks-open-questions.md, docs/design/01-visual-system.md, docs/design/03-accessibility-i18n.md, docs/library/02-tags-lyrics-tools.md, docs/product/03-requirements.md, docs/product/06-feature-parity.md, docs/quality/01-test-strategy.md, docs/quality/04-integration-codecs-dsp.md e tasks/items/TASK-E03-09.md.
