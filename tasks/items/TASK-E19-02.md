# TASK-E19-02 — Higiene, governança e README bilíngue do repo principal

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. Critério de aceite: **PENDENTE** de autoria —
> a coluna `Then` é o requisito aprovado, copiado literal; `Given` e `When` ainda
> não existem.

| Campo | Valor |
|---|---|
| ID | `TASK-E19-02` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E19-02` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M1 |
| Prioridade | P0 |
| Estado | implemented |
| Depende de | `TASK-E19-01` |
| Bloqueia | `TASK-E19-06` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | médio |

## Entrega

LICENSE dual, LEGAL, CODE_OF_CONDUCT, GOVERNANCE, templates de issue, SECURITY.md com canal de advisory e README bilíngue EN/PT, com docs de CI e Gitflow reescritos para o repo público.

## Boundary

<!-- PENDENTE: o que esta TASK não faz, mesmo parecendo relacionado. -->

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e19-02-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E19-02-AC-1` | a raiz do repositório | o `README.md` é aberto | README bilíngue EN/PT na raiz |
| `TASK-E19-02-AC-2` | o `SECURITY.md` e o recurso de relato privado do repositório | os dois são comparados | SECURITY.md aponta para GitHub Security Advisories e não inventa contato |
| `TASK-E19-02-AC-3` | a raiz do repositório | os arquivos de licença são lidos | LICENSE dual presente |
| `TASK-E19-02-AC-4` | os documentos de governança e as specs | cada afirmação de capacidade ou de alvo é confrontada com a evidência | docs não afirmam capacidade ou alvo não verificado. |

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

Fechada em 2026-11-09. O AC-4 é o que justifica reabrir esta TASK para uma
varredura: ele não pede "docs bonitos", pede que nenhuma afirmação de capacidade ou
de alvo sobreviva sem evidência. Quatro não sobreviviam.

| AC | Verificação executada | Resultado |
|---|---|---|
| AC-1 | `grep -n "^# " README.md` | `# Vynamp` (linha 1) e `# Português — Vynamp` (linha 106), com os mesmos sete blocos nas duas metades |
| AC-2 | `gh api repos/jimmykedlyn/vynamp/private-vulnerability-reporting` + leitura de `SECURITY.md` | `{"enabled": true}`; o arquivo aponta para GitHub Security Advisories e não inventa e-mail nem SLA |
| AC-3 | `ls LICENSE*` | `LICENSE`, `LICENSE-MIT`, `LICENSE-APACHE`; `LICENSE` é ponteiro bilíngue, SPDX `MIT OR Apache-2.0` |
| AC-4 | varredura de afirmações de CI/release contra o código e a API | quatro corrigidas nesta passagem, listadas abaixo |

**AC-4, as quatro afirmações que não se sustentavam.** Todas nasceram antes do
passo 34 e envelheceram quando ele enxugou os workflows:

1. `adr/ADR-014-publico-dsp-interno.md` descrevia a tabela de workflows com
   `docs`, `core` e `android` no `ci.yml`, e `release.yml` como arquivo vivo.
   Nenhum desses três jobs existe desde o passo 34, e o `release.yml` foi
   arquivado. A tabela passou a descrever os jobs reais (`specs`, `tasks`,
   `tooling` com a matriz de três runners, `submodules`, `arch`, `gate`) e ganhou
   um parágrafo dizendo onde o `release.yml` mora e por quê.
2. `engineering/03-gitflow.md:23` mandava ler o preflight "no workflow
   `release.yml`", como se ele rodasse neste repositório. Passou a dizer que nenhum
   workflow de release mora aqui desde o passo 34, e que `tag-version` e
   `release preflight` continuam como ferramentas que o repo do app chama.
3. `engineering/04-ci.md:24` descrevia o passo `publish` no presente. Passou a
   atribuí-lo ao arquivo herdado em `_arquivo/workflows/release.yml`.
4. `CONTRIBUTING.md`, nos dois idiomas, dizia que os jobs `core` e `android`
   "ficam pulados por `cargo xtask ci detect`". Eles não são pulados — não
   existem. A frase virou a descrição do que o gate realmente cobre: specs,
   registros de tarefa, tooling do `xtask`, pins de submódulo e regras de direção
   de dependência, e nada sobre áudio, UI ou binário.

Nenhuma das quatro altera comportamento; todas removem uma afirmação de capacidade
que o repositório não tem. `check-docs` segue `"result": "passed"`, 193 TASKs,
`spec_version 0.7.0`.

**O que segue não verificado, e declarado como tal:** nenhum formulário de issue foi
renderizado pelo GitHub; nenhum advisory real foi aberto; a licença não passou por
advogado. Nenhum dos três é afirmação de capacidade — são limites conhecidos e
escritos.
