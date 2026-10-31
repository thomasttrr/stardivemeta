// Liquid-glass close button (evidence dialog X).
// Layers: frost + saturation (CSS, every browser) -> true rim refraction
// (Chromium only: SVG feDisplacementMap fed by a map drawn here at the
// button's real size) -> pointer-reactive sheen (two custom properties).
// Safari/Firefox ignore url() in backdrop-filter, so they keep the frost.

const SIZE = 44;

/** Displacement map: neutral grey in the middle, pulling toward the centre
 * only in the rim band, like looking through the edge of a thick lens. */
function rimMap(): string {
  const px = SIZE * 2;
  const c = document.createElement('canvas');
  c.width = c.height = px;
  const ctx = c.getContext('2d');
  if (!ctx) return '';
  const img = ctx.createImageData(px, px);
  const half = px / 2;
  for (let y = 0; y < px; y++) {
    for (let x = 0; x < px; x++) {
      const dx = (x + 0.5 - half) / half;
      const dy = (y + 0.5 - half) / half;
      const r = Math.hypot(dx, dy);
      const t = Math.min(1, Math.max(0, (r - 0.55) / 0.45));
      const m = t * t * (3 - 2 * t) * Math.min(1, 1.4 - Math.max(0, r - 1) * 8);
      const k = r > 0 ? -m / r : 0; // sample from further in = edge magnified
      const i = (y * px + x) * 4;
      img.data[i] = 128 + dx * k * 127;
      img.data[i + 1] = 128 + dy * k * 127;
      img.data[i + 2] = 128;
      img.data[i + 3] = 255;
    }
  }
  ctx.putImageData(img, 0, 0);
  return c.toDataURL('image/png');
}

const chromium =
  (navigator as Navigator & { userAgentData?: { brands: { brand: string }[] } }).userAgentData?.brands.some(
    (b) => b.brand === 'Chromium',
  ) ?? false;
const coarse = matchMedia('(pointer: coarse)').matches;
const calm = matchMedia('(prefers-reduced-motion: reduce)').matches;

if (chromium && !coarse) {
  const url = rimMap();
  if (url) {
    document.querySelectorAll('#sdm-refract feImage').forEach((f) => f.setAttribute('href', url));
    document.documentElement.setAttribute('data-refract', '');
  }
}

if (!calm) {
  document.querySelectorAll<HTMLElement>('.ev-close').forEach((b) => {
    b.addEventListener('pointermove', (e) => {
      const r = b.getBoundingClientRect();
      b.style.setProperty('--mx', `${((e.clientX - r.left) / r.width) * 100}%`);
      b.style.setProperty('--my', `${((e.clientY - r.top) / r.height) * 100}%`);
    });
    b.addEventListener('pointerleave', () => {
      b.style.removeProperty('--mx');
      b.style.removeProperty('--my');
    });
  });
}
