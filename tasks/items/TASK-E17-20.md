# TASK-E17-20 — Fechar gate combinado de codecs, DSP e visualizers

Versão 0.7.0 · Estado: planejada, não implementada.

## Contrato

Requisito REQ-E17-20 · M3 · P0 · responsável funcional qa (pessoa a designar). Esforço provisório 2–5 dias úteis por contrato; reestimar e quebrar em slices de 1–3 dias antes execução. Risco alto; estimativa não é prazo.

## Dependências

- [TASK-E17-02](TASK-E17-02.md) — Implementar trim, gain e timeline por codec
- [TASK-E17-03](TASK-E17-03.md) — Completar bindings de containers e seleção de áudio
- [TASK-E17-04](TASK-E17-04.md) — Avaliar e integrar xHE-AAC com capacidade honesta
- [TASK-E17-05](TASK-E17-05.md) — Certificar lossless, híbridos e PCM especiais
- [TASK-E17-06](TASK-E17-06.md) — Certificar tags técnicas, capítulos e sidecars
- [TASK-E17-08](TASK-E17-08.md) — Medir teto no PCM final com oráculo independente
- [TASK-E17-10](TASK-E17-10.md) — Certificar visualizers, fallback e energia mobile
- [TASK-E17-11](TASK-E17-11.md) — Implementar política de proxy e rede isolada
- [TASK-E17-12](TASK-E17-12.md) — Publicar manifest real de capacidades por artefato
- [TASK-E17-19](TASK-E17-19.md) — Expor capacidades Bluetooth e headroom por rota
- [TASK-E16-07](TASK-E16-07.md) — Certificar ativação e associações em cinco alvos

## Entrega concreta

Evidência por build/alvo do corpus comum, condicionais, cadeia final e visualização.

[Spec canônica](../../docs/delivery/04-capability-release-gates.md) · [Matriz normativa](../../docs/product/03-requirements.md).

## Aceite

Cinco alvos passam required M3; conditional reportado honestamente; nenhum gate depende de features M4/M5; QA e limites publicados.

## Verificação e evidência

QA-025, QA-026, QA-027, QA-028, QA-029, QA-030 em [QA](../../docs/quality/01-test-strategy.md) e [cenários](../../docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, fixtures/hashes/licenças, OS/ABI/hardware, esperado/observado e limitações. Documento/schema não aprova app. Aplicar Superpowers, owner/worktree exclusivos, TDD de comportamento e revisão do outro agente. Atualizar JSON/MD/REQ e capabilities juntos; somente aceitar com evidência.
