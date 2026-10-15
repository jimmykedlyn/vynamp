# TASK-E17-03 — Completar bindings de containers e seleção de áudio

Versão 0.7.0 · Estado: planejada, não implementada.

## Contrato

Requisito REQ-E17-03 · M3 · P0 · responsável funcional audio (pessoa a designar). Esforço provisório 2–5 dias úteis por contrato; reestimar e quebrar em slices de 1–3 dias antes execução. Risco alto; estimativa não é prazo.

## Dependências

- [TASK-E01-08](TASK-E01-08.md) — Completar codecs desktop e large files

## Entrega concreta

CAF/AIFF-C/AU/Ogg FLAC, MP4 e Matroska/TS com seleção de faixas de áudio.

[Spec canônica](../../docs/audio/06-codec-capability-registry.md) · [Matriz normativa](../../docs/product/03-requirements.md).

## Aceite

Bindings exigidos passam duration/seek/cancel; WebM não anuncia FLAC; seleção language/default é determinística sem decodificar vídeo.

## Verificação e evidência

QA-025 em [QA](../../docs/quality/01-test-strategy.md) e [cenários](../../docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, fixtures/hashes/licenças, OS/ABI/hardware, esperado/observado e limitações. Documento/schema não aprova app. Aplicar Superpowers, owner/worktree exclusivos, TDD de comportamento e revisão do outro agente. Atualizar JSON/MD/REQ e capabilities juntos; somente aceitar com evidência.
