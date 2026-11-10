# TASK-E11-08 — Preparar documentação de lançamento

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E11-08` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E11-08` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E11-07`, `TASK-E06-08`, `TASK-E07-08` |
| Bloqueia | `TASK-E11-09` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | médio |

## Entrega

Guia de usuário/SDK, notices, privacidade, contato real de segurança e release notes.

## Boundary

- **Não** produz nem assina os artefatos que o guia descreve: instaladores, pacotes e Universal2/APK são `TASK-E11-07`, dependência direta desta TASK.
- **Não** escreve a documentação de API, lifecycle e budgets do SDK de plugins: isso é `TASK-E07-08`, e o guia de SDK daqui referencia aquela documentação em vez de repeti-la.
- **Não** valida a galeria local e o safe mode que o guia de recuperação descreve: essa implementação é `TASK-E06-08`, e documentar não a substitui.
- **Não** decide licença de terceiro nem monta o inventário de dependências: a licença e o inventário SPDX são `TASK-E11-03`, e os notices daqui só refletem o que ele decidiu.
- **Não** executa o release candidate, o relatório de evidência nem o plano canary: isso é `TASK-E11-09`, que esta TASK bloqueia.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e11-08-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E11-08-AC-1` | o guia de usuário e de SDK escritos para a versão candidata e a árvore do app no commit da release, com config/project.json ainda em application_ready=false | cada funcionalidade descrita no guia é confrontada com o handler, comando ou schema que a implementa, e o gate de documentação roda no mesmo commit | Guia corresponde a features reais |
| `TASK-E11-08-AC-2` | o SBOM do artefato distribuído e o THIRD_PARTY_NOTICES gerados a partir do mesmo Cargo.lock do commit que produziu o artefato | os dois documentos são comparados componente a componente contra o Cargo.lock e contra a árvore nativa efetivamente empacotada | SBOM/notices coerentes |
| `TASK-E11-08-AC-3` | SECURITY.md com o canal de relato e a decisão de governança registrada em product/05-decisoes.md | um relato de vulnerabilidade é aberto pelo canal publicado e o mantenedor confirma que ele chega, é triado e recebe resposta | canal de disclosure aprovado |
| `TASK-E11-08-AC-4` | as limitações conhecidas registradas na matriz multiplataforma e no perfil de cada alvo, incluindo ausência de hardware real e recursos condicionalmente suportados | as claims de codec, hi-res e plataformas no guia e nas release notes são confrontadas com o manifest de capacidades do artefato distribuído | limitações e claims corretos. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | cargo xtask check-docs e cargo xtask check-delivery no commit do candidato, mais o SBOM gerado do Cargo.lock e confrontado com o THIRD_PARTY_NOTICES; hoje o repo não tem THIRD_PARTY_NOTICES nem evidence/release/ — e o preflight de xtask/src/release.rs exige os dois —, enquanto README.md ainda declara Status: specification | cada funcionalidade citada no guia resolve para um arquivo ou comando existente da árvore, com 0 claims sem implementação; o SBOM cobre todas as entradas do Cargo.lock e 0 entradas ficam sem campo de licença; SECURITY.md mantém GitHub Security Advisories como único canal, coerente com D-GOV-01 em product/05-decisoes.md, e nenhuma claim de codec aparece fora do manifest de capacidades. O que ainda não existe: THIRD_PARTY_NOTICES, SBOM gerado e evidência de release publicada |
| Hardware | nenhum: guia, notices, SBOM e canal de disclosure são verificados por leitura de arquivo e pelo canal do GitHub, sem abrir dispositivo de áudio; o canal real é o relato privado de vulnerabilidade do repositório, provado por um relato efetivo e não por hardware | ausência é limitação registrada, não aprovação |

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
