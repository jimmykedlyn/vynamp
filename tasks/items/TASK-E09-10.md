# TASK-E09-10 — Validar operação, carga e recuperação

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E09-10` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E09-10` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M4 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E09-06`, `TASK-E09-07`, `TASK-E09-08`, `TASK-E09-09`, `TASK-E11-10` |
| Bloqueia | `TASK-E12-08`, `TASK-E12-09` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 8–20 dias úteis |
| Risco | alto |

## Entrega

Teste de 1.000 sessões, custos, restore drill, alertas e runbooks.

## Boundary

- **Não** escreve o IaC, o provisionamento, os secrets nem os runbooks de restore e rollback. Isso é `TASK-E11-10`, que esta TASK consome: aqui se executa o drill e se mede o RPO e o RTO, lá se escreve o que é executado, e escrever o runbook durante o drill apagaria a única coisa que o drill prova, que o documento anterior funcionava.
- **Não** implementa o resolve assinado nem o TTL de URL que a medição de CDN observa. Resolve, direitos e expiração são `TASK-E09-06`; esticar o cache-control para o hit ratio subir transformaria o relatório de operação em parte do objeto medido.
- **Não** otimiza a API para o p95 fechar em 250 ms. Perfil de query, índices e paginação do catálogo são `TASK-E09-04`; um p95 acima do gate vira achado com endpoint, percentil e plano de carga anexados, não patch aplicado nesta TASK.
- **Não** cria o registry nem carga de submissão de pacotes. Isso é `TASK-E09-09`, e submissão é escrita de operador com rate limit próprio, fora do perfil de 1.000 ouvintes; misturar as duas faixas faria a cauda de uma aparecer como regressão da outra.
- **Não** mede startup, callback nem budgets do desktop. Métricas locais e baseline são `TASK-E10-07` e `quality/02-performance.md`; o p95 de 250 ms é da API de plataforma, e o startup p95≤3 s entra no relatório só como série do cliente pela CDN.
- **Não** entrega DASH, WebDAV, clouds nem casting, que dependem deste gate. São `TASK-E12-08` e `TASK-E12-09`, bloqueadas por esta TASK; p95 verde de API não é prova de áudio em navegador nem de bridge pareada.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e09-10-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E09-10-AC-1` | o ambiente de referência declarado no RFC de `TASK-E09-01` com região única, PostgreSQL com PITR, CDN na frente e o catálogo semeado, mais 1.000 sessões simultâneas simuladas divididas em duas passadas separadas, uma de Opus/AAC progressivo e outra de FLAC lossless com Range, cada uma com 5 min de warm-up e 30 min de regime | o gerador mantém as 1.000 sessões de playback em paralelo com a mistura de leitura do gate (catálogo paginado, resolve e sync), e o histograma de latência é coletado no servidor por endpoint, sem contar o tempo de rede do cliente | API p95 ≤250 ms no ambiente de referência |
| `TASK-E09-10-AC-2` | o mesmo cenário de 1.000 sessões, agora com os segmentos e as renditions puxados pela CDN de referência, o cabeçalho `x-cache` de cada resposta registrado e o TTFB do edge amostrado no cliente ao lado da latência de origem | o relatório de carga é gerado com duas séries separadas, servidor e CDN, e o p95 do playback startup é calculado só pela série do cliente | CDN medida separadamente |
| `TASK-E09-10-AC-3` | um ambiente com backup completo diário das 02:00, WAL arquivado a cada 60 s, object storage com versionamento habilitado e o alerta de falha disparando para o plantão; o incidente simulado apaga o bucket de renditions às 14:37 | o restore drill restaura o banco até 14:36:30 e o bucket para a versão anterior, cronometrado do primeiro alerta até o catálogo e o playback voltarem a responder pelo domínio de produção | demonstrar RPO/RTO |
| `TASK-E09-10-AC-4` | uma release N com o contrato de `TASK-E09-02` congelado em `schemas/openapi-platform.json` e a suíte de contract tests passando contra ela, mais a release N+1 já aplicada com sync, resolve e pacotes em uso pelo cliente mock | o rollback para o binário da release N é executado e os mesmos contract tests e os mesmos fixtures de sync e resolve rodam contra o serviço revertido | rollback mantém contratos. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `k6 run tests/load/gate-1000.js` contra o serviço e, no drill, `pg_basebackup` mais `s3api list-object-versions` no bucket de renditions, com o relatório de percentis escrito em `evidence/`; hoje não existe gerador de carga no repo: `tests/` tem só `fixtures/` e um README, e nenhum script de `k6`, `vegeta` ou `oha` aparece em qualquer arquivo versionado | 1.000 VUs por 30 min por passada, com p95 por endpoint: o gate é ≤250 ms no p95 de leitura e o relatório traz o p99 ao lado para não esconder a cauda; CDN em coluna própria, com hit ratio e TTFB do edge medidos no cliente e separados da origem; drill devolve o banco ao ponto 14:36:30 com a perda máxima em segundos declarada como RPO e o tempo até catálogo e playback responderem declarado como RTO; rollback roda a mesma suíte de contrato da release N. Duas coisas não são provadas por isso e ficam escritas como não provadas: o SLO de 99,9%/mês da spec não sai de um run de 30 min, e a suíte de contract tests de `TASK-E09-02` ainda não existe, porque os 25 paths de `schemas/openapi-platform.json` não têm nenhum teste de contrato no repo |
| Hardware | nenhum: a carga é gerada por processo local contra o serviço e o drill restaura backup em disco e bucket S3-compatible; não há placa de áudio, aparelho móvel nem CDN de referência neste ambiente, então os números medidos aqui são do ambiente local e o p95 de 250 ms no ambiente de referência exige a máquina e a CDN do RFC de `TASK-E09-01` | ausência é limitação registrada, não aprovação |

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
