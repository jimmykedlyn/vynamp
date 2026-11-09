# Evidência TASK-CI-01 — `delivery()` e `github_actions()` na árvore do umbrella

Versão 0.1.0 · 2026-11-09 · Autor Claude. Verificação local; nada foi executado
no remoto neste passo.

## AC-1 — valida spec, `xtask`, pins e arquitetura; nenhuma checagem de empacotamento

`cargo xtask check-delivery` está verde e o relatório declara o escopo:

```
$ cargo xtask check-delivery
{"errors":[],"result":"passed","scope":"Structural config validation;
 no remote CI/signing/hardware claim","workflow_files":2}
```

O único lugar onde a palavra "packaging" ainda aparece em `validate.rs` (linha 771)
é um comentário dizendo que o gate **deixou** de checar empacotamento — não uma
checagem. Nenhuma referência a `docs/specs`, `npm` ou `node` sobrou no caminho.

## AC-2 — os testes de workflow passam com o `ci.yml` do passo 34

`cargo test --package xtask --locked`:

```
test result: ok. 72 passed; 0 failed; 0 ignored
test result: ok. 33 passed; 0 failed; 0 ignored
```

São 11 testes cujo nome começa com `github_actions_`, mais o fixture
`ci_fixture()` compartilhado. O fixture não é uma cópia do workflow real: ele é o
mínimo que satisfaz as regras, de forma que o teste continua válido quando o
`ci.yml` real muda.

## AC-3 — os submódulos do passo 31 no estado semeado dão verde

`cargo xtask check-delivery` roda contra a árvore de trabalho atual, com os nove
submódulos no estado pinado pelo umbrella, e não acusa pin. O `git submodule status`
abaixo mostra os SHAs que o gate leu:

```
   4bf037b3ab2d90226cd8c54f91889154cc6e465f apps/converter (v0.1.0-6-g4bf037b)
   7c8d54034871824f7783d237bfe928df95f6daf3 apps/player (v0.1.0-6-g7c8d540)
   99316cc98436793b3aee78e33e7095c74336b3e7 apps/restore (v0.1.0-6-g99316cc)
   d4c17a0d69011f43d6547022c7c17019de52c9a3 libs/decode (v0.1.0-6-gd4c17a0)
   1b1bc09c2781cbf20b20fe12681f97ec720f11fd libs/dsp (v0.1.0-5-g1b1bc09)
   7be72b4ab8addf05d5da1edaee31ce4b891b4964 libs/encode (v0.1.0-5-g7be72b4)
   e31c630c2ab93e13ed19fa3b6784aee0a2214f83 libs/engine (v0.1.0-5-ge31c630)
   538398079bdd7dd5137966bb2cc8dace0113c20b libs/output (v0.1.0-7-g5383980)
   741316f8f37aa958db9e879b6fa73f44eaf0ec91 reference/vynamp-mvp (heads/main)
```

## AC-4 — um `uses:` sem SHA reprova e nomeia o arquivo e a linha

**Este AC não estava atendido pelo código herdado do passo 34.** A mensagem era
`Unpinned action <ref> in <stem>`, com o *file stem* (`ci`) e sem linha — o AC pede
o arquivo e a linha. Corrigido neste passo:

- `action_ref_locations()` faz a mesma varredura de `action_refs()`, mas guarda o
  caminho YAML de cada referência (`jobs.specs.steps.0.uses`).
- `uses_lines()` lista as linhas de `uses:` da ordem do documento, para que a
  n-ésima referência case com a n-ésima linha. Procurar pelo texto da ação daria a
  todas as ocorrências repetidas a linha da primeira — errado exatamente no caso que
  motivou o AC.
- A mensagem passou a ser `Unpinned action <ref> at ci.yml:<linha> (<caminho>)`.

Um run real com uma ação sem SHA no fixture:

```
Unpinned action actions/checkout@v6 at ci.yml:11 (jobs.specs.steps.0.uses)
Unpinned action actions/checkout@v6 at ci.yml:26 (jobs.tooling.steps.0.uses)
```

Dois testes cobrem isso: `github_actions_names_the_file_and_the_line_of_the_unpinned_action`
(exige que cada ocorrência repetida receba **sua** linha) e
`action_ref_locations_report_the_key_path` (exige o caminho YAML).

## O que esta TASK não fez

Não reescreveu o job de CI (isso é o passo 34, já mergeado) e não criou
`check-tasks` (é a TASK-CI-02).
