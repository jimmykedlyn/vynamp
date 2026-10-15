# Build, CI, empacotamento e operação

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

Cargo workspace com toolchain pin e lockfile; CI e build matrices Windowsx64/Linux x64/macOSx64+arm64/Android arm64 são obrigatórios desde M0/M1, com testes nativos e hardware antes1.0. Universal2/macOS slices e AndroidNDK/Gradle/ABIs verificados, não só crates portáveis. Checks format/clippy, unit/property/integration, schema/examples, fixtures hash, native builds, license/security audit e SBOM. GUI/hardware runners dedicados para e2e/soak, resultados publicados por versão; CI sem hardware não aprova exclusivo.

Release channels dev/nightly/beta/stable com independent update feeds e pacote assinado. Windows: installer user-scope NSIS EXE user scope (ADR-007); MSIX adicional futuro; portable ZIP obrigatório. Validar restrições MSIX/ASIO/helpers antes decisão. Arquivos do usuário/config não estão na pasta do executável, exceto portable explicitamente selecionado. Sem admin obrigatório; uninstall preserva library/settings salvo escolha de apagar.

Artefatos: binaries, checksums, assinatura, changelog, SBOM, notices, symbols separados e recovery instructions. Code signing/supply chain sem secret em repo/log; environment-controlled signing. Instalação/updater em VM limpa e migration desde duas versões anteriores. Não auto-associar todos formatos.

Version SemVer app; package SDK/schema major independentes; database migration IDs monotônicos. Semver breaking exige changelog e migration docs. Release freeze para codec/driver changes com soak; canary beta antes stable. Atualização não reinicia durante playback/gravação sem escolha.

PlataformaM4: IaC, config/secret separation, staged deploy, migrations expand/contract, DB backup diário + PITR onde disponível, restore drill e runbooks. RPO≤15min/RTO≤4h alvos iniciais, custo/infra necessários registrados. Rollback de API preserva contratos; object storage lifecycle e direitos de exclusão verificados. Rotação keys/tokens e revocation drill. CDN/ingest métricas e budget alarms com responsável.

Distribuição1.0 adicional: Linux AppImage+DEB (Flatpak adicional futuro) após prova de portais/audio; macOS DMG + ZIP Universal2 obrigatórios, assinados/notarizados, com ambas slices nativas; Android APK/AAB assinado, targetSDK/page-size/native dependencies revisados a cada release. Stores/OS updater têm políticas próprias, não reutilizar instalador desktop em telefone.

Manifest de capacidades/associações e gates adicionais: [contrato](04-capability-release-gates.md).
