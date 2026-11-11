# Vynamp

**English** · [Português](#português--vynamp)

A multiplatform audio player with a Rust core, an original interface, and its
own themes, presets and plugin formats. Windows, Linux, macOS (Intel and Apple
Silicon) and Android.

> **Status: specification.** The player is not implemented. This repository is
> the normative specification, the Rust delivery tooling (`cargo xtask`) and the
> CI that validates both. It contains no Winamp code or third-party assets, no
> running player and no shipped installer. Performance and quality targets are
> future acceptance criteria, not measured results.

Licence: `MIT OR Apache-2.0` — see [LEGAL.md](LEGAL.md).
Security: [GitHub Security Advisories](https://github.com/jimmykedlyn/vynamp/security/advisories/new) — see [SECURITY.md](SECURITY.md).
Contributing: [CONTRIBUTING.md](CONTRIBUTING.md) · Governance: [GOVERNANCE.md](GOVERNANCE.md).

## Repository layout

The project is **one repository per module**. This one is the umbrella: it holds
the specifications that cross modules, the ADRs, the cross-module tasks and the
delivery tooling.

| Path | Repository | Kind |
|---|---|---|
| `libs/dsp` | `vynamp-dsp` | signal primitives, no domain |
| `libs/decode` | `vynamp-decode` | decoding to PCM |
| `libs/encode` | `vynamp-encode` | encoding |
| `libs/output` | `vynamp-output` | device output |
| `libs/engine` | `vynamp-engine` | integration, domain, DSP |
| `apps/player` | `vynamp-player` | player |
| `apps/converter` | `vynamp-converter` | batch converter |
| `apps/restore` | `vynamp-restore` | restoration, separate process |
| `reference/vynamp-mvp` | `vynamp-mvp-reference` | read-only reference |

Each module is a **submodule pinned at a commit on `develop`**, never at a
branch. The pin is a merge commit, so a clone reaches it regardless of which
branches exist later. `reference/vynamp-mvp` follows the same rule and has no
`develop` branch.

Dependency direction:

```
apps -> engine -> { decode, output, encode, dsp }
```

`decode`, `encode` and `output` depend on no domain `vynamp-*` crate. Every
library compiles on its own: `git clone` + `cargo test`, without this repository.

## Where to start

1. [Master contract](SPEC.md), [full index](INDEX.md), [scope](product/01-visao-escopo.md) and the [requirement matrix](product/03-requisitos.md).
2. [Platform support matrix](platforms/01-matriz-suporte.md), [Android](platforms/02-android.md), [visual system](design/01-sistema-visual.md), [layouts](design/02-layouts.md) and the [visual reference](assets/layout-reference.html).
3. [Audio engine](reference/audio/01-engine.md), [formats](reference/audio/02-formats.md), [streaming](streaming/01-cliente.md).
4. [Architecture](architecture/01-sistema.md), [data](architecture/03-dados.md), [contracts](architecture/04-contratos.md).
5. [Roadmap](delivery/01-roadmap.md), [backlog](tasks/README.md) and [next work](tasks/NEXT.md).

The specs of `audio/` and `tools/` are frozen under [reference/](reference/LEIA-ME.md)
until each module repository is created; the migration plan moves each file to
its module on the step that creates that repository.

## Conventions

- `MUST` is mandatory in the stated phase; `SHOULD` needs an ADR to deviate; `MAY` is optional.
- `P0` is the critical path of a phase, `P1` completes it, `P2` is planned expansion. Priority is not chronological order.
- `M0` technical proofs · `M1` local alpha · `M2` multiplatform beta · `M3` 1.0 on the five targets · `M4` platform · `M5` expansion. The 1.0 promise does not include M4/M5.
- `REQ-*`, `TASK-*`, `ADR-*` and `QA-*` are stable identifiers.
- The [decisions register](product/05-decisoes.md) records choices made so work is not blocked on questions.

## Tooling and verification

Engineering automation is Rust: [repository layout](engineering/08-reinicio-windows.md), [agents and Superpowers](AGENTS.md), [Gitflow](engineering/03-gitflow.md), [CI/CD](engineering/04-ci.md), [tests](engineering/05-testes.md), [installers](delivery/04-gates.md).

```bash
sh setup.sh                       # once per clone: links .githooks into core.hooksPath
cargo xtask check-docs
cargo xtask check-delivery
cargo test --workspace --locked
```

`setup.sh` is required once per clone: `core.hooksPath` lives in `.git/config`,
which is not versioned, so a fresh clone commits with no local rule at all until
it runs. The server-side branch protection still blocks a direct pull request to
`develop`, but the local `guard-branch` is what catches the commit before it
exists. Each module repository has its own `setup.sh`; the one here does not
reach the submodules.

CI runs on GitHub Actions: on every pull request to `develop`, on push to
`develop`/`main`, and never on a bare feature branch. The required status is
`CI Gate`, and branch protection enforces it on `main` and `develop` — a pull
request cannot merge with the gate red, admins included.

Auxiliary deliverables: [design tokens](assets/design-tokens.json), [schema examples](schemas/README.md), [tasks as JSON](tasks/backlog.json), [documentation validator](xtask/src/validate.rs), `VALIDATION.json` (generated by `cargo xtask ci docs`, not tracked), [changelog](CHANGELOG.md).

Repository: `jimmykedlyn/vynamp`. It is **public** since TASK-E19-06
(2026-11-08), when the ten repositories moved to public with branch protection
applied, as [ADR-014](adr/ADR-014-publico-dsp-interno.md) decided.
`vynamp-mediaplayer` is the pre-reorganization repository, archived and read-only.

Identity: [Vynamp C / Pulso brand guide](design/04-marca.md) and the
[approved board](assets/branding/approved-c-board.png). Native exports are still
pending, per the manifest. The PNGs are raster references, not vector sources or
compiled `ICO`/`ICNS`/Android resources.

Related repositories, one per module: `vynamp-dsp` (signal primitives),
`vynamp-decode` (decoding), `vynamp-encode` (encoding), `vynamp-output` (audio
output), `vynamp-engine` (integration), `vynamp-player`, `vynamp-converter` and
`vynamp-restore` as separate applications, plus `vynamp-mvp-reference` (the
read-only visual reference for the native UI).

---

# Português — Vynamp

[English](#vynamp) · **Português**

Player de áudio multiplataforma com núcleo Rust, interface original e formatos
próprios de temas, presets e plugins. Windows, Linux, macOS (Intel e Apple
Silicon) e Android.

> **Estado: especificação.** O player não está implementado. Este repositório é
> a especificação normativa, o tooling de entrega em Rust (`cargo xtask`) e o CI
> que valida os dois. Não contém código do Winamp nem assets de terceiros, não
> contém player em execução nem instalador publicado. Metas de desempenho e
> qualidade são critérios futuros de aceite, não resultados medidos.

Licença: `MIT OR Apache-2.0` — ver [LEGAL.md](LEGAL.md).
Segurança: [GitHub Security Advisories](https://github.com/jimmykedlyn/vynamp/security/advisories/new) — ver [SECURITY.md](SECURITY.md).
Contribuições: [CONTRIBUTING.md](CONTRIBUTING.md) · Governança: [GOVERNANCE.md](GOVERNANCE.md).

## Estrutura do repositório

O projeto é **um repositório por módulo**. Este é o umbrella: guarda as specs que
atravessam módulos, as ADRs, as TASKs cross-módulo e o tooling de entrega.

| Caminho | Repositório | Tipo |
|---|---|---|
| `libs/dsp` | `vynamp-dsp` | primitivas de sinal, sem domínio |
| `libs/decode` | `vynamp-decode` | decodificação para PCM |
| `libs/encode` | `vynamp-encode` | codificação |
| `libs/output` | `vynamp-output` | saída para device |
| `libs/engine` | `vynamp-engine` | integração, domínio, DSP |
| `apps/player` | `vynamp-player` | player |
| `apps/converter` | `vynamp-converter` | conversor em lote |
| `apps/restore` | `vynamp-restore` | restauração |
| `reference/vynamp-mvp` | `vynamp-mvp-reference` | referência read-only |

Cada módulo entra como **submódulo fixado em commit na `develop`**, nunca em
branch. O pin é um merge commit, então um clone o alcança independentemente das
branches que existirem depois. `reference/vynamp-mvp` segue a mesma regra e não
tem branch `develop`.

Direção de dependência:

```
apps -> engine -> { decode, output, encode, dsp }
```

`decode`, `encode` e `output` não dependem de nenhum crate `vynamp-*` de domínio.
Cada lib compila sozinha: `git clone` + `cargo test`, sem este repositório.

## Ordem de leitura

1. [Contrato mestre](SPEC.md), [índice completo](INDEX.md), [escopo](product/01-visao-escopo.md) e [matriz de requisitos](product/03-requisitos.md).
2. [Plataformas obrigatórias](platforms/01-matriz-suporte.md), [Android](platforms/02-android.md), [Design](design/01-sistema-visual.md), [telas](design/02-layouts.md) e [referência visual](assets/layout-reference.html).
3. [Áudio](reference/audio/01-engine.md), [formatos](reference/audio/02-formats.md), [streaming](streaming/01-cliente.md).
4. [Arquitetura](architecture/01-sistema.md), [dados](architecture/03-dados.md), [contratos](architecture/04-contratos.md).
5. [Roadmap](delivery/01-roadmap.md), [backlog](tasks/README.md) e [próxima execução](tasks/NEXT.md).

As specs de `audio/` e `tools/` ficam congeladas em [reference/](reference/LEIA-ME.md)
até cada repositório de módulo ser criado; o plano de migração leva cada arquivo
para o módulo correspondente no passo que cria aquele repositório.

## Como usar

- MUST / DEVE: obrigatório na fase indicada; SHOULD / RECOMENDADO: exceção justificada por ADR; MAY / OPCIONAL: capacidade adicional.
- P0: caminho crítico da fase; P1: necessário para completar a fase; P2: expansão planejada. Prioridade não é ordem cronológica.
- M0: provas técnicas; M1: alpha local; M2: beta multiplataforma; M3: versão 1.0 nos cinco alvos; M4: plataforma; M5: expansão. A visão completa inclui todas as fases, mas a versão 1.0 não promete entregas M4/M5.
- Requisitos `REQ-*`, tarefas `TASK-*`, decisões `ADR-*` e cenários `QA-*` são identificadores estáveis.
- O [registro de decisões](product/05-decisoes.md) explicita escolhas feitas para permitir execução sem bloquear em perguntas.

## Tooling e verificação

A automação de engenharia é Rust: [estrutura](engineering/08-reinicio-windows.md), [agentes/Superpowers](AGENTS.md), [Gitflow](engineering/03-gitflow.md), [CI/CD](engineering/04-ci.md), [testes](engineering/05-testes.md), [instaladores](delivery/04-gates.md).

```bash
sh setup.sh                       # uma vez por clone: liga .githooks ao core.hooksPath
cargo xtask check-docs
cargo xtask check-delivery
cargo test --workspace --locked
```

O `setup.sh` é obrigatório uma vez por clone: `core.hooksPath` mora em
`.git/config`, que não é versionado, então um clone novo commita sem regra local
nenhuma até rodá-lo. A branch protection do servidor continua barrando pull
request direto para `develop`, mas é o `guard-branch` local que pega o commit
antes de ele existir. Cada repositório de módulo tem o próprio `setup.sh`; o
daqui não alcança os submódulos.

O CI roda em GitHub Actions: em todo pull request para `develop`, em push para
`develop`/`main`, e nunca em branch de feature isolada. O status exigido é
`CI Gate`, e a branch protection o exige em `main` e `develop` — um pull request
não é mergeado com o gate vermelho, inclusive para administradores.

Entregáveis auxiliares: [tokens visuais](assets/design-tokens.json), [exemplos de contratos](schemas/README.md), [tarefas em JSON](tasks/backlog.json), [validador documental](xtask/src/validate.rs), `VALIDATION.json` (gerado por `cargo xtask ci docs`, não versionado), [histórico](CHANGELOG.md).

Repositório: `jimmykedlyn/vynamp`. É **público** desde a TASK-E19-06
(2026-11-08), quando os dez repositórios passaram a público com a branch
protection aplicada, como a [ADR-014](adr/ADR-014-publico-dsp-interno.md)
decidiu. `vynamp-mediaplayer` é o repositório anterior à reorganização,
arquivado e somente leitura.

Identidade: [guia Vynamp C / Pulso](design/04-marca.md) e [prancha
aprovada](assets/branding/approved-c-board.png). Exports nativos ainda pendentes,
conforme manifest. Os PNGs são referências raster, não fontes vetoriais nem
recursos `ICO`/`ICNS`/Android compilados.

Repositórios relacionados, um por módulo: `vynamp-dsp` (primitivas de sinal),
`vynamp-decode` (decodificação), `vynamp-encode` (codificação), `vynamp-output`
(saída de áudio), `vynamp-engine` (integração), `vynamp-player`,
`vynamp-converter` e `vynamp-restore` como aplicações separadas, além de
`vynamp-mvp-reference` (referência visual read-only da UI nativa).
