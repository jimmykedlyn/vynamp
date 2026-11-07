# ADR-011 — pilha de áudio própria

Versão: 0.1.1 · Data: 2026-11-05 · Estado: decisão do mantenedor; implementação parcial nas bibliotecas (ver estado por componente).

Status: aceito pelo mantenedor em 2026-11-03. Substitui as partes de [ADR-002](ADR-002-audio-core.md) sobre decode, saída, resampling e tags (os "candidatos" Rust de terceiros de decode e saída).

## Contexto

O [ADR-002](ADR-002-audio-core.md) nomeou decoders e saídas Rust de terceiros como candidatos, e as specs de áudio citavam resamplers e uma biblioteca de tags de terceiros. O mantenedor prefere bibliotecas próprias, modulares e sem dependências de terceiros no caminho de áudio, cada uma em repositório próprio. Os designs dessas bibliotecas já existem; as APIs estão em estágios diferentes.

## Decisão

O Vynamp usa uma pilha de áudio própria. O núcleo Rust, a RT callback restrita e o caminho transparente inteiro separado do DSP float do ADR-002 permanecem.

| Componente | Biblioteca | Repositório e SHA fixo | Estado |
|---|---|---|---|
| Decode (PCM, WAV/W64/RF64/BW64, AIFF/AIFC, FLAC) | vynamp-decode | [jimmykedlyn/vynamp-decode @ 0391c64](https://github.com/jimmykedlyn/vynamp-decode/tree/0391c645d251d47c1fdb31b72fb8e49a6d1e0ac0) | API real: core, PCM, RIFF, AIFF/AIFC e FLAC (conformance IETF); planos 01 e 02 executados. Zero dependências de terceiros |
| Decode (MP1/2/3, AAC, ALAC, Ogg, CAF, AU, WavPack, APE, DSF/DFF etc.) | vynamp-decode | idem | Planejado nas Camadas 1–4 do [design](https://github.com/jimmykedlyn/vynamp-decode/blob/0391c645d251d47c1fdb31b72fb8e49a6d1e0ac0/docs/superpowers/specs/2026-10-23-vynamp-decode-design.md) §6 de vynamp-decode @0391c64; MPEG = plano 03 de vynamp-decode (plano ainda não escrito) |
| Saída de áudio | vynamp-output | [jimmykedlyn/vynamp-output](https://github.com/jimmykedlyn/vynamp-output): design @ [e534aad](https://github.com/jimmykedlyn/vynamp-output/blob/e534aad60f65918a37d6532fd923b70c84c36af4/docs/superpowers/specs/2026-10-25-vynamp-output-design.md); core em PR #1 @ [d0497b5](https://github.com/jimmykedlyn/vynamp-output/tree/d0497b5dc9851d6531bc25fc3cfc5865a23ab1c6) | Core (`RenderSource`, `StreamConfig`, `PresentationTiming`, backends Null e Capture, ADR-001 de render contract) em PR #1, ainda não integrado ao develop; backends de SO planejados (planos 02–04) |
| Efeitos vinil/válvula | vitrola — **módulo DSP interno** (sem repositório) | — | Somente design, aprovado em 2026-10-23 e incorporado em [10-vitrola](../../audio/10-vitrola.md) §8 (trait `Processor`); sem API ainda. Deixou de ser biblioteca em repositório próprio por decisão do mantenedor em 2026-11-05 ([ADR-014](ADR-014-public-github-actions-and-in-app-dsp.md)). Posição no grafo: depois do EQ e antes do volume; a relação com o resampler final está aberta em D-DSP-05 |
| Resampling | nova biblioteca própria de resampling (nome e repositório a definir pelo mantenedor) | a definir | Sem repositório, design nem plano |
| Tags: leitura | `vynamp-decode-meta` (crate de vynamp-decode) | idem vynamp-decode | Planejado: Camada 1 do design de vynamp-decode @0391c64; plano 04 de vynamp-decode (plano ainda não escrito) |
| Tags: escrita | `vynamp-decode-meta`, escopo futuro estendido (escrita com preservação de campos desconhecidos) | idem | Exige atualização do design de vynamp-decode; plano a definir |
| CUE | `vynamp-decode-cue` (crate de vynamp-decode) | idem | Planejado: Camada 1 do design de vynamp-decode @0391c64; plano 04 de vynamp-decode (plano ainda não escrito) |

FFmpeg continua admitido apenas como oráculo de teste e como adapter opcional isolado para codecs fora da pilha, conforme [privacidade e licenças](../../security/02-privacy-licensing.md).

A saída nunca faz resampling, dither nem ganho (design de vynamp-output @e534aad, §3 e §5.6); essas etapas pertencem ao grafo DSP do Vynamp.

**Escopo desta decisão.** Ela governa o caminho de **reprodução** — decodificação, processamento em tempo real, saída — e ali não há terceiros. Um **job offline** de processamento em lote (por exemplo, restauração por IA) é outro contexto: usar runtime de inferência de terceiros ali exige **ADR nova que emende esta**, com a exceção nomeada e datada, e nunca alcança a callback de áudio. Ver [ADR-015](ADR-015-ai-restoration-future-phase.md).

## Lacunas

| Lacuna | Biblioteca que cobre | Plano / camada |
|---|---|---|
| Resampler de alta qualidade | nova biblioteca própria de resampling | Nome a definir; sem plano. Atualiza D-DSP-08 e TASK-E04-02 |
| Escrita de tags | `vynamp-decode-meta` estendida | Plano a definir; exige atualizar o design de vynamp-decode |
| Leitura de tags/metadados e CUE antes do plano 04 | vynamp-decode-meta / vynamp-decode-cue | Plano 04 de vynamp-decode (plano ainda não escrito) |
| MP3 e demais formatos com perda; M4A, Ogg, CAF, AU, WavPack, APE, DSF/DFF | vynamp-decode | MP1/2/3: Camada 1, plano 03 (ainda não escrito); AAC-LC/ALAC/Ogg/CAF/AU: Camada 2; WavPack/APE/TTA/MPC/DSD→PCM/Matroska/WMA/HE-AAC: Camada 3; especiais: Camada 4 |
| WASAPI shared e ALSA simples | vynamp-output | Camada 1; planos 02 e 03 de vynamp-output (planos ainda não escritos) |
| Linux PipeWire nativo e PulseAudio | vynamp-output | Camada 3 do design de vynamp-output @e534aad |
| Android AAudio | vynamp-output | Camada 3; plano 04 de vynamp-output (plano ainda não escrito) |
| Exclusivo (WASAPI exclusive, ALSA hw, CoreAudio hog), capabilities, hot-plug, IDs estáveis | vynamp-output | Camada 2 do design de vynamp-output @e534aad |
| DSD/DoP nativo, 384/768 kHz, volume de hardware | vynamp-output | Camada 4 |
| macOS CoreAudio | vynamp-output | Plano 04 de vynamp-output; compila no alvo macOS Intel/Apple Silicon verificado pelo CI desde a [ADR-014](ADR-014-public-github-actions-and-in-app-dsp.md) — hardware de áudio ainda não testado |
| ASIO | fora do núcleo (design de vynamp-output §5.4) | Adapter privado opcional futuro; sem plano |
| iOS | nenhuma | Fora do escopo 1.0 do produto; sem backend |

## Alternativas

- Manter decoders, saída, resampler e tags de terceiros: menos trabalho próprio, mas contraria a preferência do mantenedor por bibliotecas próprias e modulares e mantém dependências no caminho de áudio.
- Adotar um resampler de terceiros: rejeitado pelo mantenedor; a escolha é uma nova biblioteca própria.
- Manter FFmpeg como decoder principal: licenciamento e superfície de ataque; fica como oráculo de teste e adapter opcional.

## Consequências

- Integração por dependência git com `rev` fixo (SHA completo), nunca por branch; atualizar o SHA exige PR com evidência. Os SHAs acima são os verificados em 2026-11-03.
- Enquanto uma camada não existir, o formato ou backend correspondente fica "planejado", sem alegar suporte; os spikes de TASK-E01-01/TASK-E01-02 passam a provar essas bibliotecas.
- O crate de spike `crates/vynamp-output` do Vynamp colide de nome com a biblioteca (design de vynamp-output §5.11) e deve ser renomeado ou removido na integração.
- O PR #14 (spike TASK-E01-02 com a biblioteca de saída de terceiros) fica como está, com métricas e evidências preservadas; a troca para vynamp-output é uma TASK própria após a Camada 1 (design de vynamp-output §5.11), no plano 05 de vynamp-output (plano ainda não escrito).
- Android usa AAudio direto (design de vynamp-output §5.4); Oboe e OpenSL ES não fazem parte da pilha.
- O perfil de licenças simplifica (sem dependências de terceiros nessas bibliotecas), mas o custo de cobertura de formatos, backends e qualidade do resampler passa a ser do projeto.
- Se uma lacuna não fechar a tempo de um marco, a spec correspondente deve ser rebaixada explicitamente, com novo ADR.
