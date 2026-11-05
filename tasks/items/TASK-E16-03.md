# TASK-E16-03 — Entregar associação, menu de contexto e pacotes de ícones no Windows

Versão 0.8.0 · Estado: planejada, não implementada (conforme [ADR-012](../../docs/architecture/adr/ADR-012-shell-integration.md)).

## Contrato

Requisito REQ-E16-03 · M2 · P0 · responsável funcional platform (pessoa a designar). Esforço provisório 2–5 dias úteis por contrato; reestimar e quebrar em slices de 1–3 dias antes execução. Risco alto; estimativa não é prazo.

## Dependências

- [TASK-E16-02](TASK-E16-02.md) — Implementar owner único por perfil e IPC autorizado
- [TASK-E16-09](TASK-E16-09.md) — Gerar declarações OS a partir do build verificado
- [TASK-E08-06](TASK-E08-06.md) — Integrar controles do Windows

## Entrega concreta

Registro HKCU, pacote esparso MSIX com IExplorerCommand, verbos completos e pacotes de ícones .ico multi-res. Detalhe: o pacote esparso assinado habilita o menu moderno do Windows 11 (`IExplorerCommand`), o submenu clássico cobre o Windows 10 e o fallback "Mostrar mais opções", os verbos canônicos são Reproduzir, Abrir em nova lista, Adicionar à lista, Adicionar à fila, Editar etiquetas, Converter e Restaurar com Vynamp Restore, o lote chega atomicamente via `IShellItemArray` e os `.ico` multi-resolução (16 a 256px) cobrem as 3 famílias (Documento, Disco e Sinal) nos 17 formatos.

[Spec canônica](../../docs/platforms/03-system-integration.md) · [Matriz normativa](../../docs/product/03-requirements.md) · [ADR-012](../../docs/architecture/adr/ADR-012-shell-integration.md).

## Aceite

Explorer moderno (Win11) e clássico (Win10) abrem lote ordenado por IShellItemArray; pacote esparso registrado via PackageManager; remoção limpa; suporte às 3 famílias de ícones. Detalhe: o lote atômico evita processos múltiplos, o `UserChoice` não é alterado, a instalação é em escopo de usuário (`$LOCALAPPDATA\Programs\Vynamp`), a desinstalação remove ProgIDs e pacote esparso sem sobras, o modo portátil permite associação explícita e a troca de família de ícones reflete no Explorer.

## Verificação e evidência

QA-022, QA-023, QA-031 em [QA](../../docs/quality/01-test-strategy.md) e [cenários](../../docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, fixtures/hashes/licenças, OS/ABI/hardware, esperado/observado e limitações. Documento/schema não aprova app. Aplicar Superpowers, owner/worktree exclusivos, TDD de comportamento e revisão do outro agente. Atualizar JSON/MD/REQ e capabilities juntos; somente aceitar com evidência.
