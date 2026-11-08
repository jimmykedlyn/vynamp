# ADR-014 — Repositórios públicos, CI em GitHub Actions e vinil/válvula como DSP interno

Versão: 0.1.0 · Data: 2026-11-05 · Estado: decisão do mantenedor; especificação normativa de CI e de organização dos repositórios.

Status: aceito pelo mantenedor em 2026-11-05. **Substitui a [ADR-010](../_arquivo/repo/docs/architecture/adr/ADR-010-woodpecker-only-ci.md)** (CI/CD somente pelo Woodpecker) e revisa a linha do `vitrola` na [ADR-011](ADR-011-pilha-propria.md). Orienta [TASK-E19-01](../tasks/items/TASK-E19-01.md) a [TASK-E19-07](../tasks/items/TASK-E19-07.md).

## Contexto

A ADR-010 (2026-10-19) decidiu CI/CD exclusivamente pelo Woodpecker local, e para isso **deletou** três workflows de GitHub Actions que já existiam e funcionavam, além de reescrever o validador para **proibir** `.github/workflows`. Duas razões a sustentavam:

1. **Billing.** Em outubro de 2026, o GitHub Actions do repositório **privado** estava bloqueado por billing; nenhum job iniciava. O plano Free dá US$ 12/mês de compute a repositório privado. Esse bloqueio **não** é permanente nem determinístico: em 2026-11-05 o mesmo repositório privado executou o `ci.yml` com sucesso no PR #30 (run `42089094392`, todos os jobs com steps reais). A decisão de tornar os repositórios públicos não depende mais de billing — ela se sustenta na gratuidade de runners em repo público, na branch protection e na abertura pretendida. Ver [evidência TASK-E19-01](../evidence/PR/task-e19-01/README.md).
2. **Branch protection.** `rulesets` e a API de proteção retornavam HTTP 403 "Upgrade to GitHub Pro" no repositório privado, então o `CI Gate` publicado pelo Woodpecker não podia ser exigido pelo GitHub.

Em 2026-11-05 o mantenedor decidiu tornar os repositórios Vynamp **públicos**. Isso derruba as duas razões de uma vez:

- **Runners hospedados do GitHub são gratuitos em repositórios públicos.** A cobrança que bloqueava o Actions era do repositório privado; em repo público, `ubuntu-*`, `windows-*` e `macos-*` padrão não consomem minutos pagos.
- **Branch protection volta a existir no plano gratuito** em repositório público. O 403 era limite do plano **privado**, não da conta.

Há ainda um ganho de **cobertura** que a ADR-010 registrava como limitação:

3. **macOS deixa de ser "não verificado".** O Woodpecker local não tinha agente Mac, e por isso `gate.yaml` imprimia `NOT VERIFIED - macOS Intel and macOS Apple Silicon`. O GitHub oferece `macos-15-intel` (Intel x86_64) e `macos-latest`/`macos-15`/`macos-26` (Apple Silicon arm64). Os cinco alvos obrigatórios passam a ter runner hospedado.

Em paralelo, o mantenedor corrigiu um erro de modelagem sobre os efeitos de vinil e válvula: *"os Tube e Vinil são recursos do DSP dentro da aplicação, não são libs e nem repositórios externos"*. O `vitrola` era tratado como biblioteca em repositório próprio, o que criava um sexto repositório, uma dependência git com SHA fixo e uma spec de biblioteca — para código que só existe dentro do player.

## Decisão

### 1. CI/CD em GitHub Actions, e só quando necessário

Os três workflows são **restaurados** do histórico Git (commit `247b424`, "configure Vynamp Rust tooling and delivery"), não escritos do zero, e então adaptados. O commit `e7562fd` (TASK-E15-04) apagou exatamente esses arquivos; a restauração preserva o desenho já validado — actions fixadas por SHA completo, toolchain pinado por `rustup`, gate agregado.

| Arquivo | Trigger | Papel |
|---|---|---|
| `.github/workflows/ci.yml` | `pull_request` → `develop`; `push` em `develop`/`main`; `workflow_dispatch` | jobs `docs`, `tooling`, `core`, `android`, e o job agregado `gate` |
| `.github/workflows/release.yml` | `push` em tag `v*` | preflight → installers → draft; **falha fechado** antes de publicar |
| `.github/workflows/quality.yml` | `workflow_dispatch` | qualidade em hardware; permanece desabilitado até existirem runners de hardware |

**"Só quando realmente necessário"** é honrado pela restrição de gatilho: push em branch de feature **não** dispara workflow — o PR já cobre a mudança. Não há cron, não há workflow de branch isolada.

O contexto exigido continua se chamando **`CI Gate`**, agora publicado por um job do Actions em vez do servidor Woodpecker. `config/github.json` mantém `required_contexts: ["CI Gate"]` sem alteração, e `xtask ci gate` continua lendo `DOCS`/`CORE`/`HAS_CORE`/`ANDROID`/`HAS_ANDROID` do ambiente — os mesmos nomes que o workflow histórico exporta.

**Fail-closed é regra, não preferência.** O job `gate` roda com `if: always()` e falha se qualquer dependência estiver `failure` ou `cancelled`. Release só dispara em tag e termina em falha explícita antes de qualquer artefato assinado — o comportamento que `release.yaml` do Woodpecker já tinha (`exit 1`), preservado.

**macOS entra na matriz** (`macos-15-intel`, `macos-15`), removendo a limitação registrada na ADR-010. A documentação de CI não pode continuar declarando "macOS não verificado" depois desta ADR.

### 2. Repositórios públicos

Sete repositórios passam a públicos: `vynamp-mediaplayer`, `vynamp-decode`, `vynamp-output`, `vynamp-mvp-reference`, `vynamp-encode`, `vynamp-converter` e `vynamp-restore`.

Duas obrigações decorrem disso e **não são opcionais**:

- **Branch protection imediatamente após cada abertura.** Repositório público aceita PR de fork; sem `required_status_checks` o `CI Gate` não barra nada. A abertura também desativa push rulesets (consequência documentada pelo GitHub), então a proteção tem de ser reaplicada **depois** da abertura, não antes.
- **Conteúdo antes da visibilidade.** Nenhum repositório nasce público sem LICENSE, SECURITY.md, README e CI funcionando. Mudança de visibilidade privado→público é de graça; público→privado exige suporte do GitHub, ou seja, é **irreversível na prática**.

O `vitrola` **não** entra no conjunto público: é arquivado (reversível), preservando histórico e os SHAs que documentos citam.

### 3. Canal de segurança por GitHub Security Advisories

Repositório público sem canal de vulnerabilidade é inapto a uso externo, e o `SECURITY.md` atual declarava que não havia "URL/email de segurança aprovado". O canal decidido é **GitHub Security Advisories**: privado por padrão, visível apenas ao mantenedor e ao reportante, sem criar endereço novo, e com o aviso "Report a vulnerability" exibido no próprio repositório.

### 4. Vinil e válvula são módulos DSP internos

O design aprovado do `vitrola` (2026-10-23, 77 linhas, 9 seções) é **incorporado** em [reference/audio/10-vitrola.md](../reference/audio/10-vitrola.md) como módulo DSP interno do app. Consequências:

- Não há biblioteca externa, nem repositório, nem dependência git com SHA para os efeitos.
- A trait `Processor` do design continua sendo o contrato de nó; ela passa a ser interna.
- Os testes VIT-T-01..10, o `WearState`, os rituais e as regras de desativação automática (bit-perfect, DSD, multicanal) permanecem como requisitos.
- A integração no grafo vira [TASK-E19-07](../tasks/items/TASK-E19-07.md), que fecha a pendência que `10-vitrola.md` §6 registrava como "criar uma TASK de integração".

A linha do `vitrola` na tabela da [ADR-011](ADR-011-pilha-propria.md) deixa de ser um repositório e passa a apontar para o módulo interno. A ADR-011 continua válida para `vynamp-decode` e `vynamp-output`.

### 5. Restore e Converter com repositórios próprios

Decisão do mantenedor em 2026-11-05: `vynamp-converter` e `vynamp-restore` têm repositórios próprios, somando-se a `vynamp-encode` como biblioteca de codificadores. A **fonte normativa** das duas specs permanece no repositório principal — é onde a matriz de requisitos, os QA e as evidências vivem — e cada repositório novo aponta para ela por URL e SHA.

Duplicar a spec em dois repositórios criaria divergência silenciosa entre arquivos que nenhum validador reconcilia; por isso a regra é fonte única com referência explícita.

### 6. Validador e `xtask` acompanham a decisão

`cargo xtask check-delivery` **deixa de proibir** `.github/workflows` e passa a exigi-lo, validando o que a ADR-010 validava num vocabulário diferente (actions fixadas, gate dependente de todos os jobs, release só em tag e fail-closed) e abandonando as regras que só existiam por causa do Woodpecker (`privileged`, `volumes`, `network_mode`, `image == "pwsh"`). `xtask/src/release.rs` volta a validar a tag por `GITHUB_REF`, revertendo `woodpecker_tag()`. `xtask/src/repo.rs` passa a exigir visibilidade pública em vez de privada.

## Alternativas consideradas

1. **Manter Woodpecker e tornar os repositórios públicos apenas por causa da licença.**
   *Rejeitado.* Manteria a limitação de macOS, exigiria que o notebook do mantenedor estivesse ligado para haver CI, e o `CI Gate` continuaria sem poder ser exigido pelo GitHub.
2. **Usar GitHub Actions mantendo os repositórios privados.**
   *Rejeitado por dois motivos independentes.* O compute pago do plano Free (US$ 12/mês) não sustenta com folga a matriz de cinco alvos por PR, com picos de uso esgotando a cota e bloqueando o CI sem aviso — o que já aconteceu em outubro de 2026. E a branch protection continuaria em 403 no repositório privado, então o `CI Gate` seguiria sem poder ser exigido. Note que Actions **funciona** hoje em repo privado (ver Contexto): o argumento não é que ele não rode, é que não há garantia de cota nem de proteção.
3. **Escrever os workflows do zero em vez de restaurar `247b424`.**
   *Rejeitado.* O desenho já existia, já passava no validador da época e usa actions fixadas por SHA. Reescrever descartaria trabalho validado e reintroduziria risco de pin por tag flutuante.
4. **Manter o `vitrola` como biblioteca e apenas deixá-la sem repositório público.**
   *Rejeitado pelo mantenedor.* Os efeitos só existem dentro do app; uma biblioteca separada cria contrato público, versionamento e publicação para código que ninguém consome.
5. **Deletar o repositório `vitrola`.**
   *Rejeitado.* Documentos citam SHAs daquele repositório, e deletar quebra a rastreabilidade. Arquivar é reversível e preserva o histórico.
6. **Duplicar as specs de Restore e Converter dentro dos repos novos.**
   *Rejeitado.* Duas cópias divergem sem aviso; a matriz de requisitos e os QA vivem no repo principal, então ele detém a fonte normativa.

## Consequências

- `.woodpecker/` e `infra/ci/` saem do repositório principal. Os pins de **toolchain** já tinham fonte fora delas — `rust-toolchain.toml` (canal `1.99.0`, componentes `rustfmt`/`clippy`) e `config/toolchains.json` —, e continuam intactos. O que sai junto com `infra/ci/woodpecker/versions.json` são os pins das **imagens do servidor Woodpecker**, de `plugin-git` e de `cloudflared`: são operacionais, só fazem sentido enquanto o Woodpecker local existir, e seguem recuperáveis no histórico Git se a desmontagem precisar deles.
- O `CI Gate` passa a depender da disponibilidade do GitHub, não do notebook do mantenedor — ganho operacional, e perda da possibilidade de rodar CI offline.
- A matriz de CI fica maior (macOS Intel e Apple Silicon entram), então o tempo de PR cresce; a restrição de gatilho compensa parte disso ao não rodar em push de branch isolada.
- Abrir os repositórios expõe o **histórico de execuções e logs** do CI ao público. O scan de segredos em todo o histórico dos cinco repositórios existentes não encontrou token, PAT, chave AWS ou chave privada; `release.yml` referencia apenas **nomes** de secrets, nunca valores.
- A branch protection passa a ser exigível e obrigatória; `protection_status` em `config/github.json` deixa de ser `pending_plan`.
- Os efeitos de vinil e válvula passam a ser código interno: sem versão pública própria, sem API externa, sem repositório. O custo de manutenção cai; a reutilização fora do app deixa de ser possível — o que é o desenho pretendido.
- `release.yml` restaurado **não** habilita publicação de release: build/assinatura distribuída continua bloqueada e falha fechado, conforme AGENTS.md.
- A ADR-010 permanece no repositório marcada como substituída; o histórico da decisão não é apagado.
