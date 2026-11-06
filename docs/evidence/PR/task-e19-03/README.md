# Evidência — TASK-E19-03 (dissolver o vitrola no DSP interno)

Data: 2026-11-05 · Autor: Claude Opus 5.5 · TASK: [TASK-E19-03](../../../../tasks/items/TASK-E19-03.md)

Registra **por que** o repositório `vitrola` foi dissolvido, **o que** mudou e **o que ficou pendente** — inclusive uma descoberta que só apareceu ao executar.

## Origem

Decisão do mantenedor em 2026-11-05, registrada na [ADR-014](../../../../docs/architecture/adr/ADR-014-public-github-actions-and-in-app-dsp.md):

> "os Tube e Vinil são recursos do DSP dentro da aplicação, não são libs e nem repositórios externos"

O `vitrola` era tratado como biblioteca em repositório próprio. Isso criava um sexto repositório, uma dependência git com SHA fixo e uma spec de biblioteca — para código que só existe dentro do player. Dissolver não é implementar: os efeitos continuam sem implementação nativa.

## A descoberta

O clone local em `D:\Projects\vitrola` estava **um commit à frente** do remoto:

| Ref | SHA | Estado |
|---|---|---|
| `origin/develop`, `origin/main` | `3d44c68` | publicado — ainda diz `agulha` (nome antigo da lib de decode) |
| `develop` local | `75c4ca2` | **não publicado** — renomeia três ocorrências de `agulha` para `vynamp-decode` |

Um único arquivo difere, em três linhas. Se o repositório fosse arquivado sem preservar o commit local, o texto sobrevivente seria o que usa o nome antigo. A cópia preservada em [vitrola-design.md](vitrola-design.md) é a de `75c4ca2` — a correta.

## O que mudou

| Arquivo | Mudança |
|---|---|
| [docs/audio/10-vitrola.md](../../../../docs/audio/10-vitrola.md) | 0.8.0. Deixa de "integrar por referência" um design externo e passa a ser **fonte única**: o design aprovado entra como §8.1–8.9, integral. Ordem das seções corrigida para o design ficar por último. §1 reescrita; §6 aponta a TASK-E19-07 em vez da pendência de "criar uma TASK"; referências internas para a numeração original (§8.1, §6, §5) re-apontadas |
| [ADR-011](../../../../docs/architecture/adr/ADR-011-own-audio-stack.md) | 0.1.1. A linha de inventário do vitrola deixa de ser repositório com SHA e passa a "módulo DSP interno (sem repositório)", apontando a ADR-014 |
| [docs/GLOSSARY.md](../../../../docs/GLOSSARY.md) | "Biblioteca própria" → módulo DSP interno, não é lib nem repo |
| [docs/audio/09-dsp-catalog.md](../../../../docs/audio/09-dsp-catalog.md) | A linha da família Vitrola dizia "sem processamento desses modelos; dependem de adaptadores" — passa a design aprovado incorporado como módulo interno, sem validação acústica executada |
| [docs/audio/03-dsp.md](../../../../docs/audio/03-dsp.md) | "módulo hospedado" → módulo interno; a linha da posição cita §8.8 e a TASK-E19-07 |
| [docs/audio/07-dsp-signal-contract.md](../../../../docs/audio/07-dsp-signal-contract.md) | 0.7.2; nota de revisão registra a mudança de embalagem e confirma que a cadeia não mudou |
| [docs/design/06-mvp-reference.md](../../../../docs/design/06-mvp-reference.md) | A regra "biblioteca nova ganha repo próprio, como vynamp-decode, vynamp-output e vitrola" ganha a exceção: o vitrola não é mais biblioteca |
| [docs/product/06-feature-parity.md](../../../../docs/product/06-feature-parity.md) | "módulo DSP interno" na linha de som avançado |
| [docs/security/02-privacy-licensing.md](../../../../docs/security/02-privacy-licensing.md) | O módulo entra no inventário de licença (MIT OR Apache-2.0), dentro do app |
| [docs/delivery/01-roadmap.md](../../../../docs/delivery/01-roadmap.md) | "TASK a criar" → TASK-E19-07; a nota de risco deixa de citar "ausência de TASK" |
| [docs/delivery/03-risks-open-questions.md](../../../../docs/delivery/03-risks-open-questions.md) | O risco D-DSP-05 aponta a integração em TASK-E19-07 |
| [docs/product/05-decisions.md](../../../../docs/product/05-decisions.md) | Referências `vitrola §N` re-apontadas para a numeração nova (§8.2/§8.4/§8.6/§8.8) com link; a linha que dizia "TASK de integração pendente" passa a apontar a TASK-E19-07 |
| [docs/quality/05-dsp-test-matrix.md](../../../../docs/quality/05-dsp-test-matrix.md) | `vitrola §5` → `vitrola §8.5` |
| [tasks/items/TASK-E19-03.md](../../../../tasks/items/TASK-E19-03.md) | 0.9.0, com a situação por critério de aceite e a descoberta do commit não publicado |
| [docs/INDEX.md](../../../../docs/INDEX.md) | Nova seção indexando esta evidência e a cópia do design |
| `CHANGELOG.md` | Entrada bilíngue EN/PT |

**Nenhuma alteração de código, dependência, workflow ou backlog.** 191 requisitos, 191 tarefas, 19 épicos.

## O que não foi feito

- **O arquivamento do repositório remoto é do mantenedor.** `gh repo archive jimmykedlyn/vitrola` está prescrito na TASK-E19-06 e na E19-03, mas o agente não executa ação de visibilidade/ciclo de vida de repositório ([AGENTS.md](../../../../AGENTS.md), ADR-014). É reversível por `gh repo unarchive`.
- **O clone local não foi removido.** `D:\Projects\vitrola` é a única cópia do commit `75c4ca2`; só pode sair depois que este PR estiver no `develop` e a cópia em `docs/evidence/` estiver versionada.
- **Nada foi implementado.** Vinil e válvula continuam sem código: a integração no grafo é a [TASK-E19-07](../../../../tasks/items/TASK-E19-07.md), que depende de E04-03.
- **A matriz VU-INT-06 não foi tocada** — não é escopo desta TASK (é da E02-07).

## Preservação do design

A incorporação **reorganiza em subseções** (as nove seções originais viraram 8.1–8.9) e faz duas correções conscientes: o nome da lib de decode (`agulha` → `vynamp-decode`) e a restrição "repositório privado" em 8.7, que deixou de valer. Nada do conteúdo técnico foi reescrito nem reinterpretado. Para que isso seja auditável e não uma alegação, o texto **literal** de `75c4ca2` está em [vitrola-design.md](vitrola-design.md) — e o arquivo diz o que é cópia e qual é a versão normativa.

## Verificação

| Comando | Resultado |
|---|---|
| `cargo xtask check-docs` | `result: passed`, `errors: []` — 357 arquivos, 191 requisitos, 191 tarefas, spec 0.7.0 |
| `cargo xtask check-delivery` | `result: passed`, `errors: []`, `workflow_files: 3` |
| `grep -rn "jimmykedlyn/vitrola" docs/ tasks/ config/ AGENTS.md` | 5 ocorrências, nenhuma em documento normativo: 3 nesta evidência (registro de origem), 1 em [vitrola-design.md](vitrola-design.md) (cópia literal do design) e 2 como o **comando de arquivamento** em [TASK-E19-03](../../../../tasks/items/TASK-E19-03.md) e [TASK-E19-06](../../../../tasks/items/TASK-E19-06.md) |
| `gh repo view jimmykedlyn/vitrola --json isArchived` | `false` — **pendente, é do mantenedor**; não executado |

## Limitações

- **Revisão independente por subagente:** o plano semanal esgotou em 2026-11-05 (HTTP 429), então a revisão de contexto limpo não roda; a conferência foi feita pelo próprio autor, critério declarado em vez de escondido.
- **A cópia literal é um `git show` do clone local**, não uma verificação de que `75c4ca2` está correto como design — o design foi aprovado em 2026-10-23; o commit posterior só renomeia.
- **Nada foi exercitado em runtime**: não há código para exercitar.
- Este PR não altera visibilidade, proteções, assinatura nem plano — isso é da TASK-E19-06, por decisão da ADR-014.
