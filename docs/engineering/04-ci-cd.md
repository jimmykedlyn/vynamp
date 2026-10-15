# CI/CD e gates por plataforma

Versão 0.7.0 · 2026-10-14 · Configuração do projeto; player ainda não implementado.

Workflows reais em .github/workflows: ci.yml, release.yml, quality.yml. Actions pinadas por SHA verificado, Dependabot propõe updates semanais. Permissões read por padrão; write só no draft publisher. Sem pull_request_target, secrets ou runners self-hosted para código de fork. Ref/SemVer valida antes interpolação; nomes de matrix são constantes.

CI PR/push: documentação/schema/backlog/delivery/tests tooling sempre; Tooling Rust é testado nos quatro runners desktop, independentemente do app. Fontes reais em apps/desktop ativam fmt/clippy/test native em Windows2022, Ubuntu22.04, macOS15Intel e macOS15Silicon; Gradle real ativa Android unit/lint/debug. Toolchains/locks obrigatórios a partir de código real. CI Gate agregado falha quando job ativo falha/cancela; não oculta falha com allow-failure. Nesta fase o Cargo workspace contém somente xtask; sem fontes desktop/Gradle reais, skips declarados não significam que player passou testes.

Release em tag v*: preflight exige implementation/release flags, Cargo/Gradle/locks/licença/notices/public key, evidência completa dos cinco alvos e digest de source revisado. Builders Windows/Linux/macOSUniversal/Android geram formatos assinados; environment de signing protegido. Publish job baixa artifacts do mesmo run, confere conjunto completo/não vazio, gera hashes e assina manifesto; cria apenas draft privado. Só mantenedor publica após verificação. Não distribuir chave privada em artifacts/logs.

Hardware workflow manual, main/develop apenas, environment próprio e runners confiáveis com Windows/Linux/MacIntel/MacSilicon/Androiddevice. Depende de tests/hardware/Cargo.toml e harness Rust reais, sem auto-green caso harness/aplicativo faltem. Process death/a11y/soak/bit-perfect não são aprovados pelo job build hospedado. Fuzz, numerical regressions, update/install drills e perf são detalhados em testes; precisam de harness conforme M1–M3.

Rust 1.99.0 foi instalado e pinado; config/toolchains.json mantém NDK/SDK como decisões M0 pendentes e CI de código falha se não preenchidas. Versões de actions/OS devem ser auditadas antes release e atualizadas por PR. [Runner matrix oficial](https://docs.github.com/en/actions/reference/runners/github-hosted-runners) e [segurança Actions](https://docs.github.com/en/actions/reference/security/secure-use).

CD web M4: implementar IaC/healthcheck/migrations expand-contract, staging e environment production separado; não publicar infraestrutura vazia ou usar credentials mobile para backend. TASK-E11-10/E09 continuam donos dessa entrega.
