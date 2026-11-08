# Evidência — chore-specs-ia-futura

Registra o que este PR mudou, o que foi verificado localmente e o que **não** foi.

## Origem do trabalho

O usuário pediu que o conteúdo dos arquivos Markdown da pasta raiz do projeto fosse incluído nas specs. A varredura dos 12 arquivos (`445.859 bytes`) mostrou que apenas **dois** tinham conteúdo ausente do repositório:

| Arquivo | Bytes | Situação antes deste PR |
|---|---|---|
| `AUDIOSR-RUST-CPU.md` | 10.467 | Especificação de super-resolução de banda por difusão latente, sem nenhuma representação no repositório |
| `SOURCE_SEPARATION_MODELS.md` | 3.630 | Especificação de separação de fontes por modelos ONNX, sem nenhuma representação no repositório |

Os outros dez já estavam absorvidos: letras (`LYR-01..15`, `LYR-AC-01..12`), DSP/equalizadores (`DSP-UX-01..21`), `AGENTS.md`, `STATE.md`, `LEIA-ME.md`, `ROADMAP-rodadas.md`, as duas ordens de trabalho executadas e os dois arquivos de SKIN. Os SKIN são **consumidores** — seus próprios changelogs citam `DSP-UX`, `VU-INT` e `LYR` como documentos canônicos deste repositório — e por isso não são fonte de requisito.

## Decisão do mantenedor

Em 2026-11-05, o mantenedor escolheu **"spec futura, sem mudar a 1.0"**: registrar a capacidade como fase pós-1.0, preservando integralmente a decisão da [ADR-013](../../../adr/ADR-013-restore.md) §4, que declara a separação de voz por IA como "decisão futura e não é prometida" e remove o tipo de região `overlay` da 1.0.

O problema real que este PR resolve não é a falta da capacidade: é a **especificação órfã** — detalhe de nível de implementação sem status normativo, que pode ser lido como promessa atrasada ou como descarte.

## Arquivos

| Arquivo | Mudança |
|---|---|
| [`docs/tools/03-vynamp-restore-ia.md`](../../../reference/tools/03-vynamp-restore-ia.md) | **Novo.** Spec da fase futura: estado, condições para sair de "planejada", alinhamento com a ADR-011, CPU/GPU, modelos, opt-in e download, processamento, `RSAI-01..14`, bloqueio legal, desempenho, aceite, riscos, escopo negativo, ordem de implementação e limitações |
| [`docs/architecture/adr/ADR-015-ai-restoration-future-phase.md`](../../../adr/ADR-015-restauracao-ia.md) | **Novo.** Registra a decisão: fase opt-in pós-1.0, cinco condições bloqueantes, pesos não embutidos, CPU como piso, escopo da ADR-011, rótulo de síntese, quatro alternativas rejeitadas |
| [`docs/product/05-decisions.md`](../../../product/05-decisoes.md) | 0.7.5 → 0.7.6. Seção "Decisões de restauração por IA (2026-11-05)" com **D-IA-01..06** e cinco divergências registradas |
| [`docs/tools/01-vynamp-restore.md`](../../../reference/tools/01-vynamp-restore.md) | 0.1.0 → 0.1.1. §1.2 e a nota sobre `kind` remetem à fase futura; a exclusão da 1.0 continua idêntica |
| [`docs/architecture/adr/ADR-013-vynamp-restore.md`](../../../adr/ADR-013-restore.md) | 0.1.0 → 0.1.1. Nota "Destino da 'decisão futura'" ao fim de §4 |
| [`docs/architecture/adr/ADR-011-own-audio-stack.md`](../../../adr/ADR-011-pilha-propria.md) | 0.1.0 → 0.1.1. Nota "Escopo desta decisão": vale no caminho de reprodução; runtime de ML offline exige emenda própria |
| [`docs/security/02-privacy-licensing.md`](../../../security/02-privacidade-licencas.md) | 0.7.1 → 0.7.2. Pesos de modelo como obra de terceiros no inventário, checksum, sem download automático, licença bloqueia release |
| [`docs/INDEX.md`](../../../INDEX.md) | Entrada na lista plana de ferramentas e nova seção datada |

**Nenhuma alteração de código, dependência, workflow ou backlog.** O build e o CI da 1.0 são idênticos depois deste PR.

## O que foi deliberadamente **não** feito

- **Sem TASK no backlog.** Uma TASK é compromisso de execução; criar `TASK-E19-08/09` prometeria o que §1.3 ainda bloqueia e alteraria as contagens da matriz (191 requisitos / 19 épicos) para trabalho que não entrou em fase. O precedente usado é o mesmo dos PRs `chore/claude-*` anteriores (`chore-specs-vynamp-restore`, `chore-specs-shell-integration`, `chore-dsp-eq-docs`): evidência em `docs/evidence/PR/chore-*/`, sem ID de TASK.
- **Sem REQ na matriz.** Os IDs `RSAI-01..14` são locais ao documento, como os `LYR-*` são por D-LYR-01.
- **Sem milestone.** Não há item de ML em nenhuma fase de [01-roadmap](../../../delivery/01-roadmap.md), e este PR não cria.
- **Sem tocar nos SKIN.** Os arquivos de skin ficam fora do repositório; a absorção das lacunas deles é o segundo PR desta rodada.

## Verificação

| Comando | Resultado |
|---|---|
| `cargo xtask check-docs` | ver abaixo |
| `cargo xtask check-delivery` | ver abaixo |
| `cargo fmt --all --check` | ver abaixo |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | ver abaixo |
| `cargo test --workspace --locked` | ver abaixo |

## Limitações

- **Nada foi medido.** Nenhum modelo foi baixado, nenhum runtime exercitado, nenhum spike executado. Tempo, RAM e qualidade em `03-vynamp-restore-ia.md` §9 são estimativas herdadas das especificações originais e estão marcadas como tal.
- **Nenhuma licença de peso foi conferida.** A regra de §8.2 é processo, não resultado.
- **Nenhuma ADR emenda a ADR-011.** A exceção para runtime de terceiros em job offline está proposta, não decidida.
- A ADR-015 não passou por revisão jurídica; a leitura de "obra de terceiros" para pesos de modelo é a mesma que a política de licenças já aplica a codecs.
- Este PR não altera visibilidade, proteções, assinatura nem plano — isso é da TASK-E19-06, por decisão do mantenedor na ADR-014.
