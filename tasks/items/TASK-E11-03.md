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
| Estado | planned |
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
- [ ] `backlog.json` com o estado correto — vai para `implemented` no mesmo PR, junto com
  a evidência
- [ ] Revisão feita por contexto limpo, não pelo implementador
- [ ] PR aberto com template preenchido; CI Gate verde no SHA final

## Notas de execução

Executada em 2026-11-12. Os números abaixo saíram de `cargo deny 0.20.2`,
`cargo metadata --locked --all-features` e `cargo tree`; a saída bruta está em
`evidence/PR/task-e11-03/` com o SHA-256 de cada arquivo.

| AC | O que foi executado | Resultado |
|---|---|---|
| AC-1, crates de terceiro | `cargo deny --format json check licenses 2>&1` na raiz e em `libs/decode`; contagens derivadas da `metadata` | **Satisfeito.** 165 pacotes no lock da raiz: 164 de registry, todos do `xtask` (não distribuído), e 1 de path. 18 expressões distintas, todas com rota permissiva; o único copyleft possível é opcional em `r-efi` (`MIT OR Apache-2.0 OR LGPL-2.1-or-later`, UEFI-only, fora do alvo Windows). Os módulos têm **zero** terceiro: `libs/decode` 9 pacotes, todos de path; `output` 4; `dsp`, `encode` e `engine` 1 cada |
| AC-1, features de build | `find libs apps -name build.rs` e busca por `ffmpeg`/`libav` nos `Cargo.toml` | **Satisfeito.** Não há `build.rs` em nenhum módulo nem app, então não há feature nativa a fixar. FFmpeg não é dependência: é oráculo de dev por `$env:FFMPEG`. O binário local é `n8.1.3-35-gf1acb0a266-20261111` com `--enable-gpl --enable-version3`, e é exatamente por isso que ele nunca é embarcado |
| AC-1, licença de codec | leitura da fonte oficial de cada formato implementado, em 2026-11-12 | **Satisfeito, com uma distinção que o `Then` não separa.** O código é `MIT OR Apache-2.0` em todos os crates de decoder; o formato tem status próprio. FLAC: Xiph afirma "no known patent". MP3: a Fraunhofer afirma que o programa de licenciamento "has been terminated" em 2017-04-23 — **a página não diz que as patentes expiraram**, e o documento registra a diferença em vez de convertê-la |
| AC-1, ASIO | leitura de `ASIOSDK/LICENSE.txt` do SDK 2.3.4, baixado de `steinberg.net/asiosdk` | **Satisfeito.** Dual-licenciado: Steinberg ASIO License **ou** GPLv3. A rota proprietária proíbe distribuir o SDK "in parts or its entirety" sem acordo escrito. O SDK **não está no repositório** e o ADR-011 já o coloca fora do núcleo |
| AC-1, fontes | `git ls-files` por `.ttf`/`.otf`/`.woff2`, mais a leitura do `asset-manifest.json` | **Satisfeito, e o resultado é ausência.** Nenhuma fonte versionada; a única ocorrência é `codicon-*.ttf` dentro de `reference/vynamp-mvp/node_modules`, que é read-only e não entra no build. Noto é OFL-1.1 se for a escolha, mas nenhum arquivo foi admitido |
| AC-2, itens que exigem autorização | três casos concretos, com o texto da fonte citado | **Satisfeito como inventário.** ASIO proprietário exige acordo assinado e proíbe redistribuir o SDK; FFmpeg GPL/nonfree não entra em pacote permissivo; MP3 não tem programa de licenciamento ativo desde 2017 mas também não tem declaração de expiração de patente. Nenhum dos três é usado de forma que hoje distribua algo |
| AC-2, decisões de marca e licença final | confronto com `adr/ADR-008-marca.md`, `design/04-marca.md` e o `asset-manifest.json` | **Satisfeito por registro, não por decisão.** Marca decidida (conceito C/Pulso), `state` = `approved-reference-native-exports-pending`, `branding_exports_ready=false`. As seis pendências que só o mantenedor fecha estão listadas em §9 do inventário, sem alegar autorização que não existe |

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

**Limitação registrada, não corrigida.** A contagem de 164 crates de terceiro é do
`Cargo.lock` do umbrella, que só contém o `xtask`. Os oito submódulos de módulo foram lidos
pelo lock de cada um (todos checados out nesta máquina em 2026-11-12, zero terceiro), mas
qualquer um deles que passe a ter dependência externa reabre a §3 do inventário sem que
este documento mude. Nada aqui mede o `vynamp-mvp-reference`, que é read-only e está fora
do build.

