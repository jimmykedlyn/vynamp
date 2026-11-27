# ADR-024 — o orquestrador devolve a próxima TASK ao integrar

- **Status:** Aceita
- **Data:** 2026-11-27
- **Decisor:** jimmykedlyn

## Contexto

O fluxo do §6.3 do `AGENTS.md` terminava na entrega ao mantenedor: URL do PR, SHA
revisado, base, veredito e CI. Escolher a TASK seguinte ficava a cargo de um pedido
explícito do mantenedor ("qual a próxima task?"). Depois vinha um segundo pedido, o prompt
para a outra sessão.

A escolha já tem regra (§8.2: Onda → Prio → TASK, com as dependências em `Integrada`), e
quem acabou de integrar é quem tem o estado mais fresco do Project e do `develop`. A
sessão nova não conhece o contrato até lê-lo; por isso o prompt começa mandando ler o
`AGENTS.md` e não precisa repetir regra nenhuma.

O passo entrou no contrato pelo PR #304 (merge `47b5342`). Como muda o processo do
contrato, o §1.6 pede esta ADR.

## Decisão

1. O §6.3 ganha o passo 8, **Próxima TASK**. Quando a TASK chega a `Integrada` (merge
   feito, repin quando houver, worktree e branch removidas), o orquestrador escolhe a
   próxima pelo §8.2 e a devolve ao mantenedor.
2. Junto vai um prompt para outra sessão, num bloco de código de no máximo ~10 linhas:
   - abre mandando ler o `AGENTS.md` inteiro, que vale acima do prompt (se divergir, parar
     e avisar);
   - nomeia TASK, título, módulo e Issue;
   - traz Estado, Onda, Prio, dependências e o SHA do `develop` verificados, com data, para
     conferir de novo antes de reservar;
   - lista as TASKs que não podem rodar em paralelo e as worktrees de outras sessões.
3. O prompt não repete regra do contrato. Se não houver TASK elegível, o orquestrador diz
   isso e o porquê, em vez de mandar o prompt.
4. Devolver não é reservar: quem reserva é a sessão que receber o prompt, conferindo o
   §8.2 de novo.

## Alternativas consideradas

- **Só sob pedido (como era).** Custa duas idas e voltas por TASK e deixa a escolha sem
  rastro quando o mantenedor esquece de pedir.
- **Reservar a próxima automaticamente.** Ocuparia a claim sem sessão para trabalhar nela
  e tiraria do mantenedor a decisão de quando e onde começar.
- **Prompt longo, com as regras do fluxo.** Duplica o contrato e passa a divergir dele
  quando o §6 mudar. Ler o `AGENTS.md` já cobre essas regras.

## Consequências

Positivas:

- Uma TASK puxa a seguinte sem novo pedido, pela regra do §8.2.
- O prompt fica pequeno e sempre remete ao contrato vigente.

Negativas:

- **O estado envelhece.** Entre a entrega do prompt e o início da outra sessão, o Project
  e o `develop` podem mudar. Mitigação: o prompt traz a data e o SHA e manda conferir de
  novo antes de reservar.
- **Mais um passo no fim do fluxo,** com uma consulta a mais ao Project por TASK
  integrada.
- **Duas sessões podem receber a mesma TASK** se dois prompts saírem antes de alguém
  reservar. A claim local do `xtask` só é atômica dentro de uma máquina. Mitigação: antes
  de reservar, o orquestrador confere `Responsável` no Project (§5, retomada; §8.2;
  [ADR-023](ADR-023-andamento-no-github.md)).
