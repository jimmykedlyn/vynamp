# TASK-E17-04 — Avaliar e integrar xHE-AAC com capacidade honesta

Versão 0.7.0 · Estado: planejada, não implementada.

## Contrato

Requisito REQ-E17-04 · M3 · P0 · responsável funcional audio (pessoa a designar). Esforço provisório 2–5 dias úteis por contrato; reestimar e quebrar em slices de 1–3 dias antes execução. Risco alto; estimativa não é prazo.

## Dependências

- [TASK-E17-01](TASK-E17-01.md) — Fixar catálogo por binding, capacidades e corpus
- [TASK-E01-07](TASK-E01-07.md) — Adicionar codecs modernos e capítulos

## Entrega concreta

Adapter USAC/AOT42 quando viável, ou relatório de indisponibilidade por build/licença.

[Spec canônica](../../reference/audio/06-codec-capability-registry.md) · [Matriz normativa](../../product/03-requisitos.md).

## Aceite

Fixtures DRC/seek/trim por alvo ou motivo unsupported auditado; sem fallback LC nem declaração universal; spike M0 registrado.

## Verificação e evidência

QA-025, QA-026 em [QA](../../quality/01-estrategia-testes.md) e [cenários](../../_arquivo/repo/docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, fixtures/hashes/licenças, OS/ABI/hardware, esperado/observado e limitações. Documento/schema não aprova app. Aplicar Superpowers, owner/worktree exclusivos, TDD de comportamento e revisão do outro agente. Atualizar JSON/MD/REQ e capabilities juntos; somente aceitar com evidência.
