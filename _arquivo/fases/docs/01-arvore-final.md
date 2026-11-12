# Árvore final do umbrella

Destino: `D:\Projects\vynamp` → `jimmykedlyn/vynamp`. Cada módulo é **submódulo fixado
em tag de release**, nunca em branch.

---

## 1. Árvore de diretórios

```
D:\Projects\vynamp\                         ← repo umbrella (git)
├── AGENTS.md                               contrato único do agente
├── CLAUDE.md                               4 linhas: @AGENTS.md
├── README.md                               mapa do projeto e dos repos
├── SPEC.md                                 contrato mestre do produto
├── STATE.md                                gerado do backlog.json
├── CHANGELOG.md                            histórico (menções a Woodpecker preservadas)
├── CONTRIBUTING.md  GOVERNANCE.md  LEGAL.md
├── SECURITY.md  CODE_OF_CONDUCT.md
├── LICENSE  LICENSE-APACHE  LICENSE-MIT
├── Cargo.toml                              workspace virtual: membros = xtask só
├── Cargo.lock
├── rust-toolchain.toml
├── .editorconfig  .gitattributes  .gitignore
├── .cargo/config.toml
├── .githooks/{pre-commit,pre-push}
├── .claude/settings.json                   (settings.local.json NÃO é versionado)
├── .github/
│   ├── workflows/{ci-gate,docs-check,dependency-review}.yml
│   └── pull_request_template.md
├── .gitmodules                             libs/* e apps/* e reference/* (9 entradas)
│
├── architecture/                           specs que atravessam módulos
│   ├── 01-sistema.md                       camadas, direção de dependência, isolamento
│   ├── 02-stack.md
│   ├── 03-dados.md
│   ├── 04-contratos.md                     reescrito do zero em M3
│   ├── GLOSSARY.md                         65 termos + decoder/engine/packet/sink
│   └── adr/
│       ├── ADR-001..ADR-014                ADR-010 sai (M6); ADR-015 entra pelo merge
│
├── product/   01-visao-escopo · 02-benchmark · 03-requisitos (191 REQ)
│              04-jornadas · 05-decisoes · 06-paridade
├── delivery/  01-roadmap · 02-release-devops (absorve engineering/04) · 03-riscos · 04-gates
├── design/    01-sistema-visual · 02-layouts · 03-acessibilidade · 04-marca
│              05-mobile-tablet · 06-painel-dsp · 07-pin-mvp
├── engineering/  01-agentes → absorvido por AGENTS.md
│                 02-worktrees · 03-gitflow · 04-ci (M6) · 05-testes
│                 07-segredos · 08-reinicio-windows
├── extensions/   01-temas · 02-plugins · 04-sdk · 05-visualizador
├── library/      01-biblioteca-playlists · 02-tags-letras
├── platforms/    01-matriz-suporte · 02-android · 03-integracao-sistema
│                 04-ativacao-ownership
├── quality/      01-estrategia-testes · 02-performance · 03-matriz-multiplataforma
│                 04-matriz-dsp (VU-INT-06 = 12 skins × 16 VUs = 192)
├── security/     01-threat-model · 02-privacidade-licencas
├── streaming/    01-cliente · 02-podcasts-gravacao · 03-plataforma · 04-rede-proxy
├── evidence/     evidência de PR, por task
├── tasks/
│   ├── backlog.json                        fonte única de status
│   ├── NEXT.md  README.md  EVIDENCE.md
│   └── epics/E01..E19.md
│
├── libs/                                   ← submódulos
│   ├── dsp/        → vynamp-dsp
│   ├── decode/     → vynamp-decode
│   ├── encode/     → vynamp-encode
│   ├── output/     → vynamp-output
│   └── engine/     → vynamp-engine
├── apps/                                   ← submódulos
│   ├── player/     → vynamp-player
│   ├── converter/  → vynamp-converter
│   └── restore/    → vynamp-restore
├── reference/
│   └── vynamp-mvp/ → vynamp-mvp-reference  (read-only; pin por commit 741316f)
└── xtask/                                  ferramentaria do umbrella
```

Cada subdiretório de `libs/` e `apps/` é o repositório inteiro: seu próprio
`Cargo.toml`, `specs/`, `tasks/`, `.github/workflows/`, `AGENTS.md`, gitflow próprio.

---

## 2. O que cada módulo contém

### `libs/dsp` — `vynamp-dsp`

| | |
|---|---|
| Tipo | lib, std-only, zero `vynamp-*` |
| Crates | `vynamp-dsp` |
| Specs | `specs/dsp.md` |
| Conteúdo | `Resampler`, `Dither`, `ChannelMap`, `Mixer`, ganho/fade, crossfade |
| Depende de | nada além de `std` |
| É usado por | `engine` (obrigatório), `output` e `encode` (opcional, feature `resample`), `restore` |

**Por que existe:** resampler/dither/downmix não podem divergir entre o que o player
toca e o que o conversor grava. Uma diferença de 0,1 dB no downmix entre os dois seria
um bug impossível de achar.

### `libs/decode` — `vynamp-decode` (já existe)

| | |
|---|---|
| Tipo | lib |
| Crates | 9 crates-folha + fachada |
| Código hoje | 29.008 LOC em `feature/mpeg` |
| Specs | `specs/decode.md`, `specs/formatos.md`, `specs/registro-capacidades.md` |
| Recebe spec de | `docs/audio/02-formats.md`, `audio/06-codec-capability-registry.md` |
| Depende de | `std`, `vynamp-dsp` (opcional) |
| Não faz | metadata, CUE (C12 — spec anuncia, crate não existe) |

### `libs/encode` — `vynamp-encode` (novo)

| | |
|---|---|
| Tipo | lib |
| Specs | `specs/encode.md`, `specs/formatos.md` |
| Recebe spec de | `docs/audio/02-formats.md`, `audio/06-*`, `tools/02` |
| Depende de | `std`, `vynamp-dsp` (opcional, feature `resample`) |
| Nota | AAC sem ressalva; MP3 quando o encoder estiver na build (C7) |

### `libs/output` — `vynamp-output` (já existe)

| | |
|---|---|
| Tipo | lib |
| Crates | 3 crates + probe |
| Código hoje | 7.768 LOC em `feature/core` |
| Specs | `specs/output.md`, `specs/backends.md` |
| Recebe spec de | `docs/audio/04-outputs-hires.md` + `05-platform-backends.md` (fundidos) |
| API pública | inclui `PathProperties` (primitivos e enums, sem conhecimento de arquivo) |
| Depende de | `std`, `vynamp-dsp` (opcional, feature `resample`) |

### `libs/engine` — `vynamp-engine` (novo, o maior)

| | |
|---|---|
| Tipo | lib de integração |
| Specs | `specs/engine.md`, `specs/pipeline.md`, `specs/dsp.md`, `specs/equalizadores.md`, `specs/catalogo-dsp.md` |
| Recebe spec de | `docs/audio/01`, `03`, `07`, `08`, `09`, `10` |
| Depende de | `decode`, `encode`, `output`, `dsp` |
| Domínio | `SampleRate`, `FrameCount`, `ChannelLayout`, `MediaTime`, `SourceDescriptor` vivem **só aqui** |
| Interno (não público) | `Processor`, `AudioBlock`, `BlockContext`, `WearState` |
| Não existe | `vynamp-audio`, `vynamp-domain`, `vynamp-app`, `vynamp-core` |

### `apps/player` — `vynamp-player`

| | |
|---|---|
| Tipo | app desktop + Android |
| Specs | `specs/player.md`, `specs/ui.md`, `specs/biblioteca.md`, `specs/streaming.md`, `specs/temas.md`, `specs/plugins.md` |
| Recebe spec de | E02, E03, E05, E06, E07, E09, E13, E16 |
| Depende de | `vynamp-engine` |
| Código | nada hoje |

### `apps/converter` — `vynamp-converter`

| | |
|---|---|
| Tipo | app desktop separado |
| Specs | `specs/converter.md` (recebe `docs/tools/02`), `specs/lote.md` |
| Recebe spec de | E18 |
| Depende de | `vynamp-engine` |

### `apps/restore` — `vynamp-restore`

| | |
|---|---|
| Tipo | app desktop, **processo separado** (ADR-013 §1) |
| Specs | `specs/restore.md` (recebe `docs/tools/01` + `tools/03` do merge), `specs/ia.md` |
| Recebe spec de | E18 + ADR-015 |
| Depende de | `vynamp-engine`, `vynamp-dsp` |
| Nota | modelos de IA **nunca** no instalador; download opt-in com SHA-256 |

### `reference/vynamp-mvp` — `vynamp-mvp-reference` (já existe)

Read-only, pin `741316f` / rev.38. **Fora do workspace** (não é membro do Cargo), mas
**é** entrada do `.gitmodules`: é o único submódulo pinado por **commit**, e não por
tag. Nunca editado.

Por que a diferença: os 8 repos de código têm `develop` + tag `v0.1.0`; a referência
só tem `main` e não recebe tag (é congelada, não versionada). Piná-la por commit
mantém o pin exato e satisfaz a regra do umbrella — que proíbe *branch*, não
*commit*.

---

## 3. Direção de dependência, verificável

```
apps/{player,converter,restore}
        │
        ▼
   vynamp-engine
        │
        ├──▶ vynamp-decode ─┐
        ├──▶ vynamp-encode ─┼──▶ vynamp-dsp
        ├──▶ vynamp-output ─┘        (output/encode: opcional, feature "resample")
        └──▶ vynamp-dsp
```

Regras que entram no CI como verificação, não como recomendação:

| Regra | Como se verifica |
|---|---|
| `decode`/`encode`/`output` não dependem de `vynamp-*` de domínio | `cargo tree` sem `vynamp-engine`/`vynamp-player`/entre si |
| `dsp` não depende de nada | `cargo tree -p vynamp-dsp` só com `std` |
| Só o `engine` tem mais de uma lib do projeto | grep nos `Cargo.toml` |
| Ninguém depende de `apps` | `cargo tree` |
| Libs compilam isoladas | `cargo build --manifest-path libs/<x>/Cargo.toml` |

---

## 4. Repos que saem de cena

| Repo | Situação |
|---|---|
| `jimmykedlyn/vynamp-mediaplayer` | **arquivado** após o primeiro commit verde do umbrella; segue legível e read-only |
| `jimmykedlyn/vitrola` | fica como está; fora de todas as specs (M10) |
| `D:\Projects\vynamp-chatgpt-arquivo` | não é git, não migra, fora das specs (M10) |
| `D:\Projects\vynamp-media-player` (raiz) | fonte da migração; nada dela **exceto** os arquivos mapeados em `00-mapa-destino.csv` |

Consequência registrada em M11: o umbrella nasce com histórico vazio. As citações a
PR (`#36`, `#38`, `#24`) ganham o repo explícito ao serem movidas
(`jimmykedlyn/vynamp-mediaplayer#36`), e `CHANGELOG.md`/`STATE.md` registram a data da
mudança e o SHA `29e14e6` como último estado herdado.
