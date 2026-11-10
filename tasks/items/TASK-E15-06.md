# TASK-E15-06 — Validar NSIS/portable e assinatura Windows

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E15-06` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E15-06` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M3 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E01-10`, `TASK-E11-04` |
| Bloqueia | `TASK-E15-10`, `TASK-E16-08` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–10 dias úteis |
| Risco | alto |

## Entrega

EXE user scope + ZIP, runtime inventory e SignTool.

## Boundary

- **Não** escreve a recipe de empacotamento nem o signing do binário: `TASK-E11-07` entrega o Windows installer/ZIP com signing e symbols, e `xtask/src/package.rs` já tem a recipe `windows()`; aqui se valida o artefato que ela produz e se reprova quando o `signtool verify /pa` não fecha.
- **Não** preenche `packaging/runtime-files.json`. O inventário de DLLs nativas e o SBOM são de `TASK-E11-04`; hoje o arquivo tem `audit_complete: false` e as três listas vazias, e `dependencies()` aborta com 'Runtime inventory audit incomplete' — isso é bloqueio a registrar, não trabalho desta TASK.
- **Não** assina o `manifest.json` com minisign nem abre o draft privado: `TASK-E15-10` é a dona do source digest, do manifesto assinado e da publicação. O Authenticode de `vynamp.exe` e do setup é que é desta TASK.
- **Não** cobre associação de arquivos, ProgIDs, pacote esparso MSIX nem o uninstall desses registros: `TASK-E16-03` entrega a associação e os ícones, e `TASK-E16-08` fecha portable relocável e limpeza dos registros próprios. O `RMDir /r` de `installer.nsi` apaga só a pasta de instalação.
- **Não** prova áudio, gapless ou qualidade numérica no build instalado: `TASK-E10-08` executa soak e matriz de hardware. 'Playback sem admin' aqui é o processo subir sem elevação, não o corpus de áudio.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e15-06-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E15-06-AC-1` | uma máquina Windows 11 x64 com conta de usuário padrão, sem a entrada de desinstalação em HKCU e sem privilégio de administrador, com `vynamp-1.0.0-windows-x64-setup.exe` e `vynamp-1.0.0-windows-x64-portable.zip` no mesmo diretório | o EXE é instalado, o app é aberto e fechado, o setup da versão seguinte roda por cima, o ZIP portable é descompactado e aberto do diretório do usuário, e por fim `Uninstall.exe` é executado — tudo na mesma sessão, recusando qualquer prompt de UAC | Install/upgrade/uninstall sem admin para playback |
| `TASK-E15-06-AC-2` | um perfil com biblioteca apontando para uma pasta fora de `%LOCALAPPDATA%\Programs\Vynamp`, settings em `%APPDATA%\Vynamp` e 1 playlist com 3 faixas mais a posição de reprodução salva, todos com sha256 registrados antes | o setup de 1.0.1 sobrescreve o diretório de instalação e depois `Uninstall.exe` do menu Iniciar é executado até o fim | dados preservados |
| `TASK-E15-06-AC-3` | o PFX de release importado em `Cert:\CurrentUser\My`, `WIN_TIMESTAMP_URL` apontando para o TSA de produção e `packaging/icons/vynamp.ico` com os nove tamanhos exigidos por `branding::native_format` (16, 20, 24, 32, 40, 48, 64, 128 e 256 px) | `signtool verify /pa /all` roda sobre `vynamp.exe`, sobre cada DLL do staging e sobre `vynamp-1.0.0-windows-x64-setup.exe`, e o SHA256 recalculado de cada artefato é confrontado com `SHA256SUMS` e com `manifest.json` | binaries/installer assinados, manifesto/hash correto. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `cargo xtask package --platform windows-x64 --version 1.0.0`, depois `signtool verify /pa /all` sobre `dist/release/vynamp-1.0.0-windows-x64-setup.exe` e sobre cada DLL do staging, `unzip -l` do portable e `sha256sum -c SHA256SUMS`; o ciclo install/upgrade/uninstall numa VM Windows 11 com conta padrão e snapshot antes/depois | hoje o comando aborta em `release::preflight` com 'Release disabled: actual player/build/hardware/signing acceptance missing' — medido em 2026-11-10 com `cargo xtask release preflight --version 0.1.0-dev` — e, mesmo passando o preflight, `binary()` aborta com 'Real application binary not produced' porque `apps/desktop/` não existe e `ci::detect` devolve `core=false`; adiante `dependencies()` para em `audit_complete: false`, `licenses()` falha porque `THIRD_PARTY_NOTICES` não existe e `packaging/icons/vynamp.ico` não existe. `package.rs` não tem nenhum `#[test]`. A evidência esperada é `verify /pa` retornando 0 para 1 setup + N DLL, nenhuma linha de UAC no log da VM, `Uninstall.exe` e a pasta de instalação ausentes no snapshot final, e a playlist e a posição de reprodução com sha256 idênticos ao anterior. |
| Hardware | VM Windows 11 x64 descartável com conta de usuário padrão: ausente aqui, que é um Windows 11 Pro único e não uma VM com snapshot. O `signtool.exe` do Windows SDK está em `C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64` e roda; `makensis` não está instalado (nem no PATH nem em Program Files (x86)\NSIS) e o PFX de release não existe. Nenhum dispositivo de áudio é exigido, porque a prova é o processo subir sem elevação. | ausência é limitação registrada, não aprovação |

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
