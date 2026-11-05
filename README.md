# Vynamp — especificação de produto e engenharia

Versão: 0.7.0 · Data: 2026-10-14 · Estado: player especificado; automação Rust implementada e validada localmente.

**Nome escolhido pelo usuário:** Vynamp. Identidade aprovada: C — Pulso. Registro de marca/domínio não foi verificado.
Player multiplataforma de áudio com núcleo Rust (Windows, Linux, macOS Intel/Apple Silicon e Android), independente, com estética clássica inspirada no Winamp, profundidade de organização do AIMP e controle de áudio inspirado no Poweramp. Temas, presets e plugins terão formatos próprios. Streaming é parte do produto em todas as plataformas; a plataforma web provedora é uma entrega posterior.

Este pacote define o produto completo em fases. Não contém código do Winamp, assets de terceiros, player implementado, testes executados do player ou garantia de paridade integral. As metas de desempenho e qualidade são critérios futuros de aceite, não resultados medidos.

## Ordem de leitura

1. [Contrato mestre](SPEC.md), [índice completo](docs/INDEX.md), [escopo](docs/product/01-vision-scope.md) e [matriz de requisitos](docs/product/03-requirements.md).
2. [Plataformas obrigatórias](docs/platforms/01-support-matrix.md), [Android](docs/platforms/02-android.md), [Design](docs/design/01-visual-system.md), [telas](docs/design/02-layouts.md) e [referência visual](assets/layout-reference.html).
3. [Áudio](docs/audio/01-engine.md), [formatos](docs/audio/02-formats.md), [streaming](docs/streaming/01-client.md).
4. [Arquitetura](docs/architecture/01-system.md), [dados](docs/architecture/03-data.md), [contratos](docs/architecture/04-contracts.md).
5. [Roadmap](docs/delivery/01-roadmap.md), [backlog](tasks/README.md) e [próxima execução](tasks/NEXT.md).

## Como usar

- MUST / DEVE: obrigatório na fase indicada; SHOULD / RECOMENDADO: exceção justificada por ADR; MAY / OPCIONAL: capacidade adicional.
- P0: caminho crítico da fase; P1: necessário para completar a fase; P2: expansão planejada. Prioridade não é ordem cronológica.
- M0: provas técnicas; M1: alpha local; M2: beta multiplataforma; M3: versão1.0 nos cinco alvos; M4: plataforma; M5: expansão. A visão completa inclui todas as fases, mas a versão 1.0 não promete entregas M4/M5.
- Requisitos `REQ-*`, tarefas `TASK-*`, decisões `ADR-*` e cenários `QA-*` são identificadores estáveis.
- [Registro de decisões e hipóteses](docs/product/05-decisions.md) explicita escolhas feitas para permitir execução sem bloquear em perguntas.

## Entregáveis auxiliares

[Tokens visuais](assets/design-tokens.json), [exemplos de contratos](schemas/README.md), [tarefas em JSON](tasks/backlog.json), [validação documental](xtask/src/validate.rs), [validação documental](VALIDATION.json), [histórico](CHANGELOG.md), [política de contribuições](CONTRIBUTING.md), [segurança](SECURITY.md), [instruções de execução](AGENTS.md).

Validar: `cargo xtask check-docs`. Abrir `assets/layout-reference.html` no navegador para conferir os layouts clássico, biblioteca e compacto. A referência é estática, com assets originais e dados fictícios.

Preparação de engenharia 0.7.0, com automação Rust implementada: [START-HERE](START-HERE.md), [estrutura](docs/engineering/08-folder-structure.md), [agentes/Superpowers](docs/engineering/01-agents-superpowers.md), [Gitflow](docs/engineering/03-gitflow-github.md), [CI/CD](docs/engineering/04-ci-cd.md), [testes](docs/engineering/05-tests.md), [instaladores](docs/engineering/06-installers.md). Repositório: `jimmykedlyn/vynamp-mediaplayer`. Hoje **privado**; a abertura está decidida na [ADR-014](docs/architecture/adr/ADR-014-public-github-actions-and-in-app-dsp.md) e é executada pela TASK-E19-06.

Identidade aprovada: [guia Vynamp C / Pulso](docs/design/04-brand-identity.md) e [prancha incluída](assets/branding/approved-c-board.png). Exports nativos ainda pendentes, conforme manifest.

## Revisão 0.6

[Auditoria de cobertura](docs/product/08-completeness-audit.md): 30 novas tarefas, 68 entradas de catálogo e QA-022–031. Formatos condicionais exigem licença/backend/hardware conforme perfil. Nenhum suporte de aplicativo ou ícone nativo final é anunciado por documentação.

## Reinício Windows 0.7.0

ZIP completo em raiz `vynamp-media-player`, fontes em `pacote` e checkout em `repo`. Executar INICIAR.ps1 externo uma vez e os atalhos dos dois agentes. [Guia completo](docs/engineering/09-windows-restart.md). Correções Windows e claims/worktrees estão consolidadas; nada de projeto precisa ficar fora de D:\Projects\vynamp-media-player.
