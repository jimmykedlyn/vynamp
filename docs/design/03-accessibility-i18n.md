# Acessibilidade e localização

Versão: 0.7.1 · Data: 2026-10-28 · Estado: especificação proposta, implementação não iniciada.

DEVE oferecer árvore semântica para todos os controles, nomes estáveis, roles, valor e estado; sliders expõem valor/unidade e aceitam teclado. Tema customizado não remove estrutura semântica. Testar Narrator e NVDA no Windows; VoiceOver/Orca na expansão. Toolkit sem boa integração reprova M0.

Tab/Shift+Tab percorrem grupos previsíveis; setas movem seleção; Enter ativa; Escape fecha popup/dialog; Home/End navegam lista. Foco não desaparece quando painel é ocultado. Botões com ícones têm tooltip e nome; atalhos acessíveis no menu. Tempo não é anunciado a cada frame: mudança de faixa/evento e consulta do usuário apenas. Erros e fim de job usam live announcements com rate limit.

Modos Alto Contraste e Reduced Motion têm precedência sobre cores/animação do tema. Fonte escalável 100–200%, não vinculada ao DPI; no display clássico, 200% pode expandir altura sem clipping. Estados e resolução de áudio não dependem só de cor. Visualizador tem desligamento e não pisca de forma agressiva.

PT-BR e EN completos na primeira beta; fallback EN, strings por chave, pluralização e datas por locale. Texto do usuário preserva Unicode; busca usa normalização para comparação sem modificar tags originais. Testar caracteres combinados, emoji, CJK, RTL e nomes longos. RTL é alvo M5, dados RTL renderizam corretamente já em M2. Teclas/decimal do EQ respeitam locale; contratos usam ponto decimal e UTC.

QA: abrir/tocar/pause/seek/volume/playlist/preferences sem mouse; leitor anuncia linha tocando; foco visível em todos os temas; zoom 200% sem ações inacessíveis. WCAG é referência de desenho, não certificado automático de aplicativo desktop.

## Letras: acessibilidade e localização

Requisitos do componente de letras sob o VU (LYR-02, LYR-03, LYR-06, LYR-10, LYR-13, LYR-AC-09, LYR-AC-10); contrato completo em [Letras: componente resumido sob o VU e fontes](../library/02-tags-lyrics-tools.md#design-system-acessibilidade-e-desempenho). Estado: especificado; validação no protótipo é relato externo não verificado; nativo: nada.

- **Leitor de tela:** não anunciar cada mudança de verso automaticamente (coerente com "tempo não é anunciado a cada frame"); leitura da linha atual e da letra completa sob demanda. Ações e erros (importar LRC, formato não suportado) têm feedback separado, em região de status.
- **Menu de opções:** acessível por teclado, rótulos claros, foco visível, Escape fecha e o foco volta ao acionador; nenhuma ação depende de hover.
- **Contraste e cor:** tokens semânticos da skin; a linha atual se distingue também por peso/marcação, nunca só por cor; contraste das vizinhas validado após qualquer transparência; em Alto Contraste a hierarquia é por peso/marcação.
- **Movimento:** com Reduced Motion, atualização imediata sem transição ascendente; salto de posição não percorre versos intermediários.
- **Escala de texto:** 100–200% sem corte essencial nem overflow da página; os tamanhos em DIP do protótipo são referência, não limite. Texto longo tem alternativa completa.
- **Estados distintos:** sem letra, erro, formato não suportado, instrumental e desligada têm mensagens e rótulos próprios, localizados (PT-BR e EN por chave); origem (metadados, LRC local, DEMO) não depende só de cor.
- **Unicode:** letras são texto, nunca HTML; preservar Unicode (combinados, emoji, CJK, RTL) sem alterar a letra original nem as tags; RTL é alvo M5.
