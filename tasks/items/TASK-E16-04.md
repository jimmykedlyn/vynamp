# TASK-E16-04 — Entregar MIME, extensões de shell e abertura externa no Linux

Versão 0.8.0 · Estado: planejada, não implementada (conforme [ADR-012](../../docs/architecture/adr/ADR-012-shell-integration.md)).

## Contrato

Requisito REQ-E16-04 · M2 · P0 · responsável funcional platform (pessoa a designar). Esforço provisório 2–5 dias úteis por contrato; reestimar e quebrar em slices de 1–3 dias antes execução. Risco alto; estimativa não é prazo.

## Dependências

- [TASK-E16-02](TASK-E16-02.md) — Implementar owner único por perfil e IPC autorizado
- [TASK-E16-09](TASK-E16-09.md) — Gerar declarações OS a partir do build verificado
- [TASK-E14-02](TASK-E14-02.md) — Entregar ports de arquivos, credenciais e sistema

## Entrega concreta

Desktop Entry com Desktop Actions, extensões/ServiceMenus (Dolphin, Nautilus, Thunar), MIME hicolor e MPRIS. Detalhe: as Desktop Actions cobrem NewPlaylist, AddPlaylist, Enqueue, Convert e Restore; as extensões de shell são o ServiceMenu KIO para Dolphin, o plugin Python/C para Nautilus e as Custom Actions para Thunar/Nemo; o registro MIME usa `shared-mime-info`, os ícones são instalados na hierarquia `hicolor` por família e o MPRIS fica ligado ao mesmo owner.

[Spec canônica](../../docs/platforms/03-system-integration.md) · [Matriz normativa](../../docs/product/03-requirements.md) · [ADR-012](../../docs/architecture/adr/ADR-012-shell-integration.md).

## Aceite

Gerenciadores (Dolphin, Nautilus, Thunar) exibem submenu Vynamp com verbos canônicos; %U repassa lote sem shell; ícones hicolor por família instalados. Detalhe: X11 e Wayland abrem arquivos concedidos, `%U` preserva o lote completo sem invocar shell, `mimeapps.list` e os defaults escolhidos pelo usuário são preservados, os ícones de MIME são atualizados via `gtk-update-icon-cache` e as ferramentas Restore e Converter operam a partir do menu.

## Verificação e evidência

QA-022, QA-023, QA-031 em [QA](../../docs/quality/01-test-strategy.md) e [cenários](../../docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, fixtures/hashes/licenças, OS/ABI/hardware, esperado/observado e limitações. Documento/schema não aprova app. Aplicar Superpowers, owner/worktree exclusivos, TDD de comportamento e revisão do outro agente. Atualizar JSON/MD/REQ e capabilities juntos; somente aceitar com evidência.
