# TASK-E17-19 — Expor capacidades Bluetooth e headroom por rota

Versão 0.7.0 · Estado: planejada, não implementada.

## Contrato

Requisito REQ-E17-19 · M3 · P0 · responsável funcional audio (pessoa a designar). Esforço provisório 2–5 dias úteis por contrato; reestimar e quebrar em slices de 1–3 dias antes execução. Risco alto; estimativa não é prazo.

## Dependências

- [TASK-E13-03](TASK-E13-03.md) — Entregar audio focus e rotas Android
- [TASK-E14-02](TASK-E14-02.md) — Entregar ports de arquivos, credenciais e sistema
- [TASK-E04-08](TASK-E04-08.md) — Entregar tempo, pitch e efeitos criativos

## Entrega concreta

Diagnóstico de rota OS, codec negociado quando exposto e volume/headroom por dispositivo.

[Spec canônica](../../docs/audio/06-codec-capability-registry.md) · [Matriz normativa](../../docs/product/03-requirements.md).

## Aceite

SBC/AAC/aptX/LDAC/LC3 não geram associações; API ausente mostra desconhecido; troca de rota não aumenta volume nem garante hi-res.

## Verificação e evidência

QA-025, QA-027 em [QA](../../docs/quality/01-test-strategy.md) e [cenários](../../docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, fixtures/hashes/licenças, OS/ABI/hardware, esperado/observado e limitações. Documento/schema não aprova app. Aplicar Superpowers, owner/worktree exclusivos, TDD de comportamento e revisão do outro agente. Atualizar JSON/MD/REQ e capabilities juntos; somente aceitar com evidência.
