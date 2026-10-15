# Política de segurança proposta

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

Para implementação futura, vulnerabilidade não deve ser reportada publicamente com secrets ou dados do usuário. Canal privado de disclosure e SLA serão cadastrados antesM3; este projeto ainda não tem URL/email de segurança aprovado. Não inventar contato operacional.

Componentes prioritários: parsers, codec native helpers, packages, capabilities, local IPC, updater, web auth/ingest. Triagem deve registrar versão, entrada mínima licenciada, impacto e mitigação; reproduções locais sem conteúdo pessoal. Avisos de dependência exigem análise de reachability, não ignorar nem atribuir exploit sem prova.

Ver [modelo de ameaças](docs/security/01-threat-model.md) e [privacidade](docs/security/02-privacy-licensing.md). Nenhum mecanismo é declarado “seguro” só porque escrito em Rust.
