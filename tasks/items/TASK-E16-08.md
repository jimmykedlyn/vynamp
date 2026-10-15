# TASK-E16-08 — Validar portable, backup e limpeza de integração

Versão 0.7.0 · Estado: planejada, não implementada.

## Contrato

Requisito REQ-E16-08 · M3 · P0 · responsável funcional delivery (pessoa a designar). Esforço provisório 2–5 dias úteis por contrato; reestimar e quebrar em slices de 1–3 dias antes execução. Risco alto; estimativa não é prazo.

## Dependências

- [TASK-E16-02](TASK-E16-02.md) — Implementar owner único por perfil e IPC autorizado
- [TASK-E15-06](TASK-E15-06.md) — Validar NSIS/portable e assinatura Windows
- [TASK-E15-07](TASK-E15-07.md) — Validar AppImage/DEB e dependências Linux
- [TASK-E15-08](TASK-E15-08.md) — Validar DMG/ZIP Universal2 e notarização

## Entrega concreta

Backup/restore consistente, portable relocável e uninstall de registros próprios.

[Spec canônica](../../docs/platforms/04-activation-ownership.md) · [Matriz normativa](../../docs/product/03-requirements.md).

## Aceite

Restore por staging e rollback preserva mídia; credenciais não exportadas; mover portable não toma defaults; update respeita owner e playback.

## Verificação e evidência

QA-024, QA-031 em [QA](../../docs/quality/01-test-strategy.md) e [cenários](../../docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, fixtures/hashes/licenças, OS/ABI/hardware, esperado/observado e limitações. Documento/schema não aprova app. Aplicar Superpowers, owner/worktree exclusivos, TDD de comportamento e revisão do outro agente. Atualizar JSON/MD/REQ e capabilities juntos; somente aceitar com evidência.
