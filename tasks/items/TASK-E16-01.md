# TASK-E16-01 — Provar envelope de ativação e protocolo comum

Versão 0.7.0 · Estado: planejada, não implementada.

## Contrato

Requisito REQ-E16-01 · M0 · P0 · responsável funcional platform (pessoa a designar). Esforço provisório 2–5 dias úteis por contrato; reestimar e quebrar em slices de 1–3 dias antes execução. Risco alto; estimativa não é prazo.

## Dependências

- [TASK-E11-01](TASK-E11-01.md) — Criar workspace e baseline de engenharia

## Entrega concreta

Contrato OpenRequest, limites e spikes de entrega nativa em cinco alvos.

[Spec canônica](../../docs/platforms/04-activation-ownership.md) · [Matriz normativa](../../docs/product/03-requirements.md).

## Aceite

Cold/warm e lotes 1/2/20/100 preservam ordem; nomes especiais são dados; limites e ACK/generation definidos.

## Verificação e evidência

QA-023 em [QA](../../docs/quality/01-test-strategy.md) e [cenários](../../docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, fixtures/hashes/licenças, OS/ABI/hardware, esperado/observado e limitações. Documento/schema não aprova app. Aplicar Superpowers, owner/worktree exclusivos, TDD de comportamento e revisão do outro agente. Atualizar JSON/MD/REQ e capabilities juntos; somente aceitar com evidência.
