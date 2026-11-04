# Histórico da especificação

Versão: 0.7.2 · Data: 2026-11-03 · Estado: especificação proposta, implementação não iniciada.

## 0.1.0 — 2026-10-14

Especificação inicial de produto independente Rust; temas/plugins próprios; visual clássico original e tokens; funcionalidades locais/hi-res/DSP/streaming; plataforma futura; arquitetura/contratos/qualidade/segurança; tarefas rastreáveis e exemplos validados. Hipóteses e fases explicitadas. Não houve implementação do player ou uso de código do Winamp.

## 0.2.0 — 2026-10-14

Windows/Linux/macOS Intel+Silicon/Android obrigatórios desde provas até 1.0. Linux/macOS promovidos M5→M3 com IDs preservados; Android/core Rust+shell nativo, mobile/tablet/backends/QA/CI/distribuição adicionados. 16 novas tarefas/requisitos, épicos E13/E14, gates de release atualizados. Não houve implementação/teste de áudio no produto.

## 0.3.0 — 2026-10-14

Projeto raiz preparado para Codex/Claude com Superpowers obrigatório; worktree/claim/handoff/Gitflow/GitHub privado; workflows CI/release/hardware, tooling tests e recipes de installer/assinatura. 146 tarefas/15 épicos. Nenhum plugin foi instalado, repositório privado acessado/pushado, aplicativo ou instalador compilado nesta preparação.

## 0.4.0 — 2026-10-14

Automação migrada de Python para Rust/cargo xtask. Workspace, Rust 1.99.0 e Cargo.lock reais; Git/claims/doctor/importação, validadores, CI/hardware gates e receitas de assinatura/instaladores/releases. Hooks e workflows atualizados; CI distingue xtask de fontes reais do player. Package mantém release/application flags false. Execução local documentada em docs/evidence/rust-tooling.json; CI remoto/assinatura/player/hardware não executados nesta entrega.

## 0.5.0 — 2026-10-14

Nome Vynamp e identidade C — Pulso escolhidos pelo usuário. Inclui três PNGs originais de referência com SHA/dimensões, guia multiplataforma/brand tokens, ADR-008 e gates de export. Renomeia namespaces/contratos/templates/previews e ZIP, atualiza GitHub informado renomeado e preserva claims/evidências históricas. Não exporta/finge vetores/ícones nativos ou instaladores executados.

## 0.6.0 — 2026-10-14

Auditoria de completude: associações/Open With em cinco alvos, batch/IPC/grants, owner por perfil/portable/backup/deep links; catálogo de 68 bindings, USAC/imersivos e Bluetooth separado; trim/DRC/metadata; proteção true-peak após master/mix/resampler; contrato de visualizers e proxy. Dois novos épicos/30 tarefas, total 176 TASK/REQ em 17 épicos; QA-022–031 e quatro novos schemas. Corrige divergências de distribuição mantendo oito artefatos obrigatórios. Documentação e validação Rust; nenhuma implementação de player, export nativo ou publicação remota.

## 0.7.0 — 2026-10-14

Consolida correções Windows de caminhos verbatim/UNC, checks em cache distinto do executable e aliases de cache. Import evita colidir com branch de bootstrap remota existente. Novo prepare-agents preserva import branch/PR, cria Codex com claim e Claude detached/índice próprio; não depende de develop local ausente. ZIP em raiz única vynamp-media-player com pacote/repo, wrappers PowerShell, prompts automáticos e caches/tmp/logs locais. Mantém 176 TASK/REQ e 68 entradas planejadas, flags de app/release/exports false e evidências nativas ainda exigidas.

## Não lançado — 2026-10-23

Fluxo migrado para agente único: o Codex foi removido do tooling (`doctor`/`install` só para Claude, sem `prepare-agents` nem worktrees `.worktrees/codex`), do bootstrap, do lock do Superpowers e dos docs vivos. Claude Code é o único autor e revisor (Opus orquestra, Sonnet implementa e revisa, Haiku executa checks); merge e release permanecem com o mantenedor. ADR-007 recebeu nota de superação parcial. Evidências e planos históricos que citam o Codex foram preservados.

## Não lançado — 2026-10-25

Documentação DSP revisada para 0.7.1 nos arquivos tocados: visão funcional do DSP com diagrama Mermaid da cadeia real ([03-dsp](docs/audio/03-dsp.md)); subcapítulo obrigatório de equalizadores Gráfico 10/20/31/64 + Paramétrico até 32 filtros, com grades de frequência, Nyquist, modelo de dados, UI, MVP × nativo e testes ([08](docs/audio/08-equalizers.md)); catálogo de módulos ([09](docs/audio/09-dsp-catalog.md)); integração por referência do vitrola, com design aprovado em 2026-10-23 ([10](docs/audio/10-vitrola.md)); painel DSP/medidores, matriz de testes, QA-032/033, roadmap DSP, riscos, glossário e decisões D-DSP-01..10 com divergências. PEQ32 nativo segue M3; EQ-64 gráfico e PEQ64 nativos seguem M5; o MVP HTML/SVG/JS cobre agora a UI dos cinco modos. Só Markdown; nenhum código, schema, backlog ou pipeline alterado.

## Não lançado — 2026-10-28 (DSP v1.4)

Documentação DSP v1.4: painel Áudio e efeitos com acesso e controles DSP-UX-01..21 e integração dos VUs VU-INT-01..06 em [design/05-dsp-panel.md](docs/design/05-dsp-panel.md) (§6.1, §8, §9); critérios DSP-UX-AC-01..12 e testes do grafo/VU-INT em [quality/05-dsp-test-matrix.md](docs/quality/05-dsp-test-matrix.md) (§2); grafo modular × nativo em [audio/03-dsp.md](docs/audio/03-dsp.md); status da revisão 15 em [audio/08-equalizers.md](docs/audio/08-equalizers.md) (§14); decisões D-DSP-11..18 em [product/05-decisions.md](docs/product/05-decisions.md). O protótipo HTML/SVG/JS é externo: evidência relatada da referência externa rev. 14/15, não verificada neste repositório; nada nativo implementado. Só Markdown; nenhum código, schema, backlog ou pipeline alterado.

## Não lançado — 2026-10-28 (letras)

Letras: componente resumido sob o VU e fontes em [library/02-tags-lyrics-tools.md](docs/library/02-tags-lyrics-tools.md), com IDs LYR-01..15 e LYR-AC-01..12, layout em [design/02-layouts.md](docs/design/02-layouts.md), decisões D-LYR-01..09 em [product/05-decisions.md](docs/product/05-decisions.md) e critérios da tarefa em [TASK-E03-09](tasks/items/TASK-E03-09.md). A referência do protótipo é externa: evidência relatada da referência externa rev. 14, não verificada neste repositório; nada nativo implementado. Só Markdown; nenhum código, schema, backlog ou pipeline alterado (TASK-E03-09 foi atualizada apenas em texto: entrega, dependências e critérios).

## Não lançado — 2026-11-03 (referência do MVP)

MVP web rev.37 congelado por hash como especificação da UI nativa (egui) em [design/06-mvp-reference.md](docs/design/06-mvp-reference.md); pacote extraído (12 temas, 91 ícones, 16 medidores, SVGs renderizados) pelo repositório próprio `vynamp-mvp-reference`; QA-034 e TASK-E02-11..16 (fases de gabarito visual, movimento, comportamento, lacunas e harness egui_kittest). Verificação resvg × Chrome em [evidência](docs/evidence/PR/task-e02-11/README.md). Nada nativo implementado.

Pacote do MVP retirado deste repositório: nenhum arquivo do MVP fica aqui; ele e o pacote extraído vivem só em `vynamp-mvp-reference`.

## Não lançado — 2026-11-03 (pilha de áudio própria)

[ADR-011](docs/architecture/adr/ADR-011-own-audio-stack.md): decode, saída, efeitos e tags passam a vir das bibliotecas próprias vynamp-decode, vynamp-output e vitrola (SHAs fixos, estado API real × planejado e tabela de lacunas); o resampler será uma nova lib própria (nome a definir, D-DSP-08 decidido) e a escrita de tags estenderá `vynamp-decode-meta`. ADR-002 marcada como parcialmente substituída; specs de stack, formatos, saídas, backends, catálogo DSP, referências, licenças e TASK-E04-02 reescritas sem os nomes de bibliotecas de terceiros. Somente documentação.
