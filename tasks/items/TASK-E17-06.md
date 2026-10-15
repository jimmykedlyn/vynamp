# TASK-E17-06 — Certificar tags técnicas, capítulos e sidecars

Versão 0.7.0 · Estado: planejada, não implementada.

## Contrato

Requisito REQ-E17-06 · M3 · P0 · responsável funcional library (pessoa a designar). Esforço provisório 2–5 dias úteis por contrato; reestimar e quebrar em slices de 1–3 dias antes execução. Risco alto; estimativa não é prazo.

## Dependências

- [TASK-E03-08](TASK-E03-08.md) — Implementar escrita de tags e capítulos em lote

## Entrega concreta

Matriz read/write de ID3/MP4/Matroska/CAF/AIFF/BWF e LRC/WebVTT.

[Spec canônica](../../docs/audio/06-codec-capability-registry.md) · [Matriz normativa](../../docs/product/03-requirements.md).

## Aceite

Roundtrip preserva desconhecidos; capítulos/ratings e WVC não somem no rename; grant de sidecar não amplia acesso nem baixa URLs.

## Verificação e evidência

QA-025, QA-031 em [QA](../../docs/quality/01-test-strategy.md) e [cenários](../../docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, fixtures/hashes/licenças, OS/ABI/hardware, esperado/observado e limitações. Documento/schema não aprova app. Aplicar Superpowers, owner/worktree exclusivos, TDD de comportamento e revisão do outro agente. Atualizar JSON/MD/REQ e capabilities juntos; somente aceitar com evidência.
