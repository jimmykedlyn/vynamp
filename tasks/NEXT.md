# Primeira execução recomendada

Versão: 0.8.1 · Data: 2026-11-13 · Estado: especificação proposta; provas M0 de E01-01, E01-02, E02-11, E04-02, E11-01, E11-03, E15-03 e E19 entregues, nenhum código de player.

O status de cada TASK vive em [`backlog.json`](backlog.json) e não aqui. Este
arquivo responde uma pergunta só: qual TASK abrir depois das que já fecharam.
Entrega conta pelo critério da TASK no PR e no CI, nunca por esta lista.

## O que já está feito

As provas M0 de áudio fecharam: `TASK-E11-01` (workspace e baseline de
engenharia), `TASK-E01-02` (callback e output clock), `TASK-E01-01` (decode,
gapless e precisão de samples), `TASK-E04-02` (resampler e conversão de sample)
e `TASK-E02-11` (MVP de referência congelado, com SVGs, ícones e tokens).
Fecharam também `TASK-E15-03` (importação, Gitflow, branch protection) e
`TASK-E11-03` (licenças e inventário). O épico `E19` fechou inteiro —
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
produto da fatia vertical, e agora está destravada: as duas dependências dela,
`TASK-E01-01` e `TASK-E01-02`, fecharam. Depois vêm E01-04/05, E03-01/02,
E04-01 e E02-02/03/04: abrir WAV/FLAC/MP3 → decodificar → saída shared →
pause/seek/stop → fila independente → Classic Graphite com EQ10 e volume. O
resultado tem que tocar de verdade, com cancelamento e recovery, não apenas tela.

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
