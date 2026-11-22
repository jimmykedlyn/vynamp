# ADR-020 — Teste de mutação como gate de CI

- **Status:** Proposta
- **Data:** 2026-11-22
- **Decisor:** jimmykedlyn
- **Relacionadas:** AGENTS.md §3 ("Teste que sobrevive ao vazio não vale", oráculo independente) e §4 (TDD, evidência)

## Contexto

O contrato exige teste com oráculo independente e TDD com vermelho provado. Hoje, nada **mede** se um teste de fato detecta erro: o vermelho prova que o teste falhou uma vez, não que ele pegaria a próxima regressão. Com teste escrito por agente, o risco concreto é o teste que espelha a implementação, ou que continua passando com a função devolvendo vazio.

O teste de mutação mede isso de forma determinística: altera o código (troca operador, devolve valor padrão, remove chamada) e confere se algum teste falha. Mutante que sobrevive é comportamento que nenhum teste vigia.

## Decisão

1. Os repos de lib e de app rodam `cargo-mutants` no CI de PR, **só sobre o código alterado no PR** (`--in-diff` com o diff contra a base).
2. Mutante sobrevivente em linha alterada reprova o PR. Mutante com timeout é reportado e não reprova.
3. Exceção (mutante equivalente, código de FFI, callback de tempo real medido por outro meio) vai em `.cargo/mutants.toml` (`exclude_re`), com um comentário que diga o motivo, como no `#[allow]`. Exceção sem motivo é achado de review.
4. Uma rodada completa, fora do diff, roda agendada (semanal) e só reporta. Não bloqueia nada.
5. A versão do `cargo-mutants` fica fixada no CI.
6. A implementação vira TASK própria, no `backlog.json` e no Notion, sob o épico de CI. Candidatas: `TASK-E11-04` (CI e supply chain) ou `TASK-E15-05` (testes e evidência operacional).

## Alternativas consideradas

- **Não ter gate; confiar no revisor.** Custa zero, mas o revisor é outro modelo lendo o teste e não prova nada.
- **Gate de cobertura de linha (`cargo-llvm-cov`).** É barato e conhecido, mas cobertura diz que a linha rodou, não que algum teste olharia o resultado. É justamente o problema do teste que sobrevive ao vazio.
- **Skill de agente com ferramentas próprias (`ub-mode`: `crapper`, `dryer`, `mutator`).** Aplica a mesma ideia, mas fica a critério do agente, fora do CI. Pelo §4, o que não está no CI não conta. A skill também não traz licença visível.
- **Rodada completa em todo PR.** É a mais rigorosa, mas é lenta demais: o tempo cresce com o número de mutantes vezes o tempo da suíte.

## Consequências

Positivas:

- Uma medida objetiva de que os testes do PR vigiam o comportamento que o PR mudou.
- O revisor ganha um dado, em vez de opinião, sobre a força dos testes.

Negativas:

- **Tempo de CI.** Cada mutante recompila e roda a suíte. Diff grande deixa o PR lento.
- **Mutantes equivalentes.** Alguns não mudam o comportamento e precisam de exceção justificada à mão.
- **Código de tempo real e FFI** (callback de áudio, JNI, bindings de codec) muta mal ou não muta. Vai para exceção e depende de outra prova (a receita de `quality/02-performance.md`).
- **Dependência nova na cadeia de CI** (`cargo-mutants`), que precisa de versão fixada e entra na varredura de supply chain.
- **Timeouts** podem deixar o resultado instável. Por isso timeout reporta e não reprova.