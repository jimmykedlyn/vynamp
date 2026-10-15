# Completeness Audit Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [x]`) syntax for tracking.

**Goal:** Fechar lacunas documentais e manter cobertura de produto rastreável.
**Architecture:** Contratos/docs/catálogos separados de capacidades runtime; schemas versionados; adapters por OS e backend por codec/perfil. Execução inline da edição documental, com auditorias independentes somente leitura.
**Tech Stack:** Markdown/JSON/JSON Schema Draft2020-12/Rust xtask; fontes oficiais OS/codecs/benchmark.
**Spec:** [desenho](../specs/2026-10-14-completeness-audit-design.md).

## Global Constraints

Nome Vynamp e C/Pulso; Rust tooling e cinco alvos; marca não altera layout; native exports/app/release false. Sem compatibilidade Winamp, DRM bypass ou universal codec claim. Não auto-associar arquivos/defaults. Fases M0–M5 preservadas. Alterar TASK exige backlog/Markdown/REQ sincronizados. Sem push/merge/publicação.

## Review Focus

- Associação não pode anunciar codec de catálogo não verificado no build.
- Arquivo começando '-' e URI com metacaracteres não executam shell nem viram flags.
- Novo ganho/mix/resampling não ultrapassa a proteção true-peak final.
- Múltiplos processos não escrevem o mesmo perfil; stale lock não apaga dados.
- Catálogo diferencia imersivo/metadados/BT/DRM e não substitui decoder ou renderer.

### Task 1: Auditoria e contrato OS

**Files:** docs/platforms/03-system-integration.md, 04-activation-ownership.md, schemas/open-request.schema.json/examples, sources/audit.
- [x] Pesquisar fontes primárias; mapear achados e decisões com fases.
- [x] Definir registro/default/uninstall, open/enqueue/batch/grants/IPC/portable e QA por alvo.

### Task 2: Formatos/capacidades, DSP e visualizers

**Files:** docs/audio/02-formats.md, 06-codec-capability-registry.md, config/format-catalog.json, schemas/codec-capability.schema.json/examples, audio engine/DSP/outputs, extensions/visualizers.
- [x] Separar profiles/containers/decode/encode/BT e atualizar catálogo com restrições.
- [x] Corrigir cadeia/master limiter e especificar limites/meter/testes.
- [x] Fechar visualizer capability/latency/render/UX e proxy/network contracts.

### Task 3: Rastreabilidade e validação

**Files:** docs/product/08-completeness-audit.md, tasks/backlog/epics/items, requirements/QA/index, config/stage/CHANGELOG, xtask validators/tests.
- [x] Criar tarefas enxutas para contratos novos e testes relevantes de tooling se necessários.
- [x] Harmonizar distribuição e reforçar gates sem ciclos/dependências futuras.
- [x] Validar docs/schemas/DAG/catálogos, revisar e empacotar ZIP0.6.0 com SHA.

## Resultado de execução

Contrato/documentação concluídos, revisão independente de três subagentes Codex e achados corrigidos; nenhuma revisão Claude declarada. Checks finais e artefato em docs/evidence/completeness-audit.json e log correspondente. Etapas assinaladas referem-se à preparação documental, não à implementação das TASKs. ZIP final só entregue após validação e conferência de hashes.
