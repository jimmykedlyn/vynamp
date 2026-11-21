# TASK-E09-01 — Definir catálogo, direitos, escala e custos

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E09-01` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E09-01` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M4 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E05-09`, `TASK-E11-09` |
| Bloqueia | `TASK-E09-02`, `TASK-E11-10` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 8–20 dias úteis |
| Risco | alto |

## Entrega

RFC de plataforma com regiões, rights, renditions, limites e orçamento de bandwidth.

## Boundary

- **Não** congela nem implementa o contrato da API. O OpenAPI v1, os exemplos e os contract tests são `TASK-E09-02`, que depende desta; aqui a saída é prosa e tabela de política, e transformar a RFC em schema faria dois donos do mesmo arquivo `schemas/openapi-platform.json`.
- **Não** provisiona nada: IaC, secrets, PITR do banco, métricas de CDN e runbooks de restore/rollback são `TASK-E11-10`, bloqueada por esta. A RFC diz qual orçamento e qual região se quer; o custo real de infraestrutura e o alarme de budget são dela.
- **Não** mede carga nem fecha SLO. As 1000 sessões simultâneas, o p95 de 250 ms e a prova de startup ficam com `TASK-E09-10`; a estimativa daqui é analítica, feita sobre bitrate declarado, e não substitui medição.
- **Não** gera nem compra o corpus licenciado de fixtures. Gerar tons, sweeps, corrupções e os manifestos de hash e licença é `TASK-E10-01`; esta TASK apenas julga se um fixture proposto tem autorização para entrar no catálogo.
- **Não** faz o inventário legal das dependências do player (FFmpeg, ASIO, encoders AAC/MP3, fontes, assets). Isso é `TASK-E11-03`; direitos de conteúdo e licença de software são dois problemas separados e juntá-los aqui produziria uma aprovação de distribuição que ninguém assinou.
- **Não** implementa ingest, quarentena, probe ou transcode dos renditions que orça. O pipeline de upload e workers é `TASK-E09-05`; a RFC fixa perfis e pressupõe o custo deles, não o processo que os produz.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e09-01-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E09-01-AC-1` | uma lista de candidatos a fixture de catálogo com origem, autor e licença declarados em cada linha, misturando os FLAC sintéticos que o repo já usa (`libs/decode/crates/vynamp-decode-flac/tests/fixtures/sine-16-stereo.flac` e `sine-24-stereo.flac`, gerados por tom), material próprio sem contrato assinado e um lote de faixas de terceiros sem licença de redistribuição | a RFC é revisada linha por linha e cada candidato é classificado como autorizado, autorizado com restrição de transformação, ou excluído, citando a licença e o escopo de uso que sustenta a classificação | Fixtures autorizadas |
| `TASK-E09-01-AC-2` | as quatro roles da spec (listener, editor, operator, admin) e três territórios de exemplo (BR, PT, US), mais uma faixa cuja janela de disponibilidade começa e termina em datas concretas e uma segunda faixa sem data final | a RFC publica a matriz de role contra operação de catálogo/admin e a tabela de direitos preenchida por fixture, com territórios, janela de disponibilidade, permissão de download e permissão de gravação para cada uma | roles/territórios/janela de disponibilidade definidos |
| `TASK-E09-01-AC-3` | os renditions iniciais da spec (AAC-LC 128k e 256k em HLS, Opus 96k e 160k progressivo, FLAC lossless) e um ouvinte-hora definido como 60 minutos de playback numa sessão, com preço unitário de egress, transcode e storage citado com fornecedor e data de consulta | a RFC decompõe o custo por ouvinte-hora em bandwidth (bitrate vezes 3600 s dividido por 8, ajustado por cache hit), transcode, storage e um perfil de sessões concorrentes, e o total por profile é recalculado a partir da própria tabela | custo por ouvinte-hora estimado com dados. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | a RFC escrita sob `streaming/` e referenciada em `INDEX.md`, conferida com `./target/debug/xtask.exe check-docs` no umbrella, mais o cálculo de custo reexecutado a partir da tabela de bitrates e de preços da própria RFC por um script versionado ao lado dela, e `git diff --stat` mostrando que `schemas/openapi-platform.json` não foi tocado | Hoje `check-docs` prova forma e não conteúdo: no estado atual ele imprime `markdown_files: 210`, `openapi_paths: 25`, `operation_count: 26`, `schemas_and_examples: 10`, `errors: []`, `result: passed`, e o que ele verifica em cada markdown é que começa em `# `, está citado no `INDEX.md` e não tem link relativo quebrado. Nenhum número de custo é lido por gate nenhum. Não existe fixture de catálogo autorizado no repo: `tests/fixtures/` tem só `README.md`, e as pastas `audio/`, `network/`, `library/`, `packages/`, `ui/` e `manifests/` que o README reserva ainda não existem. A aritmética que a evidência tem que mostrar é verificável à mão a partir da tabela: Opus 96k dá 43,2 MB por ouvinte-hora (96000 vezes 3600 dividido por 8), AAC-LC 256k dá 115,2 MB, e FLAC lossless não tem bitrate fixo nem transcode no repo para medir, então esse profile só pode ser fechado com a ressalva explícita de que o número é pendente de medição real. |
| Hardware | nenhum: a TASK produz um documento de política e o orçamento sai de bitrate declarado por profile e de preço de egress/transcode/storage citado com fonte e data, não de execução; não há placa de áudio, Android, iOS ou macOS envolvidos e o único número que dependeria de medição (FLAC lossless) fica declarado como pendente porque nenhum transcode existe no repo | ausência é limitação registrada, não aprovação |

## DoD — pronto para review

- [ ] Todos os critérios de aceite com evidência
- [ ] `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` limpos
- [ ] `cargo test` verde; testes ignorados listados com motivo
- [ ] Spec do módulo atualizada se a implementação revelou algo novo
- [ ] `backlog.json` com o estado correto
- [ ] Revisão feita por contexto limpo, não pelo implementador
- [ ] PR aberto com template preenchido; CI Gate verde no SHA final
## Notas de execução

Preenchido durante a execução, não antes.
