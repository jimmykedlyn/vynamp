# Legal notes / Notas legais

Each section appears in English and then in Portuguese; where the two texts
appear to differ, the Portuguese text of that section prevails.

Cada seção aparece em inglês e depois em português; onde os dois textos
parecerem divergir, prevalece o texto em português daquela seção.

This file is not legal advice. Whoever distributes a build is responsible for
complying with the licences of everything they ship, and for obtaining any
patent licences that apply in their jurisdiction.

Este arquivo não é aconselhamento jurídico. Quem distribui um build responde
pelo cumprimento das licenças de tudo o que entrega e pela obtenção de
licenças de patente aplicáveis na sua jurisdição.

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

### Licença deste repositório

O código deste repositório é licenciado duplamente sob **MIT OR Apache-2.0**,
à escolha de quem usa:

- [`LICENSE-MIT`](LICENSE-MIT)
- [`LICENSE-APACHE`](LICENSE-APACHE)
- [`LICENSE`](LICENSE) — a declaração de que as duas se aplicam

`SPDX-License-Identifier: MIT OR Apache-2.0`. Contribuições são aceitas sob a
mesma licença dupla salvo declaração em contrário.

A licença cobre o **código-fonte deste repositório**. Não concede direitos sobre
codecs, conteúdo, fontes, marcas ou SDKs de terceiros. Também não relicencia
código que as bibliotecas de áudio mantêm sob licença copyleft: elas reservam um
diretório `crates-copyleft/` para isso, convenção documentada nas notas legais do
próprio [`vynamp-decode`](https://github.com/jimmykedlyn/vynamp-decode) e ainda
não preenchida em nenhuma delas.

## Why the licence exists now, and why it did not before

Until 2026-11-05 this repository had **no** licence file, by decision rather
than by oversight: `security/02-privacidade-licencas.md` said not to create a
`LICENSE` "as if it had been the user's decision", and `AGENTS.md` forbade
creating a final licence without a recorded decision. The maintainer made that
decision on 2026-11-05 — MIT OR Apache-2.0, matching `vynamp-decode` and
`vynamp-output` — and it is recorded as D-LIC-01 in
[`product/05-decisoes.md`](product/05-decisoes.md). Without a
licence, opening the repositories (TASK-E19-06) would publish source that
nobody may legally use.

The licence is **not** a release claim. `cargo xtask release preflight` still
requires `THIRD_PARTY_NOTICES`, real-hardware evidence for every supported
platform and a signed reviewer record — none of which exist yet — and
`publish-draft` additionally verifies a minisign signature. A tagged release
remains fail-closed.

### Por que a licença existe agora, e por que não antes

Até 2026-11-05 este repositório **não** tinha arquivo de licença, por decisão e
não por descuido: `security/02-privacidade-licencas.md` mandava não criar
`LICENSE` "como se tivesse sido decisão do usuário", e `AGENTS.md` proibia criar
licença final sem decisão registrada. O mantenedor tomou essa decisão em
2026-11-05 — MIT OR Apache-2.0, igual a `vynamp-decode` e `vynamp-output` — e ela
está registrada como D-LIC-01 em
[`product/05-decisoes.md`](product/05-decisoes.md). Sem licença,
abrir os repositórios (TASK-E19-06) publicaria código que ninguém pode usar
legalmente.

A licença **não** é alegação de release. `cargo xtask release preflight` ainda
exige `THIRD_PARTY_NOTICES`, evidência em hardware real para cada plataforma
suportada e registro assinado de revisor — nenhum dos três existe ainda — e
`publish-draft` verifica adicionalmente uma assinatura minisign. Um release com
tag continua fail-closed.

## Format policy

This repository holds no codec implementation. The policy for the decode,
encode and output libraries lives with those libraries, where the code does:

- `vynamp-decode` — open specifications or expired patents in `default`;
  encumbered and reverse-engineered formats off by default.
- `vynamp-output` — no third-party code linked; platform components used only
  through their public APIs.

`vynamp-encode` exists as a repository and is pinned here, but has no codec code
yet: only the module stub, and a format matrix in
[`libs/encode/specs/formatos.md`](libs/encode/specs/formatos.md) that is drafted
and **not approved**. Until it is, no number in it is a contract and this file
makes no claim about what the encoder will accept.

The measured inventory of third-party code, codecs and assets — with the command
behind every figure — is [`delivery/05-inventario-licencas.md`](delivery/05-inventario-licencas.md).

### Política de formatos

Este repositório não contém implementação de codec. A política das bibliotecas
de decodificação, codificação e saída vive com elas, onde está o código:

- `vynamp-decode` — especificações abertas ou patentes expiradas em `default`;
  formatos onerosos ou obtidos por engenharia reversa desligados por padrão.
- `vynamp-output` — nenhum código de terceiros linkado; componentes de
  plataforma usados apenas por suas APIs públicas.

`vynamp-encode` existe como repositório e está pinado aqui, mas ainda não tem
código de codec: só o stub do módulo e uma matriz de formatos em
[`libs/encode/specs/formatos.md`](libs/encode/specs/formatos.md) que está
**autorada e não aprovada**. Enquanto não for, nenhum número dela é contrato e
este arquivo não afirma o que o encoder vai aceitar.

O inventário medido de código de terceiro, codecs e assets — com o comando por
trás de cada número — é
[`delivery/05-inventario-licencas.md`](delivery/05-inventario-licencas.md).

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
`THIRD_PARTY_NOTICES`, which does not exist yet, and `release preflight` blocks a
release until the file is created.

One third-party asset is already admitted and is **not** in that inventory: the
FLAC conformance corpus in `libs/decode/tests/fixtures/flac-conformance/` (36
files, IETF WG CELLAR, CC0 1.0, licence and provenance in that directory's
`LICENSE.txt` and `SOURCE.txt`). Its licence is identified; its notices entry is
missing because the file it would go in does not exist. This file records the
gap instead of claiming no asset was admitted. Creating `THIRD_PARTY_NOTICES` is
`TASK-E11-04`/`TASK-E11-08`; see `delivery/05-inventario-licencas.md` §3 and §9.

### Marcas e ativos de terceiros

"Vynamp" é o nome escolhido pelo mantenedor. Nenhum registro de marca ou domínio
foi verificado. Os ativos de marca em `assets/branding/` foram gerados para este
projeto e aprovados pelo mantenedor; foram copiados para o repositório sem
edição, e [`assets/branding/README.md`](assets/branding/README.md) registra essa
procedência junto com os hashes. Não há código, arte ou ativo do Winamp nem de
qualquer outro player; a ausência é regra do projeto, não alegação de licença.

Fixtures de teste, fontes e amostras de terceiros entram apenas com licença
identificada e entrada no inventário de avisos. Esse inventário é
`THIRD_PARTY_NOTICES`, que ainda não existe, e `release preflight` bloqueia o
release até o arquivo existir.

Um ativo de terceiros **já está admitido** e não está nesse inventário: o corpus
de conformidade FLAC em `libs/decode/tests/fixtures/flac-conformance/` (36
arquivos, IETF WG CELLAR, CC0 1.0, licença e procedência no `LICENSE.txt` e no
`SOURCE.txt` do próprio diretório). A licença está identificada; a entrada em
notices falta porque o arquivo onde ela iria não existe. Este arquivo registra a
lacuna em vez de afirmar que nenhum ativo foi admitido. Criar
`THIRD_PARTY_NOTICES` é `TASK-E11-04`/`TASK-E11-08`; ver
`delivery/05-inventario-licencas.md` §3 e §9.
