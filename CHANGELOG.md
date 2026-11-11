# Changelog / Histórico

Versão: 0.9.8 · Data: 2026-11-11 · Estado: especificação proposta, implementação não iniciada.

Each entry appears in English and Portuguese. The two texts say the same thing;
where they appear to differ, the Portuguese text of that entry is the original
and prevails.

Cada entrada aparece em inglês e português. Os dois textos dizem a mesma coisa;
onde parecerem divergir, o texto em português daquela entrada é o original e
prevalece.

## 0.1.0 — 2026-10-14

**EN** — Initial specification of an independent Rust product; own themes and
plugins; original classic visual and tokens; local, hi-res, DSP and streaming
features; future platform; architecture, contracts, quality and security;
traceable tasks and validated examples. Assumptions and phases made explicit. No
player implementation and no Winamp code was used.

**PT** — Especificação inicial de produto independente Rust; temas/plugins
próprios; visual clássico original e tokens; funcionalidades
locais/hi-res/DSP/streaming; plataforma futura;
arquitetura/contratos/qualidade/segurança; tarefas rastreáveis e exemplos
validados. Hipóteses e fases explicitadas. Não houve implementação do player ou
uso de código do Winamp.

## 0.2.0 — 2026-10-14

**EN** — Windows/Linux/macOS Intel+Silicon/Android mandatory from proofs to 1.0.
Linux/macOS promoted M5→M3 with IDs preserved; Android core in Rust plus native
shell; mobile/tablet/backends/QA/CI/distribution added. 16 new
tasks/requirements, epics E13/E14, release gates updated. No audio
implementation or testing in the product.

**PT** — Windows/Linux/macOS Intel+Silicon/Android obrigatórios desde provas até
1.0. Linux/macOS promovidos M5→M3 com IDs preservados; Android/core Rust+shell
nativo, mobile/tablet/backends/QA/CI/distribuição adicionados. 16 novas
tarefas/requisitos, épicos E13/E14, gates de release atualizados. Não houve
implementação/teste de áudio no produto.

## 0.3.0 — 2026-10-14

**EN** — Root project prepared for the agent pair with Superpowers mandatory;
worktree/claim/handoff/Gitflow/private GitHub; CI/release/hardware workflows,
tooling tests and installer/signing recipes. 146 tasks/15 epics. No plugin was
installed, no private repository was accessed or pushed, and no application or
installer was compiled in this preparation.

**PT** — Projeto raiz preparado para o par de agentes com Superpowers obrigatório;
worktree/claim/handoff/Gitflow/GitHub privado; workflows CI/release/hardware,
tooling tests e recipes de installer/assinatura. 146 tarefas/15 épicos. Nenhum
plugin foi instalado, repositório privado acessado/pushado, aplicativo ou
instalador compilado nesta preparação.

## 0.4.0 — 2026-10-14

**EN** — Automation migrated from Python to Rust/`cargo xtask`. Real workspace,
Rust 1.99.0 and `Cargo.lock`; Git/claims/doctor/import, validators, CI and
hardware gates, and signing/installer/release recipes. Hooks and workflows
updated; CI distinguishes `xtask` from real player sources. The package keeps
release/application flags false. Local execution documented in
`docs/evidence/rust-tooling.json`; remote CI, signing, player and hardware were
not executed in this delivery.

**PT** — Automação migrada de Python para Rust/cargo xtask. Workspace, Rust
1.99.0 e Cargo.lock reais; Git/claims/doctor/importação, validadores,
CI/hardware gates e receitas de assinatura/instaladores/releases. Hooks e
workflows atualizados; CI distingue xtask de fontes reais do player. Package
mantém release/application flags false. Execução local documentada em
docs/evidence/rust-tooling.json; CI remoto/assinatura/player/hardware não
executados nesta entrega.

## 0.5.0 — 2026-10-14

**EN** — Name Vynamp and identity C — Pulso chosen by the user. Includes three
original reference PNGs with SHA and dimensions, a multiplatform guide, brand
tokens, ADR-008 and export gates. Renames namespaces/contracts/templates/previews
and the ZIP, updates the renamed GitHub target and preserves historical claims
and evidence. Does not export or fake vectors, native icons or executed
installers.

**PT** — Nome Vynamp e identidade C — Pulso escolhidos pelo usuário. Inclui
três PNGs originais de referência com SHA/dimensões, guia
multiplataforma/brand tokens, ADR-008 e gates de export. Renomeia
namespaces/contratos/templates/previews e ZIP, atualiza GitHub informado
renomeado e preserva claims/evidências históricas. Não exporta/finge
vetores/ícones nativos ou instaladores executados.

## 0.6.0 — 2026-10-14

**EN** — Completeness audit: associations/Open With on five targets,
batch/IPC/grants, owner per profile/portable/backup/deep links; catalogue of 68
bindings, USAC/immersive and Bluetooth separated; trim/DRC/metadata; true-peak
protection after master/mix/resampler; visualizer and proxy contract. Two new
epics/30 tasks, total 176 TASK/REQ across 17 epics; QA-022–031 and four new
schemas. Corrects distribution divergences while keeping eight mandatory
artefacts. Documentation and Rust validation only; no player implementation,
native export or remote publication.

**PT** — Auditoria de completude: associações/Open With em cinco alvos,
batch/IPC/grants, owner por perfil/portable/backup/deep links; catálogo de 68
bindings, USAC/imersivos e Bluetooth separado; trim/DRC/metadata; proteção
true-peak após master/mix/resampler; contrato de visualizers e proxy. Dois novos
épicos/30 tarefas, total 176 TASK/REQ em 17 épicos; QA-022–031 e quatro novos
schemas. Corrige divergências de distribuição mantendo oito artefatos
obrigatórios. Documentação e validação Rust; nenhuma implementação de player,
export nativo ou publicação remota.

## 0.7.0 — 2026-10-14

**EN** — Consolidates Windows corrections for verbatim/UNC paths, checks running
in a cache distinct from the executable, and cache aliases. Import avoids
colliding with an existing remote bootstrap branch. New `prepare-agents`
preserves the import branch/PR, creates one agent with a claim and the other
detached with its own index; it does not depend on a `develop` branch that does not exist
yet. ZIP in a single `vynamp-media-player` root with package/repo, PowerShell
wrappers, automatic prompts and local caches/tmp/logs. Keeps 176 TASK/REQ and 68
planned catalogue entries, app/release/export flags false, and native evidence
still required.

**PT** — Consolida correções Windows de caminhos verbatim/UNC, checks em cache
distinto do executable e aliases de cache. Import evita colidir com branch de
bootstrap remota existente. Novo prepare-agents preserva import branch/PR, cria
um agente com claim e o outro detached/índice próprio; não depende de develop
local ausente. ZIP em raiz única vynamp-media-player com pacote/repo, wrappers
PowerShell, prompts automáticos e caches/tmp/logs locais. Mantém 176 TASK/REQ e
68 entradas planejadas, flags de app/release/exports false e evidências nativas
ainda exigidas.

## Unreleased — 2026-10-23 / Não lançado — 2026-10-23

**EN** — Workflow migrated to a single agent: the second agent was removed from
the tooling (`doctor`/`install` for Claude only, no `prepare-agents`), from the
bootstrap, from the Superpowers lock and from the live docs. Claude Code is the
only author and reviewer (Opus orchestrates, Sonnet implements and reviews, Haiku
runs checks); merge and release stay with the maintainer. ADR-007 received a
partial-supersession note. Historical evidence and plans that mention it were
preserved.

**PT** — Fluxo migrado para agente único: o segundo agente foi removido do
tooling (`doctor`/`install` só para Claude, sem `prepare-agents`), do bootstrap,
do lock do Superpowers e dos docs vivos. Claude Code é o único autor e revisor
(Opus orquestra, Sonnet implementa e revisa, Haiku executa checks); merge e
release permanecem com o mantenedor. ADR-007 recebeu nota de superação parcial.
Evidências e planos históricos que o citam foram preservados.

## Unreleased — 2026-10-25 / Não lançado — 2026-10-25

**EN** — DSP documentation revised to 0.7.1 in the files touched: functional DSP
overview with a Mermaid diagram of the real chain ([03-dsp](reference/audio/03-dsp.md));
a mandatory equalizer subchapter — Graphic 10/20/31/64 plus Parametric up to 32
filters, with frequency grids, Nyquist, data model, UI, MVP versus native and
tests ([08](reference/audio/08-equalizers.md)); module catalogue
([09](reference/audio/09-dsp-catalog.md)); vitrola integration by reference, with the
design approved on 2026-10-23 ([10](reference/audio/10-vitrola.md)); DSP
panel/meters, test matrix, QA-032/033, DSP roadmap, risks, glossary and
decisions D-DSP-01..10 with divergences. Native PEQ32 stays M3; native Graphic
EQ-64 and PEQ64 stay M5; the HTML/SVG/JS MVP now covers the UI of the five
modes. Markdown only; no code, schema, backlog or pipeline changed.

**PT** — Documentação DSP revisada para 0.7.1 nos arquivos tocados: visão
funcional do DSP com diagrama Mermaid da cadeia real
([03-dsp](reference/audio/03-dsp.md)); subcapítulo obrigatório de equalizadores
Gráfico 10/20/31/64 + Paramétrico até 32 filtros, com grades de frequência,
Nyquist, modelo de dados, UI, MVP × nativo e testes
([08](reference/audio/08-equalizers.md)); catálogo de módulos
([09](reference/audio/09-dsp-catalog.md)); integração por referência do vitrola, com
design aprovado em 2026-10-23 ([10](reference/audio/10-vitrola.md)); painel
DSP/medidores, matriz de testes, QA-032/033, roadmap DSP, riscos, glossário e
decisões D-DSP-01..10 com divergências. PEQ32 nativo segue M3; EQ-64 gráfico e
PEQ64 nativos seguem M5; o MVP HTML/SVG/JS cobre agora a UI dos cinco modos. Só
Markdown; nenhum código, schema, backlog ou pipeline alterado.

## Unreleased — 2026-10-28 (DSP v1.4) / Não lançado — 2026-10-28 (DSP v1.4)

**EN** — DSP v1.4 documentation: Audio and effects panel with DSP-UX-01..21
access and controls and VU-INT-01..06 integration in
[design/05-dsp-panel.md](design/06-painel-dsp.md) (§6.1, §8, §9);
DSP-UX-AC-01..12 criteria and graph/VU-INT tests in
[quality/05-dsp-test-matrix.md](quality/04-matriz-dsp.md) (§2); modular
graph versus native in [audio/03-dsp.md](reference/audio/03-dsp.md); revision 15
status in [audio/08-equalizers.md](reference/audio/08-equalizers.md) (§14); decisions
D-DSP-11..18 in [product/05-decisions.md](product/05-decisoes.md). The
HTML/SVG/JS prototype is external: reported evidence of the external reference
rev. 14/15, not verified in this repository; nothing native implemented.
Markdown only; no code, schema, backlog or pipeline changed.

**PT** — Documentação DSP v1.4: painel Áudio e efeitos com acesso e controles
DSP-UX-01..21 e integração dos VUs VU-INT-01..06 em
[design/05-dsp-panel.md](design/06-painel-dsp.md) (§6.1, §8, §9);
critérios DSP-UX-AC-01..12 e testes do grafo/VU-INT em
[quality/05-dsp-test-matrix.md](quality/04-matriz-dsp.md) (§2); grafo
modular × nativo em [audio/03-dsp.md](reference/audio/03-dsp.md); status da revisão
15 em [audio/08-equalizers.md](reference/audio/08-equalizers.md) (§14); decisões
D-DSP-11..18 em [product/05-decisions.md](product/05-decisoes.md). O
protótipo HTML/SVG/JS é externo: evidência relatada da referência externa
rev. 14/15, não verificada neste repositório; nada nativo implementado. Só
Markdown; nenhum código, schema, backlog ou pipeline alterado.

## Unreleased — 2026-10-28 (lyrics) / Não lançado — 2026-10-28 (letras)

**EN** — Lyrics: a summarised component below the VU and its sources in
[library/02-tags-lyrics-tools.md](library/02-tags-letras.md), with IDs
LYR-01..15 and LYR-AC-01..12, layout in
[design/02-layouts.md](design/02-layouts.md), decisions D-LYR-01..09 in
[product/05-decisions.md](product/05-decisoes.md) and the task criteria in
[TASK-E03-09](apps/player/tasks/items/TASK-E03-09.md). The prototype reference is external:
reported evidence of the external reference rev. 14, not verified in this
repository; nothing native implemented. Markdown only; no code, schema, backlog
or pipeline changed (TASK-E03-09 was updated in text only: deliverable,
dependencies and criteria).

**PT** — Letras: componente resumido sob o VU e fontes em
[library/02-tags-lyrics-tools.md](library/02-tags-letras.md), com IDs
LYR-01..15 e LYR-AC-01..12, layout em
[design/02-layouts.md](design/02-layouts.md), decisões D-LYR-01..09 em
[product/05-decisions.md](product/05-decisoes.md) e critérios da tarefa em
[TASK-E03-09](apps/player/tasks/items/TASK-E03-09.md). A referência do protótipo é externa:
evidência relatada da referência externa rev. 14, não verificada neste
repositório; nada nativo implementado. Só Markdown; nenhum código, schema,
backlog ou pipeline alterado (TASK-E03-09 foi atualizada apenas em texto:
entrega, dependências e critérios).

## Unreleased — 2026-11-03 (MVP reference) / Não lançado — 2026-11-03 (referência do MVP)

**EN** — Web MVP rev.37 frozen by hash as the specification of the native UI
(egui) in [design/06-mvp-reference.md](design/07-pin-mvp.md); extracted
package (12 themes, 91 icons, 16 meters, rendered SVGs) by the separate
repository `vynamp-mvp-reference`; QA-034 and TASK-E02-11..16 (visual baseline,
motion, behaviour, gaps and the egui_kittest harness). resvg versus Chrome
verification in [evidence](evidence/PR/task-e02-11/README.md). Nothing
native implemented.

The MVP package was withdrawn from this repository: no MVP file stays here; it
and the extracted package live only in `vynamp-mvp-reference`.

**PT** — MVP web rev.37 congelado por hash como especificação da UI nativa (egui)
em [design/06-mvp-reference.md](design/07-pin-mvp.md); pacote extraído
(12 temas, 91 ícones, 16 medidores, SVGs renderizados) pelo repositório próprio
`vynamp-mvp-reference`; QA-034 e TASK-E02-11..16 (fases de gabarito visual,
movimento, comportamento, lacunas e harness egui_kittest). Verificação resvg ×
Chrome em [evidência](evidence/PR/task-e02-11/README.md). Nada nativo
implementado.

Pacote do MVP retirado deste repositório: nenhum arquivo do MVP fica aqui; ele e
o pacote extraído vivem só em `vynamp-mvp-reference`.

## Unreleased — 2026-11-03 (MVP rev.38 references) / Não lançado — 2026-11-03 (referências ao MVP rev.38)

**EN** — Specs now cite the web MVP rev.38 by URL, fixed SHA and path:
`vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452` (`mvp/...`,
`out/...`); no MVP file is copied into this repository.
[MVP reference](design/07-pin-mvp.md) rewritten (v0.8.0): the 16
meters are selectable, real `out/` paths, content hashes and git tree labelled,
no `SOURCE.json`; at pin time, `tools/package.cjs` and the reference
repository's README still copied `out/` into `assets/reference/mvp/`, which was
removed in `vynamp-mvp-reference@741316f` (`tools/pin.cjs`). TASK-E02-01..10,
E02-12..16 and the UI tasks pending outside E02 (E06-01, 03, 06, 07, 08, 10,
E10-06, E13-05, E13-09, E17-09 and E17-10) point at the pin; TASK-E02-11 remains
as history of rev.37. The TASK-E02-16 harness runs locally only; the CI Gate
marks it as not executed. iOS is no longer a target in TASK-E02-15 (outside
1.0). Citations of `interactive/` (prototype rev.14/15) marked as
historical.

**PT** — As specs passam a citar o MVP web rev.38 por URL, SHA fixo e caminho:
`vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452` (`mvp/...`,
`out/...`); nenhum arquivo do MVP é copiado para este repositório.
[Referência do MVP](design/07-pin-mvp.md) reescrita (v0.8.0): os 16
medidores são selecionáveis, caminhos reais de `out/`, hashes de conteúdo e git
tree rotulados, sem `SOURCE.json`; no pin, `tools/package.cjs` e o README do
repositório de referência ainda copiavam `out/` para `assets/reference/mvp/`, o
que foi removido em `vynamp-mvp-reference@741316f` (`tools/pin.cjs`).
TASK-E02-01..10, E02-12..16 e as TASKs de UI pendentes fora de E02 (E06-01, 03,
06, 07, 08, 10, E10-06, E13-05, E13-09, E17-09 e E17-10) apontam para o pin;
TASK-E02-11 permanece como histórico da rev.37. O harness de TASK-E02-16 roda só
localmente; o CI Gate o marca como não executado. iOS deixa de constar como alvo
em TASK-E02-15 (fora da 1.0). Citações de `interactive/` (protótipo
rev.14/15) marcadas como históricas.

## Unreleased — 2026-11-03 (own audio stack) / Não lançado — 2026-11-03 (pilha de áudio própria)

**EN** — [ADR-011](adr/ADR-011-pilha-propria.md): decode,
output, effects and tags now come from the project's own libraries
`vynamp-decode`, `vynamp-output` and `vitrola` (fixed SHAs, real versus planned
API state and gap table); the resampler will be a new own library (name to be
defined, D-DSP-08 decided) and tag writing will extend `vynamp-decode-meta`.
ADR-002 marked partially superseded; stack, formats, outputs, backends, DSP
catalogue, references, licensing specs and TASK-E04-02 rewritten without the
third-party library names. Documentation only.

**PT** — [ADR-011](adr/ADR-011-pilha-propria.md): decode,
saída, efeitos e tags passam a vir das bibliotecas próprias vynamp-decode,
vynamp-output e vitrola (SHAs fixos, estado API real × planejado e tabela de
lacunas); o resampler será uma nova lib própria (nome a definir, D-DSP-08
decidido) e a escrita de tags estenderá `vynamp-decode-meta`. ADR-002 marcada
como parcialmente substituída; specs de stack, formatos, saídas, backends,
catálogo DSP, referências, licenças e TASK-E04-02 reescritas sem os nomes de
bibliotecas de terceiros. Somente documentação.

## Unreleased — 2026-11-05 (shell integration and icon packs) / Não lançado — 2026-11-05 (integração com o shell e pacotes de ícones)

**EN** — [ADR-012](adr/ADR-012-integracao-shell.md) formalises
native shell integration: the modern Windows 11 menu through a signed sparse MSIX
package and `IExplorerCommand`, the classic HKCU menu for Windows 10/"Show more
options", Desktop Actions and file-manager extensions (Dolphin KIO ServiceMenu,
Nautilus, Thunar/Nemo) on Linux, Quick Actions (`NSServices`) and
`CFBundleDocumentTypes` on macOS, and Intent Filters
(`ACTION_VIEW`/`ACTION_SEND`/`ACTION_SEND_MULTIPLE`) with SAF on Android. A
"Vynamp" submenu with 7 canonical verbs (Play, Open in new playlist, Add to
playlist, Add to queue, Edit tags, Convert, Restore with Vynamp Restore) and the
corresponding CLI contract (`--play`, `--new-playlist`, `--add-playlist`,
`--enqueue`, `--edit-tags`, `vynamp-converter`, `vynamp-restore`), with atomic
batching through `IShellItemArray`. Vynamp Restore and Vynamp Converter become
separate desktop executables. A file-icon pack system with 3 families
(Document, Disc, Signal) for 17 formats, multi-resolution `.ico` (16–256 px) on
Windows, `hicolor` hierarchy on Linux and in-app application on macOS/Android;
catalogue in [assets/icons](assets/icons/README.md) with manifest and interactive
preview. [03-system-integration](platforms/03-integracao-sistema.md)
rewritten to 0.8.0; TASK-E16-03/04/05/08 and the backlog updated; glossary and
index extended. Documentation and assets only; no integration implemented or
tested.

**PT** — [ADR-012](adr/ADR-012-integracao-shell.md) formaliza
a integração nativa com o shell: menu moderno do Windows 11 via pacote esparso
MSIX assinado e `IExplorerCommand`, menu clássico HKCU para Windows 10/"Mostrar
mais opções", Desktop Actions e extensões de gerenciador (Dolphin KIO
ServiceMenu, Nautilus, Thunar/Nemo) no Linux, Quick Actions (`NSServices`) e
`CFBundleDocumentTypes` no macOS, e Intent Filters
(`ACTION_VIEW`/`ACTION_SEND`/`ACTION_SEND_MULTIPLE`) com SAF no Android. Submenu
"Vynamp" com 7 verbos canônicos (Reproduzir, Abrir em nova lista, Adicionar à
lista, Adicionar à fila, Editar etiquetas, Converter, Restaurar com Vynamp
Restore) e contrato CLI correspondente (`--play`, `--new-playlist`,
`--add-playlist`, `--enqueue`, `--edit-tags`, `vynamp-converter`,
`vynamp-restore`), com batching atômico via `IShellItemArray`. Vynamp Restore e
Vynamp Converter passam a ser executáveis desktop separados. Sistema de pacotes
de ícones de arquivo com 3 famílias (Documento, Disco, Sinal) para 17 formatos,
`.ico` multi-resolução (16–256px) no Windows, hierarquia `hicolor` no Linux e
aplicação in-app no macOS/Android; catálogo em
[assets/icons](assets/icons/README.md) com manifesto e preview interativo.
[03-system-integration](platforms/03-integracao-sistema.md) reescrito para
0.8.0; TASK-E16-03/04/05/08 e o backlog atualizados; glossário e índice
ampliados. Somente documentação e assets; nenhuma integração implementada ou
testada.

## Unreleased — 2026-11-05 (file tools: Restore and Converter) / Não lançado — 2026-11-05 (ferramentas de arquivo: Restore e Converter)

**EN** — [ADR-013](adr/ADR-013-restore.md) fixes Vynamp
Restore as a dedicated desktop tool: a separate process (the player's real-time
engine is untouched), the restoration engine as a reusable internal library, and
an explicit scope of what restoration is not (AI voice separation, master
reconstruction, hi-res upscaling). The spec
[tools/01-vynamp-restore.md](reference/tools/01-vynamp-restore.md) closes the
`vynamp-restore` contract: a normative chain of 11 DSP stages in fixed order
(AR de-crackle, linear-phase LPF, MP3 artefact removal, BWE, de-click, DC offset,
de-rumble, hum notch, gate, anti-saturation, region blend), 12 defect detectors
with severity and numerical evidence, A/B comparison, batch, metadata,
deterministic export with atomic write and post-write validation, and a
`.vynamp-restore` project. The spec
[tools/02-vynamp-converter.md](reference/tools/02-vynamp-converter.md) closes the
`vynamp-converter` scope for PR5: format matrix, mandatory lossless/lossy
labelling, persisted job queue, and the own library `vynamp-encode` as the single
source of encoders. MVP prototype divergences are recorded: the de-crackle
actually running in the MVP is the AR model (not the inert mid/side module), the
Wiener denoise is unreachable and out of scope for 1.0, and `kind: overlay` was a
placeholder. Epic [E18](tasks/epics/E18.md) created with TASK-E18-01 (Restore)
and TASK-E18-02 (Converter), REQ-E18-01/02 in the normative matrix (184
requirements/tasks across 18 epics) and backlog entries. Documentation only; not
one line of restoration, conversion or encoder code was implemented or executed.

**PT** — [ADR-013](adr/ADR-013-restore.md) fixa o Vynamp
Restore como ferramenta desktop dedicada: processo separado (o motor de tempo
real do player fica intocado), motor de restauração como biblioteca interna
reaproveitável e escopo explícito do que não é restauração (separação de voz por
IA, reconstrução de master, upscaling hi-res). A spec
[tools/01-vynamp-restore.md](reference/tools/01-vynamp-restore.md) fecha o contrato do
`vynamp-restore`: cadeia normativa de 11 estágios DSP em ordem fixa (de-crackle
AR, LPF linear-phase, remoção de artefatos MP3, BWE, de-click, offset DC,
de-rumble, notch de hum, gate, anti-saturação, blend de regiões), 12 detectores
de defeito com severidade e evidência numérica, comparação A/B, lote, metadados,
exportação determinística com escrita atômica e validação pós-escrita, e projeto
`.vynamp-restore`. A spec
[tools/02-vynamp-converter.md](reference/tools/02-vynamp-converter.md) fecha o escopo
do `vynamp-converter` para o PR5: matriz de formatos, rótulagem lossless/lossy
obrigatória, fila de jobs persistida e a lib própria `vynamp-encode` como única
fonte de codificadores. As divergências do protótipo MVP ficam registradas: o
de-crackle realmente executado no MVP é o modelo AR (não o módulo mid/side
inerte), o denoise Wiener é inalcançável e está fora de escopo da 1.0, e
`kind: overlay` era placeholder. Épico [E18](tasks/epics/E18.md) criado com
TASK-E18-01 (Restore) e TASK-E18-02 (Converter), REQ-E18-01/02 na matriz
normativa (184 requisitos/tarefas em 18 épicos) e entradas no backlog. Somente
documentação; nenhuma linha de código de restauração, conversão ou encoder foi
implementada ou executada.

## Unreleased — 2026-11-05 (GitHub Actions, public repositories and governance) / Não lançado — 2026-11-05 (GitHub Actions, repositórios públicos e governança)

**EN** — CI moved back to GitHub Actions ([ADR-014](adr/ADR-014-publico-dsp-interno.md),
which supersedes [ADR-010](_arquivo/repo/docs/architecture/adr/ADR-010-woodpecker-only-ci.md)):
`ci.yml`, `release.yml` and `quality.yml` under `.github/workflows/`, actions
pinned by full commit SHA, the `gate` job published as `CI Gate`, and the
Woodpecker server, its Cloudflare tunnel and `infra/ci/` removed from the
repository. `xtask` was inverted to require the workflows instead of forbidding
them, and `release` validates its tag through `GITHUB_REF`/`GITHUB_REF_NAME`.
Verified in practice: the workflow ran on pull request #30 and again on push to
`develop`, green in `docs`, the four `tooling` targets (`windows-2022`,
`ubuntu-22.04`, `macos-15-intel`, `macos-15`) and `CI Gate`, with `core` and
`android` skipped because `cargo xtask ci detect` finds no sources for them. The
repository is still **private** and its branch protection is still unavailable,
so `CI Gate` is a reviewed discipline rather than a GitHub-enforced block.

Repository hygiene and governance (TASK-E19-02): the project finally has a
licence — `MIT OR Apache-2.0`, with `LICENSE`, `LICENSE-MIT` and `LICENSE-APACHE`
— recorded as D-LIC-01 because the previous state was an explicit decision *not*
to have one; `LEGAL.md`, `CODE_OF_CONDUCT.md`, `GOVERNANCE.md`, issue forms and a
bilingual `README.md`, `SECURITY.md`, `CONTRIBUTING.md` and this changelog.
`SECURITY.md` now points at GitHub Security Advisories (D-GOV-01) instead of
declaring that no channel existed. Documentation and repository files only; no
player code.

**PT** — CI voltou para GitHub Actions
([ADR-014](adr/ADR-014-publico-dsp-interno.md),
que substitui a [ADR-010](_arquivo/repo/docs/architecture/adr/ADR-010-woodpecker-only-ci.md)):
`ci.yml`, `release.yml` e `quality.yml` em `.github/workflows/`, actions fixadas
por SHA completo de commit, job `gate` publicado como `CI Gate`, e o servidor
Woodpecker, seu túnel Cloudflare e `infra/ci/` removidos do repositório. O
`xtask` foi invertido para exigir os workflows em vez de proibi-los, e o
`release` valida a tag por `GITHUB_REF`/`GITHUB_REF_NAME`. Verificado na prática:
o workflow rodou no pull request #30 e de novo no push para `develop`, verde em
`docs`, nos quatro alvos de `tooling` (`windows-2022`, `ubuntu-22.04`,
`macos-15-intel`, `macos-15`) e no `CI Gate`, com `core` e `android` pulados
porque `cargo xtask ci detect` não encontra fontes para eles. O repositório
continua **privado** e a branch protection segue indisponível, então o `CI Gate`
é disciplina revisada, não bloqueio imposto pelo GitHub.

Higiene e governança do repositório (TASK-E19-02): o projeto passa a ter
licença — `MIT OR Apache-2.0`, com `LICENSE`, `LICENSE-MIT` e `LICENSE-APACHE` —
registrada como D-LIC-01 porque o estado anterior era uma decisão explícita de
*não* ter licença; `LEGAL.md`, `CODE_OF_CONDUCT.md`, `GOVERNANCE.md`, formulários
de issue e `README.md`, `SECURITY.md`, `CONTRIBUTING.md` e este changelog
bilíngues. O `SECURITY.md` passa a apontar para GitHub Security Advisories
(D-GOV-01) em vez de declarar que não havia canal. Somente documentação e
arquivos de repositório; nenhum código de player.

## Não lançado — 2026-11-05 (o MVP web é a fonte única da UI nativa)

Decisão do mantenedor: os dois arquivos de skin da raiz do projeto (`SKIN-REQUIREMENTS-CODEX.md` e `SKIN-REQUIREMENTS-GPT-MVP-OBRIGATORIO-v1.2.md`) **não são fonte de requisito** e saem da cadeia documental do produto; a fonte única da UI nativa é o MVP web em [design/06-mvp-reference](design/07-pin-mvp.md), pinado por SHA. Registrado em [05-decisions](product/05-decisoes.md) como D-MVP-01..04, com duas tabelas: os comportamentos que os SKIN atribuíam ao produto e o MVP **não** tem (0 VU = −18 dBFS ajustável, slot fixo de 28 DIP, atalho da Vitrola no inspector da cadeia, bypass de seção por "intensidade 0", presets Subtle/Warm/Rustic, passo de 2 dB no editor clássico), e as divergências encerradas por esta decisão. D-MVP-02 fixa que só herda comportamento que o código do MVP comprova; D-MVP-03, que nenhuma TASK, REQ ou marco nasce dos SKIN; D-MVP-04, que em divergência de **número** vale o MVP — caso atual, os 12 medidores do contrato VU-INT (número da rev.15) contra os **16, todos selecionáveis**, do pin rev.38. [08-equalizers](reference/audio/08-equalizers.md), [10-vitrola](reference/audio/10-vitrola.md) e [05-dsp-panel](design/06-painel-dsp.md) passam a citar o código do pin no lugar dos SKIN, apenas onde ele de fato implementa; oito afirmações foram conferidas contra `vynamp-mvp-reference@0d9860b`. A matriz VU-INT-06 não foi reescrita — atualizá-la é da TASK-E02-07. Somente documentação: 191 requisitos, 191 tarefas, 19 épicos, nenhuma alteração de código, workflow ou backlog.

## Não lançado — 2026-11-06 (remoção de planos substituídos)

Dez arquivos de `docs/superpowers/` saem do repositório: `plans/2026-10-14-completeness-audit.md`, `specs/2026-10-14-completeness-audit-design.md`, `plans/2026-10-14-project-tooling.md`, `plans/2026-10-14-rust-tooling.md`, `specs/2026-10-14-rust-tooling-design.md`, `plans/2026-10-14-vynamp-branding.md`, `plans/2026-10-15-windows-restart.md`, `plans/2026-10-18-e15-04-woodpecker.md`, `plans/2026-10-19-e15-04-woodpecker-only-ci.md` e `specs/2026-10-18-e15-04-woodpecker-cloudflare-design.md`. Critério aplicado, e não "arquivo sem uso" em geral: plano ou desenho cuja decisão **já vive** num documento normativo posterior — [ADR-008](adr/ADR-008-marca.md) para a marca, [ADR-009](adr/ADR-009-baseline.md) para o tooling Rust, [ADR-014](adr/ADR-014-publico-dsp-interno.md) para o CI, [08-completeness-audit](_arquivo/repo/docs/product/08-completeness-audit.md) para a auditoria e [09-windows-restart](engineering/08-reinicio-windows.md) para o reinício — e cujo conteúdo nenhum INDEX, backlog ou ROADMAP cita como autoridade. Os seis planos restantes ficam: cada um é citado pela TASK ou pela evidência do PR que o executou ([TASK-E15-01](tasks/items/TASK-E15-01.md), E15-02, E15-03, E11-01, e as evidências dos PRs de shell e Restore). O conteúdo integral continua no histórico Git. `docs/INDEX.md` perde as dez entradas; `check-docs` foi de 356 para 346 arquivos Markdown, com 191 requisitos, 191 tarefas e 19 épicos inalterados. Somente documentação.

## Não lançado — 2026-11-05 (o vitrola é módulo DSP interno, não biblioteca)

**EN** — The `vitrola` design (approved 2026-10-23) is no longer a library in its own repository: vinyl and tube are **DSP modules inside the app**, by the maintainer's decision, recorded in [ADR-014](adr/ADR-014-publico-dsp-interno.md). The approved design was preserved **by incorporation**, not by reimplementation: its nine sections are now §8.1–8.9 of [10-vitrola](reference/audio/10-vitrola.md), which became the single source — previously that document integrated an external design by reference and deferred to it. A literal copy of the original text is kept at `docs/evidence/PR/task-e19-03/vitrola-design.md`, because it is the only surviving copy once the repository is archived. The external repository is archived, reverts with `gh repo unarchive`, and is **not** opened to the public. Citations updated in the ADR-011 inventory row, the glossary, the DSP and signal-contract docs, the DSP catalog, the MVP reference, feature parity, privacy/licensing and the roadmap. Integration remains [TASK-E19-07](libs/engine/tasks/items/TASK-E19-07.md), after TASK-E04-03. Documentation only: no player code, 191 requirements, 191 tasks, 19 epics.

**PT** — O design do `vitrola` (aprovado em 2026-10-23) deixou de ser biblioteca em repositório próprio: vinil e válvula são **módulos DSP dentro do app**, por decisão do mantenedor registrada na [ADR-014](adr/ADR-014-publico-dsp-interno.md). O design aprovado foi preservado por **incorporação**, não por reimplementação: suas nove seções são agora §8.1–8.9 de [10-vitrola](reference/audio/10-vitrola.md), que passou a ser fonte única — antes esse documento integrava um design externo por referência e se subordinava a ele. Uma cópia literal do texto original fica em `docs/evidence/PR/task-e19-03/vitrola-design.md`, porque é a única cópia sobrevivente depois que o repositório for arquivado. O repositório externo é arquivado, reversível por `gh repo unarchive`, e **não** é aberto ao público. Citações atualizadas na linha de inventário da ADR-011, no glossário, nos docs de DSP e do contrato de sinal, no catálogo DSP, na referência do MVP, na paridade de recursos, em privacidade/licenças e no roadmap. A integração continua na [TASK-E19-07](libs/engine/tasks/items/TASK-E19-07.md), depois da TASK-E04-03. Somente documentação: nenhum código de player, 191 requisitos, 191 tarefas, 19 épicos.

## Unreleased — 2026-11-09 (READMEs de CI nos seis módulos restantes) / Não lançado — 2026-11-09 (READMEs de CI nos seis módulos restantes)

### Português

Os READMEs de `vynamp-dsp`, `vynamp-encode`, `vynamp-engine`, `vynamp-player`,
`vynamp-converter` e `vynamp-restore` já eram bilíngues, mas nenhum dizia que o
repositório tem CI — os seis publicam `CI Gate` desde o passo 36. A seção `CI`
entrou em inglês e português em cada um, sem texto copiado de um para o outro:
cada README descreve os jobs do seu próprio `ci.yml`. São três formatos.
`dsp` e `encode` rodam `fmt`, `clippy`, `test` e `machete`; `engine` acrescenta
`arch`, a regra que pergunta ao `cargo tree` se algo abaixo dele cita um app —
invariante daquele crate, não da família. Os três apps rodam `fmt`, `clippy`,
`test` e uma matriz `build` de quatro alvos (Windows MSVC, Linux GNU, macOS
Intel, macOS Apple Silicon), e não têm job `machete`: `[dependencies]` do
`Cargo.toml` deles ainda está vazio, então a verificação de dependência não
usada não teria em que se apoiar. A matriz é portão de compilação, não release —
nada é enviado nem assinado.

### English

The READMEs of `vynamp-dsp`, `vynamp-encode`, `vynamp-engine`, `vynamp-player`,
`vynamp-converter` and `vynamp-restore` were already bilingual, but none of them
said the repository has CI — all six have published `CI Gate` since step 36.
A `CI` section went into each in English and Portuguese, with no text copied
between them: every README describes the jobs of its own `ci.yml`. There are
three shapes. `dsp` and `encode` run `fmt`, `clippy`, `test` and `machete`;
`engine` adds `arch`, the rule that asks `cargo tree` whether anything under it
names an app — an invariant of that crate, not of the family. The three apps run
`fmt`, `clippy`, `test` and a `build` matrix of four targets (Windows MSVC,
Linux GNU, macOS Intel, macOS Apple Silicon), and have no `machete` job: their
`Cargo.toml` `[dependencies]` is still empty, so the unused-dependency check
would have nothing to stand on. The matrix is a compile gate, not a release —
nothing is uploaded or signed.

## Unreleased — 2026-11-09 (TASK-E19-05: CI Gate nos módulos, CI do MVP e READMEs) / Não lançado — 2026-11-09 (TASK-E19-05: CI Gate nos módulos, CI do MVP e READMEs)

**EN** — `vynamp-decode` and `vynamp-output` had no workflow at all: the seed that
created repositories in step 35 only wrote `.github/workflows/ci.yml` into an empty
directory, and these two existed before it. Both now run `fmt`, `clippy`, `test` and
`machete`, and publish the required status **`CI Gate`** from a `gate` job with
`if: always()` reading `needs.*.result` — so a dependency that fails, is cancelled or
never reports fails the gate. A job without `if: always()` is skipped when a
dependency fails, and a skipped check is not a red one: the pull request sits
`blocked` waiting for a status that will never arrive. The proof is an interrupted
run, not a reading of the YAML: decode run `42650469273` had `test` and `machete`
cancelled and `CI Gate` concluded `failure`.

The MVP reference got its own CI in Node, running `serve` and `verify` chained in
one step — a verification that does not re-extract through the server proves
nothing — and the job was red on `ubuntu-22.04` for a reason worth writing down.
`out/` is an artifact of *measured* geometry: `drawMini()` derives the minimap's
visibility and content from the measured rect of `#graph-viewport`, which is `flex:1`
under a text-sized header. Swapping the font family moves that rect by one pixel,
which is enough to flip `overflow` at a different point in the click sweep, and
`drawMini` leaves the previous state's children in place when it hides the element.
Measured here: 499 files compared with 0 differing across four local runs; five files
differing deterministically on the Linux runner. The runner is `windows-2022`, the
family that froze `out/` — matching the platform is the fix, and re-freezing `out/`
on another one is a different decision, for a maintainer.

Two steps were also dead weight. `cargo install resvg` bought 33 seconds and nothing
else, because `check-svg` is not a CI step (`out/check/report.json` lists 6 files
that fail on renderer gaps awaiting the font and loader tasks). And the `Chrome` step
used `chrome.exe --version`, which never returned: on Windows Chrome is a
GUI-subsystem binary that neither writes to the console nor exits, so run
`42654628242` sat in that step for 30 minutes with no output. It now reads
`VersionInfo.ProductVersion` with `pwsh`.

The three READMEs were already bilingual; what they did not say was that a CI exists,
so the `CI` section was added in both languages to `vynamp-decode`, `vynamp-output`
and `vynamp-mvp-reference`. No workflow in any of the ten repositories references
Woodpecker as a CI system, and no `.woodpecker/` path survives in any of them.

**PT** — `vynamp-decode` e `vynamp-output` não tinham workflow nenhum: o seed que
criou repositórios no passo 35 só escrevia `.github/workflows/ci.yml` em diretório
vazio, e esses dois existiam antes dele. Os dois passam a rodar `fmt`, `clippy`,
`test` e `machete`, e a publicar o status exigido **`CI Gate`** a partir de um job
`gate` com `if: always()` lendo `needs.*.result` — então dependência que falha, é
cancelada ou nunca reporta reprova o gate. Um job sem `if: always()` é pulado quando
uma dependência falha, e check pulado não é vermelho: o pull request fica `blocked`
esperando um status que nunca vem. A prova é uma execução interrompida, não a leitura
do YAML: o run `42650469273` do decode teve `test` e `machete` cancelados e o
`CI Gate` concluiu `failure`.

A referência do MVP ganhou CI próprio em Node, rodando `serve` e `verify` encadeados
num passo só — uma verificação que não reextrai pelo servidor não prova nada —, e o
job estava vermelho no `ubuntu-22.04` por um motivo que vale registrar. `out/` é
artefato de geometria *medida*: `drawMini()` deriva a visibilidade e o conteúdo do
minimapa do rect medido de `#graph-viewport`, que é `flex:1` sob um cabeçalho
dimensionado por texto. Trocar a família de fonte move esse rect em um pixel, o que
basta para virar `overflow` em outro ponto da varredura de cliques, e o `drawMini`
deixa os filhos do estado anterior no lugar quando esconde o elemento. Medido aqui:
499 arquivos comparados, 0 divergência, em quatro execuções locais; cinco arquivos
divergindo de forma determinística no runner Linux. O runner é `windows-2022`, a
família que congelou `out/` — igualar a plataforma é a correção, e recongelar `out/`
em outra é decisão diferente, do mantenedor.

Dois passos também eram peso morto. `cargo install resvg` comprava 33 segundos e nada
mais, porque `check-svg` não é passo de CI (`out/check/report.json` lista 6 arquivos
que reprovam por lacuna de renderizador, à espera das tasks de fonte e de carregador).
E o passo `Chrome` usava `chrome.exe --version`, que nunca retornava: no Windows o
Chrome é binário de subsistema GUI que não escreve no console nem termina, então o run
`42654628242` ficou 30 minutos nesse passo sem saída. Agora ele lê
`VersionInfo.ProductVersion` com `pwsh`.

Os três READMEs já eram bilíngues; o que não diziam era que existe CI, então a seção
`CI` foi acrescentada nas duas línguas em `vynamp-decode`, `vynamp-output` e
`vynamp-mvp-reference`. Nenhum workflow de nenhum dos dez repositórios referencia
Woodpecker como sistema de CI, e nenhum caminho `.woodpecker/` sobrevive em nenhum
deles.

## Unreleased — 2026-11-09 (TASK-E19-01 AC-5, READMEs bilíngues, JWT do Woodpecker) / Não lançado — 2026-11-09 (TASK-E19-01 AC-5, READMEs bilíngues, JWT do Woodpecker)

**EN** — Three pendencies closed, and in each the finding was the gap between the
claim and what anything checked. **AC-5 of TASK-E19-01** requires that no credential
sits in the history. Nothing checked it: the Woodpecker preflight that looked for
token shapes lived in `infra/ci/`, left with ADR-014, and no replacement was written.
`cargo xtask check-secrets` now reads every blob reachable from every ref — a secret
deleted in the next commit is still in the clone, and the working tree would call it
clean — and reports the blob, the path and the line without ever printing the matched
text, because CI logs are retained and a scanner that prints the secret has published
it. It runs in the `tooling` job, and `check-delivery` fails if that step or the
`git fetch --unshallow` it needs goes missing.

Writing it surfaced two defects, both measured on this repository's own ~3,300
objects. One `git cat-file -p` per blob costs a process start each: 330 s in one run
and past 600 s in another, against 9.5 s batched. And `object_types` wrote every id
before reading any output: 330 s sequentially against 9.5 s with the writer on its own
thread. The second one was first described here as a deadlock; it is not — the
sequential version terminates, and the timing test passes on both versions. What is
recorded is the measurement, not a mechanism that was never confirmed.

**The Woodpecker JWT was removed completely**, by the maintainer's decision on
2026-11-09. The two hooks had gone in TASK-E19-06, but the token itself survived in an
untracked backup at `.claude/settings.local.json.bak-30`; that file is gone and no
copy remains in the tree. Revoking it at the service is not something this repository
can do — the server was already dead — so the value simply no longer exists here.

Five module READMEs became bilingual EN/PT with the status block, licence, security
channel and scope that a public repository needs (`vynamp-dsp`, `vynamp-decode`,
`vynamp-engine`, `vynamp-output`, `vynamp-player`), and the MVP reference README was
repointed from the archived `vynamp-mediaplayer` and a `06-mvp-reference.md` that no
longer exists to `jimmykedlyn/vynamp` and `design/07-pin-mvp.md`. Two places still
called `vynamp-dsp` a private repository; the glossary now says public.

**PT** — Três pendências fechadas, e em cada uma o achado foi a distância entre a
afirmação e o que alguma coisa verificava. O **AC-5 da TASK-E19-01** exige que nenhum
segredo esteja no histórico. Nada verificava: o preflight do Woodpecker que procurava
forma de token morava em `infra/ci/`, saiu com a ADR-014, e nada o substituiu. O
`cargo xtask check-secrets` lê agora todo blob alcançável de toda ref — segredo
apagado no commit seguinte continua no clone, e a árvore de trabalho diria que está
limpo — e informa o blob, o caminho e a linha sem nunca imprimir o texto casado,
porque log de CI é retido e um scanner que imprime o segredo o publicou. Roda no job
`tooling`, e o `check-delivery` reprova se esse passo ou o `git fetch --unshallow` de
que ele depende sumir.

Escrevê-lo revelou dois defeitos, os dois medidos nos ~3.300 objetos deste
repositório. Um `git cat-file -p` por blob custa um processo por blob: 330 s numa
execução e mais de 600 s em outra, contra 9,5 s em lote. E o `object_types` escrevia
todo id antes de ler qualquer saída: 330 s em sequência contra 9,5 s com o escritor em
thread própria. O segundo chegou a ser descrito aqui como deadlock; não é — a versão
sequencial termina, e o teste de tempo passa nas duas. O que fica registrado é a
medição, não um mecanismo que nunca foi confirmado.

**O JWT do Woodpecker foi removido por completo**, por decisão do mantenedor em
2026-11-09. Os dois hooks já tinham saído na TASK-E19-06, mas o token sobrevivia num
backup não versionado em `.claude/settings.local.json.bak-30`; esse arquivo não existe
mais e nenhuma cópia resta na árvore. Revogá-lo no serviço não é algo que este
repositório possa fazer — o servidor já estava morto — então o valor simplesmente não
existe mais aqui.

Cinco READMEs de módulo passaram a ser bilíngues EN/PT com o bloco de estado, licença,
canal de segurança e escopo que um repositório público precisa (`vynamp-dsp`,
`vynamp-decode`, `vynamp-engine`, `vynamp-output`, `vynamp-player`), e o README da
referência do MVP foi reapontado do `vynamp-mediaplayer` arquivado e de um
`06-mvp-reference.md` que não existe mais para `jimmykedlyn/vynamp` e
`design/07-pin-mvp.md`. Dois lugares ainda chamavam `vynamp-dsp` de repositório
privado; o glossário agora diz público.

## Unreleased — 2026-11-09 (dependencies: six Dependabot PRs, two of them broken) / Não lançado — 2026-11-09 (dependências: seis PRs do Dependabot, dois deles quebrados)

**EN** — Six Dependabot PRs were open and four were safe. The other two did not
compile, and neither did their descriptions say so: `sha2` 0.11 stopped implementing
`LowerHex` on the digest — `format!("{:x}", …)` fails with `E0277` in `util::sha` and
`release::source_digest` — and `quick-xml` 0.42 changed `QName::as_ref` to return
`&str`, breaking two `from_utf8` calls in the icon gate, and deprecated
`unescape_value` in favour of `normalized_value(XmlVersion)`. Both bumps were redone
in one PR (`#16`) with the API work: a `util::hex` that encodes the digest bytes, and
a `normalized_value` read. Both fixes carry tests, because both could regress
silently — the source digest is compared against the one recorded in the release
evidence, so a different hex spelling (uppercase, separators, a dropped leading zero)
would break the comparison without breaking any build; and the SVG gate depends on
reading the *unescaped* attribute, so a `fill="&#117;rl(#x)"` would be the same
external reference as a literal `url(#x)` and would pass. The SVG test was verified
red with a raw-value read before the fix. `vynamp-decode` and `vynamp-output` also
had their README `Status` line corrected from "private repository" to "public
repository", false since the repositories were opened.

**PT** — Seis PRs do Dependabot estavam abertos e quatro eram seguros. Os outros dois
não compilavam, e nem a descrição deles dizia isso: `sha2` 0.11 deixou de implementar
`LowerHex` no digest — `format!("{:x}", …)` falha com `E0277` em `util::sha` e
`release::source_digest` — e `quick-xml` 0.42 mudou `QName::as_ref` para devolver
`&str`, quebrando dois `from_utf8` no gate de ícone, e depreciou `unescape_value` em
favor de `normalized_value(XmlVersion)`. Os dois bumps foram refeitos num PR só
(`#16`) com o trabalho de API: um `util::hex` que codifica os bytes do digest, e uma
leitura por `normalized_value`. Os dois consertos têm teste, porque os dois
regrediriam calados — o digest de source é comparado com o gravado na evidência de
release, então outra grafia de hex (maiúscula, separador, zero à esquerda perdido)
quebraria a comparação sem quebrar build nenhum; e o gate de SVG depende de ler o
atributo **desescapado**, então um `fill="&#117;rl(#x)"` seria a mesma referência
externa que um `url(#x)` literal e passaria. O teste de SVG foi verificado vermelho
com a leitura crua antes do conserto. `vynamp-decode` e `vynamp-output` também
tiveram a linha `Status` do README corrigida de "private repository" para "public
repository", falsa desde que os repositórios foram abertos.

## Unreleased — 2026-11-10 (ponteiros de execução que apontavam para trabalho entregue) / Não lançado — 2026-11-10 (ponteiros de execução que apontavam para trabalho entregue)

**EN** — Two files told a reader where to start, and both were stale in the same
direction: they described finished work as pending. `tasks/NEXT.md` was frozen at
2026-10-14 and listed six TASKs to execute first, three of which had already
closed (`E11-01`, `E01-02`, `E02-11`); it now leads with what closed, names
`TASK-E01-01` as the next task — the gate the [ADR-002](adr/ADR-002-audio-core.md)
declares for its own unfreezing — and states what mobile does not have: no
converter, no restore, and the DSP layout adapted rather than ported, with iOS as
phase 6 (`E20`). Two claims in the first draft were wrong and were corrected
against the repository: the ADR cited was `002-versao-features`, which does not
exist, and the Android DSP work is not `E05-09` (network and streaming) but
`E13-05` for the basic EQ, with the full chain staying in `E04-01`. The nine
`tasks/README.md` files, emitted by step 37, had three defects in their header: the
count was glued to the text (`57tarefa(s)`), the number did not come from
`tasks/backlog.json`, and every file said "the last two are engineering", a sentence
that describes no file. The count is now derived from the backlog — total, product
and engineering, naming the engineering ones — and the seven module repositories
carry the fix; the umbrella re-pins to those commits.
**The first correction classified engineering by epic (`E19`, `CI`) and was wrong:**
the rule lives in `xtask/src/validate.rs`, which requires a `REQ` from every product
task and from no engineering one, so engineering is the TASK **without a `REQ`** —
`TASK-CI-01` and `TASK-CI-02` in the umbrella, and none in the seven modules. The
seven `E19` tasks carry `REQ-E19-01`..`REQ-E19-07` and are product tasks; the
`engine`, which step 37's header listed as owning two engineering tasks (`E19-03`,
`E19-07`), owns none. All nine headers were redone by the gate's rule, and the
umbrella's `57` had never been wrong: at `23319a1` it was 57 product plus 2
engineering, and the table has 59 rows — what was missing was the space between the
number and the word.

Markdown and pins only: no player code, 191 requirements, 199 tasks, 19 epics.

**PT** — Dois arquivos diziam a um leitor por onde começar, e ambos estavam
desatualizados na mesma direção: descreviam trabalho entregue como pendente.
`tasks/NEXT.md` estava congelado em 2026-10-14 e listava seis TASKs para executar
primeiro, três das quais já tinham fechado (`E11-01`, `E01-02`, `E02-11`); agora
começa pelo que fechou, nomeia `TASK-E01-01` como a próxima — o gate que a
[ADR-002](adr/ADR-002-audio-core.md) declara para se descongelar — e declara o que
o mobile não tem: converter, restore, e o layout do DSP adaptado em vez de portado,
com iOS como fase 6 (`E20`). Duas afirmações do primeiro rascunho estavam erradas e
foram corrigidas contra o repositório: a ADR citada era `002-versao-features`, que
não existe, e o DSP Android não é `E05-09` (rede e streaming) e sim `E13-05` para o
EQ básico, com a cadeia completa permanecendo em `E04-01`. Os nove `tasks/README.md`,
emitidos pelo passo 37, tinham três defeitos no cabeçalho: a contagem estava colada
no texto (`57tarefa(s)`), o número não vinha do `tasks/backlog.json`, e todo arquivo
dizia "as duas últimas são de engenharia" — frase que não descreve arquivo nenhum.
A contagem agora é derivada do backlog — total, produto e engenharia, nomeando as de
engenharia — e os sete repositórios de módulo carregam a correção; o umbrella
re-pina nesses commits.
**A primeira correção classificou engenharia pelo épico (`E19`, `CI`) e estava errada:**
a regra vive em `xtask/src/validate.rs`, que exige `REQ` de toda tarefa de produto e de
nenhuma de engenharia, então engenharia é a TASK **sem `REQ`** — `TASK-CI-01` e
`TASK-CI-02`, no umbrella, e nenhuma nos sete módulos. As sete do épico `E19` carregam
`REQ-E19-01`..`REQ-E19-07` e são de produto; o `engine`, que o cabeçalho do passo 37
listava como dono de duas tarefas de engenharia (`E19-03`, `E19-07`), não tem nenhuma.
Os nove cabeçalhos foram refeitos pela regra do gate, e o `57` do umbrella nunca tinha
estado errado: em `23319a1` eram 57 de produto mais 2 de engenharia, e a tabela tem 59
linhas — faltava o espaço entre o número e a palavra.

Somente Markdown e pins: nenhum código de player, 191 requisitos, 199 tarefas, 19 épicos.

## Unreleased — 2026-11-10 (autoria das TASKs vira gate) / Não lançado — 2026-11-10 (autoria das TASKs vira gate)

**EN** — The authoring rules of the TASKs existed only as prose, and nothing
measured them. Measured, they failed: 113 `Boundary` bullets said what a task does
*not* do without saying who does, so the reader had nobody to hold the excluded
scope to and the next task repeated the same gap. `cargo xtask check-authoring`
ports the nine rules to `xtask` and a new `authoring` job puts it in CI, required by
the `CI Gate` aggregate; `xtask/src/validate.rs` fails if the aggregate stops
depending on it, because a job that runs but is not aggregated blocks no merge. Two
decisions in the gate are worth review. The owner named in R1 may be an ADR or a
document, not only a TASK: several boundaries are maintainer decisions no task
covers, and inventing a task to satisfy the gate would produce a fictitious owner —
so the cited ADR has to exist in `adr/` and the cited document has to exist on disk,
and `README.md` does not count because it is cited to list tasks, not to name an
owner. And the document path is resolved in two roots, the umbrella and the TASK's
own repository: a module spec lives at `apps/player/specs/ui.md`, and a player bullet
that says `specs/ui.md` is citing real authority — reading only the umbrella root
would reject the correct bullet. The 113 bullets were then rewritten to name their
owner, keeping each original argument, and the gate now reports
`{"blocking":0,"result":"passed","tasks":199,"warnings":15}`; the 15 warnings are
legitimate short bullets and short `When` cells and block nothing. `engineering/04-ci.md`
and `CONTRIBUTING.md` were corrected to list the real jobs and the commands a
contributor actually runs. Markdown and tooling only: no player code, 191
requirements, 199 tasks, 19 epics.

**PT** — As regras de autoria das TASKs existiam só como prosa, e nada as media.
Medidas, elas reprovavam: 113 bullets de `Boundary` diziam o que a tarefa *não* faz
sem dizer quem faz, então o leitor não tinha a quem cobrar o escopo excluído e a
tarefa seguinte repetia a mesma lacuna. `cargo xtask check-authoring` porta as nove
regras para o `xtask` e um job `authoring` novo as põe no CI, exigido pelo agregado
`CI Gate`; `xtask/src/validate.rs` reprova se o agregado deixar de depender dele,
porque job que roda mas não está no agregado não bloqueia merge nenhum. Duas
decisões do gate merecem revisão. O dono nomeado na R1 pode ser uma ADR ou um
documento, não só uma TASK: várias fronteiras são decisões do mantenedor que tarefa
nenhuma cobre, e inventar uma tarefa para satisfazer o gate produziria um dono
fictício — então a ADR citada precisa existir em `adr/` e o documento citado precisa
existir em disco, e `README.md` não conta porque é citado para listar tarefas, não
para nomear dono. E o caminho do documento é resolvido em duas raízes, a do umbrella
e a do repositório da TASK: a spec de um módulo mora em `apps/player/specs/ui.md`, e
um bullet do player que diz `specs/ui.md` cita autoridade real — olhar só a raiz do
umbrella reprovaria o bullet correto. Os 113 bullets foram então reescritos para
nomear seu dono, preservando o argumento original de cada um, e o gate passa a
reportar `{"blocking":0,"result":"passed","tasks":199,"warnings":15}`; os 15 avisos
são bullets curtos e `When` curto legítimos e não bloqueiam.
`engineering/04-ci.md` e `CONTRIBUTING.md` foram corrigidos para listar os jobs reais
e os comandos que um contribuidor de fato roda. Somente Markdown e ferramental:
nenhum código de player, 191 requisitos, 199 tarefas, 19 épicos.

## Unreleased — 2026-11-09 (TASK-E19-04, TASK-CI-01 and TASK-CI-02: own repos, tasks gate) / Não lançado — 2026-11-09 (TASK-E19-04, TASK-CI-01 e TASK-CI-02: repos próprios, gate de TASKs)

**EN** — Three tasks closed, and the interesting part of each is what was wrong
before. `TASK-E19-04`: `02-vynamp-converter.md` still described `vynamp-encode` as a
*private* repository, false since the repositories went public, and neither canonical
tool spec cited a repository URL at all; both now do, read back from the REST API. The
three module `specs/README.md` files opened with "Normative: the code follows the spec",
which puts the module repository itself as the source — they now state that the
normative text lives in the main repository and point at the canonical document, and
each `specs/*.md` header repeats it. `vynamp-encode` has no canonical document of its
own: its norm is §1 of the converter spec, and that is where its skeleton points
rather than inventing a fourth document. `vynamp-fases` is not a repository
(`GET` returns 404), so the reference to `01-arvore-final.md` is plain text, not a
link. The three repositories now carry `LICENSE`, `LICENSE-MIT`, `LICENSE-APACHE`,
`SECURITY.md` and `CONTRIBUTING.md` in addition to the bilingual READMEs, and
`visibility=public` was read from the API rather than assumed. `TASK-CI-02`: a new
`cargo xtask check-tasks` and a `tasks` job in `ci.yml`, required by the gate. It
enforces six rules over the backlog and the TASK Markdown, and its `AC-4`/`AC-5`
contradict each other as written — a red naming *exactly five* tasks and a green once
those five are written cannot both hold at one commit, because six tasks were outside
`planned` without written criteria. The rule applied is the one both describe: every
task outside `planned` whose criteria are not written fails the gate. No exception was
added between the red and the green; the seven tasks were authored instead, and
`TASK-E19-06`'s `Then` column was left untouched even where it says "seven repos"
where there are ten, because that column is the approved requirement. `TASK-CI-01`
was already met except for `AC-4`: the unpinned-action error named the file *stem* and
no line. It now reports `Unpinned action <ref> at ci.yml:<line> (<yaml path>)`, pairing
each reference with its own line in document order — searching by the action text
would give every repeated `uses:` the first one's line, which is the case the
criterion exists for. `engineering/04-ci.md` described the four pre-step-34 jobs and
an `apps/desktop` that does not exist; corrected to the five real jobs. Documentation,
tooling and repository governance only: no player code, 191 requirements, 193 tasks,
19 epics.

**PT** — Três TASKs fechadas, e a parte interessante de cada uma é o que estava
errado antes. `TASK-E19-04`: `02-vynamp-converter.md` ainda descrevia `vynamp-encode`
como repositório *privado*, falso desde a abertura, e nenhuma das specs canônicas
citava URL de repositório; ambas citam agora, lidas de volta da API REST. As três
`specs/README.md` de módulo abriam com "Normativas: o código segue a spec", o que põe
o repo de módulo como fonte — passam a declarar que o texto normativo vive no
repositório principal e apontam o documento canônico, e o cabeçalho de cada
`specs/*.md` repete isso. `vynamp-encode` não tem documento canônico próprio: sua
norma é §1 da spec do converter, e é para lá que o esqueleto aponta em vez de inventar
um quarto documento. `vynamp-fases` não é repositório (`GET` devolve 404), então a
referência a `01-arvore-final.md` é texto, não link. Os três repositórios passam a ter
`LICENSE`, `LICENSE-MIT`, `LICENSE-APACHE`, `SECURITY.md` e `CONTRIBUTING.md` além dos
READMEs bilíngues, e `visibility=public` foi lido da API, não presumido. `TASK-CI-02`:
novo `cargo xtask check-tasks` e job `tasks` no `ci.yml`, exigido pelo gate. Aplica
seis regras sobre o backlog e o Markdown das TASKs, e seus `AC-4`/`AC-5` contradizem-se
como escritos — um vermelho nomeando *exatamente cinco* TASKs e um verde quando essas
cinco estiverem escritas não cabem no mesmo commit, porque seis estavam fora de
`planned` sem critério escrito. A regra aplicada é a que os dois descrevem: toda TASK
fora de `planned` sem critério escrito reprova. Nenhuma exceção foi introduzida entre
o vermelho e o verde; as sete TASKs foram autoradas. A coluna `Then` da `TASK-E19-06`
ficou intacta mesmo dizendo "sete repos" onde são dez, porque aquela coluna é o
requisito aprovado. `TASK-CI-01` já estava atendida exceto no `AC-4`: o erro de ação
sem SHA nomeava o *file stem* e nenhuma linha. Agora reporta `Unpinned action <ref> at
ci.yml:<linha> (<caminho yaml>)`, casando cada referência com a sua própria linha na
ordem do documento — buscar pelo texto da ação daria a todas as ocorrências repetidas
de `uses:` a linha da primeira, que é justamente o caso que o critério existe para
cobrir. `engineering/04-ci.md` descrevia os quatro jobs de antes do passo 34 e um
`apps/desktop` que não existe; corrigido para os cinco jobs reais. Somente
documentação, ferramental e governança de repositório: nenhum código de player, 191
requisitos, 193 tarefas, 19 épicos.

## Unreleased — 2026-11-08 (TASK-E19-06: repositórios públicos e branches protegidas) / Não lançado — 2026-11-08 (TASK-E19-06: repositórios públicos e branches protegidas)

**EN** — TASK-E19-06 executed. The ten repositories are public with `main` and
`develop` protected: `CI Gate` required in strict mode, required pull request with
stale-review dismissal, conversation resolution, admins subject to the rules, and
force push and deletion blocked. `cargo xtask repo protect --repo <owner/name>
--reviews 0` applied it and `cargo xtask repo audit` reads it back with no pending
item. `--reviews 0` is not a shortcut: with a single collaborator account and
`enforce_admins=true`, `--reviews 1` would leave every pull request unmergeable,
including the maintainer's. Three defects were found and fixed on the way: private
vulnerability reporting was off in all nine public repositories, so `AC-1` was not
met; `vynamp-output`'s description ended in "(private; opening later)", exactly what
`AC-3` forbids, and was the only one of the ten filled in; and `SECURITY.md`,
`CODE_OF_CONDUCT.md`, `CONTRIBUTING.md` and `README.md` told readers to open a
public issue because the channel "goes live in TASK-E19-06" while pointing at
`vynamp-mediaplayer`, now archived. The advisory URLs move to `jimmykedlyn/vynamp`.
`vynamp-mvp-reference` is deliberately excluded from branch protection: it has no
`develop` and no workflow, so requiring `CI Gate` there would block every pull
request instead of guarding one; it stays public with reporting enabled. Two
Woodpecker webhooks were deleted (decode, output) and `vynamp-mediaplayer` and
`vitrola` were archived. Still open: the six module repositories without `LICENSE`
are public, which is TASK-E19-04. Documentation and remote configuration only: no
player code, 191 requirements, 191 tasks, 19 epics.

**PT** — TASK-E19-06 executada. Os dez repositórios são públicos com `main` e
`develop` protegidas: `CI Gate` exigido em modo strict, pull request obrigatório
com dismiss de revisão obsoleta, conversações resolvidas, administradores sujeitos
às regras, e force push e exclusão bloqueados. `cargo xtask repo protect --repo
<owner/name> --reviews 0` aplicou, e `cargo xtask repo audit` lê de volta sem
pendência. `--reviews 0` não é atalho: com uma única conta colaboradora e
`enforce_admins=true`, `--reviews 1` deixaria todo pull request sem merge possível,
inclusive o do mantenedor. Três defeitos foram encontrados e corrigidos no caminho:
o relato privado de vulnerabilidade estava desligado nos nove repositórios
públicos, então `AC-1` não estava atendido; a descrição de `vynamp-output` terminava
em "(privada; abertura futura)", exatamente o que `AC-3` proíbe, e era a única das
dez preenchida; e `SECURITY.md`, `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md` e
`README.md` mandavam abrir issue pública porque o canal "entra no ar na
TASK-E19-06", apontando para `vynamp-mediaplayer`, agora arquivado. As URLs de
advisory passam para `jimmykedlyn/vynamp`. `vynamp-mvp-reference` fica de fora da
proteção de propósito: não tem `develop` nem workflow, então exigir `CI Gate` ali
bloquearia todo pull request em vez de guardar um; segue público com o relato
habilitado. Dois webhooks do Woodpecker foram apagados (decode, output) e
`vynamp-mediaplayer` e `vitrola` foram arquivados. Continua aberto: os seis
repositórios de módulo sem `LICENSE` estão públicos, o que é a TASK-E19-04.
Somente documentação e configuração remota: nenhum código de player, 191
requisitos, 191 tarefas, 19 épicos.

## Unreleased — 2026-11-08 (step 33: agent definitions) / Não lançado — 2026-11-08 (passo 33: definições de agente)

**EN** — Step 33 of the phase-3 reorganization. `.claude/agents/` now holds the
four roles the agent contract described in prose only: `orquestrador`,
`implementador`, `checador` and `revisor`, each with an explicit `model:` in its
frontmatter and no inherited session model. The tiers are the ones already recorded
in ADR-007: **Opus orchestrates, Sonnet implements and reviews, Haiku runs the
checks.** A high-risk TASK — real-time audio, concurrency, security, release —
raises the implementer for that TASK to Opus before the call. The reviewer never
corrects code: it reports, another session fixes, otherwise the reviewer ends up
defending its own work. `cargo xtask check-docs` stops scanning `.claude/`: agent definitions
open with a YAML frontmatter block, not with the `# ` title the check requires, and
they are configuration rather than specification. The exclusion is scoped to the new
`util::doc_files()` and leaves `util::files()` untouched: that walker also feeds
`copy_tree` and the Python-artifact gate, and excluding `.claude/` there would have
dropped `.claude/settings.json` from the distribution import target and blinded the
gate. Because no documentation check sees `.claude/` any more, `check-delivery` gains
the frontmatter gate: opening and closing `---`, the four required keys, an unknown or
repeated key, and a `model:` outside `opus|sonnet|haiku` are all errors — otherwise a
typo silently stops the definition from loading. Tooling and configuration only: no
player code, 191 requirements, 191 tasks, 19 epics.

**PT** — Passo 33 da reorganização da fase 3. O `.claude/agents/` passa a ter os
quatro papéis que o contrato do agente descrevia só em prosa: `orquestrador`,
`implementador`, `checador` e `revisor`, cada um com `model:` explícito no
frontmatter e nenhuma herança do modelo da sessão. Os tiers são os que a ADR-007 já
registra: **Opus orquestra, Sonnet implementa e revisa, Haiku executa os checks.**
TASK de alto risco — áudio/tempo real, concorrência, segurança, release — sobe o
implementador daquela TASK para Opus antes da chamada. O revisor nunca corrige código: ele aponta,
outra sessão corrige, senão o revisor acaba defendendo o próprio trabalho. O
`cargo xtask check-docs` deixa de varrer `.claude/`: definições de agente começam
com bloco de frontmatter YAML, não com o título `# ` que a checagem exige, e são
configuração, não especificação. A exclusão fica no novo `util::doc_files()` e não
toca `util::files()`: aquele walker também alimenta o `copy_tree` e o gate de artefato
Python, e excluir `.claude/` ali teria tirado `.claude/settings.json` do alvo do
import de distribuição e cegado o gate. Como nenhuma checagem de documentação vê
`.claude/` agora, o `check-delivery` ganha o gate do frontmatter: abertura e
fechamento `---`, as quatro chaves obrigatórias, chave desconhecida ou repetida, e
`model:` fora de `opus|sonnet|haiku` são erro — sem ele, um erro de digitação para a
definição de carregar em silêncio. Somente ferramentaria e configuração: nenhum código
de player, 191 requisitos, 191 tarefas, 19 épicos.

## Não lançado — 2026-11-07 (passo 30: limpeza do umbrella)

**EN** — Step 30 of the phase-3 reorganization. **M4:** the specification now
carries **16** VU meters, not 12 — every count derived from the old number was
rewritten. **M6:** Woodpecker is removed in full; CI is GitHub Actions only
([ADR-014](adr/ADR-014-publico-dsp-interno.md)),
and `_arquivo/` is created with `INDEX.md`, which records the reason for every
file it holds. **M11:** the inherited anchor SHA `29e14e6` is recorded. The
`cargo xtask bundle-spec` subcommand is gone: it packaged the five
`bootstrap/*` templates, all of which are now archived, so the command could
only fail; its only job was producing the ZIP package this umbrella no longer
is. `VALIDATION.json` stops being tracked — it is generated by
`cargo xtask ci docs` and now listed in `.gitignore`, and the three documents
that linked to it name the command instead, because a fresh checkout never had
the file. The `.gitignore` negation that keeps pipeline `.log` files in
`docs/evidence/` is narrowed to `*.log`, so it no longer reopens `.env`,
`*.p12` or `*.jks` files placed there. **M4, second count:** the VU finishes go
from three to **six** — `mvp/meter-finish.js` in the rev.38 pin defines
`original`, `integrated` (the default), `frameless`, `ultrafine`, `adaptive`
and `recessed`, and no finish is called "Metal adaptativo"; D-DSP-18 carries a
dated correction rather than a rewrite. Documentation and tooling only: no
player code, 191 requirements, 191 tasks, 19 epics.

**PT** — Passo 30 da reorganização da fase 3. **M4:** a especificação passa a
ter **16** VU meters, não 12 — toda contagem derivada do número antigo foi
reescrita. **M6:** o Woodpecker é removido por inteiro; o CI é só GitHub
Actions ([ADR-014](adr/ADR-014-publico-dsp-interno.md)),
e `_arquivo/` é criado com `INDEX.md`, que registra o motivo de cada arquivo
guardado ali. **M11:** o SHA-âncora herdado `29e14e6` fica registrado. O
subcomando `cargo xtask bundle-spec` sai: ele empacotava os cinco templates de
`bootstrap/*`, todos agora arquivados, então o comando só podia falhar; seu
único serviço era produzir o ZIP que este umbrella deixou de ser.
`VALIDATION.json` deixa de ser versionado — é gerado por
`cargo xtask ci docs` e passa a constar do `.gitignore`; os três documentos que
apontavam para ele nomeiam o comando, porque uma cópia nova do repositório
nunca teve o arquivo. A negação do `.gitignore` que mantém `.log` de pipeline em
`docs/evidence/` fica restrita a `*.log`, e deixa de reabrir `.env`, `*.p12` ou
`*.jks` colocados ali. **M4, segunda contagem:** os acabamentos de VU passam de
três para **seis** — `mvp/meter-finish.js` do pin rev.38 define `original`,
`integrated` (o padrão), `frameless`, `ultrafine`, `adaptive` e `recessed`, e
não existe acabamento chamado "Metal adaptativo"; a D-DSP-18 recebe correção
datada em vez de reescrita. Somente documentação e ferramentaria: nenhum código
de player, 191 requisitos, 191 tarefas, 19 épicos.

## Não lançado — 2026-11-07 (o que o spike E01-02 provou entra nas specs)

**EN** — Salvage of the TASK-E01-02 spike (PR #14, still a draft) into the
canonical specification, before the phase-3 move. Four things are absorbed
into [01-engine.md](reference/audio/01-engine.md): the **SPSC ring protocol**
(one allocation at construction, monotonic `u64` counters, Release stores of the
write/read counters after writing/reading, neither side waits), the exact
**presented clock** contract (`presented = max(previous, content written before this callback −
latency of the buffer's first frame)`; the backend's latency measures against
content delivered by *earlier* callbacks, and measuring it against content
already added overstates position by one device buffer — a real defect of the
spike, fixed there with tests that failed first), the **single-writer seqlock**
that publishes it, and the **drain ordering** rule (read the end-of-stream flag
before measuring the ring, or the end of every playback counts as an
underrun). Nothing of the crates comes over: [ADR-011](adr/ADR-011-pilha-propria.md)
sends audio output to the `vynamp-output` repository and already records that
the spike crate collides by name with that library. [02-performance](quality/02-performance.md)
gains the RT instrumentation recipe that actually worked, including the two
lessons the spike paid for: count allocations **per thread** (a process-wide
counter also counts other tests' RT sections running in parallel — that is why
`render_allocates_nothing` failed in CI with two events that were not the
render's), and concurrency tests must not depend on the scheduler. [04-outputs-hires](reference/audio/04-outputs-hires.md)
records the measured order of magnitude as an expectation reference, not a
target. The two raw probe JSONs, their SHA-256, the environment and the
50-run loop of the clock test go to `docs/evidence/PR/task-e01-02/`, and [TASK-E01-02](libs/output/tasks/items/TASK-E01-02.md)
now states each acceptance criterion with what was measured and what is still
missing. The same section of [01-engine.md](reference/audio/01-engine.md) also
registers the clock's **known limitation** — queued underrun or drain silence
also counts as latency, so `presented` is understated by at most
`latency_frames`, never overstated and never regressing, until the full drain
of TASK-E01-05. Documentation only: no player code, 191 requirements, 191
tasks, 19 epics.

**PT** — Aproveitamento do spike da TASK-E01-02 (PR #14, ainda draft) na
especificação canônica, antes da mudança da fase 3. Quatro coisas entram em
[01-engine.md](reference/audio/01-engine.md): o **protocolo do ring SPSC** (uma
alocação na construção, contadores monotônicos `u64`, store Release dos
contadores de escrita/leitura depois de gravar/ler, nenhum dos lados espera), o
contrato exato do **clock apresentado** (`presented = max(anterior, conteúdo escrito antes desta callback
− latência do primeiro frame do buffer)`; a latência do backend se mede contra
o conteúdo entregue nas callbacks *anteriores*, e medi-la contra o conteúdo já
acrescentado superestima a posição em um buffer — defeito real do spike,
corrigido ali com testes que falharam antes), o **seqlock de escritor único**
que o publica, e a ordem de **drenagem** (ler a marca de fim de stream antes de
medir o ring, senão o fim de toda reprodução vira underrun). Nada dos crates
vem: a [ADR-011](adr/ADR-011-pilha-propria.md) manda a
saída de áudio para o repositório `vynamp-output` e já registra que o crate do
spike colide de nome com essa biblioteca. [02-performance](quality/02-performance.md)
recebe a receita de instrumentação RT que de fato funcionou, com as duas lições
que o spike pagou: contar alocação **por thread** (um contador de processo
conta também as seções RT de outros testes em paralelo — foi por isso que
`render_allocates_nothing` falhou no CI com dois eventos que não eram do
render), e teste de concorrência não pode depender do escalonador.
[04-outputs-hires](reference/audio/04-outputs-hires.md) registra a ordem de grandeza
medida como referência de expectativa, não como meta. Os dois JSON do probe, o
SHA-256 deles, o ambiente e a repetição de 50 execuções do teste de clock vão
para `docs/evidence/PR/task-e01-02/`, e a [TASK-E01-02](libs/output/tasks/items/TASK-E01-02.md)
passa a declarar cada critério de aceite com o que foi medido e o que falta. A
mesma seção de [01-engine.md](reference/audio/01-engine.md) registra também a
**limitação conhecida** do clock — silêncio de underrun ou de drenagem
enfileirado conta como latência, então `presented` fica subestimado em no
máximo `latency_frames`, nunca superestimado e nunca regredindo, até o dreno
completo da TASK-E01-05. Somente documentação: nenhum código de player, 191
requisitos, 191 tarefas, 19 épicos.

## Estado herdado — reorganizacao em um repo por modulo

Data: 2026-11-07

Este repositorio substitui `jimmykedlyn/vynamp-mediaplayer`, que passa a ser
arquivado (somente leitura). O ultimo commit herdado foi `29e14e6` em `develop`.
Os 109 commits anteriores e os PRs #1-#39 permanecem legiveis no repositorio
arquivado; citacoes a PR nas specs usam o nome completo quando ambiguas.
