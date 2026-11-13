# Primeira execução recomendada

Versão: 0.9.0 · Data: 2026-11-13 · Estado: especificação proposta; provas M0 de E01-01, E01-02, E01-04, E02-11, E04-02, E11-01, E11-03, E15-03 e E19 entregues, nenhum código de player.

O status de cada TASK vive em [`backlog.json`](backlog.json) e não aqui. Este
arquivo responde uma pergunta só: qual TASK abrir depois das que já fecharam.
Entrega conta pelo critério da TASK no PR e no CI, nunca por esta lista. A
sequência completa, onda por onda, está em [`ORDER.md`](ORDER.md); o bloco
gerado abaixo repete só a primeira onda, e `cargo xtask check-docs` reprova este
arquivo quando ele diverge do backlog. Para reescrever: `cargo xtask docs-sync`.

## O que está destravado agora

<!-- BEGIN GENERATED: next -->
15 TASKs entregues de 215. A onda 1 tem 19 destravada(s) agora.

| ID | Título | Módulo | Fase | Prio |
|---|---|---|---|---|
| `TASK-E02-01` | Validar toolkit Rust com UI clássica acessível | `vynamp-player` | M0 | P0 |
| `TASK-E02-12` | Gerar gabarito visual e estratégia de fontes | `vynamp-player` | M0 | P0 |
| `TASK-E02-13` | Gerar gabarito de movimento, medidores e ondas | `vynamp-player` | M0 | P0 |
| `TASK-E02-14` | Gerar gabarito de comportamento e preferências | `vynamp-player` | M0 | P0 |
| `TASK-E07-01` | Provar sandbox WASM e capabilities | `vynamp-player` | M0 | P0 |
| `TASK-E10-01` | Criar corpus de testes licenciado | `vynamp` | M0 | P0 |
| `TASK-E11-02` | Definir fronteiras de segurança | `vynamp` | M0 | P0 |
| `TASK-E13-01` | Provar Android arm64, bridge Rust e output | `vynamp-player` | M0 | P0 |
| `TASK-E14-01` | Provar os cinco targets e features portáveis | `vynamp` | M0 | P0 |
| `TASK-E15-01` | Configurar Claude Code e Superpowers obrigatório | `vynamp` | M0 | P0 |
| `TASK-E16-01` | Provar envelope de ativação e protocolo comum | `vynamp-player` | M0 | P0 |
| `TASK-E01-03` | Implementar state machine e generation cancellation | `vynamp-engine` | M1 | P0 |
| `TASK-E03-01` | Implementar schema SQLite e migrations seguras | `vynamp-player` | M1 | P0 |
| `TASK-E15-04` | Ativar workflows e CI Gate multiplataforma | `vynamp` | M1 | P0 |
| `TASK-E01-05` | Implementar mixer, drain e transições básicas | `vynamp-engine` | M1 | P1 |
| `TASK-E01-07` | Adicionar codecs modernos e capítulos | `vynamp-decode` | M2 | P1 |
| `TASK-E21-01` | Entregar registry de encoders e disponibilidade por build | `vynamp-encode` | M3 | P0 |
| `TASK-E04-05` | Entregar saída exclusiva e device profiles | `vynamp-output` | M3 | P1 |
| `TASK-E20-01` | Provar iOS arm64, bridge Rust e CoreAudio | `vynamp-player` | F6 | P0 |
<!-- END GENERATED: next -->

## O que já está feito

As provas M0 de áudio fecharam: `TASK-E11-01` (workspace e baseline de
engenharia), `TASK-E01-02` (callback e output clock), `TASK-E01-01` (decode,
gapless e precisão de samples), `TASK-E04-02` (resampler e conversão de sample)
e `TASK-E02-11` (MVP de referência congelado, com SVGs, ícones e tokens).
Fecharam também `TASK-E15-03` (importação, Gitflow, branch protection),
`TASK-E11-03` (licenças e inventário) e `TASK-E01-04` (fonte local e decoder
registry, 2026-11-13). O épico `E19` fechou inteiro —
repositórios próprios, política de segurança, e os gates `check-docs`,
`check-tasks`, `check-delivery`, `check-authoring` e `check-secrets` no GitHub
Actions sob o agregado `CI Gate`. `TASK-CI-01` e `TASK-CI-02` também. O umbrella
hoje verifica documentação, autoria das TASKs, pins de submódulo, direção de
dependência e segredos; ele não constrói aplicação (D-CI-1).

## A próxima TASK

`TASK-E11-02` — definir as fronteiras de segurança. É a que o próprio backlog
ordena antes de ampliar dependências nativas, e é o que ainda falta para o
sandbox de plugins ter um contrato. Roda junto com `TASK-E07-01` (spike de
sandbox WASM e capabilities), que depende só de `TASK-E11-01` e é o que prova o
`ADR-004` na prática.

`TASK-E10-01` (corpus de testes licenciado) é a terceira da fila: o corpus de
gapless do MPEG já está licenciado e com manifesto medido pela `TASK-E01-01`,
mas o corpus licenciado de tons, sweep e corrupções continua faltando.

## A primeira fatia vertical

`TASK-E01-03` (state machine e generation cancellation) é a primeira TASK de
produto da fatia vertical, e está destravada desde que `TASK-E01-01` e
`TASK-E01-02` fecharam. `TASK-E01-04` (fonte local e decoder registry) fechou em
2026-11-13, então `TASK-E01-05` (mixer, drain e transições) e `TASK-E01-07`
(codecs modernos e capítulos) também estão destravadas. A fatia segue: abrir
WAV/FLAC/MP3 → decodificar → saída shared → pause/seek/stop → fila independente
→ Classic Graphite com EQ10 e volume. O resultado tem que tocar de verdade, com
cancelamento e recovery, não apenas tela.

Não abrir `TASK-E02-01` (toolkit Rust com UI clássica acessível) antes do corpus
e do resampler estarem provados, porque é ela que mede o gate do leitor de tela.

## Depois

Biblioteca, tags e busca; Opus/AAC/ALAC/Vorbis; rádio por URL e podcast.
Congelar os contratos de tema e de plugin antes do editor e do registry. Backend
web só em M4, depois das provas do provider mock.

## Multiplataforma

Cinco alvos obrigatórios desde as provas até 1.0: Windows, Linux, macOS Intel,
macOS Apple Silicon e Android. `TASK-E13-01` (Android arm64, bridge Rust e
output) e `TASK-E14-01` (os cinco targets e features portáteis) entram em M0
depois de `TASK-E11-01`, que já fechou. A primeira fatia M1 tem que tocar
nativamente nos cinco: ports E14-02 e shell/service/focus/UI E13-02/03/05.
Completar só Windows e chamar o núcleo portátil de release multiplataforma é
exatamente o que a spec proíbe. E12-01/02 são portas M3, com prova inicial em
M0/M1.

## Mobile: o que não existe lá

Android e iOS recebem o player, não a suíte. `apps/converter` e `apps/restore`
**não existem** no mobile, e nenhuma TASK mobile os traz — é `D-MOB-01`, com a
fonte em [ADR-012 §1](../adr/ADR-012-integracao-shell.md#1-arquitetura-de-integração-por-plataforma)
e nas specs de [Android](../platforms/02-android.md#escopo-do-app-só-o-player) e
[iOS](../platforms/05-ios.md). O layout do DSP é adaptado ao orçamento de tela e
de CPU de cada um em vez de portar o painel do desktop, com o mapeamento área por
área em [painel DSP §10](../design/06-painel-dsp.md#10-layout-no-toque-android-e-ios)
e os limites de ajuste da §8.5 intactos (`D-MOB-02`): no iOS isso é
`TASK-E20-05`; no Android, o EQ básico entra com `TASK-E13-05` e a cadeia
completa continua sendo `TASK-E04-01`, compartilhada com o desktop. iOS é a fase
6 (épico `E20`, seis TASKs: bridge CoreAudio, AVAudioSession, shell SwiftUI
touch-first, sandbox de arquivos, DSP adaptado e assinatura/TestFlight).
