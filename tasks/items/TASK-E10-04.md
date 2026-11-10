# TASK-E10-04 — Criar servidor de streaming adversarial

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E10-04` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E10-04` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M2 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E05-03`, `TASK-E10-01` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–12 dias úteis |
| Risco | médio |

## Entrega

Fixtures HTTP Range/ICY fragmentado/stall/redirect e scripts de timeline.

## Boundary

- **Não** implementa o adapter HTTP com Range, redirects, autenticação, timeouts e cancelamento: isso é `TASK-E05-02`. Esta TASK entrega o servidor que responde errado de propósito; escrever o cliente aqui faria o teste exercitar a si mesmo.
- **Não** implementa o parser ICY, os metadados de estação nem o favorito de rádio: `TASK-E05-03` é a dependência e a dona do cliente Icecast/SHOUTcast. O fixture apenas serve um bloco `StreamTitle` partido entre dois reads.
- **Não** implementa HLS: parser master/media, segmentos, discontinuity e AES-128 são `TASK-E05-06`, e janela live, ABR e timeshift são `TASK-E05-07`. Nenhum `.m3u8` é interpretado aqui; quando existir, ele é servido como corpo opaco.
- **Não** define a política de proxy, CONNECT, trust store ou 407: isso é `TASK-E17-11`. O certificado autoassinado de AC-2 prova que o cliente não repete cegamente, não que a configuração de proxy dele está correta.
- **Não** gera o áudio nem os hashes do corpus: `TASK-E10-01` é a dona das fixtures licenciadas. O MP3 servido vem do corpus existente e é conferido por SHA-256, não recodificado aqui.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e10-04-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E10-04-AC-1` | o servidor fixture local com oito roteiros sobre o mesmo MP3 de teste: 206 com Range contíguo, 200 apesar de Range, stall de 15 s no meio do corpo, cadeia de 5 redirects 302, conexão fechada no byte 4096, ICY com metaint 8192 e bloco de metadados partido entre dois reads, 401 e 416 | a suíte de streaming executa cancel no meio do corpo, retry com backoff, resume por Range e troca de fonte, e registra a geração de cada evento emitido | CI cobre cancel/retry/resume/generation |
| `TASK-E10-04-AC-2` | o mesmo servidor configurado para responder 401 com `WWW-Authenticate` e, noutra porta, para apresentar um certificado autoassinado fora da trust store, com o cliente HTTP em política default | a suíte tenta abrir o stream nos dois cenários e conta as tentativas por erro e o estado final emitido | 401/TLS não entram em retry cego |
| `TASK-E10-04-AC-3` | o script de timeline com seed fixa, atrasos declarados em milissegundos virtuais e o SHA-256 dos corpos servidos, mais o mesmo MP3 de teste de AC-1 | a suíte roda duas vezes, com `--test-threads=1` e com `--test-threads=4`, e os dois logs de timeline são comparados byte a byte | resultados são reproduzíveis. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `cargo test --test adversarial --locked` no crate novo do servidor de fixtures, com o servidor em processo, mais a mesma suíte duas vezes, com `--test-threads=1` e com `--test-threads=4`, comparando os logs de timeline por SHA-256 | Nada disso existe: `tests/fixtures/network/` está apenas reservado em `tests/fixtures/README.md`, nenhum crate do repositório implementa cliente HTTP (os `https://` de `xtask/src/validate.rs` e `xtask/src/package.rs` são literais de URL em validação Git e de assinatura, não cliente) (`apps/player/src/main.rs` é `fn main() { println!("vynamp_player 0.1.0"); }`) e o crate de fixtures não existe. A política que a evidência tem que mostrar está em `streaming/01-cliente.md` §Política de rede: 401/403, codec não suportado e TLS inválido sem retry cego (a spec não fixa o número de tentativas; a evidência registra a contagem e exige que não haja segunda abertura); erro recuperável com backoff 1/2/4/8/16 s, teto 30 s e no máximo 5 consecutivos; redirect no máximo 5; `Authorization` nunca reenviada para outra origem. AC-3 reprova se dois runs diferirem em um único byte do log de timeline. Hoje a contagem de tentativas de retry não existe em nenhum lugar do repositório. |
| Hardware | nenhum: o tráfego é loopback no próprio processo de teste e o relógio é virtual, sem rádio pública nem CDN, como a spec exige; nenhum dispositivo de áudio é aberto porque a suíte compara bytes de corpo HTTP, códigos de status e eventos, não PCM | ausência é limitação registrada, não aprovação |

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
