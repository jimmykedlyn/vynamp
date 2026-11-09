# TASK-E19-06 — Abrir os repositórios ao público e proteger as branches

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. Critério de aceite: **PENDENTE** de autoria —
> a coluna `Then` é o requisito aprovado, copiado literal; `Given` e `When` ainda
> não existem.

| Campo | Valor |
|---|---|
| ID | `TASK-E19-06` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E19-06` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M3 |
| Prioridade | P0 |
| Estado | implemented |
| Depende de | `TASK-E19-01`, `TASK-E19-02`, `TASK-E19-04`, `TASK-E19-05` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 1–3 dias úteis |
| Risco | alto |

## Entrega

Sete repositórios Vynamp abertos ao público com relato privado de vulnerabilidade habilitado, e branch protection aplicada em develop exigindo o contexto CI Gate, com descriptions e wiki ajustadas.

## Boundary

<!-- PENDENTE: o que esta TASK não faz, mesmo parecendo relacionado. -->

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e19-06-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E19-06-AC-1` | os nove repositórios públicos e o histórico | a API do GitHub é lida em cada um | Os sete repos reportam visibility public e `private_vulnerability_reporting` habilitado |
| `TASK-E19-06-AC-2` | `main` e `develop` de cada repositório protegido | `gh api repos/<owner>/<name>/branches/<branch>/protection` é lido | develop exige CI Gate e proíbe force push e exclusão |
| `TASK-E19-06-AC-3` | a description de cada um dos dez repositórios | ela é comparada com o README do módulo | description não promete abertura futura |
| `TASK-E19-06-AC-4` | um repositório cuja proteção não pôde ser aplicada | `cargo xtask repo audit --repo <owner/name>` roda | repo sem proteção não é considerado pronto. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | PENDENTE | comparação numérica, não "parece certo" |
| Hardware | PENDENTE | ausência é limitação registrada, não aprovação |

## DoD — pronto para review

- [ ] Todos os critérios de aceite com evidência
- [ ] `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` limpos
- [ ] `cargo test` verde; testes ignorados listados com motivo
- [ ] Spec do módulo atualizada se a implementação revelou algo novo
- [ ] `backlog.json` com o estado correto
- [ ] Revisão feita por contexto limpo, não pelo implementador
- [ ] PR aberto com template preenchido; CI Gate verde no SHA final
## Notas de execução

Executada em 2026-11-08; evidência em
[`evidence/PR/task-e19-06/`](../../evidence/PR/task-e19-06/README.md).

- **Escopo real: dez repositórios, não sete.** O `Entrega` e o `AC-1` foram
  escritos antes do passo 35, quando existiam sete repositórios. Hoje são nove
  públicos (um umbrella, cinco bibliotecas, três aplicações) mais
  `vynamp-mediaplayer`, o repositório histórico. A TASK foi executada nos dez;
  `vynamp-mvp-reference` ficou de fora da proteção porque não tem `develop` nem
  workflow, e exigir `CI Gate` ali bloquearia todo pull request.
- **`AC-1` não estava atendido antes desta execução.** O relato privado de
  vulnerabilidade respondia `enabled=false` nos nove; foi habilitado com `PUT
  repos/<owner>/<name>/private-vulnerability-reporting`.
- **`AC-3` estava violado.** `vynamp-output` tinha a descrição
  `"... sem dependências de terceiros (privada; abertura futura)"` — a única das
  dez preenchida, e a única que prometia abertura futura.
- **`--reviews 0`, não 1.** Existe uma única conta colaboradora; com
  `enforce_admins=true`, `--reviews 1` deixaria todo PR sem merge possível.
- **Pendência que esta TASK não fecha:** os seis repositórios de módulo sem
  `LICENSE`, `SECURITY.md` e `CONTRIBUTING.md` estão públicos. É a TASK-E19-04
  (D-GH-13), e é a única parte da ADR-014 §2 que a abertura fez na ordem errada.
