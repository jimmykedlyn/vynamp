# Privacidade, conteúdo e distribuição

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

Sem analytics por padrão, sem upload de biblioteca, sem fingerprint online automático. Crash reports opt-in e preview do conteúdo, redaction de paths/URLs/tokens/metadata privada. Logs locais rotativos limitados20MiB por padrão e retenção7dias; usuário pode exportar diagnóstico sanitizado. Plays/history são locais até autorizar scrobble/sync.

Contas futuras: explicitar dados coletados, finalidade, retenção, export/delete e processors; não derivar publicidade de hábitos sem decisão de produto/consentimento. Logout limpa credentials/cache privado, mantém mídia local. Deletar conta inclui sync/server data conforme política, não apaga arquivos do disco. Credenciais guardadas no vault/keychain OS, modo portable pede armazenamento seguro separado ou sessão sem persistência.

Implementação independente: não transportar código/assets do repositório Winamp; inspiração de UX e funcionalidades não transfere licença. Fonte do projeto novo ainda será escolhida; documentos deste pacote não são licença de terceiros nem autorização de distribuir conteúdo. Cada dependência e asset recebe inventário SPDX, origem, versão, license, notices e alterações. Headers SDK de terceiros não entram automaticamente.

Symphonia, native decoder/encoder, FFmpeg configuration, ASIO SDK, fontes e assets exigem checks específicos antes distribuição. Features GPL/nonfree de FFmpeg não são ativadas silenciosamente em pacote permissivo; separar builds se necessário. Patentes/termos de codec variam por implementação/território: responsável de distribuição decide antes shipping, sem alegação universal de livre uso.

Themes/plugins têm license metadata; publisher é responsável pela origem, registry mantém denúncia/revogação e notices. Conteúdo streaming próprio precisa rights/territories/availability_window e flags download/record; plataforma não publica apenas porque consegue fazer transcode. Conteúdo não autorizado não é requisito para validar player: usar fixtures autorais/públicas com licença identificada.

No release: LICENSE de código efetivamente escolhido, THIRD_PARTY_NOTICES gerado, SBOM, política de privacidade, termos de conteúdo/plataforma quando aplicável. Não criar LICENSE “MIT” agora como se tivesse sido decisão do usuário.

Android: permissões de áudio/grants selecionados somente quando necessários; negar acesso local não bloqueia rádio; sem MANAGE_EXTERNAL_STORAGE obrigatório. Tokens protegidos por Keystore. Native plugins externos e políticas de código dinâmico exigem validação de plataforma/loja; não prometer unrestricted desktop plugins no APK. LinuxSecretService indisponível oferece sessão não persistida ou armazenamento seguro escolhido, nunca fallbackplaintext automático.
