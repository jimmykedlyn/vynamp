# TASK-E16-09 — Gerar declarações OS a partir do build verificado

Versão 0.7.0 · Estado: planejada, não implementada.

## Contrato

Requisito REQ-E16-09 · M2 · P0 · responsável funcional delivery (pessoa a designar). Esforço provisório 2–5 dias úteis por contrato; reestimar e quebrar em slices de 1–3 dias antes execução. Risco alto; estimativa não é prazo.

## Dependências

- [TASK-E17-01](TASK-E17-01.md) — Fixar catálogo por binding, capacidades e corpus
- [TASK-E16-01](TASK-E16-01.md) — Provar envelope de ativação e protocolo comum

## Entrega concreta

Gerador de handlers por container/parser e capabilities reais de cada artefato.

[Spec canônica](../../delivery/04-gates.md) · [Matriz normativa](../../product/03-requisitos.md).

## Aceite

planned/experimental/sem backend ou licença não gera handler; MIME genérico/video/BT não é associado; manifest e pacote têm mesmos hashes.

## Verificação e evidência

QA-030 em [QA](../../quality/01-estrategia-testes.md) e [cenários](../../_arquivo/repo/docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, fixtures/hashes/licenças, OS/ABI/hardware, esperado/observado e limitações. Documento/schema não aprova app. Aplicar Superpowers, owner/worktree exclusivos, TDD de comportamento e revisão do outro agente. Atualizar JSON/MD/REQ e capabilities juntos; somente aceitar com evidência.
