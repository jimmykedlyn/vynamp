# TASK-E19-04 — Re-apontar as specs de Restore e Converter e criar os repos próprios

Versão 0.8.0 · Estado planejado; criar repositório vazio não equivale a ferramenta entregue.

Fase M3 / P0 · Owner engineering-delivery · Esforço inicial 2–6 dias úteis; reestimar.

## Dependências

- [TASK-E18-01](TASK-E18-01.md) — Entregar Vynamp Restore como ferramenta desktop de restauração
- [TASK-E18-02](TASK-E18-02.md) — Entregar Vynamp Converter e a lib de encoders

## Entrega concreta

Specs de Restore e Converter re-apontadas para repositórios próprios, com a fonte normativa mantida no repo principal e os repos vynamp-encode, vynamp-converter e vynamp-restore criados públicos.

Decisão do mantenedor (2026-11-05): *"vynamp converter e restore devem ter seus próprios repos"*. A **fonte normativa** das duas specs continua no repo principal — é onde a matriz de requisitos, os QA e as evidências vivem — e cada repo novo aponta para ela por URL e SHA. Duplicar a spec criaria divergência silenciosa entre dois arquivos que nenhum validador reconcilia. Os repos novos nascem como esqueletos **completos o bastante para não parecer quebrados**: README bilíngue honesto, LICENSE dual, SECURITY.md, CONTRIBUTING.md, CODEOWNERS e `.github/workflows/ci.yml` funcional.

Fronteira de conteúdo: `vynamp-encode` é a **biblioteca** de codificadores, única fonte de encoders do produto; `vynamp-converter` é o **aplicativo desktop** (CLI, fila de jobs, workers, relatório, escrita atômica), que consome `vynamp-decode` e `vynamp-encode`; `vynamp-restore` é o **aplicativo desktop** de restauração (CLI, motor, diagnóstico, presets, comparação, exportação), que consome `vynamp-decode`. Nenhum dos três contém o player nem o DSP do player.

Escopo: `docs/tools/01-vynamp-restore.md` e `docs/tools/02-vynamp-converter.md` (cabeçalho de origem), `docs/architecture/adr/ADR-012-shell-integration.md` §3, `docs/architecture/adr/ADR-013-vynamp-restore.md`, `docs/GLOSSARY.md`, `tasks/epics/E18.md`, `docs/engineering/08-folder-structure.md` (layout `D:\Projects\vynamp-{encode,converter,restore}`); criação dos três repositórios GitHub e dos clones locais.

Boundary: não implementa as ferramentas; não move a fonte normativa para fora do repo principal; não abre os repositórios ao público — a abertura é TASK-E19-06.

## Critérios de aceite

As specs apontam para URLs reais dos repos novos; a fonte normativa continua única no repo principal; os três repos existem, públicos, com README bilíngue, LICENSE e SECURITY.md.

Detalhamento: `docs/tools/01-vynamp-restore.md` cita `https://github.com/jimmykedlyn/vynamp-restore`; `docs/tools/02-vynamp-converter.md` cita `https://github.com/jimmykedlyn/vynamp-converter` e `https://github.com/jimmykedlyn/vynamp-encode`; `gh repo view jimmykedlyn/vynamp-restore --json visibility` retorna `PUBLIC`; cada repos novo tem `README.md` bilíngue, `LICENSE`, `LICENSE-MIT`, `LICENSE-APACHE`, `SECURITY.md` e `CONTRIBUTING.md`; `D:\Projects\vynamp-restore`, `D:\Projects\vynamp-converter` e `D:\Projects\vynamp-encode` existem com clone do remoto.

[Spec](../../docs/tools/02-vynamp-converter.md) · REQ-E19-04 em [matriz](../../docs/product/03-requirements.md).

## Evidência

QA-022, QA-030; registrar commit/digest, comando, ambiente/OS/ABI, outputs e limitações. Quebrar em slices pequenas antes execução, implementar sucesso/erro/cancelamento, revisar pelo outro agente e preservar resultados no PR/docs/evidence.
