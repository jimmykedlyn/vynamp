# Evidência TASK-E19-01 — CI do repo principal em GitHub Actions

Versão 0.8.1 · 2026-11-05 · Autor Claude (branch `feature/task-e19-01-claude-public-github-actions`, base develop `a381d27`). Documento, gate local e configuração de CI; **não** é aceite de app, hardware nem release.

## Escopo entregue

| Item | Arquivo |
|---|---|
| Workflow de CI com gate `CI Gate` | `.github/workflows/ci.yml` |
| Release por tag `v*`, fail-closed | `.github/workflows/release.yml` |
| Hardware manual, opt-in explícito | `.github/workflows/quality.yml` |
| Validador invertido (exige Actions, proíbe Woodpecker) | `xtask/src/validate.rs` |
| Release valida tag por `GITHUB_REF`/`GITHUB_REF_NAME` | `xtask/src/release.rs` |
| `repo audit`/`repo protect` exigem repo público | `xtask/src/repo.rs` |
| Visibilidade esperada `public` | `config/github.json` |
| Dependabot volta a atualizar actions | `.github/dependabot.yml` |
| Woodpecker removido | `.woodpecker/`, `infra/ci/` |
| Docs normativos reescritos | `docs/engineering/{03-gitflow-github,04-ci-cd}.md`, `ADR-009`, `ADR-010`, `ADR-011`, `AGENTS.md`, PR template, `docs/INDEX.md` |

## Desenho dos workflows

`ci.yml` dispara em PR para `develop`, push em `develop`/`main` e manual. Push em branch de feature não dispara — o PR da mesma branch cobre o diff. Concurrency cancela execuções antigas da mesma ref.

Jobs: `docs` (docs/schema/backlog/delivery/fmt/clippy dos tooling + outputs `core`/`android` de `cargo xtask ci detect`), `tooling` (clippy do workspace em `windows-2022`, `ubuntu-22.04`, `macos-15-intel`, `macos-15`), `core` (matriz de quatro alvos desktop, `x86_64-pc-windows-msvc`, `x86_64-unknown-linux-gnu`, `x86_64-apple-darwin`, `aarch64-apple-darwin`, condicionada a `needs.docs.outputs.core == 'true'`), `android` (`ubuntu-24.04`, condicionada ao output), `gate`.

`gate` tem `name: CI Gate`, `if: always()` e `needs: [docs, tooling, core, android]`; as checagens fazem `test "${{ needs.<job>.result }}" = success` e só conferem `core`/`android` quando o `detect` diz que existem. Dependência falha, cancelada ou ausente reprova — não há caminho para gate verde sem as dependências reais.

`release.yml` dispara só em `push` de tag `v*`: `preflight` (checkout com `fetch-depth: 0`, `cargo xtask tag-version`, `cargo xtask release preflight`) e `publish` com `exit 1`. A matriz de quatro installers do commit `247b424` **não** foi restaurada: depende de artefatos entre agentes, secrets de signing e caminho de build macOS arm64 que o preflight ainda recusa.

`quality.yml` é `workflow_dispatch` com input booleano `confirm_hardware_runners`, default `false`. Com `false` o job `preflight` registra que os runners `vynamp-hardware`/`vynamp-android-device` (TASK-E15-05) ainda não existem e a matriz é pulada, em vez de ficar em fila.

Todas as `uses:` são `actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683`. Nenhum `uses:` por tag.

## Comandos e resultados reais

Ambiente: Windows 11 Pro 10.0.26300, Rust 1.99.0 (rustup), git 2.47.1.windows.2.

| Comando | Resultado |
|---|---|
| `cargo test --workspace --locked` | 86 passed (2 suites) |
| `cargo fmt --all --check` | limpo |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | No issues found |
| `cargo xtask check-docs` | `"result": "passed"`, `"errors": []`, 347 arquivos markdown |
| `cargo xtask check-delivery` | `"result": "passed"`, `"errors": []`, `"workflow_files": 3` |
| `cargo xtask ci detect` | `{"android":"false","core":"false"}` |
| `grep -rn "uses:" .github/workflows/` | 6 referências, todas com SHA de 40 hex |

### Execução real no GitHub

O workflow rodou no PR #30, em `d8fd3ba` (evento `pull_request`), e concluiu com sucesso. Run [42101013430](https://github.com/jimmykedlyn/vynamp-mediaplayer/actions/runs/42101013430). Execuções verdes anteriores: `36d0e02` (run `42089094392`), `a23408f` (`42093428684`), `7565e12` (`42096253664`) — os três últimos commits apenas corrigem esta evidência e a documentação.

| Check | Resultado | Duração |
|---|---|---|
| `docs` | pass | 1m29s |
| `tooling (windows-2022)` | pass | 1m49s |
| `tooling (ubuntu-22.04)` | pass | 55s |
| `tooling (macos-15-intel)` | pass | 2m14s |
| `tooling (macos-15)` | pass | 24s |
| **`CI Gate`** | **pass** | 5s |
| `core` | skipped | — |
| `android` | skipped | — |

Isso corrige a afirmação anterior desta evidência de que nenhum workflow teria rodado. O repositório é **privado** e o Actions executou normalmente — o bloqueio de billing registrado em 2026-10-18/23 (runs `39171754196`, `39979729441` etc., que falhavam sem executar nenhum step) não se repete hoje. A razão da mudança não foi determinada: pode ser consumo de cota do plano, ciclo de reset ou configuração da conta. **Não** se deve tratar "repo privado" como suficiente para prever bloqueio, nem como suficiente para prever que roda.

### Provas de comportamento (não só estrutura)

Duas verificações que testam o comportamento em vez de só parsear o YAML:

1. **O validador reprova pin por tag.** Trocar `actions/checkout@11bd7190…` por `actions/checkout@v4` em `ci.yml` faz `cargo xtask check-delivery` sair `"result":"failed"` com `pinned action actions/checkout@v4 in ci`. Restaurar o SHA volta a `"passed"`. A árvore foi conferida limpa depois (`git status --short` vazio).
2. **A lógica do gate reprova dependência não-verde.** O corpo do step é `bash -e` com `test "$…" = success` em sequência. Reproduzido fora do Actions: `success`+`success` → verde; `failure`+`success` → vermelho; `success`+`cancelled` → vermelho; `skipped`+`success` → vermelho. Só a combinação totalmente verde passa.

O que continua **não** provado: o comportamento real do runner do GitHub para esses casos (só a lógica é exercitada aqui) e qualquer coisa que dependa de `core`/`android` rodarem.

## Limitações — o que este PR não prova

- `core` e `android` ficam **pulados**: `cargo xtask ci detect` devolve `false`/`false` porque o workspace contém só `xtask`. Skip declarado não significa que o player passou testes, e o `CI Gate` verde não exercita build de áudio, app desktop nem Android.
- O gate publicado como status `CI Gate` já existe e passou, mas `config/github.json` continua `protection_status: pending_plan`: a branch protection **é o que exige** esse status, e segue indisponível no repositório privado. Sem ela o check é informativo, não bloqueante.
- `release.yml` falha de propósito em `publish`; nenhum artefato assinado é produzido. O caminho de tag nunca foi executado.
- `quality.yml` nunca rodou — depende de runners self-hosted inexistentes.
- Hardware segue sem cobertura de CI — depende da TASK-E15-05.
- macOS Intel/Apple Silicon executaram `tooling` com sucesso, mas o job `core` neles está pulado; a cobertura macOS de código de produto continua não exercitada.

## Controles de segurança preservados

- `permissions: contents: read` no CI e no release; o único job com permissão para publicar não existe ainda.
- `persist-credentials: false` em todo checkout.
- `release.yml` referencia secrets de signing apenas por **nome**; nenhum valor, nenhuma chave privada em workflow, artefato ou log.
- O validador recusa `.woodpecker/` e qualquer `uses:` sem SHA completo, para que a inversão da ADR-010 não reabra a porta a tag flutuante.
- `git log --all -p` do repositório não revela token, PAT, chave AWS ou chave privada (scan de 2026-11-05, sem achado além de um falso positivo no próprio detector em `infra/ci/`, agora removido).

## AC-4 e AC-5 — implementados em 2026-11-09

Os dois critérios existiam na TASK desde a geração do passo 37 e nada os verificava.

### AC-4 — macOS Intel e Apple Silicon como alvos verificados

O job `tooling` passou a rodar numa matriz `[ubuntu-22.04, macos-15-intel, macos-15]`.
`macos-latest` é recusado de propósito: ele troca de arquitetura quando o GitHub
rotaciona a imagem, e alvo que se move não é alvo fixado. `check-delivery` reprova se
qualquer um dos dois Macs sair da matriz **e** se `macos-latest` aparecer nela — um
gate e não uma preferência, porque tirar uma linha da matriz devolve o macOS a "não
verificado" com todo o resto verde, que é exatamente o estado que a ADR-014 encerra.

### AC-5 — varredura de segredo no histórico

`cargo xtask check-secrets` (`xtask/src/secrets.rs`) lê **todo blob alcançável de toda
ref**, não a árvore de trabalho. Segredo apagado no commit seguinte continua no clone
e a árvore de trabalho diria que está limpo — é o caso para o qual o critério existe.

Cobre forma sem ambiguidade à vista: token de forge e de nuvem com prefixo, chave
privada PEM. Não faz heurística de entropia: ela exigiria um arquivo de baseline para
ser usável, e baseline é onde achado real vai para ser esquecido.

O achado informa blob, caminho e linha e **nunca** o texto casado — log de CI é retido
e um scanner que imprime o segredo o publicou.

Resultado real neste repositório (2026-11-09): `No credential shape found in the
reachable history.`, 3.184 blobs, 4,2 s.

### Dois defeitos encontrados só porque a varredura precisava rodar

| Defeito | Medição |
|---|---|
| um `git cat-file -p` por blob, um processo cada | 330 s numa execução, mais de 600 s em outra; **9,5 s** com um `cat-file --batch` só |
| `object_types` escrevia todo id antes de ler saída | 330 s em sequência; **9,5 s** com o escritor em thread própria |

O segundo foi descrito primeiro como deadlock e **não é**: a versão sequencial
termina, e o teste de tempo passa nas duas versões. Fica registrado o que foi medido,
não um mecanismo que nunca se confirmou — a mesma regra que este repositório aplica a
"parece certo" em qualquer outro lugar.

O passo está no job `tooling`, precedido de `git fetch --unshallow` (o
`actions/checkout` busca um commit só, e um clone raso varreria a árvore de trabalho).
`check-delivery` reprova se qualquer um dos dois passos sumir: um scanner que o CI não
roda é ferramenta que ninguém usa.

### JWT do Woodpecker

Removido por completo por decisão do mantenedor em 2026-11-09. Os dois hooks saíram na
TASK-E19-06, mas o valor sobrevivia num backup não versionado em
`.claude/settings.local.json.bak-30`; o arquivo não existe mais e nenhuma cópia resta na
árvore. Revogar no serviço não é possível deste lado — o servidor já estava morto.

## Pendente

- Abertura dos repositórios e reaplicação de branch protection: TASK-E19-06 (feito).
- Licença, governança, SECURITY.md e README bilíngue: TASK-E19-02.
- Remoção do Woodpecker local (Docker Desktop, agente Windows, Cloudflare Tunnel) fora
  do repositório: operacional, do mantenedor. O JWT não está mais na árvore; o servidor
  e o agente continuam instalados na máquina.
