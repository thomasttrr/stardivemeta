// The frost dial, read from CSS. `--frost` in styles/tokens.css is the one
// source of truth; tokens.css also derives `--frost-tint` from it (a
// registered <number>, so the computed value comes back resolved). The
// WebGL lens and the dock lens multiply their Balanced constants by these,
// exactly as the CSS glass does. See docs/DESIGN.md, "Frost dial".

export interface Frost {
  /** Blur / frost scale: Clear 0.3, Balanced 1, Frosted 2. */
  frost: number;
  /** Legibility tint scale, (1 + frost) / 2. */
  tint: number;
}

export function readFrost(): Frost {
  const cs = getComputedStyle(document.documentElement);
  const f = parseFloat(cs.getPropertyValue('--frost'));
  const frost = Number.isFinite(f) && f > 0 ? f : 1;
  const t = parseFloat(cs.getPropertyValue('--frost-tint'));
  return { frost, tint: Number.isFinite(t) && t > 0 ? t : (1 + frost) / 2 };
}
