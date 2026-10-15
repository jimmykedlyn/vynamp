# TASK-E17-07 — Fixar proteção true-peak depois do processamento

Versão 0.7.0 · Estado: planejada, não implementada.

## Contrato

Requisito REQ-E17-07 · M3 · P0 · responsável funcional audio (pessoa a designar). Esforço provisório 2–5 dias úteis por contrato; reestimar e quebrar em slices de 1–3 dias antes execução. Risco alto; estimativa não é prazo.

## Dependências

- [TASK-E04-03](TASK-E04-03.md) — Construir grafo DSP e transições sem glitches
- [TASK-E04-07](TASK-E04-07.md) — Entregar dynamics, tone e channel controls

## Entrega concreta

Guard final após mix/master/resampler e antes da quantização, separado do limiter criativo.

[Spec canônica](../../docs/audio/07-dsp-signal-contract.md) · [Matriz normativa](../../docs/product/03-requirements.md).

## Aceite

Nenhum boost/processamento posterior; ceiling padrão -1 dBTP; latency/reset/generation e modo transparente coerentes.

## Verificação e evidência

QA-027 em [QA](../../docs/quality/01-test-strategy.md) e [cenários](../../docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, fixtures/hashes/licenças, OS/ABI/hardware, esperado/observado e limitações. Documento/schema não aprova app. Aplicar Superpowers, owner/worktree exclusivos, TDD de comportamento e revisão do outro agente. Atualizar JSON/MD/REQ e capabilities juntos; somente aceitar com evidência.
