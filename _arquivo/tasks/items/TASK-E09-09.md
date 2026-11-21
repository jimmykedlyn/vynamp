# TASK-E09-09 — Entregar registry de temas e plugins

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E09-09` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E09-09` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M4 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E09-04`, `TASK-E07-08`, `TASK-E06-08` |
| Bloqueia | `TASK-E06-09`, `TASK-E07-09`, `TASK-E09-10` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 8–20 dias úteis |
| Risco | alto |

## Entrega

Admin de publisher, validação de pacotes, assinatura, versão imutável e revogação.

## Boundary

- **Não** constrói o corpus de fuzz de parser e de pacote. Os fuzz targets e o corpus adversarial automatizado são `TASK-E10-05`; aqui o corpus é um caso por cláusula do aceite, escrito à mão e versionado como fixture, e servir de semente ao fuzzer não é escrever o fuzzer.
- **Não** escreve o CLI de pacote do lado do autor nem o relatório de compatibilidade. `pack`, `sign`, `validate` e o harness offline do SDK são `TASK-E07-08`; esta TASK é o serviço que recebe o que esse CLI produziu, e reimplementar o validador no servidor daria duas cópias para divergir no primeiro caso de borda.
- **Não** implementa o parser de tema nem os erros por JSON Pointer do manifest e dos layouts. Schema e validador de tema são `TASK-E06-01`; o serviço de registry confia no laudo deles e só recusa o que a política de pacote recusa, senão a geometria e o contraste de tema ganham dois donos.
- **Não** aplica a revogação no host do player nem interrompe chamada em curso. A verificação de assinatura, a lista de revogados e o corte de chamada no cliente são `TASK-E07-09`, que esta TASK bloqueia; aqui se publica o estado assinado e imutável, lá se consome.
- **Não** implementa o OIDC nem a emissão do escopo `admin.registry` que o endpoint exige. Identidade, roles e isolamento por conta são `TASK-E09-03`; o registry consome o escopo e não decide quem o recebe.
- **Não** autora o contrato OpenAPI de pacotes. Os paths já existem em `schemas/openapi-platform.json` e o contrato é `TASK-E09-02`; mudar o schema no meio da implementação faria o contract test medir outro contrato, o que é pior do que ajustar o código ao schema.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e09-09-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E09-09-AC-1` | um corpus montado à mão a partir do aceite de `extensions/02-plugins.md` e dos limites de `extensions/01-temas.md`: zip com entrada `../../evil.wasm`, symlink que escapa do diretório, razão de compressão 120:1, 201 arquivos, manifest de 300 KiB, `schema_version` major 2, `sdk_version` de major anterior, WASM em loop infinito, WASM que cresce 80 MiB, tema com imagem de 4097 px, SVG com `<script>` e SVG com `href` externo | cada pacote do corpus percorre as duas etapas do contrato congelado — upload do zip em `POST /v1/admin/assets/upload`, finalize em `POST /v1/admin/assets/{id}/finalize` e submissão em `POST /v1/admin/packages` com o `asset_id` devolvido e uma `Idempotency-Key` própria —, e o validador registra o motivo da recusa por arquivo e o estado do catálogo depois de cada envio | Rejeitar pacotes adversariais |
| `TASK-E09-09-AC-2` | um `.vynplug` com 5 arquivos cujo manifest declara o sha256 de cada um e cujo `Package.sha256` cobre o zip inteiro, mais uma cópia do mesmo pacote com 1 byte invertido em `module.wasm` depois da assinatura | o validador recalcula o sha256 de cada arquivo extraído e o do artefato completo e compara com o que o manifest e o `Package` declaram, para as duas cópias | conferir hash por artifact |
| `TASK-E09-09-AC-3` | um par Ed25519 efêmero gerado em `tempdir`, com a metade privada lida de um secret fora do repositório e a metade pública servida ao lado da assinatura na resposta de `GET /v1/packages/{id}/versions/{version}`, que hoje não tem campo próprio de chave pública, mais a API em nível de log debug e o histórico alcançável por `cargo xtask check-secrets` | um pacote é assinado e submetido, e em seguida a resposta HTTP, os logs do processo, o corpo de `GET /v1/packages/{id}/versions/{version}` e a saída de `cargo xtask check-secrets` são varridos em busca do material privado | não expor chaves privadas |
| `TASK-E09-09-AC-4` | uma máquina sem conta, sem sessão OIDC e sem rede, com `sample.vynplug` e `sample.vyntheme` copiados à mão para o disco a partir de um pendrive | o player instala os dois arquivos locais pela tela de extensões, com staging, validação completa e commit atômico por versão, sem abertura de navegador nem chamada ao registry | instalação manual continua sem conta. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `cargo test --test registry_adversarial --locked` no crate do serviço de registry, alimentado por `tests/fixtures/registry/adversarial/` com um diretório por caso e um `expected.json` com o motivo; hoje não existe nada disso: nenhum `Cargo.toml` do repo depende de axum ou de crate de assinatura, e as quatro operações de pacote (`submitPackage`, `listPackages`, `getPackageVersion`, `revokePackageVersion`) vivem só no JSON do OpenAPI | 12 recusas com motivo estável e zero versões criadas ao fim do corpus; 5 hashes de arquivo e 1 do artefato conferidos nos dois lados, com a cópia de 1 byte invertido recusada; zero ocorrências da semente privada na resposta, no log debug e na varredura do histórico; `sample.vynplug` e `sample.vyntheme` instalados a partir do disco sem conta e sem socket aberto. O que ainda não existe: entre os 168 arquivos `.rs` do repo `vynamp`, nenhum é de servidor; as duas assinaturas implementadas são a de release em `xtask/src/signing.rs` (PFX e signtool do Windows) e a do manifest de update em `xtask/src/release.rs` linhas 266 e 280 (minisign), nenhuma delas assinatura de pacote; o lado do player que AC-4 exercita também não existe, porque `apps/player/src/main.rs` tem 3 linhas e um `println!`, `vynplug` e `vyntheme` não aparecem em nenhum `.rs` e nenhuma fixture `sample.vynplug` ou `sample.vyntheme` está versionada |
| Hardware | nenhum: o corpus é de arquivos em disco e o validador roda em processo local; nenhum passo abre dispositivo de áudio nem aparelho móvel, então a placa de áudio desta máquina não entra como evidência; um HSM ou KMS para a chave privada não existe neste ambiente, então a prova de AC-3 usa par efêmero em `tempdir` e isso fica registrado como limitação em vez de aprovado | ausência é limitação registrada, não aprovação |

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
