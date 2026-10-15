# Backlog executável e mapa de dependências

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

176 tarefas em 17 épicos; todos os itens estão planejados. Cada tarefa tem requisito, fase, prioridade, entregável, dependências, aceite, QA e faixa inicial de esforço. Nenhuma tarefa está concluída por este pacote documental.

O [JSON canônico](backlog.json) é fonte para tooling; os Markdown são a interface de leitura. Alterar ambos ao mudar scope/status. As faixas variam de 2–5 dias para provas a 10–25 dias para expansões; são esforço provisório, não calendário. Antes de execução, decompor itens grandes em subtarefas de 1–3 dias conforme descoberta técnica.

## Épicos

- [E01 — Engine, decode e reprodução](epics/E01.md)
- [E02 — Interface, layout e acessibilidade](epics/E02.md)
- [E03 — Biblioteca, playlists e metadados](epics/E03.md)
- [E04 — DSP, outputs e hi-res](epics/E04.md)
- [E05 — Streaming, rádios e podcasts](epics/E05.md)
- [E06 — Temas, editor e visualizações](epics/E06.md)
- [E07 — Plugins próprios e SDK](epics/E07.md)
- [E08 — Ferramentas, jobs e integrações](epics/E08.md)
- [E09 — Plataforma web e provider](epics/E09.md)
- [E10 — QA, segurança eperformance](epics/E10.md)
- [E11 — Fundação, CI e distribuição](epics/E11.md)
- [E12 — Expansões de plataforma e áudio](epics/E12.md)

## Sequência inicial

Leia [NEXT](NEXT.md). Foundation → decoder/output/UI/plugin spikes → playback/queue → classic UI → library/network → extension/themes → hi-res/DSP/HLS/tools → quality/release → platform → expansions.

Dependências incluem gates entre trilhas; títulos não indicam que tarefas possam ser executadas antes das dependencies. IDs de etapa não são ordem global. O validador verifica IDs existentes, ciclos, dependência em fase futura e cobertura de requisitos.

## Template e evidência

[Template](TASK-TEMPLATE.md) e [matriz de evidência](EVIDENCE.md). Um item só muda para accepted depois de registrar teste/artefato/limitações e revisão. UI/demo e tests/documentação podem integrar o mesmo item; não multiplicar trabalho por testes que espelham implementação.

- [E13 — Android](epics/E13.md)
- [E14 — Portabilidade/gates](epics/E14.md)

TASK-E12-01/02 foram promovidas paraM3; TASK-E12-03 mantém apenas arquiteturas adicionais Windows/Linux. Mac arm64/Android arm64 fazem parte do mínimo1.0. IDs estáveis preservados. Gate stable inclui TASK-E14-06; backlog de plataformas não depende de stableWindows prévio.

- [E15 — Agentes/Gitflow/CI/instaladores](epics/E15.md)

176 tarefas, 17 épicos. TASK-E11-01/04/09 agora dependem de gates de infraestrutura E15. Superpowers obrigatório para ambos; evidência dos scripts não aprova o aplicativo.

- [E16 — Ativação/associações](epics/E16.md)
- [E17 — Codecs/DSP/visualizers](epics/E17.md)

TASK-E14-06 exige os gates E16-07/08 e E17-12/20. Condicionais M3 exigem relatório; recursos M5 não bloqueiam o comum M3.
