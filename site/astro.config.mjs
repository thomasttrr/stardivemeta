// @ts-check
import { defineConfig } from 'astro/config';
import sitemap from '@astrojs/sitemap';
import tailwindcss from '@tailwindcss/vite';
import { readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';

// Astro writes its one-line island style (`astro-island{display:contents}`)
// in front of the first island, i.e. inside <body>, where <style> is not
// valid HTML (html-validate: element-permitted-content). Move it into
// <head> after the build; the rule is byte-identical, only its place moves.
const ISLAND_STYLE = '<style>astro-island,astro-slot,astro-static-slot{display:contents}</style>';
/** @type {import('astro').AstroIntegration} */
const hoistIslandStyle = {
  name: 'sdm-hoist-island-style',
  hooks: {
    'astro:build:done': ({ dir }) => {
      const walk = (/** @type {string} */ d) => {
        for (const name of readdirSync(d)) {
          const p = join(d, name);
          if (statSync(p).isDirectory()) walk(p);
          else if (p.endsWith('.html')) {
            const html = readFileSync(p, 'utf8');
            if (!html.includes(ISLAND_STYLE)) continue;
            const moved = html.replace(ISLAND_STYLE, '').replace('</head>', `${ISLAND_STYLE}</head>`);
            writeFileSync(p, moved);
          }
        }
      };
      walk(fileURLToPath(dir));
    },
  },
};

export default defineConfig({
  site: 'https://stardivemeta.com',
  integrations: [sitemap(), hoistIslandStyle],
  vite: {
    plugins: [tailwindcss()],
  },
});
