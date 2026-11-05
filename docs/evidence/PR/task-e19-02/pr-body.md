# Pull request — TASK-E19-02

## Resultado

**Problema.** O repositório principal não tinha licença nenhuma, `SECURITY.md` declarava que não havia canal de contato aprovado, não havia CODE_OF_CONDUCT, GOVERNANCE nem formulários de issue, e o README era monolíngue. Os três primeiros são bloqueios para a abertura pública (TASK-E19-06) — publicar sem licença publica código que ninguém pode usar legalmente, e publicar sem canal de segurança deixa uma vulnerabilidade sem destino.

**Comportamento final.** O repositório passa a ter licença dual `MIT OR Apache-2.0`, canal de vulnerabilidade em GitHub Security Advisories, governança e código de conduta declarados, formulários de issue com campos obrigatórios de evidência, e os quatro documentos de entrada — `README`, `CONTRIBUTING`, `SECURITY`, `CHANGELOG` — em inglês e português no mesmo arquivo.

**TASK / REQ / QA.** TASK-E19-02 · REQ-E19-02 · QA-018, QA-030.

**Plano e evidência.** [Evidência de execução](README.md) — inclui a saída literal do validador antes e depois e a tabela de verificação de aceite.

### Decisões registradas, não implícitas

O projeto estava num estado em que `AGENTS.md` proibia criar licença sem decisão registrada e `docs/security/02-privacy-licensing.md` proibia criar `LICENSE` "como se tivesse sido decisão do usuário". Criar a licença sem resolver isso contradiria a própria governança. Por isso as duas proibições foram **substituídas** por decisões explícitas em `docs/product/05-decisions.md` (v0.8.0):

| ID | Decisão |
|---|---|
| D-LIC-01 | O código é `MIT OR Apache-2.0`, com `LICENSE`, `LICENSE-MIT` e `LICENSE-APACHE` na raiz, igual às libs `vynamp-decode`/`vynamp-output` |
| D-LIC-02 | A licença cobre só este repositório: não relicencia `crates-copyleft/`, não concede direitos sobre codecs de terceiros e **não equivale a release** |
| D-GOV-01 | Canal de vulnerabilidade = GitHub Security Advisories do próprio repositório, sem e-mail novo |

`02-privacy-licensing.md` (0.8.0) mantém a exigência real: `LICENSE` não substitui `THIRD_PARTY_NOTICES`, SBOM nem evidência assinada. O `release preflight` exige `THIRD_PARTY_NOTICES`, evidência de hardware real por plataforma e registro de revisores; SBOM e assinatura minisign são do `publish-draft` e da spec de entrega, não do preflight.

### Escolhas conservadoras

- **Sem SLA.** A primeira versão do `SECURITY.md` prometia resposta em 5 dias úteis; foi trocada por "no committed response SLA", porque criar prazo de resposta não é decisão do agente.
- **Sem e-mail inventado.** O `CODE_OF_CONDUCT.md` também encaminha conduta para os advisories, marcados como assunto de conduta.
- **Issue Forms `.yml`, não templates `.md`.** `xtask::validate::spec` varre o repositório inteiro e exige, para todo `.md`, título `# ` e entrada em `docs/INDEX.md` — um template de issue em Markdown viraria documento normativo indexado. Este foi o único achado real do validador neste trabalho (`Missing index` para `CODE_OF_CONDUCT.md`, `GOVERNANCE.md`, `LEGAL.md`), corrigido com três entradas no índice.

## Verificação

Ambiente: Windows 11 Pro 10.0.26300 · `rustc 1.99.0 (b940084d7 2026-09-28)` · `git 2.47.1.windows.2`.

| Comando | Resultado |
|---|---|
| `cargo fmt --all --check` | exit 0, sem diff |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | sem aviso |
| `cargo test --workspace --locked` | 86 passed (2 suites) |
| `cargo xtask check-docs` | `"result": "passed"`, `"errors": []` — 353 arquivos Markdown, 191 requisitos, 19 épicos |
| `cargo xtask check-delivery` | `{"errors":[],"result":"passed","workflow_files":3}` |

Aceite da TASK, item por item:

| Critério | Verificação |
|---|---|
| README com seção English e seção Português | `README.md` tem `# Vynamp` (EN) e `# Português — Vynamp` (PT) com os mesmos 7 blocos |
| `SECURITY.md` com o link e sem frase de pendência | link presente; `grep "ainda n.o aprovado"` só encontra a própria spec da TASK |
| Três arquivos de licença e expressão em `LEGAL.md` | existem; `LEGAL.md` declara a expressão SPDX e D-LIC-01 |
| `04-ci-cd.md` sem "NOT VERIFIED" | `04-ci-cd.md:19` declara macOS Intel e Apple Silicon verificados; as duas ocorrências restantes são a ADR-014 e a evidência histórica da E15-04, citando a limitação antiga |
| `ADR-010` marcado como substituído e preservado | `ADR-010:3` traz o aviso; o texto original segue no arquivo |

`docs/engineering/03-gitflow-github.md`, `04-ci-cd.md` e a marcação da `ADR-010` já vinham da TASK-E19-01 (PR #30) e foram apenas verificados aqui.

**Limitações.**

- Nada foi aberto: visibilidade, `description`, Issues e branch protection continuam na TASK-E19-06.
- Os três Issue Forms são YAML válido, mas o GitHub só os valida ao renderizar a página de nova issue — não verificado.
- Nenhum advisory foi aberto; o canal está correto e existe, mas o fluxo não foi exercitado com relatório real.
- O `CHANGELOG.md` bilíngue reconstrói decisões a partir do registro de decisões, das ADRs e das evidências; não reemite releases, que nunca existiram como tags.
- A escolha de licença não passou por revisão jurídica; foi feita por paridade com as libs e registrada como decisão do mantenedor.
- `crates-copyleft/`, codecs de terceiros e a marca Vynamp seguem fora da licença — declarado, não resolvido.

## Revisão independente

Duas passagens em contexto limpo (Sonnet 5-5 / Opus 5-5, sem o raciocínio do autor), oito achados, todos reconferidos pelo autor e todos corrigidos.

**Primeira passagem — cinco achados:**

1. `CONTRIBUTING.md` documentava o padrão de branch `feature|fix|chore/<task>-<slug>`, que o hook real recusa → substituído pelo regex de `xtask/src/repo.rs:225` nos dois idiomas.
2. `bug_report.yml` apontava para `../blob/develop/SECURITY.md`, caminho relativo que não resolve → `../../SECURITY.md`.
3. TASK-E19-01 estava `implemented` no `backlog.json` com a spec ainda em "planejado" → JSON de volta a `in_progress`, as duas specs em "em execução". O validador não pega isso: `validate::spec` compara `title`/`deliverable`/`acceptance`, não `status`.
4. `README.md` listava `vynamp-restore`/`vynamp-converter`/`vynamp-encode` como repositórios existentes → corrigido para "decididos, ainda não criados" nos dois idiomas.
5. `docs_problem.yml` pedia confirmação de que `check-docs` "não reporta a falha", mas o validador é estrutural e passa num erro semântico → rótulo reescrito.

**Segunda passagem — três afirmações que o código não sustentava:**

6. Documentos diziam que `release preflight` verifica SBOM e assinatura. Não verifica: `grep -ri sbom xtask/src/` é vazio, e `minisign` só aparece em `release.rs:266,280`, dentro de `publish-draft` → `LEGAL.md`, `05-decisions.md` e `02-privacy-licensing.md` corrigidos.
7. `SECURITY.md` e `CODE_OF_CONDUCT.md` tratavam o canal de advisories como disponível. `gh api .../private-vulnerability-reporting` → 404, e a URL de advisory respondida a anônimo → 404 → os dois arquivos passam a dizer que o canal entra no ar na TASK-E19-06, que ganhou o `PUT` do PVR como critério de aceite.
8. `crates-copyleft/` descrito como diretório existente; é convenção do `vynamp-decode`, não populada → corrigido.

Junto: `LICENSE` dizia "both … and" para uma licença **OR**; `xtask/Cargo.toml` ganhou `license = "MIT OR Apache-2.0"`; `LEGAL.md` completou o bilinguismo (era o único arquivo de governança com quatro seções em inglês apenas); `tasks/README.md:5` deixou de afirmar que todos os itens estão planejados.

**Não há terceira passagem.** As duas revisões independentes seguintes morreram por cota do plano esgotada (HTTP 429 em `claude-opus-5-5`, reset em ~76 h), sem emitir relatório. Os oito achados acima estão corrigidos e reconferidos no código, mas este PR não tem revisão independente posterior às correções.

- [x] Worktree e owner próprios.
- [x] Superpowers carregado; verificação executada.
- [x] Sucesso/erro/cancelamento: validador exercitado no caminho de falha (`Missing index`) e no de sucesso.
- [x] Docs/contracts/notices atualizados; nenhuma credencial no diff.
- [x] Revisão técnica independente — oito achados em duas passagens, todos corrigidos; sem terceira passagem (as seguintes morreram por cota do plano).
- [x] CI Gate (GitHub Actions) — verde em `ae10e3b` (run 42125289298): `docs`, `tooling` em ubuntu-22.04, windows-2022, macos-15-intel e macos-15, mais o gate; `core` e `android` pulados por `cargo xtask ci detect`.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
