# Contrato mestre do produto

Versão 0.7.0 · 2026-10-14 · Nome escolhido pelo usuário: Vynamp.

Este documento fixa a leitura normativa do pacote. A spec está distribuída por domínio para permitir execução em tarefas e evitar um único Markdown impossível de manter. O [índice completo](INDEX.md) lista todos os arquivos; o [README](README.md) explica o uso.

## Prioridade dos documentos

1. Direcionamento do usuário: Rust, produto independente, temas/plugins próprios, visual clássico inspirado, funcionalidades amplas, hi-res, streaming cliente e plataforma futura.
2. [Visão e fronteiras](product/01-visao-escopo.md), [decisões](product/05-decisoes.md) e [matriz normativa](product/03-requisitos.md).
3. Specs canônicas de cada domínio; [tokens](assets/design-tokens.json) são autoridade para cores/medidas compartilhadas.
4. [Catálogo de cobertura](product/06-paridade.md), tarefas, exemplos e previews. Exemplos mínimos não reduzem requisitos do produto completo.

Contradições são corrigidas por ADR/alteração da matriz; não escolher silenciosamente o documento mais conveniente. A fase determina a obrigação de entrega. API e WIT são propostas a validar/congelar, não garantias de disponibilidade.

## Produto e aparência

Player multiplataforma com modos desktop e mobile. Desktop possui três modos: Classic Stack, Library Workspace e Mini. Classic Graphite é padrão, com módulos de player/EQ/playlist, gabinete grafite e display verde-lima. Temas Midnight Modern e Copper completam a primeira coleção original. Layout, DPI, estados, foco, fontes e cores estão em [design](design/01-sistema-visual.md) e [geometria](design/02-layouts.md); [preview](assets/layout-reference.html) e [PNG clássico](assets/classic-preview.png) materializam a direção.

Não há compatibilidade com Winamp DLLs, skins, scripts ou IPC. Não reutilizar código, logo, sprites ou fontes proprietárias. As medidas e cores desta spec são criação própria inspirada na experiência clássica, sem alegação de cópia pixel a pixel de skin de terceiros.

## Áudio e dados

Engine Rust com I/O/decode em workers e callback restrita. Gapless, clocks, cancellation generations, DSP, formatos e outputs têm critérios independentes. Modo transparente tem condições e caminho inteiro específicos; “bit-perfect verificado” exige prova do conjunto. Biblioteca local, fila ativa e playlists são entidades distintas, com backup/migration e jobs canceláveis.

## Rede e extensões

Cliente toca local e rádio sem conta. HTTP/ICY, HLS, podcast, downloads e gravação respeitam limites e restrições. Provider API conecta plataforma posterior sem acoplar UI/engine. Temas são declarativos; plugins usam capacidades, WASM ou helper isolado. SDK próprio tem schema/version/budgets e safe mode.

## Fases e cobertura

```mermaid
flowchart LR
  M0["M0: provas e decisões"] --> M1["M1: alpha local"]
  M1 --> M2["M2: biblioteca, rádio, temas"]
  M2 --> M3["M3: versão 1.0 multiplataforma"]
  M3 --> M4["M4: plataforma e sync"]
  M3 --> M5["M5: expansão áudio e arquiteturas adicionais"]
  M4 --> X["Integrações e delivery avançado"]
  M5 --> X
```

M3/1.0 exige Windows/Linux/macOS Intel+Silicon/Android, UI adequada a cada host, advanced DSP, hi-res negociado, HLS e utilitários aplicáveis por capability. M4 inclui ingest/catalog/live/VOD/auth/rights/sync/registry. M5 inclui ARM adicional Windows/Linux, DSD nativo, 768 kHz, formatos raros, CD, VST3, DASH/cloud e casting. Os requisitos dessas fases têm tarefas explícitas; não estão omitidos nem prometidos na alpha.

## Aceite e execução

191 requisitos ligados a 191 tarefas em 19 épicos. Cada tarefa tem dependências e aceite; itens grandes são agrupadores e devem ser decompostos em slices de 1–3 dias antes execução, conforme descoberta M0. Faixas são esforço provisório, não calendário.

Começar por [NEXT](tasks/NEXT.md). Aceitar tarefas apenas com evidência numérica, integração, UI, hardware ou segurança apropriada. O relatório documental (`VALIDATION.json`, gerado por `cargo xtask ci docs` e não versionado) só valida consistência de arquivos/contratos. Não existem player, testes de áudio executados ou servidor funcionando neste pacote.

Plataformas são obrigatórias no mesmo release1.0: [matriz](platforms/01-matriz-suporte.md), [host Android](platforms/02-android.md), [design phone/tablet](design/05-mobile-tablet.md) e [QA por alvo](quality/03-matriz-multiplataforma.md). Android UI/serviço proposta usa Kotlin/Compose e núcleoRust; Mac Intel+Silicon exigem execução nativa em ambos.

Preparação de engenharia0.7.0: [estrutura](engineering/08-reinicio-windows.md), [agentes/Superpowers](_arquivo/repo/docs/engineering/01-agents-superpowers.md), [Gitflow](engineering/03-gitflow.md), [CI/CD](engineering/04-ci.md), [testes](engineering/05-testes.md), [instaladores](_arquivo/repo/docs/engineering/06-installers.md). GitHub privado escolhido: jimmykedlyn/vynamp-mediaplayer; acesso remoto ainda não confirmado no ambiente.

## Identidade Vynamp

Nome Vynamp e conceito C — Pulso aprovados. [Guia de marca](design/04-marca.md) e [PNGs incluídos](assets/branding/README.md). Preserve geometria e cores da marca e paleta semântica da UI. Não usar PNG de referência como ícone nativo já verificado. Repo informado renomeado: jimmykedlyn/vynamp-mediaplayer.

## Auditoria de completude 0.6

[catálogo de 68 bindings](config/format-catalog.json), [ativação/associações](platforms/03-integracao-sistema.md), [ownership](platforms/04-ativacao-ownership.md), [sinal final](reference/audio/07-dsp-signal-contract.md), [visualizers](extensions/05-visualizador.md) e [novos gates](delivery/04-gates.md). São 191 TASK/REQ em 19 épicos; nenhum item de aplicativo está concluído por este pacote.

## Ferramentas de arquivo 0.8

[Vynamp Restore](reference/tools/01-vynamp-restore.md) e [Vynamp Converter](reference/tools/02-vynamp-converter.md) são executáveis desktop separados sobre as mesmas libs do player ([ADR-012](adr/ADR-012-integracao-shell.md) §3, [ADR-013](adr/ADR-013-restore.md)), com épico próprio [E18](tasks/epics/E18.md). O Restore entrega diagnóstico de defeitos, restauração e exportação não destrutiva; o Converter entrega transcodificação em lote sobre a lib própria `vynamp-encode`. Nenhuma das duas foi implementada por este pacote.

## Bootstrap consolidado 0.7

[Guia de desenvolvimento](engineering/08-reinicio-windows.md): desde 2026-11-11 tudo vive no umbrella D:\Projects\vynamp, com libs/apps/reference como submódulos; o bootstrap 0.7 em D:\Projects\vynamp-media-player foi aposentado e está em _arquivo/media-player. Codecs/áudio/UI e as 191 tarefas permanecem planejados. Não altera o nome do repositório GitHub nem os contratos funcionais.
