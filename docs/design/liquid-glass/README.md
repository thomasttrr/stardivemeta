# Liquid Glass — tier list design (theme revamp, first pass)

Design source for the home tier board, from the Claude design canvas:
https://claude.ai/artifact/DERYEKZrVzPbtY1rGsj5G4 (private to the owner).

- `Main.dc.html` — the page at desktop width; Low/High/Max switching works.
- `Mobile.dc.html` — the same page at 390px.
- `canvas.json` — the canvas layout.

These are canvas files (`.dc.html`), not site code. Nothing in `site/`,
`crates/` or `data/` has changed yet. The board data inside `Main.dc.html`
is copied from `tierlist.json` (build of 2026-10-30) and is used for
display only.

## Direction

Dark Liquid Glass:
- Frosted, refractive panels sit over a slow-moving amber / teal / coral
  colour field.
- Tiers are glass cards with a tinted "lens" for the tier letter. Chips are
  lighter glass with a specular highlight.
- Low/High/Max is a floating glass dock. Its gold slider springs and squishes
  between levels.
- Inter (already bundled in the repo) is used throughout.

## Motion

All motion is CSS, and all of it is off under `prefers-reduced-motion`:
- The colour field drifts over 38–60 s.
- The hero and tier cards rise in on load.
- Chips re-flow with a short stagger when the level changes.
- Hovering a chip runs a sheen across it, and pressing it shrinks it slightly.
- The dock slider moves with a spring.

This goes past DESIGN.md's old ≤200 ms rule, by the owner's call
("free roam").

## Not yet verified

- The pa11y / WCAG contrast checks haven't been run, and the backdrop blur
  makes contrast depend on the colour field.
- It hasn't been rendered or screenshotted.
