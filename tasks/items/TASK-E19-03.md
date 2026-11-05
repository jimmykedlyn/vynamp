# TASK-E19-03 — Dissolver o vitrola no DSP interno do app

Versão 0.8.0 · Estado planejado; dissolver o repositório não equivale a implementar os efeitos.

Fase M3 / P0 · Owner audio-dsp · Esforço inicial 1–3 dias úteis; reestimar.

## Dependências

- [TASK-E04-03](TASK-E04-03.md) — Construir grafo DSP e transições sem glitches

## Entrega concreta

Design aprovado do vitrola incorporado como módulo DSP interno, referências ao repositório externo removidas e repositório vitrola arquivado de forma reversível.

Decisão do mantenedor (2026-11-05): *"os Tube e Vinil são recursos do DSP dentro da aplicação, não são libs e nem repositórios externos"*. O design aprovado (77 linhas, 9 seções, redigido em 2026-10-23) é preservado por **incorporação** — não por reimplementação — dentro de `docs/audio/10-vitrola.md`. As referências que apontavam para `jimmykedlyn/vitrola` passam a apontar para o módulo interno. O repositório remoto é **arquivado**, que é reversível (`gh repo unarchive`); o clone local só é removido depois que o design estiver no repositório principal, porque ele é a única cópia.

Escopo: `docs/audio/10-vitrola.md` (§1 reescrita + seção "Design aprovado"), `docs/architecture/adr/ADR-011-own-audio-stack.md`, `docs/GLOSSARY.md`, `docs/audio/09-dsp-catalog.md`, `docs/design/06-mvp-reference.md`, `docs/audio/07-dsp-signal-contract.md`, `docs/audio/03-dsp.md`, `docs/security/02-privacy-licensing.md`, `docs/product/06-feature-parity.md`, `docs/delivery/03-risks-open-questions.md`; `docs/evidence/PR/task-e19-03/vitrola-design.md` com o conteúdo local; `gh repo archive jimmykedlyn/vitrola`.

Boundary: não implementa os efeitos (a integração é TASK-E19-07); não **deleta** o repositório `vitrola` — arquiva, preservando histórico e SHAs citados; não reescreve histórico.

## Critérios de aceite

Nenhum documento normativo cita jimmykedlyn/vitrola; a spec descreve módulo DSP interno preservando VIT-T-01..10; o design local não se perde; o repositório remoto fica arquivado e reversível.

Detalhamento: `grep -rn "jimmykedlyn/vitrola" docs/ tasks/ config/ AGENTS.md` retorna vazio e `docs/audio/10-vitrola.md` descreve o vinil e a válvula como módulos DSP internos do app, preservando a tabela VIT-T-01..10; `docs/evidence/PR/task-e19-03/vitrola-design.md` contém o design aprovado integral; `gh repo view jimmykedlyn/vitrola --json isArchived` retorna `true`.

[Spec](../../docs/audio/10-vitrola.md) · REQ-E19-03 em [matriz](../../docs/product/03-requirements.md).

## Evidência

QA-033, QA-005; registrar commit/digest, comando, ambiente/OS/ABI, outputs e limitações. Quebrar em slices pequenas antes execução, implementar sucesso/erro/cancelamento, revisar pelo outro agente e preservar resultados no PR/docs/evidence.
