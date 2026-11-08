# TASK-E17-05 — Certificar lossless, híbridos e PCM especiais

Versão 0.7.0 · Estado: planejada, não implementada.

## Contrato

Requisito REQ-E17-05 · M3 · P0 · responsável funcional audio (pessoa a designar). Esforço provisório 2–5 dias úteis por contrato; reestimar e quebrar em slices de 1–3 dias antes execução. Risco alto; estimativa não é prazo.

## Dependências

- [TASK-E01-08](TASK-E01-08.md) — Completar codecs desktop e large files

## Entrega concreta

Corpus WavPack/WVC/float, APE/TTA/MPC/WMA e PCM de containers adicionais.

[Spec canônica](../../reference/audio/06-codec-capability-registry.md) · [Matriz normativa](../../product/03-requisitos.md).

## Aceite

Lossless inteiro sample-exact; WV sem correção não recebe selo lossless; níveis/versões/endianness/validbits/u64 são verificados.

## Verificação e evidência

QA-025 em [QA](../../quality/01-estrategia-testes.md) e [cenários](../../_arquivo/repo/docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, fixtures/hashes/licenças, OS/ABI/hardware, esperado/observado e limitações. Documento/schema não aprova app. Aplicar Superpowers, owner/worktree exclusivos, TDD de comportamento e revisão do outro agente. Atualizar JSON/MD/REQ e capabilities juntos; somente aceitar com evidência.
