# TASK-E17-17 — Expandir speech, AAC low-delay e legados

Versão 0.7.0 · Estado: planejada, não implementada.

## Contrato

Requisito REQ-E17-17 · M5 · P1 · responsável funcional audio (pessoa a designar). Esforço provisório 2–5 dias úteis por contrato; reestimar e quebrar em slices de 1–3 dias antes execução. Risco alto; estimativa não é prazo.

## Dependências

- [TASK-E01-08](TASK-E01-08.md) — Completar codecs desktop e large files
- [TASK-E12-05](TASK-E12-05.md) — Entregar trackers, MIDI e codecs raros

## Entrega concreta

Certificação AMR/3GP/Speex/LD/ELD/WMA Voice e bindings legacy opcionais.

[Spec canônica](../../docs/audio/06-codec-capability-registry.md) · [Matriz normativa](../../docs/product/03-requirements.md).

## Aceite

Fixture por perfil, cancel/seek limites e helper/licença publicados; não associar vídeo em lote nem prometer todos codecs.

## Verificação e evidência

QA-025 em [QA](../../docs/quality/01-test-strategy.md) e [cenários](../../docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, fixtures/hashes/licenças, OS/ABI/hardware, esperado/observado e limitações. Documento/schema não aprova app. Aplicar Superpowers, owner/worktree exclusivos, TDD de comportamento e revisão do outro agente. Atualizar JSON/MD/REQ e capabilities juntos; somente aceitar com evidência.
