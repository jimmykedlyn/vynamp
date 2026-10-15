# Contrato do SDK de extensões

Versão 0.7.0 · Proposta, a compilar e validar em M0/M3.

[WIT draft](../../schemas/plugin-host.wit) apresenta host e lifecycle mínimo. Não foi compilado neste pacote. As tarefas E07-01/02/08 validam toolchain, interface real e testes de compatibilidade.

## Chamadas e permissões

| Operação host | Permissão | Limites / semântica |
|---|---|---|
| http-fetch | network.domains | GET/HEAD default; POST requer caso documentado; body/payload limitado, timeout, redirects revalidados |
| read-selected-file | files.read.selected_roots | Root handle opaco concedido pelo host; path relativo seguro, sem symlink escape; leitura limitada |
| selected-metadata | metadata.read | Snapshot da seleção, não acesso a paths/secrets de todas as faixas |
| notify | system.notifications | Rate limit e mensagem sanitizada; não simular prompts de senha |

WIT síncrono inicial roda fora do callback e não no UI thread. HTTP bloqueante do guest precisa broker async/worker isolado no host; timeout de I/O não consome budget de compute ilimitadamente. Antes freeze, avaliar async component interfaces; não afirmar suporte ausente do runtime usado.

Outras operações planejadas: library query AST/paginação, mutation por command e revision, playback control, scene/visualizer feed, provider resolve/refresh e config namespaced. Cada tipo de plugin registra os métodos aceitos, schema de input/output, permissões e custo. `invoke(method,payload-json)` é dispatcher tipado pelo host; não é execução de código/script fornecido no payload.

## Lifecycle

Install valida pacote completo em staging. Init recebe configuração sem tokens; registrar capabilities não concede permissões. Start é método de lifecycle explícito no contrato final; invoke carrega request ID/generation por envelope. Stop cancela operações pendentes, fecha handles e descarrega fora da callback. Trap não exige que guest coopere: host libera recursos e atualiza estado. Desinstalar pode manter dados do plugin por escolha do usuário.

Métodos de lyrics recebem metadata ID e retornam texto/LRC, origem/licença e language. Provider retorna SourceDescriptor validado pelo host, nunca instrui executar programa. Visualizer recebe FFT/PCM limitado e retorna scene allowlisted; dropped frames aceitáveis. Utility retorna plano de alterações antes commit, sem escrever o SQLite livremente.

## RPC de helper nativo

Envelope contém protocol_version, request_id, method, payload_length e generation. Transporte named pipe/socket local com validação de peer, token de sessão/handle herdado mínimo e max payload 1 MiB. Shared-memory descriptor inclui região, offset, length e sample format; bounds verificados dos dois lados, não ponteiro cru. Heartbeat/watchdog encerra worker travado, mantendo fonte anterior se possível. A sandbox é política de OS testada, não consequência automática de usar processo separado.

## Versionamento e experiência

Major muda contratos; minor aditivo, negociar capabilities e unknown enums explicitamente. Pacote informa mínimo SDK/app; host rejeita incompatível com mensagem útil. Update com permissões adicionais pede consentimento apenas para diferença. Revoke interrompe calls; rollback de versão mantém settings somente com migration conhecida.

Harness oficial simula permissões negadas, timeout, memória cheia, network failures, event flood e host shutdown. Exemplos do SDK precisam ser executáveis na implementação, com fixtures e licença; nesta spec são contratos e dados ilustrativos. Assinatura de pacote não permite pular testes de comportamento.
