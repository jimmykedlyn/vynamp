# ADR-005 — streaming desacoplado da plataforma

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

Status: proposto.

Decisão: SourceDescriptor/provider port comum para local/radio/VOD; API própria versionada e mock antes backendM4. HLS/AAC para live interoperável e FLAC Range para lossless VOD; sem forçar lossless HLS não testado. Monólito modular + workers/CDN antes microservices. Consequência: player funciona sem conta/plataforma; scale/rights/cost decididos no gateM4.
