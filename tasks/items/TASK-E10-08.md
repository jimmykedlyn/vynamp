# TASK-E10-08 — Executar soak e matriz de hardware

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E10-08` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E10-08` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E04-10`, `TASK-E01-10`, `TASK-E10-07`, `TASK-E14-04` |
| Bloqueia | `TASK-E10-10`, `TASK-E14-06` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | médio |

## Entrega

8 h em 24/192, DAC unplug, taxa, busy/sleep e stress de CPU.

## Boundary

- **Não** implementa a pausa no desplugue nem a recuperação de device lost. O comportamento é de `TASK-E04-05`, cuja AC já exige pausa em busy, unplug e sleep sem fallback automático para o alto-falante; aqui só se executa o cenário e se registra o que aconteceu, e corrigir a política nesta TASK criaria dois donos do mesmo caminho.
- **Não** entrega o modo transparente nem os diagnósticos hi-res. É `TASK-E04-06`; a captura exata de PCM24/32 nasce lá, e o soak aqui só reexecuta sobre combinação que aquela TASK já validou.
- **Não** mede qualidade numérica, resposta FFT, trim, delay ou mapa de canais. É `TASK-E04-10`, com o harness de `TASK-E10-03`; um relatório de soak com amostra errada seria laudo de estabilidade se passando por laudo de correção.
- **Não** roda o corpus nos cinco alvos nem fecha a matriz por plataforma. É `TASK-E14-04`, e o agregado de release é `TASK-E14-06`; esta TASK entrega a linha do ambiente de referência Windows, e copiar as outras quatro de build cross-compile seria declarar alvo aprovado sem máquina.
- **Não** abre baseline nem compara regressão percentual. Isso é `TASK-E10-07`, dependência desta TASK; comparar runs aqui duplicaria o baseline e as duas TASKs discordariam sobre o mesmo número.
- **Não** repete o soak de engine de `TASK-E01-10`. Lá o alvo é o motor com fonte, drain e cancelamento; aqui é a cadeia inteira até o dispositivo, e o relatório precisa dizer qual dos dois produziu cada número.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e10-08-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E10-08-AC-1` | a máquina de referência da spec (Windows 11 x64, 4 núcleos, 16 GiB, SSD, DAC USB 24/192 em modo compartilhado e em exclusivo) tocando FLAC estéreo 24/192 por 8 h contínuas, com o baseline de `TASK-E10-07` já registrado | o soak termina e `Stream::counters()` com `source_underruns` e `os_xruns`, os eventos `Event::SourceUnderrun` e o estado de energia da máquina são lidos e cada underrun é atribuído ao app ou ao sistema operacional | Zero underruns causados pelo app no ambiente de referência |
| `TASK-E10-08-AC-2` | reprodução em andamento num DAC USB em modo exclusivo, com a opção de cair para o alto-falante interno desligada na configuração | o cabo do DAC é retirado durante a reprodução e a interface e o estado do stream são observados | pausar ao unplug |
| `TASK-E10-08-AC-3` | modo transparente com EQ, ReplayGain e volume digital desligados, fonte PCM24 e PCM32, com o `CaptureHost` gravando os bytes entregues ao dispositivo e o manifesto `<path>.manifest` ao lado | os bytes capturados são comparados amostra a amostra com o arquivo de origem e o manifesto é lido para conferir `frames`, `period_frames`, `channels` e `overruns` | captura transparente real |
| `TASK-E10-08-AC-4` | as combinações a exercitar (44,1, 48, 96 e 192 kHz; PCM24, PCM32 e float; modo compartilhado e exclusivo) e o manifesto de driver, versão e build de cada célula, com as combinações que este host alcança e as que não alcança | cada combinação é executada e o relatório da matriz é publicado com uma célula por combinação, e cada célula que não rodou é marcada como não medida com o motivo em vez de ficar implícita | limites declarados. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `cargo xtask quality-gate`, a entrada prevista em `xtask/src/ci.rs` que exige `application_ready` verdadeiro, `tests/hardware/Cargo.toml` e roda o harness com `--soak --device-loss --gapless --transparent --report-dir dist/evidence`, sobre um DAC USB 24/192 e 8 h de tempo de parede | Hoje o comando aborta na primeira cláusula com `Hardware gate blocked: application not implemented` e exit 1, porque `config/project.json` tem `application_ready` em false e `tests/hardware/Cargo.toml` não existe; não há `dist/`. Nenhum backend de OS é compilado: `vynamp-output/src/lib.rs` declara que só existe o `NullHost` e `hosts()` devolve um host Null, então nem modo exclusivo nem desplugue real são executáveis. O que existe serve de caracterização, não de prova: 130 testes em `libs/output`, entre eles `underrun_tail_is_silence_and_samples_are_never_replayed` em `driver.rs:370`, `NullClock::inject_device_lost` em `null.rs:115`, `Event::SourceUnderrun` em `event.rs:27` e o manifesto do `CaptureHost` fechando em `overruns=0` — todos em relógio virtual. Registro de DAC: nenhum em `evidence/`, que tem 107 arquivos em 26 diretórios de PR e só `task-e10-01` no épico E10. |
| Hardware | DAC USB 24/192 e a máquina de referência da spec — ausentes: esta máquina é Intel i5-8350U, 4 núcleos, 23,8 GiB, Windows 11 x64, e os endpoints enumerados são Conexant ISST Audio com seu alto-falante interno e o áudio Intel para telas; nenhum DAC USB está presente na enumeração — um `USB 384KZ HIFI Audio` consta no registro do Windows sem estar anexado —, então nem o 24/192 em modo exclusivo nem o desplugue podem ser executados aqui | ausência é limitação registrada, não aprovação |

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
