# TASK-E14-01 — Provar os cinco targets e features portáveis

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E14-01` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E14-01` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M0 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E11-01` |
| Bloqueia | `TASK-E11-04`, `TASK-E14-02`, `TASK-E17-01` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Matriz de toolchains/ABIs Windows/Linux/macOS Intel/macOS Silicon/Android e dependencies.

## Boundary

- **Não** congela as baselines por conta própria. A AC-3 avalia o freeze **target a
  target**, e as licenças vêm do inventário de `TASK-E11-03`; congelar aqui seria
  congelar sem inventário.
- **Não** implementa o smoke de adapter. A AC-2 mede output, arquivo e credencial por
  host, e adapter por host é `TASK-E14-02`.
- **Não** audita a qualidade numérica do sinal: isso é `TASK-E04-10` e a matriz de
  hardware.
- **Não** inclui iOS. A matriz desta TASK é a dos cinco alvos existentes; iOS nasce na
  FASE 6, com a [ADR-016](../../adr/ADR-016-ios.md) e a
  [spec de iOS](../../platforms/05-ios.md), e não entra num freeze retroativo.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e14-01-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E14-01-AC-1` | a matriz de toolchains e ABIs de Windows, Linux, macOS Intel, macOS Silicon e Android | cada ABI é compilada com os codecs nativos e o artefato resultante é inspecionado | Core e codecs nativos resolvem em cada ABI |
| `TASK-E14-01-AC-2` | os adapters de output, de arquivos e de credenciais de cada host | o smoke de cada adapter roda no target correspondente | outputs/files/keychain provados |
| `TASK-E14-01-AC-3` | as baselines de cada target e o inventário de licenças de E11-03 | o freeze é avaliado target a target | baselines e licenças suportam todos os alvos antes freeze. |

## Verificação
| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `cargo check --workspace --locked --target <t>` dentro de cada repo que tem código, para os cinco alvos — não na raiz, que compila só o `xtask` | **15/15 exit 0** nas 15 combinações medidas (xtask, `libs/decode` e `libs/output` × 5 alvos); os outros seis repos têm `[dependencies]` vazio e 3–6 linhas de `lib.rs`, então não há o que resolver por ABI |
| Hardware | `cargo build` cruzado nesta máquina: `libs/decode` exit 0 nos três alvos testados; `libs/output` exit 101 por falta de linker cruzado (`which cc gcc clang` não acha nenhum) | o 101 é ambiente, não código — o build de host do `libs/output` fecha exit 0 em 10,75 s; ausência de linker é limitação registrada, não aprovação |

## DoD — pronto para review

- [ ] Todos os critérios de aceite com evidência
- [ ] `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` limpos
- [ ] `cargo test` verde; testes ignorados listados com motivo
- [ ] Spec do módulo atualizada se a implementação revelou algo novo
- [ ] `backlog.json` com o estado correto
- [ ] Revisão feita por contexto limpo, não pelo implementador
- [ ] PR aberto com template preenchido; CI Gate verde no SHA final
## Notas de execução

Medido em 2026-11-09. Provas em `evidence/PR/task-e14-01/`.

**Correção de escopo, antes dos ACs.** A raiz do umbrella tem `exclude` para os
oito módulos, então `cargo check --workspace` lá dentro compila **só o `xtask`**.
Uma matriz medida na raiz não prova nada sobre core nem codecs. A matriz abaixo foi
medida **dentro de cada repo que tem código**, e o comentário de escopo está no
próprio arquivo de evidência.

| AC | Verificação executada | Resultado |
|---|---|---|
| AC-1 | `cargo check --workspace --locked --target <t>` para os cinco alvos, dentro de `vynamp` (xtask), `libs/decode` (9 crates, 29 100 linhas) e `libs/output` (40 arquivos, 7 454 linhas) — 15 combinações | **15/15 exit 0.** `libs/dsp`, `libs/engine`, `libs/encode`, `apps/player`, `apps/converter`, `apps/restore` têm `[dependencies]` vazio e 3–6 linhas de `lib.rs`: não há o que resolver por ABI neles |
| AC-1, artefato | `cargo build --workspace --locked --target aarch64-apple-darwin / aarch64-linux-android / x86_64-unknown-linux-gnu` | `libs/decode` **exit 0** nos três (só libs, sem linker). `libs/output` **exit 101** nos três: `output-probe` é binário e não há linker cruzado nesta máquina — `which cc gcc clang` não acha nenhum, só o `link.exe` da MSVC serve o host. Host `cargo build --workspace` no `libs/output` fecha **exit 0** em 10,75 s, então o 101 é ambiente, não código |
| AC-2 | procura por adapters por host: `grep -rn "cfg(target_os\|wasapi\|coreaudio\|pipewire\|aaudio"` em `libs/output`, e `grep -rn "keyring\|credential-store\|dpapi"` em todo o repo | **Nenhum adapter de OS existe.** `libs/output/crates/vynamp-output/src/lib.rs:5-6` diz isso com todas as letras: "release only the pure-Rust `NullHost` exists; OS backends (WASAPI, ALSA, CoreAudio, AAudio, PipeWire, Pulse) arrive in later releases". **Nenhuma implementação de keychain** em lugar nenhum do tree. AC-2 **não satisfeito** |
| AC-3 | inventário de licenças de E11-03 refeito por medição (§E11-03) e baselines de `platforms/01-matriz-suporte.md:9-17` | Inventário de terceiro fechado: 164 pacotes no `xtask`, **0 em `decode` e `output`**, nenhum copyleft sem rota permissiva, nenhuma fonte embutida, nenhum asset de origem externa. Baselines seguem **propostas** ("revisadas em M0 conforme toolchain e testes"), e este M0 não pôde inspecionar artefato cruzado (linha acima). AC-3 **parcialmente satisfeito**: as licenças suportam os alvos, as baselines ainda não foram exercidas em artefato |

| Suite | Comando | Resultado |
|---|---|---|
| tooling | `cargo test --package xtask --locked --test tooling` | `test result: ok. 33 passed; 0 failed` em 22,03 s. Os testes de guarda medidos por nome: os 2 `hotfix_*` (`hotfix_requires_both_main_and_develop`, `hotfix_merged_only_to_main_is_preserved`) e os 9 `removal_*` (claim de outro owner, upstream configurado, worktree detached, repo/claim estrangeiro, base inconsistente, branch inconsistente, task inconsistente, feature integrada com head central não mergeado, worktree trocada) |
| docs | `cargo xtask ci docs` | `test result: ok. 33 passed; 0 failed` em 25,20 s |

**O que esta TASK não prova, e por que isso importa.** O `Then` do AC-1 diz
"cada ABI é compilada com os codecs nativos e o artefato resultante é
inspecionado". Duas coisas desse enunciado são falsas hoje: (a) **não há codec
nativo** — `find libs/decode libs/output -name build.rs` não retorna nada e todos os
crates de codec são Rust puro, então não existe "codecs nativos" a linkar em ABI
nenhuma; (b) **o artefato não pôde ser inspecionado**, porque produzir artefato
cruzado exige linker de cada alvo e esta máquina só tem o da MSVC. O que ficou
provado é a resolução de tipos e a compilação, alvo por alvo, do código que existe.

**O que faria AC-1 e AC-2 fecharem de verdade:** instalar os toolchains de link por
alvo (cross-rs ou NDK + toolchain Apple) e escrever os adapters de output/files/
keychain. Nenhum dos dois pertence a esta TASK: os adapters são TASK-E01-03
(`vynamp-engine`) e as TASKs de plataforma de E12/E13. Esta TASK fecha a matriz e
registra os dois vazios; não os transforma em verde.

**Custo se essa leitura estiver errada:** se o mantenedor considerar que
`cargo check` verde por alvo já satisfaz AC-1 e que AC-2 pode ser fechado sem
adapter, então a TASK está completa com a matriz em 15/15. Se a leitura correta for
"artefato inspecionado ou nada", AC-1 segue aberto por infraestrutura e AC-2 por
código ausente — e não há evidência neste repositório que os feche.
