# ADR-018 — entrega automática de TASK pelo `cargo xtask deliver`

Versão: 0.1.0 · Data: 2026-11-14 · Estado: decisão do mantenedor, tomada em 2026-11-14
antes de qualquer código do runner. Autoriza uma exceção estreita ao `AGENTS.md` §6, que
proíbe merge pelo agente.

Status: aprovada pelo mantenedor na mesma data. O código que a implementa é
`cargo xtask deliver`, e ele não existe antes desta ADR de propósito: a regra §1.6 exige
ADR antes do código quando a decisão muda o contrato, e esta muda.

Superada parcialmente pela [ADR-019](ADR-019-subagentes-e-notion.md) em 2026-11-21: o
merge em `develop` passa ao subagente integrador, sob as mesmas condições do §1, e o
`docs-sync` saiu. O `deliver` nunca chegou a `develop`.

## Contexto

O `AGENTS.md` §6 lista "merge, tag, release pelo agente" como proibido, e a §Git diz que
"merge é do mantenedor, pelo GitHub, como merge commit". A regra existe por um motivo
bom: merge é o ponto em que trabalho não revisado entra na linha que todo mundo usa, e um
agente que mergeia sozinho pode transformar um erro de julgamento em `develop` quebrado.

O que mudou não foi o motivo, foi o custo do contorno. Entregar uma TASK neste projeto
hoje exige, além de implementar: escolher a TASK no backlog, criar worktree e claim,
rodar `fmt`/`clippy`/`test`, rodar cinco gates do umbrella, abrir PR com corpo no
template, esperar o CI Gate, mergear, repinar o submódulo no merge commit, atualizar
backlog e épico, rodar `docs-sync`, abrir um segundo PR, esperar o CI Gate de novo,
mergear, fazer fast-forward local, `submodule update` e remover worktree e claim. Desses
passos, **nenhum exige julgamento** — todos são mecânicos, verificáveis e determinísticos.
O julgamento está em implementar e em revisar, e esses dois continuam sendo feitos por
sessões de agente com contexto limpo.

O mantenedor pediu, em 2026-11-14, que esse contorno seja automatizado de ponta a ponta,
com uma sessão nova por TASK. A alternativa de parar no PR verde foi oferecida e
recusada: parar no PR verde é exatamente o contorno que se quer remover.

## Decisão

### 1. Existe um caminho de merge do agente, e ele é um só

`cargo xtask deliver` é o **único** caminho pelo qual um agente mergeia neste projeto.
Não é uma permissão geral: é um comando com escopo fixo no código, e o que ele não cobre
continua proibido como antes.

O que o `deliver` pode mergear, e nada além disso:

| Condição | Verificada por |
|---|---|
| PR criado pelo próprio `deliver` nesta execução, registrado em `state.json` | comparação do número do PR com o que o runner gravou ao criá-lo |
| Base `develop` do repo dono da TASK | `gh pr view --json baseRefName` |
| Head SHA igual ao SHA que o runner revisou e empurrou | `--match-head-commit` no `gh pr merge` |
| Todos os checks exigidos em `SUCCESS`, incluindo o agregado `CI Gate` | `gh pr checks --json` |
| `mergeStateStatus` em `CLEAN` | `gh pr view --json mergeStateStatus` |
| Estratégia **merge commit** | `gh pr merge --merge`, nunca `--squash` nem `--rebase` |

O que o `deliver` **não** pode, em nenhuma configuração: mergear em `main`, criar tag,
publicar release, alterar visibilidade ou proteção de branch, fazer force push, reescrever
histórico, ou mergear um PR que ele não abriu. Essas continuam sendo decisões do
mantenedor, e o código não tem caminho para elas.

### 2. O que o runner faz por TASK

Uma sessão nova de agente por TASK, nunca retomada. A cadeia, com o estado gravado entre
etapas em `.superpowers/deliver/<task>/state.json` (já ignorado pelo Git):

1. **Seleciona** a TASK no `tasks/backlog.json`: `planned`, todas as dependências
   `implemented`, ordenada por fase e prioridade. Recusa TASK marcada como não
   automatizável (§4).
2. **Reivindica**: `cargo xtask repo worktree` no umbrella, ou branch de trabalho na
   `develop` do repo do módulo. A claim atômica continua sendo o mecanismo de posse.
3. **Implementa** numa sessão nova, com o agente `implementador` e TDD. Risco `alto` sobe
   o modelo para Opus, como o §5 já manda.
4. **Porta**: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
   `cargo test`, e no umbrella `check-docs`, `check-tasks`, `check-authoring`,
   `check-delivery`, `check-secrets`, `ci docs`. Falhou, para a TASK e o lote.
5. **Revisa** numa sessão nova, com o agente `revisor` e contexto limpo. O revisor aponta
   e não corrige. Achado bloqueante para a TASK; o runner não mergeia com bloqueio.
6. **Abre o PR** com `--body-file`, corpo no template, evidência em `evidence/PR/<task>/`.
7. **Espera o CI Gate** com teto de tempo. Estourou o teto, a TASK fica em `ci_pending` e
   o lote para — não vira falha nem sucesso.
8. **Mergeia** sob as condições da §1, e verifica o resultado: `state=MERGED`, base
   correta, merge commit presente.
9. **Repina** o umbrella no **merge commit** (nunca no topo da branch), atualiza
   `tasks/backlog.json` e o épico, roda `docs-sync`, e entrega esse segundo PR pela mesma
   cadeia.
10. **Atualiza o local**: `fetch`, `merge --ff-only`, `submodule update`, `docs-sync`,
    remove worktree e claim.

### 3. O runner para, não adivinha

O lote para na primeira TASK que falha, e o relatório diz em qual etapa e com qual saída.
Não há retry automático de etapa que falhou por conteúdo — retry é para falha transitória
de rede, e mesmo esse é limitado e registrado. Uma TASK que falhou fica com o estado
preservado para inspeção; nada é descartado, nenhum `reset --hard`, nenhum `clean -fdx`.

### 4. Nem toda TASK é automatizável, e o runner diz isso em voz alta

Uma TASK é recusada pelo `deliver` quando o critério de aceite depende de algo que o
runner não pode observar:

- hardware de áudio real, dispositivo físico, aparelho Android ou iOS;
- leitor de tela, avaliação visual ou de movimento;
- decisão de produto que a TASK deixa aberta;
- mudança de spec que a TASK não declara.

A lista é um marcador explícito por TASK, não uma heurística sobre o texto. TASK recusada
aparece no relatório com o motivo, e o lote segue para a próxima — recusar não é falhar.

### 5. Guardas que existem porque o erro já aconteceu neste projeto

- **Spec seguindo código.** Se o diff tocar `specs/**/*.md` ou
  `product/03-requisitos.md` sem a TASK declarar mudança de spec, o lote para. Foi o que
  aconteceu no commit `2227943` da `TASK-E01-04`, e é o achado que a revisão não pegou.
- **Teste que nasce verde.** A TASK só passa se o log da sessão trouxer o vermelho de um
  teste novo antes do verde, arquivado em `evidence/PR/<task>/red-proof.txt`. Não é prova
  contra má-fé; é o suficiente para pegar o caso comum.
- **Pin em topo de branch.** O repin aponta para o merge commit, conferido por
  `rev-list --parents` (dois pais). Um pin em topo de branch morre se a branch for
  apagada, e o precedente deste repo é merge commit.
- **Árvore suja no `pre-push`.** O hook roda `cargo xtask ci docs` na árvore de trabalho;
  o runner exige árvore limpa antes de empurrar, senão o push reprova por arquivo de
  outra sessão.

### 6. O que continua sendo do mantenedor

Tag, release, visibilidade, proteção de branch, merge em `main`, e qualquer PR que o
`deliver` não abriu. O `AGENTS.md` §6 passa a dizer isso, em vez de proibir todo merge
pelo agente.

## Lacunas

| Lacuna | Quem cobre | Estado |
|---|---|---|
| TASK que depende de hardware ou de julgamento humano | Mantenedor, TASK a TASK | Fora do escopo do runner por decisão (§4) |
| Custo por TASK em Opus com effort máximo | Mantenedor | Medido por execução em `state.json`; o teto é tempo de parede e número de etapas, não dólar — o proxy em uso reporta `costBasis: unknown` |
| Duas TASKs em paralelo | Ninguém | O runner é sequencial por decisão; o §5 do contrato permite 2–3, e paralelizar é trabalho futuro |
| Revisão que aprova o que não deveria | Mantenedor, na amostragem | O revisor é um agente; o runner garante contexto limpo, não julgamento correto |

## Alternativas

- **Parar no PR verde e o mantenedor mergeia em lote.** Rejeitada pelo mantenedor em
  2026-11-14: é o contorno que se quer remover, e um comando de merge em lote é o mesmo
  merge automático com um passo manual a mais.
- **Paperclip como control plane.** Rejeitada: cria uma segunda fonte de status ao lado do
  `tasks/backlog.json`, que o §4.7 declara única; exige servidor Node, Postgres e UI para
  uma fila que roda uma TASK por vez; e o adapter `claude_local` retoma sessão entre
  execuções, contra o requisito de sessão nova por TASK.
- **Mudar o §6 sem ADR.** Rejeitada pela §1.6: mudar o contrato de entrega é decisão
  estrutural, e decisão só no chat não existe.
- **Permitir merge pelo agente em geral.** Rejeitada: o escopo estreito da §1 é o que
  torna a exceção auditável. Uma permissão geral não teria como ser conferida por gate.

## Consequências

- **O contrato enfraquece, e isso é visível.** "Merge é do mantenedor" passa a "merge é do
  mantenedor, exceto pelo caminho estreito da ADR-018". Quem ler o `AGENTS.md` §6 encontra
  a exceção nomeada, não uma brecha.
- **Um bug no runner mergeia código ruim sem humano no laço.** A mitigação é que as
  condições da §1 são determinísticas e testadas, que o runner nunca mergeia PR que não
  abriu, e que o lote para na primeira falha. Não é garantia; é redução de superfície.
- **O `deliver` vira código de produção do projeto**, com `clippy`, teste e gate no CI
  como qualquer outro. Um runner sem teste seria a pior forma de automatizar merge.
- **A fila fica sequencial e lenta.** Uma TASK de risco alto em Opus com effort máximo é
  sessão longa. Um lote de três numa noite é realista; dez é fantasia, e o runner não
  promete o contrário.
- **O `STATE.md` e o `tasks/NEXT.md` passam a ser atualizados por máquina com mais
  frequência.** O bloco gerado já é conferido pelo `check-docs`, então a frequência não
  muda o risco.
