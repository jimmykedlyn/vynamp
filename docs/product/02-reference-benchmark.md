# Referências de mercado e interpretação

Versão: 0.7.1 · Data: 2026-11-03 · Estado: especificação proposta, implementação não iniciada.

Pesquisa de páginas oficiais em 2026-10-14. Referências são inspirações de experiência; a lista de requisitos do Vynamp é projeto próprio. Produtos mudam e funcionalidades em beta não equivalem a disponibilidade estável.

| Referência | Aprendizado aplicado | Adaptação própria |
|---|---|---|
| Winamp clássico | Player compacto, painéis de EQ/playlist, personalização e visualizações | Composição reconhecível, escalável e acessível, assets e formatos próprios |
| AIMP | Biblioteca, várias playlists, utilitários e saídas avançadas | Biblioteca local, fila independente e ferramentas em jobs canceláveis |
| Poweramp | Ajustes detalhados de áudio e perfis por dispositivo | EQ gráfico/paramétrico, painel da cadeia de áudio e perfis desktop |
| foobar2000 | Reprodução precisa e modularidade | Gapless verificável, ReplayGain, contratos simples |

Fontes primárias: [repositório de referência Winamp](https://github.com/manfromafar/winamp); [AIMP: funcionalidades](https://aimp.ru/?do=features&os=windows); [Poweramp: produto e beta](https://powerampapp.com/); [foobar2000: visão geral](https://www.foobar2000.org/?page=Overview).

O AIMP documenta organização, ferramentas e saídas WASAPI/ASIO. O Poweramp descreve EQ e perfis de saída, além de capacidades avançadas condicionadas ao dispositivo; sua página distingue uma nova engine beta. Não transportar funcionalidades específicas de Android como se fossem APIs desktop.

## Critério “melhor disponível”

Escolher tecnologias pelo conjunto: qualidade de áudio demonstrável, estabilidade, licença de distribuição, portabilidade, acessibilidade e custo de manutenção. Um benchmark técnico M0 é obrigatório antes de congelar crates/versões. Não existe prova, nesta spec, de que uma biblioteca seja universalmente a melhor.

## Tecnologias de referência

Decoders e saídas Rust de terceiros foram avaliados; a decisão e a pilha própria estão no [ADR-011](../architecture/adr/ADR-011-own-audio-stack.md). Nenhuma saída multiplataforma garante exclusivo/bit-perfect por si só. [egui](https://github.com/emilk/egui) é candidato para UI Rust customizável. [Wasmtime](https://docs.wasmtime.dev/security.html) orienta sandbox de extensões. [FFmpeg](https://ffmpeg.org/legal.html) exige revisar configuração/licenciamento do build, não apenas citar o nome.

Normas: [HLS RFC 8216](https://www.rfc-editor.org/info/rfc8216/) para streaming; [HTTP RFC 9110](https://www.rfc-editor.org/info/rfc9110/) para semântica de transferência; [WCAG 2.2](https://www.w3.org/TR/WCAG22/) como referência de contraste, foco e alvos, complementada por testes desktop com leitor de tela.
