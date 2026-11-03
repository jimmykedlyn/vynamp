# Estratégia de testes e critérios de qualidade

Versão: 0.7.1 · Data: 2026-10-25 · Estado: especificação proposta, implementação não iniciada.

## Camadas

Unit/property tests para state machine, queue, ranges, parser/schema, DSP e sample conversion. Integration tests para decoder/output mocks, biblioteca/migration e stream fixture server. Hardware tests WASAPI/DAC reais. UI tests por interação/semântica e screenshot. Security fuzz nos parsers e limites. End-to-end nos workflows críticos; soak e performance com ambiente registrado.

Fixtures devem ter origem/licença, SHA256, codec/container/profile/rate/channels, expected duration/frames, tags e resultado. Não depender de URL de rádio pública instável no CI. Usar arquivos gerados: tone/impulse/sweep/digital silence/PCM pseudorandom, gapless pairs, corrupt variants e stream scripts. Comprimir/armazenar large fixture fora git quando preciso, download com hash e quota.

## Cenários canônicos

| ID | Cenário | Oráculo / gate |
|---|---|---|
| QA-001 | Open/pause/seek/stop local | State/event generation e relógio consumido corretos |
| QA-002 | Gapless FLAC/MP3/AAC apropriados | concat de frames contra referência, sem artificial zeros/drop |
| QA-003 | Hi-res/source-output | negotiation e UI mostram cadeia real |
| QA-004 | Transparente PCM24/32 | sample-exact capture em combinação validada |
| QA-005 | DSP/preset/rapid change | resposta±0.2dB, sem NaN/click grosseiro |
| QA-006 | Large library/queue restore |100k, deadlines e recovery sem perda |
| QA-007 | Tag writes/crash | source preservada, atomicity e rollback |
| QA-008 | HTTP/ICY/HLS adverse | cancel/retry/generation, sem stale samples |
| QA-009 | Package/theme/plugin adversarial | budgets/grants e safe fallback |
| QA-010 | Screen reader/keyboard/DPI | workflows sem mouse e sem clipping |
| QA-011 | Device lost/sleep/resume | posição preservada, não tocar speaker sem opção |
| QA-012 | Update/migration failure | validação, backup e recovery |
| QA-013 | Platform auth/rights/sync | autorização por objeto e conflitos corretos |
| QA-014 | Stream load/ingest/restore | SLO, cost e backups conforme gate |
| QA-015 | Formats/legacy expansion | fixtures/profile e unsupported correto |
| QA-016 | Tools/podcasts/schedule | jobs canceláveis, progresso e dados preservados |
| QA-017 | OS/visualizer/integration | controles externos corretos, áudio independente |

## Hardware e ambientes

Windows11 x64: 4-core moderno,16GiB RAM,SSD, GPU integrada, interface44.1/48 e DAC USB24/192, driver compartilhado/exclusivo. Baseline reduzido2-core/8GiB para stress (metas separadas). Testes100/125/150/200%DPI e multi-monitor; NVDA/Narrator. NAS lento, rede limitada5Mbps/100ms/1%loss, rede rápida50Mbps/20ms para streaming hi-res. VOD fixture server de referência, sem CDN externa no teste unitário.

## Release gates

M1 smoke local e callback discipline; M2 streams/formats/a11y/library; M3 numerical/hardware/soak/security/update/notices; M4 authz/load/restore; M5 per-platform per-format. Zero bug P0/P1 conhecido no caminho crítico; exceções menores documentadas e visíveis. Coverage percent é apoio, não gate único. Dados/recording corrompidos, secret leaks e audio stalls graves bloqueiam release.

## Gates adicionais obrigatórios

QA-018: Linux/macOS Intel+Silicon nativos. QA-019: Android playback/lifecycle/focus. QA-020: Android arquivos/UX/energia. QA-021: packaging/assinaturas/ABI nos alvos. Detalhes e hardware na [matriz multiplataforma](03-multiplatform-matrix.md). QA comum deve passar em todos os alvos aplicáveis, não apenas Windows.

## QA adicionais 0.6

| QA | Cenário | Oráculo |
|---|---|---|
| QA-022 | Associações/Abrir com nos cinco alvos | Handler candidato abre a mesma fila; defaults de terceiros preservados; sem admin obrigatório |
| QA-023 | Batch, CLI, IPC, deep link e grants | Ordem, commit único, idempotência/Conflict e generation; metacaracteres não executam comandos |
| QA-024 | Owner, perfis, portable e locks | Vinte launches elegem um owner; stale PID não autoriza reset; não há dois writers no perfil comum |
| QA-025 | Codec/container/perfil, encode e metadata | Fixture/hash/oráculo por binding; WebM não aceita FLAC convencional; trim/gain/canais corretos |
| QA-026 | Áudio imersivo, DRC, render e passthrough | Decoder e renderer separados; referência por apresentação/layout; licenças e limitações explícitas |
| QA-027 | Teto true-peak no sinal final | PCM quantizado do backend após mix/gain/resample não excede ceiling +0.1 dBTP; sem NaN ou drift |
| QA-028 | Visualizers, budgets, fallback e acessibilidade | Falha GPU/worker não bloqueia áudio; Android background sem rendering; FFT/tap/Fs coerentes |
| QA-029 | Proxy, stream bindings e reconnect | 407 distinto de 401; credenciais separadas; sem direct fallback silencioso; cancel e ABR seguros |
| QA-030 | Manifest real e associações | Só capacidades verificadas, disponíveis e licenciadas geram handler; catálogo não concede suporte |
| QA-031 | Upgrade, uninstall, portable e backup | Remove apenas registros próprios; preserva mídia/defaults; staging/rollback e lock respeitados |
| QA-032 | Equalizadores 10/20/31/64 + paramétrico 32 (nativo e MVP HTML/SVG/JS) | Cinco modos alcançáveis; exatamente 10/20/31/64 controles e até 32 filtros; centros/Q/Nyquist conforme [equalizadores](../audio/08-equalizers.md); curva = cascata calculada; estado independente por modo; MVP rotula `[DEMO]` o que não processa áudio |
| QA-033 | vitrola (vinil + válvula) integrado ao grafo | Bypass transparente, desativação automática em bit-perfect/DSD, multicanal, picos de estalos, rituais/WearState e combinação com EQ sob QA-027 ([vitrola](../audio/10-vitrola.md)) |
| QA-034 | Fidelidade ao MVP de referência | Gabaritos da [referência do MVP](../design/06-mvp-reference.md) (SVG, tema, ícone, captura, movimento, roteiro) reproduzidos no nativo dentro da tolerância declarada; exceção só para API intrínseca do navegador, com comportamento reimplementado |

QA-032/033 (2026-10-25) detalham a [matriz de testes DSP](05-dsp-test-matrix.md). [Cenários completos](04-integration-codecs-dsp.md). QA-026 nos formatos M5 é futuro; M3 exige decisões/limitações dos condicionais e provas dos codecs comuns.
