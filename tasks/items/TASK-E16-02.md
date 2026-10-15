# TASK-E16-02 — Implementar owner único por perfil e IPC autorizado

Versão 0.7.0 · Estado: planejada, não implementada.

## Contrato

Requisito REQ-E16-02 · M1 · P0 · responsável funcional platform (pessoa a designar). Esforço provisório 2–5 dias úteis por contrato; reestimar e quebrar em slices de 1–3 dias antes execução. Risco alto; estimativa não é prazo.

## Dependências

- [TASK-E16-01](TASK-E16-01.md) — Provar envelope de ativação e protocolo comum
- [TASK-E01-03](TASK-E01-03.md) — Implementar state machine e generation cancellation

## Entrega concreta

Lock OS, encaminhamento IPC, idempotência e isolamento de perfis.

[Spec canônica](../../docs/platforms/04-activation-ownership.md) · [Matriz normativa](../../docs/product/03-requirements.md).

## Aceite

Vinte launches elegem um owner; installed/portable não escrevem o mesmo perfil simultaneamente; stale PID não libera lock; ACK e Conflict testados.

## Verificação e evidência

QA-023, QA-024 em [QA](../../docs/quality/01-test-strategy.md) e [cenários](../../docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, fixtures/hashes/licenças, OS/ABI/hardware, esperado/observado e limitações. Documento/schema não aprova app. Aplicar Superpowers, owner/worktree exclusivos, TDD de comportamento e revisão do outro agente. Atualizar JSON/MD/REQ e capabilities juntos; somente aceitar com evidência.
