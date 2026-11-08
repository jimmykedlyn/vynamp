# Evidência — TASK-E01-02 (callback e output clock)

Data: 2026-11-07 · Autor: Claude Opus 5.5 · TASK: [TASK-E01-02](../../../tasks/items/TASK-E01-02.md) · Spike original: 2026-10-18, branch `feature/task-e01-02-claude-callback`, [PR #14](https://github.com/jimmykedlyn/vynamp-mediaplayer/pull/14) (draft, aberto).

Registra o que o spike de M0 **provou**, com que números, e o que deste trabalho **passou para as specs canônicas** em 2026-11-07. O spike não é player, não altera flags de prontidão e não aceita a TASK: revisão, integração e os demais hosts continuam pendentes.

## O que foi medido

Probe real em Windows/WASAPI shared, dispositivo padrão `Headset (USB 384KZ HIFI Audio)`, 384 000 Hz, 2 canais, f32, buffer 3 840 frames. O produtor envia senoide de 440 Hz com amplitude configurável (`output_probe.rs`); em amplitude 0.0 são frames de conteúdo reais com silêncio digital, inaudíveis. Ambiente completo em [environment.txt](environment.txt). O oráculo de rampa — que confere amostra a amostra ordem, wraparound e ausência de sobreposição — está nos testes do ring, não no probe: aqui o que se compara é a contagem de frames produzidos, consumidos como conteúdo e zerados, mais a posição apresentada amostrada a cada 100 ms.

| Métrica | [Sem stall](probe-steady.json) | [Stall de 300 ms em t=10 s](probe-stall.json) |
|---|---|---|
| Produzido / consumido como conteúdo | 11 520 000 / 11 520 000 | 11 520 000 / 11 520 000 |
| Underruns / frames zerados | 0 / 0 | 25 / 94 976 (≈247 ms dos 300 ms) |
| Silêncio de drenagem, depois do fim de stream | 89 088 | 109 312 |
| Eventos de heap na callback | 0 | 0 |
| Callbacks do dispositivo | 3 022 | 3 052 |
| Duração máx. / p99.9 (limite do bucket) | 168,5 µs / ≤131,1 µs | 102,2 µs / ≤131,1 µs |
| p99.9 ÷ menor período (10 ms) | 0,0131 (gate < 0,25) | 0,0131 |
| Latência do dispositivo (frames) | 12 393–12 800 | 12 304–12 802 |
| Taxa do clock ÷ nominal | 1,00017 | 0,99163 (= 1 − 3 214 ÷ 384 000; o déficit de 3 214 frames/s ao longo de ≈29,55 s é o silêncio de underrun, 94 976 frames) |
| Regressões de posição apresentada | 0 | 0 |
| Erros de stream | 0 | 0 |

SHA-256: `probe-steady.json` `a3621cc778aefb278f4d013576cdef808724b1a6b5f4097fbc7ebf022e38931c`; `probe-stall.json` `4844d2fd49fd65ec35188cd154ee44c294d519e55dbddd3207953c124d3b4f01`. Os mesmos hashes foram reconferidos nesta cópia.

O que estes números sustentam, e o que não sustentam: provam que a callback consome PCM pronto sem alocar e dentro de 1,3% do período medido, mesmo durante underrun real; que sob stall de 300 ms o dispositivo recebe zeros sem repetir amostra; e que a posição apresentada não regride. **Não** provam 8 h de soak, não provam Linux/macOS/Android, e o CI não compila o backend de OS (feature desligada, sem headers ALSA).

## O que passou para as specs canônicas

Absorvido em [01-engine.md](../../../reference/audio/01-engine.md) §Threads (versão 0.7.2), mais a limitação do §"Limitação conhecida do relógio" abaixo, que passou a estar também naquela seção:

1. **Semântica do relógio apresentado.** `presented = max(anterior, conteúdo escrito antes desta callback − latência do primeiro frame do buffer)`. A latência informada pelo backend é o atraso até o primeiro frame do buffer em escrita, logo se mede contra o conteúdo das callbacks **anteriores**. Medir contra o conteúdo já acrescentado superestima a posição em um buffer inteiro do dispositivo — foi um defeito real do spike, corrigido em `0bf193e` com testes que falharam antes.
2. **Seqlock de escritor único** para publicar o snapshot: leitores repetem a leitura, a callback nunca espera.
3. **Ordem de leitura na drenagem.** A callback lê a marca de fim de stream antes de medir o ring; sem isso, o fim de toda reprodução seria contado como underrun.
4. **Protocolo do ring SPSC** — uma única alocação na construção, contadores monotônicos `u64`, handoff por store Release dos contadores depois de gravar/ler, nenhum dos lados espera.

Não absorvido, de propósito: o código dos crates. A [ADR-011](../../../adr/ADR-011-pilha-propria.md) manda a saída de áudio para o repositório `vynamp-output`, e a própria ADR registra que o crate de spike `crates/vynamp-output` do umbrella colide de nome com a biblioteca e deve ser renomeado ou removido na integração (§Consequências). Trazê-lo para cá criaria o defeito que a decisão proíbe. O código e os testes ficam no PR #14.

## Limitação conhecida do relógio

Caracterizada por teste, razão registrada aqui porque a correção é da E01-05:

`presented = conteúdo escrito antes da callback − latência` supõe que a fila do dispositivo contém **só** conteúdo. Silêncio de underrun ou de drenagem enfileirado também compõe a latência reportada, então `presented` fica **subestimado** em no máximo `latency_frames` — ≈12 300–12 800 frames no hardware medido. Após um underrun ele atrasa até a latência inteira e volta a acompanhar quando o conteúdo flui; na drenagem estabiliza em `content_frames − latency_frames` sem alcançar `content_frames`. Nunca superestima e nunca regride — os 0 regressões da tabela acima incluem o run com stall. Sem redesenho nesta TASK; a semântica correta depende do dreno completo da E01-05. O mesmo texto está na [TASK-E01-02](../../../tasks/items/TASK-E01-02.md).

Os dois JSON confirmam o limite numericamente, e não só qualitativamente: a posição final fica **abaixo** do conteúdo nas duas execuções, pela latência — `probe-steady` termina em 11 507 474 contra 11 520 000 de conteúdo (12 526 de diferença, dentro da faixa de latência medida de 12 393–12 800), e `probe-stall` em 11 507 560 (12 440, dentro de 12 304–12 802). É exatamente o `content_frames − latency_frames` previsto. Quem for consertar o dreno deve ver esses dois números virarem `content_frames`.

## Instrumentação RT: a receita que funcionou

Vale registrar porque a próxima biblioteca de saída vai precisar dela:

- **Contador por thread, não global.** O primeiro CI do spike falhou em `render_allocates_nothing` com 2 eventos. A causa não era alocação do render: o teste media o contador global do processo enquanto outro teste, no mesmo binário e em paralelo, fazia um alloc+dealloc dentro da própria seção RT. A correção foi contar por thread. Um contador global só serve onde a única seção RT é a thread de callback medida.
- **Guarda estática complementar.** Os módulos do caminho RT delimitam regiões `RT-BEGIN`/`RT-END`, e um teste rejeita tokens de bloqueio, alocação, I/O e log no código dessas regiões (comentários são ignorados). É complemento, não substituto da revisão.
- **Testes de concorrência contra o escalonador.** O teste de seqlock falhava em ~5 de 30 execuções porque o escritor terminava antes de o leitor ser escalonado. Passou a escrever até o leitor completar um número mínimo de leituras — [50 execuções depois, zero falhas](clock-flaky-50x.txt), 6 testes por execução. Um teste de concorrência que depende do escalonador é defeito latente, mesmo quando passa.
- **A limitação do relógio vira teste de caracterização**, não comentário solto: `presented_bias_with_queued_silence_is_bounded_by_latency`.

## Pendências

- O codec da saída, o dreno completo (E01-05) e a interpolação entre callbacks ficam nas TASKs donas.
- Linux, macOS Intel/Apple Silicon e Android não foram medidos; o backend de OS não é compilado no CI.
- `TASK-E11-03` absorve licenças e inventário transitivo do CPAL 0.18.2 usado no spike.
