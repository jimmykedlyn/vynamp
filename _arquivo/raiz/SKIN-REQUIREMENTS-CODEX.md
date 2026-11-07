# Vynamp — Skin Requirements for Visual Design (Codex)

Document version: 1.2 · Date: 2026-10-28 (lyrics note, version unchanged: summary lyrics component under the VU, **visual design only** — LYR-01..15 / LYR-AC-01..12, canonical docs/library/02-tags-lyrics-tools.md#letras-componente-resumido-sob-o-vu-e-fontes, D-LYR-01..09, TASK-E03-09; prototype references are reported evidence from the external reference, not verified in this repository; native: nothing; 1.2: DSP access/controls design (DSP-UX), VU meter visual integration (VU-INT), modular-flow canvas visuals, unified demo label — **visual design only**, see repo docs/design/05-dsp-panel.md §6.1, §8, §9; 1.1: equalizer five-mode correction, see repo docs/audio/08-equalizers.md) · Language: English (UI string examples shown in en and pt-BR where i18n matters) · Audience: Codex (visual design agent with no access to the maintainer)

> Reading conventions
> - Unmarked text comes from the product spec; the source is given in parentheses (file under `repo\docs\...` or `TASK-Exx-yy`).
> - **[proposal]** = the spec is silent; this document proposes. Codex may design the proposal but must list it in the deliverables as a proposal and must not contradict the spec.
> - **DSP demo label (single, exact string):** wherever a DSP / equalizer / Vitrola / meter screen shows processing that is simulated or absent, the badge reads exactly `[DEMO — no audio processing in this mode]` (pt-BR rendering of the same key: `[DEMO — sem processamento de áudio neste modo]`). Design it once as a reusable badge; no other wording variant in these contexts. (docs/design/05-dsp-panel.md §5)
> - **Status vocabulary:** screens may show only what the status allows — *specified* · *implemented in the prototype (reported)* · *validated in the prototype (reported)* · *native (today: nothing)*. The reference prototype is external (outside the Vynamp repository); anything it reports is *reported evidence from the external reference rev. 14/15, not verified in the Vynamp repository*. (docs/product/05-decisions.md D-DSP-16)
> - **MUST** = mandatory in the design. **MAY** = optional. **MUST NOT** = forbidden.
> - Units: DIP (desktop) and dp/sp (Android). 1 DIP = 1 logical dp. Base grid 4 DIP.
> - "Skin" and "theme" are the same concept: a `.vyntheme` package (docs/extensions/01-themes.md). There is NO compatibility with Winamp/AIMP/Poweramp skins (docs/product/01-vision-scope.md).
> - Source docs are written in Portuguese; terms are translated here. Keep UI strings keyed (en + pt-BR).

---

## 1. Goal, audience and how to use this document

### 1.1 Goal
Specify, unambiguously, **everything the Vynamp skin design must cover**: screens, areas, controls, states, behaviors, measurements, themes and variations. Codex will produce mockups/visual design (and SVG assets) from this document without being able to ask anyone. This document does NOT draw anything.

### 1.2 What Vynamp is
A local-first, cross-platform **audio** media player (Windows 11 x64, Linux x64, macOS Intel, macOS Apple Silicon, Android 10+ arm64). Works without an account for local files and public radio; default UI is the classic three-panel layout (transport, equalizer, playlist), with an expandable library, radio/podcasts, DSP (EQ, ReplayGain, effects), transparent/hi-res output, visualizers and own themes (docs/product/01-vision-scope.md). It does **not** play video (no video screen) and does not import legacy skins.

### 1.3 Audience (affects density and vocabulary)
Audio enthusiasts and DAC owners; local-library users (target 100k tracks); radio and podcast listeners; theme/visualizer creators (docs/product/01-vision-scope.md).

### 1.4 How to use
1. Read sections 2, 3 and 8 (principles, window formats, skin system) before designing.
2. For each screen/area (section 4) follow section 5 (controls, states, behaviors). Section 6 is a cross-check table; section 7 lists shortcuts.
3. Deliver what section 10 asks; run section 11 before finishing.
4. When in doubt: accessibility and spec semantics > fidelity to the "classic" look > novelty.
5. Items tagged with a phase (M1…M5) say when the feature ships. **Design all states even for M3/M4 features**, tagging the phase on the mockup. M5 features (native DSD, CD, VST3, casting, native 64-band EQ processing, MIDI/trackers) only get a reserved slot and an "unavailable in this version" state when they appear in capability lists. **Exception (1.1): the 64-band graphic EQ editor MUST be fully designed** (64 real band controls, scroll/zoom, states), because its interface is mandatory in the HTML/SVG/JS MVP; in the native product it shows "unavailable in this version (M5)" until TASK-E12-10.

### 1.5 Interface honesty principles (apply everywhere)
- Never show a badge the real audio chain does not justify (e.g. "bit-perfect" only when verified; "Transparent mode active" lists conditions). (docs/audio/04-outputs-hires.md)
- Every audible transformation is identifiable (EQ, ReplayGain, resample, dither, effects). (docs/product/01-vision-scope.md)
- State and quality never rely on color alone: always an icon or text label too. (docs/design/01, 03)
- No stack traces or crate names in the normal flow; technical detail goes in an optional "Details" panel. (docs/design/02-layouts.md)

---

## 2. Visual and brand principles

### 2.1 Visual direction: "Classic Graphite" (default theme)
Graphite/dark-blue cabinet, cool metallic details, black display with lime green, amber and cyan meters. Three stacked modules, short title bars, **square** transport buttons, compact tracks, dense playlist. Faithful to the classic feel and hierarchy; **all illustration, icons, geometry and tokens are original** — do not copy existing skins, sprites or fonts. (docs/design/01-visual-system.md)

### 2.2 Mandatory semantic tokens (Classic Graphite)
Components use tokens, never arbitrary RGB. (docs/design/01-visual-system.md; canonical JSON `assets/design-tokens.json`)

| Token | Hex | Use |
|---|---|---|
| background | #11131B | Outer area |
| chassis | #252936 | Panels |
| panel-raised | #333A4D | Buttons and headers |
| panel-inset | #191D2A | Recesses |
| display | #080D0B | Time, spectrum, classic list |
| border-light | #677086 | Top/left lit edge |
| border-dark | #090C14 | Bottom/right edge |
| text-primary | #F1F3F7 | Control text |
| text-secondary | #B8C0CF | Metadata and labels |
| text-muted | #8D97AB | Disabled / secondary info |
| lime | #B8F35A | Time and playing item (playback) |
| cyan | #66D9EF | Focus and tool selection |
| amber | #FFBF69 | RMS/VU and edit actions |
| danger | #FF6B7A | Error/clip |
| selection | #32453A | Selected row |

Rules: lime = playback/time; cyan = focus/tools; amber = VU/edit; danger = error/clip. The brand (amber #FFBF69 / white #F0F2F5 / graphite #11131B) is **separate** from semantic tokens: brand white does not replace text-primary. (docs/design/04-brand-identity.md)

### 2.3 Bundled themes (each needs Classic, Library and Mini layouts)
(TASK-E06-04; docs/design/01; docs/extensions/01-themes.md)
1. **Classic Graphite** (default): 1 DIP bevel (border-light/dark), radius 2.
2. **Midnight Modern**: same color family, **less bevel**, radius 8, larger display.
3. **Copper**: amber as accent.
4. **[proposal] Light variant** of each theme (see 2.4).
All keep token names, states and accessible layout identical. None copies an existing skin.

### 2.4 Light and dark themes
- The spec guarantees that "dark/light variants preserve contrast" on mobile (docs/design/04-mobile-tablet.md) but only defines the dark palette on desktop. **[proposal]** Deliver for each bundled theme a **Dark (default)** and a **Light** pair with identical token names; in light, the time/spectrum `display` MAY stay dark (instrument window) to keep lime legible, provided contrast is measured.
- Following the OS theme is an option in Preferences > Appearance (default: Classic Graphite dark regardless of OS) **[proposal]**.
- "Dynamic system colors" (Material You) on Android is an **option**; the canonical palette is the default. (docs/design/04-mobile-tablet.md)
- OS High Contrast and Reduced Motion **take precedence** over skin colors and animation; design a High Contrast variant (black/white/yellow or system colors, 2 DIP borders, no reliance on bevel). (docs/design/03-accessibility-i18n.md)

### 2.5 Typography
- UI: system sans or Noto Sans (license bundled). Metadata 13–14 DIP; actions 14; section 16; title 18. (docs/design/01)
- Display: own monospace or reviewed free font; **30 DIP for time**, 13 DIP for codec. "Digital/LED" letterforms are a visual option only, **never the sole accessible representation** (equivalent semantic text). (docs/design/01)
- Android: 14–16 sp; time 28 sp; 200% font scale must reflow without clipping. (docs/design/04-mobile-tablet.md)
- Desktop font scale 100–200% independent of DPI; on the classic display 200% MAY grow the height without clipping. (docs/design/03)
- Brand wordmark/lettering as paths (no installed-font dependency). (docs/design/04-brand-identity.md)

### 2.6 Grid, spacing, radius, borders
4 DIP grid; padding 8/12/16; gap between modules 8; radius classic 2 / modern 8; outline 1; inner bevel 1. (docs/design/01)

### 2.7 Density
- **Compact**: list row 24 DIP, buttons 28 DIP.
- **Comfortable**: row 36 DIP, buttons 40 DIP.
- **Touch mode**: minimum target 44×44 DIP (desktop touch) and 48×48 dp (Android). Minimum desktop mouse target: 24×24 DIP. **Never shrink below minimums to imitate legacy pixels.** (docs/design/01, 04-mobile)
- Mini-player: 8 DIP visual seek with **24 DIP interaction area**. (docs/design/02)

### 2.8 Iconography
- **SVG path** icons, original (no icon fonts, no external refs, sanitizable). Grids: 16, 20, 24 (desktop), 24 dp (Android). Uniform stroke (suggested 1.5–2 DIP); square terminals in Classic, rounded in Midnight Modern.
- The **brand icon (V + play triangle) MUST NOT be used as the play button** and does not replace controls. (docs/design/04-brand-identity.md; TASK-E02-02)
- Every icon-only control has a **tooltip and accessible name**. State is never color only.
- Brand: horizontal, stacked, symbol-only and monochrome lockups; clear space 20% of symbol height; symbol with text from 24 DIP; below that use the simplified optical version (16/20 px: keep triangle and gap). Tray/menu bar: monochrome/alpha glyph, no wordmark; macOS menu bar = alpha template (18 pt / 36 px). (docs/design/04-brand-identity.md)

### 2.9 Component states (default for EVERY control)
(docs/design/01-visual-system.md)

| State | Mandatory treatment |
|---|---|
| Normal | "Raised" (bevel) / subtle elevation in Modern |
| Hover | Cyan border and +8% luminosity |
| Pressed | "Inset" look |
| Checked/active (toggle) | Lime dot + accessible label/state (not color only) |
| Disabled | text-muted, **no** hover effect, not opacity-only (add struck icon or label) |
| Focus-visible | Cyan ring 2 DIP with 2 DIP gap, visible in ALL themes |
| Error | Icon + label + danger (not color only) |
| Playing (current track) | Triangular icon + lime; **selection ≠ current track** |
| Loading | Action label + bounded spinner (cancellable for long operations) |

Draw every control in all these states in the component sheet (section 10).

### 2.10 Contrast (WCAG as design reference)
- Normal text ≥ 4.5:1; large text and essential indicators ≥ 3:1. Measure **real pairs** (including selected/disabled/hover). A preview is not proof. (docs/design/01, 03)
- Brand amber on white is not for essential controls (use monochrome graphite). (docs/design/04-brand-identity.md)
- Deliver a measured-contrast table per theme and state (section 10).

### 2.11 Motion
- Hover 80 ms; expand 160 ms; modals 120 ms; **no blocking animation**. (docs/design/01)
- Interactive UI 60 Hz; visualization 30 Hz default, 60 optional (foreground), 10 in power-saving; minimized window suspends drawing (audio continues). (docs/design/01; extensions/04-visualizer-contract.md)
- **Reduced Motion**: no transitions, marquee or animated visualizer; meters static or slow-updating (see 5.13). No flash > 3/s; no automatic high-contrast animation. (docs/design/01, 03)
- Long titles: ellipsis + tooltip; marquee optional **only on hover**, off in Reduced Motion.

### 2.12 Rendering and DPI
References at 100/125/150/200% DPI, both themes, all states. Layout in DIP; the renderer handles DPI (do not multiply pixels manually). Text never clips an essential control in PT-BR/EN. (docs/design/01–02)

---

## 3. Platforms and window formats

### 3.1 Mode matrix per platform
(docs/design/02-layouts.md, 04-mobile-tablet.md; docs/platforms/*)

| Mode | Windows | Linux | macOS (Intel/Silicon) | Android |
|---|---|---|---|---|
| Classic Stack (default) | yes | yes | yes | no (phone/tablet profile) |
| Library Workspace | yes | yes | yes | adapted (expanded tablet) |
| Mini | yes | yes | yes | sticky mini-player + notification |
| Full-screen visualizer | yes | yes | yes | yes (landscape/immersive) [proposal] |
| Phone / Tablet | — | — | — | yes |

The same skin delivers common tokens; **geometry depends on the host** (never shrink desktop to phone).

### 3.2 Desktop — main window (Classic Stack)
(docs/design/02-layouts.md)
- Initial usable window **600×760**, minimum **480×660**. Outer padding 12. Panel width W = usable width − 24 (W=576 at initial size).
- Regions (origin top-left of usable area), x/y/w/h:
  - Main header 12/12/576/28
  - Player 12/40/576/184
  - EQ 12/232/576/180 (hidden → returns space to playlist)
  - Playlist 12/420/576/328
  - Gaps 8 between modules.
- Player inner regions (local origin), x/y/w/h:
  - Display (time, title, spectrum/meter, quality) 12/12/352/78
  - Optional cover 376/12/72/72
  - Source actions (Local / Radio / Library) 460/12/104/32
  - Seek 12/102/552/20
  - Transport (Previous, Play, Pause, Stop, Next, Open) 12/140/264/32
  - Volume/pan 288/140/164/32 (volume + mute; pan in popup)
  - Modes (Shuffle/Repeat) 464/140/100/32
- Player header: menu left, name ("Vynamp") center, minimize/close right.
- Display: time takes 100 DIP width; title takes the rest; codec and resolution on a second line; **bottom spectrum/VU strip 28 DIP, never covering text**.
- EQ: titlebar 24, toolbar 28, preamp + ten bands = 100, footer 16.
- Playlist: titlebar 24, tabs 28, flex list, footer 32 (add/remove/sort + count/duration).
- **Width < 560 DIP**: cover removed; source buttons become a menu; volume moves to a second row; player grows 32 DIP and playlist loses 32. Even at minimum, keyboard and adequate targets.
- **Lyrics under the VU (LYR-01, LYR-15; D-LYR-09)** [specified; exact slot is a design proposal]: in the approved extended layout the summary lyrics component sits **immediately below the VU**, integrated into the skin surface with no extra frame or box. It adapts to the space **without shrinking or distorting the 12 stereo VUs or covering playback controls**, and is not imposed where space is lacking. Design it as its own slot; it never takes the 28 DIP display strip. Canonical: docs/library/02-tags-lyrics-tools.md#letras-componente-resumido-sob-o-vu-e-fontes; docs/design/02-layouts.md#letras-sob-o-vu-2026-10-28.
- Advanced graphic EQ opens a larger editor (do not squeeze 31 bands into the classic panel).
- Snap docking at 12 DIP; Alt ignores snap; detachable modules only in M3 (TASK-E02-08). Positions persisted **per monitor** and clamped to the visible area on restore (a removed monitor must not strand the window).
- Resizing: free width/height between minimum and monitor maximum; panels stretch in width; the playlist absorbs extra height. **[proposal]** Max usable Player width before centering: 960 DIP (above that, side margins; do not stretch transport).

### 3.3 Desktop — Library Workspace
(docs/design/02-layouts.md)
- Initial **1280×800**, minimum **960×640**.
- Topbar 56; sidebar 200 (collapsible to 56); central panel flex ≥ 480; optional right inspector 280; bottom now-playing 96.
- Sidebar: Library, Folders, Playlists, Radio, Podcasts, Favorites, Downloads, Themes/Plugins.
- Width < 1152: inspector becomes a drawer; sidebar may collapse to 56.
- Opening the library as a workspace **preserves playback and previous geometry**.
- **Lyrics (LYR-01, LYR-15)**: wherever the large VU is shown (now-playing bar, inspector "Meter" card) the summary component may sit below it under the §3.2 rule; the Lyrics inspector/panel (§5.16) stays the full view. Without room, the summary component is omitted (D-LYR-09).

### 3.4 Desktop — Mini
(docs/design/02-layouts.md)
- Minimum **420×112**. Contents: title/time, play and next, volume/mute, expand; 8 DIP seek with 24 interaction area.
- Always-on-top **optional** with a **clear indicator** (active pin icon + tooltip).
- "Close to tray" **off by default**; when on, the first close shows a notice saying where to reopen.
- Switching mode keeps audio (never restarts).
- **[proposal]** Mini may be resized horizontally only (420 → 640 DIP); at wider sizes it shows a mini meter and a 56×56 cover.
- **Lyrics**: the three-line summary component is **not imposed in Mini** when space is lacking (D-LYR-09); any Mini lyrics presentation is a separate [proposal] that must keep minimum targets.

### 3.5 Title bar, tray and menu bar
- **Windows/Linux**: skin-drawn title bar (28 DIP header with menu, name, minimize, close [and maximize in resizable Library mode]). Dragging the header moves the window; double-click on the Library header maximizes/restores. When the window uses OS decorations, the skin still draws the 28 DIP inner header **[proposal]** for consistency. Respect window controls under Wayland. (docs/platforms/03)
- **macOS**: native traffic lights at left, app menu bar at the top of the screen; inner header omits duplicate window buttons **[proposal]**. Menu bar icon = alpha template, 18 pt.
- **Tray (Windows/Linux)**: Vynamp glyph (monochrome 16/20/24/32 px), context menu: Open Vynamp, Play/Pause, Previous, Next, Mini-player, Quit; paused/error via badge, not another mark. Click restores window. Tooltip "Artist — Title" **[proposal]**. (docs/design/04-brand-identity.md; docs/extensions/03)
- **Windows taskbar**: playback progress (taskbar progress) and thumbnail buttons (Previous/Play-Pause/Next) **[proposal]**; SMTC with cover, title, artist. (docs/extensions/03)
- **OS media controls**: Windows SMTC/media keys; Linux MPRIS; macOS Now Playing/media keys; Android MediaSession. The skin only supplies cover art and labels; the controls belong to the OS. (docs/extensions/03; docs/platforms/03)

### 3.6 Android
(docs/design/04-mobile-tablet.md; docs/platforms/02-android.md)
- Phone reference **400×880 dp**; minimum 360 dp; flexible height; real system insets (never constants).
- Now Playing vertical structure (phone): app bar 48; display 64; cover up to 208; controls height 56; seek 48; tabs 56 (**Queue / EQ / Lyrics / Info**); flex panel; bottom navigation 64. Margins 16; gap 8/12; targets 48×48 dp.
- Play/pause center 56; previous/next 48; volume in an accessible **sheet** (physical volume follows the OS route; label "Device volume" vs "Vynamp digital volume").
- Bottom navigation: **Library, Playlists, Radio, Podcasts**.
- **Sticky** mini-player above bottom navigation outside Now Playing, ≥ 48 dp; not to be confused with the notification.
- Expanded player opens by tap; **Back returns to previous navigation and does not stop sound**.
- **Portrait**: layout above. **Compact landscape**: reduced cover + two columns (controls/cover left; queue/EQ/lyrics right), without hiding controls.
- **Lyrics on Android**: the **Lyrics** tab remains the full lyrics access. The summary component is specified only for the approved extended layout; its Android placement is **not specified** (open; no value invented). The reported 30 dp narrow-screen line height (external reference rev. 14, not verified in this repository) is only a starting point subject to tokens, text scale and 48 dp targets.
- **Tablet/foldable (breakpoints by usable width)**: compact < 600 dp; medium 600–839 dp (navigation rail + player/queue); expanded ≥ 840 dp (library + playback/EQ inspector in 2–3 panes). Consider hinge/posture; resizing does not restart the engine.
- External keyboard/mouse accessible; tablet does **not** become desktop windows.
- **Media notification** (foreground service mediaPlayback): cover, title, artist, actions Previous / Play-Pause / Next (+ close/stop **[proposal]**), progress bar where the OS allows; notification icon: own 24 dp white/alpha symbol (no colored tile). Mirrors the Rust engine state. (docs/platforms/02; brand-identity)
- **Lock screen**: standard OS media controls with cover; the skin does not draw the lock screen, it only supplies metadata and cover.
- **Widget [proposal]**: resizable 4×1 (cover 48 dp, title/artist, previous/play-pause/next) and 4×2 (+ progress bar). Not in the spec; tag as proposal, M4+.
- Adaptive icon: 108×108 dp, V/play inside the central 66 dp circle; monochrome layer (themed icons). (docs/design/04-brand-identity.md)
- Gestures: always with a button alternative; **swiping an item never deletes a file without confirmation**.

### 3.7 Consolidated breakpoints

| Context | Breakpoint | Change |
|---|---|---|
| Desktop Classic | width < 560 DIP | cover hidden, sources → menu, volume on 2nd row |
| Desktop Classic | height < 660 | not allowed (minimum) |
| Library | < 1152 DIP | inspector → drawer; sidebar collapsible to 56 |
| Library | < 960 | not allowed (minimum) |
| Mini | 420×112 minimum | see 3.4 |
| Android | < 600 dp / 600–839 / ≥ 840 | compact / medium with rail / expanded multi-pane |
| Font | 100–200% | reflow/height growth, never clipping |
| Lyrics summary component | narrow screen / enlarged text | one visual line per position, no horizontal page scroll, full text on demand; omitted rather than shrinking the VUs (LYR-03, D-LYR-09) |

---

## 4. Screen and area inventory

Master list (each has a section 5.x). Phase in brackets.

| # | Screen/Area | Where it appears | Phase |
|---|---|---|---|
| 5.1 | Player: display, transport, source actions | Classic, Mini, Now Playing | M1 |
| 5.2 | Track info and cover | all | M1/M2 |
| 5.3 | Seek / progress bar (incl. A-B, LIVE) | all | M1 |
| 5.4 | Volume, mute, pan, balance | all | M1 |
| 5.5 | Queue and playlist (tabs, drag-and-drop, undo) | Classic, Library, Android | M1–M3 |
| 5.6 | Library (views, search, filters, sort, inspector) | Library, Android | M2 |
| 5.7 | CUE and virtual tracks, chapters | playlist/library/inspector | M2 |
| 5.8 | Track technical details and audio chain | inspector, dialog, display | M2/M3 |
| 5.9 | Equalizer: graphic + advanced/parametric editor + presets | EQ panel, advanced window, Android | M1–M3 |
| 5.10 | DSP: ReplayGain, crossfade, gapless, dynamics, tone, tempo/pitch, channels | Preferences > DSP | M1–M3 |
| 5.11 | Vitrola effects (vinyl + tube) | DSP/Effects panel | M3 (after E04-03) |
| 5.12 | Audio output / devices / transparent mode / hi-res | display popup + Preferences > Output | M3 |
| 5.13 | **Visualizers and switchable VU meter** | display, Mini, full screen, Android | M2/M3 |
| 5.14 | Radio and streams | Library/Classic/Android | M2/M3 |
| 5.15 | Podcasts and audiobooks | Library/Android | M2–M4 |
| 5.16 | Lyrics: full panel (plain/synced) **plus summary component under the VU** (LYR-01..15; library/02#letras-componente-resumido-sob-o-vu-e-fontes) | Lyrics tab, inspector, and below the VU in the extended layout | M3 |
| 5.17 | Recording, sleep timer, scheduler/alarm | dialogs | M3 |
| 5.18 | Preferences | 880×680 window | M2 |
| 5.19 | Themes (gallery/preview/editor) and Plugins (manager) | Library sidebar | M2/M3 |
| 5.20 | Job manager (scan, convert, download, record) | 800×520 window | M2/M3 |
| 5.21 | Track details / batch tag editor | 800×640 window | M3 |
| 5.22 | Notifications, errors, toasts, dialogs | cross-cutting | M1 |
| 5.23 | Onboarding, first run, empty states | cross-cutting | M1 |
| 5.24 | About, splash, safe mode | dialog | M1 |
| 5.25 | Menus (main, context) | cross-cutting | M1 |
| 5.26 | Android: navigation, sheets, gestures, notification | Android | M1+ |

Mandatory screen states for EVERY data screen (docs/design/02-layouts.md; TASK-E02-10): **Empty (with CTA)**, **Loading (cancellable)**, **Populated**, **Partial error (valid items kept)**, **Offline (local content not erased)**, **Unsupported (with reason)**, **Permission denied**, **Conflict**, **Recovery**. Dialogs: focus the first safe action, Escape cancels, focus returns to the invoker.

---

## 5. Per-screen specification

Control format: **Name** · suggested icon · action · shortcut · tooltip · specific states (beyond the 2.9 defaults). Shortcuts marked "[proposal]" are not in the spec; spec ones cite (docs/product/04-user-journeys.md).

### 5.1 Player: display, transport and source actions

**Display content (352×78 in Classic)**: elapsed time (30 DIP, lime, display font) with elapsed/remaining toggle (click the time); title; artist/album (2nd line, text-secondary); technical line (codec, rate, bits, channels, bitrate; e.g. "FLAC · 24 bit · 96 kHz · stereo"); badges (HI-RES SOURCE, TRANSPARENT, LOSSY, LIVE, gapless, RG); engine state (Opening… / Buffering… / Reconnecting… / Device lost / Error); **meter/visualizer on the 28 DIP bottom strip** (section 5.13). (docs/design/02; docs/audio/04)

**Player states (state machine)**: Idle → Opening → Buffering → Playing ↔ Paused; Seeking; Stopping; Reconnecting; DeviceLost; Error. Each needs a visual state in the display (text label + icon). Buffering does **not** show fake progress. (docs/audio/01-engine.md; docs/product/04)

**Transport (264×32; square buttons)**

| Control | Icon | Action | Shortcut | Tooltip | Extra states |
|---|---|---|---|---|---|
| Previous | |◀◀ | Previous track (if > 3 s played, restarts the track **[proposal]**; second press goes back) | Alt+Left (spec); media Prev | "Previous" / "Anterior" | disabled with empty queue |
| Play | ▶ | Play / resume | Space (when focus is not a text field) (spec) | "Play" / "Reproduzir" | loading during Opening |
| Pause | ❚❚ | Pause, keep position | Space | "Pause" / "Pausar" | active when paused; live radio: resumes at live edge with notice |
| Stop | ■ | Stop and go Idle | S **[proposal]** | "Stop" / "Parar" | disabled in Idle |
| Next | ▶▶| | Next in queue (beats repeat) | Alt+Right (spec); media Next | "Next" / "Próxima" | disabled at queue end without repeat |
| Open | ⏏/folder | Open file(s) | Ctrl+O (spec) | "Open file…" / "Abrir arquivo…" | opens native dialog/picker |

Note: Classic Graphite shows Play and Pause as **separate buttons** (spec); in other layouts (Mini/Android) a single toggling play/pause button is allowed, with its accessible name switching. Either way the current state is announced to screen readers.

**Source actions (104×32)**: Local (open file/folder), Radio (open URL; Ctrl+L), Library (opens Library Workspace). Below 560 DIP they collapse into a "Source" menu.

**Modes (100×32)**
| Control | Icon | Action | Shortcut | States |
|---|---|---|---|---|
| Shuffle | crossed arrows | toggle shuffle without replacement per cycle | H **[proposal]** | on (lime dot + text "Shuffle: on") / off |
| Repeat | circular arrows | cycles: off → sequence → track | R **[proposal]** | 3 states with distinct icon (plain / loop / loop with "1"); text label |

**Extra accessible player controls (menu "Playback" or popover)**: Stop after this track; Stop after queue; A-B loop (set A, set B, clear); Speed (podcast); Skip +30 s / −15 s for podcast/audiobook (configurable); Sleep timer. (docs/product/04; docs/library/02; TASK-E01-05)

**Behaviors**: dropping files on the player **replaces the source after explicit intent** (drop zone labeled "Drop to play now" + confirmation if a queue exists); double-click on the display toggles Library/Classic **[proposal]**; right-click opens the player context menu (copy title, show in folder, details, open audio chain).

### 5.2 Track info and cover
- Optional cover 72×72 (Classic), 56 (wide Mini), up to 208 dp (Android), large in the inspector and full screen. Priority: embedded > folder > online cache (opt-in) > original Vynamp fallback. PNG/JPEG/WebP. (docs/library/02)
- **Cover placeholder** (no art): neutral note glyph — do NOT use the logo as a cover.
- Fields shown: title, artists, album, year, track/disc no., genre (inspector), rating, favorite, play count, file name/URL (no token), current chapter (CUE/M4B).
- Behavior: long title → ellipsis + full tooltip; optional marquee on hover only (off in Reduced Motion); click cover opens enlarged cover (lightbox) **[proposal]**; click on artist/album filters the library.
- States: no metadata (show file name), loading cover (skeleton), cover error (placeholder, no intrusive warning), live/radio (station name + ICY title; **no fake duration**, show "LIVE").
- Display text MUST exist as **semantic text** (not canvas only).

### 5.3 Seek / progress bar
- Horizontal slider 552×20 (Classic), 8 DIP visual + 24 interaction (Mini), 48 dp (Android). Shows: elapsed and duration/remaining; **buffered range** (lighter section) for network sources; chapter/CUE track ticks; A and B markers.
- Hover: tooltip with timestamp under the pointer. Dragging the thumb previews time; release → seek. Click: jump. Keyboard: ←/→ ±5 s, Shift+←/→ ±30 s, PageUp/PageDown ±1 min, Home/End start/end **[proposal for values]**; on focus announces "position X of Y".
- States: normal; hover; dragging; **disabled** when the source is live without DVR (replace with **LIVE** badge + buffer time, **no** fake slider); live with timeshift: slider only within the window (30 min/512 MiB) with a "Go live" button; seek unavailable on remote file without Range (disabled + reason tooltip); Buffering (activity indicator without fake %).
- **A-B loop**: buttons A, B, Clear; highlighted region (translucent cyan/amber); A<B; exits on track change. (TASK-E01-05)
- Android: seek announces position via accessibility and moves through accessibility actions (increments).

### 5.4 Volume, mute, pan
- Volume: horizontal slider (164 DIP including mute) with numeric value (0–100%) in tooltip/hover; keyboard step ↑/↓ 2% or 5% **[proposal]**; Mute (speaker-with-waves / struck-through icon) toggle, shortcut M **[proposal]**; mouse wheel over slider adjusts volume. Pan/Balance in a popup: centered slider with center mark, readout "L 30 / C / R 30", double-click to reset.
- States: normal/hover/dragging/muted (struck icon **and** text "Muted")/disabled (transparent mode with hardware volume — show volume type: **digital** vs **hardware**; distinct labels; unity indicated).
- Visual warning when digital gain exceeds 0 dB (possible clip) **[proposal]**.
- Android: physical volume controls the OS; in-app sheet with "Device volume" and "Vynamp volume".
- Output change (DAC) → never raise volume silently (no misleading visual warning). (docs/audio/03)

### 5.5 Queue and playlist
(docs/library/01-library-playlists.md; docs/product/04-user-journeys.md; TASK-E02-04)

**Structure (Classic)**: titlebar 24 · tabs 28 (multiple playlists + **Queue** tab) · virtualized list · footer 32.
**List columns (configurable)**: playing indicator (▶ lime), no., title, artist, album, duration, format/bitrate, rating, year, genre, path; compact (24) / comfortable (36) density. **Current** track distinguished from **selected** (selection #32453A for selection; lime + icon for current). Unavailable items (missing file, offline root, revoked grant) show an alert icon + text "Unavailable" (not removed).
**Footer**: Add (file/folder/URL), Remove, Sort (menu), Misc/Options, item count and total duration.

| Control | Icon | Action | Shortcut |
|---|---|---|---|
| New playlist (+ tab) | + | creates tab | Ctrl+T **[proposal]** |
| Play now | ▶ | starts new sequence | Enter / double-click (spec) |
| Play next | ⤵+ | inserts after current | Alt+Enter **[proposal]** |
| Add to queue | + queue | appends | Shift+Enter **[proposal]** |
| Remove from playlist | trash | removes item(s) | Delete (spec; never deletes physical media) |
| Move item | handle ⋮⋮ | reorders (no audio restart) | Alt+↑/↓ **[proposal]** |
| Sort/Group | ↕ | menu: title, artist, album, duration, year, path, random | — |
| Undo/Redo | ↶ ↷ | ≥ 100 operations per session | Ctrl+Z / Ctrl+Shift+Z |
| Lock playlist | padlock | blocks editing (not playback) | — |
| Remove duplicates | — | explicit command, never changes queue unasked | — |
| Save/Export | floppy | M3U/M3U8/PLS/XSPF/CUE; UTF-8; no token | Ctrl+S **[proposal]** |
| Import | arrow in | M3U/M3U8/PLS/XSPF/CUE | — |
| Select all | — | — | Ctrl+A |
| Search in list | magnifier | filters | Ctrl+F (spec) |

**Keyboard**: ↑/↓ navigate; Home/End; Enter plays; Delete removes; multi-select with Shift/Ctrl; screen reader announces the playing row.
**Row context menu**: Play now · Play next · Add to queue · Remove from playlist · Remove from library (≠) · Delete file… (confirmation + recycle bin) · Track details · Edit tags… · Show in folder · Copy path · Favorite · Rate (1–5) · Stop after this · Identify online (opt-in) **[M3]**.
**Drag-and-drop**: dragging files/folders onto the playlist adds **at the end**; dropping on the **marker between rows** inserts there (cyan insertion line); reorder by dragging the handle. Folders expand in the background. Invalid files do not block; errors are grouped in one notice.
**Queue**: separate tab (snapshot independent from the edited playlist) with an ordered "Up next"; actions Clear queue, Save queue as playlist; shuffle seed only in advanced. Crash restore: banner "Resume where you left off" (no autoplay).
**Smart playlists (M3)**: rule editor (field/operator/value, sort, limit); distinct icon; no free-form SQL.
**States**: empty (CTA "Add files / folder / open radio"); loading (import with cancellable progress); partial error; missing file; locked.
**Android**: virtualized list with handle reordering; actions in a **bottom sheet**; optional side swipe (add to queue / remove from list **with Undo**, never delete file); long-press selects.

### 5.6 Library
(docs/library/01; TASK-E02-05, TASK-E03-05)

**Library Workspace**: topbar (56): back/forward, **global search** (title, artist, album, folder; Ctrl+F; cancellable, does not alter queue), breadcrumb, filters button, view toggle (grid / list / folders), sort, Settings button. Sidebar (200): Library (Songs, Albums, Artists/Album Artists, Genres, Years, Folders, Recent, Favorites, Hi-res, Streams), Playlists, Radio, Podcasts, Downloads, Themes/Plugins, + "Sources" area with status (online/offline/NAS unavailable). Center: album grid (cover, title, artist, year), track list (configurable columns, safe-template grouping, stable sort, virtualized — 100k rows without a widget per row), or folder tree. Inspector (280): cover, credits, tags, lyrics, audio chain. Bottom now-playing (96): cover, title/artist, full transport, seek, volume, queue button, visualizer button, EQ button.

**Filters**: codec, resolution (incl. Hi-res), genre, year, rating, unplayed, origin (local/radio/podcast/NAS). Removable chips. Save filter as smart playlist (M3).
**Sort**: title, artist, album, year, duration, recently added, most played, rating, random; asc/desc. Sorting while playing does not alter the queue.
**Item actions**: Play, Play next, Add to queue, Add to playlist ▸, Favorite, Rate, Details, Edit tags, Show in folder, Remove from library (≠ Delete file, with confirmation and recycle bin).
**Import**: "Add folder…" with a **count preview** and cancellable progress; incremental scan with a discreet sidebar indicator; offline roots become **unavailable (dimmed + icon), never removed**.
**States**: empty library (onboarding), indexing (progress + usable partial), search with no result (message + suggestion to clear filters), source error, permission denied, offline.
**Android**: Library/Playlists/Radio/Podcasts in bottom navigation; denied media permission → explanatory screen offering the file picker (SAF) or continuing with radio; sticky mini-player.

### 5.7 CUE and virtual tracks
(docs/library/01; docs/audio/02-formats.md; TASK-E03-06)
- CUE virtual tracks (and M4B/Matroska chapters) appear as **child** items of a parent file (disc/CUE icon and indent), referencing a range of the same file — no duplicated media. Show index, title, performer, start (mm:ss:ff, 75 frames/s) and duration.
- Seek bar shows track ticks; display shows "Track 3/12 · CUE" and current chapter.
- Actions: Extract CUE tracks (M3 tool), Show source file, Open .cue.
- States: invalid/missing CUE (alert "CUE media file not found"), explicit gaps/offsets.

### 5.8 Track technical details and audio chain
(docs/audio/04-outputs-hires.md; docs/audio/01; TASK-E02-07)

**Technical line on display**: codec, valid bits, rate, channels, bitrate (approx. for streams).
**Badges** (text + icon, never color only):
- **HI-RES SOURCE**: lossless PCM source >16 bits and/or >48 kHz (definition in tooltip; no implied audible improvement).
- **LOSSY** (lossy source; lossy at 192k does not become lossless).
- **TRANSPARENT** ("Transparent mode active"; tooltip lists conditions; **"bit-perfect verified" only for tested combinations**).
- **EXCLUSIVE / SHARED / ASIO**.
- **GAPLESS**, **RG** (ReplayGain track/album applied), **EQ**, **FX**, **RESAMPLE**, **DITHER**.
- **LIVE**, **OFFLINE** (downloaded), **Bluetooth** (show known output; never assume codec/bit-perfect).
**"Audio chain" inspector** (linear diagram with nodes): Source → Decode → Gain/ReplayGain/Preamp → EQ/Tone/Effects → Mixer/Crossfade → Master gain → Resampler → True-peak guard → Dither/Conversion → Output. Each node: name, on/bypass (icon + text), key parameters, declared latency. **SOURCE** block (rate/bits/channels/codec) vs **OUTPUT** block (**negotiated** real rate/valid bits/channels, backend, device). FLAC 24/192 on a 16/48 output shows "resample + quantization". A 32-bit container with 24 valid bits is not a 32-bit source. State "output transport" when driver/hardware transformation is not observable.
**Tap/limiter latency readout** (visible increase); **underrun** counter; **clipping** indicator detected per block.
**Behavior**: turning EQ on in transparent mode → dialog "Leave transparent mode?" (confirms intent and updates the badge); opening a lossy file in transparent → "processing disabled; lossy source".
**States**: no track (empty), partial data (show "—"), unsupported profile/decoder (reason), loading.

### 5.9 Equalizer
(docs/audio/03-dsp.md; TASK-E04-01/04/09; TASK-E02-07)

**Classic panel (576×180)**: titlebar 24; toolbar 28; preamp + 10 bands area 100; footer 16.
- Toolbar controls: **On/Off** (toggle), **Auto** (applies preset per track/genre/device), **Reset** (flat), **Presets ▾** (menu: load, save as…, rename, delete, import, export), mini response curve.
- **Preamp**: vertical slider −24..+12 dB. **10 bands**: 31/62/125/250/500/1k/2k/4k/8k/16k Hz, gain ±12 dB (fixed frequency label below, dB value on drag/focus). Mini curve computed from the real filters (not decorative).
- Keyboard step: ↑/↓ 0.5 dB, Shift 2 dB, Home/End min/max, double-click = 0 dB. Value/unit announced ("62 hertz, plus 3.0 decibels"). Decimal follows locale.
- States: on/off (bands dimmed but editable **[proposal]**), modified preset (asterisk "*"), **locked by transparent mode** (notice and "Leave transparent mode" button), loading preset, preset error.
**Advanced editor (960×600)**: large graph (log frequency 10 Hz–40 kHz limited to 0.45×Fs; gain dB), real curve, draggable points; **band table** (type, frequency, gain, Q, channel, slope, enabled); **mode selector Graphic 10 / Graphic 20 / Graphic 31 / Graphic 64 / Parametric (up to 32 filters)**; graphic modes gain ±15 dB, with exactly 10/20/31/64 editable band controls (64: horizontal scroll, zoom and minimap, targets not shrunk; native 64-band processing is M5 — show "unavailable in this version (M5)" in the native build); parametric up to 32 filters (M3; 64 filters in M5 — show "unavailable"). Center frequencies: repo docs/audio/08-equalizers.md (31-band = third-octave; 64-band is log-spaced, never labelled "1/3 octave"). Types: peaking, low/high shelf, low/high pass, notch, band pass. Freq 10 Hz–min(40 kHz; 0.45×Fs); gain −24..+24; Q 0.1..18; channel Both/Left/Right (multichannel M5). Frequencies above Nyquist **unavailable** (disabled with tooltip), no hidden readjustment. Preset **per device** (tab). Import AutoEQ/headphone correction: explicit, origin/license identified, never assume the headphone model. Precedence shown: session > track > genre > device > global.
**Android**: 10 bands in a page with **explicit horizontal scroll**, fixed labels, presets and preamp always visible; advanced graphic 20/31/64 in a scrollable full-screen editor and parametric in a fullscreen editor with table; touch changes interpolated.

### 5.10 General DSP (Preferences > DSP and shortcuts)
(docs/audio/03-dsp.md; docs/audio/01-engine.md; TASK-E04-01/03/07/08)
Each item needs control + state + tooltip:
- **ReplayGain**: mode Off / Track / Album; preamp with tag (−24..+12) and preamp for untagged files; peak guard (on/off); show applied gain ("RG −6.2 dB") in inspector/display. No automatic scan on playback; "Analyze loudness…" opens a job (M3).
- **Crossfade**: duration 0–12 s (slider), equal-power curve (default), **separate** for manual vs automatic advance; off by default; note: crossfade off does not affect gapless.
- **Gapless**: toggle (default on); notice when a sample-rate change in exclusive mode breaks gapless (visible limitation).
- **True-peak limiter**: toggle, default ceiling −1 dBTP, announced lookahead, visible latency.
- **Compressor**: threshold, ratio, attack, release, knee, makeup + **gain-reduction meter** (downward bar).
- **Tone**: bass/treble. **Balance/pan**, **mono**, **channel routing**, **fades**, **downmix** (with declared headroom).
- **Tempo** 0.5–2× with pitch preservation; **Pitch** ±12 semitones; independent speed (podcast per category; music does not inherit 1.5×).
- **Creative effects (M3)**: reverb, echo, chorus, flanger; silence trim (visible indicator when on); exploratory "Voice remover" (M5, warning that it is not professional separation).
- **Dither** (TPDF, only when reducing to integer, never in DSD/float/transparent) and **Resampler** (balanced / high quality) as advanced options with latency/CPU.
- Each node can be switched on/off; off node = real bypass; plugin declares latency/channels/cost. Node failure → disabled with visible diagnostic (NaN/Inf).
- States: default/custom; unavailable in transparent; unavailable by channels/rate; node error.

#### 5.10.1 DSP access and navigation — visual design (DSP-UX-01..05)
(docs/design/05-dsp-panel.md §8.1 "Acesso visível e navegação por famílias"; docs/product/05-decisions.md D-DSP-11, D-DSP-13)
Origin: feedback on the existing prototype ("too many inputs", wish for round buttons and slides, DSP hidden too deep). Read as **circular knobs, sliders and switches suited to each parameter**, with precise numeric entry on demand — not every action becomes a round button, and graphic-EQ sliders are never replaced by knobs.
- **DSP-UX-01 — Main entry:** design an **"Áudio e efeitos"** entry in the extended-layout header: icon plus visible label. Keep the existing transport/preferences access and the compact EQ panel.
- **DSP-UX-02 — Contextual shortcuts:** next to the instrument area, four shortcuts: **EQ**, **Dinâmica**, **Estéreo**, **Vitrola** (EQ opens the advanced editor; the others land on the matching module/family; reference destinations: compressor, stereo width, vinyl). Same state behind every entry point; the Vitrola shortcut is an entry, not a merge of Vinyl and Tube.
- **DSP-UX-03 — Categories:** eight recognizable families at the top of the panel — **Ganho/EQ, Dinâmica, Acústica, Estéreo, Tempo/formato, Criativos, Vitrola, Saída** — as *navigation* (tabs/segmented rail), never drawn as on/off switches, never implying "only eight effects". Keep the full searchable module list and the advanced-equalizer entry. **Meters is its own area**, outside the family rail.
- **DSP-UX-04 — Coherent selection:** design current-category and current-module highlight, selected item scrolled into view, and an active-search state that cannot hide the destination of a shortcut.
- **DSP-UX-05 — Space and focus:** narrow supported widths get an openable/closable module navigation (drawer); essential parameters and targets are never shrunk. Closing returns focus to the trigger (or to the main entry if it is not visible) — draw the focus ring on both.

#### 5.10.2 Control choice by parameter — visual rule (DSP-UX-06..10)
(docs/design/05-dsp-panel.md §8.2 "Escolha do controle conforme o parâmetro"; D-DSP-15)

| ID | Component | Where it is used (design rule) |
|---|---|---|
| DSP-UX-06 | Vector circular **knob** | Gain, frequency, Q, intensity, drive, ratio, other continuous parameters. Label, unit, position and value legible. Visual reference: **88×88 DIP** area and **270° arc** (consolidate into existing tokens; not a fixed size on every platform). |
| DSP-UX-07 | **Slider** | Times, dry/wet, stereo width, balance/pan, linear comparisons. Graphic EQ keeps one real slider per band. |
| DSP-UX-08 | **On/off switch** | Binary options; the label makes "enabled" vs "bypass" unambiguous. |
| DSP-UX-09 | **Text field or selector** | Names, filter types, channels, models, discrete options. |
| DSP-UX-10 | **Clickable value** + exact editor | Discreet readout below/beside the knob or slider; clicking opens a small numeric editor with unit and range. Never a permanent grid of numeric inputs. |

Group related parameters by function with logical reading order and aligned labels/units; avoid decorative boxes around every parameter and many competing colors (DSP-UX-20). Draw the **exact-editor error state**: editor stays open, explanatory text, last valid value still shown (DSP-UX-15); a rejected value is never drawn as accepted. Draw a **preserved off-step value** (for example −23,5 dB) exactly as stored, not rounded (DSP-UX-14). A rejected crossover pair (equal/inverted) shows the relation in the message and keeps the previous values on knobs/graph (DSP-UX-16). Gestures, keyboard steps, scales and validation rules (DSP-UX-11..13, 17) are behavior, not design scope: see docs/design/05-dsp-panel.md §8.3 and the GPT/MVP document.

#### 5.10.3 Hierarchy, accessibility and feedback — visual states (DSP-UX-18..21)
(docs/design/05-dsp-panel.md §8.4 "Hierarquia, acessibilidade e feedback")
- **DSP-UX-18 — States:** distinct hover, focus, drag, active, bypass, disabled, error and incompatible, each with icon/text, not only color or hover tooltip; unavailability reason in readable text. The whole-chain state is visible separately from any single switch.
- **DSP-UX-19 — Accessibility:** visible focus, sufficient contrast, design-system targets, enlarged text, high contrast and reduced-motion variants; a numeric alternative to drag is visible on demand.
- **DSP-UX-20 — Harmonization:** skin tokens for surfaces, type, icons, states, spacing. **Permanent DSP controls never reduce the space reserved for the original stereo VU meters.**
- **DSP-UX-21 — Feedback:** the control reflects the change at once; a smooth knob animation is not a claim of smooth signal transition.

**Honest-status visuals:** the badge `[DEMO — no audio processing in this mode]` is drawn once and reused on every DSP / EQ / Vitrola / meter screen where processing is simulated or absent. Vinyl and Tube show full configuration with that badge; a "Soft saturation" node is never styled as the Tube Amplifier, and a sample limiter is never styled as True Peak. Real capability list and statuses: see the GPT/MVP document §12.8.2 (full table) and docs/audio/09-dsp-catalog.md "Famílias v1.4 e estado no protótipo (rev. 15, relatado)" (not repeated here). Categories do not imply free reordering of the chain (D-DSP-10; D-DSP-12 open).

### 5.11 Vitrola effects (vinyl + tube) — CONTROLS ONLY
(..\vitrola\docs\superpowers\specs\2026-10-23-vitrola-design.md — §3: Vitrola is **audio simulation only**; **do not draw a turntable, platter, tonearm, stylus, tubes or any illustration of the device**.)

Location: Preferences > DSP > "Vitrola" **and** a shortcut in the EQ/chain inspector ("Vitrola" button) **[proposal]**. Use only standard Vynamp components (sliders, toggles, menus).

**Level 1 — Main intensity control (macro)**
- One large control: **"Vitrola intensity" slider/knob** 0–100% (default **subtle**, e.g. 25% **[proposal]**) that moves all parameters coherently (the `macro` map). **On/Off** toggle (bypass with crossfade, no click). Text label of the value ("Subtle / Medium / Strong" **[proposal]** besides the number).
- Status indicator: Active / Automatically disabled ("disabled: bit-perfect path / native DSD / transparent") — vitrola **auto-disables** in those cases; show reason and a "Leave transparent mode" button.
- Multichannel: notice "Vinyl disabled in multichannel; tube per channel" (spec §2).

**Level 2 — Advanced panel (per element; parameter layout is the designer's decision, spec §9)**
*Vinyl section*
- Position on track → disc radius: toggle "Simulate end of side (inner-groove)"; HF loss/distortion intensity.
- A/B sides: "Album in order" vs single-track mode; show current side (A/B, disc n) as text; "Flip side A→B" option.
- Surface noise / crackle / pops: intensity, density, "respect the music (mask)".
- Rotation: speed **33⅓ / 45 rpm** (selector), wow, flutter, warp.
- Rituals: "Needle drop at start", "Run-out groove loop" (until stopped, or 30 s + fade, configurable), "Side flip". **Rituals change duration** (short notice).
- Cartridge: resonance/roll-off, rumble.
- **Aging**: per-disc (album) wear (read-only wear bar), **"Buy new record"** button (resets that album; confirmation); play counter.
*Tube section*
- Warm-up (a few seconds; text indicator "Warming up… / Ready").
- Profile: **Warm pre-amp / Classic push-pull / Single-ended** (menu; no brand names).
- Saturation (dynamic), light compression, power-supply **sag**, transformer (LF/HF roll-off).
- Advanced option **"Couple to volume"** (tube opens up at high volume).
*Footer*: "Restore defaults", Vitrola presets (Subtle / Warm / Rustic **[proposal]**), optional CPU meter, "Sound guide" link.
**States**: off; on; automatically disabled (reason); radio/stream without album context: "virtual 20-min side, rituals off"; node error (safe bypass).
**Forbidden in the design**: any image of a turntable, stylus, spinning disc, tonearm, glowing animated tubes.

### 5.12 Audio output, devices, transparent mode and hi-res
(docs/audio/04-outputs-hires.md, 05-platform-backends.md; TASK-E04-05/06)

**Quick selector** (popover from the display or now-playing bar): device list with type icon (speaker, headphones, USB DAC, Bluetooth, HDMI), name, current backend, "following system default" (mark), **exclusive-busy** indicator. Refresh never blocks the UI; a hung driver shows "Device not responding".
**Preferences > Output**:
- Backend: Shared (WASAPI/PipeWire/PulseAudio/CoreAudio/AAudio) · Exclusive (WASAPI exclusive; direct ALSA; CoreAudio hog) · ASIO (Windows, if installed) — options unavailable on the platform hidden or disabled with reason.
- Rate policy: Follow system · Follow source (exclusive) · Fixed (rate selector) · Auto highest compatible (opt-in).
- Format/bits, channels, buffer (conservative / advanced low-latency with underrun counter), fallback (ask/fail).
- **Per-device profile** (stable ID; persists while absent for reconnection): rate, format, channels, buffer, EQ, volume.
- **Transparent mode / bit-perfect** toggle: on selection, a dialog lists the effects that will be disabled (DSP, RG, fades, limiter, resampling, digital volume) and confirms **once**; condition checklist with ✓/✗ (exact lossless decode, same rate, same channel map, compatible valid bits, unity volume, exclusive-capable backend, no known OS mixer). Exclusive warning text: "occupies the device; other apps may lose sound".
- "Follow default output" (explicit preference).
**Failures (banners/dialogs)**: DAC removed → pause, keep position, **Reconnect** button (never redirects to speakers by default); exclusive busy → offers Shared (does not apply on its own); format rejected → ask for fallback or fail; sleep/resume renegotiates; underruns.
**States**: empty list (no devices), loading, absent device (profile preserved), permission denied (Android: audio/Bluetooth).

### 5.13 Visualizers and switchable VU meter  ★ EXPLICIT MAINTAINER REQUIREMENT

Sources: docs/extensions/03-visualizations-integrations.md; docs/extensions/04-visualizer-contract.md; TASK-E06-05, TASK-E06-06; docs/design/01-visual-system.md.

#### 5.13.1 Principle
The display meter is **switchable**: the user chooses which instrument occupies the space (the 28 DIP classic display strip and the other slots). It is not decoration: technical meters have units (dBFS/dBTP) and a visible scale; decorative effects are labeled as such. A visualizer is **never** presented as an acoustic measurement of the room. **Off** always exists.

#### 5.13.2 How the user switches (all must exist; the spec requires a way to disable and does not prescribe gestures — the rest is [proposal])
1. **Click the meter itself** (left click) → advances to the next mode in the cycle; **Shift+click** → previous mode. [proposal]
2. **"Meter" button (needle/bars icon)** on the display's bottom row next to the meter, opening a **selection menu** with all modes (radio group) + configuration submenus. [proposal]
3. **Right-click the meter** → same menu. [proposal]
4. **Keyboard shortcut: `V`** (next), `Shift+V` (previous) when focus is not a text field; `Ctrl+Shift+V` opens the menu; `Alt+V` toggles Off ↔ last mode. [proposal]
5. **Main menu > View > Meter** and **Preferences > Appearance > Visualization** (default mode per layout). [proposal]
6. Android: **tap** the meter cycles; **long-press** opens a selection sheet; also in Preferences. [proposal]
7. Accessible: the meter is a focusable control (Tab) with a toggle-button role and name "Meter: <current mode>. Press to change"; on change the screen reader announces the new mode name once (no per-frame values).
Tooltip: "Meter: Analog VU — click to switch (V)" / "Medidor: VU analógico — clique para alternar (V)". Mode transition: fade ≤ 120 ms (none in Reduced Motion).

#### 5.13.3 Available modes (default cycle in this order)
| # | Mode | Origin | Visual description |
|---|---|---|---|
| 1 | **Analog VU** (needle) | spec: "stereo VU/RMS" (E06-05) | Two gauges (L and R) with needle, −20…+3 VU scale, red zone above 0 VU, label "VU". Amber for needle/scale (token amber), danger on overload zone. |
| 2 | **Digital peak / PPM bars** | spec: "peak/sample/true-peak" | Horizontal (vertical on Android) L/R bars, dBFS scale (−60…0), peak hold, clip lamp. |
| 3 | **True-peak (dBTP)** | spec: "true-peak" | Like 2, dBTP scale with ceiling mark (−1 dBTP) and numeric readout; aligned with the final guard. |
| 4 | **RMS** | spec: "RMS" | Smooth (average) bars, same aesthetic as 2 in amber. |
| 5 | **Loudness LUFS** (momentary/short-term/integrated) | [proposal] (the spec has a *batch* loudness analyzer and meters for reduction/true-peak; real-time LUFS is a proposal) | Large numeric + horizontal bar, markable −14/−23 LUFS target; **LUFS is never mixed with ReplayGain dB** (distinct labels). |
| 6 | **Spectrum** (log and linear) | spec | Frequency bars 20 Hz–20 kHz (FFT 2048/4096, Hann, smoothing/hold/decay), dB scale, Hz axis consistent with sample rate. |
| 7 | **Oscilloscope / waveform** | spec ("oscilloscope/waveform") | Real-time waveform trace (L/R overlaid or separate). |
| 8 | **Spectrogram** | spec (M3) | Time × frequency cascade. |
| 9 | **Phase / correlation / goniometer** | spec (visualizer-contract) | Lissajous goniometer + correlation bar (−1…+1). |
| 10 | **Scenes / discrete particles** | spec (M3) | Decorative visual, labeled "decorative effect". Normally full screen only. |
| 0 | **Off** | spec (disabling is mandatory) | The 28 DIP strip shows discreet text "Meter off" + icon; zero render cost. |

Modes 1–4 plus 6–7 form the **default short cycle** for the 28 DIP display (VU, PPM, Spectrum, Oscilloscope, Off) **[proposal]**; the rest live in the menu and full screen. Show only modes the build/GPU supports (real capabilities); an unavailable mode appears disabled with a reason (e.g. "requires GPU"). Without GPU: CPU bar fallback or off, never interrupting audio.

#### 5.13.4 Ballistics (visual behavior per mode) — values are [proposal] unless stated
(the spec requires "smoothing/hold/decay in real units"; numbers below are the visual target to design/annotate)
- **Analog VU**: ~300 ms integration (needle takes ≈ 300 ms to reach ~99% of a steady tone), overshoot ≤ 1–1.5%, symmetric return (~300 ms). Reference 0 VU = −18 dBFS (adjustable in Preferences: −14/−18/−20 dBFS). Needle rests at the left when stopped.
- **PPM / sample peak**: **instant** rise (≤ 1 render frame), fall ≈ 20 dB in 1.5 s; **peak hold** 1.5 s (thin marker stays then falls), configurable 0/1/2/5 s/∞.
- **True-peak (dBTP)**: same ballistics as PPM; fixed −1 dBTP ceiling tick.
- **RMS**: ≈ 300 ms window (smooth rise/fall ~300 ms).
- **LUFS**: momentary 400 ms window, short-term 3 s, integrated accumulating; "Reset integrated" button (this mode only).
- **Spectrum**: fast rise (~50 ms), ~300–500 ms decay per band, optional peak (hold) line 1 s; smoothing/persistence configurable.
- **Oscilloscope**: updated in sync with the consumed clock; no persistence by default.
- **Spectrogram**: continuous scroll; configurable speed.
- All: **tap latency** is corrected by the consumed clock (visuals never lead the sound). A slow renderer drops visual frames, never audio. Render rate: 30 fps default, 60 optional (foreground), 10 in power-saving; minimized/background suspends.

#### 5.13.5 Stereo L/R and multichannel
- **Stereo**: two channels labeled "L" and "R" (letters always visible, not just position). Optional "M/S sum" **[proposal]** in advanced.
- **Mono**: a single "M" meter; the second disappears (or mirrors, labeled "mono").
- **Multichannel (up to 8 channels in M3)**: one meter per channel labeled by layout (5.1: L R C LFE Ls Rs; 7.1: L R C LFE Ls Rs Lb Rb); analog VU with more than 2 channels shows only L/R (or downmix) and offers "Show all channels" opening bars (PPM). In the 28 DIP space: thin PPM bars side by side.
- Channel selection (All / L / R / M / S) as a submenu **[proposal]**.
- Selectable tap source: Source / Pre-EQ / Post-EQ / **Final (default)** (Preferences > Appearance > Visualization; unit and Fs always visible in the tooltip). In transparent mode the meter receives a copy and is allowed (does not alter output).

#### 5.13.6 Clip / peak hold
- **CLIP lamp** per channel (rectangle with text "CLIP", danger color + icon/text — never color only). Lights when a sample/true-peak ≥ 0 dBFS (or above ceiling in dBTP mode); **latches for 3 s** or until clicked (clear) **[proposal]**; Shift+click clears all. Clip count in tooltip.
- **Peak hold**: thin marker that stays for the configured time and falls with the release ballistics; double-click "clears peaks".
- ARIA/announcement: on the first clip in a track, a single rate-limited announcement ("Clipping detected") — never per frame.
- Transparent mode does not silently fix clipping; it only indicates it. (docs/audio/01-engine.md)

#### 5.13.7 Paused / stopped / no track / buffering
- **Pause**: needle/bars fall to rest with normal release ballistics and **stay at rest**; peak hold falls/remains 1.5 s then disappears; spectrum falls to zero; oscilloscope shows a flat center line. On resume it returns smoothly (no jump).
- **Stopped/Idle/No track**: meter at rest, scale visible, no animation; optional discreet text "—∞ dB".
- **Buffering/Seeking**: meter at rest (does not freeze at the last value). Stale frames from another *generation* are discarded.
- **Minimized window / app in background / screen off (Android)**: render suspended (zero cost); on return, current state with no "catch-up" animation.
- **Device lost / error**: rest + alert icon on the display.
- **Reduced Motion**: meters **static or slow-updating (2–4 Hz) showing a numeric value**; no animated needle or dancing spectrum; oscilloscope/spectrogram/particles off by default with manual opt-in. No flash > 3/s; CLIP lamp does not blink (lit steady).
- **Accessibility**: one accessible element "Meter: <mode>" with a short description; current value **on demand** (user query), never hundreds of events per second; coalesced snapshot. Units and scale also in text.

#### 5.13.8 Persistence of the choice
- The chosen mode is remembered **across sessions** and per context: **global default** + **per-layout override** (Classic, Mini, Library now-playing, full screen, Android) **[proposal]**. When the layout changes, the meter takes that layout's choice; "Sync with other layouts" (toggle, default on) **[proposal]**.
- Mode settings (VU reference 0 VU = −18 dBFS, hold time, FFT, channel, tap, linear/log, scale) persist **per mode**.
- Per-theme: a theme may suggest an initial mode, but **the user's choice wins**; a theme cannot hide the Off option or the switch control.
- **Off** persists. A power-saving profile (battery/Android background) temporarily overrides without changing the saved preference.

#### 5.13.9 Where it appears in each layout
| Layout | Position / size | Available modes | Notes |
|---|---|---|---|
| **Classic Stack — Player** | bottom strip of the display, 28 DIP tall, display width (352 DIP), not covering text | short cycle: VU (compact version with 2 mini needles), PPM, Spectrum, Oscilloscope, Off | Analog VU in the 28 DIP slot uses **compact horizontal gauges** (L and R side by side) or miniature needles; "Enlarged VU" opens the large meter (popover/instrument window 360×160 DIP **[proposal]**). |
| **Library Workspace** | now-playing bar (96 DIP): 120×40 DIP block beside the seek **[proposal]**; inspector: "Meter" card 280×120 | short cycle + full menu | Can be hidden in preferences. |
| **Mini 420×112** | mini meter of **~64×24 DIP** (two-bar PPM or compact spectrum) beside the time **[proposal]**; analog VU does **not** fit (falls back to PPM) | PPM, compact Spectrum, Off | Click cycles only among these; hideable. |
| **Full screen** (visualizer) | fills window/monitor; discreet overlay with mode name, controls and title | all modes (incl. spectrogram, goniometer, scenes/particles, large VU) | Entered by "Visualizer" button / `F`; **Escape exits** returning focus and position; **does not hijack playback shortcuts** (Space, arrows keep working); no second engine; HUD hides after 3 s idle and reappears on mouse move. |
| **Android Now Playing** | 64 dp strip ("display") + optional immersive full screen | VU/PPM/Spectrum/Oscilloscope/Off | Zero render with screen off or in background; tap cycles; long-press selection sheet. |
| **Android sticky mini-player** | no meter (48 dp) | — | Thin progress bar only. |

Floating/detached visualizer windows (M3, docking): the same selector appears in the window header.

**Lyrics below the VU (LYR-01, LYR-15; D-LYR-09)** — spatial integration only: the summary lyrics component (§5.16) sits immediately below the VU. States are **independent**: switching or turning off the meter never erases the lyrics preference or source; turning lyrics off never turns the instrument off. The 12 original stereo VUs are neither replaced, shrunk nor distorted (VU-INT-01..03 stay intact). Canonical: docs/library/02-tags-lyrics-tools.md#letras-componente-resumido-sob-o-vu-e-fontes.

#### 5.13.10 Presets/scenes and extensibility
Scenes are declarative JSON presets (ID, renderer family, tap, channels, FFT, motion, palette, bounded parameters). No arbitrary shaders/code in 1.0. A skin may **style** meters (palette, frame, scale, needle shape) via tokens and assets, but does **not** change measurement logic. A scene referencing an uninstalled effect falls back explicitly.

#### 5.13.11 Visual integration of the 12 stereo VU meters (VU-INT-01..06)
(docs/design/05-dsp-panel.md §6.1 "Integração visual dos VU meters (VU-INT)"; D-DSP-18)
Finding (reported, external reference rev. 15, not verified in the Vynamp repository): 10 instruments share a common cabinet, one is wood, one champagne; the problem was the fixed contrast of the outer cabinet against each skin's surface. Three presentations are designed per skin: **Original**, **Adaptive metal** (discreet skin-conditioned gradient) and **Integrated to panel** (**default**: surface tokens bring the common cabinet closer to the panel, subtle outlines). Wood and champagne keep their own materials in all three. The choice lives in **Appearance** and persists.
- **VU-INT-01:** the **12 approved SVGs are preserved byte for byte** — no edits to scales, needles, glass, LEDs, markings, coordinates, proportions or identity.
- **VU-INT-02:** adapt only the outer cabinet presentation, its joint/outline and external labels (on the displayed copy); nothing is deleted, cropped or simplified.
- **VU-INT-03:** no competing frame, shadow or container background; the host gives size, focus and interaction, the SVG the instrument.
- **VU-INT-04:** semantic skin tokens only; LEDs, scale and needles are never recolored to match the theme.
- **VU-INT-05:** natural responsive size from the `viewBox`, *Enlarge* option, click to switch, right-click to choose among the 12 models; the finish never alters that menu's order or behavior.
- **VU-INT-06:** designs cover all **12 skins × 12 VUs (144 combinations)**, light and dark, mobile, and the restored original; small details rely on enlargement, never on distortion.
Adaptation is a presentation-time matter, not baked into `.vyntheme` packages. Evidence the reference reports (`meter-finish-validation.json`, `comparacao-acabamentos-vu.html`) is *reported evidence from the external reference rev. 14/15, not verified in the Vynamp repository*. Status: specified; implemented/validated in the prototype (reported); native: nothing today.

#### 5.13.12 Modular flow (graph) canvas — visual design
(docs/design/05-dsp-panel.md §9 "Fluxo modular (grafo) — referência do protótipo"; D-DSP-12)
A **"Fluxo modular"** view, reached from "Áudio e efeitos", complements the traditional editor; it is a **prototype reference**, not a promise of free reordering in the native product (D-DSP-10; D-DSP-12 open). Design:
- **Canvas:** draggable node cards (header drag area), single/Shift/rectangle selection, named group/rack frames, collapsed/expanded node, fixed Input/Output nodes, side inspector, searchable processor library, context menus (background, module, wire), zoom/pan/Fit, discreet grid, optional snap, minimap when content exceeds the view. Zoomed-out is an overview; precise editing stays in the inspector, which also offers a **drag-free connection** (source/target selectors).
- **Ports and wires:** Bézier wires follow the ports. **Audio = solid line; control (CV) = dashed, colored, square ports.** Direction and type visible; invalid-connection preview with message; selection highlight; bypass state. Palettes cyan/violet, amber/green or theme tokens, with luminosity adapted to light and dark themes; glow reducible to zero; animation switchable off.
- **Flow animation** only when local audio is playing, the chain is enabled and both ends belong to the compiled route without bypass; a silent demo shows no flow and carries `[DEMO — no audio processing in this mode]`. System/player reduced motion wins; glow is decorative and never implies level. Text and instruments keep contrast with animation off and in high contrast. Limit messages (node/connection budgets) show an explicit reason, never a silent refusal.
- **Chain presets** Clean, Warm Analog, Vinyl, Studio, Spatial load in bypass; Vinyl shows a configuration-only state with the demo badge. "Studio" is not a mastering-certification badge.
Capabilities, limits and processing status of each node family: GPT/MVP document §12.8.2 and docs/audio/09-dsp-catalog.md ("Famílias v1.4 e estado no protótipo (rev. 15, relatado)"); docs/design/05-dsp-panel.md §9 is the visual/graph reference — not repeated in this visual document.

### 5.14 Radio and streams
(docs/streaming/01-client.md; TASK-E05-03/07/08; docs/product/04)
- **Open URL**: URL field (Ctrl+L), paste, validate protocol, Connect button; inline warning for unencrypted `http://` (no credentials); supports M3U/PLS/Icecast/SHOUTcast/HLS.
- **Station favorites** (bookmarks): list with name, logo (optional), genre, bitrate, country **[proposal]**; add/remove/rename; drag to playlist.
- **Live playback**: **LIVE** badge, station name, current ICY title (encoding selectable in advanced preferences; markup sanitized), approximate bitrate indicator ("~128 kbps (estimated)"), buffer time and **estimated live latency**; **no fake duration**, seek disabled without DVR.
- **Connection states** (text + icon): Connecting → Buffering → Playing; **Reconnecting (attempt n/5, backoff 1/2/4/8/16 s)**; Recoverable vs terminal failure (401/403, unsupported codec, invalid TLS → no blind retry, clear message and action); "Station ended — Resume" (explicit).
- **Rebuffer**: short fade/mute, resumes without repeating old audio.
- **Timeshift (M3)**: pausing live with up to 30 min window: "Go live" button, limited timeline; without timeshift, pause resumes at live edge and **informs of the jump**.
- **Record** (M3): REC button (danger red + text "Recording"), size, free space, source; optional split on title change (with CUE); safe abort; only when permitted. Content-responsibility notice.
- **HLS quality (M3)**: Auto / manual (variant list); manual does not switch by itself except on error with confirmation.
- Radio screen states: empty (CTA + suggested stations **[proposal]**), loading, offline (local favorites not erased), network error, permission denied (Android).
- Stream health ("Details" panel): startup time, bitrate, rebuffer count, live latency; **private URLs redacted**.

### 5.15 Podcasts and audiobooks
(docs/streaming/02-podcasts-recording.md; TASK-E05-05, E08-09)
- Subscribe by **RSS/Atom URL**; feed list (cover, title, unplayed count); episode list with **unplayed / in progress (progress bar) / completed** states; download: queued / downloading % / paused / verifying / completed / failed; quota and metered-network limits (Android).
- Specific controls: **speed** (0.5–2×, per category), **skip −15 s / +30 s** (configurable), chapters (list), bookmarks (add, name), trim silence (visible indicator when on), mark as completed, delete download (keeps bookmarks).
- Audiobooks (M4): M4B, per-book progress, current chapter, named bookmark; separate from the music queue.
- States: no subscriptions (CTA), feed error, offline (downloaded episodes keep working), no space, rights/expiry ("Available offline until …").

### 5.16 Lyrics
(docs/library/02-tags-lyrics-tools.md; TASK-E03-09)
- **Lyrics** tab/panel (Android: tab; desktop: inspector or window/overlay). Plain or **synced (LRC/SYLT)**: current line highlighted (lime + weight), previous/next in text-secondary, auto-scroll following the **consumed clock**.
- Controls: **Pause auto-scroll** (toggle; re-enabled by tapping "Back to current line"), **Offset** (−/+ 100 ms, reset, value in ms), text size, search lyrics (opt-in provider, with credit/license), edit (M3), copy.
- Clicking a synced line = seek to that line's time **[proposal]**.
- States: no lyrics (CTA "Search" opt-in), loading, error, offline (no involuntary attempt). Screen reader reads the current line on demand, not on every automatic line change.

**Summary component under the VU — visual design (LYR-01..15; D-LYR-01..09).** Canonical: docs/library/02-tags-lyrics-tools.md#letras-componente-resumido-sob-o-vu-e-fontes. This document covers only visual/layout/states/tokens/accessibility; behavior (sources, persistence, concurrency, tag reading) lives in the canonical doc and the GPT MVP document. The component **adds** to the full panel above and does not replace it; the full panel's controls remain required and **pending**; line-click-to-seek stays a **[proposal]** (D-LYR-02). Status levels, never merged (D-LYR-08): *specified* · *implemented in the prototype (reported)* · *validated in the prototype (reported)* · *native (today: nothing)*; anything about the rev. 14 reference is reported evidence from the external reference, not verified in this repository.

| ID | Design obligation |
|---|---|
| LYR-01 | Slot immediately below the VU, on the skin surface, no extra frame or box. |
| LYR-02 | Three positions (previous / current / next); adjacent dimmer but legible; current stronger in weight and contrast. |
| LYR-03 | One visual line per position; no grid break, no page horizontal scroll; a control to see the full text without truncation. |
| LYR-04 | On advance the set moves up and the next line takes the center; paused playback freezes it (design the frozen state). |
| LYR-05 | The visual line reflects the playback position only; the design shows no independent clock or progress of its own. |
| LYR-06 | Reduced motion or a jump: instant change, no travel through intermediate lines; the transition is decoration and never changes which line is selected. |
| LYR-07 | An embedded source needs no import step in the UI; show the discreet origin instead. |
| LYR-08 | Plain-text state: "Lyrics are in the metadata but not synchronized" + "View full lyrics" action; no highlighted "current" line. |
| LYR-09 | Import LRC / return to metadata actions; invalid-LRC error message that keeps the previous valid lyrics visible. |
| LYR-10 | Options menu beside the lyrics title: Show lyrics · Turn lyrics off · Instrumental / no lyrics · Import LRC · Use lyrics from metadata; no permanent select. |
| LYR-11 | "Off" state of the slot (global preference) is visually distinct from "Instrumental". |
| LYR-12 | "Instrumental / no lyrics" state is a per-track, reversible choice, with an obvious way back (Show lyrics); never inferred from missing tags (D-LYR-04). |
| LYR-13 | Distinct designs for: synchronized, plain text, searching metadata, no lyrics found, instrumental, off, error, unsupported format. Absence, read failure and unsupported format must not look alike. |
| LYR-14 | Discreet origin label: metadata/field, local LRC or DEMO; sample data carries DEMO and never implies real singing. |
| LYR-15 | The slot keeps its own state when the VU mode changes or is turned off; turning lyrics off leaves the VU untouched. |

- **Edge states:** before the first timestamp a discreet waiting state (the first line is not repeated as already sung); with no previous/next line the position stays empty; an empty synchronized segment may read as an instrumental gap without switching the track to Instrumental.
- **Reported starting values (rev. 14 reference, not verified in this repository; tokens and text scale win):** line height 26 DIP desktop / 30 DIP narrow; current text 14 DIP, adjacent 13 DIP; upward shift 260 ms. They are not fixed sizes that authorize clipping.
- **Tokens and contrast:** semantic skin tokens only, no colors that work in a single theme; the current line never relies on color alone; adjacent-line dimming is checked for contrast after any transparency; in High Contrast the hierarchy comes from weight/marking.
- **Menu and focus:** keyboard-operable menu, clear labels, visible focus ring, focus returns to the trigger; no hover needed to find actions; verse changes are not announced automatically.
- **Visual acceptance checks (LYR-AC-01..12; full table in the GPT MVP document §12.14.2):** LYR-AC-01, LYR-AC-02, LYR-AC-03 correct line and upward step on advance, pause and seek/track change; LYR-AC-04 plain-text state; LYR-AC-05 tag-family fixtures (behavioral); LYR-AC-06 LRC import valid then invalid; LYR-AC-07 Off versus Instrumental persistence; LYR-AC-08 stale-result race (behavioral); LYR-AC-09 reduced motion and focus return; LYR-AC-10 the 12 original VUs, themes, tall/short, narrow and enlarged text without distortion, clipping or overflow; LYR-AC-11 distinct no-lyrics/error/unsupported messages; LYR-AC-12 regression of transport, waveform, VU switching and DSP.
- Native: nothing implemented or validated.

### 5.17 Recording, sleep timer, scheduler/alarm
(docs/library/02; TASK-E08-05)
- **Sleep timer**: popover with presets (15/30/45/60 min, End of track, End of queue), custom field, optional fade out, visible countdown in the player (chip "Sleep 12:30"), Cancel. Android: closes playback, does not power off the phone. Powering off the OS is an option disabled by default, with confirmation.
- **Scheduler/alarm (M3, desktop)**: schedule list (play playlist/station, stop, pause, fade, start/stop recording, alarm), time/duration/recurrence, **explicit time zone**, visible next run, alarm volume ceiling, quiet hours; states active/inactive/executed/failed. No wake-from-sleep promise (show real capability).

### 5.18 Preferences (880×680 window)
(docs/design/02-layouts.md)
Side navigation (vertical list, keyboard ↑/↓): **Playback · Output · DSP · Library · Streaming · Appearance · Shortcuts · Plugins · Privacy · Advanced**. Footer: Restore defaults (per section, confirmation), Close. Settings search at the top **[proposal]**. Non-modal when possible; changes apply immediately (no "Apply" button) except those that restart output (Apply button with countdown).

Minimum content per section:
- **Playback**: resume queue on open (no autoplay), gapless, crossfade, ReplayGain, stop after track/queue, Previous behavior, podcast skip, history (clear), "stop on close".
- **Output**: see 5.12.
- **DSP**: see 5.10/5.11, presets, per-device profiles.
- **Library**: watched folders (add/remove, online/offline status), scheduled scan, cover rules (embedded > folder), safe naming templates, duplicates, recycle bin on delete, tags (configurable backup), search language.
- **Streaming**: buffers (VOD 3 s / radio 8 s; 2–30 s), disk cache (2 GiB default limit, can be disabled), timeout/retry, proxy, HLS quality, ICY encoding, recording (folder, passthrough/FLAC/WAV/Opus format, split), downloads (2 GiB quota, Wi-Fi only).
- **Appearance**: theme (gallery), density (Compact/Comfortable), touch mode, font scale (100–200%), reduced motion (follow OS / force), High contrast, language (pt-BR/EN), visualization (meter mode, FPS 10/30/60, tap, FFT, VU reference), show cover, show EQ by default, Mini always-on-top, close to tray, follow system theme **[proposal]**; **Lyrics** (LYR-10, LYR-11, LYR-15): global "Show lyrics / Turn lyrics off" preference, independent of the meter mode and of the per-track Instrumental mark (library/02#letras-componente-resumido-sob-o-vu-e-fontes; D-LYR-04).
- **Shortcuts**: table (action, local shortcut, global shortcut) with **conflict detection**, rebind (key capture), restore, export/import **[proposal]**; global shortcuts disabled by default **[proposal]**; notice about Wayland lacking global hotkeys.
- **Plugins**: see 5.19.
- **Privacy**: no mandatory telemetry; scrobblers (opt-in/revoke), Discord rich presence (opt-in; never send local paths), online cover/lyrics providers (opt-in), local history, diagnostics (.vyndiag: manual export). Lyrics: local reading by default; design any online-search entry as opt-in with provider credit/license, never implying silent discovery (D-LYR-03, LYR-07, LYR-09).
- **Advanced**: safe mode, logs, reset settings, OS integration (file association **by consent** with extension selection; open system default-apps settings), single instance, CLI, remote control (M4: disabled by default; pairing), available formats (from the real build registry; shows Required/Conditional/Unavailable).

### 5.19 Themes/skins (gallery, preview, editor) and Plugins
(docs/extensions/01-themes.md, 02-plugins.md; TASK-E06-02/07/08; TASK-E07-07)
- **Gallery (Library > Themes)**: cards with **app-generated** thumbnail, name, author, version, license, tags (Dark/Light/High contrast), badges "Bundled", "Installed", "Active". Actions: **Preview**, **Apply**, Duplicate to edit, Export (.vyntheme), Remove, Install from file (.vyntheme; drag and drop). Install: staging → validation → preview → apply; an invalid theme **keeps the previous one** with errors shown by JSON pointer; ID conflict shows version/author (no silent overwrite). Online registry (M4) only with a "future" mark.
- **Preview**: side-by-side view of the 3 layouts (Classic/Library/Mini) with sample data and states (normal/hover/pressed/focus/disabled), DPI selector (100/125/150/200%) and density; applying does not affect audio; "Cancel preview" restores immediately.
- **Theme editor (M3)**: tokens (color with contrast meter), position/constraints, import assets, preview across DPI/density/states, validate contrast (failure alert), undo/redo, export. Errors by JSON pointer.
- **Safe mode**: restores the bundled theme and ignores user packages without deleting; entered by a key at startup.
- **Plugin manager**: list (name, author, version, origin, permissions, signature, status active/disabled/error/circuit open), details, granted permissions (network, files) with revocation, enable/disable, update/rollback, remove, install from file (.vynplug). Installing does **not** grant network/files automatically.

### 5.20 Job manager (800×520)
(TASK-E08-01)
Job table: type (scan, conversion, ReplayGain/loudness/BPM, download, recording, dedupe, organize, tag write), target, progress (bar + %), status (queued/running/paused/completed/failed/cancelled), time, actions **Cancel / Pause / Resume / Retry / Open result / Clear completed**. Cancelling removes temporaries and preserves the source. Errors with useful text + "Details" panel. Summary indicator (count chip) in sidebar/status bar; job completion uses a rate-limited accessible announcement. Converter: limited worker queue; overwrite off by default.

### 5.21 Track details and tag editor (800×640)
(docs/library/02-tags-lyrics-tools.md)
Tabs: **Info** (read-only technical) · **Tags** (editor; batch with preview before write; mixed fields = "different values"; "empty" ≠ "do not change"; title, artists, album artist, album, genre, date, track/disc/totals, composer, comment, copyright, BPM, key, rating, ISRC, RG track/album and peak) · **Cover** (replace, extract, remove; PNG/JPEG/WebP) · **Lyrics** · **Chapters**. Footer: Cancel / Preview changes / Write. Writing: temp file + rename, configurable backup, report; file in use: wait for release; Android: warn if the provider lacks atomic write. Online identification: opt-in, confidence (%), never auto-rewrite low confidence.
States: read-only file, in use, write error, no changes.

### 5.22 Notifications, errors, toasts and dialogs
(docs/design/02-layouts.md; docs/design/03)
- **Toasts** (4–6 s, accessible via live region, rate-limited): action confirmation ("Added to queue"), with **Undo** where applicable.
- **Persistent banners** at the top of the area: offline, device lost, permission denied, queue restore, scan in progress.
- **Modal dialogs**: focus on the first safe action (e.g. "Cancel"), Escape cancels, focus returns to the invoker. Delete file: confirmation + recycle bin. Leave transparent mode. Apply theme. Close to tray (first time).
- **Errors**: short title, plain-language cause, **useful action** (Try again, Choose another file, Open Settings, See details). Collapsed "Technical details" panel. Partial error keeps valid items.
- Types to design: file not found; unsupported format (UnsupportedFormat/Profile — "no decoder for this profile", never retried as AAC-LC); corrupt file; permission denied; network/DNS/TLS; stream ended; device busy/removed; disk full; theme/ID conflict; plugin failed (circuit open); license/rights expired (never silently delete).
- States: info, success, warning, error (icon + text, not color only).

### 5.23 Onboarding, first run and empty states
(docs/product/04-user-journeys.md; docs/product/01-vision-scope.md)
- First run: opens in **Classic Graphite**, Idle, **volume 50%**, ReplayGain on when tags exist, EQ flat/off, crossfade off. **No** account prompt, **no** disk indexing, **no** autoplay.
- Empty home screen with three actions: **Open file**, **Add folder**, **Open radio**; drag-and-drop accepted. Folder import is optional, with count preview and cancellable progress.
- Light, dismissible tips (first time): main shortcuts, Mini mode, library. **No mandatory tour.**
- Empty states with CTA for: playlist, queue, library, favorites, radio, podcasts, downloads, lyrics, jobs, plugins, themes, history.
- Android first run: permission screen **explaining** (media, notifications) with alternative (SAF picker / radio); denied permission does not block radio.

### 5.24 About, splash, safe mode
- About: logo (lockup), name Vynamp, version, build, third-party licenses, guide link, "Copy diagnostic info" button. Splash and About use the C — Pulso brand; image with alt text "Vynamp". The symbol next to the name is decorative.
- Safe mode: banner "Safe mode: bundled theme and extensions disabled" + Exit safe mode button.

### 5.25 Menus
**Main menu (hamburger icon / macOS menu bar)**: File (Open file Ctrl+O, Open folder, Open URL Ctrl+L, Open playlist, Save playlist, Import, Quit Ctrl+Q) · Playback (Play/Pause, Stop, Previous, Next, Shuffle, Repeat, A-B, Stop after track/queue, Speed, Sleep timer) · Audio (EQ, DSP, Vitrola, Output, Transparent mode, Audio chain) · View (Meter ▸ modes, Full-screen visualizer, Show EQ, Show playlist, Show cover, Mini-player, Library, Always on top) · Library (Add folder, Rescan, Manage sources) · Tools (Jobs, Converter, ReplayGain, Dedupe, Rename, Scheduler) · Themes and Plugins · Help (Shortcuts, About, Diagnostics, Safe mode). **All items show their shortcut; icons have accessible names.**
**Context menus**: track/row, cover, meter (modes), seek slider (add A/B marker/bookmark), title bar (layout modes), tray, sidebar (rename/remove source), stations, episodes, playlists (tab: rename, duplicate, lock, close).

### 5.26 Android — navigation, gestures and notification
(docs/design/04-mobile-tablet.md; docs/platforms/02-android.md)
- **App bar 48 dp**: screen title, search, overflow. **Bottom nav 64 dp**: Library / Playlists / Radio / Podcasts.
- **Now Playing** (phone): app bar (minimize/back, overflow), display 64 dp (multi-line limited title/artist, elapsed/remaining time 28 sp, badges), cover ≤ 208 dp, seek 48 dp, controls: Shuffle, Previous 48, **Play/Pause 56**, Next 48, Repeat; row: Favorite, Queue, Volume (sheet), Output (device/Bluetooth), Sleep timer, Share, Overflow (EQ, Details, Audio chain). Tabs 56: **Queue · EQ · Lyrics · Info**.
- **Gestures [proposal, always with an equivalent button]**: swipe down on Now Playing minimizes; horizontal swipe on cover = previous/next; double-tap cover = play/pause; swipe a queue item sideways = actions (add to queue / remove from list with **Undo**; **never** deletes file); long-press = multi-select/reorder via handle; pull-to-refresh in library/podcasts. System back gesture returns to navigation (does not stop sound).
- **Notification / lock screen / Bluetooth / headset**: mirror engine state; Previous/Play-Pause/Next buttons; headset unplug (noisy) pauses without playing through the speaker.
- **Audio focus**: textual indication when paused by another app ("Paused: another app is using audio") with a Resume button.
- Accessibility: TalkBack and Switch Access; consistent focus order; value/unit on sliders; name of each EQ band; seek moves via accessibility actions.
- Specific states: permission denied, expired SAF grant (item marked unavailable, playlist preserved), metered/battery (downloads), airplane mode (offline), screen off (no render).

---

## 6. Summary table of controls

State legend: N = normal, H = hover, P = pressed, F = focus, D = disabled, A = active/toggle on, C = loading, E = error. "default" = all 2.9 defaults. Shortcuts marked [proposal] carry ‡.

| Area | Control | Action | Shortcut | States |
|---|---|---|---|---|
| Header | Menu | opens main menu | Alt+M‡ / F10 | N H P F |
| Header | Minimize | minimizes | — | default |
| Header | Close | closes/tray | Ctrl+Q (quit) | default |
| Header | Maximize/Restore (Library) | toggles | double-click header | default |
| Header | Pin (always on top, Mini) | toggles | — | A indicated |
| Player | Previous | previous track | Alt+← | default (D empty queue) |
| Player | Play | play | Space | C/A |
| Player | Pause | pause | Space | A |
| Player | Stop | stop | S‡ | D in Idle |
| Player | Next | next | Alt+→ | D at end |
| Player | Open | open file | Ctrl+O | N |
| Player | Source Local/Radio/Library | opens source | Ctrl+O / Ctrl+L / L‡ | N |
| Player | Shuffle | toggle | H‡ | A |
| Player | Repeat | 3 states | R‡ | off/seq/track |
| Player | Time (click) | toggles elapsed/remaining | T‡ | text |
| Player | Meter (click / menu) | switches mode | V‡ / Shift+V‡ | 11 modes |
| Player | Cover | enlarges | — | N D (no cover) |
| Seek | Slider | position | ←/→ ±5 s‡ | N H dragging D LIVE |
| Seek | A / B / Clear | A-B loop | [ ‡ / ] ‡ / \ ‡ | A |
| Seek | Go live | live edge (timeshift) | — | D outside window |
| Volume | Slider | volume | ↑/↓‡ | N H D |
| Volume | Mute | mute | M‡ | A (struck + text) |
| Volume | Pan/Balance | popup | — | N |
| Playlist | Tab / New | switch/create | Ctrl+T‡ | A |
| Playlist | Row | plays/selects | Enter, double-click | current/selected/unavail. |
| Playlist | Add / Remove / Sort | management | Delete | default |
| Playlist | Undo / Redo | history | Ctrl+Z / Ctrl+Shift+Z | D no history |
| Playlist | Save / Import | file | Ctrl+S‡ | C E |
| Playlist | Lock | padlock | — | A |
| Playlist | Search | filters | Ctrl+F | N |
| EQ | On/Off | enable | E‡ | A |
| EQ | Auto | auto preset | — | A |
| EQ | Reset | flat | — | N |
| EQ | Presets ▾ | menu | — | N |
| EQ | Preamp (slider) | −24..+12 dB | ↑/↓ | default |
| EQ | 10 bands (sliders) | ±12 dB | ↑/↓ Home/End | default |
| EQ | Open advanced editor | 960×600 window | Ctrl+E‡ | N |
| Adv. EQ | Mode Graphic 10 / 20 / 31 / 64 / Parametric | changes editable band count and graph | Tab / arrows | A (64: native M5 unavailable state) |
| Adv. EQ | Graphic band sliders 10/20/31/64 | individual gain, frequency shown | ↑/↓ Home/End | A |
| Adv. EQ | Parametric 1–32 filters | add/remove/enable/edit | Tab / arrows | A |
| Adv. EQ | Band: type/freq/gain/Q/channel | edit | Tab / arrows | N D (>Nyquist) |
| DSP | ReplayGain mode | Off/Track/Album | — | N |
| DSP | Crossfade (slider) | 0–12 s | — | N |
| DSP | Gapless | toggle | — | A |
| DSP | Limiter/Compressor/Tone/Tempo/Pitch | parameters | — | N D |
| DSP | "Áudio e efeitos" (header entry) + shortcuts EQ / Dinâmica / Estéreo / Vitrola | opens panel / editor / module | — | N H F A |
| DSP | Family navigation (8 families, search, module list) | selects family/module | Tab / arrows | A D |
| DSP | Knob / Slider / Switch / Selector / clickable readout | per parameter metadata (DSP-UX-06..10) | arrows, Shift = fine | N H F P D E incompatible |
| Vitrola | On/Off | bypass | — | A / D auto |
| Vitrola | Intensity (macro) | 0–100% | ←/→ | N |
| Vitrola | Advanced panel (items of 5.11) | per element | — | N |
| Vitrola | Buy new record | resets wear | — | N (confirm) |
| Output | Device selector | switch | — | N C E |
| Output | Backend / Rate / Buffer | config | — | N D |
| Output | Transparent mode | toggle with confirmation | — | A |
| Meter | Switch mode | see 5.13 | V‡ | — |
| Meter | Clear clip/peaks | reset | click / Shift+click | — |
| Meter | Full screen | visualizer | F‡ / Esc exits | A |
| Meter | VU finish (Appearance) | Original / Adaptive metal / Integrated to panel (default) | — | A |
| Library | Global search | filters | Ctrl+F | C |
| Library | Filters / Sort / View | refine | — | A |
| Library | Add folder / Rescan | imports | — | C E |
| Library | Favorite / Rate | metadata | F‡ (favorite)/1–5‡ | A |
| Radio | URL / Connect | opens stream | Ctrl+L | C E |
| Radio | REC | records | — | A E |
| Radio | HLS quality | Auto/manual | — | N |
| Podcast | Speed / ±15/30 s / Download | long-form | ‡ | C |
| Lyrics | Auto-scroll / Offset | reading | — | A |
| Sleep | Timer | popover | — | A (chip) |
| Jobs | Cancel/Pause/Resume | jobs | — | D C |
| Prefs | Navigation / Apply / Restore | config | Ctrl+P | N |
| Themes | Preview / Apply / Install / Remove | skins | — | C E |
| Plugins | Enable / Permissions / Update | plugins | — | A E |
| Android | Play/Pause 56 dp, Prev/Next 48 dp, Seek 48 dp | transport | media buttons | default |
| Android | Tabs Queue/EQ/Lyrics/Info; Bottom nav; Sheets | navigation | — | A |
| Notification | Prev / Play-Pause / Next | MediaSession | media buttons | C |

---

## 7. Keyboard shortcuts

### 7.1 From the spec (docs/product/04-user-journeys.md; docs/design/03)
Space play/pause (when focus is not an editor) · Ctrl+O open · Ctrl+L URL · Ctrl+F search · Ctrl+P preferences · Ctrl+Q quit · Alt+← / Alt+→ previous/next · ↑/↓ navigate list · Delete removes from playlist (never deletes physical media) · Tab/Shift+Tab traverse groups · Enter activates · Escape closes popup/dialog/full screen · Home/End start/end of list. Global shortcuts (TASK-E08-06): configurable, with conflict detection and rebind; OS media keys.

### 7.2 Proposals [proposal] (to be validated; no known conflict with the spec)
Window-local: S stop · H shuffle · R repeat · M mute · T toggle time · E EQ on/off · Ctrl+E EQ editor · V / Shift+V next/previous meter · Alt+V meter on/off · F full-screen visualizer · Ctrl+T new playlist · Ctrl+S save playlist · Alt+Enter play next · Shift+Enter add to queue · Alt+↑/↓ move item · ←/→ seek ±5 s, Shift ±30 s, PageUp/PageDown ±1 min · ↑/↓ on volume focus ±2% · `[` `]` `\` set A, set B, clear loop · Ctrl+Z / Ctrl+Shift+Z undo/redo · F (favorite) and 1–5 (rating) in lists · Ctrl+1/2/3 switch Classic/Library/Mini · F1 shortcut help · F10 focus menu.
Global (disabled by default): Play/Pause, Next, Previous, Stop, Volume ±, Mute. Show detected conflicts.
Rules: no app shortcut captures keys inside text fields; **the full-screen visualizer does not hijack playback shortcuts**; shortcuts appear in menus and tooltips; all are rebindable (Preferences > Shortcuts).
macOS: Cmd instead of Ctrl; Opt instead of Alt **[proposal]**. Android: external keyboard maps the same local shortcuts.

---

## 8. The skin system

### 8.1 Format and package (from the spec: docs/extensions/01-themes.md)
- **`.vyntheme`** package: ZIP with `manifest.json`, `tokens.json`, `layouts/*.json`, `assets/`, `license.txt`.
- **Forbidden** in the package: JS, arbitrary shaders, executables, DLLs, WASM, remote fonts. Budgets: 20 MiB unpacked, ≤ 200 files, images ≤ 4096 px per dimension, decode ≤ 64 MiB, manifest ≤ 256 KiB. SVG is sanitized (no scripts/external refs) or rasterized.
- Manifest: `schema_version` 1, `id` reverse-domain, `version` SemVer, `name`, `author`, `license`, `min_app_version`, `layouts`, `assets`; SHA256 hashes; optional v1 signature.
- Install in staging, full validation, atomic commit per version, restore previous if the renderer fails. Unknown optional fields warn; an unknown schema major is rejected.
- Layout nodes: `panel`, `text`, `icon`, `button`, `slider`, `list`, `image`, `visualizer`; constraints min/max/anchor/gap; **no loops or unbounded expressions**; cycles = error.
- Allowed bindings: `playback.state/title/time`, `volume`, EQ summary, queue count; actions: `play/pause/seek/open-url/show-library`. **No** arbitrary bindings.
- The skin **never** changes audio, network or security settings.
- Codex's design MUST be expressible in these nodes/constraints. Avoid effects needing code (per-skin particles, custom shaders). Any "animation" is a simple declarative transition.

### 8.2 What a skin MAY customize
Colors (tokens), declarative geometry within constraints, assets (images/SVG frame, textures, buttons, display background, meter needle/scale, icons), typography (free fonts bundled with license; no remote fonts), radius/bevel/elevation, position and size of regions respecting minimums, visualizer/VU palette (via scene), **lyrics component styling (LYR-01, LYR-02, LYR-14): tokens for current line (weight + contrast), adjacent lines, origin/state text, menu and focus ring in one slot below the VU on the skin surface; sizes and line height from tokens and text scale (canonical library/02#letras-componente-resumido-sob-o-vu-e-fontes)**, default density, Classic/Library/Mini layouts (or inherit fallback), optional Android phone/tablet profiles (mandatory builtin fallback; mapped to accessible Compose components).

### 8.3 What is FIXED (a skin cannot change)
- Semantic structure and accessibility: names, roles, values, focus order, visible focus ring, tooltips.
- Mandatory controls that **cannot be hidden**: **Stop, Mute**, play/pause, volume, seek (or LIVE), menu access, the meter switch/off control, window controls and access to Preferences/Themes/safe mode; transparent-mode/quality indicator when active; error/offline notices.
- Minimum targets (24×24 DIP desktop, 44 touch, 48 dp Android), minimum contrast (4.5:1 / 3:1), Reduced Motion / High Contrast / font scale (take precedence).
- Measurement logic, ballistics, meter units and quality badges.
- **Lyrics (LYR-02, LYR-05, LYR-11..13, LYR-15):** line selection by the playback clock, the distinct state vocabulary, the menu contents (LYR-10), the global Off and per-track Instrumental semantics, and independence from the VU. A skin styles the component only.
- Safety and legal text (licenses, unencrypted http notice, deletion confirmations).
- The Vynamp brand in About/splash/installers follows the identity guide.

### 8.4 Runtime switching
Apply a theme without interrupting audio or restarting the engine; an invalid theme keeps the previous; preview (Library > Themes) applies in a "provisional" mode with an "Apply / Cancel" bar and automatic revert on exit; theme version update/rollback without losing compatible settings; **safe mode** returns to the bundled one.

### 8.5 Default skin and alternatives
- Default: **Classic Graphite** (dark).
- Bundled by the spec: **Midnight Modern**, **Copper**.
- **[proposal]** Minimum alternatives to design beyond the 3 bundled: (a) **Classic Graphite Light**; (b) **High Contrast** (pure black/white/yellow, 2 DIP borders); (c) optionally **VU Retro** (a Copper variant that puts analog needle gauges prominently in Classic) as a demo skin emphasizing the analog VU. Codex should prioritize (a) and (b).

### 8.6 Skin preview
App-generated thumbnail; comparative preview of the 3 layouts with fake data and states; DPI/density/language (pt-BR/EN) selector; preview of the main meter modes; contrast report; export as .vyntheme.

---

## 9. Accessibility and internationalization

### 9.1 Semantic tree and screen readers
(docs/design/03-accessibility-i18n.md)
- **All** controls have a stable name, role, value and state; sliders expose value + unit and accept keyboard ("Volume, 50 percent"; "Band 1 kilohertz, plus 3 decibels"). A custom theme does not remove semantic structure.
- The display (time/title/quality) is **semantic text**, not canvas only. Time is **not** announced per frame: only on track change/event or on query. Errors and job end: live announcements with rate limit. The playing row is announced in the playlist.
- Target tests: Narrator and NVDA (Windows), VoiceOver (macOS), Orca (Linux), TalkBack/Switch Access (Android).
- The visualizer/VU is a single accessible element with an option to disable (see 5.13).

### 9.2 Keyboard
Tab/Shift+Tab traverse predictable groups (Header → Player → Seek/Volume → EQ → Playlist); arrows move selection; Enter activates; Escape closes popup/dialog/full screen; Home/End navigate lists. Focus **does not vanish** when a panel is hidden. Full flow **without a mouse**: open → play → seek → queue → output → theme (TASK-E02-10). Visible focus (cyan 2 DIP ring, 2 gap) in all themes, including High Contrast.

### 9.3 Color, contrast, motion, scale
State and audio resolution never color only; high contrast and reduced motion prevail; font 100–200% (200% zoom without inaccessible actions); visualizer without aggressive flashing.

### 9.4 i18n
- **pt-BR and en** complete at first beta; EN fallback; keyed strings; pluralization ("1 track / 2 tracks" · "1 faixa / 2 faixas"); dates, numbers and decimal separator by locale (EQ respects decimal). Contracts use decimal point and UTC.
- pt-BR text is typically **20–40% longer** than EN (German/Russian more): buttons and labels need elastic width or truncation with tooltip; **no essential control may clip**. Short alternative labels for 24–32 DIP slots (e.g. "Shuffle"/"Embaralhar" → icon + tooltip).
- Example pairs to show in mockups: "Play now" / "Tocar agora"; "Add to queue" / "Adicionar à fila"; "Transparent mode active" / "Modo transparente ativo"; "Stop after this track" / "Parar após esta faixa"; "Device not responding" / "Dispositivo não responde".
- User data preserves Unicode: combining characters, emoji, **CJK**, **RTL** (render correctly from M2; fully mirrored layout is an M5 target — design the skin with logical start/end margins and alignments, not left/right). Search normalization is for comparison only.
- Fallback fonts for CJK/Arabic/Hebrew/emoji.
- Show in mockups: very long titles, CJK, RTL, emoji, accents.

### 9.5 Lyrics: accessibility and i18n (LYR-02, LYR-03, LYR-06, LYR-10, LYR-13, LYR-14)
(docs/library/02-tags-lyrics-tools.md#letras-componente-resumido-sob-o-vu-e-fontes; docs/design/03-accessibility-i18n.md#letras-acessibilidade-e-localização)
- Current line never by color alone (weight/marking too); High Contrast keeps the hierarchy by weight/marking; adjacent-line dimming validated after any transparency.
- Options menu: keyboard, clear labels, visible focus, focus returns to the trigger; no hover dependence. Verse changes are not announced automatically; reading on demand; action/error feedback separate. Source and state (LYR-13, LYR-14) also as text.
- Reduced motion: no upward transition. Enlarged text 100–200% and compact layouts: one visual line per position, no page horizontal scroll, full text reachable (LYR-03).
- i18n: keyed en + pt-BR strings (e.g. "View full lyrics" / "Ver letra completa"; pt-BR is typically 20–40% longer, so state/menu labels need elastic width); lyrics keep Unicode (CJK, RTL, emoji) with logical start/end alignment. Mockups should show a very long line, CJK and RTL in the slot.
- Status: *specified*; prototype checks are reported evidence from the external reference, not verified in this repository; native: nothing.

---

## 10. Expected Codex deliverables

### 10.1 Format
Design files (Figma-like exportable as PNG + SVG; or static HTML/SVG), organized in folders per theme. Each mockup at **1x and 2x**, with measurement annotations in DIP and tokens used. Naming: `<theme>/<variant>/<screen>_<state>_<dpi>.png`.

### 10.2 Screens/mockups to deliver (per bundled theme: Classic Graphite, Midnight Modern, Copper; dark and light [proposal]; plus High Contrast)
Reference resolutions: Classic 600×760 (plus minimum 480×660, and <560 DIP); Library 1280×800 (plus 960×640, and <1152 with drawer); Mini 420×112 (plus 640×112 [proposal]); Phone 400×880 dp (plus 360 dp); Tablet medium 720×1024 dp, expanded 1280×800 dp; Android landscape 880×400 dp.

1. **Classic Stack**: empty Idle; playing (FLAC 24/96, hi-res); paused; buffering; live radio; device error; transparent mode; with/without EQ; with/without cover; width <560; 200% font; full/empty playlist.
2. **Library Workspace**: album grid; track list; folders; search results; empty; indexing; offline; inspector open/drawer; collapsed sidebar.
3. **Mini**: playing; paused; mini meter; always-on-top active; error.
4. **Full-screen visualizer** (all modes in 5.13.3) with HUD visible/hidden.
5. **Meters — dedicated sheet**: each mode (VU, PPM, True-peak, RMS, LUFS, Spectrum log/linear, Oscilloscope, Spectrogram, Goniometer, Off) in stereo, mono and 5.1; states: playing, peak, **CLIP lit**, peak hold, paused, stopped, off, Reduced Motion (static with numbers), High Contrast; slots 28 DIP (Classic), Mini 64×24, now-playing 120×40, card 280×120, full screen; the **meter selection menu** and click feedback.
6. **EQ panel** (on/off/modified preset/locked by transparent) + **advanced EQ 960×600** + Android 10 bands.
7. **DSP / Preferences > DSP**, **Vitrola** (macro + advanced; states: off/on/auto-disabled/warming up/no album context) — **no turntable illustration**. Add: header entry "Áudio e efeitos" and the four shortcuts; the eight-family rail with search; a **component sheet** of knob (88×88 DIP, 270°), slider, switch, selector and clickable readout in all states; exact-editor error and off-step preserved value; the unified `[DEMO — no audio processing in this mode]` badge.
7a. **VU finish sheet**: the 12 approved VUs × 12 skins in the three presentations (Original / Adaptive metal / Integrated to panel), light and dark, SVGs unmodified.
7b. **Modular flow canvas**: nodes, audio vs CV wires and ports, selection, bypass, invalid-connection preview, inspector, library, minimap, palettes (light/dark), animation on/off and high-contrast states.
8. **Output**: quick selector, preferences, transparent-mode dialog with checklist, DAC-removed banner, exclusive busy.
9. **Audio chain/inspector** (source → output; with resample/dither; EQ active; transparent).
10. **Radio** (favorites, connecting, reconnecting, live with ICY, timeshift, recording, TLS/401 error), **Podcasts** (feeds, episodes, downloads, chapters), **Lyrics** (synced, auto-scroll paused).
11. **Preferences 880×680** — all 10 sections; **Shortcuts** with conflict; **Jobs 800×520**; **Details/tags 800×640** (mixed values); **Sleep timer/Scheduler**.
12. **Themes**: gallery, side-by-side preview, editor, validation error, safe mode; **Plugins**: list, detail/permissions, error.
13. **Dialogs and feedback**: toasts (with Undo), banners, confirmation dialogs (delete file, leave transparent, close to tray), typed errors (5.22), onboarding/first run, About/splash.
14. **Android**: Now Playing (portrait/landscape; tabs Queue/EQ/Lyrics/Info), sticky mini-player, bottom nav, Library (compact/medium/expanded), Radio, Podcasts, sheets (volume, item actions, output), EQ 10 bands with horizontal scroll, permissions (denied/explanation), **media notification** (expanded/collapsed), **widget [proposal]**, lock screen (metadata/cover reference only), adaptive icons.
15. **Tray/menu bar/taskbar**: light/dark icons, context menu.

### 10.3 Mandatory variations
- Themes: Classic Graphite, Midnight Modern, Copper (+ light [proposal], + High Contrast).
- DPI: 100/125/150/200% (Windows/Linux), 1x/2x (macOS), mdpi–xxxhdpi densities (Android).
- Density: Compact/Comfortable; touch mode.
- Languages: pt-BR and EN (and stress: long text/CJK/RTL).
- Font scale: 100% and 200%.
- Reduced Motion and High Contrast.

### 10.4 Components (design system)
Deliver a component sheet with all states (2.9): transport button (square), icon button, text button, toggle, chip, segmented (radio group), horizontal/vertical slider (seek, volume, EQ), knob (Vitrola, [proposal]), list row (normal/current/selected/unavailable/dragging), column header, tab, menu and submenu, popover, dialog, toast, banner, tooltip, text field (search/URL), select (dropdown), table, job progress bar, badge (HI-RES, LOSSY, TRANSPARENT, LIVE, RG, EQ), meter (each mode), time display, cover/placeholder, scrollbar, focus ring, divider, drawer/sheet, folder tree, album grid, theme/plugin card.

### 10.5 Assets
- **SVG icons** (paths, no external refs, `currentColor`, 16/20/24 grids): transport (previous, play, pause, stop, next, open), shuffle, repeat (3 states), volume/mute, A-B, EQ, DSP, Vitrola (a generic 24 dp "disc/tube" glyph only, **no illustration**), output/devices (speaker, headphones, USB, Bluetooth, HDMI), meter/visualizer (per mode), library, folders, playlists, radio, podcast, favorites, downloads, themes, plugins, search, filter, sort, trash, edit, add, remove, menu, close, minimize, maximize, pin, full screen, alert, error, success, offline, padlock, CD/CUE, REC, sleep, clock, lyrics, info, hamburger, overflow.
- Vector badges and seals; cover placeholders; skin frames/textures (SVG or PNG ≤ 4096 px, 20 MiB total per theme).
- Brand: lockups, symbol, monochrome per the guide (only use `assets/branding` assets; do not redraw the logo).
- Tray/menu bar/notification icons (monochrome).
- `tokens.json` per theme (token names from 2.2 + extras: `meter.needle`, `meter.scale`, `meter.clip`, `meter.hold`, `viz.palette.*`), with contrast table.

### 10.6 Measurement specs
For each mockup: grid, rectangles x/y/w/h in DIP, paddings, gaps, typography (family/size/weight/line), colors by token, radii, borders/bevel, icon sizes, hit targets, states and transitions (duration/easing). Annotate interaction area where it differs from the visual (e.g. Mini seek).

### 10.7 What NOT to deliver
- **No drawing of a turntable, platter, tonearm, stylus, spinning disc or tubes** for Vitrola (spec §3: controls only).
- No assets, sprites, layouts or fonts copied from Winamp, AIMP, Poweramp or commercial skins; nothing "imported" from legacy skins.
- No **video** screen, DVD, paid store/marketplace, commercial DRM, system-wide EQ, iOS, Android Auto (outside 1.0).
- No shaders/executable code inside the skin.
- No change to the brand (logo/symbol) beyond permitted use; no gradient/shadow/glow/metal on the logo master.
- No play button using the logo; no dependence on an installed font for the brand.
- No mandatory login/account screen; no blocking onboarding.
- No new functionality that contradicts the spec; proposals must be marked.

---

## 11. Final design acceptance checklist

**General**
- [ ] All screens in section 4 have mockups in the 3 bundled themes (+ High Contrast) and in the mandatory states (empty, loading, populated, partial error, offline, unsupported, permission denied, conflict, recovery).
- [ ] Every control in section 6 appears in all 2.9 states on the component sheet.
- [ ] Semantic tokens used; no stray RGB; contrast ≥ 4.5:1 (text) and ≥ 3:1 (indicators) measured on real pairs, including selected/disabled/hover.
- [ ] Targets ≥ 24×24 DIP (desktop), 44 (touch), 48 dp (Android); none shrunk to imitate legacy pixels.
- [ ] Geometry matches the spec: Classic 600×760/min 480×660; Library 1280×800/min 960×640; Mini 420×112; Prefs 880×680; Advanced EQ 960×600; Details 800×640; Jobs 800×520; Android 400×880/min 360.
- [ ] Layout <560 DIP, <1152 DIP, and Android 600/840 breakpoints designed.
- [ ] Nothing relies on color alone; icons have tooltip and name; focus visible in all themes.
- [ ] Reduced Motion, High Contrast and 200% font designed; pt-BR/EN, long, CJK, RTL and emoji text exercised in mockups.
- [ ] Stop and Mute never hidden; the meter off control always present.

**VU / visualizers (maintainer requirement)**
- [ ] Switching mechanism designed (click/Shift+click, button with menu, right-click, V shortcut, Android tap/long-press) and annotated.
- [ ] All modes (analog VU, PPM, true-peak, RMS, LUFS [proposal], spectrum, oscilloscope, spectrogram, goniometer, scenes, off) in stereo, mono and multichannel with channel labels.
- [ ] Ballistics annotated per mode; peak hold; CLIP lamp (color + text); clip clearing.
- [ ] Paused/stopped/buffering/minimized/Reduced Motion/High Contrast meter states.
- [ ] Where it appears: Classic (28 DIP), Library, Mini (64×24), full screen, Android; persistence (global + per layout) documented.

**Audio chain and honesty**
- [ ] HI-RES, LOSSY, TRANSPARENT, EXCLUSIVE, GAPLESS, RG, EQ, RESAMPLE badges designed; "bit-perfect verified" only in a verified context; leave-transparent dialogs.
- [ ] Source → output chain with negotiated output block.

**Vitrola**
- [ ] Main intensity control + advanced panel (vinyl and tube) + states; **no turntable drawing**.

**DSP access, controls and VU finish (visual)**
- [ ] "Áudio e efeitos" entry and the four shortcuts in the extended header; eight families drawn as navigation (not switches); Meters as its own area.
- [ ] Component sheet: knob (88×88 DIP reference, 270° arc), slider, switch, selector and clickable readout in hover/focus/drag/active/bypass/disabled/error/incompatible states, with a non-color, non-hover reason for unavailability.
- [ ] Exact-editor error state and preserved off-step value drawn; reduced motion / high contrast / enlarged text variants of the DSP panel.
- [ ] Unified `[DEMO — no audio processing in this mode]` badge used on every DSP/EQ/Vitrola/meter screen with simulated or absent processing; Vinyl/Tube configuration-only; no node styled as a capability it lacks.
- [ ] VU-INT: 12 SVGs untouched; three presentations per skin (default Integrated to panel); original stereo VU space not reduced by DSP controls.
- [ ] Modular flow canvas: audio solid vs CV dashed/square ports, selection/bypass/invalid preview, contrast in light/dark and with animation off.

**Platforms**
- [ ] Tray/menu bar/taskbar; SMTC/MPRIS/Now Playing referenced; Android notification, lock screen, sticky mini, tablet, landscape; widget marked as proposal.

**Skins**
- [ ] Design expressible with the nodes/constraints/bindings of 8.1, no code; runtime switching and preview designed; default skin + 2 alternatives.
- [ ] `tokens.json` per theme + contrast table delivered; sanitizable SVG icons; assets within budgets (20 MiB, 200 files, 4096 px).

**Brand**
- [ ] Lockup/symbol per the guide; brand never used as a button; below 24 DIP use the optical version; alt text "Vynamp".

**Proposals**
- [ ] Every item marked **[proposal]** is listed in the deliverables for the maintainer to approve/reject.

---

## Appendix A — Source map
docs/design/01-visual-system.md (tokens, typography, states, motion) · 02-layouts.md (geometry, screens, states) · 03-accessibility-i18n.md · 04-brand-identity.md · 04-mobile-tablet.md · docs/extensions/01-themes.md (.vyntheme) · 03-visualizations-integrations.md · 04-visualizer-contract.md (meters/visualizers) · docs/audio/01-engine.md (states, gapless, clipping) · 02-formats.md (formats, CUE, tags) · 03-dsp.md (EQ, chain, presets) · 04-outputs-hires.md (outputs, hi-res, transparent) · docs/library/01-library-playlists.md · 02-tags-lyrics-tools.md · docs/streaming/01-client.md · 02-podcasts-recording.md · docs/platforms/01-support-matrix.md · 02-android.md · 03-system-integration.md · docs/product/01-vision-scope.md · 04-user-journeys.md (shortcuts, journeys) · 06-feature-parity.md · 03-requirements.md · tasks: TASK-E02-02…E02-10, E03-05/06/09, E04-01…E04-09, E05-03/05/07/08, E06-01…E06-10, E07-07, E08-01/05/06, E13 (Android) · ..\vitrola\docs\superpowers\specs\2026-10-23-vitrola-design.md · DSP v1.4 (visual): docs/design/05-dsp-panel.md (§6.1, §8, §9) · docs/product/05-decisions.md (D-DSP-10..D-DSP-18).

## Appendix B — Quick glossary
DIP = density-independent pixel · dBFS = digital level relative to full scale · dBTP = true-peak · PPM = Peak Programme Meter · VU = Volume Unit · LUFS = Loudness Units Full Scale · RG = ReplayGain · CUE = virtual track sheet · ICY = Icecast/SHOUTcast radio metadata · HLS = adaptive streaming · SAF = Storage Access Framework (Android) · SMTC = System Media Transport Controls (Windows) · MPRIS = D-Bus player interface (Linux).
