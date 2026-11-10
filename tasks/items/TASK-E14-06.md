# TASK-E14-06 — Fechar release 1.0 multiplataforma

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E14-06` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E14-06` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M3 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E14-04`, `TASK-E14-05`, `TASK-E13-10`, `TASK-E10-08`, `TASK-E16-07`, `TASK-E16-08`, `TASK-E17-12`, `TASK-E17-20` |
| Bloqueia | `TASK-E10-10` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | alto |

## Entrega

Matriz de evidência agregada para Windows/Linux/macOS Intel/macOS Silicon/Android.

## Boundary

- **Não** executa o corpus de áudio nem mede clock, seek, drain ou gapless em cada alvo. Isso é `TASK-E14-04`; aqui só se consome o relatório por OS/ABI que ela produz, e reexecutar o corpus nesta TASK criaria um segundo oráculo numérico para o mesmo backend.
- **Não** valida installer, update, Universal2, APK/AAB, migrations nem SBOM. É `TASK-E14-05`, com `TASK-E16-07` e `TASK-E16-08` para ativação, backup e limpeza de integração; a matriz apenas registra o resultado, e refazer o smoke de instalação aqui exigiria as cinco máquinas de novo.
- **Não** roda o soak de 8 h em 24/192 nem a matriz de hardware com DAC desplugado, taxa trocada e CPU estressada. É `TASK-E10-08`; confundir soak com aceite de release transformaria uma medição de estabilidade em aprovação de plataforma.
- **Não** fecha o aceite Android 1.0 em aparelho físico, com lifecycle, foco, SAF/MediaStore, TalkBack e energia. É `TASK-E13-10`; esta TASK lê a linha android-arm64 do relatório dela e nada mais, porque o aparelho arm64 de 6GiB não existe nesta máquina.
- **Não** gera o manifest de capacidades por artefato nem decide quais codecs, cadeias de DSP e visualizers entram no build. São `TASK-E17-12` e `TASK-E17-20`; a matriz herda as exceções de capability que eles declaram, e inventar uma exceção aqui seria declarar suporte que o artefato não tem.
- **Não** monta a matriz de cobertura REQ/TASK/QA do desktop nem o relatório de problemas conhecidos do desktop. É `TASK-E10-10`, que esta TASK bloqueia; duplicar a matriz lá dentro faria dois documentos discordarem sobre o mesmo gate.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e14-06-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E14-06-AC-1` | os cinco conjuntos de evidência por alvo (windows-x64, linux-x64, macos-intel, macos-silicon e android-arm64) completos nas fases M1, M2 e M3, cada um com relatório de hardware real e com os relatórios de acessibilidade de QA-018 a QA-021 (NVDA e Narrator no Windows, Orca no Linux, VoiceOver nos dois Macs, TalkBack no Android, nos DPI 100/125/150/200) | a matriz de evidência agregada é fechada em evidence/ e o aceite é conferido alvo por alvo contra as fases M1 a M3, sem reprovar por cross-compile nem por emulador no lugar de hardware | Todos os alvos M1–M3 aprovados com hardware e acessibilidade |
| `TASK-E14-06-AC-2` | a matriz com quatro dos cinco alvos completos e um deles declarado apenas por build cross-compile no CI, sem relatório de hardware — por exemplo macos-intel sem Mac Intel real, ou android-arm64 provado só no emulador x64 | o fechamento da stable 1.0 multiplataforma é tentado sobre essa matriz, sem pedido explícito de redução de escopo aprovado | alvo ausente bloqueia stable multiplataforma |
| `TASK-E14-06-AC-3` | matriz com os cinco alvos aprovados, a lista de problemas conhecidos por alvo e as exceções de capability declaradas no manifest de TASK-E17-12, cada uma nomeando capability, alvo, motivo e evidência | o item de qualidade de release é conferido sobre a matriz antes do aceite da stable | zero bugs P0/P1 e exceções por capability explícitas. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `cargo xtask release preflight --version 1.0.0` numa árvore com `evidence/release/1.0.0.json` e os cinco relatórios de alvo referenciados por caminho e sha256, complementado por `cargo xtask check-tasks` e pela conferência manual das cinco linhas de acessibilidade da matriz agregada em `evidence/` | Hoje o comando para na primeira cláusula de `xtask/src/release.rs`: `config/project.json` tem `application_ready`, `release_enabled` e `branding_exports_ready` em false e `product_version` em `0.1.0-dev`, então a mensagem é 'Release disabled: actual player/build/hardware/signing acceptance missing' (linha 76). Não existe `evidence/release/` nem `dist/release`, e `apps/player/src/main.rs` tem 3 linhas, `libs/engine/src/lib.rs` 6 e `libs/dsp/src/lib.rs` 6: não há player para aprovar em alvo nenhum. `evidence/` tem 38 entradas e 107 arquivos, nenhum deles relatório de hardware por alvo ou de leitor de tela. Os únicos testes que tocam esse caminho são `release_cannot_publish_unimplemented_player` e `release_blocks_pending_native_branding_exports` em `xtask/tests/tooling.rs`, e eles provam o bloqueio, nunca a aprovação. Quando a matriz existir, a evidência tem que mostrar as 5 linhas de `release::PLATFORMS` (windows-x64, linux-x64, macos-intel, macos-silicon, android-arm64) com `status` passed, `hardware` não vazio e `sha256` do relatório conferido, os 8 artefatos de `release::SUFFIXES` referenciados, 5 relatórios de acessibilidade nomeando o leitor de tela de cada alvo, 0 bugs P0/P1 abertos e cada exceção de capability com id vindo do manifest de TASK-E17-12 e motivo escrito. |
| Hardware | precisa de cinco alvos reais: Windows 11 x64 com NVDA e Narrator, Linux x64 nativo em Wayland e X11 com Orca, um Mac Intel, um Mac Silicon com VoiceOver e um Android arm64 físico de porte médio com 6GiB e TalkBack; neste ambiente existe só o Windows 11 x64, sem DAC USB nem placa de áudio profissional, e Mac Intel, Mac Silicon, Linux nativo e aparelho Android físico estão ausentes, então quatro dos cinco alvos não podem ser aprovados nesta máquina e a ausência fica registrada como limitação, nunca como aprovação | ausência é limitação registrada, não aprovação |

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
