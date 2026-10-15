# TASK-E17-18 — Certificar expansão MIDI, trackers e DSD especial

Versão 0.7.0 · Estado: planejada, não implementada.

## Contrato

Requisito REQ-E17-18 · M5 · P1 · responsável funcional qa (pessoa a designar). Esforço provisório 2–5 dias úteis por contrato; reestimar e quebrar em slices de 1–3 dias antes execução. Risco alto; estimativa não é prazo.

## Dependências

- [TASK-E12-05](TASK-E12-05.md) — Entregar trackers, MIDI e codecs raros
- [TASK-E12-04](TASK-E12-04.md) — Entregar DSD nativo/DoP e PCM 768 kHz

## Entrega concreta

Fixtures de subsongs/loops/SF2/WavPack DSD/DST e native/DoP após implementações E12. Extração SACD ISO legal não protegida, sem handler ISO genérico.

[Spec canônica](../../docs/audio/06-codec-capability-registry.md) · [Matriz normativa](../../docs/product/03-requirements.md).

## Aceite

Rendering MIDI não é decode PCM; SoundFont licenciado explícito; loops/memória bound; DSD raw/DST/PCM/DoP distinguíveis. SACD ISO separa extração raw/DST, perfis e capacidade legal, sem contornar proteção.

## Verificação e evidência

QA-025, QA-026 em [QA](../../docs/quality/01-test-strategy.md) e [cenários](../../docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, fixtures/hashes/licenças, OS/ABI/hardware, esperado/observado e limitações. Documento/schema não aprova app. Aplicar Superpowers, owner/worktree exclusivos, TDD de comportamento e revisão do outro agente. Atualizar JSON/MD/REQ e capabilities juntos; somente aceitar com evidência.
