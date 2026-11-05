# TASK-E19-07 — Integrar o módulo DSP vitrola no grafo do player

Versão 0.8.0 · Estado planejado; módulo interno projetado não equivale a efeito implementado.

Fase M3 / P0 · Owner audio-dsp · Esforço inicial 8–20 dias úteis; reestimar.

## Dependências

- [TASK-E19-03](TASK-E19-03.md) — Dissolver o vitrola no DSP interno do app
- [TASK-E04-03](TASK-E04-03.md) — Construir grafo DSP e transições sem glitches

## Entrega concreta

Módulo DSP interno de vinil e válvula integrado ao grafo do player, com bypass, desativação automática, persistência de WearState e os testes VIT-T-01..10 executáveis.

Fecha a pendência que `docs/audio/10-vitrola.md` §6 registrava como "criar uma TASK de integração". O módulo é **interno ao app** — não é crate externo, não é repositório, não tem API pública. O grafo recebe um nó `Processor` que aceita f32 intercalado ou planar e recebe posição em samples; o host fornece `Context` por bloco (posição, duração, índice no álbum, total de faixas, eventos de transporte, contagem de reproduções).

Ordem: o efeito fica **depois do EQ e antes do volume do usuário**. O conflito D-DSP-05 (o vitrola diz "depois do resampler", o contrato 07 tem um único resampler final depois do master) permanece **preservado como requisito** e o contrato 07 não é alterado por esta tarefa. O overrun do efeito tem meta de CPU < 5% de um núcleo moderno a 96 kHz estéreo com tudo ligado — é meta, não gate.

Escopo: crate interno do workspace (nome a definir), `docs/audio/10-vitrola.md` (seção 3 do design passa de "proposto" para implementado), `docs/audio/03-dsp.md` (cadeia real), `docs/audio/07-dsp-signal-contract.md` (se D-DSP-05 for decidido), `docs/quality/05-dsp-test-matrix.md` (VIT-T-01..10), `docs/design/05-dsp-panel.md` (UX hospedeira).

Boundary: não altera o caminho bit-perfect nem DSD nativo (o efeito se desativa sozinho, com motivo visível); não introduz alocação nem lock na callback; não toca a acentuação de rota de saída nem o teto true-peak final.

## Critérios de aceite

Bypass transparente após o crossfade; desativação automática em bit-perfect e DSD com motivo visível; WearState persistido por álbum; os testes VIT-T-01..10 rodam com oráculo independente.

Detalhamento: VIT-T-01 compara o sinal após o crossfade com o seco e exige igualdade sample a sample; VIT-T-02 entra em transparente/DSD com o efeito ligado e exige bypass com motivo; VIT-T-03 com fonte 5.1 exige vinil indisponível com aviso e válvula por canal; VIT-T-06 exige bytes de WearState estáveis e versionados, com reset só do álbum pedido; VIT-T-09 publica o número de CPU medido em 96 kHz estéreo.

[Spec](../../docs/audio/03-dsp.md) · REQ-E19-07 em [matriz](../../docs/product/03-requirements.md).

## Evidência

QA-033, QA-005, QA-027; registrar commit/digest, comando, ambiente/OS/ABI, outputs e limitações. Ausência de hardware é limitação, não resultado aprovado. Quebrar em slices pequenas antes execução, implementar sucesso/erro/cancelamento, revisar pelo outro agente e preservar resultados no PR/docs/evidence.
