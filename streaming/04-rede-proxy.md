# Rede, proxy e bindings de streaming

Versão 0.7.0 · Complementa HTTP/ICY/HLS e o provider existente.

## Proxy M2

Perfis system/manual/direct. Default system somente onde adapter validado exista, com origem da política informada. Manual HTTP CONNECT/HTTPS proxy conforme backend e versão fixados. Credenciais no vault; nunca em playlist, logs ou export. Proxy-Authorization não chega à origem; 407 mostra configuração/cancel e retry limitado, separado de 401/403. Alterar perfil gera reconnect cancelável sem repetir áudio/jobs. PAC/WPAD/SOCKS5 são capacidades opcionais M5; PAC indisponível não cai silenciosamente para direct.

Bypass explícito por domínio/porta/CIDR para NAS escolhido pelo usuário; normalizar IDNA/IP e proteger paths privados. Redirects, manifests, segmentos e keys passam a mesma política de origem, autorização e DNS. Playlist remota não autoriza loopback/link-local/metadata endpoints. Proxy com DNS remoto exige política/allowlist verificável; não alegar proteção por IP não observado. NAS autorizado não concede toda a LAN. TLS mantém validação de certificados e trust store enterprise configurado; sem toggle global para ignorar certificados.

Falha de proxy, cancel, stall, vault indisponível e offline têm erros distintos. Credenciais provider/proxy/origem usam namespaces separados. Direct é escolha explícita conforme política, não fallback em erro. Tela técnica informa perfil em uso; playback normal não mostra detalhes de API.

## Transporte, container e codec/perfil

ABR escolhe tuple verificado: HTTP/ICY/HLS/DASH, container/framing, codec/profile/config, canais/layout/DRC e direito. HLS AAC-LC/HE em TS/fMP4/ADTS exige bindings específicos; FLAC/Opus em HLS não são universais. M3 testa init segment, mudança de configuração, sequence/discontinuity/clock, limites UTF-8 e trim. LOAS/LATM/ADIF/AOT42 não são substituídos por decoder LC. Throughput alto não habilita perfil indisponível; seleção manual dá erro/alternativa explícita.

DASH M5 cobre adaptation sets, idioma, init e indicação DRM. Widevine/FairPlay/PlayReady permanecem fora de suporte comercial até RFC/licenciamento; não remover proteção. HLS AES-128 autorizado é outro recurso, com keys/grants/TTL e redaction. Renderer imersivo/passthrough exige capacidade própria, não troca de URL.

Range 200/206/416, signed URL expiry, ETag, cache volátil versus download e purge de conta seguem contratos existentes. Probe e Content-Disposition são untrusted; basename sanitizado, tamanho/tempo limitados, sem overwrite de source e validação antes de finalizar. MP4 com moov ao fim ou sem ranges informa limites progressivos. Vídeo em container é descartado; só seleciona faixa de áudio comprovada, sem decode de vídeo em background.

## QA-029/025

Servidor/proxy local: 407/CONNECT/TLS, trust enterprise autorizado, stall/cancel, isolamento de credenciais, bypass NAS, IPv6/IDNA/DNS rebinding e DNS remoto. Filhos maliciosos, OAuth refresh, HLS init/config/codec desconhecido e ICY parcial. Medir samples/duração/seek/reconnect e publicar fixtures/hashes/licenças/backend/OS; não logar URLs privadas nem depender de rádios públicas no CI.
