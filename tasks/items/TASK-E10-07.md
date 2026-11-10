# TASK-E10-07 — Instrumentar e medir performance

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E10-07` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E10-07` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M2 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E01-05`, `TASK-E03-05`, `TASK-E10-03` |
| Bloqueia | `TASK-E10-08` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–12 dias úteis |
| Risco | médio |

## Entrega

Métricas sanitizadas, callback histogram, launch/search benchmarks e baseline.

## Boundary

- **Não** executa o soak de 8 h em 24/192, o desplugue de DAC nem a matriz de hardware: é `TASK-E10-08`, que esta TASK bloqueia. Um histograma de 1 milhão de invocações em relógio virtual parece soak, mas não tem dispositivo nem tempo de parede, e esse número não sustenta a AC-1 daquela TASK.
- **Não** constrói os oráculos numéricos de PCM, resposta FFT, trim e mapa de canais. É `TASK-E10-03`; a evidência dela é comparação de amostra, não duração, e misturar as duas faria o relatório de performance carregar um oráculo que ele não verificou.
- **Não** implementa nem corrige a disciplina da callback (ring SPSC, seqlock de escritor único, zeros em vez de replay). Foi entregue em `TASK-E01-02` e aqui só se mede; reescrever o ring nesta TASK trocaria o objeto da medição pelo objeto da correção.
- **Não** entrega o painel diagnóstico em UI nem o export `.vyndiag`. A redaction e a retenção de log são de `TASK-E11-05` e o diagnóstico hi-res é de `TASK-E04-06`; aqui o produto é métrica coletada e baseline em JSON, que aquelas consomem.
- **Não** mede latência de startup de stream HTTP ou HLS. Ela precisa do servidor determinístico de `TASK-E10-04`, e medir contra rádio pública daria um número irreproduzível, que é o que a spec proíbe no CI.
- **Não** implementa a busca FTS que a meta de 100k p95 até 100 ms mede. A query é de `TASK-E03-05`; um benchmark de busca sobre tabela vazia passaria no limite sem provar nada.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e10-07-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E10-07-AC-1` | os sete módulos RT que `rt_guard.rs` lista em `RT_MODULES` (`render.rs`, `convert.rs`, `silence.rs`, `dop.rs`, `driver.rs`, `rtflags.rs` e `ring.rs` de `vynamp-output-core`) mais a instrumentação de métricas nova, com o contador por thread de `vynamp-output-testkit` declarado como global allocator do binário de teste | a instrumentação roda 200 períodos de 256 frames no `NullHost` com relógio virtual, e depois da execução o guarda estático e o número de alocações do device thread são conferidos | Instrumentação não faz I/O na callback |
| `TASK-E10-07-AC-2` | o gerador de relatório de benchmark que preenche hardware, versões e contagem de runs só a partir de `rustc -vV`, `cargo -V`, do host e do build, sem campo digitado à mão, rodando nesta máquina (Intel i5-8350U, 4 núcleos, 23,8 GiB, Windows 11 10.0.26300, rustc e cargo 1.99.0) | o relatório é gerado a partir da execução e cada campo de hardware, versão e contagem de runs é confrontado com o que o sistema e o build informam | reportar hardware, versões e runs |
| `TASK-E10-07-AC-3` | os budgets de `quality/02-performance.md` (launch frio p95 até 1,5 s, quente até 700 ms, memória idle até 120 MiB, CPU estéreo 24/192 até 5 por cento, busca 100k p95 até 100 ms, callback p99.9 abaixo de 25 por cento do período) e o protocolo de bench que exige build Release, processo aquecido, runs de no mínimo 30 para p95 e histograma de no mínimo 1 milhão de invocações | cada meta é confrontada com o valor medido no baseline em JSON, e a meta sem caminho de medição é marcada como não medida em vez de herdar o número da spec | metas são medidas e não presumidas. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `cargo run --manifest-path libs/output/tools/output-probe/Cargo.toml --locked -- --host null --clock real --periods 200` sobre o histograma de duração de callback que esta TASK adiciona ao lado do `TimingSeqlock`, mais `cargo test --manifest-path libs/output/crates/vynamp-output-core/Cargo.toml --locked` para os testes de `rt_guard.rs` e de `tests/zero_alloc_device.rs` | Hoje `output-probe` imprime `counters: callbacks=200 frames_delivered=51200 source_underruns=0 os_xruns=0` e `device-thread allocations: 0 (between the first and the last of 200 render calls)`, e `rt_guard.rs` cobre 7 módulos RT com 23 tokens proibidos — mas nenhum deles é de I/O: `std::fs`, `File` e `write` não estão na lista, então a AC-1 exige ampliar o guarda antes de valer. Histograma não existe: não há ocorrência de `histogram` em nenhum `.rs`, o `TimingSeqlock` publica só `device_frame`, `at`, `latency_frames` e `source`, e não há diretório `benches` nem dependência `criterion` em `Cargo.toml` nenhum, logo p99.9 abaixo de 25 por cento do período não é calculável hoje. Launch e search não têm o que medir: `apps/player/src/main.rs` tem 3 linhas de `println!` e a query de 100k é `TASK-E03-05`. O único host é o Null, como diz `vynamp-output/src/lib.rs`. |
| Hardware | nenhum: o histograma e os benchmarks rodam no `NullHost` sem abrir dispositivo de áudio — histograma em relógio virtual, probe e benchmarks em relógio real, com contador de alocação por thread; o hardware só aparece como campo do relatório, e esta máquina é Intel i5-8350U com 4 núcleos, 23,8 GiB e Windows 11 x64, sem DAC USB 24/192 — os endpoints são Conexant ISST Audio e áudio Intel para telas, então todo número de CPU ou memória daqui caracteriza esta máquina e não a máquina de referência da spec | ausência é limitação registrada, não aprovação |

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
