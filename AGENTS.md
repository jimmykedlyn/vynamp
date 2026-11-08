# Vynamp — contrato do agente

Este é o **único** contrato de agente do projeto. Não existe segundo `AGENTS.md`, nem
`CLAUDE.md` com regras próprias: o `CLAUDE.md` da raiz apenas importa este arquivo.
Regras locais por repo de módulo só podem **acrescentar** detalhe de execução, nunca
contradizer o que está aqui.

Mantenedor: jimmykedlyn. Agente: Claude. Merge, release, visibilidade e proteção de
branch são do mantenedor.

---

## 1. Arquitetura

### 1.1 Um repositório por módulo

| Repo | Tipo | Papel |
|---|---|---|
| `vynamp` | umbrella | workspace, specs de arquitetura, ADRs, CI, agentes |
| `vynamp-dsp` | lib | primitivas de sinal, sem domínio |
| `vynamp-decode` | lib | decodificação de áudio para PCM |
| `vynamp-encode` | lib | codificação |
| `vynamp-output` | lib | saída para device, formatos, `PathProperties` |
| `vynamp-engine` | lib | integração: fila, DSP, clock, tradução de domínio |
| `vynamp-player` | app | player |
| `vynamp-converter` | app | conversor em lote |
| `vynamp-restore` | app | restauração, processo separado |
| `vynamp-mvp-reference` | referência | pin do MVP, **read-only**, fora do build |

Regra de como o umbrella vai buscar cada módulo: **submódulo fixado em tag**, nunca em
branch. Única exceção: `reference/vynamp-mvp`, pinado por **commit** (`741316f`) — não tem
`develop` nem recebe tag, porque é referência congelada e não versionada. Ele fica
**fora do workspace do Cargo**, mas **dentro** do `.gitmodules`.

Estado hoje: os submódulos ainda não existem. O `.gitmodules` é criado no passo 35
(`35-github.md`), junto com os repos de módulo; até lá, `libs/`, `apps/` e
`reference/vynamp-mvp/` são só diretórios com `LEIA-ME.md` de instrução, e o workspace
do Cargo tem só `xtask`. Verificar o estado real antes de afirmar o que existe.

### 1.2 Direção de dependência

```
apps ──→ engine ──→ { decode, output, encode, dsp }
                    │
  decode, output, encode ──→ dsp   (opcional, via feature)
```

Regras. As 2 a 6 são as que o job `arch` do CI do umbrella vai checar — ele **ainda
não existe**: nasce no passo 34 (`34-ci.md`), onde as verificações de
`01-arvore-final.md` §3 viram `cargo tree`, `grep` e `cargo build --manifest-path`.
Até lá, todas são revisadas no PR. Só a regra 1 fica fora: apps são repos próprios,
fora do build do umbrella, e quem a revisa é o PR do app.

1. `apps` dependem de `engine` (mais `std`, `dsp` e crates de UI). **Exceção
   aprovada:** `restore` usa `engine` **e** `dsp` direto — é primitivas (§1.4), não
   domínio, então a regra 3 continua valendo para ele.
2. Nenhuma **lib** depende de mais de uma lib do projeto; `engine` é a única
   exceção. (`restore` é app, não lib.)
3. `decode`, `encode`, `output` **não** dependem de nenhum crate `vynamp-*` de
   domínio. `dsp` não é domínio: é permitido, opcional, via feature.
   `dsp` **não depende de nada** além de `std`.
4. A seta entre libs do projeto é sempre `módulo → dsp`. **Nunca** `lib → lib`
   entre duas libs de domínio. A exceção é a seta para `dsp`, que é primitiva
   (§1.4) e vale para `decode`, `encode` e `output`.
5. Ninguém depende de `apps`.
6. **Cada lib compila isolada**: `cargo build --manifest-path libs/<x>/Cargo.toml`
   funciona sem o umbrella. É a prova de que o módulo não pegou carona em outro.

### 1.3 Primitivos atravessam, domínio não

`decode`, `encode` e `output` recebem e devolvem **primitivos**: `&[f32]`, `&mut [f32]`,
`u8`, `u16`, `u32`, `usize`, slices, arrays.

Tipos de domínio — `SampleRate`, `FrameCount`, `ChannelLayout`, `MediaTime`,
`SourceDescriptor` — vivem **só** no `engine`, que faz a tradução na fronteira.
Exceção única já aprovada: `PathProperties` é API pública do `output`, e é feito só
de primitivos e enums sem conhecimento de arquivo.

O custo é duplicar alguns newtypes minúsculos. O benefício é que nenhuma lib precisa
ser recompilada, nem reespecificada, quando o domínio muda.

### 1.4 `vynamp-dsp` é primitivas, não domínio

`Resampler`, `Dither`, `ChannelMap`, `Mixer`, ganho/fade, crossfade — API em primitivos,
zero conhecimento de arquivo, device ou container. É a mesma categoria de `std`: pode
ser dependência obrigatória do `engine` e **opcional** (`default-features = false`,
feature `resample`) de `decode`, `output` e `encode`.

Motivo de existir: uma diferença de 0,1 dB no downmix entre o que o player toca e o
que o conversor grava seria um bug impossível de achar.

### 1.5 Referência é read-only

`reference/vynamp-mvp/` (e o repo `vynamp-mvp-reference`) são **pinned e read-only**.
Nunca editar, nunca incluir no workspace, nunca "corrigir" o texto da referência —
se a referência diverge da spec, a spec registra a divergência e o motivo.

### 1.6 Decisão estrutural vira ADR

Mudou a direção de dependência, a fronteira de um módulo, a contagem de repos ou o
formato de uma API pública: **ADR antes do código**, com alternativas e consequências
negativas escritas. Decisão só no chat não existe.

---

## 2. DDD

- **Contexto delimitado por módulo.** A linguagem ubíqua de cada módulo vive na
  spec principal dele (`specs/<módulo>.md`); a do umbrella, em
  `architecture/GLOSSARY.md`. O mesmo conceito com nomes diferentes em dois
  módulos é aceitável e esperado; o mesmo nome com significados diferentes
  **não** é.
- **Nenhum tipo de domínio atravessa fronteira de repo.** Se precisa atravessar, é
  primitivo ou é um erro de modelagem.
- **Uma trait não é contrato universal.** `Decoder`, `Library`, `Provider`, `Theme`,
  `PluginHost` pertencem cada uma ao módulo ou app que a hospeda. Nenhuma nasce antes
  da spec do módulo dono.
- **Linguagem primeiro.** Antes de nomear um tipo, o termo entra no glossário do
  módulo com definição de uma linha. Termo sem definição é termo proibido.
- **Sem `vynamp-core`.** Não existe crate agregadora de domínio. Integração é
  responsabilidade do `engine` e de mais ninguém.

---

## 3. Clean code

- `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` são portão, não
  sugestão. Falhou, não abre PR.
- `cargo machete` limpo: dependência declarada e não usada sai antes do release.
  Não vem no toolchain; o `ci-lib.yml` dos repos de módulo roda `cargo machete ||
  echo 'ausente; pulado e listado'`, então ausência é registro, não portão.
  Dependência não usada ainda é achado de review. O `ci.yml` do umbrella não roda
  `machete` — ele só compila `xtask`.
- Erro é dado: `Result` com `thiserror` por crate. `panic!` só em invariante violada
  de forma irrecuperável, com mensagem que diz o que fazer.
- `unwrap()`/`expect()` em caminho de produção precisa de justificativa no comentário
  imediatamente acima.
- `unsafe` só com bloco `// SAFETY:` explicando o contrato mantido.
- **Callback de tempo real: sem alocação, lock, I/O, log, plugin ou JNI.** A receita
  medida para provar isso (contador por thread, região `RT-BEGIN`/`RT-END`) está em
  [quality/02-performance.md](quality/02-performance.md).
- Função pública sem documentação `///` é função não terminada.
- Nada de `todo!()` ou stub que finge funcionar. Botão desabilitado não conta como
  funcionalidade entregue.
- Nomes em inglês no código e na API pública; documentação e specs em português.
- Sem `#[allow(...)]` novo sem justificativa escrita ao lado.

---

## 4. SDD — spec driven development

**Sem código sem spec aprovada. A spec muda antes do código, nunca depois.**

Ordem obrigatória:

```
spec (requisito + critério Given/When/Then)
  → plan (passos + arquivos + verificação)
    → tasks (uma TASK, um módulo, um REQ, um modelo)
      → código (TDD: vermelho → verde → refatorar)
        → review (contexto limpo, nunca o implementador)
          → evidência no PR
```

Regras que não têm exceção:

1. **Nenhuma TASK sem critério de aceite em Given/When/Then**, observável de fora.
2. **Toda TASK pertence a exatamente um módulo.** TASK que toca dois módulos é duas
   TASKs.
3. **TDD**: o teste do critério existe e falha antes da implementação.
4. **Evidência ou não aconteceu.** O que não está no commit, no PR ou no CI não conta.
5. **CI ausente, pendente ou falho ≠ pronto para merge.** O Gate do GitHub Actions vale
   pelo SHA final, não pelo anterior.
6. **Teste ignorado aparece listado com motivo escrito.** Nunca silencioso.
7. **`tasks/backlog.json` é a fonte única de status.** Ele é versionado e viaja com
   o clone. `STATE.md` é o resumo que aponta para o backlog, atualizado a cada TASK
   entregue, e mora **fora do git**, na pasta do mantenedor: quem clona não o
   recebe, e a ausência dele não bloqueia nada.

---

## 5. Fluxo de execução (Superpowers)

**Superpowers é obrigatório.** Antes de implementar, confirmar que o plugin está
instalado e que o *skill discovery* responde — configuração em `.claude/settings.json`
habilita mas não instala. Nunca alegar uso de skill que não existe, e nunca fabricar
teste, assinatura ou resultado de CI. `cargo xtask doctor` confere o ambiente local.

Comece por [SPEC.md](SPEC.md) e [tasks/NEXT.md](tasks/NEXT.md) antes de escolher a
TASK.

Cadeia de skills, na ordem: `using-superpowers` → `spec`/`brainstorming` →
`writing-plans` → `using-git-worktrees` → TDD → `verification-before-completion` →
revisão por subagente → `finishing-a-development-branch`. Cada passo entrega
evidência observável; sem a evidência, o passo não aconteceu.

**Uma TASK, um owner.** `cargo xtask repo worktree` cria a claim atômica no Git
common dir (`.git/auralis/claims/<TASK>/owner.json`), compartilhada entre worktrees,
que evita duas sessões na mesma TASK. Nunca abrir duas sessões no mesmo diretório
ou índice Git; dependências podem ser revisadas por subagente em modo leitura.
Worktree de outra sessão não se toca. Detalhes em
[engineering/02-worktrees.md](engineering/02-worktrees.md).

As quatro definições estão em `.claude/agents/` (`orquestrador.md`,
`implementador.md`, `checador.md` e `revisor.md`), com `model:` explícito em cada uma —
nenhuma herda o modelo da sessão. Os tiers são a decisão registrada em ADR-007 e
repetida na lista abaixo: **Opus orquestra, Sonnet implementa e revisa, Haiku checa.**
TASK de alto risco — áudio/tempo real, concorrência, segurança, release — já está em
Opus na orquestração e na revisão, e o implementador daquela TASK sobe de Sonnet para
Opus trocando o `model:` da definição **antes** de chamar. A definição é a versão
executável do que segue; se as duas divergirem, a definição perde e este contrato é
corrigido no mesmo PR.

1. **Orquestrador (Opus)** lê a spec da TASK e escreve o plano curto
   (`writing-plans`). O plano é rascunho de execução, não spec, e **não é versionado**:
   mora no scratch `.superpowers/` (já no `.gitignore`), nunca no documento da TASK nem
   em `tasks/`. O que sobrevive é o commit, a evidência em `evidence/PR/<task>/` e o
   corpo do PR, onde o plano aparece resumido — não como arquivo do repositório.
2. **Worktree própria**: `cargo xtask repo worktree --agent claude --task
   TASK-E<II>-<NN> --slug <slug>` (com `--kind fix|chore|hotfix` quando não for
   feature). `--task` só aceita id existente em `tasks/backlog.json`.
3. **Implementação (Sonnet)** com TDD. Alto risco — áudio/tempo real, concorrência,
   segurança, release — sobe para **Opus**.
4. **Checks (Haiku)** devolvem só passou/falhou e o comando exato. Baseline
   obrigatório antes de mexer: `cargo xtask ci docs`.
5. **Review (contexto limpo, nunca o implementador)**: Sonnet; alto risco, Opus.
   Ele aponta e não corrige: quem corrige é outra sessão, senão o revisor passa a
   defender o próprio código.
6. **PR** com template, `--body-file`, sem duplicar PR existente.
7. **Entrega ao mantenedor**: URL do PR, SHA revisado, base, status de review e CI.

Até 2–3 TASKs independentes em paralelo. Serializar as que tocam `Cargo.toml`,
`Cargo.lock` ou índices de docs.

**Decisões já dadas pelo mantenedor não pedem aprovação de novo**, e o escopo e as
autorizações dele prevalecem sobre a leitura padrão das skills.

Mudança de comportamento atualiza junto o que a nomeia: docs, schema, backlog,
ADR e notices — mantendo IDs e links estáveis. Não quebrar referência viva sem
registrar no [INDEX.md](INDEX.md) e mover o arquivo para `_arquivo/`.

Teste precisa de **oráculo independente**: assertar que o espelho da implementação
não prova nada. Fixtures exigem hash, licença, formato e oráculo próprio. Contrato
em [engineering/05-testes.md](engineering/05-testes.md).

### Licença, marca e identidade (decisões já tomadas)

Não reabrir decisão registrada. O contrato fixa o destino de cada uma:

| Assunto | Onde está a decisão |
|---|---|
| Licença do código: `MIT OR Apache-2.0` | D-LIC-01 em [product/05-decisoes.md](product/05-decisoes.md), [LEGAL.md](LEGAL.md) |
| Nome Vynamp, marca C / Pulso, ícones | [design/04-marca.md](design/04-marca.md) |
| Nenhum código/asset/skin/plugin do Winamp | [ADR-001](adr/ADR-001-produto-independente.md) |
| Gates por OS — Android real, não só emulador | [quality/03-matriz-multiplataforma.md](quality/03-matriz-multiplataforma.md) |
| Selo hi-res só com PCM exato em hardware validado | REQ-E04-06 em [product/03-requisitos.md](product/03-requisitos.md) |
| Sem executável/installer/placeholder fictício | [delivery/01-roadmap.md](delivery/01-roadmap.md) |
| Backend web não antes de M4 | H-08 em [product/05-decisoes.md](product/05-decisoes.md) |

Criar licença nova, renomear a marca ou alterar esses números **exige decisão
registrada** — não edição de contrato.

### Git

- Gitflow: `main` (release) ← `develop` ← branch de trabalho.
- Nome de branch não é livre: o hook `pre-commit` (`xtask guard-branch`) recusa
  commit fora de `{feature,fix,chore,hotfix}/task-eNN-NN-claude-<slug>`,
  `chore/claude-<slug>` ou `release/X.Y.Z`. TASK usa a primeira forma, criada por
  `xtask repo worktree`; trabalho de fase sem TASK usa `chore/claude-<slug>`.
- Conventional Commits. Um commit por passo lógico.
- **Sem merge, tag, release, force push ou reescrita de histórico pelo agente.**
  Merge é do mantenedor, pelo GitHub, como merge commit.
- **Nunca `git reset --hard`, `git clean -fdx` nem qualquer descarte destrutivo.**
  Trabalho não commitado é de outra sessão ou do mantenedor; se precisa sumir,
  vai para `_arquivo/` ou fica para quem o criou.
- Sem bypass de hooks. Nunca.
- Antes de qualquer repo ficar público: varredura de segredos no histórico, resultado
  apresentado, e espera explícita por "pode publicar".

### API do GitHub

REST apenas. **GraphQL é instável neste ambiente** e não é usado para nada — PR,
comentário, review, label e arquivo, tudo por REST (`gh api --method PATCH` para
editar). Nunca passar token como argumento de `gh`: a credencial vem do ambiente
(`. 'D:\Projects\vynamp-media-player\AMBIENTE.ps1'` e `Invoke-VynampAuthenticatedAgent`).

### Após "merge feito"

Verificar o PR real (`state=MERGED`, base, merge commit) antes de qualquer coisa:
`gh` pode falhar por rede. Depois `git fetch origin` e fast-forward do `develop` no
checkout central limpo, confirmando `HEAD` = `origin/develop`. Worktree e branch da
TASK integrada são descartáveis; worktree de outra sessão não é. Atualizar
`STATE.md` e começar a próxima TASK.

### Entrega

Ao entregar trabalho, dizer: branch, SHA completo **revisado**, base, URL real do PR
(ou o motivo de não existir), escopo commitado, comandos com resultado, ambiente e
limitações. **Nunca inventar SHA, número de PR ou resultado de CI.** Falha de
autenticação ou rede é registrada com o erro real, não mascarada.

### Economia de tokens

Evidência vai no commit e no corpo do PR. `STATE.md` só aponta. Subagente devolve
resumo curto, não despejo de saída. Não registrar handoff longo.

---

## 6. Proibições

| Proibido | Motivo |
|---|---|
| `git push --force`, reescrever histórico | irreversível para quem já clonou |
| merge, tag, release pelo agente | decisão do mantenedor |
| alterar visibilidade, proteção ou plano do GitHub | decisão do mantenedor |
| `--no-verify` ou bypass de hook | contorna o portão |
| imprimir token, credencial ou chave | vaza em transcript e log |
| tratar todo 403 como limite de plano | o 403 de branch protection vem do repo **privado** (ADR-014); o de permissão vem de autenticação. Diagnosticar qual é antes de concluir |
| apagar arquivo direto | vai para `_arquivo/` com motivo no `INDEX.md` |
| abrir PR com teste falhando | fonte de PR que envelhece sem merge |
| editar `reference/` | referência congelada |
| declarar pronto sem evidência | a regra mais importante desta lista |
