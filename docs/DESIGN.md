# StarDiveMeta — Design (Direction C: "Arcade Board")

## Status

Direction **C — "Arcade Board"** is locked as the **build baseline** for X's
build (CP1 pick, Tom, 2026-10-29). Tom plans a future Claude design
revamp/improvement run, so all design tokens stay centralized in
`site/src/styles/tokens.css` to make that revamp a theme-level change; this
file is expected to be revised by that pass.

## The direction

A card / board-game direction on a dark board, speaking the players'
language: each tier is a **rack** — a chunky coloured badge rail on the left
plus a grid of character card tiles — and the evidence view is a
**"character file" dossier card**. Flat CSS only, no game art. Extracted
from the CP1 mock `site/src/pages/_design/c.astro` (deleted after the pick,
per plan T1.8); every value below is the mock's actual value.

## Tokens

All tokens live in `site/src/styles/tokens.css` as CSS custom properties
with exactly these names. No component may hard-code a colour, size or
radius that has a token.

### Colour — surfaces and text

| Token | Value | Use |
| --- | --- | --- |
| `--bg` | `#10131a` | page background (the board) |
| `--panel` | `#171c28` | panel surface: toggle track, legend box, dossier card |
| `--panel-2` | `#1f2637` | raised surface: character cards, dossier stat blocks |
| `--line` | `#2c3549` | all borders / hairlines |
| `--text` | `#f2f4f8` | primary text |
| `--sub` | `#aeb8c9` | secondary text (notes, dates, pill scores) |
| `--pill-bg` | `#0d1017` | label pill background |
| `--why-bg` | `#241a26` | dossier "why" callout background |
| `--why-line` | `#4a2c3f` | dossier "why" callout border |

### Colour — palette accents and tier rails

| Token | Value | Use |
| --- | --- | --- |
| `--gold` | `#ffd166` | accent (tagline), current-level chip |
| `--pink` | `#ef6aa5` | contested accent (pill border, "why" lead-in) |
| `--teal` | `#3fd0c9` | logo accent, active toggle segment, dossier kicker |
| `--violet` | `#9d8cff` | — (tier C rail) |
| `--grey` | `#8d99ae` | — (tier D rail) |
| `--green` | `#7bd88f` | — (mode-split rail) |
| `--tier-s` | `var(--gold)` | S rail |
| `--tier-a` | `var(--pink)` | A rail |
| `--tier-b` | `var(--teal)` | B rail |
| `--tier-c` | `var(--violet)` | C rail |
| `--tier-d` | `var(--grey)` | D rail |
| `--tier-mode-split` | `var(--green)` | mode-split rail |
| `--on-tier` | `#12141c` | text on any tier rail (letter + count) |
| `--on-teal` | `#0b2b29` | text on the active toggle segment |
| `--on-gold` | `#221a02` | text on the current-level chip |

### Type

Font family token `--font-sans`: **Inter** (see Font below), falling back
to `ui-sans-serif, system-ui, -apple-system, "Segoe UI", sans-serif`.

| Token | Value | Use |
| --- | --- | --- |
| `--text-base` | `15px` | body text (line-height `--leading-base`) |
| `--leading-base` | `1.45` | body line-height |
| `--leading-tight` | `1.15` | h1 line-height |
| `--leading-card` | `1.25` | card name line-height |
| `--text-h1` | `27px` | page title, weight `--weight-extrabold`, letter-spacing `-0.01em` |
| `--text-h2` | `21px` | section heading, weight `--weight-extrabold` |
| `--text-tagline` | `16.5px` | tagline under h1, weight `--weight-semibold`, colour `--gold` |
| `--text-dossier-name` | `24px` | dossier character name, weight `--weight-extrabold` |
| `--text-card-name` | `14.5px` | character card name, weight `--weight-bold` |
| `--text-body-sm` | `13.5px` | intro line, evidence rows |
| `--text-note` | `12.5px` | toggle note, legend, criteria, stamp |
| `--text-pill` | `11px` | label pill text, weight `--weight-bold` |
| `--text-badge-count` | `10.5px` | rail count ("4 chars"), weight `--weight-bold` |
| `--badge-tier-size` | `24px` | rail tier letter, weight `--weight-black` |
| `--badge-tier-size-split` | `19px` | rail glyph when the tier is mode-split (⇄) |
| `--stat-value-size` | `16px` | dossier stat value, weight `--weight-extrabold` |
| `--text-stat-key` | `10.5px` | dossier stat key, uppercase, letter-spacing `0.08em`, weight `--weight-bold`, colour `--sub` |
| `--text-kicker` | `11px` | dossier kicker ("Character file"), uppercase, letter-spacing `0.14em`, weight `--weight-extrabold`, colour `--teal` |
| `--text-section-label` | `12px` | dossier sub-headings, uppercase, letter-spacing `0.1em`, colour `--sub` |
| `--weight-semibold` | `600` | |
| `--weight-bold` | `700` | |
| `--weight-extrabold` | `800` | |
| `--weight-black` | `900` | |

### Spacing and layout

| Token | Value | Use |
| --- | --- | --- |
| `--content-max` | `680px` | page column max-width, centred |
| `--space-page-x` | `14px` | page side padding |
| `--space-page-top` | `18px` | page top padding |
| `--space-page-bottom` | `44px` | page bottom padding |
| `--space-rack-gap` | `10px` | gap between rail and card grid; gap between dossier stat blocks is `--space-grid-gap` |
| `--space-grid-gap` | `8px` | card grid gap |
| `--space-section` | `22px` | margin above each rack |
| `--badge-width` | `52px` | tier rail fixed width (`flex: 0 0`) |
| `--card-min-width` | `118px` | card grid column minimum (`repeat(auto-fill, minmax(..., 1fr))`) |
| `--card-min-height` | `64px` | character card minimum height |
| `--space-card-x` | `11px` | card horizontal padding |
| `--space-card-y` | `10px` | card vertical padding (bottom is 9px in the mock; token rounds to one value) |
| `--space-panel` | `16px` | dossier padding |
| `--space-legend-x` | `14px` | legend horizontal padding |
| `--space-legend-y` | `12px` | legend vertical padding |

### Radius

| Token | Value | Use |
| --- | --- | --- |
| `--radius-sm` | `8px` | toggle segments |
| `--radius-md` | `10px` | dossier stat blocks, "why" callout |
| `--radius-lg` | `12px` | tier rails, character cards, toggle track, legend |
| `--radius-xl` | `16px` | dossier card |
| `--radius-pill` | `999px` | label pills, level chips, tag chips |

### Motion

| Token | Value | Rule |
| --- | --- | --- |
| `--duration-max` | `200ms` | no transition/animation may exceed this |

Motion rules (plan T1.8): CSS only — no JS-driven animation. Transitions
and animations are disabled entirely under
`@media (prefers-reduced-motion: reduce)` (enforced in
`site/src/styles/global.css`).

## Label glyphs

Labels are **never shown by colour alone**: every label is glyph + word
(+ score where applicable), in a pill on the character card and in the
legend.

| Label | Glyph | Word |
| --- | --- | --- |
| Solid | `●` | Solid |
| Contested | `◐` | Contested |
| Speculation | `○` | Speculation |
| Mode-split | `⇄` | Mode-split |
| Unranked | `—` | Unranked |

## Component rules (from the mock)

- **Tier rack:** flex row, gap `--space-rack-gap`. Left rail: fixed
  `--badge-width`, `--radius-lg`, background = the tier token, centred
  tier letter (`--badge-tier-size`, `--weight-black`, colour `--on-tier`)
  over the character count (`--text-badge-count`). Mode-split rail shows
  `⇄` at `--badge-tier-size-split` instead of a letter.
- **Character card:** `--panel-2` background, 1px `--line` border,
  `--radius-lg`, name then label pill, stacked with a 5px gap.
- **Label pill:** inline-flex, `--pill-bg` background, 1px `--line`
  border, `--radius-pill`, padding 2.5px 8px, `--text-pill`; contents are
  glyph (aria-hidden) + label word + score (score in `--sub`,
  `--weight-semibold`). Contested cards: pill border is `--pink`.
  Speculation cards: card border is dashed, and the pill adds "1 list".
- **Investment toggle:** pill track on `--panel` with 1px `--line`
  border, `--radius-lg`, 5px inner padding; segments `--radius-sm`,
  6px 16px padding; the active segment is `--teal` with `--on-teal` text.
- **Legend:** `--panel` box, 1px `--line` border, `--radius-lg`,
  two-column grid (one column under 480px), `--text-note` in `--sub`
  with the label word in `--text`.
- **Evidence dossier ("character file"):** `--panel` card, 1px `--line`
  border, `--radius-xl`, padding `--space-panel`. Structure, in order:
  1. kicker "Character file" (`--text-kicker`)
  2. character name (`--text-dossier-name`)
  3. stat blocks row: 3 equal columns, `--panel-2` blocks at
     `--radius-md` — Tier (with label word + glyph), Score (of 4),
     Lists · gap
  4. level chips row: one pill per investment level; the current level
     is `--gold` with `--on-gold` text and `--weight-extrabold`
  5. "why" callout: `--why-bg` background, 1px `--why-line` border,
     `--radius-md`; lead-in ("Why Contested:") in `--pink`
  6. "What each list says" rows: source name (fixed 92px, wraps to full
     width under 480px), placement text with note/reason beneath in
     `--sub`, converted number (`--weight-extrabold`), list date in
     `--sub`; rows separated by 1px `--line` top borders
  7. "Criteria notes" paragraphs (`--sub` with source name in `--text`)
  8. "Not counted (shown for context)" rows, same row layout, number
     shown as `—`
  9. Modes line

## Contrast (WCAG AA)

All text/background pairs in the direction were computed and pass WCAG AA
for normal text (≥ 4.5:1). Weakest pairs: `--pink` on `--why-bg` 5.82,
`--on-tier` on `--pink` 6.37, on `--grey` 6.39, on `--violet` 6.68.
Primary pairs are far above: `--text` on `--bg` 16.88, `--sub` on `--bg`
9.29, `--gold` on `--bg` 12.89, `--teal` on `--panel` 8.98. Any future
token change must re-check the affected pairs and keep every pair ≥ 4.5:1.

## Font

**Inter**, designed by Rasmus Andersson — SIL Open Font License, Version
1.1. The mock used a system sans stack; Inter is the bundled OFL family
closest in spirit (clean humanist sans with true 800/900 weights, which
this direction uses heavily).

- Bundled file (variable font, weight axis 100–900):
  `crates/mongil-render/fonts/Inter-Variable.ttf`, with the licence at
  `crates/mongil-render/fonts/OFL.txt` — this is the file the PNG
  renderer (T3.9) loads.
- Web copy: `site/public/fonts/Inter-Variable.ttf`, loaded via
  `@font-face` in `site/src/styles/global.css`.
- `--font-sans` stack: `"Inter", ui-sans-serif, system-ui, -apple-system,
  "Segoe UI", sans-serif`.
