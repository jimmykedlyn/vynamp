# Tarefa complementar: ajustes no contrato, ADR de mutação e `ci docs` fora do `pre-push`

Este prompt complementa o anterior, o da integração do contrato novo (branch
`chore/claude-contrato-v2`), e **substitui** qualquer outro prompt complementar recebido
antes. Todos os ajustes no `AGENTS.md` estão aqui: o mantenedor não vai colocar outra versão
do arquivo na mão.

Vale o `AGENTS.md` novo e as mesmas regras: não mexer no Notion, não inventar, não apagar,
não mergear, GitHub só por REST, nunca `--no-verify`.

Origem das mudanças (decisões do mantenedor em 2026-11-22):

- **Skills avaliadas e não instaladas.** O mantenedor avaliou duas skills de terceiros
  (`andrej-karpathy-skills` e `unclebob/ub-mode`) e decidiu não instalar nenhuma. Ficam
  algumas ideias delas, reescritas com palavras próprias: nenhum trecho é copiado, e a
  `ub-mode` não mostra licença.
- **Teste de mutação** vira proposta de gate de CI, numa ADR.
- **`cargo xtask ci docs` sai do hook `pre-push`** e passa a valer só no CI. Essa decisão já
  foi tomada e dispensa nova aprovação, inclusive para mexer no código do `xtask`, se for
  preciso.

## Onde trabalhar

- **PR do prompt anterior ainda aberto:** use a mesma worktree
  (`.worktrees/claude/contrato-v2`) e a mesma branch. Acrescente commits e atualize o corpo
  do PR.
- **PR já mergeado:** rode `cargo xtask ci docs` como baseline e depois
  `git worktree add -b chore/claude-contrato-ajustes .worktrees/claude/contrato-ajustes develop`.
  Abra um PR novo.
- **Prompt anterior ainda não rodou:** pare e avise.

## Parte A — `AGENTS.md`

1. **§3 (Código): regras de mudança cirúrgica.** Confira se os itens **"Mudança
   cirúrgica"** e **"Nada especulativo"** já estão no §3. O mantenedor pode ter colocado uma
   versão anterior, sem eles. Se não estiverem, acrescente-os logo depois do item que começa
   com `- Função pública sem`:

   ```markdown
   - **Mudança cirúrgica.** Toda linha alterada no diff liga a um critério da TASK. Código
     vizinho que funciona não é "melhorado", reformatado nem refatorado de carona. Código morto
     que não é da TASK é apontado no PR, não apagado; o que a própria mudança deixou sem uso
     sai junto.
   - **Nada especulativo.** Sem abstração para uso único, sem trait ou genérico com um só
     implementador, sem opção, flag ou configuração que a spec não pede. Quando a spec pedir,
     entra pela spec.
   ```

2. **§3 (Código): quatro regras novas.** Logo depois do item "Nada especulativo",
   acrescente exatamente:

   ```markdown
   - **Idempotência.** Operação que grava estado (job, conversão, restauração, migração,
     instalação de tema ou plugin) deixa o mesmo resultado se rodar duas vezes, ou de novo
     depois de um crash no meio. O teste prova isso repetindo a operação e interrompendo-a.
   - **Escritor único.** Estado mutável compartilhado entre threads tem um só escritor, e
     isso está na estrutura (posse, canal, fila), não numa convenção. Trabalho independente
     tem estado próprio.
   - **Causa raiz.** Bug primeiro vira teste que o reproduz. Guarda que só esconde o sintoma
     (`if` que engole o caso, `unwrap_or_default` para calar erro, retry cego) não é
     correção.
   - **Teste que sobrevive ao vazio não vale.** Se o teste continuaria passando com a função
     sob teste devolvendo um valor vazio ou padrão, ou sem fazer nada, ele não prova o
     critério e não conta como cobertura.
   ```

3. **§6.3, passo 5 (Revisor).** Substitua o passo 5 inteiro, seja qual for o texto atual
   dele, por:

   ```markdown
   5. **Revisor**, em contexto limpo: aponta, não corrige. Além de correção e contrato, confere
      o §3. São achados: linha do diff que não liga a um critério da TASK; abstração ou
      configuração não pedida; teste que sobrevive ao vazio; guarda que esconde sintoma;
      operação que grava estado sem prova de idempotência. BLOQUEADO → implementador, noutra
      chamada.
   ```

4. **§7.1 (Git): `pre-push`.** Logo depois de
   `- Conventional Commits, um commit por passo lógico.`, acrescente:

   ```markdown
   - O `pre-push` **não** roda `cargo xtask ci docs`. Esse gate roda no CI do umbrella e entra
     no `CI Gate`. Quem muda o umbrella roda o comando à mão antes de abrir o PR (§6.3,
     passo 2).
   ```

   Faça este passo só depois da Parte C estar concluída.

5. **`.claude/agents/revisor.md`.** Acrescente à lista de verificação do revisor os cinco
   achados do passo 3. Não mude `model` nem `effort`.

## Parte B — ADR do teste de mutação

6. **Grave a ADR.** Use o próximo número livre em `adr/` (provavelmente 021) e grave
   `adr/ADR-0MM-teste-de-mutacao-no-ci.md` com o conteúdo do fim deste prompt, trocando
   `0MM` pelo número.
   - Antes, confira as opções do `cargo-mutants` na versão atual (`--in-diff`,
     `.cargo/mutants.toml`, `exclude_re`). Se mudaram de nome, corrija na ADR e diga no
     relatório.
   - Não implemente o gate. A ADR é só a proposta.

## Parte C — `ci docs` fora do `pre-push`

7. **Descubra como o hook é instalado.** Rode `git config core.hooksPath` e procure com
   grep `pre-push` no repo inteiro: pasta de hooks, instalador no `xtask`,
   `cargo xtask doctor`, testes do `xtask`, docs. Liste o que achou antes de mexer.
8. **Primeiro garanta o gate no CI.** É obrigatório, e vem antes de tirar o hook. O contrato
   antigo dizia que o `ci.yml` do umbrella "só compila `xtask`".
   - Confira se algum workflow do umbrella roda `cargo xtask ci docs` em PR para `develop`
     e se esse job entra no `CI Gate`.
   - **Se não roda:** acrescente o job (mesma toolchain do resto do CI) e faça o `CI Gate`
     depender dele.
   - Se isso exigir segredo, permissão ou mudança de proteção de branch, **pare e
     pergunte**. Proteção de branch é do mantenedor.
9. **Tire o `cargo xtask ci docs` do `pre-push`.**
   - Se o hook faz outras coisas, remova só essa chamada.
   - Se ele só fazia isso, desregistre o hook e mova o arquivo para `_arquivo/`, com o
     motivo no `INDEX.md`. Não apague.
   - Não mexa no `pre-commit` (`xtask guard-branch`).
10. **Código do `xtask`, se houver** (instalador de hook, checagem do `doctor`, testes que
    conferem o hook):
    - TDD: primeiro o teste com o comportamento novo, provando que falha;
    - depois a mudança;
    - o teste que roda `git` usa `util::scrub_git_env`.
11. **Relato do incidente de 2026-11-12** (em `engineering/05-testes.md` ou onde estiver):
    não reescreva a história. Acrescente "(até 2026-11-22; desde então o `pre-push` não
    roda mais o `ci docs`)". Corrija qualquer instrução atual que diga que o hook roda o
    `ci docs`.
12. **Grave a ADR.** Grave `adr/ADR-0PP-ci-docs-fora-do-pre-push.md` com o número seguinte
    ao da Parte B, usando o conteúdo do fim deste prompt e trocando `0PP` pelo número.

## Parte D — fechamento

13. **`INDEX.md`.** Registre as duas ADRs.
14. **Verificação.**
    - Rode à mão `cargo xtask ci docs` (com `check-docs`), `cargo xtask doctor` e os testes
      do `xtask`.
    - Faça o push e confirme que o hook não roda mais o `ci docs`.
    - No PR, confirme que o job de docs aparece e entra no `CI Gate`.
    - Consulte o CI uma vez, sem loop de `sleep`.
15. **Commits** em pt-BR, Conventional Commits, um por passo lógico:
    - `docs(agents): acrescenta regras de mudança cirúrgica, idempotência, escritor único, causa raiz e teste vazio`
    - `chore(agents): alinha o revisor às regras novas do §3`
    - `docs(adr): propõe teste de mutação como gate de CI`
    - `ci: roda cargo xtask ci docs no CI do umbrella` (se o passo 8 precisou)
    - `chore(hooks): tira cargo xtask ci docs do pre-push`
    - `docs: registra a saída do ci docs do pre-push`
16. **PR por REST.**
    - **PR aberto:** acrescente a seção "Complemento: ajustes no contrato, ADR de mutação e
      ci docs fora do pre-push" ao corpo, com `PATCH`.
    - **PR novo:** mesmo formato do prompt anterior.
17. **Pare.** Não mergeie.

## Relatório final

- Branch, SHA completo e URL do PR.
- Que versão do `AGENTS.md` você encontrou, e se o passo 1 precisou acrescentar os itens.
- Trechos que não bateram.
- Números finais das duas ADRs e correções nas opções do `cargo-mutants`.
- Como o hook era instalado, o que mudou, e se o CI já rodava o `ci docs` ou se você
  acrescentou o job.
- Comandos com resultado, incluindo o push sem o hook.
- Pendências.

Nunca invente SHA, número de PR ou resultado de CI.

---

<arquivo caminho="adr/ADR-0MM-teste-de-mutacao-no-ci.md">
# ADR-0MM — Teste de mutação como gate de CI

- **Status:** Proposta
- **Data:** 2026-11-22
- **Decisor:** jimmykedlyn
- **Relacionadas:** AGENTS.md §3 ("Teste que sobrevive ao vazio não vale", oráculo
  independente) e §4 (TDD, evidência)

## Contexto

O contrato exige teste com oráculo independente e TDD com vermelho provado. Hoje, nada
**mede** se um teste de fato detecta erro: o vermelho prova que o teste falhou uma vez, não
que ele pegaria a próxima regressão. Com teste escrito por agente, o risco concreto é o
teste que espelha a implementação, ou que continua passando com a função devolvendo vazio.

O teste de mutação mede isso de forma determinística: altera o código (troca operador,
devolve valor padrão, remove chamada) e confere se algum teste falha. Mutante que sobrevive
é comportamento que nenhum teste vigia.

## Decisão

1. Os repos de lib e de app rodam `cargo-mutants` no CI de PR, **só sobre o código alterado
   no PR** (`--in-diff` com o diff contra a base).
2. Mutante sobrevivente em linha alterada reprova o PR. Mutante com timeout é reportado e
   não reprova.
3. Exceção (mutante equivalente, código de FFI, callback de tempo real medido por outro
   meio) vai em `.cargo/mutants.toml` (`exclude_re`), com um comentário que diga o motivo,
   como no `#[allow]`. Exceção sem motivo é achado de review.
4. Uma rodada completa, fora do diff, roda agendada (semanal) e só reporta. Não bloqueia
   nada.
5. A versão do `cargo-mutants` fica fixada no CI.
6. A implementação vira TASK própria, no `backlog.json` e no Notion, sob o épico de CI.
   Candidatas: `TASK-E11-04` (CI e supply chain) ou `TASK-E15-05` (testes e evidência
   operacional).

## Alternativas consideradas

- **Não ter gate; confiar no revisor.** Custa zero, mas o revisor é outro modelo lendo o
  teste e não prova nada.
- **Gate de cobertura de linha (`cargo-llvm-cov`).** É barato e conhecido, mas cobertura diz
  que a linha rodou, não que algum teste olharia o resultado. É justamente o problema do
  teste que sobrevive ao vazio.
- **Skill de agente com ferramentas próprias (`ub-mode`: `crapper`, `dryer`, `mutator`).**
  Aplica a mesma ideia, mas fica a critério do agente, fora do CI. Pelo §4, o que não está
  no CI não conta. A skill também não traz licença visível.
- **Rodada completa em todo PR.** É a mais rigorosa, mas é lenta demais: o tempo cresce com
  o número de mutantes vezes o tempo da suíte.

## Consequências

Positivas:

- Uma medida objetiva de que os testes do PR vigiam o comportamento que o PR mudou.
- O revisor ganha um dado, em vez de opinião, sobre a força dos testes.

Negativas:

- **Tempo de CI.** Cada mutante recompila e roda a suíte. Diff grande deixa o PR lento.
- **Mutantes equivalentes.** Alguns não mudam o comportamento e precisam de exceção
  justificada à mão.
- **Código de tempo real e FFI** (callback de áudio, JNI, bindings de codec) muta mal ou
  não muta. Vai para exceção e depende de outra prova (a receita de `quality/02-performance.md`).
- **Dependência nova na cadeia de CI** (`cargo-mutants`), que precisa de versão fixada e
  entra na varredura de supply chain.
- **Timeouts** podem deixar o resultado instável. Por isso timeout reporta e não reprova.
</arquivo>

<arquivo caminho="adr/ADR-0PP-ci-docs-fora-do-pre-push.md">
# ADR-0PP — `cargo xtask ci docs` sai do `pre-push`

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
2. O `cargo xtask ci docs` roda no CI do umbrella, em todo PR para `develop`, e entra no
   `CI Gate`.
3. Quem muda o umbrella roda o comando à mão antes de abrir o PR. É o passo 2 do fluxo de
   TASK (§6.3) e a verificação final dos PRs de documentação.

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
</arquivo>
