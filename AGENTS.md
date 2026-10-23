# Contrato de trabalho do agente

Versão 0.7.0 · 2026-10-14 · Configuração do projeto; player ainda não implementado.

Fluxo de agente único: Claude Code é o único autor e revisor. Subagentes: Opus orquestra e trata alto risco; Sonnet implementa e revisa; Haiku executa checks. Merge, tag e release pertencem ao mantenedor humano.

## Antes da tarefa

Superpowers é obrigatório. Ler START-HERE, SPEC, docs/engineering, TASK/REQ/QA e ADRs; instalar/confirmar o native plugin e a skill discovery. Configurações não equivalem a plugin instalado. Não alegar usar skill inexistente nem fabricar testes/assinaturas.

## Trabalho

Um owner por TASK, worktree/branch/index próprios. `.worktrees/claude/<task>-<slug>` é o padrão do script; worktree nativa existente é reutilizada. Root/main/develop são para sincronização, não feature edits. Claims no common Git dir evitam sessões concorrentes na mesma TASK. Nunca reset --hard, clean -fdx ou force push.

Superpowers: using-superpowers → spec/brainstorming se decisão nova → writing-plans → using-git-worktrees → TDD para comportamento → verification-before-completion → revisão técnica por subagente em contexto limpo (Sonnet; Opus para alto risco) → finishing-a-development-branch. Planos em docs/superpowers/plans; evidência permanente em docs/evidence/PR; scratch .superpowers ignored. Decisões já dadas pelo usuário não requerem aprovação repetida. Escopo explícito/autorizações do usuário prevalecem sobre skills.

Branch feature/fix/chore por TASK de develop; hotfix de main; PR e merge commit. main/develop protegidas, CI Gate (Woodpecker, ADR-010) obrigatório, merge/publicação pelo mantenedor. PR template documenta fonte/task/command/result/limitações e a revisão técnica.

## Qualidade

Executar cargo xtask ci docs e checks do subsistema. Não testar só espelho de implementação. RT callback: sem alloc/lock/I/O/log/plugin/JNI. Baselines/pins resolvidos M0, sem Cargo.lock inventado. Windows/Linux/MacIntel/MacSilicon/Android têm gates reais. Emulator/cross-build não aprova hardware; dados/segredos/assinaturas protegidos. Temas próprios, sem código/assets Winamp.

## Entrega

Ao entregar: branch, SHA completo, base, PR real (ou motivo da pendência), escopo commitado, comandos/resultados/ambiente/limitações. Publicar a branch sem force push e criar/atualizar o PR por head+base (template e --body-file), sem duplicar. Entregar ao mantenedor a URL real do PR, head revisado, status de revisão/CI e o pedido “aprove e faça o merge commit pelo GitHub”. Não inventar SHA/PR/CI. Falha de autenticação/rede é registrada com o erro real. Após o merge, verificar o PR real (state=MERGED), `git fetch origin` e sincronizar a integração por fast-forward, preservando worktrees alheias.

Produto ainda não implementado; stages/release flags distinguem tooling de app. Não criar executável placeholder, selo hi-res falso ou installer fictício. Release precisa licenças/notices/fixtures/evidências/runtime inventory/certificados reais. Não criar licença final sem decisão registrada. Não publicar plataforma web antes de M4. Atualizar docs/schema/backlog/ADR/notices conforme mudança, mantendo IDs e links.

## Identidade Vynamp

Nome Vynamp e conceito C — Pulso aprovados. [Guia de marca](docs/design/04-brand-identity.md) e [PNGs incluídos](assets/branding/README.md). Preserve geometria e cores da marca e paleta semântica da UI. Não usar PNG de referência como ícone nativo já verificado. Repo informado renomeado: jimmykedlyn/vynamp-mediaplayer.
