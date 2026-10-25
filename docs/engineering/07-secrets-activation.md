# Ativação externa e segredos

Versão 0.7.0 · 2026-10-14 · Configuração do projeto; player ainda não implementado.

Não commitar credenciais, .env, PFX/P12/JKS/P8 ou export de tokens. Plugin/account auth é por máquina; o agente não herda token de outra ferramenta automaticamente. gh auth login/setup-git na máquina autorizada, sem token colado na conversa/repo. O ambiente de preparação não acessou repo privado; config/github.json registra isso explicitamente.

Environment release-signing: WIN_PFX_BASE64/WIN_PFX_PASSWORD e WIN_TIMESTAMP_URL como variável validada; MAC_P12_BASE64/MAC_P12_PASSWORD/MAC_SIGN_IDENTITY e MAC_NOTARY_KEY_BASE64/MAC_NOTARY_KEY_ID/MAC_NOTARY_ISSUER; ANDROID_KEYSTORE_BASE64/ANDROID_STORE_PASSWORD/ANDROID_KEY_ALIAS/ANDROID_KEY_PASSWORD. Linux VYNAMP_MAINTAINER é variável pública real; appimagetool URL/hash em tools.lock. Java/SDK/build-tools pin noM0.

Environment release-publish: MINISIGN_SECRET_KEY_BASE64 de chave dedicada não interativa, protegida no store de secrets/vault; chave pública em packaging/update.pub. Guardar backup privado seguro para recuperação/rotação. Nenhuma chave é gerada/fornecida no pacote. O updater deve reconhecer key rotation; não trocar public key de release já distribuído sem plano.

Configurar responsáveis/environments e proteção de tag/branches conforme plano GitHub; scripts não instalam magicamente ambientes protegidos nem pagam certificado/contaDeveloper. Apenas mantenedor emite credentials e publica release. Primeira1.0 exige canal security/license/brand definidos, regras de distribuição e evidência.

Ativação técnica: resolver toolchains noM0, criar Cargo/Android build reais, completar runtime inventory/public key/LICENSE/notices, executar QA real em cinco alvos, criar evidência por versão e ativar application_ready/release_enabled por PR. Sem esses gates, release falha explicitamente. Logs nunca devem conter secrets decodificados; runner de signing é efêmero e não executa PR de fork.
