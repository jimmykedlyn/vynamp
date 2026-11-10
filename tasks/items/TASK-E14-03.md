# TASK-E14-03 — Entregar paridade de presets e schema de temas

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E14-03` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E14-03` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M2 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E14-02`, `TASK-E06-01` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–12 dias úteis |
| Risco | alto |

## Entrega

Registry de capabilities por OS e perfis desktop/phone/tablet de tema e preset.

## Boundary

- **Não** define nem versiona o schema de tema. Schemas de manifest, tokens e layout, o parser e os erros por JSON Pointer são `TASK-E06-01`, e rejeitar schema major desconhecido ou detectar ciclo aqui seria um segundo dono do mesmo contrato.
- **Não** implementa a curva do EQ nem valida Nyquist das bandas. O filtro, a tolerância de ±0,2 dB contra referência e a recusa de import inválido são `TASK-E04-04`; esta TASK só prova que o mesmo preset responde igual nos dois caminhos que consomem esse filtro.
- **Não** instala tema de forma atômica. Staging, validação completa, commit por versão e restore do tema anterior quando o renderer falha são `TASK-E06-02`, e reimplementar o swap aqui criaria dois caminhos de instalação.
- **Não** descobre nem verifica codec. O catálogo por binding, as capabilities e o corpus são `TASK-E17-01` e o decoder registry é `TASK-E01-04`; o registry desta TASK projeta o que já foi verificado, e a execução do corpus por alvo é `TASK-E14-04`.
- **Não** constrói a shell phone, tablet ou foldable. Navegação, TalkBack, classes de largura e reflow a 200% são `TASK-E13-05` e `TASK-E13-09`; aqui o perfil phone é um nome de layout e um conjunto de tokens, não uma tela do Android.
- **Não** entrega galeria nem publicação de temas. A galeria local e o recovery visual são `TASK-E06-08` e o registry online com revogação é `TASK-E09-09`, ambos depois desta paridade.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e14-03-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E14-03-AC-1` | um pacote .vyntheme cujo manifest.json declara só as chaves classic, library e mini em layouts, como schemas/theme-manifest.schema.json exige, e cujos tokens de cor saem do enum de theme-layout.schema.json | o tema é carregado nos cinco perfis de suporte, classic, library, mini, phone e tablet, e o renderer resolve o layout de cada perfil antes de pintar | Tema carrega com fallback em todos |
| `TASK-E14-03-AC-2` | o catálogo config/format-catalog.json com 68 entradas, todas em state planned e available false, e o support() de libs/output-core, que devolve None para wasapi, alsa, pipewire, pulse, coreaudio e aaudio e LibraryVerified para null e capture | o registry de capabilities é consultado por plataforma no alvo windows-x64 e a tabela resultante é comparada com docs/support-matrix.md, que é a cópia comitada da saída de cargo run -p vynamp-output --example support_matrix | registry mostra formatos reais |
| `TASK-E14-03-AC-3` | um preset compartilhado com preamp_db -3 e três bandas, low-shelf em 80 Hz com +4 dB e Q 0,7, peaking em 1 kHz com -2 dB e Q 1,4 e high-shelf em 12 kHz com +3 dB e Q 0,7, avaliado a 48 kHz nos dois caminhos de implementação | a resposta em frequência do preset é medida em 31 pontos log-espaçados entre 20 Hz e 20 kHz nos dois caminhos e as duas séries são subtraídas ponto a ponto | preset compartilhado tem resposta numérica equivalente |
| `TASK-E14-03-AC-4` | um layout phone e um layout classic do mesmo tema, onde o phone declara nós com as ações open-url e show-library que não existem no alvo móvel e o classic declara docking e tray que não existem no telefone | o renderer resolve cada nó-ação nos dois perfis e a lista de ações efetivas do tema é lida para cada alvo | ações não aplicáveis explícitas. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | uma fixture nova em tests/fixtures/themes/, um .vyntheme montado como ZIP com manifest.json, tokens.json, layouts/classic.json, layouts/library.json e layouts/mini.json, exercitada por um teste de paridade que percorre os cinco perfis, mais a medição de resposta do preset nos dois caminhos; hoje não existe leitor de .vyntheme nem avaliador de EQ — apps/player/src/main.rs tem 3 linhas de println!, libs/dsp/src/lib.rs é só um const VERSION, e o único código de tema que roda é xtask/src/validate.rs::schema_example, que valida schemas/examples/theme-manifest.json e theme-layout.json contra os dois schemas draft 2020-12 | o pacote carrega nos cinco perfis com zero erro, e phone e tablet registram fallback para o builtin com o motivo nomeado em vez de sumirem em silêncio; a resposta do preset fica dentro de ±0,2 dB entre os dois caminhos em todos os 31 pontos (QA-005); o registry mostra 0 formatos available, porque as 68 entradas de config/format-catalog.json estão em planned e docs/support-matrix.md tem as 6 colunas de wasapi, alsa, pipewire, pulse, coreaudio e aaudio em '-' e só null e capture em library-verified; o que ainda não existe hoje: leitor de pacote, avaliador numérico de preset e qualquer linha de registry por OS fora do support() de libs/output-core |
| Hardware | Windows 11 x64, que é esta máquina, cobre o perfil desktop, a medição numérica do preset e o registry de wasapi; não existem aqui Linux x64, Mac Intel, Mac Silicon nem aparelho Android arm64, e a AC-1 pede o tema carregando nos cinco perfis, então os perfis phone e tablet ficam sem prova visual até haver aparelho — ausência registrada, não aprovação | ausência é limitação registrada, não aprovação |

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
