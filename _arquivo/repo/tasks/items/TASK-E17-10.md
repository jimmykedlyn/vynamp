# TASK-E17-10 — Certificar visualizers, fallback e energia mobile

Versão 0.7.1 · Data: 2026-11-03 · Estado: planejada, não implementada.

## Contrato

Requisito REQ-E17-10 · M3 · P0 · responsável funcional qa (pessoa a designar). Esforço provisório 2–5 dias úteis por contrato; reestimar e quebrar em slices de 1–3 dias antes execução. Risco alto; estimativa não é prazo.

## Dependências

- [TASK-E17-09](TASK-E17-09.md) — Entregar presets e taps limitados de visualização
- [TASK-E13-05](TASK-E13-05.md) — Entregar interface phone e navegação acessível
- [TASK-E14-04](TASK-E14-04.md) — Executar corpus e outputs por alvo

Referência visual: vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452 (rev.38), caminhos `mvp/...` e `out/...` conforme [referência do MVP](../../design/07-pin-mvp.md). Caminhos relevantes: `out/svg/meters/` e `out/svg/meters/dynamic.json`.

## Entrega concreta

Testes CPU/GPU/device-loss, acessibilidade, fullscreen e background Android.

[Spec canônica](../../extensions/05-visualizador.md) · [Matriz normativa](../../product/03-requisitos.md).

## Aceite

GPU perdida mantém áudio; zero rendering Android screen-off/background; reduced motion, foco e FPS/budgets medidos.

## Verificação e evidência

QA-028 em [QA](../../quality/01-estrategia-testes.md) e [cenários](../../_arquivo/repo/docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, fixtures/hashes/licenças, OS/ABI/hardware, esperado/observado e limitações. Documento/schema não aprova app. Aplicar Superpowers, owner/worktree exclusivos, TDD de comportamento e revisão do outro agente. Atualizar JSON/MD/REQ e capabilities juntos; somente aceitar com evidência.
