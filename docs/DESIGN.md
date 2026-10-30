# StarDiveMeta — Design ("Liquid Glass")

## Status

The **Liquid Glass** direction replaces the CP1 baseline "Arcade Board"
(Direction C, 2026-10-29). It came from the theme-revamp pass on branch
`claude/theme-revamp` (2026-10-30, owner's brief + "free roam" go-ahead).
Every token lives in `site/src/styles/tokens.css`. That file defines exactly
the tokens named below and no others. Components use the tokens and never
hard-code a colour, size or radius that has one.

## The direction

The tier list sits on dark glass over a slow, deep-sea colour field.

- **Glass:** each panel (tier rack, legend, dossier, table, list rows) is a
  real lens. A WebGL shader refracts the field through the panel's live
  on-screen shape: strong bending at the rim, a slight chromatic split, a
  frosted body, a dark tint that keeps text legible, a specular rim lit from
  the top-left and from the pointer, and a soft drop shadow.
- **Hero:** a transmissive glass star (Three.js physical material) floats
  in the home hero.
- **Board surface:** chips show only the name and the label mark. Everything
  else is one tap away in the evidence sheet.
- **Ruled out:** gradients as decoration, glow effects and dashboard chrome.

## Stack and budgets

- **Astro** (static output, unchanged), **React 19** islands, **Three.js +
  React Three Fiber** for the glass layer, **Motion** for the dock springs.
- `src/islands/GlassScene.tsx` (`client:idle`): one fixed full-viewport
  canvas.
  - `glass-shader.ts` paints the field and lenses every
    `[data-glass]:not([data-glass="overlay"])` element. It reads up to 24
    rects per frame, culled to the viewport.
  - `<GlassStar>` renders at the `[data-star]` anchor (home only).
  - Decorative only. It never reads page data, and `aria-hidden` sits on its
    wrapper.
- `src/islands/LevelDock.tsx` (`client:only="react"`): the floating
  Low/High/Max dock.
  - It flips `<html data-level>` and stores `sdm.level`, exactly as the old
    `level.ts` did.
  - Native radios inside a fieldset.
  - The Motion shared-layout pill springs and squishes between segments.
  - In Chromium the dock's backdrop also gets an SVG displacement-map lens
    (`#sdm-dock-lens`).
- **The emitted JSON is still the contract.** Pages render emitted values
  only. Neither island does ranking math, and neither touches data.
- **Touch devices:** the WebGL scene does not mount on coarse pointers
  (phones/tablets). It repaints panel glass from live rects each frame,
  which trails iOS Safari's compositor scrolling and reads as bouncing;
  touch devices keep the CSS glass below, which is static and calm.
- **No-JS / no-WebGL:** every page is complete static HTML.
  - Until the first GL frame (`<html class="gl">`), and for good where WebGL
    is missing, panels use CSS glass: fill, rim and `backdrop-filter`.
  - CSS blobs stand in for the field.
  - Without JS the board shows Low, and the noscript line links the table's
    level anchors.
- **Budgets** (enforced by `site/scripts/check-dist.mjs`, G12):
  - Blocking JS (inline scripts + `<script src>`): ≤ 6 KB gzip per page.
    This rule is unchanged.
  - Island JS (every module reachable from each page's `astro-island`
    component/renderer URLs): ≤ 420 KB gzip per page. The largest page is
    ~360 KB, most of it Three.js. Islands are deferred, and the page never
    waits on them.
- **Build step:** `astro.config.mjs` moves Astro's one-line island style
  into `<head>` after the build, so dist HTML stays valid (html-validate
  `element-permitted-content`).

## Tokens

### Colour — ground, text, glass

| Token | Value | Use |
| --- | --- | --- |
| `--bg` | `#050d10` | page ground, WebGL clear colour |
| `--text` | `#f4f6f1` | primary text |
| `--sub` | `#b8c5c8` | secondary text |
| `--mark` | `rgb(244 246 241 / .82)` | label marks on chips |
| `--frost` | `0.3` (Clear) | the frost dial, see "Frost dial" below |
| `--frost-tint` | `(1 + --frost) / 2` | legibility tint scale (registered `<number>`) |
| `--glass-fill` | `rgb(18 30 34 / .58 × tint)` | CSS-glass panel fill (fallback) |
| `--glass-fill-strong` | `rgb(12 22 26 / .78 × tint, ≤ .92)` | overlays: header capsule, dialog |
| `--dock-fill` | `rgb(12 22 26 / .56 × tint)` | lensed dock fill (Chromium) |
| `--glass-sheen` | top-light linear gradient | glass body sheen |
| `--glass-rim` / `--glass-rim-strong` | white 15% / 24% | panel rims |
| `--glass-edge` | inset top light + bottom shade | specular edge |
| `--glass-shadow` | edge + 0 14px 34px drop | panel shadow |
| `--glass-blur` / `--glass-blur-strong` | blur 22px / 32px × frost + saturate | `backdrop-filter` |
| `--chip-fill` / `--chip-fill-hover` | white 6.5% / 11% × tint | chips, stat tiles, small rows |
| `--chip-rim` / `--chip-edge` | white 11% / inset top 14% | chip edge |
| `--hairline` | white 10% | dividers |
| `--scrim` | `rgb(2 6 8 / .55)` | dialog backdrop |
| `--tint-dark` | black 20% | dock track, code wells |

### Frost dial

One design-time number, `--frost` in `tokens.css`, sets how see-through
every glass surface is. It is not a user setting. `--frost` scales blur and
frost directly. `--frost-tint = (1 + frost) / 2` scales the darkening that
keeps text legible. Both paths read it:

- **CSS:** the panel, overlay, dock and chip fills, and the
  `backdrop-filter` blurs. The CSS path is what you see before the first
  WebGL frame and wherever WebGL is missing.
- **WebGL:** `islands/frost.ts` reads the two computed values when the
  scene mounts and on resize. The lens frost becomes `min(.62 × frost, 1)`
  and the tint becomes `min(.42 × tint, .9)`. The dock's SVG lens blur
  becomes `1.4 × frost`.

| Stop | `--frost` | tint | CSS blur / strong | panel fill | overlay fill | dock fill | chip fill | GL frost | GL tint | dock lens blur |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| **Clear** (shipped) | **0.3** | **0.65** | **6.6 / 9.6px** | **.377** | **.78 †** | **.56 †** | **.042** | **.186** | **.273** | **1.4 †** |
| Balanced | 1 | 1 | 22 / 32px | .58 | .78 | .56 | .065 | .62 | .42 | 1.4 |
| Frosted | 2 | 1.5 | 44 / 64px | .87 | .92 (cap) | .84 | .0975 | 1.0 (cap) | .63 | 2.8 |

Clear is the owner's pick (2026-10-30), locked — chosen from the three-stop
screenshots after Balanced shipped as the default. Balanced reproduces the
pre-dial look exactly: with motion frozen, home and /c/francis/ screenshots
at Balanced are byte-identical to the previous build on both the WebGL and
the CSS path.

† Chrome-floored values, see "Chrome floor" below.

**Chrome floor.** Floating chrome — the header capsule, the dock and the
evidence dialogs — never drops below Balanced, whatever the dial says:
`--frost-chrome = max(--frost, 1)` drives `--glass-fill-strong`,
`--glass-blur-strong`, `--dock-fill` and the dock lens (via
`readFrost().chrome`). Board panels and chips follow `--frost` exactly.
Reason: at Clear, page content scrolling under the near-transparent
header stayed sharp enough to slice across the nav pill, reading as
clipping. The board keeps the Clear look; the chrome keeps Balanced
strength so what passes underneath blurs out. Screenshots at each stop are in
`docs/design/screenshots/frost/`: `{stop}-*.png` is the WebGL path,
`css-{stop}-*.png` the CSS path. To lock a different stop, change the one
`--frost` line.

### Colour — accents, tiers, ambient

| Token | Value | Use |
| --- | --- | --- |
| `--gold` | `#ffcf5a` | brand accent, S tier, current level, focus ring |
| `--copper` | `#ff9e6e` | A tier, "why" callout |
| `--mint` | `#86e0b5` | B tier, links (`--link`), active codes, kicker |
| `--sky` | `#93c6ff` | C tier |
| `--ash` | `#b3bdc1` | D tier |
| `--cream` | `#ece4cf` | mode-split |
| `--link` | `var(--mint)` | links |
| `--on-accent` | `#17120a` | text on solid accent |
| `--gold-glass` / `--gold-rim` | gold 36→16% / 55% | dock pill, current-level chip, New tag |
| `--copper-glass` / `--copper-rim` | copper 10% / 34% | "why" callout |
| `--tier-s` … `--tier-d`, `--tier-mode-split`, `--tier-unranked` | the accents above, `--sub` | lenses, stat values, table tier letters |
| `--amb-amber` `--amb-teal` `--amb-coral` `--amb-moss` | `#b9791c` `#0e8a80` `#b24a39` `#2c7a5c` | CSS fallback field |

### Type

Font: **Inter** (OFL 1.1), bundled at `site/public/fonts/Inter-Variable.ttf`
(and `crates/mongil-render/fonts/` for the PNG). `--font-mono` is the
system monospace stack, used for codes only.

| Token | Value |
| --- | --- |
| `--text-base` / `--leading-base` | 15px / 1.5 |
| `--text-display` | clamp(40px, 7vw, 64px), home h1 |
| `--text-h1` | clamp(32px, 5.4vw, 46px), page titles |
| `--text-h2` | 20px |
| `--text-tagline` | clamp(16.5px, 2.2vw, 19px) |
| `--text-dossier-name` | clamp(28px, 5vw, 36px) |
| `--text-card-name` · `--text-body-sm` · `--text-note` · `--text-pill` · `--text-kicker` | 15 · 14 · 12.5 · 11.5 · 11.5px |
| `--text-lens` / `--text-lens-sm` | 38 / 26px tier letters |
| `--stat-value-size` / `--text-stat-key` | 22 / 11px |
| `--leading-tight` / `--leading-card` | 1.02 / 1.2 |
| `--tracking-display` / `--tracking-tight` / `--tracking-caps` | -0.045em / -0.02em / 0.12em |
| `--weight-medium` … `--weight-extrabold` | 500, 600, 700, 800 |

### Spacing, layout, radius

| Token | Value |
| --- | --- |
| `--content-max` / `--prose-max` | 1080px boards and tables / 760px reading pages |
| `--space-page-x` / `--space-page-top` | 16 / 12px |
| `--space-section` / `--space-stack` / `--space-grid-gap` | 28 / 14 / 8px |
| `--space-panel` / `--space-panel-sm` | 22 / 14px |
| `--card-min-width` / `--card-min-height` | 176 / 50px (≥ 44px touch) |
| `--space-card-x` / `--space-card-y` | 16 / 8px |
| `--lens-size` / `--lens-size-sm` / `--rail-width` | 72 / 48 / 84px |
| `--dock-clearance` | 120px footer padding under the dock |
| `--radius-sm` / `-md` / `-lg` / `-xl` / `-pill` | 10 / 17 / 22 / 28 / 999px |

### Motion

| Token | Value |
| --- | --- |
| `--ease-out` | cubic-bezier(.22, 1, .36, 1) |
| `--ease-spring` | cubic-bezier(.34, 1.56, .64, 1) |
| `--duration-fast` / `-base` / `-slow` | 200 / 450 / 700ms |
| `--stagger` | 22ms per chip |

The old ≤ 200 ms cap is lifted by the owner's revamp brief.

- **What moves:**
  - The page rises in on load.
  - Chips settle in with a stagger each time a level board appears.
    Display toggling restarts the CSS animation.
  - Pressed chips and pills spring.
  - Hovering a chip runs a pointer-tracking specular glint.
  - The dock springs up on load, and its pill springs and squishes between
    levels.
  - The evidence dialog springs in (a bottom sheet on phones).
  - The WebGL field drifts, and its caustics and plankton sparks move.
- **Reduced motion:** everything is off under `prefers-reduced-motion`. The
  CSS durations collapse, Motion honours the user setting, and the WebGL
  clock freezes. Frames then render only on scroll, resize or pointer.

## Label marks

A label is never shown by colour or shape alone. On chips the mark is
`aria-hidden`, and the chip's `aria-label` carries tier, label word, score
and list count. Elsewhere the mark sits next to the word.
(`site/src/components/LabelMark.astro`)

| Label | Mark |
| --- | --- |
| Solid | filled disc |
| Contested | half-filled ring |
| Speculation | dashed ring (the chip also gets a dashed rim and "1 list") |
| Mode-split | two opposing arrows |
| Unranked | short dash |

## Components

- **Header:** a sticky glass capsule (`data-glass="overlay"`) holding the
  logo (gold four-point star mark) and the nav. The current page gets a
  raised glass pill (`aria-current`). On phones the capsule wraps, the nav
  scrolls sideways, and the current item is centred.
- **Tier rack:**
  - A glass panel with a radial tint in the tier colour at the top-left.
  - A tinted glass lens holds the tier letter, with the count beneath (an
    sr-only "characters" follows the count).
  - The chip grid is `auto-fill minmax(--card-min-width)`. On phones it is 2
    columns, names longer than 14 characters span both, and the lens moves
    into a header row.
- **Character chip:** name + mark only. Light chip glass, a tier-tinted rim
  on hover and a pointer glint. Mode pages add the small "Overall {tier} ·
  {label}" line.
- **Level dock:** a fixed bottom-centre capsule (full width on phones) with
  the "Investment" caption, a gold glass pill and native radios. The
  one-line Kaiden note stays in the page flow.
- **Evidence dossier:**
  - Mint "Character file" kicker, display-weight name, and a gold-glass New
    tag.
  - Level summary, then three stat tiles; the tier value is in its tier
    colour.
  - Level chips: the current one is gold glass.
  - Copper "why" callout.
  - Evidence rows split by hairlines, with the current Kaiden level in
    gold.
  - Criteria, Not counted, modes, corrections.
- **Evidence dialog:** a strong glass sheet over a blurred scrim. Centred on
  wide screens, a bottom sheet on phones. 44px close button.
- **Lists:** `.glass-list` / `.glass-row` (`data-glass`) for notes, history,
  corrections, data, banner and the beginner rise list. `.glass-list.flat`
  (chip glass, not lensed) for many small rows, such as snapshot changes.
- **Pills:** `.pill-link` glass pills for the page links under the board.
- **Table:** one lensed glass panel. Uppercase headers, row hover, and tier
  letters coloured by tier.
- **Codes:** a grid of glass cards. Code strings sit in a dark mono well,
  in mint for Active codes. The Expired disclosure is a glass pill.
- **Footer:** hairline, footer nav, and the fan disclaimer exactly once per
  page.

## Contrast (WCAG AA)

- **Glass:** text sits on glass that the shader darkens (42% tint toward
  `#081011`) over a field whose brightest point stays dark. The CSS fallback
  fill is 58% `#121e22` over blurred blobs at ≤ 55% opacity.
- **Worst case:** `--sub` (#b8c5c8) over fallback glass above the amber blob
  is about 7:1. Directly over the amber blob with no glass (hero) it is about
  4.8:1.
- **Accents:** gold and the tier accents are only used at ≥ 15px bold or as
  large lens letters.
- **Checked:** pa11y WCAG2AA reports 0 errors on /, /c/francis/, /codes/,
  /rules/ and /table/, and on every other page type in the local run. Any
  token change must re-run pa11y and keep every text pair ≥ 4.5:1.
