# Contrato comum de trabalho dos agentes

Versão 0.7.0 · 2026-10-14 · Configuração do projeto; player ainda não implementado.

## Antes da tarefa

Superpowers é obrigatório para Codex e Claude Code. Ler START-HERE, SPEC, docs/engineering, TASK/REQ/QA e ADRs; instalar/confirmar native plugin em cada agente e skill discovery. Configurações não equivalem a plugin instalado. Não alegar usar skill inexistente nem fabricar testes/assinaturas.

## Trabalho

Um owner porTASK, worktree/branch/index próprios. `.worktrees/<agent>/<task>-<slug>` é padrão do script; worktree nativa existente é reutilizada. Root/main/develop são para bootstrap/sincronização, não feature edits. Claims no common Git dir para sessões locais; máquinas diferentes coordenam pelo PR/issue. Nunca editar worktree do outro agente, reset --hard, clean -fdx ou force push.

Superpowers: using-superpowers → spec/brainstorming se decisão nova → writing-plans → using-git-worktrees → TDD para comportamento → verification-before-completion → revisão técnica pelo outro agente → finishing-a-development-branch. Planos em docs/superpowers/plans; evidência permanente em docs/evidence/PR; scratch .superpowers ignored. Decisões já dadas pelo usuário não requerem aprovação repetida. Escopo explícito/autorizações do usuário prevalecem sobre skills.

Branch feature/fix/chore porTASK de develop; hotfix de main; PR e merge commit. main/develop protegidas, CI Gate obrigatório, merge/publicação pelo mantenedor. PR template documenta fonte/task/command/result/limitações e review do outro agente; duas ferramentas não são duas identidades GitHub.

## Qualidade

Executar cargo xtask ci docs e checks do subsistema. Não testar só espelho de implementação. RT callback: sem alloc/lock/I/O/log/plugin/JNI. Baselines/pins resolvidos M0, sem Cargo.lock inventado. Windows/Linux/MacIntel/MacSilicon/Android têm gates reais. Emulator/cross-build não aprova hardware; dados/segredos/assinaturas protegidos. Temas próprios, sem código/assets Winamp.

## Entrega

Produto ainda não implementado; stages/release flags distinguem tooling de app. Não criar executável placeholder, selo hi-res falso ou installer fictício. Release precisa licenças/notices/fixtures/evidências/runtime inventory/certificados reais. Não criar licença final sem decisão registrada. Não publicar plataformaweb antesM4. Atualizar docs/schema/backlog/ADR/notices conforme mudança, mantendo IDs e links.

## Identidade Vynamp

Nome Vynamp e conceito C — Pulso aprovados. [Guia de marca](docs/design/04-brand-identity.md) e [PNGs incluídos](assets/branding/README.md). Preserve geometria e cores da marca e paleta semântica da UI. Não usar PNG de referência como ícone nativo já verificado. Repo informado renomeado: jimmykedlyn/vynamp-mediaplayer.
