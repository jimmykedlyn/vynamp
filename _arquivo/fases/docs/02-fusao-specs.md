# Fusão das specs — o que cada arquivo vira

Entrada: 346 `.md` rastreados em `repo/` + 12 `.md` na raiz externa = 358.
Saída depois de M3/M4/M5/M6/M8: **~120 arquivos**, dos quais 40 são specs de módulo
que vivem nos repos de submódulo, não no umbrella.

O arquivo completo com uma linha por arquivo é `00-mapa-destino.csv`. Este documento
explica **o critério** de cada destino e mostra o resultado por categoria.

---

## 1. Critério de destino

| Destino | Quando | Efeito |
|---|---|---|
| **manter** | a norma continua verdadeira e pertence ao umbrella | move, ajusta link |
| **reescrever** | a norma existe mas o conteúdo está revogado (M3/M4/M6) | o arquivo é reescrito, versão antiga vai para `_arquivo/` |
| **fundir** | o conteúdo é duplicata de outro arquivo | conteúdo vai para o alvo; original para `_arquivo/` com motivo |
| **absorver** | o conteúdo é de um módulo, não do umbrella | vira spec no repo do módulo; original para `_arquivo/` |
| **renomear** | colisão de numeração | move com nome novo |
| **arquivar** | histórico, CI morto, plano executado | vai para `_arquivo/repo/<caminho original>` |
| **não migrar** | cache, build, worktree, arquivo já arquivado | fica onde está |

---

## 2. Resultado por diretório

### `docs/architecture/` — 4 arquivos, 14 ADRs

| Arquivo | Vira | Por quê |
|---|---|---|
| `01-system.md` | **reescrever** `architecture/01-sistema.md` | lista 14 crates com `vynamp-audio`/`-domain`/`-app` e sem `engine` (M3) |
| `02-stack.md` | manter | ajustar só "adapters FFmpeg" |
| `03-data.md` | manter | 15 entidades e migrations seguem válidas |
| `04-contracts.md` | **reescrever do zero** | 6 traits num doc, atravessando 5 módulos que devem ser isolados (C2) |
| `ADR-001..009, 011..014` | manter | 002 e 007 ganham banner de "parcialmente/substituída" |
| `ADR-010` | **arquivar** | Woodpecker (M6) |
| `ADR-015` | **entra pelo merge** | já existe em `chore/claude-specs-ia-futura` |

Resultado: 4 specs + 13 ADRs no umbrella, 1 ADR arquivada.

### `docs/audio/` — 10 arquivos, **todos saem do umbrella**

Esta é a mudança estrutural de M3: o áudio não é um domínio do umbrella, é código em
cinco repos. Cada arquivo vira spec de módulo.

| Arquivo | Vai para | Absorvido como |
|---|---|---|
| `01-engine.md` | `libs/engine` | `specs/engine.md` + `specs/pipeline.md` |
| `02-formats.md` | `libs/decode` + `libs/encode` | particionado por capacidade (C7) |
| `03-dsp.md` | `libs/engine` | `specs/dsp.md`; o resampler sai para `libs/dsp` (M9) |
| `04-outputs-hires.md` | `libs/output` | `specs/backends.md` — **absorve 05** |
| `05-platform-backends.md` | `libs/output` | 9 linhas idênticas a 04 → funde |
| `06-codec-capability-registry.md` | `libs/decode` + `libs/encode` | catálogo por capacidade |
| `07-dsp-signal-contract.md` | `libs/engine` | `specs/pipeline.md` — ordem canônica |
| `08-equalizers.md` | `libs/engine` | `specs/equalizadores.md` (5 modalidades, EQ-T) |
| `09-dsp-catalog.md` | `libs/engine` | `specs/catalogo-dsp.md` (~40 módulos) |
| `10-vitrola.md` | `libs/engine` | Vinyl/Tube interno; **sem** repo vitrola (M10) |

Duplicação resolvida: `01` × `07` descreviam o pipeline duas vezes → um só em
`specs/pipeline.md`.

### `docs/design/` — 7 arquivos → 7

| De | Para | Nota |
|---|---|---|
| `01-visual-system.md` | `01-sistema-visual.md` | M4: 12→16 VUs |
| `02-layouts.md` | idem | M4 |
| `03-accessibility-i18n.md` | `03-acessibilidade.md` | — |
| `04-brand-identity.md` | `04-marca.md` | renomeado (colisão) |
| `04-mobile-tablet.md` | `05-mobile-tablet.md` | renomeado; absorve a paleta duplicada |
| `05-dsp-panel.md` | `06-painel-dsp.md` | M4 |
| `06-mvp-reference.md` | `07-pin-mvp.md` | aponta para `reference/` |

### `docs/engineering/` — 9 → 6

| De | Ação |
|---|---|
| `01-agents-superpowers.md` | **absorvido** por `AGENTS.md` §5 (M5) |
| `02-worktrees.md` | manter |
| `03-gitflow-github.md` | `03-gitflow.md` |
| `04-ci-cd.md` | **reescrever** sem Woodpecker (M6), C9 corrigido |
| `05-tests.md` | manter |
| `06-installers.md` | **fundir** em `delivery/04-gates.md` |
| `07-secrets-activation.md` | `07-segredos.md` |
| `08-folder-structure.md` | **arquivar** — descreve raiz de ZIP inexistente |
| `09-windows-restart.md` | `08-reinicio-windows.md`, paths atualizados |

### `docs/quality/` — 5 → 4

`04-integration-codecs-dsp.md` tem QA-022..033 **byte-idênticas** a
`01-test-strategy.md:59-70` → funde. `05` vira `04-matriz-dsp.md` com M4 aplicado.

### `docs/extensions/` — 5 → 4

Duas colisões de `04-` (sdk e visualizer) → `04-sdk.md` e `05-visualizador.md`.
`03-visualizations-integrations.md` é subconjunto → arquivado.

### `docs/streaming/` — 5 → 4

`04-api-semantics.md` duplica endpoints de `architecture/04` → funde lá. Os demais
viram `01-cliente`, `02-podcasts-gravacao`, `03-plataforma`, `04-rede-proxy`.

### `docs/delivery/`, `product/`, `platforms/`, `library/`, `security/` — mantidos

Com M4 aplicado em `product/05-decisoes.md`, `library/02-tags-letras.md` e
`design/*`. `product/08-completeness-audit.md` e `research/03-completeness-sources.md`
são auditoria histórica 0.6 → arquivados.

### `docs/tools/` — 2 → 0 no umbrella

`01-vynamp-restore.md` → `apps/restore/specs/restore.md`.
`02-vynamp-converter.md` → `apps/converter/specs/converter.md`.
`tools/03-vynamp-restore-ia.md` (vem pelo merge) → `apps/restore/specs/ia.md`.

### `docs/evidence/` — 68 arquivos

| Origem | Destino |
|---|---|
| `PR/task-e15-04/**` (57 arquivos, 618 K, 44 `.log`) | arquivar — evidência de Woodpecker (M6) |
| `PR/*/pr-body.md` de PRs fechados por engano (3) | arquivar |
| Restante | `evidence/` no umbrella, como registro datado |

### `docs/superpowers/plans/` — 6 → 0

Todos são planos de tarefas já executadas (E15-01, E15-02, E11-01, E15-03, specs de
shell e de restore). Arquivados.

### Raiz do repo — 12 arquivos

| Arquivo | Vira |
|---|---|
| `SPEC.md` | `SPEC.md` — contrato mestre |
| `README.md` | **reescrever** — umbrella com submódulos |
| `START-HERE.md` | arquivar — descreve `INICIAR.ps1` que não existe |
| `AGENTS.md` + `CLAUDE.md` | arquivar — absorvidos pelo contrato único (M5) |
| `CHANGELOG.md` | manter — menções a Woodpecker preservadas |
| `SECURITY`, `LEGAL`, `GOVERNANCE`, `CONTRIBUTING`, `CODE_OF_CONDUCT`, `LICENSE*` | manter |
| `ARTIFACTS.json` | arquivar — manifesto de outro produto |
| `VALIDATION.json` | arquivar — passa a ser gitignored |

### Raiz externa — 12 `.md` fora do git

| Arquivo | Vira |
|---|---|
| `SKIN-REQUIREMENTS-*` (2, 314 K) | arquivar — descartados por D-MVP-01 |
| `Prompt_Claude_DSP_v1.4` (72 K) | arquivar — já integrado em D-DSP-11..18 |
| `CLAUDE-SPEC-LETRAS-SINCRONIZADAS.md` | arquivar — integrado em `library/02` + D-LYR |
| `TAREFA-SPECS-*.md` (2) | arquivar — ordens de serviço concluídas |
| `ROADMAP-rodadas.md` | arquivar — substituído por `backlog.json` |
| `AUDIOSR-RUST-CPU.md` | arquivar **só após** o merge (M8): absorvido por ADR-015 + `tools/03` |
| `SOURCE_SEPARATION_MODELS.md` | idem |
| `AGENTS.md` | reescrever no umbrella (M5) |
| `STATE.md` | reescrever — gerado do `backlog.json` |
| `LEIA-ME.md` | absorver no `README.md` |

---

## 3. Tasks — 191 itens + 19 épicos (+2 de engenharia no passo 37)

Nenhuma TASK é arquivada: todas são **reescritas** no formato `padroes/tasks.md`
(com Given/When/Then e `Boundary`) e movidas para o repo do módulo dono.

O passo 37 emite, além destas 191, **duas** TASKs de engenharia (`TASK-CI-01` e
`TASK-CI-02`, dono `vynamp`), o que fecha em **193** arquivos e 193 entradas no
`backlog.json` — ver D-37-13 em `execucao/37-tasks.md`. A tabela abaixo continua
contando só as 191 de produto.

| Épico | Módulo dono | Tasks |
|---|---|---|
| E01, E04, E12 | `libs/engine` | 30 |
| E02, E03, E05, E06, E07, E09, E13, E16 | `apps/player` | 70 |
| E17 | `libs/decode` + `libs/encode` | 20 |
| E08, E18 | `apps/converter` + `apps/restore` | 12 |
| E10, E11, E14, E15, E19 | umbrella | 59 |

Os 30 esqueletos de E16/E17 (parágrafo idêntico, zero número ou fixture) são
**reescritos ou arquivados** na 3.7 — não passam para o formato novo sem conteúdo real.

O `backlog.json` continua sendo a **fonte única de status** e passa a ser a origem do
`STATE.md`.

---

## 4. Contagem final

| Categoria | Antes | Depois |
|---|---|---|
| Specs normativas no umbrella | 79 | 62 |
| Specs de módulo (nos repos de submódulo) | 0 | 40 |
| ADRs | 14 | 13 (+1 pelo merge = 14) |
| TASKs | 191 | 193 (191 reescritas + 2 de engenharia, D-37-13) |
| Épicos | 19 | 19 |
| Evidência | 68 | 11 |
| Arquivados (`_arquivo/repo/`) | 0 | 83 arquivos + 57 de evidência |
| **Total de `.md` rastreados** | **346** | **~120** |

Nada é apagado nesta fase. `_arquivo/` guarda tudo com motivo em `_arquivo/INDEX.md`,
e a exclusão definitiva só acontece na 3.8, sob "pode excluir".
