#!/usr/bin/env node
// Dist checks (plan T3.10): gates G12 + G14 over the built site, in
// dependency-free Node so CI's site/deploy jobs can run them without
// the Rust toolchain. The Rust `mongil-build validate --dist` checks
// the same rules; this is the portable twin.
//
//   node site/scripts/check-dist.mjs [--dist site/dist] [--data data]
//     [--allow-missing /codes/,/banner/]
//
// --allow-missing lists internal-link prefixes that may not resolve
// yet (routes owned by a later plan phase). It exists so the check
// can gate everything else honestly in the meantime; drop entries as
// the routes land. Every other failure exits 1.
import { readdirSync, readFileSync, statSync, existsSync } from 'node:fs';
import { dirname, join, relative } from 'node:path';
import { gzipSync } from 'node:zlib';

const DISCLAIMER =
  'Fan-made site. Not an official MONGIL: STAR DIVE or Netmarble site.';
const JS_LIMIT = 6 * 1024;
// Deferred React islands per page (gzip): the WebGL glass layer
// (client:idle) + the level dock (client:load). Pages render completely
// without them. See docs/DESIGN.md "Stack and budgets".
const ISLAND_JS_LIMIT = 420 * 1024;
const IMAGE_EXT = /\.(png|jpe?g|gif|webp|svg|avif|ico)$/i;

const args = process.argv.slice(2);
const opt = (name, fallback) => {
  const i = args.indexOf(`--${name}`);
  return i >= 0 ? args[i + 1] : fallback;
};
const distDir = opt('dist', 'site/dist');
const dataDir = opt('data', 'data');
const allowMissing = (opt('allow-missing', '') || '')
  .split(',')
  .map((s) => s.trim())
  .filter(Boolean);

const failures = [];
const fail = (msg) => failures.push(msg);

function walk(dir, out = []) {
  for (const name of readdirSync(dir)) {
    const p = join(dir, name);
    if (statSync(p).isDirectory()) walk(p, out);
    else out.push(p);
  }
  return out;
}

function resolves(href) {
  const path = href.split('#')[0].split('?')[0];
  if (path === '' || path === '/') return existsSync(join(distDir, 'index.html'));
  const relPath = path.replace(/^\//, '');
  if (/\.[a-z0-9]+$/i.test(relPath)) return existsSync(join(distDir, relPath));
  return (
    existsSync(join(distDir, relPath, 'index.html')) ||
    existsSync(join(distDir, relPath))
  );
}

// Draft notes must never render (G12): a note file whose frontmatter
// says status: draft must have no page in dist/notes/.
function draftNoteDates() {
  const notesDir = join(dataDir, 'notes');
  if (!existsSync(notesDir)) return [];
  return readdirSync(notesDir)
    .filter((f) => f.endsWith('.md'))
    .filter((f) => /^status:\s*draft/m.test(readFileSync(join(notesDir, f), 'utf8')))
    .map((f) => f.replace(/\.md$/, ''));
}

const gzCache = new Map();
function gzipSize(file) {
  if (!gzCache.has(file)) gzCache.set(file, gzipSync(readFileSync(file)).length);
  return gzCache.get(file);
}

// One ES module plus everything it statically imports (relative paths).
function collectModule(file, out) {
  if (out.has(file) || !existsSync(file)) return;
  out.add(file);
  const src = readFileSync(file, 'utf8');
  const dir = dirname(file);
  for (const m of src.matchAll(/(?:from|import)\s*["'](\.{1,2}\/[^"']+\.js)["']/g)) {
    collectModule(join(dir, m[1]), out);
  }
}
const islandTotals = [];

const files = walk(distDir);
const htmlFiles = files.filter((f) => f.endsWith('.html'));

for (const file of htmlFiles) {
  const rel = relative(distDir, file);
  const html = readFileSync(file, 'utf8');
  if (!html.includes(DISCLAIMER)) {
    fail(`G12: dist page ${rel} is missing the disclaimer string`);
  }
  // Internal links resolve.
  for (const m of html.matchAll(/href="(\/[^"]*)"/g)) {
    const href = m[1];
    if (href.startsWith('//')) continue;
    if (allowMissing.some((p) => href === p || href.startsWith(p))) continue;
    if (!resolves(href)) {
      fail(`G12: dist page ${rel} links to ${href}, which does not resolve in dist`);
    }
  }
  // Per-page JS budget (inline + local src files), gzip bytes.
  let jsBytes = 0;
  for (const m of html.matchAll(/<script[^>]*>([\s\S]*?)<\/script>/g)) {
    if (m[1].trim()) jsBytes += gzipSync(Buffer.from(m[1])).length;
  }
  for (const m of html.matchAll(/<script[^>]+src="(\/[^"]+)"/g)) {
    const srcPath = join(distDir, m[1].replace(/^\//, ''));
    if (existsSync(srcPath)) jsBytes += gzipSync(readFileSync(srcPath)).length;
  }
  if (jsBytes > JS_LIMIT) {
    fail(`G12: dist page ${rel} ships ${jsBytes} gzip JS bytes (limit ${JS_LIMIT})`);
  }
  // Island JS budget (theme revamp, 2026-10-30): React islands load
  // through <astro-island component-url/renderer-url>, which the
  // blocking-JS rule above never sees. Count every module they reach
  // (static imports, transitively) so the decorative WebGL layer and
  // the level dock stay inside a stated, enforced budget.
  const islandFiles = new Set();
  for (const m of html.matchAll(/<astro-island\b[^>]*>/g)) {
    for (const a of m[0].matchAll(/(?:component|renderer)-url="(\/[^"]+)"/g)) {
      collectModule(join(distDir, a[1].replace(/^\//, '')), islandFiles);
    }
  }
  let islandBytes = 0;
  for (const f of islandFiles) islandBytes += gzipSize(f);
  islandTotals.push([rel, islandBytes]);
  if (islandBytes > ISLAND_JS_LIMIT) {
    fail(`G12: dist page ${rel} loads ${islandBytes} gzip island JS bytes (limit ${ISLAND_JS_LIMIT})`);
  }
}

for (const date of draftNoteDates()) {
  if (existsSync(join(distDir, 'notes', date, 'index.html'))) {
    fail(`G12: draft note ${date} is rendered in dist`);
  }
}

// G14: images only under img/own/ (allowlisted) or img/generated/.
const allowlistPath = join(distDir, 'img/own/ALLOWLIST.txt');
const allowlist = existsSync(allowlistPath)
  ? readFileSync(allowlistPath, 'utf8')
      .split('\n')
      .map((s) => s.trim())
      .filter((s) => s && !s.startsWith('#'))
  : null;
for (const file of files) {
  const rel = relative(distDir, file).split('\\').join('/');
  if (!IMAGE_EXT.test(rel)) continue;
  if (rel.startsWith('img/own/')) {
    const name = rel.slice('img/own/'.length);
    if (allowlist && !allowlist.includes(name)) {
      fail(`G14: dist image img/own/${name} is not in ALLOWLIST.txt`);
    }
  } else if (!rel.startsWith('img/generated/')) {
    fail(`G14: dist image ${rel} is outside img/own/ and img/generated/`);
  }
}

if (failures.length > 0) {
  for (const f of failures) console.log(`FAIL  ${f}`);
  console.log(`check-dist: ${failures.length} check(s) failed`);
  process.exit(1);
}
const maxIsland = islandTotals.reduce((a, b) => (b[1] > a[1] ? b : a), ['', 0]);
console.log(
  `check-dist: OK — ${htmlFiles.length} pages (disclaimer, links, JS budget, island JS ≤ ${ISLAND_JS_LIMIT} B gz — max ${maxIsland[1]} B on ${maxIsland[0]}, draft notes), G14 images clean` +
    (allowMissing.length ? ` [allow-missing: ${allowMissing.join(', ')}]` : ''),
);
