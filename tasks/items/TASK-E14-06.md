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
- **Não** valida installer, update, Universal2, APK/AAB, migrations nem SBOM. Installer, update, Universal2, APK/AAB e migrations são `TASK-E14-05`, ativação, backup e limpeza de integração são `TASK-E16-07` e `TASK-E16-08`, e o SBOM do artefato distribuído é `TASK-E11-04`; a matriz apenas registra o resultado, e refazer o smoke de instalação aqui exigiria as cinco máquinas de novo.
- **Não** roda o soak de 8 h em 24/192 nem a matriz de hardware com DAC desplugado, taxa trocada e CPU estressada. É `TASK-E10-08`; confundir soak com aceite de release transformaria uma medição de estabilidade em aprovação de plataforma.
- **Não** fecha o aceite Android 1.0 em aparelho físico, com lifecycle, foco, SAF/MediaStore, TalkBack e energia. É `TASK-E13-10`; esta TASK lê a linha android-arm64 do relatório dela e nada mais, porque o aparelho arm64 de 6GiB não existe nesta máquina.
- **Não** gera o manifest de capacidades por artefato nem decide quais codecs, cadeias de DSP e visualizers entram no build. São `TASK-E17-12` e `TASK-E17-20`; a matriz herda as exceções de capability que eles declaram, e inventar uma exceção aqui seria declarar suporte que o artefato não tem.
- **Não** monta a matriz de cobertura REQ/TASK/QA do desktop nem o relatório de problemas conhecidos do desktop. É `TASK-E10-10`, que esta TASK bloqueia; aqui só entram a linha de severidade por alvo que alimenta aquele relatório e a contagem de P0/P1 que fecha a stable, porque duplicar a matriz lá dentro faria dois documentos discordarem sobre o mesmo gate.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e14-06-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E14-06-AC-1` | os cinco conjuntos de evidência por alvo (windows-x64, linux-x64, macos-intel, macos-silicon e android-arm64) completos nas fases M1, M2 e M3, cada um com relatório de hardware real, com QA-018 no Linux e nos dois Macs, QA-019 e QA-020 no Android e QA-021 nos cinco alvos, e o relatório de acessibilidade de QA-010 (NVDA e Narrator no Windows, Orca no Linux, VoiceOver nos dois Macs, TalkBack no Android, nos DPI 100, 125, 150 e 200 no desktop e com texto a 200% no Android) | a matriz agregada é gravada em `evidence/release/1.0.0.json` e `cargo xtask release preflight --version 1.0.0` roda alvo por alvo sobre ela, recusando como aprovado o alvo provado só por cross-compile no CI ou só por emulador no lugar de hardware | Todos os alvos M1–M3 aprovados com hardware e acessibilidade |
| `TASK-E14-06-AC-2` | a matriz com quatro dos cinco alvos completos e o quinto declarado apenas por build cross-compile no CI, sem relatório de hardware — por exemplo macos-intel sem Mac Intel real, ou android-arm64 provado só no emulador x64 —, e nenhuma autorização de mudança de escopo registrada para esse alvo | `cargo xtask release preflight --version 1.0.0` roda sobre essa matriz e o fechamento da stable 1.0 multiplataforma é tentado | alvo ausente bloqueia stable multiplataforma |
| `TASK-E14-06-AC-3` | a matriz com os cinco alvos aprovados, a lista de problemas abertos por alvo montada a partir dos relatórios M1 a M3 de cada alvo com a severidade P0 ou P1 atribuída a cada um, e as exceções de capability declaradas no manifest de `TASK-E17-12`, cada linha de exceção trazendo `catalog_id`, `platform`, `reason` e `evidence` preenchidos como o `schemas/codec-capability.schema.json` exige | o relatório de problemas conhecidos e as linhas de `platforms` de `evidence/release/1.0.0.json` são conferidos antes do aceite da stable, contando P0 e P1 abertos e procurando exceção de capability sem motivo ou sem evidência | zero bugs P0/P1 e exceções por capability explícitas. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `cargo xtask release preflight --version 1.0.0` numa árvore com `evidence/release/1.0.0.json` e os cinco relatórios de alvo referenciados por caminho e sha256, complementado por `cargo xtask check-tasks` e pela conferência manual das cinco linhas de acessibilidade da matriz agregada em `evidence/` | Hoje o comando sai 1 na primeira cláusula de `xtask/src/release.rs`: `config/project.json` tem `application_ready`, `release_enabled` e `branding_exports_ready` em false e `product_version` em `0.1.0-dev`, então a mensagem em stderr é 'Release disabled: actual player/build/hardware/signing acceptance missing' (linha 76). As guardas seguintes também caem: `branding::exports` reprova com `branding_exports_ready` false, `validate_version` não deixa `0.1.0-dev` casar com 1.0.0, quatro dos oito pré-requisitos de arquivo não existem (`THIRD_PARTY_NOTICES`, `apps/android/gradlew`, `apps/android/gradlew.bat` e `packaging/update.pub`) e `ci::detect` devolve `core=false` porque `apps/desktop` não existe. Não existe `evidence/release/` nem `dist/release`, e `apps/player/src/main.rs` tem 3 linhas, `libs/engine/src/lib.rs` 6 e `libs/dsp/src/lib.rs` 6: não há player para aprovar em alvo nenhum. `evidence/` tem 38 entradas e 107 arquivos, nenhum deles relatório de hardware por alvo ou de leitor de tela. Nenhum teste aprova o `preflight`: em `xtask/tests/tooling.rs`, `release_cannot_publish_unimplemented_player` e `release_blocks_pending_native_branding_exports` param na primeira guarda e o segundo só confirma a palavra 'branding' no erro, e os quatro testes de `xtask/src/release.rs` (`artifact_manifest`, `source_digest`, `version_from_tag`, `release_tag`) não chegam a `preflight`. Quando a matriz existir, a evidência tem que mostrar as 5 linhas de `release::PLATFORMS` (windows-x64, linux-x64, macos-intel, macos-silicon, android-arm64) com `status` passed, `hardware` não vazio e `sha256` do relatório conferido, os 8 artefatos de `release::SUFFIXES` referenciados, 5 relatórios de acessibilidade nomeando o leitor de tela de cada alvo, 0 bugs P0/P1 abertos e cada exceção de capability com id vindo do manifest de TASK-E17-12 e motivo escrito. `cargo xtask check-tasks` não cobre isso: hoje responde `{"errors":[],"result":"passed","tasks":199}`, que é verde também com a matriz inexistente, porque ele só compara backlog e Markdown. |
| Hardware | precisa de cinco alvos reais: Windows 11 x64 com NVDA e Narrator, Linux x64 nativo em Wayland e X11 com Orca, um Mac Intel, um Mac Silicon com VoiceOver e um Android arm64 físico de porte médio com 6GiB e TalkBack; neste ambiente há o Windows 11 x64 sem NVDA instalado (não há `C:/Program Files/NVDA`, `C:/Program Files (x86)/NVDA` nem `%LOCALAPPDATA%/nvda`) e com apenas Intel display audio e Conexant ISST, sem DAC USB nem placa profissional, e há host Linux x64 apenas como WSL2 Ubuntu parado e Docker 29.8.1 headless, sem `/dev/snd`, sem sessão Wayland/X11 e sem Orca, que não dá a evidência de QA-010 nem de QA-018; não há Mac Intel, Mac Silicon nem aparelho Android ou iPhone conectado e `adb` não está no PATH, então nenhum dos cinco alvos pode ser aprovado nesta máquina e a ausência fica registrada como limitação, nunca como aprovação | ausência é limitação registrada, não aprovação |

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
