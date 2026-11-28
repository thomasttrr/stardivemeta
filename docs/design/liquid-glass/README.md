# Liquid Glass — design canvas source

These are the canvas files for the first Liquid Glass mock of the home tier
board, from the Claude design canvas:
https://claude.ai/artifact/DERYEKZrVzPbtY1rGsj5G4 (private to the owner).

- `Main.dc.html` is the page at desktop width.
- `Mobile.dc.html` is the same page at 390px.
- `canvas.json` is the canvas layout.

This mock was a starting point only. The Liquid Glass direction was
**rejected** (Tom's call, 2026-10-31) and replaced by the Starfield
direction, which is what the site implements: a static Astro build with
no React islands, no Three.js, and no Motion — those packages were
removed with the glass theme. See `docs/DESIGN.md` for the design the
site actually uses. Before/after screenshots are in
`docs/design/screenshots/`.
