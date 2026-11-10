# ADR-004 — extensões com capabilities

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

Status: proposto, gate TASK-E07-01.

Decisão: WASM para plugins de controle/dados, helpers sandbox para native/decode, DSP third-party adiado até prova de deadline. Não carregar ABI Rust/DLL arbitrária no main process. Alternativa: tudo native in-process, menor latência mas risco e acoplamento elevados. Consequência: SDK/broker/budgets e UX de permissões são parte do produto; sandbox não é garantia absoluta.

## Registro do passo 43 — não congelado, com o motivo medido

Medido em 2026-11-09. O gate declarado na linha 5 é a TASK-E07-01, medida como
**ausência**: `apps/player` não tem dependência `wasmtime`, não tem host de plugins
e não tem módulo `.wasm` de teste. Os três ACs da TASK exigem, na ordem, um módulo
que estoure fuel, um que estoure memória, um que tente abrir arquivo e soquete, e a
medição de memória/start sem o runtime carregado — nenhum dos quatro tem alvo.

A ADR decide "WASM para plugins de controle/dados, helpers sandbox para
native/decode, DSP third-party adiado até prova de deadline". A terceira parte é
explicitamente adiada; as duas primeiras dependem de um host que ainda não foi
escrito. Congelar registraria a escolha de tecnologia sem a prova de que o modelo
de capabilities e os limites funcionam — que é a parte da ADR que a TASK E07-01
existe para testar.

**Para congelar:** implementar o host, escrever os módulos de teste e medir. Os
ACs da TASK são o plano de como congelar.

**Custo:** se o mantenedor aceitar a escolha de WASM sem o host, a ADR pode ser
congelada com a ressalva de que a prova de limites fica para a implementação. Essa
ressalva é de quem aprova, não de quem registra a medição.
