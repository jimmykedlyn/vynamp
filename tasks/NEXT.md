# Primeira execução recomendada

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

Não começar pela plataforma ou todos os codecs. Executar TASK-E11-01 (workspace independente/CI), depois os spikes TASK-E01-01, TASK-E01-02, TASK-E02-01, TASK-E04-02, TASK-E07-01 e TASK-E10-01, respeitando dependências. Resolver licenças/trust TASK-E11-02/03 antes ampliar native dependencies.

Primeira vertical slice: abrir WAV/FLAC/MP3 → decodificar → saída shared → pause/seek/stop → fila independente → Classic Graphite com EQ10 e volume. Tasks E01-03/04/05, E03-01/02, E04-01 e E02-02/03/04. Resultado deve tocar de verdade e ter cancelamento/recovery, não apenas tela.

Depois: biblioteca/tags/busca, Opus/AAC/ALAC/Vorbis, rádio por URL e podcast. Congelar contratos de tema/plugin antes editor/registry. Backend web só M4 depois provas do provider mock.

Decisões finais em M0: toolkit e screen-reader gate, decoder feature/version matrix, backend shared/exclusive estratégia, resampler, package/runtime limits e dependências/licenças. Registrar ADR com evidência. Cada candidato pode ser trocado se não atender a spec.

## Atualização multiplataforma0.6.0

Após TASK-E11-01, incluir TASK-E14-01 e TASK-E13-01 em M0. A primeira slice M1 precisa tocar nativamente em Windows/Linux/MacIntel/MacSilicon/Android, com ports E14-02 e shell/service/focus/UI E13-02/03/05. Não completar apenas Windows e declarar núcleo portátil como release multiplataforma. E12-01/02 são portas M3, já com prova inicial M0/M1.
