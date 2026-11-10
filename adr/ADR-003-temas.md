# ADR-003 — UI Rust e temas declarativos

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

Status: proposto, gate TASK-E02-01.

Decisão: prototipar egui/AccessKit/wgpu, desenho clássico original e schema de temas não executável. Alternativas: Iced/Slint após benchmark; Electron não preferido por overhead/controle, mas comparação deve usar dados. Consequência: custom UI exige semântica e snapshots; fonte única de tokens em JSON. A aprovação depende de Narrator/NVDA/DPI, não apenas aparência.

Atualização0.6.0: decisão de toolkit acima é desktop, também sujeita a Orca/VoiceOver nas duas arquiteturasMac. Android possui shell Compose proposto e engineRust compartilhada conforme ADR-006; tokens/roles de temas são comuns, layouts phone/tablet próprios.

## Registro do passo 43 — não congelado, com o motivo medido

Medido em 2026-11-09. O gate declarado na linha 5 é a TASK-E02-01, cujo `Then` é
"falhas mudam toolkit com ADR". A TASK foi medida como **ausência**: `apps/player`
tem `src/main.rs` de 3 linhas que imprime a versão, `[dependencies]` vazio, e nenhuma
dependência egui/wgpu/AccessKit. Não houve spike, não houve plataforma, não houve
leitor de tela.

A ADR decide "prototipar egui/AccessKit/wgpu" e "a aprovação depende de
Narrator/NVDA/DPI, não apenas aparência". Sem protótipo e sem medição, o único que
poderia mudar a decisão é a aparência — exatamente o que a ADR diz que não basta.
Congelar seria registrar uma aprovação que não foi medida.

**Para congelar:** rodar E02-01 com os quatro leitores de tela (Narrator, NVDA,
Orca, VoiceOver) nos quatro sistemas, medir DPI 200% e registrar o resultado na
própria ADR. Enquanto isso a ADR segue proposta.

**Custo desta leitura:** se o mantenedor aceitar o protótipo sem leitores de tela,
a ADR pode ser congelada com a ressalva de que a validação de acessibilidade fica
para E02-02. Essa ressalva precisa estar escrita no `Status`, não só aqui.
