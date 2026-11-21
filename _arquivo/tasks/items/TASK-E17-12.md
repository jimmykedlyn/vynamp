# TASK-E17-12 — Publicar manifest real de capacidades por artefato

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E17-12` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E17-12` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M3 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E17-01`, `TASK-E16-09`, `TASK-E15-04` |
| Bloqueia | `TASK-E14-06`, `TASK-E17-20` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Manifest gerado com backend/features/licenças/hashes/evidência e validação do pacote.

## Boundary

- **Não** gera as declarações de integração com o SO — handler, MIME, ProgID, `.desktop` ou UTI. A AC-3 fala de associação, e a declaração por container/parser é `TASK-E16-09`, que a deriva deste manifest; emitir a declaração aqui daria dois donos ao mesmo hash.
- **Não** assina o manifest nem publica o release. O minisign do `manifest.json`, a chave pública pinada e o draft privado são `TASK-E15-10`; aqui se entrega o conteúdo a assinar.
- **Não** decide viabilidade de codec nem o modelo de capability por alvo. O catálogo, as fases e a política são `TASK-E17-01`; um manifest que reescreve a política na hora de publicar esvazia a autoridade do catálogo.
- **Não** mede o teto true-peak nem a qualidade numérica do sinal. `TASK-E17-08` mede o PCM no buffer do backend e `TASK-E04-10` fecha a matriz numérica; o manifest referencia essa evidência, não a produz.
- **Não** instala nem valida os pacotes dos cinco alvos: NSIS/portable é `TASK-E15-06`, AppImage/DEB é `TASK-E15-07`, DMG/ZIP é `TASK-E15-08` e APK/AAB é `TASK-E15-09`, e a instalação/atualização em máquina limpa é `TASK-E14-05`. Aqui o artefato é lido por hash, não executado.
- **Não** fecha o gate combinado de codecs, DSP e visualizers: `TASK-E17-20` consome este manifest e agrega a evidência por alvo, incluindo os condicionais e as QAs de sinal e visualização.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e17-12-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E17-12-AC-1` | o catálogo `config/format-catalog.json` rev 0.7.0, com 68 entradas e `state` igual a `planned` nas 68, mais um build real de cada um dos cinco alvos com o conjunto de features efetivamente compilado e o backend de cada entrada listado na evidência de build daquele artefato | o gerador escreve o manifest de cada artefato e cada linha de capability é confrontada com o catálogo e com o probe do build | Catálogo planned não é disponibilidade |
| `TASK-E17-12-AC-2` | os 32 formatos com `policy: required` do catálogo — 4 em M1, 10 em M2 e 18 em M3 — que já declaram `target_platforms` com os cinco alvos | o manifest de cada um dos cinco artefatos é confrontado com a lista dos required comuns e o casamento é registrado por alvo | required comum comprovado nos cinco alvos |
| `TASK-E17-12-AC-3` | uma entrada `required` cujo backend não entrou no build do alvo, por exemplo `wavpack-lossless` num artefato sem adapter WavPack | o manifest do artefato é gerado e as associações de arquivo e a oferta de encode são lidas do artefato distribuído | adapter ausente não gera associação nem encode. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `cargo test --manifest-path xtask/Cargo.toml --locked` mais `cargo run --manifest-path xtask/Cargo.toml -- check-delivery`, hoje só provam forma: o schema `schemas/codec-capability.schema.json` reprova `available: true` com `state: planned`, e `xtask/src/release.rs:143` (`artifact_manifest`) escreve apenas `product_version` e `artifacts[file,bytes,sha256]` dos oito artefatos, sem uma linha de capability | esperado: as 68 entradas continuam `planned` e nenhuma passa a `available` sem `state: verified`, `licensed: true` e `backend`, `backend_version`, `evidence` e `license_ids` não vazios — o teste `catalog::tests::planned_capability_cannot_advertise_availability` passa dentro dos 119 testes do xtask e `check-delivery` fecha em 0 erros; não existe hoje o gerador de capability por artefato, o confronto dos 32 required nos cinco alvos nem o casamento de hash entre manifest e pacote: em 2026-11-10 não há `dist/release/manifest.json`, `evidence/release/` não existe e `application_ready` e `release_enabled` valem `false` em `config/project.json` |
| Hardware | ausente: a AC-2 exige artefato real dos cinco alvos, e esta máquina é Windows 11 x64 — sem máquina macOS Intel, sem macOS Silicon e sem aparelho Android arm64; `which cc gcc clang ld` não acha linker cruzado e não há cargo-ndk nem SDK/NDK Android em ANDROID_HOME, então só `windows-x64` produz artefato de build real e os outros quatro ficam declarados incompletos, nunca aprovados; no Windows o toolchain `stable-x86_64-pc-windows-msvc` está ativo e compila | ausência é limitação registrada, não aprovação |

## DoD — pronto para review

- [ ] Todos os critérios de aceite com evidência
- [ ] `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` limpos
- [ ] `cargo test` verde; testes ignorados listados com motivo
- [ ] Spec do módulo atualizada se a implementação revelou algo novo
- [ ] `backlog.json` com o estado correto
- [ ] Revisão feita por contexto limpo, não pelo implementador
- [ ] PR aberto com template preenchido; CI Gate verde no SHA final
## Notas de execução

Preenchido durante a execução, não antes.
