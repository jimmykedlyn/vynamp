# TASK-E15-02 — evidência do autor

Data: 2026-10-16. Autor Codex. Base existente: `389381cd6bec7442737bd632badb01de917215b2` (merge PR9 da dependência E15-01). Branch `chore/task-e15-02-codex-isolamento`; worktree `D:\Projects\vynamp-media-player\repo\.worktrees\codex\task-e15-02-isolamento`. O SHA de entrega/PR e estados posteriores são registrados no guia externo e no PR, sem auto-referência a um commit ainda inexistente neste snapshot.

## Ambiente e proveniência

Execução própria do autor em Windows x86_64/PowerShell 7, Rust 1.99.0/MSVC; `AMBIENTE.ps1` da raiz externa. Driver em `<worktree>\target\xtask-driver`, checks em `<worktree>\target\xtask-ci`; Cargo/TMP dentro da raiz do projeto. Fixtures são repositórios Git reais, temporários, sem rede ou hooks de produto simulados; cada comando da fixture remove as variáveis locais Git herdadas de hooks. Consulte `environment.txt` para versões capturadas.

Discovery: skills Superpowers 6.4.2 disponíveis no catálogo runtime e arquivos das skills lidos; `cargo xtask doctor --agent both` confirmou managers Codex 6.4.2 e Claude 7.0.0 no contexto normal autorizado. Isso não comprova skills executadas na sessão futura Claude. Aplicadas using-superpowers, writing-plans, using-git-worktrees, executing-plans, test-driven-development, systematic-debugging e verification-before-completion. Execução inline e revisão externa seguem o painel autorizado.

O sandbox falhou em confirmar o plugin Codex e recebeu GitHub 401/404. Reexecução autorizada no usuário normal confirmou ambos managers, repo private=true, PR9 MERGED/base develop/head `3b16c7a4d7977b4853ffa51b57d46098eb1a21cc`/merge `389381cd6bec7442737bd632badb01de917215b2`, e CI pós-merge [38948425547](https://github.com/jimmykedlyn/vynamp-mediaplayer/actions/runs/38948425547) completed/success. Primeira compilação das regressões no sandbox falhou LNK1104 ao escrever TMP externo; foi reexecutada normalmente, sem alteração de permissões, pins ou guardas. Esses erros de ambiente não contam como RED de comportamento.

## Comandos e resultados

1. Antes de editar: `git status --short` vazio; branch e HEAD/base conferidos, git-dir distinto de common-dir, sem superproject. owner.json TASK-E15-02/agent codex/branch/path/base corretos. Nenhuma nova claim ou worktree real criada/removida nesta execução.
2. `cargo xtask doctor --agent both` e `cargo xtask ci docs` no contexto normal: exit 0, baseline 29 unitários + 24 integração, docs/config/fmt/clippy aprovados.
3. RED: `cargo test --package xtask --locked --test tooling removal_ -- --nocapture` antes de modificar produção: exit 101, 3 passaram/5 falharam, [captura](red-removal.txt). Falhas reais: branch trocada, HEAD detached e claim com task/base/branch inconsistente eram aceitos pela remoção. Casos isolados em fixtures descartáveis; nenhuma perda de trabalho real do usuário.
4. GREEN inicial: `cargo test --package xtask --locked --test tooling`, após validar identidade/branch/path/common-dir e ancestry do HEAD real: exit 0, 37/37, [captura](green-tooling.txt). Captura anterior à ampliação do teste dirty para untracked/modified/staged; o gate final cobre a versão final desses testes.
5. Primeira execução do gate documental recusou dois Markdown novos ausentes em docs/INDEX.md: exit 1, captura `docs-index-failure.txt`. Índice corrigido, sem alterar o validador. Gate final: `cargo xtask ci docs`; resultado e contagens em `ci-docs.txt`. Engloba toda a suíte xtask, não apenas o filtro RED, mais docs/config/fmt/clippy.
6. `git diff --check` antes do stage: exit 0. Conferência staged e hooks de commit/push são registrados no guia/PR após ocorrerem; este snapshot não inventa esses resultados.

## Cobertura e oráculos

- Índices de Codex/Claude/raiz têm caminhos físicos distintos; staging em um não altera os outros. Conteúdo e nomes staged observados por Git, sem espelhar funções internas.
- Dois processos disputam a mesma TASK: exatamente um sucesso; claim aponta para o vencedor, branch/path do perdedor não existem. Remover o guard atômico tornaria possível dois sucessos ou claim incorreta.
- IDs/slugs/agent/kind inválidos não criam refs/claims/paths. Destino existente e branch colidente são preservados, claim própria de tentativa falha liberada e retry válido executado.
- Claim sem owner.json representa interrupção após reserva e continua bloqueada. Não há timeout, takeover ou limpeza por idade. Não se alega simular cada instante de encerramento do processo.
- Suja (untracked/modified/staged), não integrada, outro owner, branch trocada, detached e claim inconsistente preservam recursos. Na regressão de drift, o HEAD não integrado real é lido antes e depois.
- Feature com commit integrado por merge commit remove path/branch/claim próprios; arquivo integrado e worktree/claim da outra TASK permanecem. Hotfix main-only e develop-only bloqueiam; ambas bases permitem cleanup.
- Path de outro repositório é rejeitado, preservando índice/claim/worktree. Teste de symlink de destino é Unix-only e depende do CI real; não executado localmente no Windows.

## Limitações e próximo gate

QA-009/QA-012 aqui são contratos adversariais/recuperação do tooling. Não certificam packages/plugins/updater/migration do player, áudio, hardware ou release. Flags `application_ready`, `release_enabled`, `branding_exports_ready` permanecem false; sem mudança em dependências/lock, hooks, autenticação ou permissões.

Claim local coordena processos no mesmo common Git dir, não máquinas distintas. Remoção exige sessões paradas; alterações concorrentes de refs/checkout/arquivos não são suportadas e não são tornadas atômicas por esta correção. Interrupção no meio do cleanup pode exigir recuperação manual dos recursos restantes. Proteções Git sem force continuam ativas. Revisão técnica Claude do SHA final, CI nos quatro runners desktop e integração pelo mantenedor permanecem etapas separadas. Nenhum resultado Claude é preenchido pelo autor.
