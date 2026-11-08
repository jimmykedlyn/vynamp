# ADR-006 — plataformas de primeira classe

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

Status: escopo aceito por instrução explícita do usuário; tecnologias/baselines propostas para gateM0.

Decisão: Windows, Linux, macOS Intel, macOS Apple Silicon e Android são obrigatórios em 1.0. Linux/Mac deixam a condição de expansãoM5. Android tem núcleo Rust compartilhado e shell nativo Kotlin/Compose proposto com lifecycle/media APIs, além de UI/adapters específicos.

Alternativa rejeitada de escopo: lançar apenas Windows e postergar portabilidade para depois1.0. Não confundir núcleo portátil com produto multiplataforma testado. Universal2 deve conter x86_64 e arm64 válidos, execução Silicon sem Rosetta. Android arm64 físico obrigatório, emulatorx64 auxiliar.

Consequências: mais gates/infra/hardware, bridgeownership/URI abstractions, saída de áudio e armazenamento por capabilities, sem forçar UI desktop mobile. Não duplicar regras de fila/DSP emKotlin. WASAPI/ASIO/tray/docking não são recursos universais; prover equivalentes apropriados. Testes comuns e específicos obrigatórios antes stable, cronograma reestimado sem alegar manutenção do esforço anterior.

Ver [matriz](../platforms/01-matriz-suporte.md), [Android](../platforms/02-android.md), [design mobile](../design/05-mobile-tablet.md), [backends](../reference/audio/05-platform-backends.md) e [QA](../quality/03-matriz-multiplataforma.md).
