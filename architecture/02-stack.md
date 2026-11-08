# Stack, seleção e política de dependências

Versão: 0.8.0 · Data: 2026-11-03 · Estado: especificação proposta, implementação não iniciada.

| Camada | Preferência proposta | Prova antes congelar |
|---|---|---|
| Linguagem/build | Rust stable, Cargo workspace, rustfmt/clippy | MSRV definida por lockfile/CI, sem “latest” no release |
| UI desktop | egui/eframe + AccessKit + wgpu | Windows/Linux/macOS Intel+Silicon, leitores de tela/DPI M0 |
| UI Android | Kotlin/Compose, service Media3, bridge JNI/UniFFI/C ABI proposta | TalkBack, focus, lifecycle, buffer ownership e arm64 M0 |
| Decode | vynamp-decode (própria; API real: PCM, WAV/RF64/W64, AIFF, FLAC; demais formatos planejados por camada, [ADR-011](../adr/ADR-011-pilha-propria.md)) | Perfis/container/gapless/seek e custo hi-res |
| Codecs adicionais | adapters libopus/libFLAC/FFmpeg/helper | Cobertura, licenças e isolamento |
| Saída shared | vynamp-output (própria; core em PR #1, backends WASAPI/ALSA/CoreAudio planejados, [ADR-011](../adr/ADR-011-pilha-propria.md)) | Latência, device events e formato real |
| Saída avançada | vynamp-output Camada 2 (WASAPI exclusive, ALSA hw, CoreAudio hog); ASIO fora do núcleo (adapter privado opcional futuro) | Exclusive/event-mode/PCM24/32 e driver ocupado |
| Resampling | Nova biblioteca própria de resampling (nome a definir, [ADR-011](../adr/ADR-011-pilha-propria.md)) | Resposta, alias rejection, CPU/latência e distribuição |
| Tags | vynamp-decode-meta (leitura: plano 04 de vynamp-decode; escrita: escopo futuro a definir) | Roundtrip e unknown fields |
| Storage | SQLite/rusqlite/FTS5 | Migrations, Unicode search,100k dataset |
| Network | reqwest/rustls + Tokio | Cancel, proxy, TLS, Range e HLS helpers |
| Plugin | Wasmtime + WIT/host capabilities | Budgets/traps/permissions e footprint |
| Visuals | wgpu/cpu fallback, FFT crate validada | GPU fallback/accessibility/render budget |
| Dados | serde, UUID, time, tracing, thiserror candidatos | Schemas, redaction e errors |
| Segurança tooling | cargo-audit/deny, fuzz/proptest, SBOM | Licenças/lockfile/fixtures |
| Web futuro | Axum, PostgreSQL, S3/CDN, workers | Carga, custo e região após desktop |

Não impor pure Rust a decoder em detrimento de correção; adapters nativos são permitidos em workers, com inventário. Não usar wrapper que impossibilite clock, gapless, resampling e output negotiation exigidos. Rodio pode servir comparação/protótipo, não decisão automática para engine avançada.

Versões exatas, features, hash e licença entram em `Cargo.lock` e ADR ao implementar M0; este pacote não inventa lockfile sem resolução. Releases pinam toolchain/lockfile, checksums de native dependencies e imagem CI. Dependabot/renovação automatizada com tests de áudio; nenhuma atualização de codec sem regressão.

Gate UI: benchmark com100k rows virtualizadas, screen reader e custom classic UI. Se reprovar, comparar Slint/Iced e registrar custos/licença; produto não depende de toolkit específico. Gate RT: callback allocations zero via allocator instrumentation; Wasmtime não precisa estar carregado quando plugins desabilitados. Gate license: fontes/icons/assets/native libs e distribuição por plataforma revisados.
