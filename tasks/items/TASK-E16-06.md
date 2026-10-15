# TASK-E16-06 — Entregar VIEW e Share seguros no Android

Versão 0.7.0 · Estado: planejada, não implementada.

## Contrato

Requisito REQ-E16-06 · M2 · P0 · responsável funcional platform (pessoa a designar). Esforço provisório 2–5 dias úteis por contrato; reestimar e quebrar em slices de 1–3 dias antes execução. Risco alto; estimativa não é prazo.

## Dependências

- [TASK-E16-01](TASK-E16-01.md) — Provar envelope de ativação e protocolo comum
- [TASK-E16-09](TASK-E16-09.md) — Gerar declarações OS a partir do build verificado
- [TASK-E13-04](TASK-E13-04.md) — Entregar MediaStore, SAF e permissões
- [TASK-E13-05](TASK-E13-05.md) — Entregar interface phone e navegação acessível

## Entrega concreta

Manifest específico, ContentResolver/FD e encaminhamento ao serviço de sessão.

[Spec canônica](../../docs/platforms/03-system-integration.md) · [Matriz normativa](../../docs/product/03-requirements.md).

## Aceite

VIEW/SEND/MULTIPLE e onNewIntent preservam grants/ordem; rotation não duplica; Share pede ação; sem persistência de grant não oferecido.

## Verificação e evidência

QA-022, QA-023 em [QA](../../docs/quality/01-test-strategy.md) e [cenários](../../docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, fixtures/hashes/licenças, OS/ABI/hardware, esperado/observado e limitações. Documento/schema não aprova app. Aplicar Superpowers, owner/worktree exclusivos, TDD de comportamento e revisão do outro agente. Atualizar JSON/MD/REQ e capabilities juntos; somente aceitar com evidência.
