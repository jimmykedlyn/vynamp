# Contratos estruturados e exemplos

Versão 0.7.0 · Schemas candidatos draft 2020-12; implementação não iniciada.

| Contrato | Schema | Exemplo |
|---|---|---|
| theme-manifest | [Schema](theme-manifest.schema.json) | [Exemplo](examples/theme-manifest.json) |
| plugin-manifest | [Schema](plugin-manifest.schema.json) | [Exemplo](examples/plugin-manifest.json) |
| audio-preset | [Schema](audio-preset.schema.json) | [Exemplo](examples/audio-preset.json) |
| source-descriptor | [Schema](source-descriptor.schema.json) | [Exemplo](examples/source-descriptor.json) |
| theme-layout | [Schema](theme-layout.schema.json) | [Exemplo](examples/theme-layout.json) |
| playback-event | [Schema](playback-event.schema.json) | [Exemplo](examples/playback-event.json) |

Os exemplos são dados fictícios, não pacotes instaláveis. `module.wasm` não está incluído e o hash de zeros é placeholder explícito; nenhum checksum/assinatura real foi produzido. Domínios `.invalid` nunca serão servidores de produção. LicenseRef-ExampleOnly marca apenas metadado de exemplo, não escolhe licença do produto.

JSON Schema verifica forma e limites básicos. O parser deve verificar paths seguros, hashes reais, assinatura, permissões, Nyquist, bindings permitidos, geometria, unicidade de IDs e consistência live/seek/duração. Unknown optional fields podem ser avisados pelo modo forward-compatible do parser; schemas estritos são usados na autoria/CI. Schema major desconhecido é rejeitado.

Layout aqui é exemplo mínimo de nós, não o modelo completo de constraints do editor. Antes implementar, ampliar schema com anchors/min/max/fallback, migration e testes sem alterar os invariantes do design. Playback-event payload é envelope aberto para evoluir; cada evento terá schema específico no SDK/contrato final.

[OpenAPI proposto](openapi-platform.json), [SDK draft](plugin-host.wit) e [docs de contratos](../architecture/04-contratos.md). A tarefa E09-02 congela/refina a proposta da API com contract tests; este JSON não representa servidor existente. O WIT é proposta de interface a compilar no spike, não binário validado.

Versão0.7.0: `android-content` é locator local do cliente, não transporte retornado pela API web. Themes têm perfis opcionais phone/tablet com fallback builtin; plugins enumeram Android arm64/emulador x64, sujeito a runtime/permissões/distribuição de host. Macx64/arm64 são targets obrigatórios, não compatibilidade emulada.
