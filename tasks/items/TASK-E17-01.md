# TASK-E17-01 — Fixar catálogo por binding, capacidades e corpus

Versão 0.7.0 · Estado: planejada, não implementada.

## Contrato

Requisito REQ-E17-01 · M0 · P0 · responsável funcional audio (pessoa a designar). Esforço provisório 2–5 dias úteis por contrato; reestimar e quebrar em slices de 1–3 dias antes execução. Risco alto; estimativa não é prazo.

## Dependências

- [TASK-E11-01](TASK-E11-01.md) — Criar workspace e baseline de engenharia
- [TASK-E14-01](TASK-E14-01.md) — Provar os cinco targets e features portáveis

## Entrega concreta

Catálogo versionado, spikes de backends/licenças e modelo de capability por alvo. Spike M0 fixa algoritmo/oversampling/latência do guard true-peak e estimador independente com fixtures.

[Spec canônica](../../docs/audio/06-codec-capability-registry.md) · [Matriz normativa](../../docs/product/03-requirements.md).

## Aceite

Cada perfil tem fase/política/operação; codecs NGA têm decisão de viabilidade; não inferir HE/USAC por AAC-LC nem renderer por decoder. Prova true-peak e pin do oráculo M0 rastreados; implementação permanece M3.

## Verificação e evidência

QA-025, QA-026, QA-030 em [QA](../../docs/quality/01-test-strategy.md) e [cenários](../../docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, fixtures/hashes/licenças, OS/ABI/hardware, esperado/observado e limitações. Documento/schema não aprova app. Aplicar Superpowers, owner/worktree exclusivos, TDD de comportamento e revisão do outro agente. Atualizar JSON/MD/REQ e capabilities juntos; somente aceitar com evidência.
