# TASK-E10-06 — Automatizar semântica e regressão visual

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E10-06` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | `REQ-E10-06` |
| Critérios da spec | n/a — TASK transversal do umbrella; o critério vive na própria TASK |
| Fase | M2 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E02-06`, `TASK-E06-04` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–12 dias úteis |
| Risco | médio |

## Entrega

Workflows de teclado, screenshots por DPI e checklist de leitores de tela.

## Boundary

- **Não** traduz nem define as strings PT-BR/EN, nem produz os gabaritos de tema e DPI: `TASK-E02-09` é dona do locale e das strings e `TASK-E02-12` do gabarito visual. Aqui as capturas por DPI consomem esses arquivos e reprovam quando eles mudam sem atualização.
- **Não** gera os roteiros de interação de teclado, ponteiro e toque: `TASK-E02-14` produz o roteiro com estado esperado; esta TASK apenas o executa de forma automática e reporta a divergência.
- **Não** implementa o harness de fidelidade ao MVP com `egui_kittest`: isso é `TASK-E02-16`, que roda só localmente e que o CI Gate marca como não executado pela QA-034. Comparar com o MVP não é comparar com o gabarito próprio.
- **Não** implementa a janela Mini, a navegação de preferências nem os estados empty/loading/offline/recovery: `TASK-E02-06` é a dependência e a dona dessas telas, que aqui só são percorridas.
- **Não** produz os três temas nem os assets licenciados deles: `TASK-E06-04` é a dependência. Um tema ausente reprova a captura em vez de ser gerado por esta TASK.
- **Não** faz o relatório de usabilidade e jornadas nem decide ajuste de interação: `TASK-E02-10` valida a experiência ponta a ponta. Evidência automatizada de teclado, DPI e árvore semântica não é aprovação de usabilidade.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e10-06-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E10-06-AC-1` | a árvore semântica capturada de cada tela do player — transporte, playlist, painel DSP, Mini 420×112 e preferências de `TASK-E02-06` — com a lista de controles críticos e ações extraída do roteiro de `TASK-E02-14` e da linha de QA de `design/03-acessibilidade.md`, que exige nome, role, valor e estado estáveis para todos os controles mas não enumera quais são | a suíte percorre cada tela só por teclado e compara a árvore observada com essa lista, nomeando tela e controle quando falta algum | Nenhum controle crítico ausente |
| `TASK-E10-06-AC-2` | os três temas de `TASK-E06-04` (Classic Graphite, Midnight Modern, Copper), Mini em 420×112, strings PT-BR e EN completas, e a preferência de reduced motion ligada e desligada | as capturas e os dumps de geometria rodam em 100, 125, 150 e 200% de DPI, nos dois locales e nos dois estados de movimento, e cada célula é comparada com o gabarito de `TASK-E02-12` | validar DPI, PT-BR/EN e reduced motion |
| `TASK-E10-06-AC-3` | a lista de QA de leitor de tela de `design/03-acessibilidade.md` (abrir, tocar, pausar, seek, volume, playlist e preferências sem mouse, com anúncio da linha tocando e foco visível) e uma máquina Windows 11 com NVDA instalado, em 100% e em 200% de DPI | uma pessoa executa o roteiro no NVDA e anexa gravação e log de fala em `evidence/PR/task-e10-06/`, com data, versão do NVDA e SHA do build | evidência manual de screen reader registrada. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp` | verde no CI Gate |
| Oráculo independente | `cargo test --manifest-path apps/player/Cargo.toml --test a11y --locked` para a árvore semântica e `--test visual` para as capturas em `tests/fixtures/ui/`, mais o roteiro NVDA anexado em `evidence/PR/task-e10-06/` | Hoje `apps/player/src/main.rs` é `fn main() { println!("vynamp_player 0.1.0"); }`, nenhum `Cargo.toml` do repositório cita `egui`, `tests/fixtures/ui/` não existe (o caminho está apenas reservado em `tests/fixtures/README.md`) e não há NVDA instalado nesta máquina. A evidência quando existir: a árvore de cada tela reprovando com nome da tela e do controle ausente; 4 escalas de DPI por 3 temas nos dois locales, cada célula com diferença medida contra o gabarito; zero animação com duração maior que zero quando reduced motion está ligado; nenhuma string caindo para o fallback EN sob locale PT-BR; e o log de fala da AC-3 com o anúncio de linha tocando e do foco visível. QA-010 em `quality/01-estrategia-testes.md` é a linha que esses resultados alimentam. |
| Hardware | um desktop Windows 11 com NVDA instalado e monitor capaz de 100/125/150/200% de escala; esta máquina é Windows 11 Pro, não tem NVDA em `C:/Program Files/NVDA` e não registra sessão multi-monitor, o que bloqueia a evidência manual da AC-3 até a instalação; TalkBack, VoiceOver e macOS não existem neste ambiente | ausência é limitação registrada, não aprovação |

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
