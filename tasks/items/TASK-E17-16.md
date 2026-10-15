# TASK-E17-16 — Certificar áudio cinematográfico e passthrough

Versão 0.7.0 · Estado: planejada, não implementada.

## Contrato

Requisito REQ-E17-16 · M5 · P1 · responsável funcional audio (pessoa a designar). Esforço provisório 2–5 dias úteis por contrato; reestimar e quebrar em slices de 1–3 dias antes execução. Risco alto; estimativa não é prazo.

## Dependências

- [TASK-E17-03](TASK-E17-03.md) — Completar bindings de containers e seleção de áudio
- [TASK-E04-10](TASK-E04-10.md) — Validar qualidade numérica e hardware matrix

## Entrega concreta

AC3/EAC3/TrueHD/MLP/DTS Core/HD, downmix e saída codificada condicionada.

[Spec canônica](../../docs/audio/06-codec-capability-registry.md) · [Matriz normativa](../../docs/product/03-requirements.md).

## Aceite

Atmos/DTS:X não anunciados como renderer genérico; receiver/rota/licença e DSP incompatível comprovados ou recusados.

## Verificação e evidência

QA-026 em [QA](../../docs/quality/01-test-strategy.md) e [cenários](../../docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, fixtures/hashes/licenças, OS/ABI/hardware, esperado/observado e limitações. Documento/schema não aprova app. Aplicar Superpowers, owner/worktree exclusivos, TDD de comportamento e revisão do outro agente. Atualizar JSON/MD/REQ e capabilities juntos; somente aceitar com evidência.
