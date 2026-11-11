# Evidência — TASK-E04-02

| Campo | Valor |
|---|---|
| Data | 2026-11-11 |
| Autor | Claude (sessão `local_...`, execução autônoma autorizada pelo mantenedor) |
| TASK | [`TASK-E04-02`](../../../libs/dsp/tasks/items/TASK-E04-02.md) — Provar resampler e sample conversion |
| Repositório do código | `jimmykedlyn/vynamp-dsp`, branch `feature/task-e04-02-claude-resampler-spike` |
| ADR | [ADR-017 — qualidade do resampler e da conversão de amostras](../../../adr/ADR-017-resampler-e-conversao.md) |
| Artefatos | [`cpu-latency.json`](cpu-latency.json) — SHA-256 `836a2756f3df70ad4c482d58e099703d024247e3d537eec8d7502d78e82872bf` |

## O que foi medido

`cargo run --release --bin measure` em `vynamp-dsp`, bloco de 1 024 frames, 5 s de
entrada por linha, 200 blocos de aquecimento descartados. Nenhum DAC é tocado: as
medidas de CPU são de parede de relógio sobre o buffer, que é o que a TASK declara
como hardware permitido.

| Par | Modo | Taps | Latência | CPU de 1 núcleo | MAC/s |
|---|---|---|---|---|---|
| 44 100 → 48 000 | balanced | 97 | 52 frames | 1,51 % | 309 M |
| 44 100 → 48 000 | high | 261 | 141 frames ≈ 2,94 ms | 4,18 % | 299 M |
| 48 000 → 44 100 | balanced | 107 | 49 frames | 1,63 % | 290 M |
| 48 000 → 44 100 | high | 285 | 130 frames ≈ 2,95 ms | 4,25 % | 296 M |
| 44 100 → 96 000 | balanced | 97 | 104 frames | 3,29 % | 283 M |
| 44 100 → 96 000 | high | 261 | 283 frames ≈ 2,95 ms | 7,96 % | 315 M |
| 96 000 → 44 100 | balanced | 211 | 48 frames | 2,95 % | 315 M |
| 96 000 → 44 100 | high | 567 | 130 frames ≈ 2,95 ms | 7,83 % | 320 M |
| 44 100 → 192 000 | balanced | 97 | 209 frames | 6,01 % | 310 M |
| 44 100 → 192 000 | high | 261 | 566 frames ≈ 2,95 ms | 15,86 % | 316 M |
| 192 000 → 44 100 | balanced | 419 | 48 frames | 5,82 % | 317 M |
| 192 000 → 44 100 | high | 1 133 | 130 frames ≈ 2,95 ms | 15,72 % | 318 M |
| 48 000 → 192 000 | balanced | 97 | 192 frames | 6,56 % | 284 M |
| 48 000 → 192 000 | high | 261 | 520 frames ≈ 2,71 ms | 15,96 % | 314 M |
| 44 100 → 384 000 | balanced | 97 | 418 frames | 11,93 % | 312 M |
| 44 100 → 384 000 | high | 261 | 1 132 frames ≈ 2,95 ms | 31,64 % | 317 M |
| 384 000 → 44 100 | balanced | 837 | 48 frames | 11,61 % | 318 M |
| 384 000 → 44 100 | high | 2 265 | 130 frames ≈ 2,95 ms | 31,45 % | 318 M |

**Rejeição de alias medida** — tom acima do Nyquist de saída, energia no bin dobrado,
contra uma referência de 1 kHz na mesma configuração:

| Par | Modo | Medido | Alvo | Margem |
|---|---|---|---|---|
| 48 000 → 44 100 | balanced | −94,4 dB | 80 dB | 14,4 dB |
| 48 000 → 44 100 | high | −128,4 dB | 110 dB | 18,4 dB |
| 96 000 → 44 100 | balanced | −116,2 dB | 80 dB | 36,2 dB |
| 96 000 → 44 100 | high | −150,2 dB | 110 dB | 40,2 dB |
| 192 000 → 44 100 | balanced | −110,0 dB | 80 dB | 30,0 dB |
| 192 000 → 44 100 | high | −144,5 dB | 110 dB | 34,5 dB |

## O que estes números sustentam, e o que não sustentam

**Sustentam** que os dois modos atingem o alvo da ADR-017 com margem em todos os pares
medidos, que a latência publicada é a observada, e que o custo de CPU é compatível com
uma thread de áudio — mesmo o pior caso, 31,6 % de um núcleo, deixa folga.

**Não sustentam** nada sobre hardware de áudio. Não há dispositivo, não há callback RT,
não há orçamento de `p99.9` aqui: a TASK-E01-02 mediu a callback do output em 131,1 µs;
o que esta evidência acrescenta é quanto o resampler **consumiria** dela. Ligar as duas
medidas é trabalho da TASK-E04-03, e somar números de máquinas de medição diferentes
seria inferência, não medição.

Os números de CPU também são de **um** par de CPU e uma única execução por linha. Não
há `p99.9` e não há repetição. Duas execuções completas do mesmo binário nesta máquina
divergiram **~20 % em toda a coluna de CPU**: a primeira deu 3,36 % em 44,1k→48k `high`,
a segunda 4,18 %; o pior caso foi 25,8 % e depois 31,6 %. O que a tabela mostra é a ordem
de grandeza e a forma da curva — o `MAC/s` ficou entre 283 M e 320 M nas duas execuções,
e a latência não mudou —, não um orçamento fechado. Quem dimensionar o grafo deve usar a
leitura mais lenta.

## O que passou para a ADR-017

1. A tabela de taps, tabela de fases e latência por par, agora com o medido ao lado do
   projetado.
2. A rejeição de alias medida por modo, com o pior caso identificado: o par mais
   próximo (48k→44,1k), onde a transição é a mais estreita em frequência normalizada.
3. A leitura de que o MAC/s é ~constante e o que cresce com a razão é o custo de parede.
4. O custo medido de ~31 % no par extremo (44,1k↔384k), que é a justificativa
   registrada para a decimação em cascata aparecer como lacuna.

## Defeitos que os testes pegaram antes de irem para a branch

Dois, e ambos teriam ido para produção se o teste não existisse:

- **Saturação na redução de profundidade.** `(2^31−1 + 2^15) >> 16` dá 32768, um a
  mais que o máximo de 16 bits, e `32768 << 16` transborda para −2^31 — o pico
  positivo vira o pico negativo, um clique de escala inteira. Corrigido com `clamp`
  nos limites da profundidade de destino, e o teste
  `full_scale_narrows_without_wrapping_to_the_opposite_sign` fixa os dois extremos.
- **Antecipação não pré-carregada.** A instância começava com `written = 0`, então a
  meia-janela de zeros à frente do sinal não existia na prática: tudo saía deslocado
  em `half` amostras e a contagem de frames produzidos ficava abaixo do que as janelas
  fechadas permitiam. O teste `latency_is_what_the_stream_actually_holds_back`
  comparou a contagem com o número de janelas fechadas e reprovou.

Dois testes meus também estavam errados, e corrigi-los foi parte do trabalho, não um
arredondamento: o de ripple usava `0,44·fs_out` quando a banda comum é limitada pela
**menor** das duas taxas, e o de pico de impulso exigia amplitude 0,5 na decimação,
onde a janela é larga em amostras de entrada e mesmo a saída mais próxima do impulso é
uma amostra do lóbulo (0,29 em 96k→44,1k).

## Verificação

```
cargo fmt --all --check                       → limpo
cargo clippy --all-targets --locked -D warnings → limpo
cargo test --all --locked                     → 21 passaram, 0 falharam
```

Nenhum teste é `#[ignore]`: o maior tem 110 000 amostras de entrada e roda em ~6 s
para a suíte inteira. Não há fixture externa, corpus nem arquivo grande, então não há
variável de ambiente a documentar.

## Não absorvido, de propósito

- **A remoção do `PENDENTE` da `specs/dsp.md` §5.** O Boundary da TASK diz que o spike
  não aprova a spec; quem aprova o §5 é o mantenedor, por `GOVERNANCE.md`. O que fiz
  foi registrar em §3 o que o spike fixou, como proposta, sem mudar o estado da spec.
- **A posição do resampler no grafo.** A saída nunca faz resampling (ADR-011); quem
  liga isto na cadeia é a TASK-E04-03.
- **O modo `low`.** Não existe, e a ADR explica por quê.

## Pendências

- Re-pin do submódulo `libs/dsp` no umbrella depois do merge desta branch.
- A TASK-E04-02 continua `planned` no `tasks/backlog.json`: mover o estado exige o
  merge e é passo separado.
- Validação em hardware de áudio, que esta TASK não faz e a TASK-E04-10 cobre.
