# Layout Android, tablet e identidade compartilhada

Versão: 0.7.0 · Data: 2026-10-14 · Estado: especificação proposta, implementação não iniciada.

## Direção

Mesma paleta Classic Graphite, display lime, chassis grafite, accents cyan/amber e sensação de painel de áudio. Phone é layout próprio, não screenshot desktop encolhido. Classic/Library/Mini seguem desktop; phone/tablet são perfis adicionais. Sem window docking, titlebar com minimizar ou botões de 24 dp em telefone. Theme tokens compartilhados, geometria dependente do host.

## Phone

Referência útil 400×880 dp (inclui área ilustrativa de sistema); mínimo360dp, altura flex. Insets reais vêm do OS, não constantes. Área principal: app bar48, display64, capa até208, controles com altura56, seek48, tabs56 para Fila/EQ/Letras/Informações, painel flex e bottom navigation64. Margens16, gap8/12; targets48×48dp, fonte14–16sp; tempo28sp. Modo fonte200% reflow, não clipping. [Preview original](../../assets/android-layout.svg).

Now Playing: título/artista multiline limitada, tempo elapsed/remaining, formato source/output no inspector. Play/pause central56, prev/next48, volume acessível em sheet; volume físico segue rotaOS. Botão queue/favorite/overflow com nomes TalkBack; shuffle/repeat mostram estado textual. Seek anuncia posição e move via acessibilidade. Live sem DVR troca seek por LIVE/tempo de buffer, não slider falso.

Bottom navigation: Biblioteca, Playlists, Rádio, Podcasts. Mini-player sticky acima da navigation quando fora do Now Playing,48dp mínimo, não confundir com foreground notification. Expanded player abre por toque; back retorna navegação anterior e não para som. Gestos opcionais têm alternativa por botão; swipe em faixa nunca deleta arquivo sem confirmação.

EQ phone: 10 bandas em página com rolagem horizontal explícita, labels fixos e presets/preamp visíveis; gráfico paramétrico em editor fullscreen com tabela/bandas. Touch changes interpolados; nenhuma UI mexe diretamente no RT graph. Queue virtualizada com reordenação por handle e ações em sheet. Source permission denied explica abrir picker ou manter rádio.

## Tablet, foldable e landscape

Layout adaptativo por largura útil: compacto <600dp; médio600–839dp com navigation rail e player/fila; expandido≥840dp com biblioteca e inspector de playback/EQ em duas ou três áreas. Posture/hinge occlusion considerada; redimensionar não reinicia engine. Landscape compacto usa capa reduzida e duas colunas, sem esconder controles. Teclado externo/mouse acessíveis, Android tablet não vira desktop windows.

## Semântica e motion

TalkBack e Switch Access; focus order consistente; conteúdo do display como texto semântico, não apenas canvas. Descrever valor/unidade do slider e nome de cada banda. Não anunciar tempo cada frame. Reduced motion/high contrast/text scaling prevalecem sobre skin. Dynamic system colors só opção, paleta canônica é default. Dark/light variants preservam contraste; contraste validado para composições reais.

Jetpack Compose é candidato nativo; [layout adaptativo oficial](https://developer.android.com/develop/ui/compose/layouts/adaptive/get-started-with-adaptive-apps) orienta classes de largura. Theme schema define roles/constraints comuns e renderer Android mapeia para componentes acessíveis, sem executar código do tema.
