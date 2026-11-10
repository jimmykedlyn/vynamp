# Evidências e relatórios

Versão 0.7.0 · 2026-10-14 · Player ainda não implementado.

Guardar logs sanitizados, commit/source digest, fixtures e ambiente. Evidências rust-tooling.json (0.4), tooling-preparation.json (0.3) e vynamp-branding.json (0.5) são históricas e não representam a auditoria atual. completeness-audit.json e completeness-verification.log registram os checks locais 0.6 quando concluídos. Nenhum relatório local de tooling aprova áudio/hardware/installer/CI remoto.

Contrato release/<version>.json: source_digest gerado por cargo xtask release digest; reviewers reais [claude]; platforms windows-x64/linux-x64/macos-intel/macos-silicon/android-arm64 com status passed, hardware, caminho e SHA256 do relatório. Só produzir após execução/revisão real; nunca copiar exemplo e marcar pass. Esta entrega foi revisada por agente próprio; não declarar revisão que não ocorreu.

Reinício 0.7: windows-restart.json e windows-restart-verification.log são evidência atual. completeness-audit.json/0.6 e demais runs anteriores são históricos, não provas da versão nova.

E15-01 / QA-009: [relatório da worktree](task-e15-01-qa-009.json) registra a incorporação de `125db63` como `479ae30`, comandos reais, logs e hashes. O relatório mantém separados testes locais, confirmação pelos managers, discovery e revisão Claude; aceite permanece pendente. Esta evidência de tooling não aprova os testes adversariais de pacotes/temas/plugins do player.

Complemento recebido em 2026-10-16: [transcrição fornecida pelo usuário](task-e15-01-user-terminal-20261016.log), com doctor confirmando ambos managers e `ci docs` completando no PowerShell normal com 29 + 23 testes aprovados. Proveniência e limites de captura constam no relatório; os resultados anteriores do sandbox permanecem preservados.
