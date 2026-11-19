# Ativação externa e segredos

Versão 0.8.1 · 2026-11-15 · Configuração do projeto; player ainda não implementado.

Não commitar credenciais, .env, PFX/P12/JKS/P8 ou export de tokens. Plugin/account auth é por máquina; o agente não herda token de outra ferramenta automaticamente. gh auth login/setup-git na máquina autorizada, sem token colado na conversa/repo. O ambiente de preparação não acessou repo privado; config/github.json registra isso explicitamente.

## Verificação, não declaração

Duas checagens provam o que este arquivo pede, e **as duas rodam em CI** — a diferença
está dita em cada item, porque "portão" que não roda no runner não barra merge nenhum:

- **Credencial no histórico — roda em CI.** `cargo xtask check-secrets` lê todo blob
  alcançável de toda ref, não a árvore de trabalho: segredo apagado no commit seguinte
  continua no clone. Roda no job `tooling` do `ci.yml` com `git fetch --unshallow` antes,
  porque um clone raso escanearia a árvore e chamaria de limpo um histórico que ainda tem
  o token. Resultado medido em 2026-11-15: `No credential shape found in the reachable
  history.`
- **Token do workflow — roda em CI.** Todo workflow
  declara `permissions: contents: read` e nenhuma chave de permissão, no bloco de
  workflow nem no de job, tem valor diferente de `read` ou `none`. O default do GitHub
  não é seguro por omissão — em repositório antigo o token nasce read/write —, um
  `permissions:` de job substitui o do workflow inteiro, e ler só `contents` deixaria
  `id-token: write` passar. As abreviadas `write-all` e `read-all` também são recusadas:
  não são um mapa, então um gate que só percorre chaves não vê nada nelas, e `read-all`
  lê todo escopo, inclusive os que o bloco poria em `none`. O gate está em
  `xtask/src/validate.rs` (`github_actions`), com teste para o bloco de workflow, para o
  de job, para chave larga fora de `contents` e para as duas abreviadas — mais dois que
  garantem o contrário, que `none` e `{}` passam, para o gate não punir quem endurece.
  Toda action é fixada por SHA de 40 hex, conferido no mesmo gate.
  **O gate roda em CI:** o job `delivery` do `ci.yml` chama `cargo xtask check-delivery`,
  e o `pre-push` local chama `cargo xtask ci docs`, que inclui o mesmo `check-delivery`.
  Quem pôs o job no CI foi `TASK-E15-04` (commit `8d11700`), mergeada em `develop` pelo
  PR #67 (`a2f7e40`).

O que **não** está coberto: redaction de credencial em log de runtime. A varredura acha
segredo commitado, não segredo impresso. Redaction é `TASK-E11-05`.

Environment release-signing: WIN_PFX_BASE64/WIN_PFX_PASSWORD e WIN_TIMESTAMP_URL como variável validada; MAC_P12_BASE64/MAC_P12_PASSWORD/MAC_SIGN_IDENTITY e MAC_NOTARY_KEY_BASE64/MAC_NOTARY_KEY_ID/MAC_NOTARY_ISSUER; ANDROID_KEYSTORE_BASE64/ANDROID_STORE_PASSWORD/ANDROID_KEY_ALIAS/ANDROID_KEY_PASSWORD. Linux VYNAMP_MAINTAINER é variável pública real; appimagetool URL/hash em tools.lock. Java/SDK/build-tools pin noM0.

Environment release-publish: MINISIGN_SECRET_KEY_BASE64 de chave dedicada não interativa, protegida no store de secrets/vault; chave pública em packaging/update.pub. Guardar backup privado seguro para recuperação/rotação. Nenhuma chave é gerada/fornecida no pacote. O updater deve reconhecer key rotation; não trocar public key de release já distribuído sem plano.

Configurar responsáveis/environments e proteção de tag/branches conforme plano GitHub; scripts não instalam magicamente ambientes protegidos nem pagam certificado/contaDeveloper. Apenas mantenedor emite credentials e publica release. Primeira1.0 exige canal security/license/brand definidos, regras de distribuição e evidência.

Ativação técnica: resolver toolchains noM0, criar Cargo/Android build reais, completar runtime inventory/public key/LICENSE/notices, executar QA real em cinco alvos, criar evidência por versão e ativar application_ready/release_enabled por PR. Sem esses gates, release falha explicitamente. Logs nunca devem conter secrets decodificados; runner de signing é efêmero e não executa PR de fork.
