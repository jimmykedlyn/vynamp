# TASK-E17-08 — Medir teto no PCM final com oráculo independente

Versão 0.7.0 · Estado: planejada, não implementada.

## Contrato

Requisito REQ-E17-08 · M3 · P0 · responsável funcional qa (pessoa a designar). Esforço provisório 2–5 dias úteis por contrato; reestimar e quebrar em slices de 1–3 dias antes execução. Risco alto; estimativa não é prazo.

## Dependências

- [TASK-E17-07](TASK-E17-07.md) — Fixar proteção true-peak depois do processamento
- [TASK-E10-03](TASK-E10-03.md) — Criar harness numérico de áudio

## Entrega concreta

Harness no buffer do backend, estimador referência e casos adversariais multichannel.

[Spec canônica](../../docs/audio/07-dsp-signal-contract.md) · [Matriz normativa](../../docs/product/03-requirements.md).

## Aceite

Crossfade/EQ/gain/resample/dither finais não excedem ceiling +0.1 dBTP; taxas/bit-depths/NaN e latência documentados.

## Verificação e evidência

QA-027 em [QA](../../docs/quality/01-test-strategy.md) e [cenários](../../docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, fixtures/hashes/licenças, OS/ABI/hardware, esperado/observado e limitações. Documento/schema não aprova app. Aplicar Superpowers, owner/worktree exclusivos, TDD de comportamento e revisão do outro agente. Atualizar JSON/MD/REQ e capabilities juntos; somente aceitar com evidência.
