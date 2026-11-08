# _arquivo — indice de arquivamento

Toda remocao entra aqui antes de acontecer, com o motivo. A coluna **Estado**
separa o que ja foi feito do que ainda nao foi:

- `arquivado` — a origem nao existe mais no repositorio; o arquivo esta em `_arquivo/`.
- `pendente` — a linha registra a **decisao** de fundir ou absorver, e os dois lados
  continuam vivos. Nada foi movido ainda; a fusao e a remocao acontecem no passo
  anotado em **Passo**, e so o passo 38 apaga de verdade.

A exclusao definitiva so acontece no passo 38, sob confirmacao explicita do mantenedor.

| Data | Origem | Destino | Motivo | Passo | Estado |
|---|---|---|---|---|---|
| 2026-11-07 | `docs/architecture/adr/ADR-010-woodpecker-only-ci.md` | `_arquivo/repo/docs/architecture/adr/ADR-010-woodpecker-only-ci.md` | M6: Woodpecker removido por inteiro | 30 | arquivado |
| 2026-11-07 | `docs/engineering/08-folder-structure.md` | `_arquivo/repo/docs/engineering/08-folder-structure.md` | descreve raiz de ZIP que nao existe mais | 30 | arquivado |
| 2026-11-07 | `docs/product/08-completeness-audit.md` | `_arquivo/repo/docs/product/08-completeness-audit.md` | auditoria historica 0.6, superada | 30 | arquivado |
| 2026-11-07 | `docs/research/03-completeness-sources.md` | `_arquivo/repo/docs/research/03-completeness-sources.md` | fontes da auditoria 0.6, superadas | 30 | arquivado |
| 2026-11-07 | `START-HERE.md` | `_arquivo/repo/START-HERE.md` | descreve bootstrap de ZIP e INICIAR.ps1 inexistentes | 30 | arquivado |
| 2026-11-07 | `ARTIFACTS.json` | `_arquivo/repo/ARTIFACTS.json` | manifesto de bundle de outro produto | 30 | arquivado |
| 2026-11-07 | `VALIDATION.json` | `_arquivo/repo/VALIDATION.json` | saida gerada por xtask validate; passa a ser gitignored | 30 | arquivado |
| 2026-11-07 | `docs/audio/05-platform-backends.md` | `libs/output (spec, passo 35)` | nao e duplicata de audio/04: medido em 0 linhas comuns; vira spec de backends do output, agora em reference/audio/ | 30 | pendente |
| 2026-11-07 | `docs/quality/04-integration-codecs-dsp.md` | `quality/01-estrategia-testes.md` | fundida; QA-022..033 byte-identicas para quality/01 | 30 | pendente |
| 2026-11-07 | `docs/streaming/04-api-semantics.md` | `architecture/04-contratos.md` | fundida na secao Provider de architecture/04 | 30 | pendente |
| 2026-11-07 | `docs/engineering/06-installers.md` | `delivery/04-gates.md` | fundida; mesmos instaladores | 30 | pendente |
| 2026-11-07 | `docs/extensions/03-visualizations-integrations.md` | `extensions/05-visualizador.md` | fundida; era subconjunto | 30 | pendente |
| 2026-11-07 | `docs/engineering/01-agents-superpowers.md` | `_arquivo/repo/docs/engineering/01-agents-superpowers.md` | M5: absorvida no contrato unico | 32 | arquivado |
| 2026-11-07 | `docs/audio/01-engine.md` | `libs/engine (spec, passo 35)` | absorvida por M3 | 30 | pendente |
| 2026-11-07 | `docs/audio/02-formats.md` | `libs/decode + libs/encode` | absorvida por M3 | 30 | pendente |
| 2026-11-07 | `docs/audio/03-dsp.md` | `libs/engine` | absorvida por M3; resampler vai a dsp (M9) | 30 | pendente |
| 2026-11-07 | `docs/audio/04-outputs-hires.md` | `libs/output` | absorvida por M3 | 30 | pendente |
| 2026-11-07 | `docs/audio/06-codec-capability-registry.md` | `libs/decode + libs/encode` | absorvida por M3 | 30 | pendente |
| 2026-11-07 | `docs/audio/07-dsp-signal-contract.md` | `libs/engine` | absorvida por M3 | 30 | pendente |
| 2026-11-07 | `docs/audio/08-equalizers.md` | `libs/engine` | absorvida por M3 | 30 | pendente |
| 2026-11-07 | `docs/audio/09-dsp-catalog.md` | `libs/engine` | absorvida por M3 | 30 | pendente |
| 2026-11-07 | `docs/audio/10-vitrola.md` | `libs/engine` | M10: vinil/tube interno; sem repo vitrola | 30 | pendente |
| 2026-11-07 | `docs/tools/01-vynamp-restore.md` | `apps/restore` | absorvida por M3 | 30 | pendente |
| 2026-11-07 | `docs/tools/02-vynamp-converter.md` | `apps/converter` | absorvida por M3 | 30 | pendente |
| 2026-11-07 | `repo/AGENTS.md` | `_arquivo/repo/AGENTS.md` | M5: absorvido no contrato unico | 32 | arquivado |
| 2026-11-07 | `repo/CLAUDE.md` | `_arquivo/repo/CLAUDE.md` | M5: CLAUDE.md passa a ser so @AGENTS.md | 32 | arquivado |
| 2026-11-07 | `AGENTS.md (raiz externa)` | `_arquivo/raiz/AGENTS.md` | M5: segunda autoridade conflitante, absorvida | 32 | arquivado |
| 2026-11-07 | `LEIA-ME.md (raiz externa)` | `README.md do umbrella` | absorvido no passo 31 | 30 | pendente |
| 2026-11-07 | `docs/evidence/PR/chore-woodpecker-3-20-1/README.md` | `_arquivo/repo/docs/evidence/PR/chore-woodpecker-3-20-1/README.md` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/README.md` | `_arquivo/repo/docs/evidence/PR/task-e15-04/README.md` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/github-status-71f7086.json` | `_arquivo/repo/docs/evidence/PR/task-e15-04/github-status-71f7086.json` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/github-status-fda34b0.json` | `_arquivo/repo/docs/evidence/PR/task-e15-04/github-status-fda34b0.json` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/local-exitcode-probe.txt` | `_arquivo/repo/docs/evidence/PR/task-e15-04/local-exitcode-probe.txt` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-1-linux-clone.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-1-linux-clone.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-1-linux-tooling.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-1-linux-tooling.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-1-windows-clone.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-1-windows-clone.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-1-windows-tooling.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-1-windows-tooling.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-1.json` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-1.json` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-10-android-android.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-10-android-android.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-10-android-clone.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-10-android-clone.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-10-gate-gate.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-10-gate-gate.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-10-linux-clone.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-10-linux-clone.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-10-linux-tooling.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-10-linux-tooling.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-10-windows-clone.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-10-windows-clone.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-10-windows-tooling.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-10-windows-tooling.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-10.json` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-10.json` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-11-android-android.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-11-android-android.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-11-android-clone.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-11-android-clone.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-11-linux-clone.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-11-linux-clone.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-11-linux-tooling.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-11-linux-tooling.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-11-windows-clone.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-11-windows-clone.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-11-windows-tooling.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-11-windows-tooling.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-11.json` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-11.json` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-2-linux-clone.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-2-linux-clone.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-2-linux-tooling.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-2-linux-tooling.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-2-windows-clone.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-2-windows-clone.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-2-windows-tooling.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-2-windows-tooling.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-2.json` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-2.json` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-3-linux-clone.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-3-linux-clone.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-3-linux-tooling.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-3-linux-tooling.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-3-windows-clone.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-3-windows-clone.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-3-windows-tooling.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-3-windows-tooling.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-3.json` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-3.json` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-4-linux-clone.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-4-linux-clone.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-4-linux-tooling.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-4-linux-tooling.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-4-windows-clone.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-4-windows-clone.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-4-windows-tooling.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-4-windows-tooling.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-4.json` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-4.json` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-5-linux-clone.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-5-linux-clone.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-5-linux-tooling.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-5-linux-tooling.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-5-windows-clone.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-5-windows-clone.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-5-windows-tooling.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-5-windows-tooling.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-5.json` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-5.json` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-6-linux-clone.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-6-linux-clone.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-6-linux-tooling.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-6-linux-tooling.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-6-windows-clone.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-6-windows-clone.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-6-windows-tooling.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-6-windows-tooling.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-6.json` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-6.json` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-9-android-android.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-9-android-android.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-9-android-clone.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-9-android-clone.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-9-gate-gate.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-9-gate-gate.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-9-linux-clone.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-9-linux-clone.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-9-linux-tooling.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-9-linux-tooling.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-9-windows-clone.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-9-windows-clone.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-9-windows-tooling.log` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-9-windows-tooling.log` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/evidence/PR/task-e15-04/pipeline-9.json` | `_arquivo/repo/docs/evidence/PR/task-e15-04/pipeline-9.json` | M6: evidencia de CI morto (Woodpecker) | 30 | arquivado |
| 2026-11-07 | `docs/superpowers/plans/2026-10-15-e15-01-verification.md` | `_arquivo/repo/docs/superpowers/plans/2026-10-15-e15-01-verification.md` | plano de execucao ja concluido | 30 | arquivado |
| 2026-11-07 | `docs/superpowers/plans/2026-10-16-e15-02-isolation.md` | `_arquivo/repo/docs/superpowers/plans/2026-10-16-e15-02-isolation.md` | plano de execucao ja concluido | 30 | arquivado |
| 2026-11-07 | `docs/superpowers/plans/2026-10-18-e11-01-workspace.md` | `_arquivo/repo/docs/superpowers/plans/2026-10-18-e11-01-workspace.md` | plano de execucao ja concluido | 30 | arquivado |
| 2026-11-07 | `docs/superpowers/plans/2026-10-18-e15-03-gitflow.md` | `_arquivo/repo/docs/superpowers/plans/2026-10-18-e15-03-gitflow.md` | plano de execucao ja concluido | 30 | arquivado |
| 2026-11-07 | `docs/superpowers/plans/2026-11-05-specs-shell-integration.md` | `_arquivo/repo/docs/superpowers/plans/2026-11-05-specs-shell-integration.md` | plano de execucao ja concluido | 30 | arquivado |
| 2026-11-07 | `docs/superpowers/plans/2026-11-05-specs-vynamp-restore.md` | `_arquivo/repo/docs/superpowers/plans/2026-11-05-specs-vynamp-restore.md` | plano de execucao ja concluido | 30 | arquivado |
| 2026-11-07 | `apps/android/README.md` | `_arquivo/repo/apps/android/README.md` | placeholder; submodulo o substitui | 30 | arquivado |
| 2026-11-07 | `apps/desktop/README.md` | `_arquivo/repo/apps/desktop/README.md` | placeholder; submodulo o substitui | 30 | arquivado |
| 2026-11-07 | `apps/web/README.md` | `_arquivo/repo/apps/web/README.md` | placeholder; submodulo o substitui | 30 | arquivado |
| 2026-11-07 | `bootstrap/ABRIR-CLAUDE.ps1` | `_arquivo/repo/bootstrap/ABRIR-CLAUDE.ps1` | placeholder; submodulo o substitui | 30 | arquivado |
| 2026-11-07 | `bootstrap/AMBIENTE.ps1` | `_arquivo/repo/bootstrap/AMBIENTE.ps1` | placeholder; submodulo o substitui | 30 | arquivado |
| 2026-11-07 | `bootstrap/INICIAR.ps1` | `_arquivo/repo/bootstrap/INICIAR.ps1` | placeholder; submodulo o substitui | 30 | arquivado |
| 2026-11-07 | `bootstrap/LEIA-ME.txt` | `_arquivo/repo/bootstrap/LEIA-ME.txt` | placeholder; submodulo o substitui | 30 | arquivado |
| 2026-11-07 | `bootstrap/PROMPT-CLAUDE.txt` | `_arquivo/repo/bootstrap/PROMPT-CLAUDE.txt` | placeholder; submodulo o substitui | 30 | arquivado |
| 2026-11-07 | `bootstrap/README.md` | `_arquivo/repo/bootstrap/README.md` | placeholder; submodulo o substitui | 30 | arquivado |
| 2026-11-07 | `crates/README.md` | `_arquivo/repo/crates/README.md` | placeholder; submodulo o substitui | 30 | arquivado |
| 2026-11-07 | `assets/branding/reference/concepts-a-b-c-d.png` | `_arquivo/repo/assets/branding/reference/concepts-a-b-c-d.png` | conceito A/B/C/D, nao asset de produto | 30 | arquivado |
| 2026-11-07 | `assets/branding/reference/symbol-c-transparent.png` | `_arquivo/repo/assets/branding/reference/symbol-c-transparent.png` | conceito A/B/C/D, nao asset de produto | 30 | arquivado |
| 2026-11-07 | `raiz/CLAUDE-SPEC-LETRAS-SINCRONIZADAS.md` | `_arquivo/raiz/CLAUDE-SPEC-LETRAS-SINCRONIZADAS.md` | integrado em library/02 e D-LYR | 30 | arquivado |
| 2026-11-07 | `raiz/Prompt_Claude_Documentacao_DSP_Equalizadores_Obrigatorios_v1.4.md` | `_arquivo/raiz/Prompt_Claude_DSP_v1.4.md` | ja integrado em D-DSP-11..18, design/06, quality/04 | 30 | arquivado |
| 2026-11-07 | `raiz/ROADMAP-rodadas.md` | `_arquivo/raiz/ROADMAP-rodadas.md` | conceito morto; substituido por tasks/backlog.json | 30 | arquivado |
| 2026-11-07 | `raiz/SKIN-REQUIREMENTS-CODEX.md` | `_arquivo/raiz/SKIN-REQUIREMENTS-CODEX.md` | descartado por D-MVP-01; 87% subconjunto do GPT | 30 | arquivado |
| 2026-11-07 | `raiz/SKIN-REQUIREMENTS-GPT-MVP-OBRIGATORIO-v1.2.md` | `_arquivo/raiz/SKIN-REQUIREMENTS-GPT-MVP-OBRIGATORIO-v1.2.md` | descartado por D-MVP-01 | 30 | arquivado |
| 2026-11-07 | `raiz/TAREFA-SPECS-DSP-LETRAS.md` | `_arquivo/raiz/TAREFA-SPECS-DSP-LETRAS.md` | ordem de servico concluida | 30 | arquivado |
| 2026-11-07 | `raiz/TAREFA-SPECS-LIBS-MVP.md` | `_arquivo/raiz/TAREFA-SPECS-LIBS-MVP.md` | ordem de servico concluida | 30 | arquivado |
| 2026-11-07 | `raiz/AUDIOSR-RUST-CPU.md` | `_arquivo/raiz/AUDIOSR-RUST-CPU.md` | absorvido por ADR-015 + tools/03 em 2026-11-07 | 30 | arquivado |
| 2026-11-07 | `raiz/SOURCE_SEPARATION_MODELS.md` | `_arquivo/raiz/SOURCE_SEPARATION_MODELS.md` | absorvido por ADR-015 + tools/03 em 2026-11-07 | 30 | arquivado |
| 2026-11-08 | `.github/workflows/ci.yml` | `_arquivo/workflows/ci-antes-do-passo-34.yml` | versao anterior guardada antes da reescrita | 34 |
| 2026-11-08 | `.github/workflows/quality.yml` | `_arquivo/workflows/quality-antes-do-passo-34.yml` | versao anterior guardada antes da reescrita | 34 |
| 2026-11-08 | `.github/workflows/release.yml` | `_arquivo/workflows/release.yml` | D-CI-3: release e do app que publica; o umbrella nao tem binario | 34 |
| 2026-11-08 | `tasks/items/TASK-E01-01.md` | `repo/tasks/items/TASK-E01-01.md` | reescrito no formato padroes/tasks.md e movido para vynamp-decode (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E01-02.md` | `repo/tasks/items/TASK-E01-02.md` | reescrito no formato padroes/tasks.md e movido para vynamp-output (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E01-03.md` | `repo/tasks/items/TASK-E01-03.md` | reescrito no formato padroes/tasks.md e movido para vynamp-engine (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E01-04.md` | `repo/tasks/items/TASK-E01-04.md` | reescrito no formato padroes/tasks.md e movido para vynamp-decode (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E01-05.md` | `repo/tasks/items/TASK-E01-05.md` | reescrito no formato padroes/tasks.md e movido para vynamp-engine (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E01-06.md` | `repo/tasks/items/TASK-E01-06.md` | reescrito no formato padroes/tasks.md e movido para vynamp-engine (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E01-07.md` | `repo/tasks/items/TASK-E01-07.md` | reescrito no formato padroes/tasks.md e movido para vynamp-decode (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E01-08.md` | `repo/tasks/items/TASK-E01-08.md` | reescrito no formato padroes/tasks.md e movido para vynamp-decode (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E01-09.md` | `repo/tasks/items/TASK-E01-09.md` | reescrito no formato padroes/tasks.md e movido para vynamp-decode (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E01-10.md` | `repo/tasks/items/TASK-E01-10.md` | reescrito no formato padroes/tasks.md e movido para vynamp-engine (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E02-01.md` | `repo/tasks/items/TASK-E02-01.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E02-02.md` | `repo/tasks/items/TASK-E02-02.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E02-03.md` | `repo/tasks/items/TASK-E02-03.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E02-04.md` | `repo/tasks/items/TASK-E02-04.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E02-05.md` | `repo/tasks/items/TASK-E02-05.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E02-06.md` | `repo/tasks/items/TASK-E02-06.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E02-07.md` | `repo/tasks/items/TASK-E02-07.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E02-08.md` | `repo/tasks/items/TASK-E02-08.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E02-09.md` | `repo/tasks/items/TASK-E02-09.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E02-10.md` | `repo/tasks/items/TASK-E02-10.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E02-11.md` | `repo/tasks/items/TASK-E02-11.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E02-12.md` | `repo/tasks/items/TASK-E02-12.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E02-13.md` | `repo/tasks/items/TASK-E02-13.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E02-14.md` | `repo/tasks/items/TASK-E02-14.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E02-15.md` | `repo/tasks/items/TASK-E02-15.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E02-16.md` | `repo/tasks/items/TASK-E02-16.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E03-01.md` | `repo/tasks/items/TASK-E03-01.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E03-02.md` | `repo/tasks/items/TASK-E03-02.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E03-03.md` | `repo/tasks/items/TASK-E03-03.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E03-04.md` | `repo/tasks/items/TASK-E03-04.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E03-05.md` | `repo/tasks/items/TASK-E03-05.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E03-06.md` | `repo/tasks/items/TASK-E03-06.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E03-07.md` | `repo/tasks/items/TASK-E03-07.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E03-08.md` | `repo/tasks/items/TASK-E03-08.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E03-09.md` | `repo/tasks/items/TASK-E03-09.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E03-10.md` | `repo/tasks/items/TASK-E03-10.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E04-01.md` | `repo/tasks/items/TASK-E04-01.md` | reescrito no formato padroes/tasks.md e movido para vynamp-engine (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E04-02.md` | `repo/tasks/items/TASK-E04-02.md` | reescrito no formato padroes/tasks.md e movido para vynamp-dsp (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E04-03.md` | `repo/tasks/items/TASK-E04-03.md` | reescrito no formato padroes/tasks.md e movido para vynamp-engine (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E04-04.md` | `repo/tasks/items/TASK-E04-04.md` | reescrito no formato padroes/tasks.md e movido para vynamp-engine (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E04-05.md` | `repo/tasks/items/TASK-E04-05.md` | reescrito no formato padroes/tasks.md e movido para vynamp-output (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E04-06.md` | `repo/tasks/items/TASK-E04-06.md` | reescrito no formato padroes/tasks.md e movido para vynamp-output (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E04-07.md` | `repo/tasks/items/TASK-E04-07.md` | reescrito no formato padroes/tasks.md e movido para vynamp-engine (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E04-08.md` | `repo/tasks/items/TASK-E04-08.md` | reescrito no formato padroes/tasks.md e movido para vynamp-engine (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E04-09.md` | `repo/tasks/items/TASK-E04-09.md` | reescrito no formato padroes/tasks.md e movido para vynamp-output (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E05-01.md` | `repo/tasks/items/TASK-E05-01.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E05-02.md` | `repo/tasks/items/TASK-E05-02.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E05-03.md` | `repo/tasks/items/TASK-E05-03.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E05-04.md` | `repo/tasks/items/TASK-E05-04.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E05-05.md` | `repo/tasks/items/TASK-E05-05.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E05-06.md` | `repo/tasks/items/TASK-E05-06.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E05-07.md` | `repo/tasks/items/TASK-E05-07.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E05-08.md` | `repo/tasks/items/TASK-E05-08.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E05-09.md` | `repo/tasks/items/TASK-E05-09.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E05-10.md` | `repo/tasks/items/TASK-E05-10.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E06-01.md` | `repo/tasks/items/TASK-E06-01.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E06-02.md` | `repo/tasks/items/TASK-E06-02.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E06-03.md` | `repo/tasks/items/TASK-E06-03.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E06-04.md` | `repo/tasks/items/TASK-E06-04.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E06-05.md` | `repo/tasks/items/TASK-E06-05.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E06-06.md` | `repo/tasks/items/TASK-E06-06.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E06-07.md` | `repo/tasks/items/TASK-E06-07.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E06-08.md` | `repo/tasks/items/TASK-E06-08.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E06-09.md` | `repo/tasks/items/TASK-E06-09.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E06-10.md` | `repo/tasks/items/TASK-E06-10.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E07-01.md` | `repo/tasks/items/TASK-E07-01.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E07-02.md` | `repo/tasks/items/TASK-E07-02.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E07-03.md` | `repo/tasks/items/TASK-E07-03.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E07-04.md` | `repo/tasks/items/TASK-E07-04.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E07-05.md` | `repo/tasks/items/TASK-E07-05.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E07-06.md` | `repo/tasks/items/TASK-E07-06.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E07-07.md` | `repo/tasks/items/TASK-E07-07.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E07-08.md` | `repo/tasks/items/TASK-E07-08.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E07-09.md` | `repo/tasks/items/TASK-E07-09.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E07-10.md` | `repo/tasks/items/TASK-E07-10.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E08-01.md` | `repo/tasks/items/TASK-E08-01.md` | reescrito no formato padroes/tasks.md e movido para vynamp-engine (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E08-02.md` | `repo/tasks/items/TASK-E08-02.md` | reescrito no formato padroes/tasks.md e movido para vynamp-converter (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E08-03.md` | `repo/tasks/items/TASK-E08-03.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E08-04.md` | `repo/tasks/items/TASK-E08-04.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E08-05.md` | `repo/tasks/items/TASK-E08-05.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E08-06.md` | `repo/tasks/items/TASK-E08-06.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E08-07.md` | `repo/tasks/items/TASK-E08-07.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E08-08.md` | `repo/tasks/items/TASK-E08-08.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E08-09.md` | `repo/tasks/items/TASK-E08-09.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E08-10.md` | `repo/tasks/items/TASK-E08-10.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E12-01.md` | `repo/tasks/items/TASK-E12-01.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E12-02.md` | `repo/tasks/items/TASK-E12-02.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E12-04.md` | `repo/tasks/items/TASK-E12-04.md` | reescrito no formato padroes/tasks.md e movido para vynamp-output (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E12-05.md` | `repo/tasks/items/TASK-E12-05.md` | reescrito no formato padroes/tasks.md e movido para vynamp-decode (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E12-06.md` | `repo/tasks/items/TASK-E12-06.md` | reescrito no formato padroes/tasks.md e movido para vynamp-decode (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E12-07.md` | `repo/tasks/items/TASK-E12-07.md` | reescrito no formato padroes/tasks.md e movido para vynamp-engine (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E12-08.md` | `repo/tasks/items/TASK-E12-08.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E12-09.md` | `repo/tasks/items/TASK-E12-09.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E12-10.md` | `repo/tasks/items/TASK-E12-10.md` | reescrito no formato padroes/tasks.md e movido para vynamp-engine (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E13-01.md` | `repo/tasks/items/TASK-E13-01.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E13-02.md` | `repo/tasks/items/TASK-E13-02.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E13-03.md` | `repo/tasks/items/TASK-E13-03.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E13-04.md` | `repo/tasks/items/TASK-E13-04.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E13-05.md` | `repo/tasks/items/TASK-E13-05.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E13-06.md` | `repo/tasks/items/TASK-E13-06.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E13-07.md` | `repo/tasks/items/TASK-E13-07.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E13-08.md` | `repo/tasks/items/TASK-E13-08.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E13-09.md` | `repo/tasks/items/TASK-E13-09.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E13-10.md` | `repo/tasks/items/TASK-E13-10.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E16-01.md` | `repo/tasks/items/TASK-E16-01.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E16-02.md` | `repo/tasks/items/TASK-E16-02.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E16-03.md` | `repo/tasks/items/TASK-E16-03.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E16-04.md` | `repo/tasks/items/TASK-E16-04.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E16-05.md` | `repo/tasks/items/TASK-E16-05.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E16-06.md` | `repo/tasks/items/TASK-E16-06.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E16-07.md` | `repo/tasks/items/TASK-E16-07.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E16-08.md` | `repo/tasks/items/TASK-E16-08.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E16-09.md` | `repo/tasks/items/TASK-E16-09.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E16-10.md` | `repo/tasks/items/TASK-E16-10.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E17-01.md` | `repo/tasks/items/TASK-E17-01.md` | reescrito no formato padroes/tasks.md e movido para vynamp-decode (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E17-02.md` | `repo/tasks/items/TASK-E17-02.md` | reescrito no formato padroes/tasks.md e movido para vynamp-decode (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E17-03.md` | `repo/tasks/items/TASK-E17-03.md` | reescrito no formato padroes/tasks.md e movido para vynamp-decode (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E17-04.md` | `repo/tasks/items/TASK-E17-04.md` | reescrito no formato padroes/tasks.md e movido para vynamp-decode (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E17-05.md` | `repo/tasks/items/TASK-E17-05.md` | reescrito no formato padroes/tasks.md e movido para vynamp-decode (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E17-06.md` | `repo/tasks/items/TASK-E17-06.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E17-07.md` | `repo/tasks/items/TASK-E17-07.md` | reescrito no formato padroes/tasks.md e movido para vynamp-engine (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E17-09.md` | `repo/tasks/items/TASK-E17-09.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E17-10.md` | `repo/tasks/items/TASK-E17-10.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E17-11.md` | `repo/tasks/items/TASK-E17-11.md` | reescrito no formato padroes/tasks.md e movido para vynamp-player (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E17-13.md` | `repo/tasks/items/TASK-E17-13.md` | reescrito no formato padroes/tasks.md e movido para vynamp-decode (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E17-14.md` | `repo/tasks/items/TASK-E17-14.md` | reescrito no formato padroes/tasks.md e movido para vynamp-decode (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E17-15.md` | `repo/tasks/items/TASK-E17-15.md` | reescrito no formato padroes/tasks.md e movido para vynamp-decode (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E17-16.md` | `repo/tasks/items/TASK-E17-16.md` | reescrito no formato padroes/tasks.md e movido para vynamp-decode (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E17-17.md` | `repo/tasks/items/TASK-E17-17.md` | reescrito no formato padroes/tasks.md e movido para vynamp-decode (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E17-19.md` | `repo/tasks/items/TASK-E17-19.md` | reescrito no formato padroes/tasks.md e movido para vynamp-output (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E18-01.md` | `repo/tasks/items/TASK-E18-01.md` | reescrito no formato padroes/tasks.md e movido para vynamp-restore (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E18-02.md` | `repo/tasks/items/TASK-E18-02.md` | reescrito no formato padroes/tasks.md e movido para vynamp-converter (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E19-03.md` | `repo/tasks/items/TASK-E19-03.md` | reescrito no formato padroes/tasks.md e movido para vynamp-engine (passo 37) | 37 | arquivado |
| 2026-11-08 | `tasks/items/TASK-E19-07.md` | `repo/tasks/items/TASK-E19-07.md` | reescrito no formato padroes/tasks.md e movido para vynamp-engine (passo 37) | 37 | arquivado |
