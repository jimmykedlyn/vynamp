# TASK-E15-07 — Validar AppImage/DEB e dependências Linux

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E15-07` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E15-07` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M3 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E12-01`, `TASK-E11-04` |
| Bloqueia | `TASK-E15-10`, `TASK-E16-08` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–10 dias úteis |
| Risco | alto |

## Entrega

AppDir/DEB, tool hash, libs/notices e manifesto assinado.

## Boundary

- **Não** implementa a porta Linux (PipeWire/PulseAudio, MPRIS, Secret Service, Orca): `TASK-E12-01` entrega isso. Sem o binário real, `binary()` de `package.rs` aborta com 'Real application binary not produced', e esse bloqueio é o que se registra aqui.
- **Não** fixa a versão nem o hash do `appimagetool`: travar versão e hash de ferramenta de terceiro é `TASK-E11-04`, dona dos pins de supply chain. `packaging/tools.lock.json` tem `url: null` e `sha256: null` e a recipe aborta em 'AppImage tool requires HTTPS URL'; escolher asset oficial e hash é pré-requisito daquela TASK, e esta não inventa o valor para destravar o build.
- **Não** produz o inventário de licenças nem o SBOM: `TASK-E11-03` é a dona das licenças de codec/fonte/asset e `TASK-E11-04` do SBOM e das dependências nativas. `THIRD_PARTY_NOTICES` não existe no repo e é entrada de `licenses()`, não entregável desta validação.
- **Não** entrega MIME, extensões de shell, ServiceMenus, MPRIS nem abertura externa: `TASK-E16-04` é a dona. O `packaging/linux/vynamp.desktop` desta TASK é o mínimo para o AppImage abrir e o DEB registrar o launcher.
- **Não** aplica assinatura Authenticode nem de chave Apple: a spec fixa minisign sobre o manifesto para Linux. A assinatura do manifesto e a chave pública pinned no updater são de `TASK-E15-10`.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e15-07-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E15-07-AC-1` | uma imagem limpa de Ubuntu 24.04 x64 (glibc 2.39, sem PipeWire, sem libasound2 instalada à mão) mais os artefatos `vynamp-1.0.0-linux-x64.AppImage` e `vynamp-1.0.0-linux-x64.deb` | o AppImage roda com `--appimage-extract-and-run` e o DEB é instalado com `apt install ./vynamp-1.0.0-linux-x64.deb` sem nenhum `apt-get install` adicional, seguido de `ldd` do `usr/bin/vynamp` extraído do AppDir | Rodar em distro baseline limpa sem libs ausentes |
| `TASK-E15-07-AC-2` | o `usr/share/doc/vynamp` do DEB e o AppDir extraído, com `LICENSE` e `THIRD_PARTY_NOTICES` esperados por `licenses()`, e o SBOM do release ao lado | cada biblioteca embarcada no AppDir e cada crate do `Cargo.lock` é confrontado, item a item, com a licença declarada no SBOM e com a entrada correspondente de `packaging/runtime-files.json` | licenças/SBOM auditados |
| `TASK-E15-07-AC-3` | `packaging/update.pub` com a chave pública pinned, o par minisign do environment e um perfil com `~/.config/vynamp` e `~/.local/share/vynamp` populados pelo 1.0.0 | `minisign -V -p packaging/update.pub -m manifest.json -x manifest.json.minisig` roda, depois o DEB 1.0.1 é instalado sobre o 1.0.0 e por fim `dpkg -r vynamp` é executado | manifesto assinado e instalação/upgrade preservam dados. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `cargo xtask package --platform linux-x64 --version 1.0.0` e, numa `ubuntu:24.04` limpa por `docker run --rm`, `apt install ./vynamp-1.0.0-linux-x64.deb`, `ldd` do binário do AppDir, `minisign -V -p packaging/update.pub -m manifest.json -x manifest.json.minisig`, e `dpkg -l vynamp` antes e depois do upgrade para 1.0.1 com `sha256sum` do settings e da biblioteca | hoje o build aborta antes de gerar qualquer artefato: `appimagetool.url` e `.sha256` nulos reprovam a recipe, `runtime-files.json` tem `audit_complete: false` com as três listas vazias e `dependencies()` para em 'Runtime inventory audit incomplete', `THIRD_PARTY_NOTICES` não existe e `minisign`, `dpkg-deb` e `appimagetool` não estão instalados nesta máquina. `package.rs` não tem `#[test]`. A evidência esperada é `ldd` sem nenhuma linha 'not found', `apt install` do DEB terminando com 0, e `dpkg -l` mostrando uma única transição 1.0.0 para 1.0.1 com settings e biblioteca inalterados por sha256. |
| Hardware | nenhum: a prova é de artefato e de imagem Linux em container, sem dispositivo de áudio e sem placa. O que falta neste ambiente é o Linux em si — é Windows, sem `dpkg-deb`, sem `appimagetool` e sem `minisign` — e o `docker` presente em `C:\Program Files\Docker\Docker\resources\bin` cobre a `ubuntu:24.04` limpa. | ausência é limitação registrada, não aprovação |

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
