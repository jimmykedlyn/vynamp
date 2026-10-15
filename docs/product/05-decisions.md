# Decisões, hipóteses e pontos em aberto

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Decisões fixadas pelo pedido

Produto independente em Rust; inspiração visual clássica com cores especificadas; temas e plugins próprios; streaming cliente agora e plataforma futura; ampla funcionalidade de áudio; incluir hi-res e recursos inspirados em AIMP e Poweramp. Não exigir compatibilidade Winamp.

## Hipóteses adotadas para execução

| ID | Decisão provisória | Razão / revisão |
|---|---|---|
| H-01 | Substituída: Windows/Linux/macOS Intel+Silicon/Android obrigatórios | Instrução do usuário; ADR-006; matriz/gates M0–M3 |
| H-02 | Resolvida: Vynamp, identidade C — Pulso | Escolha explícita do usuário; ADR-008; exports nativos antes M3 |
| H-03 | Sem conta para uso local | Reduzir dependência da plataforma |
| H-04 | Desktop UI Rust; Android shell Kotlin/Compose proposto, core Rust comum | Provar a11y/bridge/lifecycle em M0; não duplicar engine |
| H-05 | Desktop 1.0 gratuito sem anúncios no player | Política comercial ainda em aberto; não embutir billing no núcleo |
| H-06 | Temas/assets e implementação novos | Evitar acoplamento ao projeto anterior |
| H-07 | PCM até 384 kHz como alvo 1.0 | 768 kHz e DSD nativo em expansão, dependentes de hardware |
| H-08 | Web M4 com live e VOD | Não construir backend antes de validar cliente |

## Decisões pendentes com dono e prazo

UI final e backend exclusivo: responsável engenharia, M0, provas de acessibilidade/latência. Licença do código e marca: responsável produto, antes publicar M3. Encoder AAC/MP3, ASIO e FFmpeg: engenharia/revisão de distribuição, antes empacotar. Hosting/região/custos/CDN: plataforma, entrada M4. Catálogo próprio, royalties e territórios: produto/plataforma, antes ingest público. DRM e assinatura: nova RFC, não dependência de M1–M3. Datas dependem de equipe/hardware; estimativas por tarefa são faixas, não calendário prometido.

## Gestão de escopo

Adicionar requisito exige ID, fase, critérios mensuráveis e tarefa. Remover exige decisão explícita, atualização da matriz e changelog. “Todas as funcionalidades” significa cobertura da matriz acordada e do catálogo de recursos deste pacote, não afirmação de equivalência binária ou de cada opção histórica dos três produtos.
