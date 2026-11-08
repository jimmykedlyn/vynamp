# TASK-E16-08 — Validar portable, backup e limpeza de integração

Versão 0.8.0 · Estado: planejada, não implementada (conforme [ADR-012](../../adr/ADR-012-integracao-shell.md)).

## Contrato

Requisito REQ-E16-08 · M3 · P0 · responsável funcional delivery (pessoa a designar). Esforço provisório 2–5 dias úteis por contrato; reestimar e quebrar em slices de 1–3 dias antes execução. Risco alto; estimativa não é prazo.

## Dependências

- [TASK-E16-02](TASK-E16-02.md) — Implementar owner único por perfil e IPC autorizado
- [TASK-E15-06](TASK-E15-06.md) — Validar NSIS/portable e assinatura Windows
- [TASK-E15-07](TASK-E15-07.md) — Validar AppImage/DEB e dependências Linux
- [TASK-E15-08](TASK-E15-08.md) — Validar DMG/ZIP Universal2 e notarização

## Entrega concreta

Backup/restore consistente, portable relocável e uninstall de registros próprios (ProgIDs e pacote esparso). Detalhe: cobre o backup e restore da base de dados e configurações, a relocação da pasta no modo portátil e a desinstalação completa de registros próprios (ProgIDs HKCU e pacote esparso MSIX no Windows; desktop entries, ServiceMenus e MIME no Linux).

[Spec canônica](../../platforms/04-ativacao-ownership.md) · [Spec de integração](../../platforms/03-integracao-sistema.md) · [Matriz normativa](../../product/03-requisitos.md) · [ADR-012](../../adr/ADR-012-integracao-shell.md).

## Aceite

Restore por staging e rollback preserva mídia; desinstalação remove pacote esparso e HKCU sem sobras; portable permite ativar/desativar integração com confirmação; update respeita owner. Detalhe: restore por staging e rollback preserva arquivos de mídia do usuário, credenciais não são exportadas, mover a instalação portátil não assume defaults sem confirmação, o desinstalador oficial NSIS remove o pacote esparso e ProgIDs sem deixar chaves órfãs e updates respeitam o owner e playback ativo.

## Verificação e evidência

QA-024, QA-031 em [QA](../../quality/01-estrategia-testes.md) e [cenários](../../_arquivo/repo/docs/quality/04-integration-codecs-dsp.md). Registrar commit, comando, fixtures/hashes/licenças, OS/ABI/hardware, esperado/observado e limitações. Documento/schema não aprova app. Aplicar Superpowers, owner/worktree exclusivos, TDD de comportamento e revisão do outro agente. Atualizar JSON/MD/REQ e capabilities juntos; somente aceitar com evidência.
