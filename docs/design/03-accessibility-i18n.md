# Acessibilidade e localização

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

DEVE oferecer árvore semântica para todos os controles, nomes estáveis, roles, valor e estado; sliders expõem valor/unidade e aceitam teclado. Tema customizado não remove estrutura semântica. Testar Narrator e NVDA no Windows; VoiceOver/Orca na expansão. Toolkit sem boa integração reprova M0.

Tab/Shift+Tab percorrem grupos previsíveis; setas movem seleção; Enter ativa; Escape fecha popup/dialog; Home/End navegam lista. Foco não desaparece quando painel é ocultado. Botões com ícones têm tooltip e nome; atalhos acessíveis no menu. Tempo não é anunciado a cada frame: mudança de faixa/evento e consulta do usuário apenas. Erros e fim de job usam live announcements com rate limit.

Modos Alto Contraste e Reduced Motion têm precedência sobre cores/animação do tema. Fonte escalável 100–200%, não vinculada ao DPI; no display clássico, 200% pode expandir altura sem clipping. Estados e resolução de áudio não dependem só de cor. Visualizador tem desligamento e não pisca de forma agressiva.

PT-BR e EN completos na primeira beta; fallback EN, strings por chave, pluralização e datas por locale. Texto do usuário preserva Unicode; busca usa normalização para comparação sem modificar tags originais. Testar caracteres combinados, emoji, CJK, RTL e nomes longos. RTL é alvo M5, dados RTL renderizam corretamente já em M2. Teclas/decimal do EQ respeitam locale; contratos usam ponto decimal e UTC.

QA: abrir/tocar/pause/seek/volume/playlist/preferences sem mouse; leitor anuncia linha tocando; foco visível em todos os temas; zoom 200% sem ações inacessíveis. WCAG é referência de desenho, não certificado automático de aplicativo desktop.
