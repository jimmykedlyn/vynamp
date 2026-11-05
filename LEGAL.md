# Legal notes

This file is not legal advice. Whoever distributes a build is responsible for
complying with the licences of everything they ship, and for obtaining any
patent licences that apply in their jurisdiction.

## Licence of this repository

Vynamp source is dual licensed under **MIT OR Apache-2.0**, at your option:

- [`LICENSE-MIT`](LICENSE-MIT)
- [`LICENSE-APACHE`](LICENSE-APACHE)
- [`LICENSE`](LICENSE) — the statement that both apply

`SPDX-License-Identifier: MIT OR Apache-2.0`. Contributions are accepted under
the same dual licence unless stated otherwise.

The licence covers the **source of this repository**. It does not grant rights
to third-party codecs, content, fonts, trademarks or platform SDKs. It also does
not relicense code that the audio libraries keep under a copyleft licence: those
repositories reserve a `crates-copyleft/` directory for it, a convention
documented in [`vynamp-decode`](https://github.com/jimmykedlyn/vynamp-decode)'s
own legal notes and not yet populated in any of them.

## Notas legais (português)

Este arquivo não é aconselhamento jurídico. Quem distribui um build responde
pelo cumprimento das licenças de tudo o que entrega e pela obtenção de
licenças de patente aplicáveis na sua jurisdição.

O código deste repositório é licenciado duplamente sob **MIT OR Apache-2.0**,
à escolha de quem usa. A licença cobre o **código-fonte deste repositório**;
não concede direitos sobre codecs, conteúdo, fontes, marcas ou SDKs de
terceiros. Também não relicencia código que as bibliotecas de áudio mantêm sob
licença copyleft: elas reservam um diretório `crates-copyleft/` para isso,
convenção documentada nas notas legais do próprio
[`vynamp-decode`](https://github.com/jimmykedlyn/vynamp-decode) e ainda não
preenchida em nenhuma delas.

## Why the licence exists now, and why it did not before

Until 2026-11-05 this repository had **no** licence file, by decision rather
than by oversight: `docs/security/02-privacy-licensing.md` said not to create a
`LICENSE` "as if it had been the user's decision", and `AGENTS.md` forbade
creating a final licence without a recorded decision. The maintainer made that
decision on 2026-11-05 — MIT OR Apache-2.0, matching `vynamp-decode` and
`vynamp-output` — and it is recorded as D-LIC-01 in
[`docs/product/05-decisions.md`](docs/product/05-decisions.md). Without a
licence, opening the repositories (TASK-E19-06) would publish source that
nobody may legally use.

The licence is **not** a release claim. `cargo xtask release preflight` still
requires `THIRD_PARTY_NOTICES`, real-hardware evidence for every supported
platform and a signed reviewer record — none of which exist yet — and
`publish-draft` additionally verifies a minisign signature. A tagged release
remains fail-closed.

## Format policy

This repository holds no codec implementation. The policy for the decode,
encode and output libraries lives with those libraries, where the code does:

- `vynamp-decode` — open specifications or expired patents in `default`;
  encumbered and reverse-engineered formats off by default.
- `vynamp-output` — no third-party code linked; platform components used only
  through their public APIs.

`vynamp-encode`, which will be the single source of encoders for the Converter,
does not exist as a repository yet; its format policy is written when it does.

## Trademarks and third-party assets

"Vynamp" is the project name chosen by the maintainer. No trademark or domain
registration was verified. The brand assets under `assets/branding/` were
generated for this project and approved by the maintainer; they were copied into
the repository unedited, and
[`assets/branding/README.md`](assets/branding/README.md) records that provenance
along with their hashes. No code, artwork or asset from Winamp or any other
player is included; their absence is a project rule, not a licence claim.

Third-party test fixtures, fonts and samples enter only with an identified
licence and an entry in the notices inventory. That inventory is
`THIRD_PARTY_NOTICES`, which does not exist yet: no third-party asset has been
admitted so far, and `release preflight` blocks a release until the file is
created.
