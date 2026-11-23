# ADR-021 — `cargo xtask ci docs` sai do `pre-push`

- **Status:** Aceita
- **Data:** 2026-11-22
- **Decisor:** jimmykedlyn

## Contexto

O hook `pre-push` do umbrella rodava `cargo xtask ci docs`, que compila e testa o `xtask` e
roda os gates de documentação (`check-docs`, `check-tasks`, `check-authoring`). Três
problemas:

1. **Duplicação.** O mesmo gate tem que valer no CI, pelo SHA do PR (§4 do contrato: o que
   não está no CI não conta). No hook, ele só repetia o trabalho.
2. **Custo.** Cada push esperava o build e os testes do `xtask`, inclusive em push de
   branch de trabalho que ainda nem vai virar PR.
3. **Risco.** Dentro do `pre-push`, o Git exporta variáveis como `GIT_DIR`, e os testes do
   `xtask` que rodam `git` em repositório descartável atingiram o repositório real em
   2026-11-12: dois commits na branch em uso, índice sobrescrito, `core.bare=true` no
   `.git/config`. O `scrub_git_env` corrigiu a causa, mas o hook continuava sendo o
   ambiente mais perigoso para esses testes.

## Decisão

1. O `pre-push` não roda mais o `cargo xtask ci docs`. O `pre-commit`
   (`xtask guard-branch`) continua.
2. O `cargo xtask ci docs` roda no CI do umbrella e entra no `CI Gate`.
3. Quem muda o umbrella roda `cargo xtask ci docs` à mão antes de abrir o PR (§6.3 e a
   verificação final dos PRs de documentação). A ADR é só a proposta.

## Alternativas consideradas

- **Manter como estava.** É o gate mais cedo possível, mas tem os três problemas acima.
- **Deixar no hook só as checagens rápidas** (`check-docs`, `check-tasks`), sem build nem
  teste do `xtask`. Tira o risco e quase todo o custo, mas mantém a duplicação e exige
  separar o comando em dois. Fica como opção se o CI começar a pegar erro de docs com
  frequência.
- **Mover para o `pre-commit`.** Pior: roda a cada commit e atrasa o TDD.

## Consequências

Positivas:

- Push rápido e sem o ambiente do hook interferindo nos testes do `xtask`.
- Uma fonte só para o gate de docs: o CI, pelo SHA do PR.

Negativas:

- **Erro de documentação aparece mais tarde,** no CI do PR, e não no push. Haverá mais CI
  vermelho por link quebrado ou tabela divergente.
- **Depende de disciplina:** rodar o comando à mão antes do PR. O agente tem isso no fluxo;
  quem trabalha à mão pode esquecer.
- **Branch sem PR** deixa de ter qualquer checagem de documentação.