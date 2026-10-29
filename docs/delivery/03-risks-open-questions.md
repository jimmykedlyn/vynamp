# Riscos e decisões de execução

Versão: 0.7.1 · Data: 2026-10-25 · Estado: especificação proposta, implementação não iniciada.

| Risco | Impacto | Mitigação / gatilho |
|---|---|---|
| Escopo reproduz todo ecossistema | Projeto nunca chega a release | Fases e critérios; complete1.0 é M1–M3 |
| UI custom perde a11y | Produto inacessível | GateM0 screen reader, toolkit substituível |
| Hi-res vira claim falso | Perda de confiança | negotiation e pipeline visíveis, testes hardware |
| Crate não cobre HE-AAC/Opus/DSD | Rádio/formato falha | registry por perfil e adapters, prova antes promessa |
| DSP interrompe callback | clicks/dropouts | compiled graph/worker budgets, soak |
| Plugin native comprometido | acesso indevido/crash | process sandbox, grants, optional class |
| Temas ilimitados quebram UI | controles ausentes/crash | schema/constraints/fallback |
| Platform bandwidth alto | custo inviável | CDN/cache/quotas e custo por ouvinte-hora |
| Library migration perde ratings | dado irreproduzível | backup/restore e drills |
| Encoding/distribuição | lançamento bloqueado | license inventory e builds separados |
| Download/record rights | conteúdo indevido publicado | restrictions/rights antes delivery |
| Drivers variam | suporte difícil | capability probes e hardware matrix |
| UI do MVP confundida com DSP real | claim falso de processamento | rótulo `[DEMO — sem processamento de áudio neste modo]`, EQ-T-14 |
| EQ-64/PEQ/FIR excedem orçamento de CPU | underrun/dropouts | benchmark antes do aceite M5 (TASK-E12-10), qualidade/limites declarados |
| Posição do vitrola × resampler final (D-DSP-05) | aliasing ou mudança do contrato 07 | decisão do mantenedor antes de integrar o vitrola |
| Emulações colorativas mascaram clipping ou soam "melhores" só por volume | qualidade enganosa | medidores pré/pós, A/B com loudness igualado, proteção final fixa |

## Pendências reais

Nome/marca e licença final; escala/plano comercial; hardware alvo de DAC e ASIO; política de provider e conteúdos; regulação/regiões do backend; prioridades opcionais iOS/Android Auto; Android base já é requisito obrigatório. Nenhuma bloqueia escrever a spec. Algumas bloqueiam publicação comercial ou assinatura de claims; tasks incluem decisão antes essas ações.

## Processo de alteração

Abrir RFC com objetivo/alternativas/risco/prova/impactoREQ/TASK. ADR define decisão e consequência. Atualizar versão do documento e changelog; manter IDs ou registrar supersession. Antes implementar M5 revisar APIs disponíveis em cada OS e fontes atuais, especialmente codecs e licenças.
