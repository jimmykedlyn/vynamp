# TASK-E19-02 — Higiene, governança e README bilíngue do repo principal

Versão 0.8.0 · Estado planejado; documento e governança não equivalem a release assinado.

Fase M1 / P0 · Owner engineering-delivery · Esforço inicial 2–5 dias úteis; reestimar.

## Dependências

- [TASK-E19-01](TASK-E19-01.md) — Migrar o CI do repo principal para GitHub Actions

## Entrega concreta

LICENSE dual, LEGAL, CODE_OF_CONDUCT, GOVERNANCE, templates de issue, SECURITY.md com canal de advisory e README bilíngue EN/PT, com docs de CI e Gitflow reescritos para o repo público.

O repo principal não tinha LICENSE alguma, enquanto as libs já têm `LICENSE-MIT` + `LICENSE-APACHE` + `LEGAL.md`. Abrir ao público sem licença e sem política de segurança torna o repositório inapto a uso externo. O canal de vulnerabilidade é **GitHub Security Advisories** (decisão do mantenedor em 2026-11-05): privado por padrão, visível só ao mantenedor e ao reportante, sem criar endereço de e-mail novo — o GitHub exibe o aviso "Report a vulnerability" no próprio repo. `SECURITY.md` e `CHANGELOG.md` ficam **bilíngues** junto com o README, para não haver dois regimes de idioma no mesmo repositório.

Escopo: `LICENSE` (MIT OR Apache-2.0), `LICENSE-MIT`, `LICENSE-APACHE`, `LEGAL.md`, `CODE_OF_CONDUCT.md`, `GOVERNANCE.md`, `.github/ISSUE_TEMPLATE/`, `SECURITY.md` (reescrito), `README.md` (EN+PT no mesmo arquivo); `docs/engineering/03-gitflow-github.md` e `04-ci-cd.md` reescritos (removem "Tornar o repositório público não é alternativa aceita" e a nota "macOS não verificado"); `docs/architecture/adr/ADR-010-woodpecker-only-ci.md` marcado como substituído, sem apagar o histórico; `AGENTS.md` e `repo/AGENTS.md` atualizados.

Boundary: não altera assinatura, plano nem proteções; não cria identidade legal nova; não abre os repositórios — a abertura é TASK-E19-06.

## Critérios de aceite

README bilíngue EN/PT na raiz; SECURITY.md aponta para GitHub Security Advisories e não inventa contato; LICENSE dual presente; docs não afirmam capacidade ou alvo não verificado.

Detalhamento: `README.md` tem seção English e seção Português com o mesmo conteúdo essencial; `SECURITY.md` contém o link `https://github.com/jimmykedlyn/vynamp-mediaplayer/security/advisories/new` e nenhuma frase de "URL ou e-mail de segurança ainda não aprovado"; `LICENSE`, `LICENSE-MIT` e `LICENSE-APACHE` existem e a escolha MIT OR Apache-2.0 é declarada em `LEGAL.md`; `docs/engineering/04-ci-cd.md` descreve GitHub Actions e declara macOS Intel e Apple Silicon como alvos verificados, sem a linha "NOT VERIFIED"; `ADR-010` está marcado "Substituída pela ADR-014" e permanece no repositório.

[Spec](../../docs/security/01-threat-model.md) · REQ-E19-02 em [matriz](../../docs/product/03-requirements.md).

## Evidência

QA-018, QA-030; registrar commit/digest, comando, ambiente/OS/ABI, outputs e limitações. Quebrar em slices pequenas antes execução, implementar sucesso/erro/cancelamento, revisar pelo outro agente e preservar resultados no PR/docs/evidence.
