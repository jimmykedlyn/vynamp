# TASK-E17-12 — Publicar manifest real de capacidades por artefato

Versão 0.7.0 · Estado: planejada, não implementada.

## Contrato

Requisito REQ-E17-12 · M3 · P0 · responsável funcional delivery (pessoa a designar). Esforço provisório 2–5 dias úteis por contrato; reestimar e quebrar em slices de 1–3 dias antes execução. Risco alto; estimativa não é prazo.

## Dependências

- [TASK-E17-01](TASK-E17-01.md) — Fixar catálogo por binding, capacidades e corpus
- [TASK-E16-09](TASK-E16-09.md) — Gerar declarações OS a partir do build verificado
- [TASK-E15-04](TASK-E15-04.md) — Ativar workflows e CI Gate multiplataforma

## Entrega concreta

Manifest gerado com backend/features/licenças/hashes/evidência e validação do pacote.

[Spec canônica](../../docs/delivery/04-capability-release-gates.md) · [Matriz normativa](../../docs/product/03-requirements.md).

## Aceite

Catálogo planned não é disponibilidade; required comum comprovado nos cinco alvos; adapter ausente não gera associação nem encode.

## Verificação e evidência

QA-030 em [QA](../../docs/quality/01-test-strategy.md) e [cenários](../../docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, fixtures/hashes/licenças, OS/ABI/hardware, esperado/observado e limitações. Documento/schema não aprova app. Aplicar Superpowers, owner/worktree exclusivos, TDD de comportamento e revisão do outro agente. Atualizar JSON/MD/REQ e capabilities juntos; somente aceitar com evidência.
