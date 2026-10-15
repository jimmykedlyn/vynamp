# TASK-E17-11 — Implementar política de proxy e rede isolada

Versão 0.7.0 · Estado: planejada, não implementada.

## Contrato

Requisito REQ-E17-11 · M2 · P0 · responsável funcional streaming (pessoa a designar). Esforço provisório 2–5 dias úteis por contrato; reestimar e quebrar em slices de 1–3 dias antes execução. Risco alto; estimativa não é prazo.

## Dependências

- [TASK-E05-01](TASK-E05-01.md) — Implementar SourceDescriptor e provider simulado

## Entrega concreta

Adapters system/manual/direct, vault, CONNECT/TLS e fixtures 407/DNS/redirect.

[Spec canônica](../../docs/streaming/05-network-proxy-and-containers.md) · [Matriz normativa](../../docs/product/03-requirements.md).

## Aceite

Proxy failure não cai direto; credenciais origin/proxy/provider separadas; redirects/filhos respeitam política; cancel/reconnect bounded.

## Verificação e evidência

QA-029 em [QA](../../docs/quality/01-test-strategy.md) e [cenários](../../docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, fixtures/hashes/licenças, OS/ABI/hardware, esperado/observado e limitações. Documento/schema não aprova app. Aplicar Superpowers, owner/worktree exclusivos, TDD de comportamento e revisão do outro agente. Atualizar JSON/MD/REQ e capabilities juntos; somente aceitar com evidência.
