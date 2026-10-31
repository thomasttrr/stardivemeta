# StarDiveMeta — Design ("Starfield")

## Status

The **Starfield** direction replaces "Liquid Glass" (merged 2026-10-30,
PR #2). Tom picked it on 2026-10-31 from three game-grounded mockups
(Starfield, Codex, Dawn), after rejecting the glass look as too blurry,
too generic and not like the game. Branch `claude/starfield-theme`.
Every token lives in `site/src/styles/tokens.css`. That file defines exactly
the tokens named below and no others. Components use the tokens and never
hard-code a colour, size or radius that has one.

## The direction

The site borrows the look of Mongil: Star Dive's own official site,
specifically its character section. Nothing is invented on top of it.

- **Ground:** a fixed night sky running from navy to violet
  (`#0d0a26 → #2e1c78`), a tiled scatter of star dots, and two faint
  orbital arcs. All static CSS and SVG.
- **Type:** Cardo 700 (the game's display serif) for the wordmark, page
  titles, headings and tier letters. Exo 2 (its English body face) for
  everything else. Both are self-hosted through `@fontsource`, latin
  subset only.
- **Motifs:**
  - The four-point sparkle: in the logo, in the rules, and five of them
    as an arc over S.
  - A hairline rule with a sparkle at its centre.
  - Violet pill buttons whose selected state is lavender with a caret.
  - Ringed medallion portraits.
- **Board:** tiers are open sections on the sky, as on the official
  character page. Each has a centred serif letter, a sparkle rule and
  "N characters" above the medallions. Medallion size steps down by
  tier group: S, then A, then the rest.
- **Surfaces:** flat night panels with a lavender hairline rim. Nothing
  is see-through and nothing blurs.
- **Glass close button (the one exception):** the X on the evidence dialog
  is a small liquid-glass disc. Frost (`--glass-blur`, `--glass-tint`,
  `--glass-edge`, `backdrop-filter` blur + saturate 180%) in every
  browser; Chromium on fine pointers adds rim refraction (an SVG
  `feDisplacementMap` from a map drawn in `scripts/glass-close.ts`, set
  on `html[data-refract]`), a specular top rim, a cursor-following sheen
  and a spring press. Reduced motion drops the sheen and spring. The
  filter is only ever applied to this one 44px control.
- **Single theme:** this is a night sky by definition, so the site stays
  dark (`color-scheme: dark`). A light "dawn" variant would be its own
  design pass.

## Stack and budgets

- **Astro** (static output). No islands, no React, no WebGL. Removed with
  the glass theme: `@astrojs/react`, `react`, `react-dom`, `three`,
  `@react-three/fiber` and `motion`.
- **Level tabs** (`LevelToggle.astro`):
  - Native radios in a fieldset, styled as violet pills.
  - A short inline script flips `<html data-level>` and stores
    `sdm.level`.
  - The bar sticks to the top of the screen in a full-width night band.
    A shadow-and-clip trick keeps the page from ever scrolling sideways.
  - The bar shows only with JS: the head script sets `html.js`.
- **The emitted JSON is still the contract.** Pages render emitted values
  only. Nothing client-side ranks anything.
- **Portraits** (`src/lib/portraits.ts`, `Medallion.astro`):
  - One 320×320 WebP face crop per character in
    `public/img/own/art/<id>.webp`. All are listed in
    `img/own/ALLOWLIST.txt` (gate G14).
  - Display only: an id with no file falls back to the name's first
    letter.
  - The images are lazy-loaded.
  - The per-character sources are in docs/BUILD-LOG.md.
- **No-JS:** every page is complete static HTML. Without JS the board
  shows Low, and the noscript line links the table's level anchors.
- **Budgets** (enforced by `site/scripts/check-dist.mjs`, G12):
  - Blocking JS: ≤ 6 KB gzip per page.
  - The island budget (≤ 420 KB gzip) stays as a ceiling. It currently
    reads 0.

## Tokens

### Colour — the night sky

| Token | Value | Use |
| --- | --- | --- |
| `--bg` | `#150f38` | page ground under the sky |
| `--night-0` … `--night-2`, `--night-mid` | `#0d0a26` `#1f1449` `#2e1c78` `#241866` | the sky gradient stops; `--night-1` is also the medallion gap ring |
| `--text` | `#f3f0ff` | body text |
| `--sub` | `#b9b0e2` | secondary text |
| `--mark` | 86% `--text` | label marks on chips |
| `--star-dot`, `--orbit` | white 62%, lavender 20% | the star dots and orbital arcs |

### Colour — surfaces

| Token | Use |
| --- | --- |
| `--panel-fill`, `--panel-rim`, `--panel-rim-strong`, `--panel-shadow` | `[data-panel]` surfaces: legend, dossier page, table, list rows, rules sections, code cards |
| `--panel-fill-strong`, `--sheet-fill` | the evidence dialog sheet |
| `--chip-fill`, `--chip-fill-hover`, `--chip-rim` | small fills: nav track, stat tiles, level chips, flat rows |
| `--hairline` | rules between rows |
| `--scrim` | the dialog backdrop |
| `--band` | the sticky level bar |
| `--rule-line` | the sparkle rule's hairlines |

### Colour — pills, medallions, accents

| Token | Value | Use |
| --- | --- | --- |
| `--violet`, `--violet-2`, `--pill`, `--pill-rim` | `#7b3cf2` → `#8e3ddc` | pill buttons (level tabs, page links, New tag). `--violet-2` is the official `#a24cf4` deepened so white text keeps 4.5:1 |
| `--lavender`, `--on-lavender` | `#ddd6ff` / `#1f1449` | selected tab, current nav item, current level chip |
| `--violet-ring`, `--med-fill`, `--med-halo`, `--med-shadow` | | the medallion ring stack |
| `--glow` | lavender 50% | text glow on serif headings |
| `--starlight` `--rose` `--aqua` `--periwinkle` `--ash` `--orchid` | `#ffd98a` `#ffb3c2` `#9fe6cf` `#a9c3ff` `#bdb6d6` `#e3d2ff` | accents and tier colours |
| `--link` | `#cdbfff` | links |
| `--callout-fill`, `--callout-rim` | rose 8% / 30% | the dossier "why" callout |
| `--success-rim` | aqua 34% | Active code cards |
| `--tier-s` … `--tier-unranked` | starlight, rose, aqua, periwinkle, ash, orchid, `--sub` | tier letters and table tier letters |

### Type

- **Families:** `--font-display` (Cardo), `--font-sans` (Exo 2) and
  `--font-mono`.
- **Sizes:**
  - Base: `--text-base` 15px at `--leading-base` 1.55.
  - Display: `--text-display` clamp(36–58px), `--text-h1` clamp(32–46px),
    `--text-h2` 21px, `--text-brand` 30px.
  - Tier letters: `--text-tier` 54px (S) and `--text-tier-sm` 34px.
  - Dossier: `--text-dossier-name`.
  - Chips: `--text-card-name` 14px and `--text-card-name-sm` 12px.
  - Small text: `--text-body-sm`, `--text-note`, `--text-pill`,
    `--text-kicker`, `--stat-value-size` and `--text-stat-key`.
- **Tracking and weights:** `--tracking-display` 0 (serif is never
  tightened), `--tracking-tight`, `--tracking-caps` 0.24em for uppercase
  kickers; weights 500, 600 and 700.

### Spacing, layout, radius

- **Layout:** `--content-max` 1080px and `--prose-max` 760px. The page
  gutter `--space-page-x` is 16px and also respects safe-area insets.
  On wide screens board pages (`.shell-wide`) widen to
  `--content-max-wide`: 1360px from 1100px, 1600px from 1700px.
  `--rail-w` 190px is the tier-heading rail on those screens.
- **Spacing:** `--space-section`, `--space-stack`, `--space-grid-gap`,
  `--space-panel` and `--space-panel-sm`.
- **Medallion diameter:** `--med-s` 128, `--med-a` 92, `--med-rest` 68,
  `--med-dossier` 84. On phones (≤ 620px) these become 116 / 84 / 62.
- **Chip width:** `--chip-w-s` 150, `--chip-w-a` 108, `--chip-w-rest` 82.
  On phones these become 142 / 100 / 76. That gives S two-up, A
  three-up and the rest four-up at 375–390px.
- **Desktop steps** (diameter / chip width for S, A, rest):
  - From 1100px: 152 / 112 / 80 and 172 / 128 / 90.
  - From 1700px: 168 / 124 / 96 and 188 / 140 / 112.
  - These are sized so an 11-character tier fits on one row beside the
    rail from about 1440px up.
- **Radius:** `--radius-sm` 8, `--radius-md` 14, `--radius-lg` 18,
  `--radius-xl` 24, `--radius-pill`.

### Motion

`--ease-out`, `--ease-spring`, `--duration-fast` 200ms, `--duration-base`
450ms, `--duration-slow` 700ms and `--stagger` 22ms. Sections rise in, and
chips settle in a staggered sequence. Medallions lift on hover. Everything
is off under `prefers-reduced-motion`.

## Label marks

A label is never shown by colour or shape alone. On chips the mark sits
next to the label word (or "1 list" for Speculation, or "Overall X" on mode
pages), and the chip's `aria-label` carries tier, label word, score and list
count. (`site/src/components/LabelMark.astro`)

| Label | Mark |
| --- | --- |
| Solid | filled disc |
| Contested | half-filled ring |
| Speculation | dashed ring (the medallion also gets a dashed lavender outline and "1 list") |
| Mode-split | two opposing arrows |
| Unranked | short dash |

## Components

- **Header:** centred, not sticky. The sparkle sits over the "StarDiveMeta"
  wordmark in Cardo, with a lavender glow. Below it is a pill track for
  the nav: the current page is a lavender pill (`aria-current`). On phones
  the track runs full width, scrolls sideways and centres the current
  item.
- **Home hero:** centred. The Cardo title has "tier list" on its own line
  in lavender, then a sparkle rule, the tagline and the source line.
  The level tabs follow.
- **Tier section** (`TierBoard` / `ModeBoard` + `TierLens`):
  - An sr-only `<h2>` names the tier.
  - The visible head is aria-hidden: the arc (S only), the tier letter
    in its tier colour with a soft glow, and the sparkle rule.
  - The count ("N characters") stays readable to screen readers.
  - Medallions sit in a centred, wrapping row.
  - **Wide screens (≥ 1100px):** the head moves into a left rail
    (`--rail-w`) beside its medallions, which start at the left of the
    row. Each tier becomes one row, and tiers are split by a hairline.
    Below 1100px the centred stack is unchanged.
- **Character chip:** medallion, name, then the label line (mark + word).
  On hover the medallion lifts and its ring brightens.
- **Medallion:** the portrait is a circle inside a ring stack copied from
  the official site: a 3px violet ring, a 3px night gap and a 1px lavender
  halo, with a soft drop shadow.
- **Level tabs:** three violet pills. The selected one is lavender with a
  caret pointing at the board. They sit in a sticky night band with a
  hairline and a soft fade below. The one-line Kaiden note sits under
  them.
- **Evidence dossier:**
  - Header: the 84px medallion beside a lavender "Character file" kicker,
    the Cardo name and a violet New pill.
  - Then the level summary, three stat tiles (the tier value in its tier
    colour) and the level chips, with the current one lavender.
  - A rose "why" callout.
  - Evidence rows split by hairlines, with the current Kaiden level in
    starlight.
  - Then criteria, not counted, modes and corrections.
- **Evidence dialog:** a violet sheet (`--sheet-fill`) with a strong
  lavender rim over a dark scrim. It is centred on wide screens and a
  bottom sheet on phones, with a 44px close button.
- **Lists:**
  - `.panel-list` / `.panel-row` (`data-panel`) for notes, history,
    corrections, data, banner and the beginner rise list.
  - `.panel-list.flat` for many small rows.
- **Pills:** `.pill-link` violet pills, centred, for the page links under
  the board.
- **Table:** one panel with uppercase headers and row hover. Tier letters
  are coloured by tier.
- **Codes:** a grid of panel cards. Code strings sit in a mono well, in
  aqua for Active codes.
- **Footer:**
  - A sparkle rule, then the footer nav.
  - The fan disclaimer, exactly once per page.
  - A credit line for the character art.

## Contrast (WCAG AA)

- **Ground:** text sits on the sky, whose lightest stop is `--night-2`
  `#2e1c78`, or on panels darker than that.
- **Text on `--night-2`:**
  - `--sub` 6.7:1 and `--link` 8.0:1.
  - Tier accents 7.0–10.0:1, worst case `--ash`.
- **Pills:** white text on the pill gradient is ≥ 4.86:1 at both ends.
  Navy text on lavender is 12.1:1.
- **Checked:** pa11y WCAG2AA reports 0 errors on /, /c/francis/, /codes/,
  /rules/ and /table/, and on every other page type in the local run.
- **Gradients:** pa11y cannot measure text over a gradient, so the pill
  ratios above were computed by hand. Any token change must recompute
  them and keep every text pair ≥ 4.5:1.
