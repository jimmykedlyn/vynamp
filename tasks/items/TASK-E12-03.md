# TASK-E12-03 — Validar ARM64 adicional Windows/Linux e helpers

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E12-03` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E12-03` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M5 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E12-01`, `TASK-E12-02` |
| Bloqueia | `TASK-E12-10` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 10–25 dias úteis |
| Risco | alto |

## Entrega

Matriz opcional ARM64 Windows/Linux; Android arm64 e macOSarm64 já são obrigatórios emM3.

## Boundary

- **Não** entrega o Linux x86_64 nativo nem o MPRIS. O alvo obrigatório de Linux é `TASK-E12-01`, e aceitar linux-arm64 antes do x86_64 fechado inverteria a ordem dos gates de M3 que o roadmap fixa.
- **Não** refaz o macOS Universal2 nem o `lipo` das slices. As duas slices e os helpers Universal2 são `TASK-E12-02`; aqui o padrão Mac é copiado como referência de build e assinatura, não reimplementado.
- **Não** fecha o Android arm64. O aceite Android em hardware real é `TASK-E13-10`, com o spike de build e AAudio em `TASK-E13-01`; Android arm64 é premissa desta TASK, não entrega dela.
- **Não** escreve o relatório numérico de qualidade nem promove nível em `docs/support-matrix.md`. O relatório e a matriz de combinações são `TASK-E04-10`, e a declaração de combinação validada é `TASK-E04-06`; esta TASK só acrescenta a linha da arquitetura nova em `libs/output/docs/hardware-matrix.md`, com driver e hash de captura próprios, porque `support-matrix.md` é saída literal de um gerador e editá-la à mão reprova `crates/vynamp-output/tests/support_matrix.rs`.
- **Não** produz os instaladores e o update por alvo. AppImage, DEB, NSIS e o canal de atualização são `TASK-E11-07` e a validação de packaging por alvo é `TASK-E14-05`.
- **Não** implementa o helper nativo isolado. Sandbox, RPC e memória compartilhada do helper são `TASK-E07-05`; aqui o helper é apenas um artefato a ser construído, inspecionado e assinado na arquitetura nova.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e12-03-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E12-03-AC-1` | as duas arquiteturas extras aarch64-pc-windows-msvc e aarch64-unknown-linux-gnu, ausentes de config/project.json (6 alvos declarados, nenhum ARM64 de desktop), de PLATFORMS em xtask/src/release.rs (5 entradas), de CORE_TARGETS em xtask/src/catalog.rs (5 alvos) e de rustup target list --installed (5 alvos, nenhum deles ARM64 de desktop) | cada arquitetura é construída numa máquina nativa dela, o artefato é instalado e tocado, e a prova vai para uma linha própria em libs/output/docs/hardware-matrix.md e para o inventário nativo do manifest | Arquiteturas adicionais têm build/hardware/output próprios |
| `TASK-E12-03-AC-2` | um Windows 11 on ARM com Prism ativo e um Linux ARM64 com qemu-x86_64 registrado em binfmt_misc, de modo que rodar o binário x64 na mesma máquina funcionaria, e o instalador ARM64 ao lado do instalador x64 na mesma máquina | o executável é inspecionado com dumpbin -headers (Windows) e readelf -h ou file (Linux), a camada de emulação é desativada e o binário é tocado nas duas máquinas, e a arquitetura do processo em execução é comparada com a do binário que o instalador entregou | nenhum fallback oculto |
| `TASK-E12-03-AC-3` | os dois alvos extras já aceitos e M3 ainda aberto nos alvos obrigatórios de arm64: TASK-E12-02 em macOS Apple Silicon e TASK-E13-10 em Android arm64, com o gate agregado em TASK-E14-06 | o plano de marcos é reavaliado com a expansão dentro, e os relatórios de M3 de macOS Silicon e Android arm64 são datados contra o mesmo SHA de release | a expansão não posterga Android/Mac arm64 obrigatórios. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `cargo build --lib --manifest-path libs/<m>/Cargo.toml --target aarch64-pc-windows-msvc` e o mesmo com `--target aarch64-unknown-linux-gnu` nos cinco libs de `libs/`, depois `dumpbin -headers` sobre o `.rlib` e sobre o executável (Windows), `readelf -h` sobre o objeto extraído do arquivo e `file` sobre o executável (Linux), e a leitura das duas linhas novas em `libs/output/docs/hardware-matrix.md` contra as chaves de `packaging/runtime-files.json` | Medido nesta máquina antes da TASK: `cargo build --lib --target aarch64-pc-windows-msvc` sai 0 nos cinco libs de `libs/` e `dumpbin -headers` do `.rlib` responde `AA64 machine (ARM64)`; com `--target aarch64-unknown-linux-gnu` também sai 0 e o objeto dentro do `.rlib` é ELF class 2 com machine 183, embora `file` do `.rlib` só diga `current ar archive`. O que falta é o executável, não a biblioteca: `cargo build --bins --manifest-path libs/output/Cargo.toml` morre em `linker cc not found` no alvo Linux e em `link.exe` no alvo Windows, porque não há linker ARM64 instalado. `cargo xtask ci core --target` nas duas arquiteturas termina em `Actual desktop app absent` (exit 1) antes de compilar qualquer crate, e continua sem provar os libs depois da TASK: o workspace raiz tem `members = ["xtask"]` e exclui `libs/*`. `check_host` em xtask/src/util.rs só é chamado por `package build` e aceita windows-x64, linux-x64, macos-universal e android-arm64, e `xtask package --platform` tem value_parser com esses quatro valores, então nenhuma string ARM64 é aceita. `config/project.json` declara 6 alvos sem ARM64 de desktop, `PLATFORMS` e `CORE_TARGETS` têm 5 entradas cada, `rustup target list --installed` tem 5 alvos, `packaging/runtime-files.json` tem só windows-x64, linux-x64 e macos-universal, com `audit_complete: false` e as três listas vazias, e as 7 linhas de `libs/output/docs/hardware-matrix.md` estão `not run` com device `(pending)`. Depois da TASK a evidência exigida é exit 0 dos cinco libs nas duas arquiteturas, `AA64 machine (ARM64)` e `ELF 64-bit LSB` com `aarch64` sobre o executável instalado e não só sobre o `.rlib`, um linker ARM64 no caminho, e duas linhas novas na matriz com aparelho, driver, OS, comando, commit e hash de captura preenchidos no lugar de `(pending)`. |
| Hardware | duas máquinas nativas, um Windows 11 on ARM e um Linux ARM64 (Ubuntu 24.04 arm64) com DAC USB alimentando uma captura digital; nenhuma das duas existe neste ambiente, que é Windows 11 x86_64 (PROCESSOR_ARCHITECTURE=AMD64, Win32_Processor Architecture=9) e cujos dois endpoints de áudio são Intel display audio e Conexant ISST Audio, nenhum com captura digital, além de não ter hardware ARM, aparelho Android nem macOS | ausência é limitação registrada, não aprovação |

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
