# Contrato de agente

## Visão geral

Este documento define o contrato de agente para o projeto Vynamp, incluindo requisitos, validação e critérios de aceitação. O agente deve seguir as diretrizes definidas neste documento e no `AGENTS.md`.

## Requisitos principais

- O agente deve ser configurado via `Claude Code` e confirmar plugin ativo e skill discovery.
- Configuração deve ser feita em máquina compatível (Windows, Linux, macOS).
- Configuração não substitui instalação (ex: `CLI` não é suficiente para uso).
- O agente deve validar especificações, ferramentas e arquitetura do projeto.
- Validação deve ser feita via `xtask` com `check-tasks`.
- O agente deve validar especificações de tasks, módulos e arquitetura.

## Validação

- Validação é feita via `xtask check-tasks`.
- Validação inclui verificação de:
  - Specs de tasks
  - Specs de módulos
  - Specs de arquitetura
  - Specs de agentes
  - Specs de plugins
  - Specs de CI/CD
  - Specs de segurança

## Critérios de aceitação

- Índices independentes
- Task exclusiva
- Paths válidos
- Worktree suja/unmerged preservada
- Hotfix só remove após main e develop

## Referências

- `AGENTS.md` - contrato de agente
- `tasks/backlog.json` - lista de tasks
- `xtask/src/validate.rs` - validador de specs
- `specs/` - diretório de specs (ex: `engineering/01-agents-superpowers.md`)

## Histórico

- **2026-11-23**: Migração de `_arquivo/` para `.arquivo/` e remoção do gate de release arquivado.
- **2026-11-23**: Atualização de specs para refletir estrutura consolidada.