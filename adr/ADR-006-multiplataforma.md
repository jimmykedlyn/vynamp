# ADR-006 — plataformas de primeira classe

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

Status: escopo aceito por instrução explícita do usuário; tecnologias/baselines propostas para gateM0.

Decisão: Windows, Linux, macOS Intel, macOS Apple Silicon e Android são obrigatórios em 1.0. Linux/Mac deixam a condição de expansãoM5. Android tem núcleo Rust compartilhado e shell nativo Kotlin/Compose proposto com lifecycle/media APIs, além de UI/adapters específicos.

Alternativa rejeitada de escopo: lançar apenas Windows e postergar portabilidade para depois1.0. Não confundir núcleo portátil com produto multiplataforma testado. Universal2 deve conter x86_64 e arm64 válidos, execução Silicon sem Rosetta. Android arm64 físico obrigatório, emulatorx64 auxiliar.

Consequências: mais gates/infra/hardware, bridgeownership/URI abstractions, saída de áudio e armazenamento por capabilities, sem forçar UI desktop mobile. Não duplicar regras de fila/DSP emKotlin. WASAPI/ASIO/tray/docking não são recursos universais; prover equivalentes apropriados. Testes comuns e específicos obrigatórios antes stable, cronograma reestimado sem alegar manutenção do esforço anterior.

Ver [matriz](../platforms/01-matriz-suporte.md), [Android](../platforms/02-android.md), [design mobile](../design/05-mobile-tablet.md), [backends](../reference/audio/05-platform-backends.md) e [QA](../quality/03-matriz-multiplataforma.md).

## Registro do passo 43 — congelada com ressalva medida

Medido em 2026-11-09.

**A parte já decidida está decidida.** O `Status` desta ADR diz, na própria linha 5,
que o **escopo** foi aceito por instrução explícita do usuário: Windows, Linux,
macOS Intel, macOS Apple Silicon e Android obrigatórios em 1.0. Isso é decisão de
mantenedor, não de medição, e o passo 42 não mexe nele. O escopo está congelado.

**A parte técnica não, e é isso que o `Status` separa:** "tecnologias/baselines
propostas para gateM0". O gate M0 do Android é a TASK-E13-01, medida como ausência
em 2026-11-09 — `apps/player` tem 3 linhas, não há adapter AAudio (em
`libs/output` só existe `NullHost`) e **não há aparelho arm64 físico nesta máquina**,
que é o que o AC-1 exige ("um aparelho arm64-v8a real com Android 10 ou superior").

As baseline técnicas ficam propostas, com as decisões já escritas em
[ADR-012](ADR-012-integracao-shell.md) (aceita pelo mantenedor em 2026-11-04, que
complementa esta ADR) e em [platforms/01-matriz-suporte.md](../platforms/01-matriz-suporte.md).

**O que o mantenedor decidir agora:** se aceitar congelar só o escopo — o que o
`Status` atual já declara —, nada muda. Se quiser congelar as tecnologias também
(Kotlin/Compose no shell, núcleo Rust compartilhado), falta a prova do AC-1 de
E13-01: aparelho físico, AAudio sem JNI/alloc e build compatível com requisitos de
página. Essa prova é hardware, não escrita.
