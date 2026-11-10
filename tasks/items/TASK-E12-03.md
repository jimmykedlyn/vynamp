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

- **Não** entrega o Linux x86_64 nativo nem o MPRIS. O alvo obrigatorio de Linux e `TASK-E12-01`, e aceitar linux-arm64 antes do x86_64 fechado inverteria a ordem dos gates de M3 que o roadmap fixa.
- **Não** refaz o macOS Universal2 nem o `lipo` das slices. As duas slices e os helpers Universal2 sao `TASK-E12-02`; aqui o padrao Mac e copiado como referencia de build e assinatura, nao reimplementado.
- **Não** fecha o Android arm64. O aceite Android em hardware real e `TASK-E13-10`, com o spike de build e AAudio em `TASK-E13-01`; Android arm64 e premissa desta TASK, nao entrega dela.
- **Não** escreve o relatorio numerico de qualidade nem a matriz de combinacoes de saida. `TASK-E04-10` e dona desse relatorio; esta TASK so acrescenta a linha da arquitetura nova, com driver e hash de captura proprios.
- **Não** produz os instaladores e o update por alvo. AppImage, DEB, NSIS e o canal de atualizacao sao `TASK-E11-07` e a validacao de packaging por alvo e `TASK-E14-05`.
- **Não** implementa o helper nativo isolado. Sandbox, RPC e memoria compartilhada do helper sao `TASK-E07-05`; aqui o helper e apenas um artefato a ser construido, inspecionado e assinado na arquitetura nova.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e12-03-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E12-03-AC-1` | as duas arquiteturas extras aarch64-pc-windows-msvc e aarch64-unknown-linux-gnu, ausentes de config/project.json (6 alvos declarados, nenhum ARM64 de desktop), de PLATFORMS em xtask/src/release.rs (5 entradas), de CORE_TARGETS em xtask/src/catalog.rs (5 alvos) e de rustup target list --installed (5 alvos, nenhum deles ARM64 de desktop) | cada arquitetura e construida numa maquina nativa dela, o artefato e instalado e tocado, e a prova vai para uma linha propria em libs/output/docs/hardware-matrix.md e para o inventario nativo do manifest | Arquiteturas adicionais têm build/hardware/output próprios |
| `TASK-E12-03-AC-2` | um Windows 11 on ARM com emulacao x64 (Prism) disponivel e um Linux ARM64 com qemu-user instalado, e o instalador ARM64 ao lado do instalador x64 na mesma maquina | o binario e inspecionado com dumpbin /headers (ou file) e a reproducao roda com a camada de emulacao desligada, comparando o host de saida registrado no log com a arquitetura do processo | nenhum fallback oculto |
| `TASK-E12-03-AC-3` | os dois alvos extras ja aceitos e M3 ainda aberto nos alvos obrigatorios de arm64: TASK-E12-02 em macOS Apple Silicon e TASK-E13-10 em Android arm64, com o gate agregado em TASK-E14-06 | o plano de marcos e reavaliado com a expansao dentro, e os relatorios de M3 de macOS Silicon e Android arm64 sao datados contra o mesmo SHA de release | a expansão não posterga Android/Mac arm64 obrigatórios. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `cargo xtask ci core --target aarch64-pc-windows-msvc` e `cargo xtask ci core --target aarch64-unknown-linux-gnu` sobre os crates de `libs/`, depois `dumpbin /headers` (Windows) e `file` (Linux) sobre o artefato, e a leitura da linha nova em `libs/output/docs/hardware-matrix.md` contra o inventario de `packaging/runtime-files.json` | Hoje a primeira coisa que existe e a recusa, medida nesta maquina: `cargo xtask ci core --target aarch64-pc-windows-msvc` termina com `Actual desktop app absent` (exit 1), `config/project.json` declara 6 alvos e nenhum e ARM64 de desktop, `PLATFORMS` em `xtask/src/release.rs` tem 5 entradas, `check_host` em `xtask/src/util.rs` so aceita windows-x64, linux-x64, macos-universal e android-arm64, `rustup target list --installed` traz 5 alvos sem ARM64 de desktop e todas as linhas de `hardware-matrix.md` estao `not run` com device `(pending)`. Depois da TASK, a evidencia exigida e: dois artefatos cujo cabecalho diz `Machine ARM64` e `ELF 64-bit LSB` com `aarch64`, execucao sem tracer de emulacao no log, e duas linhas novas na matriz com aparelho, driver, OS, comando, commit e hash de captura preenchidos no lugar de `(pending)`. Nenhum dos dois construiveis hoje: nao existe app de desktop, e `check_host` recusaria a plataforma nova mesmo se existisse. |
| Hardware | duas maquinas nativas, um Windows 11 on ARM e um Linux ARM64 (Ubuntu 24.04 arm64) com DAC USB; nenhuma das duas existe neste ambiente, que e Windows 11 x86_64 (PROCESSOR_ARCHITECTURE=AMD64) sem hardware ARM, sem placa pro, sem Android e sem macOS | ausência é limitação registrada, não aprovação |

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
