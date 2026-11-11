# TASK-E11-03 — Resolver licenças e inventário de dependências

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E11-03` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E11-03` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M0 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E11-01` |
| Bloqueia | `TASK-E08-02`, `TASK-E09-05`, `TASK-E11-04`, `TASK-E18-02` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Inventário de codecs, fontes, assets e políticas de build/distribuição.

## Boundary

- **Não** decide licença de terceiro por conta própria. A AC-2 exige que a decisão de
  marca e de distribuição seja **registrada**, e o `Then` proíbe "alegar autorização
  inexistente" — o que não se pode provar fica como pendência declarada. Sem dono no backlog: é decisão do mantenedor, em `adr/ADR-008-marca.md`.
- **Não** troca dependência travada no lockfile. A AC-1 verifica feature e licença na
  fonte oficial; substituir o que está fixado é outra decisão. Decisão do mantenedor em `adr/ADR-011-pilha-propria.md`.
- **Não** implementa build de Android nem fixa NDK e Gradle: `TASK-E11-04` é a dona
  desses pins.
- **Não** confunde licença de codec com suporte a codec. O inventário é de codecs,
  fontes, assets e política de distribuição; suporte é do catálogo de formatos. O catálogo é `TASK-E17-01`.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e11-03-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E11-03-AC-1` | o inventário de codecs, fontes e assets do projeto | cada item tem a feature de build e a licença verificadas na fonte oficial | Verificar features FFmpeg/ASIO/encoders e licenças de fontes |
| `TASK-E11-03-AC-2` | os itens cuja licença não permite distribuição sem autorização | a decisão de marca e de distribuição é registrada | decisões de marca/licença final explícitas, sem alegar autorização inexistente. |

## Verificação
| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | leitura da licença e da feature de build de cada item na fonte oficial, confrontada com o `Cargo.lock` e com os pins de `TASK-E11-04` | cada item tem licença e feature registradas na própria fonte — a fonte oficial é a evidência, não o resumo do inventário |
| Hardware | nenhum: inventário de licença e de build não depende de dispositivo de áudio | ausência é limitação registrada, não aprovação |

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
