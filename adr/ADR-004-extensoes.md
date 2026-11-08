# ADR-004 — extensões com capabilities

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

Status: proposto, gate TASK-E07-01.

Decisão: WASM para plugins de controle/dados, helpers sandbox para native/decode, DSP third-party adiado até prova de deadline. Não carregar ABI Rust/DLL arbitrária no main process. Alternativa: tudo native in-process, menor latência mas risco e acoplamento elevados. Consequência: SDK/broker/budgets e UX de permissões são parte do produto; sandbox não é garantia absoluta.
