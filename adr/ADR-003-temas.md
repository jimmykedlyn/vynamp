# ADR-003 — UI Rust e temas declarativos

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

Status: proposto, gate TASK-E02-01.

Decisão: prototipar egui/AccessKit/wgpu, desenho clássico original e schema de temas não executável. Alternativas: Iced/Slint após benchmark; Electron não preferido por overhead/controle, mas comparação deve usar dados. Consequência: custom UI exige semântica e snapshots; fonte única de tokens em JSON. A aprovação depende de Narrator/NVDA/DPI, não apenas aparência.

Atualização0.6.0: decisão de toolkit acima é desktop, também sujeita a Orca/VoiceOver nas duas arquiteturasMac. Android possui shell Compose proposto e engineRust compartilhada conforme ADR-006; tokens/roles de temas são comuns, layouts phone/tablet próprios.
