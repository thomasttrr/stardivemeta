# StarDiveMeta — Design ("Starfield")

## Status

The **Starfield** direction replaced "Liquid Glass" (merged
2026-10-30, PR #2). Tom picked Starfield on 2026-10-31 from three
game-grounded mockups (Starfield, Codex, Dawn), after rejecting the
glass look as too blurry, too generic and not like the game.

On 2026-11-24 the site's surface was rebuilt as a clean editorial
guide: one tier board, a short reason per character, codes and banner
pages. Everything from the consensus era — source names, the
Solid / Contested / Speculation labels, the investment (Low / High /
Max) toggle, evidence dossiers and dialogs, stat tiles, and the
method / notes / history / mode / table pages — was removed from the
public site. The Rust engine still computes and emits the data
underneath; pages render emitted values only, and nothing client-side
ranks anything.

Every token lives in `site/src/styles/tokens.css`. Components use the
tokens and never hard-code a colour, size or radius that has one.

## The direction

The site borrows the look of Mongil: Star Dive's own official site,
specifically its character section. Nothing is invented on top of it.

- **Ground:** a fixed night sky running from navy to violet
  (`#0d0a26 → #2e1c78`), a tiled scatter of star dots, and two faint
  orbital arcs. All static CSS and SVG.
- **Type:** Cardo 700 (the game's display serif) for the wordmark,
  page titles, headings and tier letters. Exo 2 (its English body
  face) for everything else. Both are self-hosted through
  `@fontsource`, latin subset only.
- **Motifs:**
  - The four-point sparkle: in the logo, in the rules, and five of
    them as an arc over S.
  - A hairline rule with a sparkle at its centre.
  - Violet pill buttons whose selected state is lavender.
  - Ringed medallion portraits.
- **Board:** tiers are open sections ("racks") on the sky, as on the
  official character page. Each rack has a tinted lens heading with
  the centred serif tier letter, a sparkle rule and "N characters",
  then the chip grid. Empty racks hide.
- **Surfaces:** flat night panels with a lavender hairline rim.
  Nothing is see-through and nothing blurs.
- **Single theme:** this is a night sky by definition, so the site
  stays dark (`color-scheme: dark`). A light "dawn" variant would be
  its own design pass.

## Pages and components

- **Header:** centred, not sticky. The sparkle sits over the
  "StarDiveMeta" wordmark in Cardo, with a lavender glow. Below it is
  a pill track for the nav (Tier list, Beginner, Codes, New
  characters): the current page is a lavender pill (`aria-current`).
  On phones the track runs full width, scrolls sideways and centres
  the current item.
- **Home:** a centred hero (Cardo title with "tier list" on its own
  line in lavender, a sparkle rule, the tagline "Who's worth building
  — and why."), then the one tier board, an "Updated <date> · Game
  version <version>" stamp, and pill links to Beginner, Codes and the
  downloadable board PNG.
- **Tier board** (`TierBoard` / `TierLens` + `CharacterChip`): one
  rack per tier — S, A, B, C, D, and "Not ranked yet". A chip is only
  the medallion portrait and the name; the tier comes from the rack
  it sits in.
- **Character page** (`/c/<id>/`): the medallion, the name, a tier
  pill ("Tier X", or "Not ranked yet"), a "New this version" tag when
  the character is new, the short reason ("why", our own words from
  `data/roster.toml`), and the release date. Nothing else.
- **Beginner page:** "Strong from the start", "Get better with
  investment", "Worth a second look", and a common-questions section.
  Every pick comes from the same home-page tier list.
- **Codes page:** Active, Unconfirmed and recently Expired sections,
  each showing the code, the reward and the dates, with a "Last
  checked" stamp. No source names.
- **Banner page ("New characters"):** the current banner, what's
  coming next, the newest characters with their tiers, and the debut
  history. Dates only — no verdicts.
- **About page:** what the site is, in plain words. No ads, no
  accounts.
- **Footer:** a sparkle rule, the About link, and the fan disclaimer,
  exactly once per page: "Fan-made site. Not an official MONGIL:
  STAR DIVE or Netmarble site. Game names and terms belong to their
  owners. Character portraits are official MONGIL: STAR DIVE art
  © Netmarble, used non-commercially."

## Stack and budgets

- **Astro** (static output, v7). No islands, no React, no WebGL.
  Removed with the glass theme: `@astrojs/react`, `react`,
  `react-dom`, `three`, `@react-three/fiber` and `motion`.
- **Portraits** (`src/lib/portraits.ts`, `Medallion.astro`): one
  320×320 WebP face crop per character in `public/img/own/art/<id>.webp`,
  all listed in `img/own/ALLOWLIST.txt` (gate G14). An id with no file
  falls back to the name's first letter. Images are lazy-loaded.
- **No-JS:** every page is complete static HTML.
- **Budgets** are enforced by `site/scripts/check-dist.mjs` (G12).

## Tokens

### Colour — the night sky

| Token | Value | Use |
| --- | --- | --- |
| `--bg` | `#150f38` | page ground under the sky |
| `--night-0` … `--night-2`, `--night-mid` | `#0d0a26` `#1f1449` `#2e1c78` `#241866` | the sky gradient stops; `--night-1` is also the medallion gap ring |
| `--text` | `#f3f0ff` | body text |
| `--sub` | `#b9b0e2` | secondary text |
| `--star-dot`, `--orbit` | white 62%, lavender 20% | the star dots and orbital arcs |

### Colour — surfaces

| Token | Use |
| --- | --- |
| `--panel-fill`, `--panel-rim`, `--panel-rim-strong`, `--panel-shadow` | `[data-panel]` surfaces: panels, list rows, code cards |
| `--chip-fill`, `--chip-fill-hover`, `--chip-rim` | small fills: nav track, flat rows |
| `--hairline` | rules between rows |
| `--rule-line` | the sparkle rule's hairlines |

### Colour — pills, medallions, accents

| Token | Value | Use |
| --- | --- | --- |
| `--violet`, `--violet-2`, `--pill`, `--pill-rim` | `#7b3cf2` → `#8e3ddc` | pill buttons (nav, page links, tier pill). `--violet-2` is the official `#a24cf4` deepened so white text keeps 4.5:1 |
| `--lavender`, `--on-lavender` | `#ddd6ff` / `#1f1449` | current nav item |
| `--violet-ring`, `--med-fill`, `--med-halo`, `--med-shadow` | | the medallion ring stack |
| `--glow` | lavender 50% | text glow on serif headings |
| `--starlight` `--rose` `--aqua` `--periwinkle` `--ash` `--orchid` | `#ffd98a` `#ffb3c2` `#9fe6cf` `#a9c3ff` `#bdb6d6` `#e3d2ff` | accents and tier colours |
| `--link` | `#cdbfff` | links |
| `--success-rim` | aqua 34% | Active code cards |
| `--tier-s` … `--tier-unranked` | starlight, rose, aqua, periwinkle, ash, orchid, `--sub` | tier letters |

### Type

- **Families:** `--font-display` (Cardo), `--font-sans` (Exo 2) and
  `--font-mono`.
- **Sizes:**
  - Base: `--text-base` 15px at `--leading-base` 1.55.
  - Display: `--text-display` clamp(36–58px), `--text-h1` clamp(32–46px),
    `--text-h2` 21px, `--text-brand` 30px.
  - Tier letters: `--text-tier` 54px (S) and `--text-tier-sm` 34px.
  - Chips: `--text-card-name` 14px and `--text-card-name-sm` 12px.
  - Small text: `--text-body-sm`, `--text-note`, `--text-pill`,
    `--text-kicker`.
- **Tracking and weights:** `--tracking-display` 0 (serif is never
  tightened), `--tracking-tight`, `--tracking-caps` 0.24em for uppercase
  kickers; weights 500, 600 and 700.

### Spacing, layout, radius

- **Layout:** `--content-max` 1080px and `--prose-max` 760px. The page
  gutter `--space-page-x` is 16px and also respects safe-area insets.
  On wide screens board pages (`.shell-wide`) widen to
  `--content-max-wide`: 1360px from 1100px, 1600px from 1700px.
- **Spacing:** `--space-section`, `--space-stack`, `--space-grid-gap`,
  `--space-panel` and `--space-panel-sm`.
- **Medallion diameter:** `--med-s` 128, `--med-a` 92, `--med-rest` 68.
  On phones (≤ 620px) these become 116 / 84 / 62.
- **Radius:** `--radius-sm` 8, `--radius-md` 14, `--radius-lg` 18,
  `--radius-xl` 24, `--radius-pill`.

### Motion

`--ease-out`, `--ease-spring`, `--duration-fast` 200ms, `--duration-base`
450ms, `--duration-slow` 700ms and `--stagger` 22ms. Sections rise in,
and chips settle in a staggered sequence. Medallions lift on hover.
Everything is off under `prefers-reduced-motion`.

## Contrast (WCAG AA)

- **Ground:** text sits on the sky, whose lightest stop is `--night-2`
  `#2e1c78`, or on panels darker than that.
- **Text on `--night-2`:** `--sub` 6.7:1 and `--link` 8.0:1. Tier
  accents 7.0–10.0:1, worst case `--ash`.
- **Pills:** white text on the pill gradient is ≥ 4.86:1 at both ends.
  Navy text on lavender is 12.1:1.
- **Gradients:** pa11y cannot measure text over a gradient, so the pill
  ratios above were computed by hand. Any token change must recompute
  them and keep every text pair ≥ 4.5:1.
