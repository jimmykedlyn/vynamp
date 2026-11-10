# TASK-E17-08 — Medir teto no PCM final com oráculo independente

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E17-08` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E17-08` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M3 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E17-07`, `TASK-E10-03` |
| Bloqueia | `TASK-E17-20` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Harness no buffer do backend, estimador referência e casos adversariais multichannel.

## Boundary

- **Não** implementa o guard final true-peak nem escolhe lookahead, oversampling ou canal-link: o nó e a política estão em `TASK-E17-07`, no repo vynamp-engine. Aqui se mede o PCM que ele entrega; mexer no guard para o número fechar transformaria o medidor em parte do objeto medido.
- **Não** escreve os oráculos PCM genéricos (resposta FFT, delay/padding, crossfade de referência, sample conversion, detector de descontinuidade): o harness numérico é `TASK-E10-03`. Esta TASK consome o que ele expõe e acrescenta só a coleta no buffer do backend e o estimador true-peak independente.
- **Não** corrige aliasing, ruído de quantização nem meta de resampler: provar o resampler e a sample conversion contra as metas do ADR é `TASK-E04-02`. Um overs aqui vira evidência para aquela TASK, não uma alteração de filtro feita neste trabalho.
- **Não** fecha o gate combinado de codecs, DSP e visualizers nem publica o veredito de release: a agregação dos cinco alvos é `TASK-E17-20`. Esta TASK entrega a medição por linha e o relatório de teto, não o PASS do gate.
- **Não** instrumenta CPU, p99.9 da callback nem budgets de performance: medição de performance é `TASK-E10-07`. A latência da AC-2 é o lookahead declarado e medido do guard, número de configuração, não benchmark sob carga.
- **Não** declara headroom por rota nem capacidade de dispositivo Bluetooth ou exclusivo: isso é `TASK-E17-19`, em vynamp-output. Aqui o backend é só o ponto de captura do PCM já quantizado.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e17-08-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E17-08-AC-1` | um fixture com crossfade equal-power entre duas fontes correlacionadas somadas em 0 dBFS, EQ gráfico 31 com +15 dB em três bandas mais preamp de +6 dB, ganho master 1,0, resampler 44,1 para 48 kHz e quantização I16 e I24 com dither ligado, mais a variante 192 para 384 kHz e a de 5.1 canais com o mesmo EQ | o backend Capture de libs/output grava o PCM quantizado que recebeu em .raw com o .manifest ao lado, e o harness aplica o estimador true-peak de referência sobre esse arquivo entregue, não sobre a saída do nó limiter | Crossfade/EQ/gain/resample/dither finais não excedem ceiling +0.1 dBTP |
| `TASK-E17-08-AC-2` | a matriz de QA-027 em cinco taxas (44,1, 48, 96, 192 e 384 kHz), três bit-depths (16, 24 e 32) com e sem dither, um trecho com NaN e Inf injetados depois de um filtro instável e um perfil de 5.1 canais com mapa identificável por canal | o harness percorre a matriz inteira, registra a linha de teto de cada combinação e emite o relatório com taxa, bit-depth, tratamento de NaN e o lookahead da proteção em amostras e em milissegundos na taxa da linha | taxas/bit-depths/NaN e latência documentados. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | um fixture novo em tests/fixtures/ceiling/ (PCM float32 de trem de impulsos, sweep correlacionado estéreo e o mesmo sinal em 5.1) mais um teste de integração que sobe o Capture host de libs/output com a feature test-capture e roda o estimador de referência sobre o .raw gravado. Hoje nada disso existe: tests/fixtures/ do umbrella só tem README.md, o workspace da raiz compila apenas xtask (members = ["xtask"]) e libs/engine/src/lib.rs tem 7 linhas cujo único símbolo é VERSION | para cada linha da matriz, o pico inter-sample do PCM entregue, medido por oversampling 4x com filtro FIR de referência independente da implementação sob teste, fica em -0,90 dBFS ou abaixo, isto é 0,9016 de escala cheia, 29542 de 32768 no I16 e 7562926 de 8388608 no I24; a evidência inclui taxa, bit-depth, presença de dither e o lookahead em amostras. O que ainda não existe hoje: nenhum código de true-peak nos 168 arquivos .rs do umbrella (busca por true_peak, dbtp, ceiling e oversampl devolve 0 linhas), nenhum crate de engine ou dsp além do stub de VERSION e nenhum estimador de referência versionado, então o teste falha antes de medir |
| Hardware | nenhum: o PCM medido é o arquivo .raw que o Capture host grava em libs/output/crates/vynamp-output-core/src/capture.rs com o formato e valid_bits declarados no .manifest, e nenhum dispositivo de áudio é aberto; a captura de DAC que a QA-027 cita como complemento não existe nesta máquina Windows, sem placa pro, sem Android, sem iOS e sem macOS | ausência é limitação registrada, não aprovação |

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
