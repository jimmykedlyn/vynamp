# TASK-E14-02 — Entregar ports de arquivos, credenciais e sistema

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E14-02` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E14-02` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M1 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E14-01`, `TASK-E01-03` |
| Bloqueia | `TASK-E12-01`, `TASK-E12-02`, `TASK-E14-03`, `TASK-E16-04`, `TASK-E16-05`, `TASK-E17-19` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–8 dias úteis |
| Risco | alto |

## Entrega

Traits/adapters para Path/FD/Content URI, clocks/media controls/config/keychain por host.

## Boundary

- **Não** implementa o domínio. A AC-1 exige que a árvore de dependências seja
  inspecionada e que o domínio **não** dependa de Win32; trait e adapter são o
  contrato, e a tradução de domínio fica no `engine`, que é `TASK-E01-03`.
- **Não** guarda segredo em plaintext. A AC-3 exige que o adapter **falhe** quando o
  keychain está indisponível: fallback não é degradação aceitável nesta fronteira. A
  guarda do secret é `TASK-E11-05`, dona da config e do perfil portable.
- **Não** deixa o callback chamar UI nem operação bloqueante de OS. A AC-4 é sobre
  disciplina de callback, não sobre aparência da interface: a disciplina é
  `TASK-E01-02`, dona do contrato RT.
- **Não** cobre `converter` nem `restore`. São apps próprios, e `restore` usa `engine`
  e `dsp` direto por decisão já registrada na
  [ADR-013](../../adr/ADR-013-restore.md).

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e14-02-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E14-02-AC-1` | o crate de domínio e a árvore de dependências do workspace | a árvore é inspecionada à procura de windows, winapi ou chamada direta de plataforma | Domain não depende de Win32 |
| `TASK-E14-02-AC-2` | os adapters de permissão de arquivo, de foco de áudio e de credenciais | o domínio pede acesso por trait e o teste confere que nenhum tipo de plataforma atravessa a fronteira | grants/focus encapsulados |
| `TASK-E14-02-AC-3` | o caminho de credenciais com o keychain indisponível | o adapter falha ao obter o segredo | nenhum fallback plaintext |
| `TASK-E14-02-AC-4` | o engine na slice M1 com os adapters de cada host ligados | a mesma reprodução, com abrir, play, pause, seek e stop, roda nos cinco targets | callback não chama UI ou operação bloqueante de OS. A slice M1 abre arquivo e executa play/pause/seek/stop nativamente em Windows, Linux, Mac Intel, Mac Silicon e Android. |

## Verificação
| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `cargo tree` do crate de domínio procurando `windows`, `winapi` ou chamada direta de plataforma, mais o teste que confere que nenhum tipo de plataforma atravessa a fronteira | a árvore não contém tipo de plataforma e o teste falha se algum atravessar — leitura da árvore e do teste, não "parece certo" |
| Hardware | a AC-4 exige a slice M1 (abrir, play, pause, seek, stop) nos cinco alvos, e cada Mac exige relatório nativo independente; emulador não substitui Android físico | cada alvo precisa do seu relatório nativo; onde não há, a ausência é limitação registrada, não aprovação |

## DoD — pronto para review

- [ ] Todos os critérios de aceite com evidência
- [ ] `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` limpos
- [ ] `cargo test` verde; testes ignorados listados com motivo
- [ ] Spec do módulo atualizada se a implementação revelou algo novo
- [ ] `backlog.json` com o estado correto
- [ ] Revisão feita por contexto limpo, não pelo implementador
- [ ] PR aberto com template preenchido; CI Gate verde no SHA final
## Notas de execução

Preenchido durante a execução, não antes.
