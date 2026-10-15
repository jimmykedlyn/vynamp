# Vynamp Branding Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [x]`) syntax for tracking.

**Goal:** Incluir a identidade C — Pulso no pacote e atualizar as specs/nomes técnicos para Vynamp.
**Architecture:** PNGs de referência preservados sem edição; guia/tokens/manifest de identidade separado dos tokens de UI. Renomear contratos e receitas ainda não lançados; GitHub atualizado conforme renomeação informada pelo usuário.
**Tech Stack:** Rust xtask, Markdown, JSON, PNG, templates nativos.
**Spec:** Conceito C escolhido pelo usuário e pedido de incluir no ZIP; [guia de marca](../../design/04-brand-identity.md).

## Global Constraints

Vynamp é o nome escolhido. Opção C — Pulso, V com play, âmbar #FFBF69/grafite #11131B/branco #F0F2F5. Windows/Linux/macOS Intel/macOS Silicon/Android obrigatórios. Repo jimmykedlyn/vynamp-mediaplayer informado renomeado, sem inferir autenticação/privacidade real. Não editar imagens geradas nem fingir SVG/ICO/ICNS/adaptive nativos já exportados. Preservar paleta/layout Classic Graphite e flags de app/release false. Claims Git legados devem continuar protegidos.

## Review Focus

- ZIP com root vynamp-mediaplayer e imagens C presentes; hashes conferidos.
- Toda URL/config ativa aponta repo novo; evidências antigas preservadas como históricas.
- Marca diferente de tokens semânticos: lime/cyan permanecem para playback/foco.
- Mesmo ícone Mac Intel/Silicon; Android layers/monochrome/safe zone, sem máscaras pré-fixadas.
- Claims legados não podem permitir dono duplicado após renomeação.

### Task 1: Identidade e documentação

**Files:** assets/branding/{README,brand-tokens,asset-manifest}, PNGs, docs/design/04-brand-identity.md, ADR-008, specs/roadmap/tasks/index.
- [x] Copiar PNGs originais sem edição e registrar SHA/dimensões/estado de referência.
- [x] Documentar marca, variações, regras/tamanhos por plataforma e critérios de exportação/teste reais.

### Task 2: Nomes técnicos e gates

**Files:** config, schemas, previews SVG/HTML, packaging, workflows, xtask/src, xtask/tests/tooling.rs.
- [x] Testar nome raiz do ZIP e inclusão dos assets; observar falha antes de ajustar.
- [x] Testar recusa de claim legado; migrar namespace com leitura de claims antigos preservados.
- [x] Renomear Vynamp/vynamp/VYNAMP, extensões próprias e repo; atualizar pins/lockfile real.
- [x] Impedir empacotar marcas/ícones nativos não exportados; preservar gated release.

### Task 3: Verificação e entrega

**Files:** VALIDATION.json, ARTIFACTS.json, docs/evidence/vynamp-branding.json, ZIP/SHA.
- [x] Regenerar previews textuais, índice; executar ci docs/testes e conferir sem resíduos ativos indevidos.
- [x] ZIP/inventário corretos, sem caches/credenciais; informar que exports nativos/hardware são gates futuros.

## Resultado e limitações

Executado em cópia isolada de distribuição sem .git. Nome/conceito escolhidos pelo usuário; 3 PNGs originais incluídos sem edição. Flags de app/release/branding false. 31 testes de tooling passaram no Linux, além de fmt/clippy/spec/schema/config checks. Revisão independente corrigiu migração de templates antigos, containers nativos vazios, inventário/dimensões e papéis Retina. Sem GitHub remoto/Claude review/app/assinatura/hardware ou native exports gerados. Evidência em docs/evidence/vynamp-branding.json.
