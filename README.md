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
Security: [GitHub Security Advisories](https://github.com/jimmykedlyn/vynamp-mediaplayer/security/advisories/new) — see [SECURITY.md](SECURITY.md).
Contributing: [CONTRIBUTING.md](CONTRIBUTING.md) · Governance: [GOVERNANCE.md](GOVERNANCE.md).

## Where to start

1. [Master contract](SPEC.md), [full index](docs/INDEX.md), [scope](docs/product/01-vision-scope.md) and the [requirement matrix](docs/product/03-requirements.md).
2. [Platform support matrix](docs/platforms/01-support-matrix.md), [Android](docs/platforms/02-android.md), [visual system](docs/design/01-visual-system.md), [layouts](docs/design/02-layouts.md) and the [visual reference](assets/layout-reference.html).
3. [Audio engine](docs/audio/01-engine.md), [formats](docs/audio/02-formats.md), [streaming](docs/streaming/01-client.md).
4. [Architecture](docs/architecture/01-system.md), [data](docs/architecture/03-data.md), [contracts](docs/architecture/04-contracts.md).
5. [Roadmap](docs/delivery/01-roadmap.md), [backlog](tasks/README.md) and [next work](tasks/NEXT.md).

## Conventions

- `MUST` is mandatory in the stated phase; `SHOULD` needs an ADR to deviate; `MAY` is optional.
- `P0` is the critical path of a phase, `P1` completes it, `P2` is planned expansion. Priority is not chronological order.
- `M0` technical proofs · `M1` local alpha · `M2` multiplatform beta · `M3` 1.0 on the five targets · `M4` platform · `M5` expansion. The 1.0 promise does not include M4/M5.
- `REQ-*`, `TASK-*`, `ADR-*` and `QA-*` are stable identifiers.
- The [decisions register](docs/product/05-decisions.md) records choices made so work is not blocked on questions.

## Tooling and verification

Engineering automation is Rust: [START-HERE](START-HERE.md), [repository layout](docs/engineering/08-folder-structure.md), [agents and Superpowers](docs/engineering/01-agents-superpowers.md), [Gitflow](docs/engineering/03-gitflow-github.md), [CI/CD](docs/engineering/04-ci-cd.md), [tests](docs/engineering/05-tests.md), [installers](docs/engineering/06-installers.md).

```bash
cargo xtask check-docs
cargo xtask check-delivery
cargo test --workspace --locked
```

CI runs on GitHub Actions: on every pull request to `develop`, on push to
`develop`/`main`, and never on a bare feature branch. The required status is
`CI Gate`. Branch protection is not applied while the repository is private —
until the repositories are opened (TASK-E19-06) the gate is a reviewed
discipline, not a GitHub-enforced block.

Auxiliary deliverables: [design tokens](assets/design-tokens.json), [schema examples](schemas/README.md), [tasks as JSON](tasks/backlog.json), [documentation validator](xtask/src/validate.rs), [last validation run](VALIDATION.json), [changelog](CHANGELOG.md).

Repository: `jimmykedlyn/vynamp-mediaplayer`. Today **private**; opening it is
decided in [ADR-014](docs/architecture/adr/ADR-014-public-github-actions-and-in-app-dsp.md)
and executed by TASK-E19-06.

Identity: [Vynamp C / Pulso brand guide](docs/design/04-brand-identity.md) and the
[approved board](assets/branding/approved-c-board.png). Native exports are still
pending, per the manifest. The PNGs are raster references, not vector sources or
compiled `ICO`/`ICNS`/Android resources.

Related repositories: `vynamp-decode` (decoding), `vynamp-output` (audio output),
`vynamp-mvp-reference` (the visual reference for the native UI), and — planned as
separate applications — `vynamp-restore` and `vynamp-converter`, with
`vynamp-encode` as the encoder library behind the converter.

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
Segurança: [GitHub Security Advisories](https://github.com/jimmykedlyn/vynamp-mediaplayer/security/advisories/new) — ver [SECURITY.md](SECURITY.md).
Contribuições: [CONTRIBUTING.md](CONTRIBUTING.md) · Governança: [GOVERNANCE.md](GOVERNANCE.md).

## Ordem de leitura

1. [Contrato mestre](SPEC.md), [índice completo](docs/INDEX.md), [escopo](docs/product/01-vision-scope.md) e [matriz de requisitos](docs/product/03-requirements.md).
2. [Plataformas obrigatórias](docs/platforms/01-support-matrix.md), [Android](docs/platforms/02-android.md), [Design](docs/design/01-visual-system.md), [telas](docs/design/02-layouts.md) e [referência visual](assets/layout-reference.html).
3. [Áudio](docs/audio/01-engine.md), [formatos](docs/audio/02-formats.md), [streaming](docs/streaming/01-client.md).
4. [Arquitetura](docs/architecture/01-system.md), [dados](docs/architecture/03-data.md), [contratos](docs/architecture/04-contracts.md).
5. [Roadmap](docs/delivery/01-roadmap.md), [backlog](tasks/README.md) e [próxima execução](tasks/NEXT.md).

## Como usar

- MUST / DEVE: obrigatório na fase indicada; SHOULD / RECOMENDADO: exceção justificada por ADR; MAY / OPCIONAL: capacidade adicional.
- P0: caminho crítico da fase; P1: necessário para completar a fase; P2: expansão planejada. Prioridade não é ordem cronológica.
- M0: provas técnicas; M1: alpha local; M2: beta multiplataforma; M3: versão 1.0 nos cinco alvos; M4: plataforma; M5: expansão. A visão completa inclui todas as fases, mas a versão 1.0 não promete entregas M4/M5.
- Requisitos `REQ-*`, tarefas `TASK-*`, decisões `ADR-*` e cenários `QA-*` são identificadores estáveis.
- O [registro de decisões](docs/product/05-decisions.md) explicita escolhas feitas para permitir execução sem bloquear em perguntas.

## Tooling e verificação

A automação de engenharia é Rust: [START-HERE](START-HERE.md), [estrutura](docs/engineering/08-folder-structure.md), [agentes/Superpowers](docs/engineering/01-agents-superpowers.md), [Gitflow](docs/engineering/03-gitflow-github.md), [CI/CD](docs/engineering/04-ci-cd.md), [testes](docs/engineering/05-tests.md), [instaladores](docs/engineering/06-installers.md).

```bash
cargo xtask check-docs
cargo xtask check-delivery
cargo test --workspace --locked
```

O CI roda em GitHub Actions: em todo pull request para `develop`, em push para
`develop`/`main`, e nunca em branch de feature isolada. O status exigido é
`CI Gate`. A branch protection não está aplicada enquanto o repositório for
privado — até os repositórios serem abertos (TASK-E19-06), o gate é disciplina
revisada, não bloqueio imposto pelo GitHub.

Entregáveis auxiliares: [tokens visuais](assets/design-tokens.json), [exemplos de contratos](schemas/README.md), [tarefas em JSON](tasks/backlog.json), [validador documental](xtask/src/validate.rs), [última validação](VALIDATION.json), [histórico](CHANGELOG.md).

Repositório: `jimmykedlyn/vynamp-mediaplayer`. Hoje **privado**; a abertura está
decidida na [ADR-014](docs/architecture/adr/ADR-014-public-github-actions-and-in-app-dsp.md)
e é executada pela TASK-E19-06.

Identidade: [guia Vynamp C / Pulso](docs/design/04-brand-identity.md) e [prancha
aprovada](assets/branding/approved-c-board.png). Exports nativos ainda pendentes,
conforme manifest. Os PNGs são referências raster, não fontes vetoriais nem
recursos `ICO`/`ICNS`/Android compilados.

Repositórios relacionados: `vynamp-decode` (decodificação), `vynamp-output`
(saída de áudio), `vynamp-mvp-reference` (referência visual da UI nativa) e — como
aplicações separadas planejadas — `vynamp-restore` e `vynamp-converter`, com
`vynamp-encode` como a biblioteca de encoders por trás do conversor.
