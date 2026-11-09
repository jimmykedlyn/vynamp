# Evidência TASK-E19-05 — CI Gate nos módulos e READMEs bilíngues

Versão 0.1.0 · 2026-11-09 · Autor Claude. Estado remoto lido da API do GitHub.
Este documento não muda código de produto e não declara capacidade implementada:
`vynamp-decode` e `vynamp-output` continuam em `Status: pre-alpha` e o
`vynamp-mvp-reference` continua sendo a especificação, não produto.

## Escopo executado

| Item | Antes | Depois |
|---|---|---|
| `.github/workflows/ci.yml` de `vynamp-decode` | ausente: nenhum workflow | `fmt`, `clippy`, `test`, `machete` e o job `gate` que publica o status `CI Gate` |
| `.github/workflows/ci.yml` de `vynamp-output` | ausente: nenhum workflow | idem, bloco `gate` idêntico ao do decode |
| `.github/workflows/ci.yml` do `vynamp-mvp-reference` | `ubuntu-22.04`, `cargo install resvg` e `ready()` com `waitForTimeout` fixo | `windows-2022`, sem resvg, e `ready()` esperando o CSS entrar no cascade e as fontes assentarem |
| `README.md` de decode e output | bilíngue, sem uma linha sobre CI | bilíngue, com seção `CI` nas duas línguas |
| `README.md` do MVP | bilíngue, sem uma palavra sobre CI nem sobre a plataforma do runner | bilíngue, com a seção `CI` e o mecanismo que obriga `windows-2022` |

## AC-1 — cada repo publica o contexto CI Gate e reprova falha ou cancelamento

O job, em `libs/decode/.github/workflows/ci.yml` e no mesmo bloco de
`libs/output` (os dois arquivos são idênticos deste `gate:` até o fim):

```yaml
  gate:
    name: CI Gate
    if: always()
    needs: [fmt, clippy, test, machete]
    runs-on: ubuntu-22.04
    steps:
      - name: Aggregate
        shell: bash
        run: |
          test "${{ needs.fmt.result }}" = success
          test "${{ needs.clippy.result }}" = success
          test "${{ needs.test.result }}" = success
          test "${{ needs.machete.result }}" = success
```

`if: always()` é o que separa "reprova cancelamento" de "não roda quando algo
cancela". Um job sem ele é pulado quando uma dependência falha, e pulado não é
vermelho: o pull request fica `blocked` esperando por um status que nunca vem.
`needs.*.result` compara contra `success`, então `failure`, `cancelled` e
`skipped` reprovam os três.

A prova não é a leitura do YAML, é uma execução interrompida. `vynamp-decode`,
run `42650469273` (branch `chore/...`, commit `c43b71c`), cancelado no meio:

```
$ gh api repos/jimmykedlyn/vynamp-decode/actions/runs/42650469273/jobs \
    --jq '.jobs[]|"\(.name) \(.conclusion//.status)"'
fmt success
clippy success
test cancelled
machete cancelled
CI Gate failure
```

Dois jobs `cancelled`, e o `CI Gate` concluiu `failure` — o contexto foi
publicado e reprovou. A contraprova, `66c1608` com tudo verde:

```
$ gh api repos/jimmykedlyn/vynamp-decode/commits/66c1608/check-runs \
    --jq '.check_runs[]|"\(.name) \(.conclusion//.status)"'
CI Gate success
machete success
test success
clippy success
fmt success
```

`vynamp-output` não tem execução cancelada no histórico recente — as corridas
canceladas dele foram reexecuções de `concurrency` sobre o mesmo head, que o
GitHub descarta da lista. O que se verifica ali é que o bloco `gate` é o mesmo
arquivo, byte a byte, e que o status é publicado:

```
$ diff <(sed -n '/^  gate:/,$p' libs/decode/.github/workflows/ci.yml) \
       <(sed -n '/^  gate:/,$p' libs/output/.github/workflows/ci.yml)
gate identico nos dois

$ gh api "repos/jimmykedlyn/vynamp-output/actions/runs?per_page=10" \
    --jq '.workflow_runs[]|"\(.id) \(.head_sha[0:7]) \(.event) \(.conclusion//.status)"'
42654453366 5c910aa pull_request success
42651593985 a667d1c push success
42650710935 e9c3b12 pull_request success
```

**Limitação registrada:** a prova de cancelamento foi feita em um dos dois
repos. Ela vale para o mecanismo, que é o mesmo arquivo, não para um evento
observado em cada repo. Quem exigir o segundo evento precisa cancelar uma
execução de `vynamp-output` à mão.

## AC-2 — o CI do MVP roda serve e verify encadeados

O passo, no repositório do MVP:

```yaml
      - name: Serve and verify, chained
        run: |
          set -eu
          node tools/serve.cjs mvp 8766 &
          server=$!
          trap 'kill $server 2>/dev/null || true' EXIT
          for _ in $(seq 1 60); do
            if curl -sf -o /dev/null http://127.0.0.1:8766/index.html; then break; fi
            sleep 1
          done
          curl -sf -o /dev/null http://127.0.0.1:8766/index.html
          npm run verify
```

Encadeado no mesmo passo, e não em dois: `verify` reextrai o MVP inteiro pelo
servidor, então um servidor que não está de pé reprova a verificação em vez de
ser mascarado. A espera é sondagem, não `sleep` — `sleep` é a corrida clássica,
e uma espera curta demais produz reprovação intermitente que ninguém consegue
reproduzir. O `curl` depois do laço é o que transforma "60 tentativas passaram"
em "o servidor responde"; sem ele um servidor que nunca subiu seguiria adiante
depois de 60 segundos.

Localmente, três execuções completas de `npm run verify` compararam 499 arquivos
com 0 divergência e saíram com 0.

A confirmação hospedada, run `42660165287` em `eb78028`:

```
$ gh api repos/jimmykedlyn/vynamp-mvp-reference/actions/runs/42660165287 \
    --jq '.status+" "+.conclusion'
completed success

$ gh api repos/jimmykedlyn/vynamp-mvp-reference/actions/runs/42660165287/jobs \
    --jq '.jobs[0].steps[]|select(.status!="pending")|"\(.number) \(.name): \(.conclusion)"'
1 Set up job: success
2 Run actions/checkout@...: success
3 Run actions/setup-node@...: success
4 Run npm ci: success
5 Chrome: success
6 Serve and verify, chained: success
```

Duas execuções anteriores do mesmo job foram canceladas **dentro do passo
`Chrome`**, que não terminava: `chrome.exe` no Windows é binário de subsistema
GUI, não escreve no console e não retorna, então `chrome.exe --version` deixou
o run `42654628242` parado 30 minutos sem uma linha de saída. O passo passou a
ler `VersionInfo.ProductVersion` com `pwsh` (`eb78028`), que prova a presença do
navegador em um segundo. Sem esse conserto o AC-2 nunca teria como ficar verde —
o job morria antes de rodar `serve` e `verify`.

## AC-3 — os READMEs são bilíngues

Os três arquivos têm duas metades, a inglesa até o cabeçalho `# Português —` e a
portuguesa dele até o fim, com âncoras cruzadas no topo de cada metade:

| Arquivo | `# ` inicial | `# Português —` | Seções nas duas línguas |
|---|---|---|---|
| `libs/decode/README.md` | `# vynamp-decode` | linha 62 | Documentação/Documentation, CI, Licença/Licence |
| `libs/output/README.md` | `# vynamp-output` | linha 46 | Documentação/Documentation, CI, Licença/Licence |
| `reference/vynamp-mvp/README.md` | `# vynamp-mvp-reference` | linha 83 | Estrutura/Layout, Execução/Run, CI, Fixação/Pinning, Saída/Output |

A metade portuguesa não é tradução decorativa: as duas dizem o mesmo, e o
`vynamp-decode` recusa traduzir a tabela de formatos porque ela é a saída de
`cargo run --example support_matrix` e uma tradução divergiria da geração — a
recusa está escrita nas duas línguas.

## AC-4 — nenhum workflow referencia Woodpecker

Varredura das trees completas, lidas recursivamente na API REST:

```
$ for r in vynamp vynamp-dsp vynamp-decode vynamp-encode vynamp-output \
           vynamp-engine vynamp-player vynamp-converter vynamp-restore \
           vynamp-mvp-reference; do
    gh api "repos/jimmykedlyn/$r/git/trees/HEAD?recursive=1" \
      --jq ".tree[].path|select(test(\"woodpecker\";\"i\"))"
  done
```

Nenhum caminho `.woodpecker/`, nenhum `.woodpecker.yml`, em nenhum dos dez
repositórios. O `ci.yml` do umbrella tem uma menção textual, num comentário:

```
# AC-5 of TASK-E19-01 requires that no credential sits in the history. The
# Woodpecker preflight that used to look for one left with ADR-014, and the
# claim went unverified until `check-secrets` was written -- so the scan has
# to run somewhere, or it is a tool nothing uses.
```

A menção é o registro de por que `check-secrets` existe, não uma referência a
Woodpecker como CI. As duas ocorrências restantes no umbrella são o próprio
ADR-014 e um arquivo de evidência arquivado, nenhum deles um workflow.

O portão que dá consequência mecânica à regra é `ci_errors` em
`xtask/src/validate.rs:540`, que reprova um repositório com `.woodpecker/`
presente. Ele é um teste Rust, não um workflow, e o teste que o cobre
(`github_actions_rejects_woodpecker`) cria o diretório num diretório temporário
e exige a mensagem de erro.

## Verificação

| Comando | Resultado |
|---|---|
| `cargo xtask check-tasks` | `{"errors":[],"result":"passed","tasks":193}` |
| `cargo xtask check-docs` | `"result": "passed"`, `tasks: 193`, `spec_version: 0.7.0` |
| `npm run verify` no MVP (local, 3×) | `compared 499 files, 0 differ`, exit 0 |
| `npm run check-svg --check` no MVP | 6 arquivos reprovados — conhecido, ver Boundary |

## Execuções

| Repo | Run | Head | Conclusão |
|---|---|---|---|
| `vynamp-decode` | `42650469273` | `c43b71c` | `failure` no `CI Gate` com `test`/`machete` cancelados |
| `vynamp-decode` | `42650585650` | `66c1608` | `success`, `CI Gate` incluído |
| `vynamp-decode` | `42654357542` | `cb3b477` | `success` |
| `vynamp-output` | `42651593985` | `a667d1c` | `success` |
| `vynamp-output` | `42654453366` | `5c910aa` | `success` |
| `vynamp-mvp-reference` | `42654628242` | `89c0059` | `cancelled` no passo `Chrome`, que não terminava |
| `vynamp-mvp-reference` | `42660165287` | `eb78028` | `success`, os seis passos |
