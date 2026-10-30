// Investment-level dock (plan T3.2 + D-2), a React island.
// The boards are static HTML for all three levels; this only flips
// <html data-level> and remembers the choice (localStorage sdm.level).
// The liquid pill is a Motion shared-layout element: it springs between
// segments and squishes on arrival. Native radios keep keyboard and
// screen-reader behaviour. In Chromium the dock's backdrop also gets a
// real lens (an SVG displacement map sized to the dock).
import { useEffect, useRef, useState } from 'react';
import { readFrost } from './frost';
import { MotionConfig, motion } from 'motion/react';

type Level = 'low' | 'high' | 'max';
const LEVELS: { id: Level; label: string }[] = [
  { id: 'low', label: 'Low' },
  { id: 'high', label: 'High' },
  { id: 'max', label: 'Max' },
];
const KEY = 'sdm.level';
const FILTER_ID = 'sdm-dock-lens';

const isLevel = (v: unknown): v is Level => v === 'low' || v === 'high' || v === 'max';

/** Edge-lens displacement map for a rounded rect: R/G encode the x/y
 * shift (128 = none), strongest at the rim, flat in the middle. */
function lensMap(w: number, h: number, radius: number, bevel: number): string {
  const c = document.createElement('canvas');
  c.width = w;
  c.height = h;
  const ctx = c.getContext('2d');
  if (!ctx) return '';
  const img = ctx.createImageData(w, h);
  const hx = w / 2;
  const hy = h / 2;
  const r = Math.min(radius, hx, hy);
  const sd = (x: number, y: number) => {
    const qx = Math.abs(x - hx) - hx + r;
    const qy = Math.abs(y - hy) - hy + r;
    return Math.hypot(Math.max(qx, 0), Math.max(qy, 0)) + Math.min(Math.max(qx, qy), 0) - r;
  };
  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      const d = sd(x + 0.5, y + 0.5);
      const i = (y * w + x) * 4;
      let dx = 0;
      let dy = 0;
      if (d < 0 && -d < bevel) {
        const t = 1 + d / bevel;
        const m = t * t * (3 - 2 * t);
        const gx = sd(x + 1.5, y + 0.5) - sd(x - 0.5, y + 0.5);
        const gy = sd(x + 0.5, y + 1.5) - sd(x + 0.5, y - 0.5);
        const len = Math.hypot(gx, gy) || 1;
        dx = (-gx / len) * m;
        dy = (-gy / len) * m;
      }
      img.data[i] = 128 + dx * 127;
      img.data[i + 1] = 128 + dy * 127;
      img.data[i + 2] = 128;
      img.data[i + 3] = 255;
    }
  }
  ctx.putImageData(img, 0, 0);
  return c.toDataURL();
}

function isChromium(): boolean {
  const brands = (navigator as Navigator & { userAgentData?: { brands: { brand: string }[] } })
    .userAgentData?.brands;
  return !!brands?.some((b) => b.brand === 'Chromium');
}

export default function LevelDock() {
  const [level, setLevel] = useState<Level>('low');
  const [lens, setLens] = useState<{ w: number; h: number; href: string } | null>(null);
  const dock = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const l = document.documentElement.dataset.level;
    if (isLevel(l)) setLevel(l);
  }, []);

  useEffect(() => {
    const el = dock.current;
    if (!el || !isChromium()) return;
    const build = () => {
      const w = Math.round(el.offsetWidth);
      const h = Math.round(el.offsetHeight);
      if (w < 10 || h < 10) return;
      setLens({ w, h, href: lensMap(w, h, h / 2, Math.min(18, h * 0.4)) });
    };
    build();
    const ro = new ResizeObserver(build);
    ro.observe(el);
    return () => ro.disconnect();
  }, []);

  const pick = (l: Level) => {
    setLevel(l);
    document.documentElement.dataset.level = l;
    try {
      localStorage.setItem(KEY, l);
    } catch {
      /* private mode etc. — the attribute alone still works */
    }
  };

  return (
    <MotionConfig reducedMotion="user">
      <div className={`dock${lens ? ' lensed' : ''}`} data-glass="overlay" ref={dock}>
        {lens && (
          <svg className="dock-filter" width="0" height="0" aria-hidden="true" focusable="false">
            <filter
              id={FILTER_ID}
              x="0"
              y="0"
              width={lens.w}
              height={lens.h}
              filterUnits="userSpaceOnUse"
              colorInterpolationFilters="sRGB"
            >
              <feImage href={lens.href} x="0" y="0" width={lens.w} height={lens.h} result="map" preserveAspectRatio="none" />
              <feDisplacementMap in="SourceGraphic" in2="map" scale="34" xChannelSelector="R" yChannelSelector="G" result="bent" />
              <feGaussianBlur in="bent" stdDeviation={1.4 * readFrost().frost} />
            </filter>
          </svg>
        )}
        <fieldset className="segs">
          <legend className="sr-only">Investment level</legend>
          <span className="dlbl" aria-hidden="true">
            Investment
          </span>
          <span className="track">
            {LEVELS.map((l) => {
              const on = level === l.id;
              return (
                <label key={l.id} className={`seg${on ? ' on' : ''}`}>
                  <input
                    type="radio"
                    name="sdm-level"
                    value={l.id}
                    checked={on}
                    onChange={() => pick(l.id)}
                  />
                  {on && (
                    <motion.span
                      layoutId="sdm-level-pill"
                      className="slider"
                      transition={{ type: 'spring', stiffness: 460, damping: 32, mass: 0.9 }}
                    >
                      <motion.span
                        key={l.id}
                        className="slider-glass"
                        initial={{ scaleX: 1.16, scaleY: 0.86 }}
                        animate={{ scaleX: 1, scaleY: 1 }}
                        transition={{ type: 'spring', stiffness: 360, damping: 13 }}
                      />
                    </motion.span>
                  )}
                  <span className="seg-text">{l.label}</span>
                </label>
              );
            })}
          </span>
        </fieldset>
      </div>
    </MotionConfig>
  );
}
