# Passo 37 — tasks

Reescreve as 191 TASKs no formato `padroes/tasks.md`, atribui **cada tarefa** ao
módulo dono e distribui os arquivos nos repos. Cria os esqueletos das 23 specs de
módulo com a fronteira preenchida. **Não inventa critério de aceite** — esse é o
ponto do passo.

---

## 1. O que existe hoje, medido

| Fato | Número |
|---|---|
| TASKs no `backlog.json` | 191, em 19 épicos |
| TASKs com critério em Given/When/Then | **0** |
| TASKs com `Boundary` | **7** (só E19) |
| Tamanho dos arquivos | 1.176–4.899 caracteres, mediana 1.804 |
| Formato A (completo) | 144 |
| Formato B (compacto) | 30 — E16-01..10 e E17-01..20 |
| Formato C (mínimo) | 17 — E15, E19 |
| `backlog.json` sincronizado com os `.md` | sim, 191/191 |
| TASKs `in_progress` | 5 (E02-11, E11-01, E15-03, E19-01, E19-02) |
| TASKs concluídas | 0 |

```bash
# o número que importa: zero de 191 têm o critério que o próprio template exige
grep -lE 'Dado que|\bGiven\b' tasks/items/*.md | wc -l        # 0
grep -l '^## Boundary\|^## Limites' tasks/items/*.md | wc -l   # 7
```

### Correção registrada — os "30 esqueletos idênticos" não existem

`02-fusao-specs.md` §3 e o relatório da FASE 1 afirmam que os 30 arquivos de E16/E17
têm "parágrafo idêntico, zero número ou fixture". **Medi os 30: nenhum par é
idêntico.** Comparando o corpo com título e ID normalizados, os 30 caem cada um no
seu próprio grupo; todos têm entre 1.247 e 2.486 caracteres e entre 9 e 55 números
(fora os IDs). O que eles têm é o formato B — campos em prosa, sem seção de tabela
de campos e sem Given/When/Then —, exatamente como os outros 161.

A consequência prática é a mesma (todos os 191 são reescritos), mas o motivo muda:
**não há 30 arquivos a arquivar; há 191 a reescrever.** Um "reescrever ou arquivar"
teria arquivado 30 TASKs com conteúdo real — o defeito que o próprio `.md` evitou.

---

## 2. O problema, em três partes

### 2.1 A atribuição por épico está errada

`02-fusao-specs.md` §3 mapeia **épico inteiro** para um módulo. Medindo cada uma das
191 pelo `deliverable` do `backlog.json` mais o `spec` que ela cita, **49 das 191 caem
no módulo errado**:

| Épico | O mapa por épico manda para | Atribuição real (por tarefa) | Divergem |
|---|---|---|---|
| E01 (10) | `libs/engine` | engine 4, **decode 5**, **output 1** | 6 |
| E04 (10) | `libs/engine` | engine 5, **output 3**, **dsp 1**, **umbrella 1** | 5 |
| E08 (10) | `apps/converter`+`restore` | **player 8**, **engine 1**, converter 1 | 10 |
| E09 (10) | `apps/player` | **umbrella 10** — é servidor, não UI | 10 |
| E12 (10) | `libs/engine` | **player 4**, **decode 2**, engine 2, **output 1**, **umbrella 1** | 8 |
| E17 (20) | `libs/decode`+`encode` | decode 10, **player 4**, **umbrella 4**, **engine 1**, **output 1** | 10 |

Os casos que explicam o tamanho do erro, um por um:

| Tarefa | O mapa manda para | Onde ela pertence | Por quê |
|---|---|---|---|
| `E01-02` | engine | `output` | "spike shared output, ring SPSC e harness de callback" — é a porta de saída |
| `E01-01`, `-04`, `-07`, `-08`, `-09` | engine | `decode` | registry, adapters WAV/AIFF/FLAC/MP3, AAC/ALAC/Vorbis/Opus, codecs raros, DSD→PCM |
| `E04-02` | engine | `dsp` | "spike da lib própria de resampling (ADR-011)" — M9 tirou o resampler do engine |
| `E04-05/06/09` | engine | `output` | saída exclusiva, modo transparente, device profiles — são backends |
| `E04-10` | engine | umbrella | "relatório DSP/output com fixtures, driver versions e combinations" — atravessa dois módulos |
| `E08-01` | converter+restore | `engine` | scheduler com checkpoints e leases, usado pelos três apps; app não pode depender de app |
| `E08-02` | converter+restore | `converter` | é o conversor, não o restore |
| `E08-03..10` | converter+restore | `player` | loudness/RG/BPM, rename, sleep timer, SMTC, CLI+pipe, scrobble, audiobooks, API remota — spec `library/02`, que é do player |
| `E09` inteiro | player | umbrella | OpenAPI, OIDC, ingest, CDN, operação: é uma plataforma de servidor, e nenhum dos 10 repos a hospeda (D-37-5) |
| `E12-01/02/08/09` | engine | `player` | Linux/MPRIS, macOS/CoreAudio, DASH/WebDAV/clouds, casting — tudo é o app |
| `E12-04` | engine | `output` | DSD nativo/DoP e capacidades de taxa são do backend |
| `E12-05/06` | engine | `decode` | libopenmpt/SF2 e adapter de CD são decodificação |
| `E12-03` | engine | umbrella | matriz ARM64 — não pertence a módulo nenhum |
| `E17-07` | decode+encode | `engine` | guard true-peak **depois** do processamento e do resampler |
| `E17-08`, `-12`, `-18`, `-20` | decode+encode | umbrella | oráculo no PCM final, manifest por artefato, fixtures cross-módulo, gate combinado |
| `E17-06` | decode+encode | `player` | "tags técnicas, capítulos e sidecars" — `decode` declara que **não faz metadata** (C12); tags são da biblioteca do player |
| `E17-09/10/11` | decode+encode | `player` | visualizers e proxy de rede são da UI/rede do app |
| `E17-19` | decode+encode | `output` | capacidades Bluetooth e headroom por rota são da porta de saída |

A atribuição por tarefa está em §3 e **substitui** a tabela de §3 de `02-fusao-specs.md`.

### 2.2 `vynamp-encode` e `vynamp-dsp` não têm TASK nenhuma

Duas das cinco libs não têm uma tarefa que entregue o seu núcleo. `vynamp-encode`
aparece só como consequência de `E08-02` ("a lib própria `vynamp-encode` como única
fonte de codificadores") e de `E19-04` (criar o repo). `vynamp-dsp` aparece só como
consequência de `E04-02` (spike) e `E19-07` (integrar).

Isso é o inverso do SDD: a lib nasce como efeito colateral de uma tarefa de app. **Não
crio TASKs novas neste passo** — o número 191 é decisão do mantenedor. Registro como
pendência D-37-6, com a proposta, para decisão na aprovação da FASE 2.

### 2.3 A spec de módulo que a TASK cita não existe

O template exige `| Critérios da spec | <MOD>-AC-01 |`, e nenhuma das 23 specs de
módulo existe. `02-fusao-specs.md` §2 manda cada uma nascer do `docs/audio/*`
correspondente, e o passo 30 **arquiva** esses arquivos em `_arquivo/repo/docs/audio/`
com o motivo "vira spec de libs/engine". Nenhum passo faz a conversão.

Sem spec, o critério de aceite da TASK não tem de onde citar. Escrever o critério
assim mesmo seria fabricar a autoridade que o SDD exige — o mesmo defeito que a
auditoria encontrou, com outra roupa.

**Decisão:** este passo cria o **esqueleto** de cada spec — propósito, fronteira e fora
de escopo, que já estão decididos em `01-arvore-final.md` §2 — e deixa §3 a §10 como
alvo de autoria. O critério de aceite da TASK só é preenchido depois que a spec tem
§5. A coluna `Then` recebe, na geração, o texto de `acceptance` do `backlog.json`
**literal** — esse texto já é requisito aprovado em `product/03-requisitos.md`, não
invenção. As colunas `Given` e `When` ficam marcadas para autoria, porque exigem
julgar o que é observável de fora.

O portão é de máquina: `backlog.json` ganha `acceptance_ready: false`, e nenhuma TASK
pode sair de `planned` com esse campo falso. CI (`check-tasks`, TASK-CI-02) recusa.

---

## 3. Atribuição por tarefa

Legenda de módulo dono: `vynamp` = umbrella. Toda linha soma exatamente o total do
épico; a tabela fecha em 191, o número do backlog.

| Épico | `vynamp` | `player` | `decode` | `output` | `engine` | `dsp` | `converter` | `restore` | `encode` |
|---|---|---|---|---|---|---|---|---|---|
| E01 (10) | | | 01, 04, 07, 08, 09 | 02 | 03, 05, 06, 10 | | | | |
| E02 (16) | | 01–16 | | | | | | | |
| E03 (10) | | 01–10 | | | | | | | |
| E04 (10) | 10 | | | 05, 06, 09 | 01, 03, 04, 07, 08 | 02 | | | |
| E05 (10) | | 01–10 | | | | | | | |
| E06 (10) | | 01–10 | | | | | | | |
| E07 (10) | | 01–10 | | | | | | | |
| E08 (10) | | 03–10 | | | 01 | | 02 | | |
| E09 (10) | 01–10 | | | | | | | | |
| E10 (10) | 01–10 | | | | | | | | |
| E11 (10) | 01–10 | | | | | | | | |
| E12 (10) | 03 | 01, 02, 08, 09 | 05, 06 | 04 | 07, 10 | | | | |
| E13 (10) | | 01–10 | | | | | | | |
| E14 (6) | 01–06 | | | | | | | | |
| E15 (10) | 01–10 | | | | | | | | |
| E16 (10) | | 01–10 | | | | | | | |
| E17 (20) | 08, 12, 18, 20 | 06, 09, 10, 11 | 01–05, 13–17 | 19 | 07 | | | | |
| E18 (2) | | | | | | | 02 | 01 | |
| E19 (7) | 01, 02, 04, 05, 06 | | | | 03, 07 | | | | |
| **Total** | **57** | **92** | **17** | **6** | **15** | **1** | **2** | **1** | **0** |

A soma é 57 + 92 + 17 + 6 + 15 + 1 + 2 + 1 = **191**. O script aborta antes de escrever
se a atribuição não fechar nesse número.

### 3.1 As duas TASKs de engenharia que este passo emite além do backlog

O `backlog.json` tem 191 TASKs de produto. Duas TASKs de engenharia já foram
declaradas em outros passos e **não existem em arquivo nenhum**:

| ID | Declarada em | Dono | O que é |
|---|---|---|---|
| `TASK-CI-01` | `34-ci.md` §7, verbatim: "Isso vira **TASK-CI-01** no passo 37" | `vynamp` | rework de `delivery()` e `github_actions()` em `xtask/src/validate.rs` para a árvore do umbrella |
| `TASK-CI-02` | aqui, D-37-13 | `vynamp` | job `check-tasks` no CI + teste em `xtask`: `acceptance_ready` falso com estado ≠ `planned` reprova |

Com elas o total gerado é **193 arquivos** e o `backlog.json` passa a ter 193 entradas.
Isso **contradiz D-37-6** ("não crio TASK nova"), e a contradição é resolvida por
D-37-13: `TASK-CI-01` não é decisão deste passo, é obrigação herdada de 34-ci.md; e
`TASK-CI-02` é o portão de máquina que o próprio D-37-3 exige para não deixar
`acceptance_ready: false` sem consequência nenhuma.

O mantenedor pode recusar as duas (ver P4) — nesse caso o gate de aceite fica só no
subagente orquestrador do passo 33, que já para quando falta campo, e a contagem volta
a 191.

Comentários onde a decisão não é óbvia:

| Tarefa | Dono | Por que não o vizinho |
|---|---|---|
| `E01-01` | `decode` | é o spike do registry de decoders + ADR-002, não um spike do engine |
| `E04-10` | umbrella | o relatório é DSP **e** output: nenhum dos dois pode ser dono do teste do outro |
| `E08-01` | `engine` | três apps precisam do scheduler; app não pode depender de app. O `spec` aponta para `library/02`, que é do player, e isso está registrado como desvio (D-37-10) |
| `E12-07` | `engine` | scanner/worker VST3 roda dentro do grafo DSP, e o grafo é do engine |
| `E12-10` | `engine` | o título é "EQ64 e escala avançada"; o `Boundary` declara que a biblioteca de 1 milhão e o RTL são aceitos no player |
| `E17-06` | `player` | `decode` declara explicitamente que **não faz metadata** (C12); tags, capítulos e sidecars são da biblioteca |
| `E17-07` | `engine` | o guard é depois do processamento **e** do resampler — não é do decoder |
| `E17-19` | `output` | headroom e codec negociado são propriedades da rota de saída |
| `E19-03`, `E19-07` | `engine` | os dois são o módulo DSP interno de vinil/válvula: um dissolve o repo, o outro integra ao grafo |

---

## 4. Decisões deste passo

| # | Decisão | Por quê |
|---|---|---|
| **D-37-1** | A atribuição é **por tarefa**, não por épico. §3 deste documento substitui a tabela de §3 de `02-fusao-specs.md`. | Mapear épico inteiro joga **49** tarefas no módulo errado (medido em §2.1). A spec que a própria tarefa cita (`spec` do backlog) é o sinal mais forte, e a entrega é o desempate. |
| **D-37-2** | `Módulo dono` aceita `vynamp` (o umbrella) além de `vynamp-<nome>`. | 57 tarefas são transversais: CI, release, matriz de plataforma, validação numérica entre módulos. Não existe repo de "umbrella" além do próprio. `padroes/tasks.md` ganha essa linha. |
| **D-37-3** | Nenhum critério de aceite é inventado. `Then` recebe o texto de `acceptance` do backlog **literal**; `Given`/`When` ficam marcados para autoria. `acceptance_ready: false` marca cada uma das 191. | O backlog e `product/03-requisitos.md` são requisito aprovado. `Given` e `When` exigem julgar o que é observável sem acesso interno — isso é decisão de spec, não de script. |
| **D-37-4** | O passo cria o **esqueleto** das 23 specs de módulo — §1 propósito, §2 fronteira, §7 fora de escopo, tirados de `01-arvore-final.md` §2 — e marca §3–§6, §8–§10 para autoria. Não cria spec de produto. | A fronteira já está decidida (M3, M9) e é o que impede o módulo de crescer. O resto depende de projeto de API que ainda não existe. |
| **D-37-5** | E09 (plataforma de streaming) fica no umbrella, dono `vynamp`, fase M4, **sem repo**. | As 10 tarefas são servidor (Axum/PostgreSQL, CDN, ingest) e M11 fixou 10 repos sem plataforma. Criar um 11º repo é decisão do mantenedor; as tarefas ficam registradas e visíveis até lá. |
| **D-37-6** | `vynamp-encode` e `vynamp-dsp` **não recebem TASK de produto nova**. Fica registrada a proposta: 4 TASKs (`ENCODE-01..02`, `DSP-01..02`) ou reatribuição de `E08-02`/`E04-02`. A exceção são `TASK-CI-01` e `TASK-CI-02` (D-37-13), que não são tarefa de produto. | Mudar a contagem de 191 **de produto** é decisão do mantenedor. O passo registra a lacuna em vez de escondê-la numa tarefa de app. |
| **D-37-7** | Os 191 arquivos do backlog são **reescritos**, nenhum arquivado. | Medido: os 30 de E16/E17 não são clones; todos os 191 têm conteúdo próprio e apenas o formato muda. |
| **D-37-8** | O passo produz **9 commits locais** (um por repo com tarefas), cada um numa branch `chore/claude-tasks-37`, e **não faz push**. | Um commit só no umbrella deixaria 134 tarefas fora dos repos donos. O hook do passo 36 bloqueia commit em `develop`, então cada repo trabalha em branch própria. Push é autorização separada. |
| **D-37-9** | E16 (integração de shell) pertence a `vynamp-player`, com `Boundary` explícito: os verbos "Converter" e "Restaurar" são **registrados** pelo player e **executados** pelos apps donos. | É a única app com perfil e lock (`E16-02`). O custo é o acoplamento na declaração de verbos, registrado no Boundary de cada TASK. |
| **D-37-10** | E08-01 (jobs persistidos) vai para `libs/engine` como módulo de infraestrutura. | Os três apps precisam do scheduler e app não pode depender de app. O custo é o engine ganhar um módulo não-audio, registrado na spec. |
| **D-37-11** | Os índices de épico ficam **no umbrella** (`tasks/epics/`), não no repo de cada módulo. Cada módulo ganha um `tasks/README.md` gerado com os IDs que possui. | E12, E08, E17 e E18 atravessam módulos; duplicar o índice criaria 4 versões divergentes. O mapa de destino (`tasks/epics/*`) fica superseded. |
| **D-37-12** | O estado de cada TASK é copiado do `backlog.json` como está, inclusive as 5 `in_progress`. Nenhuma é promovida. | `STATE.md` marcava ✅ em tarefas sem critério; o backlog é a fonte. Promover estado aqui seria repetir o erro corrigido em C11. |
| **D-37-13** | O passo emite **duas** TASKs de engenharia além das 191: `TASK-CI-01` (herdada de `34-ci.md` §7, que a delega expressamente a este passo) e `TASK-CI-02` (job `check-tasks` + teste em `xtask`), ambas donas `vynamp`. Total: **193**. | `TASK-CI-01` existe porque 34-ci.md a prometeu e sem ela `cargo xtask check-delivery` fica vermelho para sempre, sem dono. `TASK-CI-02` é o que dá consequência mecânica a `acceptance_ready: false`; sem ele o campo é decoração. **Custo se errado:** a contagem de TASKs muda de 191 para 193 e o mantenedor precisa aprovar isso (P4). |
| **D-37-14** | `check-tasks` é TASK separada, não escopo estendido de `TASK-CI-01`. | `TASK-CI-01` já tem escopo fechado em `34-ci.md` (`delivery()` e `github_actions()`). Somar-lhe um job de CI seria a mesma fabricação de escopo que este passo recusa nas specs. **Custo se errado:** uma TASK a mais para o mantenedor gerenciar. |

---

## 5. Formato gerado

O script preenche o que é determinístico e marca o que não é:

```markdown
# TASK-E01-06 — Implementar gapless sample-accurate

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Não edite à mão os campos
> de estado: eles vêm do backlog. Critério de aceite: PENDENTE (ver §Critérios).

| Campo | Valor |
|---|---|
| ID | `TASK-E01-06` |
| Módulo dono | `vynamp-engine` |
| Repo | `jimmykedlyn/vynamp-engine` |
| REQ coberto | `REQ-E01-06` |
| Critérios da spec | pendente — `specs/engine.md` §5 |
| Fase | M2 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E01-05` |
| Bloqueia | `TASK-E01-10` |
| Modelo executor | Opus |
| Esforço | 5–12 dias úteis |
| Risco | alto |

## Entrega
Predecode next e trim validado de encoder delay/padding.

## Boundary
<!-- PENDENTE: o que esta TASK nao faz, mesmo parecendo relacionado -->

## Antes de começar
- [ ] Spec do módulo aprovada e `<MOD>-AC-nn` citado existe em `specs/engine.md` §5
- [ ] Worktree própria criada
- [ ] Fixtures/hardware disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

`Given` e `When` são preenchidos na autoria da spec; `Then` é o requisito já
aprovado em `product/03-requisitos.md`, copiado literal do backlog.

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E01-06-AC-1` | PENDENTE | PENDENTE | Pair FLAC/MP3/AAC válido concatena sem frames extras/perdidos |
| `TASK-E01-06-AC-2` | PENDENTE | PENDENTE | missing trim metadata não usa silêncio heurístico |

## Verificação
| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test -p vynamp-engine` | verde no CI Gate do repo |
| Oráculo independente | PENDENTE | comparação numérica, não "parece certo" |
| Hardware | PENDENTE | ausência é limitação, não aprovação |

## DoD — pronto para review
(os 7 itens do template, verbatim)

## Notas de execução
Preenchido durante a execução, não antes.
```

Regras de conversão aplicadas pelo script:

| Campo do backlog | Vira |
|---|---|
| `acceptance`, separado em `;` e `,` de nível superior | uma linha `AC-n` por cláusula, no `Then` |
| `dependencies` | `Depende de`, como IDs |
| (inverso de `dependencies`) | `Bloqueia`, calculado |
| `risk` = alto **ou** épico em {E01, E04, E12, E17-07/08} ou segurança/release | `Modelo executor` = Opus; senão Sonnet |
| `estimate_days` | `Esforço` = `<min>–<max> dias úteis` |
| `status` | `Estado`, mais `acceptance_ready: false` no backlog |

`risk` alto não implica Opus sozinho — são **137 de 191** com `risk: alto`, e marcar
todos como Opus tornaria a regra inútil. A do `AGENTS.md` §3 é o que a escolha segue:
alto risco = áudio/tempo real (E01, E04, E12, E17-07/08), concorrência (E08-01,
E11-05), segurança (E11-02, E19-06) e release (E11-06, E11-07, E14-05, E15-06..10).
Fora desses, `risk: alto` sai como Sonnet, e a TASK diz isso para o revisor conferir.

As duas TASKs de engenharia (§3.1) **não** passam por essa conversão: elas já existem
escritas à mão em `34-ci.md` §7 e em D-37-13, com critério em Given/When/Then e
`acceptance_ready: true`. O script as copia, não as gera.

---

## 6. O que este passo **não** faz

- Não escreve critério de aceite por conta própria (D-37-3).
- Não cria TASK de **produto**, nem apaga, nem promove nenhuma (D-37-6, D-37-12). As
  únicas TASKs novas são `TASK-CI-01` e `TASK-CI-02` (D-37-13).
- Não faz push, merge, tag nem abre PR (D-37-8). Os 9 commits ficam locais.
- Não mexe em visibilidade, proteção, assinatura nem plano.
- Não cria repo novo (E09 fica sem repo, D-37-5).
- Não apaga nem move os `tasks/items/` originais do repo antigo. Eles ficam intactos
  onde estão até a exclusão definitiva (passo 38) e, aqui, são **copiados** para
  `_arquivo/repo/tasks/items/` com motivo em `_arquivo/INDEX.md`. Copiar preserva a
  fonte: apagar antes da 3.8 violaria a regra de que nada é removido direto.

---

## 7. Ordem de execução

| # | Ação | Deixa o quê |
|---|---|---|
| 1 | Pre-checagem: `backlog.json` legível, 191 tarefas, repos localizados | nada mudou |
| 2 | Calcula a atribuição (tabela §3) e valida que soma 191 | mapa em memória |
| 3 | Cria `specs/<modulo>.md` (esqueleto) em cada repo de módulo | 23 specs de módulo + `specs/README.md` |
| 4 | Escreve `tasks/items/TASK-*.md` no repo dono, as 191 derivadas | 191 arquivos |
| 5 | Escreve `TASK-CI-01` e `TASK-CI-02` à mão no umbrella | 193 arquivos no total |
| 6 | Escreve `tasks/epics/E*.md` no umbrella e `tasks/README.md` por repo | 19 + 9 índices |
| 7 | Reescreve `backlog.json` com `module`, `repo` e `acceptance_ready` | 193 entradas |
| 8 | Verifica §8 e, se passar, commita em branch própria por repo | 9 commits locais |
| 9 | Relatório: o que ficou pendente de autoria, por módulo | a lista de trabalho real |

O passo 3 **antes** do 4 é deliberado: a TASK aponta para `specs/<m>.md` §5, e o
arquivo precisa existir para o link não quebrar no `xtask check-docs`.

---

## 8. Verificação

```powershell
# 1. Toda TASK está num repo dono e nenhuma se perdeu
(Get-ChildItem -Recurse -Filter 'TASK-*.md' -Path D:\Projects\vynamp |
  Where-Object { $_.FullName -notlike '*_arquivo*' }).Count   # 193

# 2. Nenhuma TASK ficou em dois repos
... | Group-Object Name | Where-Object Count -gt 1            # vazio

# 3. Toda TASK cita o próprio repo
Select-String -Path ... -Pattern 'Repo \| `jimmykedlyn/'       # 193 linhas

# 4. Nenhum critério foi inventado: Then == cláusula do backlog
#    (o script compara os dois textos e falha na diferença)

# 5. acceptance_ready falso não coexiste com estado avançado
python -c "import json;d=json.load(open('tasks/backlog.json',encoding='utf-8'));print(
  [t['id'] for t in d['tasks'] if t['status']!='planned' and not t.get('acceptance_ready',True)])"  # []

# 6. As 23 specs de módulo existem com §2 preenchida
# 7. Link de spec na TASK resolve
cargo xtask check-docs
```

O script roda 1–7 e falha em qualquer desvio. O item 4 é o mais importante: é o
único que prova que a coluna autoritativa não foi reescrita por engenharia.

O item 5 é o que hoje falharia: das 191, **5 estão `in_progress`** e todas ficam
`acceptance_ready: false`. Ou seja, `TASK-CI-02` reprova o próprio repo no primeiro
run. A saída **não** é afrouxar a regra: cada uma das 5 recebe `acceptance_ready: true`
quando a spec do seu módulo tiver §5 e o `Given`/`When` escrito. Até lá, `check-tasks`
reprova **nomeando as 5** na mensagem. Vermelho com nome é informação; verde por
exceção é o defeito que este passo corrige.

A soma por repo é conferida contra a tabela de §3 — se a atribuição mudar e a soma
não der 191, o script para antes de escrever nada.

---

## 9. Commit

Um por repo, mesma mensagem com o módulo:

```
chore(tasks): reescreve as TASKs do <modulo> no formato padroes/tasks.md (passo 37)

- atribuicao por tarefa, nao por epico (D-37-1)
- N TASKs movidas do umbrella para este repo
- Then copiado literal do backlog; Given/When marcados para autoria (D-37-3)
- specs/ esqueleto com proposito, fronteira e fora de escopo (D-37-4)
- acceptance_ready = false; nenhuma TASK pode sair de planned sem criterio
```

```
chore(tasks): reescreve as TASKs do umbrella e consolida o backlog (passo 37)

- 57 TASKs transversais com dono vynamp (D-37-2)
- 136 TASKs movidas para os repos de modulo
- TASK-CI-01 (rework de delivery/github_actions) e TASK-CI-02 (check-tasks)
  emitidas aqui (D-37-13); total 193
- backlog.json ganha module, repo e acceptance_ready
- indices de epico ficam no umbrella (D-37-11)
- nenhuma TASK de produto criada, arquivada ou promovida (D-37-6, D-37-7, D-37-12)
```

---

## 10. Risco

| Risco | Mitigação |
|---|---|
| Atribuição errada empurra a TASK para o repo errado e ela é implementada duas vezes | a tabela fica no documento e na TASK (`Módulo dono`); `/` no `Boundary` impede que a tarefa cresça para o vizinho. Reatribuir depois é um `git mv` de um arquivo |
| `acceptance_ready: false` em 191 vira permanente e o SDD trava | é o ponto: 191 critérios sem spec são o defeito. O relatório do passo 8 lista, por módulo, quantos faltam — o trabalho fica dimensionado, não escondido |
| E09 sem repo desaparece da vista | fase M4 e dono `vynamp`; `tasks/README.md` do umbrella conta as 10 e o relatório nomeia a lacuna |
| `vynamp-encode` sem tarefa continua sem produto | D-37-6 registra a proposta. Sem decisão sua, o repo nasce no passo 35 com `src/lib.rs` vazio e nenhuma TASK o reivindica |
| 9 commits locais e nenhum push viram trabalho perdido | o script lista os 9 branches no fim; o push é do mantenedor ou de um passo autorizado |
| Mover arquivo entre repos não preserva histórico | `git mv` entre repos não existe: cada repo recebe um arquivo novo. O histórico antigo fica no `vynamp-mediaplayer`, arquivado. Está declarado em M11 |
| Índice de épico duplicado | D-37-11: um só, no umbrella |
| A contagem muda de 191 para 193 sem o mantenedor perceber | §3.1 e P4 dizem o número e de onde vêm as duas; o relatório final lista as 193 por repo e nomeia as 2 de engenharia |
| `check-tasks` nasce vermelho nas 5 `in_progress` e alguém "conserta" afrouxando a regra | §8 item 5 registra o vermelho esperado e nomeia as 5; afrouxar a regra é a única correção recusada explicitamente |

---

## 11. Pendências que este passo abre (para a aprovação)

| # | Pendência | Decisão de quem |
|---|---|---|
| **P1** | `vynamp-encode` e `vynamp-dsp` sem TASK de produto (D-37-6): criar 4 TASKs novas ou reatribuir `E08-02` e `E04-02`? | mantenedor — muda a contagem de 191 |
| **P2** | E09 sem repo (D-37-5): manter no umbrella até M4, ou criar `vynamp-platform` como 11º repo? | mantenedor — muda a estrutura de 10 repos |
| **P3** | Autoria dos critérios: 191 TASKs × `Given`/`When` + 23 specs §5 é trabalho de várias sessões, por módulo. Ordem proposta: `decode` → `engine` → `player` → `output` → resto | mantenedor aprova a ordem |
| **P4** | Duas TASKs de engenharia além das 191 (D-37-13): `TASK-CI-01` é obrigação herdada de `34-ci.md`, mas `TASK-CI-02` é acréscimo meu. Aceita as duas, só a CI-01, ou nenhuma? | mantenedor — se nenhuma, `acceptance_ready` fica sem portão de máquina |
