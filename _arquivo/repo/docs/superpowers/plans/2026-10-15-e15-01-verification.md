# E15-01 — integração e evidência QA-009

> Execução inline pelo Codex, conforme pedido explícito de incorporar `125db63`, repetir doctor/ci docs e seguir a E15-01. Skills: using-superpowers, using-git-worktrees, writing-plans e verification-before-completion. Revisão cruzada pela sessão Claude existente.

**Objetivo:** integrar a correção Windows dos managers e registrar resultados reais desta worktree para TASK-E15-01 / REQ-E15-01 / QA-009.

**Spec:** [agentes e Superpowers](../../engineering/01-agents-superpowers.md) e [TASK-E15-01](../../../tasks/items/TASK-E15-01.md).

**Arquivos:** correções de `xtask/src/{util,agents,validate}.rs`, lock e spec vindas do commit; logs e relatório em `docs/evidence/task-e15-01-*`; links no índice e na tarefa.

## Restrições

Uma worktree/branch/index por owner; nenhuma edição na worktree Claude. Não alterar perfis globais, reinstalar plugins, desativar hooks ou fabricar discovery. Preservar flags de aplicação/release. Merge no destino de integração e aceite final cabem ao mantenedor após revisão Claude e CI Gate.

## Passos e oráculos

- [x] Conferir isolamento e árvore limpa; incorporar `125db63` com `git cherry-pick -x`. Resultado: `479ae30`.
- [x] Executar `cargo xtask doctor --agent both` e preservar resultados: exit 1 no sandbox; transcrição posterior fornecida pelo usuário confirma os dois managers no PowerShell normal.
- [x] Executar `cargo xtask ci docs` e `cargo test --package xtask --locked`; testes 29 + 23 passaram. O agregado falhou no cache do sandbox; a transcrição posterior do usuário mostra o agregado completando com as validações e os 52 testes aprovados no PowerShell normal.
- [x] Preservar ambiente/ABI, SHA do código, comandos, logs, hashes e limitações no relatório QA-009; atualizar links e repetir validação documental após as edições.
- [x] Consultar PR remoto: HTTP 401; preparar evidência para commit e handoff da branch ao Claude, sem alegar publicação.
- [ ] Concluir commit da evidência: tentativa bloqueada no pre-commit, ao criar `target/xtask-hooks` (`Acesso negado`, exit 1). Evidência staged, hooks preservados.
- [x] Registrar doctor/ci docs aprovados no terminal normal segundo transcrição do usuário recebida em 2026-10-16; distinguir proveniência e ausência de exit codes/SHA explícitos nesse transcript.
- [ ] Obter discovery Claude, revisão cruzada real e CI Gate antes do aceite/integração pelo mantenedor.

## Foco de revisão

Confirmar identidade/versão ativa por manager; rejeitar projeto alheio; distinguir instalação de discovery; não promover resultados históricos a resultados atuais; não aprovar plugins do player, hardware ou release por checks de tooling.

## Registro de execução

Os targets existentes recusaram escrita nesta sessão Windows (`os error 5`). Um cache novo em `.cache/e15-01-verification-20261015`, dentro da raiz autorizada, permitiu compilar o driver e executar os 52 testes sem editar ACLs. O `ci docs` seleciona seu próprio `target/xtask-ci` e falhou ao abrir `.cargo-build-lock`. A tentativa de preservar o cache com `Move-Item` também foi negada; cache e ACLs permaneceram intactos. Os componentes restantes são verificados separadamente em cache gravável, sem declarar o comando agregado aprovado.

Ruling inicial: preservar parser e pins de `125db63` enquanto só havia listas vazias/falha de rede — esses sintomas não são um novo formato a aceitar. Atualização de 2026-10-16: o pin Claude passa a `7.0.0` após evidência real de nova instalação, sem relaxar o parser; o pin Codex permanece `6.4.2`.

Sem `CODEX_HOME`, o CLI filho lista `installed: []`. Com `CODEX_HOME=C:/Users/jimmy/.codex`, lista plugins locais, mas a consulta ao catálogo remoto falha e Superpowers não consta. Discovery das 15 skills Superpowers está presente no catálogo desta conversa; não foi tratado como aprovação do doctor nem prova de hooks/da sessão Claude.

A consulta de PR por `gh` retornou HTTP 401; publicação e CI remoto não estão confirmados. Revisão Claude e aceite final permanecem pendentes. Resultados definitivos, hashes e limites ficam no [relatório QA-009](../../evidence/task-e15-01-qa-009.json).

O commit `docs(e15-01): record QA-009 verification and sandbox limits` foi tentado e falhou antes da criação do commit: o pre-commit fixa seu driver em `target/xtask-hooks`, onde a criação do cache foi negada. Não desativar hooks nem declarar SHA de evidência inexistente; o índice da branch Codex preserva os arquivos preparados para revisão/commit no ambiente compatível.

Atualização recebida em 2026-10-16: o usuário executou `AMBIENTE.ps1`, `cargo xtask doctor --agent both` e `cargo xtask ci docs` na mesma worktree, em PowerShell normal. A saída confirma ambos managers, validação documental/configuração, clippy e 29 + 23 testes aprovados. [Excertos do transcript fornecido](../../evidence/task-e15-01-user-terminal-20261016.log) preservam a origem; não foram capturados exit codes nem SHA/digest pelo terminal do usuário. O SHA/digest do código são conferidos separadamente pelo Codex. Essa evidência complementa os logs do sandbox, sem apagar as falhas anteriores. O doctor continua exigindo confirmar discovery na sessão Claude; revisão cruzada, commit da evidência e CI remoto permanecem pendentes.

Revisão Claude registrada em 2026-10-16: código de `479ae30` aprovado tecnicamente; discovery na sessão Claude confirmado para Superpowers `7.0.0`. O aceite ficou bloqueado pelo pin `6.4.1`. O Codex reproduziu a falha do doctor Claude com esse pin (exit 1), capturou o manager real e atualizou somente o pin Claude para `7.0.0`. Sugestões de baixa prioridade sobre precedência project/user e extensões PATHEXT ficam registradas no guia, sem alteração de comportamento nesta correção. Revisão anterior e suas limitações não cobrem o novo lock nem as evidências/instruções ainda não commitadas; o novo SHA deverá ser reavaliado.
