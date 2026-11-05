# TASK-E19-01 — Migrar o CI do repo principal para GitHub Actions

Versão 0.8.0 · Estado planejado; configurar CI não equivale a player pronto nem a release assinado.

Fase M1 / P0 · Owner engineering-delivery · Esforço inicial 3–10 dias úteis; reestimar.

## Dependências

- [TASK-E15-03](TASK-E15-03.md) — Importar GitHub privado e ativar Gitflow

## Entrega concreta

ADR-014, workflows ci/release/quality em GitHub Actions com actions fixadas por SHA e gate único CI Gate, xtask de validação e release reescrito para Actions e remoção do Woodpecker.

Em repositório público os runners hospedados do GitHub são gratuitos e a branch protection volta a estar disponível no plano gratuito — são as duas razões técnicas que sustentavam a ADR-010. O gate `CI Gate` migra do contexto do Woodpecker para job do GitHub Actions com o **mesmo nome**, para que `config/github.json` continue exigindo um status que o GitHub publica. O desenho dos workflows é **restauração** do histórico (`247b424`), não projeto novo: o commit `e7562fd` havia apagado os três workflows e reescrito o validador para proibi-los.

Escopo: `docs/architecture/adr/ADR-014-public-github-actions-and-in-app-dsp.md`; `.github/workflows/{ci,release,quality}.yml` restaurados e adaptados (macOS Intel e Apple Silicon agora verificáveis, triggers restritos a PR→`develop`, push em `develop`/`main` e tag `v*`); `xtask/src/validate.rs` passa a exigir os workflows em vez de proibi-los; `xtask/src/release.rs` volta a validar a tag por `GITHUB_REF`; `xtask/src/repo.rs` passa a exigir visibilidade pública; `config/github.json` atualizado; `.woodpecker/` e `infra/ci/` removidos.

Boundary: não altera assinatura, plano, proteções existentes nem regra de merge; não faz merge, tag nem release; **não abre os repositórios** — a abertura é TASK-E19-06, executada pelo mantenedor.

## Critérios de aceite

Actions fixadas por SHA; gate reprova dependência falha, cancelada ou ausente; release só em tag v* e falha fechado; macOS Intel e Apple Silicon viram alvos verificados; nenhum segredo no histórico.

Detalhamento: nenhum `uses:` por tag flutuante ou `v1`/`v2`; as checagens do gate `CI Gate` leem `needs.*.result` (equivalente a `test "$DOCS" = success`), então cancelamento também reprova; `release.yml` dispara só em `refs/tags/v*` e termina em falha fechada antes de qualquer artefato assinado; `macos-15-intel` e `macos-15` entram na matriz de tooling e de core, retirando a limitação "macOS não verificado" do Woodpecker; `git log --all -p` não revela token, PAT, chave AWS ou chave privada.

[Spec](../../docs/engineering/04-ci-cd.md) · REQ-E19-01 em [matriz](../../docs/product/03-requirements.md).

## Evidência

QA-018, QA-021; registrar commit/digest, comando, ambiente/OS/ABI, outputs e limitações. Instalação de plugin/repo privado/assinatura requer ambiente real; não substituir por mocks/documentos. Quebrar em slices pequenas antes execução, implementar sucesso/erro/cancelamento, revisar pelo outro agente e preservar resultados no PR/docs/evidence.
