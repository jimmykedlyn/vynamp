# Auditoria de cobertura e correções 0.6

Versão 0.7.0 · 2026-10-14 · Auditoria documental; produto não implementado. Preservados Rust, cinco plataformas, marca C, layouts/tokens e independência de Winamp. [Fontes](../research/03-completeness-sources.md), [backlog](../../tasks/backlog.json) e [QA](../quality/04-integration-codecs-dsp.md).

## Lacunas fechadas nesta revisão

| Item | Correção/contrato | Tarefas | QA |
|---|---|---|---|
| Abrir com e associações | Registro por OS/capability, escolha de defaults e uninstall próprio | E16-03/04/05/06/09 | QA-022/030/031 |
| Lotes e grants | Envelope, Open versus Enqueue, commit único, URI/FD, cold/warm e idempotência | E16-01/07 | QA-023 |
| Instâncias/perfis | Um owner com lock OS e IPC; perfis separados, installed/portable e migration segura | E16-02/08 | QA-024/031 |
| Formatos recentes e bindings | 68 entradas planejadas; USAC, MPEG-H, AC-4, IAMF, CAF/AIFF-C/AU, variantes lossless/legacy | E17-01/03/04/05/13–18 | QA-025/026 |
| Metadata técnica | Priming/trim/DRC/gain e chapters/read-write independentes | E17-02/06 | QA-025 |
| Sinal final | Corrigir master/mix/resampler depois do limiter; guard final e oráculo no PCM do backend | E17-07/08 | QA-027 |
| Visualização | Famílias, taps, presets limitados, clock, CPU/GPU/fallback/energia/a11y | E17-09/10 | QA-028 |
| Rede/proxy | System/manual/direct, 407, vault e política para cada redirect/segmento | E17-11 | QA-029 |
| Bluetooth | Codec negociado quando observável; transportes não viram formatos locais | E17-19 | QA-025/027 |
| Distribuição divergente | Oito artefatos obrigatórios; Universal2 nos dois Macs; manifest real e gates agregados | E17-12/20, E16-07/08 | QA-030/031 |
| Deep links/provider | Rotas verificadas e PKCE, sem ampliar grants/exec/autoplay | E16-10 | QA-023 |

## Recursos já cobertos, revisados para evitar duplicação

| Família | Contratos existentes | Execução |
|---|---|---|
| Transporte, gapless, A/B, stop-after, clock e queue | [Engine](../audio/01-engine.md), [biblioteca](../library/01-library-playlists.md) | E01/E03 |
| Busca, tags/capas, CUE/playlists, NAS offline, symlinks, ratings/bookmarks/letras | Docs library e segurança; novos detalhes técnicos E17-02/06 | E03/E08 |
| EQ gráfico/paramétrico, compressor/limiter, RG, pitch/tempo, crossfade, efeitos e device presets | [DSP](../audio/03-dsp.md), [sinal](../audio/07-dsp-signal-contract.md) | E04/E17 |
| Hi-res, transparente, exclusive/ASIO e DSD | [Outputs](../audio/04-outputs-hires.md); hardware e captura obrigatórios | E04/E12/E14 |
| Rádio/ICY, HLS live/VOD, cache/download/gravação/timeshift e podcasts | [Cliente](../streaming/01-client.md), [podcasts](../streaming/02-podcasts-recording.md) | E05 |
| Temas originais, editor, sandbox/plugins e SDK | Docs extensions; visualizers agora têm runtime contrato completo | E06/E07 |
| Classic/Library/Mini, UI mobile/tablet, cores/marca, DPI/foco/a11y/localização | Docs design e assets originais preservados | E02/E06/E13 |
| Media keys, SMTC/MPRIS/Now Playing/MediaSession, focus e rotas | Plataformas e integrações; associações aprofundadas E16 | E08/E12/E13/E14 |
| Conversor/encoders, scan RG/BPM, batch rename/dedupe, scheduler/sleep | Ferramentas e jobs persistidos | E08 |
| Scrobble/presence, audiobook, controle remoto autenticado | Integrações e provider; opt-in/privacy | E08/E09 |
| Plataforma web, ingest, CDN, direitos, auth, sync e registry | Streaming platform/OpenAPI; futuro M4 | E09 |
| Claude Code, Superpowers, worktree, claims, Gitflow/GitHub e entrega | Docs engineering e configuração Rust existente | E15 |
| CI, testes numéricos/property/fuzz/E2E/hardware, segurança, SBOM/licenças | Docs quality/engineering; novos QA-022–031 | E10/E11/E14/E15/E17 |
| Installers, assinatura, atualização/rollback, backups/recovery | Docs delivery/engineering; nada publicado remotamente | E11/E15/E16 |

Links por família também em [índice](../INDEX.md) e [paridade](06-feature-parity.md). Catalogar não substitui implementar, testar ou adquirir SDK/licenças. Nome/marca aprovados não comprovam disponibilidade jurídica global do nome.

## Limites e decisões futuras rastreadas

M0 fecha backends/toolkit, bibliotecas nativas, fixtures/licenças e orçamentos por target. Formatos required comuns até M3 precisam das cinco provas; conditional precisa decisão e motivo quando indisponível. M5 inclui imersivos, rare/legacy, CD, VST3, DASH/cloud/casting, DSD nativo e arquiteturas extras. DRM comercial, vídeo/DVD, EQ global/DVC proprietário e skins/plugins Winamp ficam fora conforme direção do produto. Nenhum desses itens é ocultado como implementado.

Ícones nativos finais ainda dependem de export/review do conceito C; PNGs e previews continuam incluídos. Aplicativo, áudio real, hardware, CI remoto, GitHub auth/proteções e assinatura não foram executados por esta auditoria. As três flags de readiness permanecem false.

Não há promessa de que uma revisão capture todo formato futuro. A cada release atualizar versão/fontes/catálogo, inventário e limitações; revisão trimestral monitora novos codecs/containers e CVEs. Qualquer lacuna nova gera REQ/TASK/QA e passa pelo mesmo gate.
