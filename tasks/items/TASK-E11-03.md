# TASK-E11-03 — Resolver licenças e inventário de dependências

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E11-03` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E11-03` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M0 |
| Prioridade | P0 |
| Estado | implemented |
| Depende de | `TASK-E11-01` |
| Bloqueia | `TASK-E08-02`, `TASK-E09-05`, `TASK-E11-04`, `TASK-E18-02` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Inventário de codecs, fontes, assets e políticas de build/distribuição.

## Boundary

- **Não** decide licença de terceiro por conta própria. A AC-2 exige que a decisão de
  marca e de distribuição seja **registrada**, e o `Then` proíbe "alegar autorização
  inexistente" — o que não se pode provar fica como pendência declarada. Sem dono no backlog: é decisão do mantenedor, em `adr/ADR-008-marca.md`.
- **Não** troca dependência travada no lockfile. A AC-1 verifica feature e licença na
  fonte oficial; substituir o que está fixado é outra decisão. Decisão do mantenedor em `adr/ADR-011-pilha-propria.md`.
- **Não** implementa build de Android nem fixa NDK e Gradle: `TASK-E11-04` é a dona
  desses pins.
- **Não** confunde licença de codec com suporte a codec. O inventário é de codecs,
  fontes, assets e política de distribuição; suporte é do catálogo de formatos. O catálogo é `TASK-E17-01`.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e11-03-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E11-03-AC-1` | o inventário de codecs, fontes e assets do projeto | cada item tem a feature de build e a licença verificadas na fonte oficial | Verificar features FFmpeg/ASIO/encoders e licenças de fontes |
| `TASK-E11-03-AC-2` | os itens cuja licença não permite distribuição sem autorização | a decisão de marca e de distribuição é registrada | decisões de marca/licença final explícitas, sem alegar autorização inexistente. |

## Verificação
| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | leitura da licença e da feature de build de cada item na fonte oficial, confrontada com o `Cargo.lock` e com os pins de `TASK-E11-04` | cada item tem licença e feature registradas na própria fonte — a fonte oficial é a evidência, não o resumo do inventário |
| Hardware | nenhum: inventário de licença e de build não depende de dispositivo de áudio | ausência é limitação registrada, não aprovação |

## DoD — pronto para review

- [x] Todos os critérios de aceite com evidência — a tabela de §Notas de execução, uma
  linha por AC
- [x] `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` limpos — em
  2026-11-12, no workspace do umbrella
- [x] `cargo test` verde; testes ignorados listados com motivo — `cargo xtask ci docs`
  mede 108 verdes na suíte de documentos e 33 na de worktree/claim, 0 falhas, 0 ignorados
- [x] Spec do módulo atualizada se a implementação revelou algo novo — o documento novo é
  `delivery/05-inventario-licencas.md`; `security/02-privacidade-licencas.md` e
  `LEGAL.md` passaram a apontar para ele, e `LEGAL.md` deixou de afirmar que
  `vynamp-encode` "não existe como repositório", o que era falso desde a pinagem
- [x] `backlog.json` com o estado correto — `implemented`, no mesmo PR, junto com a
  evidência
- [x] Revisão feita por contexto limpo, não pelo implementador — dois revisores com
  escopos disjuntos em 2026-11-12, vereditos e achados abaixo
- [ ] PR aberto com template preenchido; CI Gate verde no SHA final — aberto, e o Gate
  corre sobre o SHA final; esta caixa só fecha com ele verde

## Notas de execução

Executada em 2026-11-12. Os números abaixo saíram de `cargo deny 0.20.2`,
`cargo metadata --locked --all-features` e `cargo tree`; a saída bruta está em
`evidence/PR/task-e11-03/` com o SHA-256 de cada arquivo.

| AC | O que foi executado | Resultado |
|---|---|---|
| AC-1, crates de terceiro | `cargo deny --format json check licenses 2>&1` na raiz e em `libs/decode`; contagens derivadas da `metadata` | **Satisfeito.** 165 pacotes no lock da raiz: 164 de registry, todos do `xtask` (não distribuído), e 1 de path. 18 expressões distintas, todas com rota permissiva; o único copyleft possível é opcional em `r-efi` (`MIT OR Apache-2.0 OR LGPL-2.1-or-later`, UEFI-only, fora do alvo Windows). Os módulos têm **zero** terceiro: `libs/decode` 9 pacotes, todos de path; `output` 4; `dsp`, `encode` e `engine` 1 cada |
| AC-1, features de build | `find libs apps -name build.rs` e busca por `ffmpeg`/`libav` nos `Cargo.toml` | **Satisfeito.** Não há `build.rs` em nenhum módulo nem app, então não há feature nativa a fixar. FFmpeg não é dependência: é oráculo de dev por `$env:FFMPEG`. O binário local é `n8.1.3-35-gf1acb0a266-20261111` com `--enable-gpl --enable-version3`, e é exatamente por isso que ele nunca é embarcado |
| AC-1, licença de codec | leitura da fonte oficial de cada formato implementado, em 2026-11-12 | **Satisfeito, com uma distinção que o `Then` não separa.** O código é `MIT OR Apache-2.0` em todos os crates de decoder; o formato tem status próprio. FLAC: Xiph afirma "no known patent". MP3: a Fraunhofer afirma que o programa de licenciamento "has been terminated" em 2017-04-23 — **a página não diz que as patentes expiraram**, e o documento registra a diferença em vez de convertê-la |
| AC-1, ASIO | leitura arquivo a arquivo do SDK 2.3.4, baixado em 2026-11-12 de `steinberg.net/asiosdk`, que responde `302` para `download.steinberg.net/sdk_downloads/ASIO-SDK_2.3.4_2025-10-15.zip` | **Satisfeito, e a primeira leitura estava errada.** Ler só `ASIOSDK/LICENSE.txt` dava "o SDK é dual-licenciado"; esse texto cobre 2 dos 48 arquivos, e o próprio arquivo diz que vale "only to files referencing this license". Medido: 2 dual (Steinberg ASIO License ou GPLv3), 14 BSD-3 da Steinberg embutido, 8 por remissão ao `LICENSE` da raiz, 4 com cabeçalho **Microsoft** `(c) 1992–1996` e **sem cláusula de redistribuição**, e 20 sem cabeçalho nenhum. O SDK **não está no repositório**, o ADR-011 já o coloca fora do núcleo, e a medição está versionada em `evidence/PR/task-e11-03/asio-sdk-licenca-20261112.txt` |
| AC-1, fontes | `git ls-files` por `.ttf`/`.otf`/`.woff2` nos nove submódulos e no umbrella, mais a leitura do `asset-manifest.json` | **Satisfeito, e o resultado é ausência.** Nenhuma fonte versionada em nenhum repo; a única ocorrência é o `codicon` do Playwright, em três arquivos dentro de `reference/vynamp-mvp/node_modules/` (dois `codicon-DCmgc-ay.ttf`, um `codicon.DCmgc-ay.ttf`), que é `node_modules` no `.gitignore` da referência e não entra no build. Noto é OFL-1.1 se for a escolha, mas nenhum arquivo foi admitido |
| AC-2, itens que exigem autorização | três casos concretos, com o texto da fonte citado | **Satisfeito como inventário.** ASIO proprietário exige acordo assinado e proíbe redistribuir o SDK; FFmpeg GPL/nonfree não entra em pacote permissivo; MP3 não tem programa de licenciamento ativo desde 2017 mas também não tem declaração de expiração de patente. Nenhum dos três é usado de forma que hoje distribua algo |
| AC-2, decisões de marca e licença final | confronto com `adr/ADR-008-marca.md`, `design/04-marca.md`, `product/05-decisoes.md` e o `asset-manifest.json` | **Satisfeito por registro, e o registro não é deste documento.** O `Boundary` desta TASK diz que a AC-2 exige a decisão **registrada** e que o que não se pode provar fica como pendência declarada. As decisões existem e têm dono e gate: D-LIC-01 (licença do código, `MIT OR Apache-2.0`, 2026-11-05); ADR-008 (Vynamp, conceito C/Pulso, `branding_exports_ready=false` até os gates); registro de marca e domínio pendente, dono produto, antes de publicar M3; encoder AAC/MP3, ASIO e FFmpeg pendentes com engenharia/revisão de distribuição, antes de empacotar. §9 cita as quatro e entrega a medição que as sustenta. Nenhuma autorização é alegada além do que a fonte dá |

**Onde o `Then` não tem objeto, e por quê.**

- **"os pins de `TASK-E11-04`"** — a TASK que a verificação manda confrontar ainda está
  `planned` e não tem pin nenhum de NDK, Gradle ou artefato. O que dava para confrontar era
  o `Cargo.lock`, e foi o que se fez. Quando a E11-04 fixar os pins, o inventário é o lugar
  onde eles entram.
- **SBOM.** O `deliverable` fala de "políticas de build/distribuição", não de SBOM de
  artefato; SBOM é `TASK-E11-04` e notices de release são `TASK-E11-08`. Nenhum dos dois
  existe, e o documento diz isso em §8 em vez de simular o arquivo.

**Duas correções de documento que a execução exigiu.**

- `LEGAL.md` afirmava que `vynamp-encode` "does not exist as a repository yet". Ele existe,
  está pinado em `libs/encode` e o repo remoto é público; o que não existe é código de
  codec e spec aprovada. Corrigido nos dois idiomas.
- `security/02-privacidade-licencas.md` repetia a regra de inventário sem apontar para
  nenhuma medição. Agora aponta para o inventário e diz explicitamente que ele mede, não
  autoriza.

**A revisão de contexto limpo (dois revisores, escopos disjuntos, em 2026-11-12).**

Revisor de aritmética e hashes: **aprovado com ressalvas**. Reproduziu os 4 SHA-256 da
tabela de §1 contra os blobs de `HEAD`, a aritmética das 18 expressões (147 + 5×2 + 7 =
164), os cinco nomes com duas versões, os cinco pacotes em forma de barra, os 4 sem arquivo
de licença, os 158 diagnósticos, as 19 fora do alvo Windows e as contagens de módulos e
assets. Apontou 5 defeitos, todos verificados e corrigidos: "cinco regimes" onde havia três
regimes e cinco classes; a tabela de raízes de fixture subcontava 36 arquivos e tratava
quatro subdiretórios como três; o corpus de conformidade era citado como cumprindo
`05-testes.md` sem ter hash; e a cadeia de `futures-util` dizia "obrigatória" onde o
`Cargo.toml` do `js-sys` diz `optional = true`.

Revisor de critérios e alegações: **bloqueado**, com 8 achados. Sete foram verificados e
corrigidos: `encumbered` e `reverse-engineered` não existem em `Cargo.toml` nenhum e o
documento os descrevia como build; o corpus `flac-conformance` é ativo de terceiro
versionado sem entrada em notices, e o `LEGAL.md` afirmava que nenhum havia sido admitido;
"encoders" da AC-1 não tinha medição nenhuma, e agora tem (zero encoders, zero features,
`libs/encode` é stub de 6 linhas); a coluna "medido na fonte" tinha duas linhas sem fonte;
o §5 justificava a ausência de FFmpeg com "fixtures versionadas com hash" que só vale para
parte do corpus; a medição do FFmpeg não tinha saída versionada nem era a versão do pin do
ADR-002; e "cinco regimes" de licença do ASIO são três regimes e cinco classes.

O oitavo motivou o bloqueio — que a AC-2 pede decisão registrada e o §9 registra
pendências — e **não procede**: o `Boundary` desta TASK diz que a AC-2 exige a decisão
**registrada**, e as decisões estão registradas fora deste documento, em D-LIC-01,
`adr/ADR-008-marca.md` e `product/05-decisoes.md:24`, com dono e gate. O revisor não leu o
`Boundary`. §9 agora cita as quatro decisões, em vez de só listar pendências — e a correção
que eu tinha feito antes (declarar a AC-2 não satisfeita) era pior que o defeito original,
porque trocava uma leitura errada do critério por outra. Registrada aqui porque o achado foi
aceito e revertido: o que ficou de pé é a citação das decisões, que faltava mesmo.

**Duas limitações registradas, não corrigidas.**

- A contagem de 164 crates de terceiro é do `Cargo.lock` do umbrella, que só contém o
  `xtask`. Os oito submódulos de módulo foram lidos pelo lock de cada um (todos checados
  out nesta máquina em 2026-11-12, zero terceiro), mas qualquer um deles que passe a ter
  dependência externa reabre a §3 do inventário sem que este documento mude. Nada aqui
  mede o `vynamp-mvp-reference`, que é read-only e está fora do build.
- Três das cinco raízes de fixture do `vynamp-decode` não têm hash versionado — 63
  arquivos. `engineering/05-testes.md` exige hash, licença, formato e oráculo no manifesto,
  e a lacuna está medida em §7 do inventário. Não é desta TASK (o corpus é do
  `vynamp-decode`) nem muda o inventário de licença, porque as fixtures são sintéticas; mas
  fica registrada, porque um inventário que olhasse só `pcm/`, `flac/` e `mpeg/` diria que
  o corpus está coberto.

