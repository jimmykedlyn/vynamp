# TASK-E19-06 — Abrir os repositórios ao público e proteger as branches

Versão 0.8.0 · Estado planejado; abrir repositório não equivale a proteção aplicada.

Fase M3 / P0 · Owner engineering-delivery · Esforço inicial 1–3 dias úteis; reestimar.

## Dependências

- [TASK-E19-01](TASK-E19-01.md) — Migrar o CI do repo principal para GitHub Actions
- [TASK-E19-02](TASK-E19-02.md) — Higiene, governança e README bilíngue do repo principal
- [TASK-E19-04](TASK-E19-04.md) — Re-apontar as specs de Restore e Converter e criar os repos próprios
- [TASK-E19-05](TASK-E19-05.md) — CI e README bilíngue nas libs Rust e no MVP de referência

## Entrega concreta

Sete repositórios Vynamp abertos ao público com relato privado de vulnerabilidade habilitado, e branch protection aplicada em develop exigindo o contexto CI Gate, com descriptions e wiki ajustadas.

A abertura é **irreversível em origem**: privado→público é gratuito, público→privado exige suporte do GitHub. Por isso a ordem importa: nenhum repo nasce público com conteúdo quebrado. Conteúdo primeiro (TASK-E19-01 a TASK-E19-05), depois a abertura, e **imediatamente depois** a proteção — repositório público sem branch protection aceita PR de qualquer pessoa.

A virada para público desativa push rulesets (consequência documentada pelo GitHub), então a proteção tem de ser **reaplicada depois**, não antes. O 403 "Upgrade to GitHub Pro" que travava a proteção era limite do plano **privado**; em repositório público o plano gratuito suporta `required_status_checks`, então `CI Gate` passa a ser status obrigatório de verdade. O `vitrola` é arquivado, não aberto — ele sai do conjunto.

O relato privado de vulnerabilidade (PVR) é **exclusivo de repositório público** e não é ligado pela abertura: `gh api repos/<owner>/<repo>/private-vulnerability-reporting` retorna 404 hoje, e o GitHub habilita o recurso por `PUT` em `/repos/{owner}/{repo}/private-vulnerability-reporting` depois que o repo fica público, ou por Settings → Security and quality → Advanced Security → Enable private vulnerability reporting. Sem esse passo, `SECURITY.md` e `CODE_OF_CONDUCT.md` apontam para um canal morto. Ele é exigido aqui porque a D-GOV-01 (TASK-E19-02) escolheu esse canal como único canal de segurança e de conduta.

Escopo: `gh repo edit --visibility public --accept-visibility-change-consequences` para `vynamp-mediaplayer`, `vynamp-decode`, `vynamp-output`, `vynamp-mvp-reference`, `vynamp-encode`, `vynamp-converter`, `vynamp-restore` — um por vez, com verificação entre eles; em cada um, `gh api --method PUT repos/jimmykedlyn/<repo>/private-vulnerability-reporting` **depois** da abertura; `gh repo archive jimmykedlyn/vitrola`; `description` sem "(privada; abertura futura)"; `--enable-issues` e `--enable-wiki=false`; `cargo xtask repo protect --repo jimmykedlyn/<repo> --reviews 0` no repo principal e `PUT /repos/{owner}/{repo}/branches/develop/protection` nos que não têm `cargo xtask`; `config/github.json` → `protection_status: applied`.

Boundary: não altera assinatura, plano nem seat count; não faz merge, tag nem release; não apaga repositório; não abre `vitrola` (arquivado, o PVR não se aplica).

## Critérios de aceite

Os sete repos reportam visibility public e `private_vulnerability_reporting` habilitado; develop exige CI Gate e proíbe force push e exclusão; description não promete abertura futura; repo sem proteção não é considerado pronto.

Detalhamento: `gh repo view jimmykedlyn/vynamp-mediaplayer --json visibility` retorna `PUBLIC`; `gh api repos/jimmykedlyn/vynamp-mediaplayer/private-vulnerability-reporting` retorna `{"enabled":true}`; `gh api repos/jimmykedlyn/vynamp-mediaplayer/branches/develop/protection` lista `CI Gate` em `required_status_checks.contexts`; `gh repo view jimmykedlyn/vitrola --json isArchived` retorna `true` e `visibility` continua `PRIVATE`; nenhuma `description` contém "privada" ou "private".

[Spec](../../docs/engineering/03-gitflow-github.md) · REQ-E19-06 em [matriz](../../docs/product/03-requirements.md).

## Evidência

QA-009, QA-030; registrar commit/digest, comando, ambiente/OS/ABI, outputs e limitações. Mudança de visibilidade é operação **irreversível em origem** — executar por último, um repositório por vez, com verificação entre eles. Quebrar em slices pequenas antes execução, implementar sucesso/erro/cancelamento, revisar pelo outro agente e preservar resultados no PR/docs/evidence.
