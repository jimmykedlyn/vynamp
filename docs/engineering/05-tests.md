# Contrato de testes, evidência e comandos

Versão 0.7.0 · 2026-10-14 · Configuração do projeto; player ainda não implementado.

Tooling entregue: `cargo test --package xtask --locked` testa Git real em diretórios temporários: main/develop, índices isolados, exclusividade de TASK, slug/ID inválido, remoção de worktree suja/não integrada e release bloqueado sem app/artifacts. Sem rede, dados pessoais ou certificados. TDD observado: testes de tooling falharam pela ausência das implementações e passaram depois; nenhuma engine de áudio foi testada.

E15-02 amplia a prova com dois processos concorrentes disputando uma claim, staging independente, entradas inválidas sem efeitos, retry após destino/branch colidente, claim interrompida, remoção de feature integrada, hotfix main-only/develop-only e preservação de conteúdo/claim quando há checkout drift ou metadados inconsistentes. Symlink adversarial usa teste Unix no CI; Windows local não executa esse teste. QA-009/QA-012 aqui validam apenas entradas adversariais e recuperação do tooling; não aprovam budgets/grants de plugins nem updater/migration do aplicativo. Ver [evidência E15-02](../evidence/PR/task-e15-02/README.md).

App Rust: unit/property domain/state/generation/queue; integration decode/metadata/migrations/output mocks; numerical PCM/gapless/filter/resampler/dither; adverse HTTP/ICY/HLS server; fuzz de parser/package/RPC; snapshots/UI/semantics. Cargo test --locked é gate quando workspace existe; sem random tests apenas espelhando funções. Fixture manifest precisa hash/licença/formato/oráculo independente.

Desktop hardware: WindowsDAC, Linuxaudio server, IntelCoreAudio e SiliconCoreAudio separadamente, unplug/rate/busy/sleep,8hsoak e captura transparente quando hardware adequado. Android: NDK bridge ownership, MediaSession/focus/noisy/BT/SAF, screen-off2h, recovery/process death, tablets/foldables, TalkBack e energia protocolada. Emulador só auxilia, não prova DAC/ARM real/bit-perfect.

Instaladores: clean install, upgrade de duas versões anteriores, launch smoke, uninstall preserva dados, rollback, assinaturas/quarantine e privacidade. Windowsuser scope semadmin; Linux AppImage/DEB libs auditadas; MacUniversal2/native fixtures/notarização; AndroidAPK/AAB signer/page size e grants. Fail-to-build não vira skip na1.0.

Evidências em docs/evidence e tasks/EVIDENCE: source digest, commit/runs, fixture hashes, device/driver/OS, resultado e limite. Para release: docs/evidence/release/<version>.json com reviewers codex/claude e cinco plataformas passed + relatório real/hardware/hash. Relatório não é criado como passed por este pacote. Assertions documentais não provam execução da API/installer/app.
