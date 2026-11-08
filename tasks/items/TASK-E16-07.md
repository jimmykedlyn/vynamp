# TASK-E16-07 — Certificar ativação e associações em cinco alvos

Versão 0.7.0 · Estado: planejada, não implementada.

## Contrato

Requisito REQ-E16-07 · M3 · P0 · responsável funcional qa (pessoa a designar). Esforço provisório 2–5 dias úteis por contrato; reestimar e quebrar em slices de 1–3 dias antes execução. Risco alto; estimativa não é prazo.

## Dependências

- [TASK-E16-03](TASK-E16-03.md) — Entregar associação e Abrir com no Windows
- [TASK-E16-04](TASK-E16-04.md) — Entregar MIME e abertura externa no Linux
- [TASK-E16-05](TASK-E16-05.md) — Entregar document types e Finder nos dois Macs
- [TASK-E16-06](TASK-E16-06.md) — Entregar VIEW e Share seguros no Android

## Entrega concreta

Relatórios por OS/ABI de instalação, defaults, lote, erros e concorrência.

[Spec canônica](../../platforms/03-integracao-sistema.md) · [Matriz normativa](../../product/03-requisitos.md).

## Aceite

QA-022/023/024 passam nos cenários aplicáveis M1–M3 dos cinco alvos; deep links/OAuth só em M4; terceiros preservados; ausência de hardware não vira aprovação.

## Verificação e evidência

QA-022, QA-023, QA-024 em [QA](../../quality/01-estrategia-testes.md) e [cenários](../../_arquivo/repo/docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, fixtures/hashes/licenças, OS/ABI/hardware, esperado/observado e limitações. Documento/schema não aprova app. Aplicar Superpowers, owner/worktree exclusivos, TDD de comportamento e revisão do outro agente. Atualizar JSON/MD/REQ e capabilities juntos; somente aceitar com evidência.
