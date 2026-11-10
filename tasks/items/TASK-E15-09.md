# TASK-E15-09 — Validar APK/AAB, SDK/NDK e identidade

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E15-09` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E15-09` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M3 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E13-09`, `TASK-E11-04` |
| Bloqueia | `TASK-E15-10` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–10 dias úteis |
| Risco | alto |

## Entrega

Android release signing, native dependencies/page-size e Play readiness.

## Boundary

- **Não** constrói o app Android nem a UI tablet/foldable: `TASK-E13-09` entrega isso. `apps/android/` não existe no repo, `ci::detect` responde `android=false` e `ci::gradle` falha antes de qualquer `apksigner`; esse bloqueio se registra, não se contorna aqui.
- **Não** fixa NDK, compile SDK, target SDK nem build-tools: `config/toolchains.json` tem os quatro valores nulos e `ci::toolchains` aborta com 'Pin android_ndk in M0 before Android builds'. O pin é de `TASK-E11-04`.
- **Não** roda o aceite Android de lifecycle, audio focus, TalkBack e energia: `TASK-E13-10` fecha o aceite Android 1.0 em hardware. Aqui se prova signer, ABI, page size e sobrevivência de grants e dados ao update.
- **Não** gera keystore nem guarda senha em arquivo Gradle: `packaging/android/signing.gradle.kts` é template que lê tudo de `System.getenv` e `Signing::provision` decodifica o `.jks` para um temporário que é apagado no `finish()`. A custódia dos secrets é de `TASK-E15-10`.
- **Não** autoriza publicação nem declara requisito atual da Play Store como cumprido: a spec diz que o pacote não inventa autorização do Google Play. A aprovação de publicação é de `TASK-E15-10`, e a política vigente de target API se valida a cada release.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e15-09-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E15-09-AC-1` | o keystore de release decodificado de `ANDROID_KEYSTORE_BASE64` para um temporário, o alias de `ANDROID_KEY_ALIAS` e os artefatos `vynamp-1.0.0-android-arm64.apk` e `.aab` produzidos por `assembleRelease` e `bundleRelease` | `apksigner verify --print-certs` roda no APK, `jarsigner -verify` no AAB, `keytool -list -v` no keystore e `keytool -printcert -jarfile` no AAB, e os quatro SHA-256 são confrontados entre si | APK/AAB signer corresponde ao keystore |
| `TASK-E15-09-AC-2` | `config/toolchains.json` com `android_ndk`, `android_compile_sdk`, `android_target_sdk` e `android_build_tools` preenchidos, e um aparelho arm64 com Android 15 onde o 1.0.0 tem 1 grant persistido de SAF e 1 playlist no banco privado | `apkanalyzer manifest min-sdk` e `apkanalyzer manifest target-sdk` leem os dois SDKs do APK, o `unzip -l` lista `lib/arm64-v8a/lib*.so` e `zipalign -c -P 16 -v 4` confirma o alinhamento de página de 16 KiB, e depois o 1.0.1 é instalado por `adb install -r` sem desinstalar antes | target SDK/NDK/ABIs validados, grants/data sobrevivem update |
| `TASK-E15-09-AC-3` | um aparelho Android arm64 físico de médio porte com 6 GiB, Android 14 ou 15, com alto-falante, fone, Bluetooth e DAC USB disponíveis | o APK de release é instalado por `adb install` e a reprodução em background com a tela apagada e uma troca de rota para Bluetooth é executada por 2 h, com `adb logcat` e `dumpsys media_session` coletados ao final | testar aparelho arm64. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `cargo xtask package --platform android-arm64 --version 1.0.0` com `ANDROID_HOME` apontando para o SDK pinado, seguido de `apksigner verify --print-certs`, `jarsigner -verify`, `keytool -list -v`, `apkanalyzer manifest target-sdk`, `unzip -l` do APK e `adb install -r` de 1.0.0 para 1.0.1 num aparelho arm64 | hoje o comando aborta já em `release::preflight` com 'Release disabled: actual player/build/hardware/signing acceptance missing' — medido em 2026-11-10 — e ainda assim o artefato não existiria: `apps/android/` não existe, sem `gradlew`, `gradlew.bat` ou `app/build.gradle.kts`, e os quatro pins de Android em `config/toolchains.json` são nulos, então `ci::toolchains` abortaria com 'Pin android_ndk in M0 before Android builds' e `ci::gradle` não teria executável. `ANDROID_HOME`, `adb` e `apksigner` não existem nesta máquina, e `fingerprint()` em `package.rs` só aceita 64 hex após o `keytool -printcert`. `package.rs` não tem `#[test]`. A evidência esperada é um único SHA-256 de 64 hex nos quatro comandos, `lib/arm64-v8a` presente no APK e no AAB, página de 16 KiB compatível, e o grant de SAF e a playlist intactos após o `adb install -r`. |
| Hardware | aparelho Android arm64 físico com 6 GiB e Android 14 ou superior: ausente neste ambiente, que é Windows 11 sem `adb`, sem `ANDROID_HOME` e sem aparelho conectado. Emulador x64 não substitui, porque a AC-3 é aparelho arm64 e a matriz de validação reserva o emulador só para automação de UI. | ausência é limitação registrada, não aprovação |

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
