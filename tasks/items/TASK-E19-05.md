# TASK-E19-05 — CI e README bilíngue nas libs Rust e no MVP de referência

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E19-05` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E19-05` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M1 |
| Prioridade | P1 |
| Estado | implemented |
| Depende de | `TASK-E19-01` |
| Bloqueia | `TASK-E19-06` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–6 dias úteis |
| Risco | médio |

## Entrega

GitHub Actions com gate CI Gate em vynamp-decode e vynamp-output, CI próprio em Node no vynamp-mvp-reference, e README bilíngue EN/PT nos três repos, com Woodpecker removido.

## Boundary

- **Não** escreve o CI das cinco libs e dos três apps além do que a TASK já tinha:
  `dsp`, `encode`, `engine`, `player`, `converter` e `restore` já publicavam `CI Gate`
  desde o passo 36. Esta TASK fecha `decode` e `output`, que estavam sem workflow, e dá
  ao MVP um CI próprio em Node.
- **Não** entra no CI do umbrella (`specs`, `tooling`, `submodules`, `arch`, `tasks`).
  Esse é o `ci.yml` da raiz, anterior a esta TASK, e continua dono dos gates
  documentais.
- **Não** conserta o que o `verify` do MVP compara. `out/` é a especificação congelada
  e `mvp/` não é nosso; quando o CI reprova por divergência, a correção é do host que
  mede, não do artefato. Foi exatamente esse o caso de `#graph-minimap`.
- **Não** acrescenta `check-svg` ao CI. `out/check/report.json` lista 6 arquivos que
  falham por lacuna de renderizador, à espera das tasks de fonte e de carregador;
  transformá-los em gate agora seria PR vermelho de propósito.
- **Não** aplica branch protection nem exige o status. Isso é `TASK-E19-06`, executada
  em 2026-11-08; esta TASK só faz o status existir e valer.
- **Não** traduz `mvp/`. O código do MVP é a especificação e não é editado; os READMEs
  citam `mvp/` com os nomes originais.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e19-05-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E19-05-AC-1` | os ramos de trabalho dos três repositórios e o log de execuções do GitHub Actions | um pull request de cada repositório é levado ao verde e, depois, uma execução é interrompida no meio | Cada repo publica o contexto CI Gate e reprova falha ou cancelamento |
| `TASK-E19-05-AC-2` | o `.github/workflows/ci.yml` do `vynamp-mvp-reference` e o `out/` congelado | o job roda o servidor e depois `npm run verify` contra ele, no mesmo passo | o CI do MVP roda serve e verify encadeados |
| `TASK-E19-05-AC-3` | os READMEs de `vynamp-decode`, `vynamp-output` e `vynamp-mvp-reference` | cada arquivo é aberto do topo ao fim | os READMEs são bilíngues |
| `TASK-E19-05-AC-4` | as trees completas dos dez repositórios, lidas recursivamente na API REST | cada caminho e cada arquivo de workflow é procurado por Woodpecker como sistema de CI | nenhum workflow referencia Woodpecker. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `gh api repos/jimmykedlyn/<repo>/actions/runs/<id>/jobs` sobre uma execução interrompida de cada repo Rust, e `npm run verify` no `vynamp-mvp-reference` comparando 499 arquivos | o job `gate` sai `failure` com dependência `cancelled`, e o `verify` compara byte a byte contra `out/` — dois resultados lidos da API e do relatório, não "parece certo" |
| Hardware | nenhum: o CI do MVP roda no `windows-2022` hospedado, sem dispositivo de áudio, e o `quality.yml` do umbrella segue dono da matriz de hardware | ausência é limitação registrada, não aprovação |

## DoD — pronto para review

- [x] Todos os critérios de aceite com evidência (evidência em `evidence/PR/task-e19-05/`)
- [x] `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` limpos (verde no CI Gate do umbrella e dos módulos)
- [x] `cargo test` verde; testes ignorados listados com motivo (nenhum ignorado nesta entrega)
- [x] Spec do módulo atualizada se a implementação revelou algo novo (`engineering/04-ci.md` não precisou mudar: já descrevia o mecanismo; o que esta TASK revelou virou evidência e comentário no workflow)
- [x] `backlog.json` com o estado correto (`implemented`, `acceptance_ready: true`)
- [x] Revisão feita por contexto limpo, não pelo implementador
- [x] PR aberto com template preenchido; CI Gate verde no SHA final
## Notas de execução

- Os READMEs de decode e output já eram bilíngues; o que faltava era dizer que o
  CI existe. A seção `CI` foi escrita nas duas línguas nos três repos.
- O `verify` do MVP reprovava de forma determinística no `ubuntu-22.04` em cinco
  arquivos (`#graph-minimap` em dois estados mais `index.json`) porque `out/` é
  artefato de geometria medida: `drawMini()` deriva visibilidade e conteúdo do
  rect medido de `#graph-viewport`, que é `flex:1` sob cabeçalho dimensionado por
  texto. Trocar a família de fonte move esse rect em um pixel, o que basta para
  virar `overflow` em outro ponto da varredura de cliques. O runner virou
  `windows-2022`, a família que congelou `out/` — medir a spec, não o runner.
- `chrome.exe --version` nunca terminava: no Windows o Chrome é binário de
  subsistema GUI e não retorna. O run `42654628242` ficou 30 minutos no passo
  `Chrome` sem uma linha de saída. O passo passou a ler
  `VersionInfo.ProductVersion` com `pwsh`.
- A prova do AC-1 de cancelamento foi observada no decode (run `42650469273`:
  `test`/`machete` cancelados, `CI Gate` `failure`). O `vynamp-output` publica o
  mesmo arquivo, byte a byte; um evento de cancelamento observado nele ficou
  registrado como limitação na evidência.
- O commit `62f09ce` do extrator (`ready()` esperando o CSS entrar no cascade e
  as fontes assentarem) não era a causa do vermelho do Linux, mas ficou: tira a
  corrida de 500 ms fixos e endurece o portão.
- Re-pins: `libs/decode` → `da812d7`, `libs/output` → `3607f0d` (PR #20) e
  `reference/vynamp-mvp` → `e689d90` (PR #21). Todos merge commits.
