# Manifest de capacidades e gates de distribuição

Versão 0.7.0 · Preparação documental; aplicativo ainda não implementado.

## Artefatos obrigatórios na 1.0

| Plataforma | Artefatos |
|---|---|
| Windows x64 | NSIS EXE e portable ZIP |
| Linux x64 | AppImage e DEB |
| macOS | DMG e ZIP Universal2, ambos com Intel e Silicon nativos |
| Android arm64 | APK e AAB |

São oito artefatos obrigatórios e cinco alvos de teste. MSIX, Flatpak, PKG e pacotes Mac separados são extras futuros; não substituem os obrigatórios. Stores têm assinatura e atualização próprias. Cada Mac exige relatório nativo independente; emulator não substitui Android físico.

## Manifest por artefato

Gerar product/version/platform/arch/minOS, sourceDigest, backend/features/configuração, inventário nativo, licenças/notices, capabilities de codecs, integração OS, visualizers, DSP e saída. Vincular evidências, fixtures e hashes ao artefato efetivamente distribuído. Catálogo planejado não alimenta declarações OS diretamente. Associação exige binding/parser/demux/decode verificados, disponíveis e licenciados; import de playlist usa capability própria. Liste desconhecido/indisponível com motivo. Um decoder listado pelo FFmpeg ou feature all-codecs não é evidência de playback.

Assinaturas, revogação, staging, health checks, rollback e compatibilidade de DB seguem os gates existentes. Installer/updater auditam bibliotecas realmente incluídas. Branding nativo e declarações de arquivo precisam de hashes e provas; não liberar flags manualmente para passar.

## Portable e concorrência

Um owner pelo armazenamento canônico do perfil, mesmo entre sessões, aliases, instalado e portable; updater respeita lock e não encerra reprodução/gravação sem escolha do usuário. Perfis distintos permitem instâncias explícitas. Portable/AppImage só registram integração por ação explícita. Uninstall remove apenas registros ainda pertencentes à instalação e preserva dados/defaults de terceiros. [Contrato de ativação](../platforms/04-activation-ownership.md).

## Gates adicionais M3

TASK-E16-07/08 e TASK-E17-12/20 integram TASK-E14-06. Formatos required comuns exigem verificação nos cinco alvos; conditional exige spike e limitação documentada, sem alegação de suporte quando indisponível. QA-022–031 cobre ativação, grants, trim/DRC/canais, sinal final, visualizers e proxy. Recursos M4/M5 não bloqueiam os requisitos comuns M3.

Documento, schema ou teste de tooling não aprova áudio ou integração nativa. application_ready, release_enabled e branding_exports_ready permanecem false neste pacote. Não gerar executáveis, instaladores ou ícones nativos fictícios.
