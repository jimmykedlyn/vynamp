# Primeira execução recomendada

Versão: 0.8.0 · Data: 2026-11-10 · Estado: especificação proposta; provas M0 de E01-02, E02-11, E11-01, E15-03 e E19 entregues, nenhum código de player.

O status de cada TASK vive em [`backlog.json`](backlog.json) e não aqui. Este
arquivo responde uma pergunta só: qual TASK abrir depois das que já fecharam.
Entrega conta pelo critério da TASK no PR e no CI, nunca por esta lista.

## O que já está feito

Quatro provas M0 fecharam: `TASK-E11-01` (workspace e baseline de engenharia),
`TASK-E01-02` (callback e output clock), `TASK-E02-11` (MVP de referência
congelado, com SVGs, ícones e tokens) e `TASK-E15-03` (importação, Gitflow,
branch protection). O épico `E19` fechou inteiro — repositórios próprios,
política de segurança, e os gates `check-docs`, `check-tasks`, `check-delivery`,
`check-authoring` e `check-secrets` no GitHub Actions sob o agregado `CI Gate`.
`TASK-CI-01` e `TASK-CI-02` também. O umbrella hoje verifica documentação,
autoria das TASKs, pins de submódulo, direção de dependência e segredos; ele não
constrói aplicação (D-CI-1).

## A próxima TASK

`TASK-E01-01` — provar decode, gapless e precisão de samples. É a prova que
destrava a camada de áudio: a [ADR-002](../adr/ADR-002-audio-core.md) está
explicitamente **não congelada** até ela, e toda a fatia vertical depende disso.
Antes de ampliar dependências nativas, resolver `TASK-E11-02` (fronteiras de
segurança) e `TASK-E11-03` (licenças e inventário).

Depois dela, na ordem de dependência: `TASK-E04-02` (resampler e conversão de
sample), `TASK-E07-01` (sandbox WASM e capabilities) e `TASK-E10-01` (corpus de
testes licenciado). `TASK-E02-01` (toolkit Rust com UI clássica acessível) só
faz sentido com o corpus e o resampler provados, porque é ele que mede o gate do
leitor de tela.

## A primeira fatia vertical

Abrir WAV/FLAC/MP3 → decodificar → saída shared → pause/seek/stop → fila
independente → Classic Graphite com EQ10 e volume. Tasks E01-03/04/05,
E03-01/02, E04-01 e E02-02/03/04. O resultado tem que tocar de verdade, com
cancelamento e recovery, não apenas tela.

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
**não existem** no mobile, e nenhuma TASK mobile os traz. O layout do DSP é
adaptado ao orçamento de tela e de CPU de cada um em vez de portar o painel do
desktop: no iOS isso é `TASK-E20-05`; no Android, o EQ básico entra com
`TASK-E13-05` e a cadeia completa continua sendo `TASK-E04-01`, compartilhada
com o desktop. iOS é a fase 6 (épico `E20`, seis TASKs: bridge CoreAudio,
AVAudioSession, shell SwiftUI touch-first, sandbox de arquivos, DSP adaptado e
assinatura/TestFlight).
