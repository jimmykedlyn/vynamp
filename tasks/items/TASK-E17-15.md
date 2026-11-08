# TASK-E17-15 — Expandir IAMF como modelo e renderer

Versão 0.7.0 · Estado: planejada, não implementada.

## Contrato

Requisito REQ-E17-15 · M5 · P1 · responsável funcional audio (pessoa a designar). Esforço provisório 2–5 dias úteis por contrato; reestimar e quebrar em slices de 1–3 dias antes execução. Risco alto; estimativa não é prazo.

## Dependências

- [TASK-E17-01](TASK-E17-01.md) — Fixar catálogo por binding, capacidades e corpus
- [TASK-E17-03](TASK-E17-03.md) — Completar bindings de containers e seleção de áudio

## Entrega concreta

Demux IAMF, codecs subjacentes e mix presentations com rendering por layout.

[Spec canônica](../../reference/audio/06-codec-capability-registry.md) · [Matriz normativa](../../product/03-requisitos.md).

## Aceite

Opus/AAC-LC/FLAC/LPCM isolados não aprovam modelo; loudness/layout/DRC e renderer por referência com licenças.

## Verificação e evidência

QA-026 em [QA](../../quality/01-estrategia-testes.md) e [cenários](../../_arquivo/repo/docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, fixtures/hashes/licenças, OS/ABI/hardware, esperado/observado e limitações. Documento/schema não aprova app. Aplicar Superpowers, owner/worktree exclusivos, TDD de comportamento e revisão do outro agente. Atualizar JSON/MD/REQ e capabilities juntos; somente aceitar com evidência.
