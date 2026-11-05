# Pull request — Vynamp

## Resultado

Problema e comportamento final:

O Vynamp já tem, no papel, o **Vynamp Restore** e o **Vynamp Converter** como ferramentas desktop separadas ([ADR-012](../../../architecture/adr/ADR-012-shell-integration.md) §3), mas nada do conteúdo normativo existe: não há ADR que fixe a fronteira do processo, não há spec de CLI, DSP, diagnóstico, exportação ou formatos, não há épico no backlog e não há tarefa rastreável. O protótipo MVP web tem a interface e os algoritmos, mas é protótipo: tem código morto, um módulo inalcançável e um placeholder declarado como funcionalidade.

Este PR formaliza, **em nível de especificação** (nenhuma implementação):

- **ADR-013** — fixa o Vynamp Restore como ferramenta desktop dedicada: processo separado com motor de restauração reaproveitável, escopo explícito do que **não** é restauração, e a correção documentada das incoerências do MVP.
- **`docs/tools/01-vynamp-restore.md`** — spec normativa do `vynamp-restore`: contrato de CLI e códigos de saída, cadeia de **11 estágios DSP** em ordem fixa com equações e faixas de parâmetro, **12 detectores de defeito** com severidade e evidência numérica, regiões, comparação A/B, lote, metadados, exportação determinística e não destrutiva, formato de projeto `.vynamp-restore`, limites e matriz de plataformas.
- **`docs/tools/02-vynamp-converter.md`** — escopo preparatório fechado do `vynamp-converter`: matriz de formatos de entrada/saída, rótulagem lossless/lossy obrigatória, contrato de fila de jobs e fronteira da lib própria `vynamp-encode`.
- **Épico E18** — "Ferramentas de arquivo: Restore e Converter", com TASK-E18-01 (Restore) e TASK-E18-02 (Converter), REQ-E18-01/02 na matriz normativa e entradas no backlog.

Decisões-chave registradas:

| Tema | Decisão |
|---|---|
| Processo | `vynamp-restore` e `vynamp-converter` são executáveis próprios, uma instância por usuário, sem depender do player em execução |
| Motor | O app é a casca; o motor de restauração é biblioteca interna reaproveitável |
| Cadeia DSP | Ordem fixa e auditável de 11 estágios: de-crackle (AR, P=48, 2 passagens) → LPF linear-phase (63 taps, Kaiser β=6) → remoção de artefatos MP3 (STFT N=512/H=128) → BWE → de-click → offset DC → de-rumble → notch de hum → gate → anti-saturação → blend de regiões |
| Diagnóstico | 12 detectores (clipping, true_peak, click, crackle, hum50, hum60, hiss, rumble, discontinuity, skip, dc_offset, band_limit) com severidade `mild`/`moderate`/`severe`, evidência numérica e correção sugerida |
| Exportação | A origem **nunca** é sobrescrita sem `--overwrite`; temporário no mesmo volume → `fsync` → `rename`; validação pós-escrita com releitura e decodificação; falha remove a saída e preserva a origem |
| Metadados | ID3v2, RIFF `LIST`/`INFO` e XMP extraídos, preservados e re-embutidos; campos desconhecidos não são descartados quando o contêiner suporta |
| Formatos | Saída em FLAC, WAV (16/24/32 int, 32 float), AIFF e ALAC; sem saída DSD na 1.0 |
| Codificadores | Lib própria `vynamp-encode` (`std`-only, repositório privado) como **única** fonte de encoders; a entrega dos codificadores é E08-02, a ferramenta que os usa é E18-02 |
| AAC | Risco de patente **aceito e registrado** pelo mantenedor — decisão explícita, não omissa |
| Rotulagem | Conversão lossy **nunca** é anunciada como restauração hi-res; degradação de profundidade ou taxa exige confirmação e aparece no relatório |

Incoerências do MVP corrigidas e **declaradas** na spec nativa:

1. **De-crackle** — o módulo `mid/side` do MVP é código morto; o que roda de fato é o modelo AR/L-SAR, que é o que a spec fixa.
2. **Denoise Wiener** — inalcançável no MVP (sem UI, descartado pelo state store); declarado fora de escopo na 1.0 em vez de ausente em silêncio.
3. **Região `kind: overlay`** — placeholder ("AI separation pending") no MVP; a 1.0 suporta apenas regiões de ganho/fade (`kind: between`).

TASK / REQ / QA:

- Specs: `docs/architecture/adr/ADR-013-vynamp-restore.md`, `docs/tools/01-vynamp-restore.md`, `docs/tools/02-vynamp-converter.md`.
- Épico e tarefas: `tasks/epics/E18.md`, `tasks/items/TASK-E18-01.md`, `tasks/items/TASK-E18-02.md` e `tasks/backlog.json` (paridade verbatim de `title`/`deliverable`/`acceptance` com os MDs).
- REQ: `REQ-E18-01`, `REQ-E18-02` — total de 184 requisitos/tarefas em 18 épicos.
- QA referenciados (não executados aqui — a execução é das próprias TASKs): `QA-015`, `QA-016`, `QA-023`, `QA-024`, `QA-031`. Nenhum ID de QA novo foi inventado.
- Índice, glossário, catálogo de funcionalidades e histórico: `docs/INDEX.md`, `docs/GLOSSARY.md`, `docs/product/06-feature-parity.md`, `CHANGELOG.md`, `VALIDATION.json`.

Plano Superpowers e evidência de execução:

Plano em `docs/superpowers/plans/2026-11-05-specs-vynamp-restore.md` (6 tarefas: ADR-013 → spec do Restore → spec do Converter → épico/tarefas/backlog → índices/glossário/changelog → validação e revisão). Worktree e branch próprios: `repo/.worktrees/claude/specs-vynamp-restore` em `chore/claude-specs-vynamp-restore`, base `chore/claude-specs-shell-integration` (64b7487). A referência factual dos algoritmos e dos números de parâmetro é o MVP web rev.38, fixado por hash em `vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452`; nenhum número foi inventado.

## Verificação

Comandos, resultados, ambientes/fixtures e limitações:

Executados localmente (Windows 11 Pro 10.0.26300, Rust 1.99.0):

- `cargo xtask check-docs` → `result: passed`, 0 erros, 336 arquivos Markdown, 184 tarefas, 184 requisitos, 18 épicos, 68 entradas de catálogo.
- `cargo xtask check-delivery` → `result: passed`, 0 erros.
- `cargo test -p xtask` → 85 testes aprovados, 0 falhas.
- Hook de pre-push (mesmos checks do CI: `xtask ci docs` + 51 testes de `xtask` + 34 testes de `tests/tooling.rs`) → aprovado.

Revisão técnica (subagente Opus, contexto limpo), commit revisado e achados:

- Revisão por subagente de contexto limpo (Opus — alto risco, define a arquitetura de uma ferramenta de áudio) sobre ADR-013, as duas specs, épico E18, TASKs, backlog, matriz normativa, índice, glossário, feature-parity e CHANGELOG → veredito e achados na evidência.
- Nenhuma credencial no diff.

Limitações declaradas:

- **Nada foi implementado nem testado.** Este PR é exclusivamente de especificação: nenhuma linha de código de restauração, conversão ou encoder foi escrita ou executada; nenhum arquivo de áudio foi restaurado ou transcodificado.
- **CI Gate (Woodpecker) não verificado**, conforme instrução vigente do mantenedor de ignorar o CI/CD por enquanto. O CI Gate precisa ser conferido pelo mantenedor no SHA final antes do merge.
- **macOS e hardware de saída de áudio não verificados** (sem agente Mac nem hardware de teste neste ambiente) — marcados como não verificados.
- **PR5 (execução) está fora deste PR**: a lib `vynamp-encode`, a fila de jobs e os artefatos de release do Converter ficam para a próxima entrega.

- [x] Worktree e owner próprios.
- [x] Superpowers carregado; teste útil e verificação executados.
- [x] Sucesso/erro/cancelamento e acessibilidade aplicáveis cobertos (n/a — entrega documental; os contratos de erro e cancelamento estão nas specs).
- [x] Docs/contracts/notices atualizados; nenhuma credencial no diff.
- [ ] CI Gate (Woodpecker) aprovado; macOS/hardware não disponível marcado como não verificado.

---

Aprovação e merge são do mantenedor (merge commit pelo GitHub).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
