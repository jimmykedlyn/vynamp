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
| 2026-11-07 | `docs/audio/05-platform-backends.md` | `libs/output (spec, passo 35)` | fundida em audio/04-outputs-hires; 9 linhas identicas | 30 | pendente |
| 2026-11-07 | `docs/quality/04-integration-codecs-dsp.md` | `quality/01-estrategia-testes.md` | fundida; QA-022..033 byte-identicas para quality/01 | 30 | pendente |
| 2026-11-07 | `docs/streaming/04-api-semantics.md` | `architecture/04-contratos.md` | fundida na secao Provider de architecture/04 | 30 | pendente |
| 2026-11-07 | `docs/engineering/06-installers.md` | `delivery/04-gates.md` | fundida; mesmos instaladores | 30 | pendente |
| 2026-11-07 | `docs/extensions/03-visualizations-integrations.md` | `extensions/05-visualizador.md` | fundida; era subconjunto | 30 | pendente |
| 2026-11-07 | `docs/engineering/01-agents-superpowers.md` | `AGENTS.md secao 5` | absorvida por M5 | 30 | pendente |
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
| 2026-11-07 | `repo/AGENTS.md` | `AGENTS.md do umbrella` | M5: absorvido no passo 32 | 30 | pendente |
| 2026-11-07 | `repo/CLAUDE.md` | `CLAUDE.md do umbrella` | M5: absorvido no passo 32 | 30 | pendente |
| 2026-11-07 | `AGENTS.md (raiz externa)` | `AGENTS.md do umbrella` | M5: absorvido no passo 32 | 30 | pendente |
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
