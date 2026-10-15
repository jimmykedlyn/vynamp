# Roadmap e gates de entrega

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

| Fase | Entrega | Gate de saída |
|---|---|---|
| M0 | Provas de áudio, UI, plugins e contratos | ADRs aprovados com evidência, não só hello-world |
| M1 | Alpha local nos cinco alvos: playback/queue/EQ10/UI por host/output nativo | Abrir/tocar/seek/pause/stop, formato básico, recuperação e callback disciplina |
| M2 | Beta multiplataforma: biblioteca/tags read, formatos modernos, rádios/podcasts/temas |100k, HTTP/ICY, packages próprios e a11y básica |
| M3 | Versão1.0 Windows/Linux/macOS Intel+Silicon/Android: DSP/hi-res/HLS/tools/plugins por capability | Hardware/soak/security/distribuição/a11y em cada alvo, gate TASK-E14-06 e matriz M1–M3 completa |
| M4 | Provider platform: catálogo/VOD/live/auth/sync/registry | rights/authz/cost/load/restore e mock contract parity |
| M5 | ARM adicional Windows/Linux, DSD nativo/768k, casting/VST/CD/legados | Gates de expansão por capability; não posterga Linux/macOS/Android obrigatórios |

## Ordem crítica

M0 repo/fixtures → decode/output clock → state/queue → UI classic → biblioteca → network/radio → temas/plugins → advanced output/DSP → HLS/tools → release readiness. Contrato provider mock pode avançar depois network antes M4. Plataforma real depende de cliente capaz e ingest rights definidos.

## Organização da equipe

Trilhas independentes: engine/output, product/UI, library/tools, streaming, extension/security e platform futura. Contratos congelados em reunião de design; integração por thin vertical slices. Dependência de tarefa indica bloqueio real, não necessidade de executar todas em fila. Tarefas em [backlog](../../tasks/README.md) com IDs/aceite/estimativa; sem atribuição a pessoas inexistentes.

## Estimativa

Cada tarefa tem uma faixa provisória de esforço por fase (2–5 dias em spikes, maior em entregas complexas); spikes e tarefas complexas explicitam investigação. Não somar faixas como calendário: equipe, dependências, plataformas e validação hardware mudam duração. Reestimar apósM0 e após primeira vertical slice. Não afirmar que toda matriz será entregue em poucas semanas.

## Definição de completo

Código revisado, criteria da tarefa demonstrados, teste apropriado, docs/contracts atualizados, erro/cancelamento/acessibilidade considerados, licença/assets registrados e artifact reproduzível. M1 não é M3; disabled placeholder e label “em breve” não contam como funcionalidade implementada.
