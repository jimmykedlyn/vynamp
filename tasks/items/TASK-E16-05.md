# TASK-E16-05 — Entregar document types, Quick Actions e Finder nos dois Macs

Versão 0.8.0 · Estado: planejada, não implementada (conforme [ADR-012](../../adr/ADR-012-integracao-shell.md)).

## Contrato

Requisito REQ-E16-05 · M2 · P0 · responsável funcional platform (pessoa a designar). Esforço provisório 2–5 dias úteis por contrato; reestimar e quebrar em slices de 1–3 dias antes execução. Risco alto; estimativa não é prazo.

## Dependências

- [TASK-E16-02](TASK-E16-02.md) — Implementar owner único por perfil e IPC autorizado
- [TASK-E16-09](TASK-E16-09.md) — Gerar declarações OS a partir do build verificado
- [TASK-E14-02](TASK-E14-02.md) — Entregar ports de arquivos, credenciais e sistema

## Entrega concreta

UTTypes/document types, Quick Actions (NSServices) no Finder, callbacks LaunchServices e pacotes de ícones in-app. Detalhe: o `Info.plist` declara `CFBundleDocumentTypes`, os serviços do Finder (`NSServices`) expõem Reproduzir, Enfileirar, Converter e Restaurar, os callbacks de `NSApplicationDelegate` (`application(_:openFiles:)`) recebem o lote com grants e bootstrap seguro, e os pacotes de ícones são renderizados na interface interna (seletor, playlist e metadados).

[Spec canônica](../../platforms/03-integracao-sistema.md) · [Matriz normativa](../../product/03-requisitos.md) · [ADR-012](../../adr/ADR-012-integracao-shell.md).

## Aceite

Intel e Silicon recebem cold/warm batches via application(_:openFiles:); Quick Actions no Finder ativas; pacotes de ícones aplicados na UI interna; defaults preservados. Detalhe: cold e warm batches chegam sem perda nem duplicação, os serviços do Finder respondem para tipos suportados, os bookmarks de arquivo stale ou placeholder são canceláveis sem travamento e a seleção de família de ícones afeta a interface sem alterar os defaults nem invalidar a assinatura do bundle.

## Verificação e evidência

QA-022, QA-023, QA-031 em [QA](../../quality/01-estrategia-testes.md) e [cenários](../../_arquivo/repo/docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, fixtures/hashes/licenças, OS/ABI/hardware, esperado/observado e limitações. Documento/schema não aprova app. Aplicar Superpowers, owner/worktree exclusivos, TDD de comportamento e revisão do outro agente. Atualizar JSON/MD/REQ e capabilities juntos; somente aceitar com evidência.
