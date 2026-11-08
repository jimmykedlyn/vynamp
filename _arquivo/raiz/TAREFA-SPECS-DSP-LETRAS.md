# Tarefa: specs DSP v1.4 + letras (só documentação)

Worktree: `repo\.worktrees\claude\chore-dsp-eq-docs` (branch `chore/claude-dsp-eq-docs`). Ela já tem, **sem commit**, a integração do pedido DSP v1.3: `docs/audio/03,07,08,09,10`, `docs/design/05-dsp-panel.md`, `docs/quality/05-dsp-test-matrix.md`, `docs/product/05-decisions.md` (D-DSP-01..10) e outros. Preserve tudo.

## Esquema de agentes (por etapa)
1. **Opus (esta sessão) orquestra.** Lê só este arquivo e os títulos (`grep -n "^#"`) dos docs-alvo. Não edita docs; escreve os prompts.
2. **Sonnet em paralelo**, cada um com um conjunto de arquivos **disjunto** (listado na etapa) e as âncoras exatas abaixo. Prompt com o contexto mínimo; retorno ≤ 20 linhas.
3. **Haiku** confere, devolvendo só PASSOU/FALHOU com a lista de problemas:
   - só arquivos `.md` foram alterados;
   - nenhum arquivo fora da lista;
   - links relativos e âncoras resolvem;
   - IDs presentes (contagem);
   - cabeçalhos com versão/data.
4. **Opus revisor com contexto limpo** (nunca o implementador) compara com a fonte: omissões, contradições, alegação de implementação nativa e protótipo tratado como verificado. Os achados voltam a um Sonnet para correção.
5. **Fim da etapa:** resumo curto ao usuário (arquivos, IDs, conflitos, pendências) e o **comando PowerShell da próxima etapa**. Parar.

## Regras
- Só Markdown. Nada de código, schemas, CI ou build. Português brasileiro, no estilo dos docs existentes. Ao editar um doc, suba a versão de patch e use a data do dia.
- O protótipo HTML/SVG/JS é do Codex, **fora do repo**. `interactive/*`, `*-validation.json` e similares não existem aqui. Cite-os como "evidência relatada da referência externa rev. 14/15, não verificada neste repositório". Separe sempre: especificado · implementado no protótipo (relatado) · validado no protótipo (relatado) · nativo (hoje: nada).
- Agentes não fazem commit. Commits e PR só na Etapa 4.

## Etapa 1 — DSP v1.4 no repo
Fonte: `Prompt_Claude_Documentacao_DSP_Equalizadores_Obrigatorios_v1.4.md`, apenas §5.1 (DSP-UX-01..21), §9.1 (DSP-UX-AC-01..12) e §12 (rev. 15). O restante já foi integrado na v1.3.

**Sonnet A — UI.** Arquivos: `design/05-dsp-panel.md`, `design/01-visual-system.md`, `GLOSSARY.md`.
- Cabeçalho do 05-dsp-panel: o protótipo agora tem UI DSP, segundo a referência.
- As 7 categorias da navegação de §1/§2 dão lugar às oito famílias da v1.4. Medidores sai das famílias e vira área própria.
- Entrada "Áudio e efeitos" e os quatro atalhos.
- Regra de controle: knob/slider/chave/campo + readout clicável.
- Títulos exatos: `### 6.1 Integração visual dos VU meters (VU-INT)` (VU-INT-01..06), `## 8. Acesso e controles (DSP-UX)` e `## 9. Fluxo modular (grafo) — referência do protótipo`.
- Tokens: knob (referência 88×88 DIP, arco de 270°) e acabamento de VU.
- Glossário: knob, readout, grafo modular, porta CV, acabamento de VU.

**Sonnet B — áudio, testes e decisões.** Arquivos: `audio/03-dsp.md`, `audio/08-equalizers.md`, `audio/09-dsp-catalog.md`, `audio/10-vitrola.md`, `quality/05-dsp-test-matrix.md`, `product/05-decisions.md`.
- 03: nova seção `## Grafo modular: protótipo × nativo`. O grafo é contrato do protótipo. No nativo, a reordenação livre continua não prometida (D-DSP-10) e fica como decisão aberta.
- 08 §14: status da rev. 15 (EQ e PEQ com áudio real dentro do grafo). EQ-64 nativo continua M5.
- 09: família de cada módulo e estado relatado no protótipo. O Limiter amostral não é True Peak e a Saturação não é o Tube.
- 10: o atalho Vitrola leva à área existente. No protótipo, Vinyl e Tube têm configuração sem processamento. Não mudar fase.
- quality/05: títulos `### 2.1 Aceite de acesso e controles (DSP-UX-AC)` e `### 2.2 Grafo modular e VU-INT`. Manter explícitas as duas regressões (não quantizar preset válido; não aceitar valor rejeitado).
- 05-decisions: D-DSP-11 em diante, mais as divergências (ex.: 7 categorias × 8 famílias; grafo livre × D-DSP-10).

## Etapa 2 — Letras no repo ‖ SKIN (parte DSP)
**Sonnet C — letras no repo.** Fonte: `CLAUDE-SPEC-LETRAS-SINCRONIZADAS.md` (LYR-01..15, LYR-AC-01..12). Arquivos: `library/02-tags-lyrics-tools.md`, `tasks/items/TASK-E03-09.md`, `product/03-requirements.md`, `design/02-layouts.md`, `design/03-accessibility-i18n.md`, mais trechos de letras em `05-decisions` e `GLOSSARY`.
- library/02: nova seção `## Letras: componente resumido sob o VU e fontes`.
- TASK-E03-09: depende de TASK-E03-04 e do vynamp-decode plano 04 (metadados). A tabela de famílias de tags é baseline relatada, não suporte nativo. Não mudar fase/prioridade.

**Sonnet D — SKIN, parte DSP.** Arquivos na raiz, fora do git; fazer backup no scratchpad antes.
- GPT-MVP passa a 1.3 e CODEX a 1.2.
- Integrar DSP-UX, VU-INT, grafo e a tabela de capacidades 12.3, referenciando as âncoras da Etapa 1.
- Unificar o rótulo `[DEMO — no audio processing in this mode]`.
- CODEX recebe só a parte de design.

## Etapa 3 — SKIN (letras) ‖ fechamento documental
**Sonnet E — SKIN, letras.** §3/§4/§5.13/§5.16/§5.18/§8/§9/§12/§13 e MVP-09, referenciando library/02.

**Haiku F — fechamento.** `INDEX.md`, `CHANGELOG.md` e `docs/evidence/PR/chore-dsp-eq-docs/summary.md`. O resumo leva:
- a tabela de conformidade EQ-10/20/31/64/PEQ32 (v1.4 §11; se faltar uma linha, declarar INCOMPLETA);
- o mapa de IDs DSP-UX/VU-INT/LYR → doc/seção.

## Etapa 4 — Commits, revisão final e PR
- Três commits separados (v1.3, v1.4, letras).
- Opus revisor de contexto limpo sobre o diff inteiro.
- Publicar a branch e abrir o PR (template, `--body-file`). Precisa de `Invoke-VynampAuthenticatedAgent`.
- CI Gate verde no SHA final.
- Atualizar `STATE.md` e entregar URL, SHA, status e "aprove e faça o merge commit pelo GitHub".
