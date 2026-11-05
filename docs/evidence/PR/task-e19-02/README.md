# Evidência TASK-E19-02 — higiene, governança e README bilíngue

Versão 0.8.1 · 2026-11-05 · Autor Claude (branch `feature/task-e19-02-claude-governance-bilingual-readme`, base develop `89fde34`). Documento e arquivos de repositório; **não** é release assinado, **não** é abertura dos repositórios e **não** aplica branch protection.

## Escopo entregue

| Item | Arquivo |
|---|---|
| Licença dual `MIT OR Apache-2.0` | `LICENSE`, `LICENSE-MIT`, `LICENSE-APACHE` |
| Notas legais e política de formato | `LEGAL.md` |
| Código de conduta | `CODE_OF_CONDUCT.md` |
| Governança e decisões registradas | `GOVERNANCE.md` |
| Canal de vulnerabilidade | `SECURITY.md` (reescrito) |
| Formulários de issue | `.github/ISSUE_TEMPLATE/{bug_report,docs_problem,config}.yml` |
| README bilíngue | `README.md` |
| Contribuição bilíngue | `CONTRIBUTING.md` |
| Histórico bilíngue | `CHANGELOG.md` |
| Contrato do agente atualizado | `AGENTS.md` |
| Decisão de licença e canal | `docs/product/05-decisions.md` (D-LIC-01, D-LIC-02, D-GOV-01) |
| Proibição de licença retirada do estado antigo | `docs/security/02-privacy-licensing.md` |
| Índice com os documentos novos | `docs/INDEX.md` |

`docs/engineering/03-gitflow-github.md`, `docs/engineering/04-ci-cd.md` e a marcação da `ADR-010` como substituída **já vinham** da TASK-E19-01 (PR #30) e foram verificados aqui, não reescritos.

## Decisões registradas

O projeto estava num estado em que `AGENTS.md` proibia *"criar licença final sem decisão registrada"* e `docs/security/02-privacy-licensing.md` proibia criar `LICENSE` "MIT" "como se tivesse sido decisão do usuário". Criar `LICENSE` sem resolver isso seria contradizer a própria governança. Por isso as duas proibições foram substituídas por decisões explícitas em `docs/product/05-decisions.md` (versão 0.8.0):

- **D-LIC-01** — o código deste repositório é `MIT OR Apache-2.0`, igual às libs `vynamp-decode` e `vynamp-output`.
- **D-LIC-02** — a licença cobre só o código deste repositório: não relicencia `crates-copyleft/` das libs de áudio, não concede direitos sobre codecs de terceiros e **não equivale a release**.
- **D-GOV-01** — canal de vulnerabilidade = GitHub Security Advisories do próprio repositório, sem e-mail novo.

`docs/security/02-privacy-licensing.md` (0.8.0) troca a proibição por uma condição: `LICENSE` existir não substitui `THIRD_PARTY_NOTICES`, SBOM nem evidência assinada. O `release preflight` exige hoje `THIRD_PARTY_NOTICES`, evidência de hardware real por plataforma e o registro de revisores (`xtask/src/release.rs:88-125`); SBOM e assinatura minisign entram no `publish-draft` e na spec de entrega. `AGENTS.md` (0.9.0) mantém a regra e aponta para D-LIC-01.

## Canal de segurança

`SECURITY.md` aponta para `https://github.com/jimmykedlyn/vynamp-mediaplayer/security/advisories/new` e não declara ausência de canal de contato.

Duas escolhas conservadoras, feitas para não inventar compromisso operacional:

- **Não há SLA declarado.** A primeira versão dizia "acknowledge a complete report within 5 business days"; foi substituída por "no committed response SLA — reports are read as capacity allows", porque criar prazo de resposta não é decisão do agente.
- **Não foi criado endereço de e-mail.** O formulário de conduta (`CODE_OF_CONDUCT.md`) também encaminha para os advisories, marcados como assunto de conduta, em vez de inventar um contato.

`.github/ISSUE_TEMPLATE/config.yml` desativa issue em branco e oferece dois links: advisories (segurança) e `CONTRIBUTING.md` no `develop`. O primeiro desenho apontava para GitHub **Discussions**, que pode não estar habilitado no repositório; foi trocado.

## Por que formulários `.yml` e não templates `.md`

`xtask/src/validate.rs::spec` varre o repositório inteiro e, para **todo** `.md`, exige título `# ` e entrada em `docs/INDEX.md`. Um template de issue em Markdown viraria documento normativo indexado. Os Issue Forms (`*.yml`) não passam por essa regra e são o formato suportado pelo GitHub para campos obrigatórios — o que importa aqui, porque o repositório exige evidência (SHA, saída de comando) e não texto livre.

Isso também explicou a única reprovação real deste trabalho: `check-docs` acusou `Missing index` para `CODE_OF_CONDUCT.md`, `GOVERNANCE.md` e `LEGAL.md`, corrigido com três entradas em `docs/INDEX.md`.

## Comandos e resultados reais

Ambiente: Windows 11 Pro 10.0.26300, `rustc 1.99.0 (b940084d7 2026-09-28)`, `git 2.47.1.windows.2`.

| Comando | Resultado |
|---|---|
| `cargo fmt --all --check` | exit 0, sem diff |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | sem aviso |
| `cargo test --workspace --locked` | 86 passed (2 suites, 26.31s) antes da revisão, 30.26s depois; mesmo resultado |
| `cargo xtask check-docs` | `"result": "passed"`, `"errors": []`, 353 arquivos Markdown, 191 requisitos, 19 épicos |
| `cargo xtask check-delivery` | `{"errors":[],"result":"passed","workflow_files":3}` |

Saída literal de `check-docs`, antes e depois da correção:

```text
"errors": [
  "Missing index: CODE_OF_CONDUCT.md",
  "Missing index: GOVERNANCE.md",
  "Missing index: LEGAL.md"
],
"result": "failed",
```

```text
"errors": [],
"result": "passed",
```

## Revisão independente e correções

Revisão em contexto limpo (Sonnet 5-5, sem acesso ao raciocínio do autor) devolveu cinco achados. Todos foram reconferidos pelo autor e todos eram reais; nenhum foi contestado.

| # | Achado | Correção |
|---|---|---|
| 1 | `CONTRIBUTING.md` documentava o padrão de branch `feature\|fix\|chore/<task>-<slug>`, que o hook real recusa | padrão substituído pelo regex de `xtask/src/repo.rs:225` nos dois idiomas, com exemplo e a menção ao `guard-branch` |
| 2 | `bug_report.yml` apontava para `../blob/develop/SECURITY.md` — caminho relativo que não resolve | `../../SECURITY.md` |
| 3 | `tasks/backlog.json` marcava TASK-E19-01 como `implemented` enquanto sua spec dizia "Estado planejado" — viola a regra de `tasks/README.md` de alterar JSON e Markdown juntos | JSON voltou a `in_progress`; as duas specs passaram a "em execução" |
| 4 | `README.md` listava `vynamp-restore`/`vynamp-converter`/`vynamp-encode` como se fossem repositórios existentes | texto corrigido nos dois idiomas: decididos, **ainda não criados** (TASK-E19-04) |
| 5 | `docs_problem.yml` pedia para o repórter confirmar que `check-docs` "não reporta a falha" — mas `check-docs` é estrutural e passa num erro semântico; o texto induzia a um check irrelevante | rótulo reescrito para "`check-docs` passa — é um problema semântico que o validador não vê" |

O achado 3 foi causado pelo próprio autor deste PR: a primeira versão marcou E19-01 como `implemented` ao mergear o PR #30, o que o validador não detecta porque `validate::spec` compara `title`/`deliverable`/`acceptance` entre backlog e Markdown, não `status`.

`tasks/README.md:5` foi corrigido junto: afirmava "todos os itens estão planejados", o que já era falso antes deste PR (três itens em `in_progress`).

### Segunda passagem: três afirmações que o código não sustentava

Uma segunda revisão atacou o que os documentos afirmavam sobre `xtask`, em vez de sobre a prosa. Três achados, todos confirmados no código-fonte e corrigidos:

| Achado | Verificação | Correção |
|---|---|---|
| Documentos diziam que `release preflight` verifica SBOM e assinatura | `grep -ri sbom xtask/src/` → vazio; `preflight` (`release.rs:72-142`) exige `THIRD_PARTY_NOTICES`, evidência de hardware e digest de revisores; `minisign` aparece só em `release.rs:266,280`, dentro de `publish-draft` | `LEGAL.md`, `05-decisions.md` e `02-privacy-licensing.md` passam a atribuir SBOM e assinatura ao `publish-draft` e à spec de entrega |
| `SECURITY.md` e `CODE_OF_CONDUCT.md` apresentavam o canal de advisories como disponível | `gh api .../private-vulnerability-reporting` → 404; `https://github.com/.../security/advisories/new` respondida a anônimo → **404** | os dois arquivos passam a dizer que o canal entra no ar na TASK-E19-06, e a E19-06 ganhou o `PUT` do PVR como critério de aceite |
| `crates-copyleft/` descrito como diretório existente | não existe em nenhum repositório do projeto | `LEGAL.md` passa a dizer que é convenção do `vynamp-decode`, não diretório populado |

Também nessa passagem: `LICENSE` dizia "both … and" para uma licença **OR**; `xtask/Cargo.toml` ganhou `license = "MIT OR Apache-2.0"` para a expressão SPDX ter lastro no manifesto; e `docs/INDEX.md` deixou de chamar a política de segurança de "proposta".

### Bilinguismo de `LEGAL.md`

`LEGAL.md` era o único arquivo de governança com bilinguismo parcial: só a seção de licença tinha versão em português, sob um cabeçalho à parte que repetia o preâmbulo. As outras quatro seções eram inglês-only. Passou ao mesmo padrão dos demais (`##` em inglês, `###` em português), com uma única declaração de qual texto prevalece — a mesma do `CHANGELOG.md`.

### Limitação deste registro

As correções descritas em "segunda passagem" e no bilinguismo de `LEGAL.md` chegaram por commits feitos em paralelo na mesma branch (`25ba40f` e `62b8bc1`), não por uma revisão sequencial que eu tenha conduzido do início ao fim. Eu reconferi cada afirmação no código antes de mantê-la aqui — `release.rs`, o endpoint de PVR e o 404 da URL — mas os dois revisores independentes que deveriam produzir uma terceira passagem morreram por **cota do plano esgotada** (HTTP 429 em `claude-opus-5-5`, reset em ~76 h), sem emitir relatório. Portanto: os achados das duas passagens estão corrigidos e verificados, e **não há uma terceira revisão independente deste PR**.

## Verificação de aceite

| Critério da TASK | Verificação | Estado |
|---|---|---|
| README tem seção English e seção Português com o mesmo conteúdo | `README.md` linhas 1–133: `# Vynamp` (EN) e `# Português — Vynamp` (PT), com os mesmos 7 blocos | atendido |
| `SECURITY.md` contém o link dos advisories e nenhuma frase de "URL ou e-mail de segurança ainda não aprovado" | o link a `security/advisories/new` está presente (`grep -c advisories/new SECURITY.md` → 1; a lista de URLs é única e serve EN e PT na mesma linha); nenhuma frase promete aprovação futura de URL ou e-mail — o que o arquivo diz é outra coisa: que o **recurso** de relato privado ainda não está ligado, porque o repo é privado, e que ele entra no ar na TASK-E19-06 (verificado: `private-vulnerability-reporting` retorna 404 hoje) | atendido |
| `LICENSE`, `LICENSE-MIT`, `LICENSE-APACHE` existem e `MIT OR Apache-2.0` está em `LEGAL.md` | os três arquivos existem; `LEGAL.md` declara a expressão SPDX e D-LIC-01 | atendido |
| `04-ci-cd.md` descreve Actions, sem "NOT VERIFIED" | `04-ci-cd.md:19` declara `macos-15-intel` e `macos-15` verificados; única ocorrência de `NOT VERIFIED` no repo fora de `archive/` está na `ADR-014:21` e na evidência histórica da E15-04, ambas citando a limitação antiga | atendido |
| `ADR-010` marcado "Substituída pela ADR-014" e preservado | `ADR-010:3` traz o aviso e o texto original segue no arquivo | atendido |

## Limitações

- **Nada foi aberto.** A visibilidade dos repositórios continua privada; abertura, `description`, `--enable-issues` e branch protection são a TASK-E19-06.
- **Nenhum formulário de issue foi renderizado pelo GitHub.** Os três `.yml` são válidos como YAML e seguem o schema de Issue Forms, mas o GitHub só os valida ao exibir a página de nova issue em repositório com Issues habilitado — não verificado.
- **`SECURITY.md` não foi exercitado.** Nenhum advisory foi aberto; o link está correto e o recurso existe no GitHub para repositórios da conta, mas o fluxo só se prova com um relatório real.
- **`CHANGELOG.md` bilíngue não é histórico versionado.** As entradas reconstroem decisões a partir de `docs/product/05-decisions.md`, das ADRs e das evidências; não são uma reemissão de releases passadas, que nunca existiram como tags.
- **A licença não foi revisada por advogado.** `MIT OR Apache-2.0` foi escolhida por paridade com as libs do projeto e registrada como decisão; a escolha de licença é do mantenedor.
- **`crates-copyleft/`, codecs de terceiros e a marca Vynamp seguem fora da licença** — declarado em `LEGAL.md` e na D-LIC-02, não resolvido.
