# TASK-E17-09 — Entregar presets e taps limitados de visualização

Versão 0.7.1 · Data: 2026-11-03 · Estado: planejada, não implementada.

## Contrato

Requisito REQ-E17-09 · M3 · P0 · responsável funcional design (pessoa a designar). Esforço provisório 2–5 dias úteis por contrato; reestimar e quebrar em slices de 1–3 dias antes execução. Risco alto; estimativa não é prazo.

## Dependências

- [TASK-E06-06](TASK-E06-06.md) — Entregar visualizações avançadas
- [TASK-E07-01](TASK-E07-01.md) — Provar sandbox WASM e capabilities

Referência visual: vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452 (rev.38), caminhos `mvp/...` e `out/...` conforme [referência do MVP](../../docs/design/06-mvp-reference.md). Caminhos relevantes: `out/svg/meters/` e `out/svg/meters/dynamic.json`.

## Entrega concreta

Scenes declarativas, ring/taps e famílias FFT/phase/spectrogram/particles.

[Spec canônica](../../docs/extensions/04-visualizer-contract.md) · [Matriz normativa](../../docs/product/03-requirements.md).

## Aceite

Preset inválido/código arbitrário rejeitado; orçamento agregado e overflow descartam visuais; nunca bloqueiam callback/clock.

## Verificação e evidência

QA-028 em [QA](../../docs/quality/01-test-strategy.md) e [cenários](../../docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, fixtures/hashes/licenças, OS/ABI/hardware, esperado/observado e limitações. Documento/schema não aprova app. Aplicar Superpowers, owner/worktree exclusivos, TDD de comportamento e revisão do outro agente. Atualizar JSON/MD/REQ e capabilities juntos; somente aceitar com evidência.
