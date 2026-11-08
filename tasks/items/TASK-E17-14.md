# TASK-E17-14 — Expandir AC-4 e apresentações de áudio

Versão 0.7.0 · Estado: planejada, não implementada.

## Contrato

Requisito REQ-E17-14 · M5 · P1 · responsável funcional audio (pessoa a designar). Esforço provisório 2–5 dias úteis por contrato; reestimar e quebrar em slices de 1–3 dias antes execução. Risco alto; estimativa não é prazo.

## Dependências

- [TASK-E17-01](TASK-E17-01.md) — Fixar catálogo por binding, capacidades e corpus
- [TASK-E17-03](TASK-E17-03.md) — Completar bindings de containers e seleção de áudio

## Entrega concreta

Adapter AC-4 condicionado, seleção de apresentação/DRC e capacidades.

[Spec canônica](../../reference/audio/06-codec-capability-registry.md) · [Matriz normativa](../../product/03-requisitos.md).

## Aceite

Apresentação/canais/loudness batem referência por build; não inferir disponibilidade por container MP4.

## Verificação e evidência

QA-026 em [QA](../../quality/01-estrategia-testes.md) e [cenários](../../_arquivo/repo/docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, fixtures/hashes/licenças, OS/ABI/hardware, esperado/observado e limitações. Documento/schema não aprova app. Aplicar Superpowers, owner/worktree exclusivos, TDD de comportamento e revisão do outro agente. Atualizar JSON/MD/REQ e capabilities juntos; somente aceitar com evidência.
