# Reinício Windows — passo a passo completo

Versão 0.7.0 · 2026-10-14 · Preparação e correções; não é um player funcionando.

## 1. Extrair

Use a pasta `vynamp-media-player` do ZIP em `D:\Projects`; a raiz final é `D:\Projects\vynamp-media-player`. Não extrair duas vezes criando uma raiz duplicada. Não levar `.git`, `target` ou worktrees das versões antigas. O script não apaga pastas antigas nem reescreve commits do GitHub. Ferramentas já instaladas não precisam ser reinstaladas.

## 2. Preparar

PowerShell normal, sem executar como administrador:

```powershell
cd D:\Projects\vynamp-media-player
powershell -NoProfile -ExecutionPolicy Bypass -File .\INICIAR.ps1
```

A permissão Bypass vale só para esse processo, sem mudar a política permanente. O script verifica cargo/git/gh/codex/claude no PATH, usa login GitHub existente ou solicita autenticação, pede nome/email Git se ausentes e configura identidade somente no checkout local. Não grava credenciais no repo/log nem configura safe.directory global.

O tooling Rust clona o repositório privado em `repo`, copia a configuração atual, valida e faz commit. Repositório vazio: publica main/develop. Repositório com histórico: cria branch e PR de bootstrap na base real, preservando branch remota já existente com sufixo novo. Nenhum force push/merge. As worktrees de configuração/revisão usam o snapshot importado; não dependem de develop local inexistente nem iniciam features antes da integração.

O cache inicial baixa/compila dependências uma vez. Cada agente tem seu target/driver; hooks usam target/xtask-hooks próprio por worktree, evitando recompilar o importador/driver durante git commit/push. Os checks usam outro target dir que não contém o xtask.exe em execução; paths externos Git ordinários/UNC recebem normalização adequada. Junctions/symlinks no bootstrap não são usados para escapar da raiz. Criar a pasta em terminal normal evita ownership de Administradores versus usuário.

Se houver erro, o script para e preserva trabalho; consulte `logs\inicio-*.log`. Depois de uma importação concluída (recibo `.cache/import-completed.json`), reexecutar o script reaproveita checkout/worktrees válidos e não sobrescreve arquivos ou claims de outra tarefa. Feche sessões antes de preparar novamente. Não usar --force, --no-verify ou executar como administrador para esconder erro.

## 3. Abrir Codex

Em outro PowerShell normal:

```powershell
cd D:\Projects\vynamp-media-player
powershell -NoProfile -ExecutionPolicy Bypass -File .\ABRIR-CODEX.ps1
```

Atalho entra em `repo\.worktrees\codex\task-e15-01-bootstrap` e passa automaticamente o prompt ao Codex. Verifique Superpowers/discovery real na sessão. Codex conclui E15-01/configuração com evidências e commit/PR, sem aceitar tarefa por mera presença de config. O prompt original está em `PROMPT-CODEX.txt` da raiz externa.

## 4. Abrir Claude

Em um terceiro PowerShell normal:

```powershell
cd D:\Projects\vynamp-media-player
powershell -NoProfile -ExecutionPolicy Bypass -File .\ABRIR-CLAUDE.ps1
```

Atalho entra em `repo\.worktrees\claude\review-bootstrap`, detached e com índice distinto, passando o prompt ao Claude. Primeiro revisa bootstrap/E15-01; não implementa na pasta do Codex. O prompt original está em `PROMPT-CLAUDE.txt`.

## 5. Revisar e integrar

Passe o branch/commit informado pelo Codex para a sessão do Claude. Ele inspeciona aquela revisão e executa os checks apropriados; sem checkout sobre alterações pendentes. Corrigir achados em branch do autor, repetir checks afetados e integrar PR pelo mantenedor após CI Gate/revisão. As sessões não se conectam automaticamente, e duas ferramentas não são duas identidades GitHub.

A integração inicial não é aceite automático de E15-01. Concluir discovery/config, depois E15-02/claims/handoff. Não iniciar E11-01 antes dessas dependências. Por haver histórico remoto, a base de integração é determinada pelo PR, não por presumir main/develop.

## 6. Desenvolver em paralelo

Após as dependências aceitas e develop atualizado: Codex assume E11-01/fundação; Claude assume E15-03/GitHub/Gitflow. O agente prepara nova worktree a partir do checkout raiz, nunca dentro de outra worktree, e respeita ownership/dependências. Um owner por TASK, revisão cruzada e outputs/cache locais. PR/merge/publicação seguem [Gitflow](03-gitflow-github.md).

## Estrutura final

```text
D:\Projects\vynamp-media-player\
├── INICIAR.ps1 / AMBIENTE.ps1
├── ABRIR-CODEX.ps1 / ABRIR-CLAUDE.ps1
├── PROMPT-CODEX.txt / PROMPT-CLAUDE.txt / LEIA-ME.md
├── pacote\                # spec e tooling originais do ZIP
├── repo\                  # checkout Git preservando histórico
│   ├── .git\
│   ├── .worktrees\codex\task-e15-01-bootstrap\
│   ├── .worktrees\claude\review-bootstrap\
│   ├── target\xtask-hooks\ # driver dos hooks, separado do chamador
│   ├── target\xtask-ci\   # cache dos checks isolado do driver
│   └── dist\              # artefatos futuros, ignored
├── .cache\cargo\          # dependências Cargo do projeto
├── .cache\bootstrap-build\
├── .cache\tmp\
└── logs\
```

Programas/SDKs e perfis existentes de Rust, Codex, Claude, GitHub e plugins permanecem suas instalações; nenhum deles é copiado ou versionado. Arquivos do projeto/cache/tmp ficam nesta raiz. Não exportar tokens/grants/credenciais em backup/ZIP. Config local de cada agente fica nas worktrees. A regra não autoriza copiar secrets globais para o projeto.

## Limites de verificação

Logs desta entrega distinguem testes Linux, parser/orquestração PowerShell e execução Windows real. Primeiro INICIAR no Windows executa também as regressões nativas; não alegamos autenticação/push privados ou hardware já executados. application_ready/release_enabled/branding_exports_ready continuam false.

## Importação interrompida antes da confirmação

Se o clone/commit existir, mas push ou criação de PR tiver falhado, o script bloqueia os agentes: não assume publicação concluída. Leia o log e resolva autenticação/rede. Antes de iniciar agentes, preserve a pasta `repo` renomeando-a para `repo-interrompido` dentro da mesma raiz (escolha outro sufixo se já existir) e execute `INICIAR.ps1` novamente. Não apague o checkout interrompido nem fabrique o recibo. A nova importação preserva branches remotas existentes e escolhe outro nome para seu PR quando necessário. Mudanças locais no checkout preservado devem ser revisadas antes de portá-las. Um PR antigo já publicado permanece no GitHub e deve ser identificado na revisão; o script não o fecha automaticamente.
