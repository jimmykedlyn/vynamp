# TASK-E16-04 — Entregar MIME e abertura externa no Linux

Versão 0.7.0 · Estado: planejada, não implementada.

## Contrato

Requisito REQ-E16-04 · M2 · P0 · responsável funcional platform (pessoa a designar). Esforço provisório 2–5 dias úteis por contrato; reestimar e quebrar em slices de 1–3 dias antes execução. Risco alto; estimativa não é prazo.

## Dependências

- [TASK-E16-02](TASK-E16-02.md) — Implementar owner único por perfil e IPC autorizado
- [TASK-E16-09](TASK-E16-09.md) — Gerar declarações OS a partir do build verificado
- [TASK-E14-02](TASK-E14-02.md) — Entregar ports de arquivos, credenciais e sistema

## Entrega concreta

Desktop Entry, DEB/AppImage, MIME, portais e MPRIS ligados ao mesmo owner.

[Spec canônica](../../docs/platforms/03-system-integration.md) · [Matriz normativa](../../docs/product/03-requirements.md).

## Aceite

X11/Wayland e Nautilus/Dolphin abrem arquivos concedidos; %U não usa shell; defaults preservados; não alegar menu contextual universal.

## Verificação e evidência

QA-022, QA-023, QA-031 em [QA](../../docs/quality/01-test-strategy.md) e [cenários](../../docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, fixtures/hashes/licenças, OS/ABI/hardware, esperado/observado e limitações. Documento/schema não aprova app. Aplicar Superpowers, owner/worktree exclusivos, TDD de comportamento e revisão do outro agente. Atualizar JSON/MD/REQ e capabilities juntos; somente aceitar com evidência.
