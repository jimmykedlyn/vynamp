# Pull request — restauração por IA como fase futura (planejada, não prometida)

## Resultado

**Problema.** Duas especificações de restauração por aprendizado de máquina existiam na pasta raiz do projeto e não tinham **nenhuma** representação no repositório: `AUDIOSR-RUST-CPU.md` (super-resolução de banda por difusão latente) e `SOURCE_SEPARATION_MODELS.md` (separação de fontes por ONNX). Não havia requisito, ADR, fase de roadmap nem TASK que as mencionasse. Isso é pior que a ausência: um documento com detalhe de implementação e sem status normativo pode ser lido como promessa atrasada **ou** como descarte, e ninguém consegue decidir qual.

Ao mesmo tempo, a [ADR-013](../../../architecture/adr/ADR-013-vynamp-restore.md) §4, já aceita, declara exatamente o oposto no escopo corrente: a região `overlay` foi **removida** da 1.0, a separação de voz por IA é "decisão futura e não é prometida", e o upscaling de hi-res é **síntese**, não recuperação. Absorver os dois arquivos como escopo corrente contradiria uma ADR aceita; descartá-los perderia análise técnica real.

**Comportamento final.** A capacidade fica registrada como **fase opt-in pós-1.0, planejada e não prometida**, com cinco condições bloqueantes explícitas, nenhuma satisfeita hoje. O escopo da 1.0 não muda: `kind: overlay` continua removido, o denoise continua fora, e o build não ganha uma única dependência de ML.

**Decisão do mantenedor (2026-11-05):** *"spec futura, sem mudar a 1.0"*.

**Plano e evidência.** [Evidência de execução](README.md).

### Decisões registradas, não implícitas

`docs/product/05-decisions.md` 0.7.5 → 0.7.6, seção "Decisões de restauração por IA":

| ID | Decisão |
|---|---|
| D-IA-01 | Planejada e não prometida: fase pós-1.0, sem milestone, sem TASK, sem REQ na matriz. `RSAI-01..14` são locais ao documento, como os `LYR-*` (D-LYR-01) |
| D-IA-02 | Opt-in de ponta a ponta: desligado por padrão, ativar não baixa nada, download só por clique com tamanho/fonte/licença visíveis, checksum SHA-256, sem download automático; build sem as features não tem dependência de ML |
| D-IA-03 | CPU é o piso e o padrão; GPU é aceleração opcional com teste de paridade e fallback que **retoma do bloco** |
| D-IA-04 | A ADR-011 vale integralmente no caminho de reprodução e **admite** runtime de terceiros apenas em job offline, mediante ADR nova que a emende. A ADR-015 não é essa emenda |
| D-IA-05 | Pesos são obra de terceiros, não são embutidos, e licença não conferida bloqueia oferecer na UI e bloqueia release |
| D-IA-06 | Super-resolução é **síntese**, nunca "recuperação"; substitui só acima do corte, com crossover linear-phase; saída sempre em arquivo novo |

### A tensão resolvida explicitamente

A spec diz "fase futura" e a ADR-013 diz "não é prometida". Isso não é contradição, e o PR registra a distinção em vez de harmonizá-la: a ADR-013 governa o **escopo da 1.0**, a ADR-015 governa a **decisão sobre a decisão futura** — custos, condições e bloqueios. Os dois textos foram preservados, não reescritos.

Três pontos onde a spec de IA é deliberadamente mais conservadora que o documento de origem:

- **Sem métricas subjetivas de qualidade.** Os alvos de energia em faixa nomeada e as notas por estrela de `AUDIOSR-RUST-CPU.md` não entram como critério de aceite — não foram medidos neste repositório. O aceite usa paridade, determinismo, recursos, opt-in e A/B cego.
- **Sem número de requisito na matriz.** Promover a capacidade a `REQ-*` numerado seria prometê-la e alteraria as contagens 191/19.
- **Sem embutir modelo.** Mesmo o menor (~30 MB) fica fora do instalador: embutir baixaria atrito de UX ao custo de redistribuir obra de terceiros no caminho de `THIRD_PARTY_NOTICES` e SBOM.

## Arquivos

| Arquivo | Mudança |
|---|---|
| `docs/tools/03-vynamp-restore-ia.md` | **Novo.** Spec da fase futura, 14 seções, `RSAI-01..14` |
| `docs/architecture/adr/ADR-015-ai-restoration-future-phase.md` | **Novo.** A decisão, cinco condições bloqueantes, quatro alternativas rejeitadas |
| `docs/product/05-decisions.md` | 0.7.6 — D-IA-01..06 e cinco divergências |
| `docs/tools/01-vynamp-restore.md` | 0.1.1 — §1.2 e nota sobre `kind` remetem à fase futura |
| `docs/architecture/adr/ADR-013-vynamp-restore.md` | 0.1.1 — nota "Destino da 'decisão futura'" em §4 |
| `docs/architecture/adr/ADR-011-own-audio-stack.md` | 0.1.1 — nota "Escopo desta decisão" (reprodução vs job offline) |
| `docs/security/02-privacy-licensing.md` | 0.7.2 — pesos de modelo como obra de terceiros |
| `docs/INDEX.md` | Lista plana de ferramentas e nova seção datada |

Nenhuma alteração de código, dependência, workflow ou backlog.

## Verificação

Ambiente: Windows 11 Pro 10.0.26300 · `rustc 1.99.0 (b940084d7 2026-09-28)`.

| Comando | Resultado |
|---|---|
| `cargo xtask check-docs` | `"result": "passed"`, `"errors": []` — 351 arquivos Markdown, 191 requisitos, 19 épicos |
| `cargo xtask check-delivery` | passou |
| `cargo fmt --all --check` | sem diff |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | sem aviso |
| `cargo test --workspace --locked` | passou |

O validador exercitou o caminho de falha de verdade neste PR: `Missing link docs/INDEX.md: evidence/PR/chore-specs-ia-futura/pr-body.md`, corrigido antes de reexecutar.

**Limitações.**

- Nada foi medido: nenhum modelo baixado, nenhum runtime exercitado, nenhum spike. Tempo, RAM e qualidade são estimativas herdadas e estão marcadas como tal.
- Nenhuma licença de peso foi conferida. A regra é processo, não resultado.
- Nenhuma ADR emenda a ADR-011; a exceção para runtime offline está proposta, não decidida.
- A leitura de "pesos são obra de terceiros" não passou por revisão jurídica; é a mesma que a política de licenças já aplica a codecs.
- Não altera visibilidade, proteções, assinatura nem plano — isso é da TASK-E19-06 (ADR-014).
- A absorção das lacunas dos arquivos de SKIN é o **segundo** PR desta rodada, não este.

- [x] Worktree e owner próprios.
- [x] Superpowers carregado; verificação executada.
- [x] Sucesso/erro/cancelamento: validador exercitado no caminho de falha e no de sucesso.
- [x] Docs/contracts/notices atualizados; nenhuma credencial no diff.
- [ ] CI Gate (GitHub Actions) — aguardando o run deste SHA.
- [ ] Revisão técnica independente — pendente.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
