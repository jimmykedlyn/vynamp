# E15-04 — Woodpecker local com Cloudflare Tunnel

Data: 2026-10-18. Estado: desenho aprovado pelo mantenedor em 2026-10-18, hostname `ci-cd-vynamp.jkca.lat`; implementação Claude conforme [plano](../plans/2026-10-18-e15-04-woodpecker.md). Desenho escrito pelo Codex; autoria da implementação transferida ao Claude, revisão técnica Codex. Fonte: [TASK-E15-04](../../../tasks/items/TASK-E15-04.md), REQ-E15-04 e QA-018/QA-021.

## Objetivo e decisões do usuário

Executar o CI do repositório privado jimmykedlyn/vynamp-mediaplayer sem consumir minutos de runners hospedados. O usuário escolheu Woodpecker CI e Cloudflare Tunnel; a instalação anterior estava em outra máquina. Esta é uma instalação nova no notebook Windows existente. Reaproveitar os checks Rust, sem mudar requisitos de aceite do produto ou considerar hosts não executados aprovados.

## Arquitetura proposta

GitHub envia webhooks HTTPS para um hostname estável do mantenedor na Cloudflare. Um túnel nomeado encaminha esse hostname somente para o HTTP do Woodpecker em 127.0.0.1:8000. O servidor, em Docker Linux via Docker Desktop/WSL2, mantém seu banco SQLite em volume persistente e agenda dois agentes: Linux/Docker e Windows nativo/backend Local. O agente Windows se conecta ao gRPC do servidor por 127.0.0.1:9000; essa porta não é publicada no túnel. O agente Linux usa a rede interna do Compose.

O caminho de configuração versionada será infra/ci/woodpecker/. Pipelines ficam em .woodpecker/. Dados, credenciais e logs operacionais ficam em D:/Projects/vynamp-media-player/.cache/woodpecker/, fora dos arquivos versionados. Não montar as worktrees dos agentes de desenvolvimento nem suas pastas de credenciais nos containers. Cada execução usa checkout/workspace próprio do CI. Não criar arquivos de runtime dentro de .woodpecker/, onde poderiam ser interpretados como workflows.

O servidor terá login por GitHub OAuth, cadastro público desativado e acesso restrito ao mantenedor. Repositório e logs privados. O OAuth usará a homepage HTTPS real e callback /authorize. Não aceitar código de forks no backend Local Windows. No piloto, executar apenas código confiável do repositório privado, sem segredos de release ou grants de escrita nos steps de build. Credenciais de clone/status permanecem sob controle do servidor/clone plugin, nunca como variável geral dos steps.

Não colocar autenticação interativa Cloudflare Access diante do endpoint de webhook: o GitHub precisa alcançá-lo sem sessão de navegador. A autenticação da UI é do Woodpecker/GitHub; webhooks dependem da validação de assinatura da integração. Aplicar regras adicionais por rota somente após validar OAuth, API e webhook reais. Não abrir portas no roteador.

## Execução e gates

Windows e Linux usam Rust 1.99.0, Cargo.lock existente e comandos canônicos: cargo xtask ci docs, cargo clippy --package xtask --all-targets --locked -- -D warnings e os checks de subsistema disponíveis no SHA exato. As tarefas E01-01/E01-02 adicionam crates em branches próprias; a configuração de CI não deve congelar o workspace antigo como conjunto completo de testes. Validar que todos os membros reais são cobertos no snapshot integrado; corrigir a cobertura na task responsável em coordenação com E01-02, sem sobrescrever seu trabalho.

Core e Android mantêm detecção e gates de fontes/toolchains reais. Não inventar sucesso de Android, hardware ou aplicativo. MacIntel e MacSilicon exigem agentes em Macs reais, que não estão disponíveis nesta máquina. A primeira entrega é um piloto Windows/Linux, não aceite completo da E15-04.

Os contextos Woodpecker devem identificar plataforma e SHA/ref efetivamente testados. Registrar quando a integração usa o merge ref do PR, em vez do head isolado; não apresentar resultados de SHAs diferentes como equivalentes. O agregado completo somente pode passar com todos os hosts/checks exigidos aprovados. Falha, cancelamento e ausência de agente exigido não viram sucesso. Um contexto parcial Windows/Linux deve ter nome e escopo diferentes de CI Gate, para não aprovar a matriz completa por acidente.

Iniciar com um workflow por vez em cada agente e limitar paralelismo total conforme uso de CPU/RAM. Validar carga antes de ampliar. Nenhum agente escreve na árvore de desenvolvimento. Cache Cargo por host/plataforma e diretórios de build por execução ou workspace; não compartilhar index/target com Codex/Claude.

## Entrega incremental

1. Especificação revisada; depois plano com arquivos, pins e testes verificáveis.
2. Configuração de servidor/agentes, scripts operacionais e documentação. Fixar versão/digest das imagens, executáveis e clone plugin a partir de releases oficiais, verificando hashes publicados. Não usar latest nem inventar digest. cloudflared instalado é 2025.8.1; validar compatibilidade e selecionar versão suportada antes da ativação.
3. Preflight local, validação YAML/Compose e execução local das pipelines em checkouts descartáveis próprios. Preservar exit codes de ferramentas nativas no PowerShell: um comando posterior não pode esconder falha anterior.
4. Login Cloudflare, criação do túnel/DNS, OAuth GitHub e ativação do repo somente com dados reais do mantenedor. Segredos informados localmente por prompt seguro ou store, nunca no chat, commit ou log. Não sobrescrever túnel/DNS/OAuth já existentes.
5. Provar webhook e status de PR no commit real; comprovar sucesso, erro e cancelamento. Obter revisão Claude do SHA e deixar merge para o mantenedor.

## Migração do GitHub Actions

Preservar os workflows atuais durante o piloto e registrar a limitação de billing já observada no PR15. Trocar os gatilhos/gate exigido apenas após a paridade estar comprovada e a política de matriz estar explicitamente resolvida. Não simplesmente remover macOS ou converter CI Gate em um alias de Windows/Linux. A indisponibilidade de proteção de branches no plano privado é uma pendência independente; Woodpecker não altera o plano GitHub nem instala proteção inexistente.

## Evidências e recuperação

Evidências permanentes em docs/evidence/PR/task-e15-04/ com SHA/base, OS/ABI, versões/digests, comandos, exit codes, timestamps e links de pipeline/PR. Evidências de baseline da preparação ficam em .superpowers/woodpecker/ e não contam como resultado de uma pipeline Woodpecker instalada.

Verificar: Compose válido; origem HTTP local; gRPC não público; túnel HTTPS e /authorize; login permitido e recusa de usuário não permitido; repo/logs privados; assinatura de webhook inválida recusada; check do SHA/ref correto; falha de teste propagada; cancelamento e agente offline não aprovam gate; checkout de CI não altera worktrees; reinício mantém DB; logs/artefatos sem credenciais.

Parar agentes/servidor/túnel sem apagar volumes/claims/worktrees. Backup consistente do SQLite com servidor parado ou mecanismo próprio do banco; não copiar apenas um arquivo SQLite ativo com WAL. Rollback para workflow anterior só preserva configuração: o bloqueio de billing permanece até resolvido por outro caminho. Não remover volumes nem serviços de outras aplicações.

## Dados necessários para ativação

O mantenedor precisa informar o hostname de um domínio gerenciado na Cloudflare e realizar o login de conta. Depois serão cadastrados OAuth e túnel para esse endereço. Não há hostname ou credenciais fictícios nesta especificação. A ausência desses dados bloqueia ativação externa, mas não a preparação e validação local.

## Fontes consultadas

- [Woodpecker — Docker Compose](https://woodpecker-ci.org/docs/administration/installation/docker-compose)
- [Woodpecker — plataformas e backends](https://woodpecker-ci.org/docs/administration/installation/supported-platforms)
- [Woodpecker — GitHub OAuth](https://woodpecker-ci.org/docs/administration/configuration/forges/github)
- [Woodpecker — backend Local](https://woodpecker-ci.org/docs/administration/configuration/backends/local)
- [Cloudflare — túnel nomeado local](https://developers.cloudflare.com/tunnel/features/locally-managed-tunnels/create-local-tunnel/)
- [Cloudflare — limites do Quick Tunnel](https://developers.cloudflare.com/tunnel/get-started/quick-tunnels/)
