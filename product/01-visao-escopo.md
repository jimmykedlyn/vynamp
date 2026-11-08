# Visão, público e fronteiras do produto

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Resultado esperado

Um player rápido, personalizável e confiável que funciona sem conta para arquivos locais e rádios públicas. A interface clássica é o padrão inicial, com três painéis reconhecíveis: transporte, equalizador e playlist. Uma biblioteca expansível resolve acervos grandes sem transformar o player em um navegador pesado.

Públicos: entusiastas de áudio e DACs; usuários de bibliotecas locais; ouvintes de rádios e podcasts; criadores de temas, visualizações e integrações. A qualidade vem da ausência de cortes, precisão de reprodução e transparência da cadeia, e não de promessas subjetivas de “melhor som”.

## Incluído no produto completo

Reprodução local e remota, filas e múltiplas playlists, biblioteca e tags, gapless, ReplayGain, EQ gráfico/paramétrico, efeitos, hi-res, perfis de saída, temas, visualizações, plugins próprios, rádios, podcasts, audiobooks, conversão, gravação autorizada, CD e integrações de sistema. Plataforma futura para catálogo, estações, streaming sob demanda, contas, sincronização e distribuição de extensões.

## Fronteiras

- Sem ABI, skins, scripts MAKI, mensagens IPC ou DLLs de Winamp/AIMP/Poweramp. Nenhum pacote antigo é importado como tema/plugin.
- Sem copiar código, sprites, logos, fontes proprietárias ou layouts de skins comerciais. A composição clássica e a paleta proposta são inspiração e criação própria.
- Windows, Linux, macOS Intel, macOS Apple Silicon e Android são alvos obrigatórios desde M0/M1 e do release1.0. Baselines/ABIs e gates estão na [matriz multiplataforma](../platforms/01-matriz-suporte.md). Windows/Linux ARM64 adicionais e Android32-bit são expansão, não substituem Mac arm64/Android arm64 obrigatórios.
- Android faz parte da1.0, com shell/touch/background e controles de mídia próprios. iOS, Android Auto completo, EQ global do sistema, serviços comerciais com DRM, reprodução de vídeo e marketplace pago não integram1.0. Video playback do Winamp fica explicitamente fora deste produto de áudio; não é omissão involuntária.
- VST3, casting, SACD, formatos raros e DSD nativo são M5, com requisitos e tarefas. A versão 1.0 inclui DSD convertido para PCM quando o backend validado permitir; não promete DSD nativo.
- A plataforma não depende de Spotify/YouTube nem de extração não documentada de mídia. Provedores entram por APIs autorizadas.

## Princípios

Local first; controles previsíveis; recursos avançados descobertos progressivamente; processamento de áudio isolado de UI/rede; nenhuma telemetria obrigatória; cada transformação audível identificável; pacotes portáveis e versões de esquema explícitas; implementação própria com bibliotecas auditáveis.

## Métricas de produto

Concluir “abrir arquivo → tocar” sem onboarding obrigatório; restaurar fila após crash; localizar uma faixa em biblioteca de 100 mil itens; abrir rádio por URL; informar formato de origem e saída real; instalar e reverter tema sem interromper áudio. Medir sucesso com cenários QA, não somente número de recursos.
