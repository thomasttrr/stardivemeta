// T1.8 design mocks — shared view-model builder.
// Reads the REAL emitted contract JSON (site/src/generated) so the three
// _design mocks render the actual Low board and Francis's actual evidence,
// never placeholder data. Mocks only; P3 builds the real components.
import tierlistJson from '../generated/tierlist.json';
import metaJson from '../generated/meta.json';
import type { Character } from '../types/contract/Character';
import type { EvidenceRow } from '../types/contract/EvidenceRow';
import type { Label } from '../types/contract/Label';
import type { Tier } from '../types/contract/Tier';
import type { TierBoard } from '../types/contract/TierBoard';

interface SourceInfo {
  id: string;
  name: string;
  role: string;
  criteria_caveat: string | null;
}

interface TierListData {
  game_version: string;
  sources: SourceInfo[];
  characters: Character[];
  boards: { low: TierBoard; high: TierBoard; max: TierBoard };
}

const data = tierlistJson as unknown as TierListData;
const meta = metaJson as { sources_checked_at: string; method_version: string };

export const GLYPH: Record<Label, string> = {
  SOLID: '●',
  CONTESTED: '◐',
  SPECULATION: '○',
  MODE_SPLIT: '⇄',
  UNRANKED: '—',
};

export const LABEL_WORD: Record<Label, string> = {
  SOLID: 'Solid',
  CONTESTED: 'Contested',
  SPECULATION: 'Speculation',
  MODE_SPLIT: 'Mode-split',
  UNRANKED: 'Unranked',
};

export const LEGEND: { glyph: string; word: string; text: string }[] = [
  { glyph: '●', word: 'Solid', text: 'the lists agree within 1 tier.' },
  { glyph: '◐', word: 'Contested', text: 'the lists are more than 1 tier apart.' },
  { glyph: '○', word: 'Speculation', text: 'only 1 list has placed them so far.' },
  { glyph: '⇄', word: 'Mode-split', text: 'strong in one mode, weak in the other. See Conquest and Rift.' },
];

const MONTHS = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];

export function fmtDate(iso: string | null): string {
  if (!iso) return 'date not shown';
  const [y, m, d] = iso.slice(0, 10).split('-').map(Number);
  return `${MONTHS[(m ?? 1) - 1]} ${d}, ${y}`;
}

export const stamp = `Lists last checked ${fmtDate(meta.sources_checked_at)} · Method v${meta.method_version} · Game version ${data.game_version}`;

export function sourceName(id: string): string {
  return data.sources.find((s) => s.id === id)?.name ?? id;
}

export interface MockChip {
  id: string;
  name: string;
  glyph: string;
  labelWord: string;
  label: Label;
  score: string | null;
  coverage: number;
}

export interface MockRow {
  tierKey: Tier;
  tierLabel: string;
  band: string;
  chips: MockChip[];
}

const TIER_ORDER: { key: Tier; label: string; band: string }[] = [
  { key: 'S', label: 'S', band: '3.5 – 4.0' },
  { key: 'A', label: 'A', band: '3.0 – 3.4' },
  { key: 'B', label: 'B', band: '2.0 – 2.9' },
  { key: 'C', label: 'C', band: '1.0 – 1.9' },
  { key: 'D', label: 'D', band: 'below 1.0' },
  { key: 'MODE_SPLIT', label: 'Mode-split', band: 'no single tier' },
  { key: 'UNRANKED', label: 'Unranked', band: 'no votes yet' },
];

const byId = new Map(data.characters.map((c) => [c.id, c]));

export const lowRows: MockRow[] = TIER_ORDER.flatMap(({ key, label, band }) => {
  const ids = data.boards.low[key];
  if (!ids || ids.length === 0) return [];
  return [
    {
      tierKey: key,
      tierLabel: label,
      band,
      chips: ids.map((id) => {
        const c = byId.get(id);
        if (!c) throw new Error(`board references unknown character ${id}`);
        const r = c.results.low;
        return {
          id,
          name: c.name,
          glyph: GLYPH[r.label],
          labelWord: LABEL_WORD[r.label],
          label: r.label,
          score: r.score_display,
          coverage: r.coverage,
        };
      }),
    },
  ];
});

// ---------- Francis evidence view-model (§B.5 Character page) ----------

export interface EvidenceLine {
  sourceName: string;
  placementText: string;
  convertedText: string;
  dateText: string;
  url: string;
  note: string | null;
}

function placementText(row: EvidenceRow): string {
  const p = row.placement;
  switch (p.kind) {
    case 'single':
      return p.raw;
    case 'modes': {
      const parts: string[] = [];
      if (p.conquest) parts.push(`Conquest ${p.conquest}`);
      if (p.rift) parts.push(`Rift ${p.rift}`);
      return parts.join(' · ');
    }
    case 'levels': {
      const cell = (word: string, c: typeof p.low) =>
        `${word} ${c.raw}${c.awakening ? ` (${c.awakening})` : ''}`;
      return [cell('Low', p.low), cell('High', p.high), cell('Max', p.max)].join(' · ');
    }
    case 'none':
      return p.reason === 'no_placement'
        ? 'No placement — review coming soon'
        : p.reason === 'not_in_text_tiers'
          ? 'Not in its text lists — not counted'
          : 'Not listed';
  }
}

function convertedText(row: EvidenceRow): string {
  const p = row.placement;
  switch (p.kind) {
    case 'single':
      return p.converted == null ? '—' : p.converted.toFixed(1);
    case 'modes':
      return p.converted_overall == null ? '—' : p.converted_overall.toFixed(1);
    case 'levels':
      return `${p.low.converted.toFixed(1)} at Low`;
    case 'none':
      return '—';
  }
}

function toLine(row: EvidenceRow): EvidenceLine {
  return {
    sourceName: sourceName(row.source),
    placementText: placementText(row),
    convertedText: convertedText(row),
    dateText: fmtDate(row.page_date),
    url: row.page_url,
    note: row.note,
  };
}

const NOT_COUNTED_REASON: Record<string, string> = {
  'urgametips': "Near-copy of Pocket Gamer's Conquest list",
  'gacha-gg': 'Aggregate that includes Kaiden — counting it would count Kaiden twice',
  'gamewith-en': 'Launch-era list',
  'mongil-gg': 'Read from images — not counted until a method update',
};

const francisChar = byId.get('francis');
if (!francisChar) throw new Error('francis missing from dataset');
const fr = francisChar.results.low;
if (!fr.why.high || !fr.why.low) throw new Error('francis why/high/low missing');

export const francis = {
  name: francisChar.name,
  summary: (['low', 'high', 'max'] as const).map((lvl) => {
    const r = francisChar.results[lvl];
    return {
      word: lvl[0].toUpperCase() + lvl.slice(1),
      tier: r.tier === 'MODE_SPLIT' ? 'Mode-split' : r.tier,
      labelWord: LABEL_WORD[r.label],
      current: lvl === 'low',
    };
  }),
  numbers: `Score ${fr.score_display} of 4 · ${fr.coverage} lists · gap ${fr.spread?.toFixed(1)}`,
  why: `${sourceName(fr.why.high.source)} says ${fr.why.high.raw} (${fr.why.high.value.toFixed(1)}). ${sourceName(fr.why.low.source)} says ${fr.why.low.raw} (${fr.why.low.value.toFixed(1)}). That gap is more than 1.0.`,
  counted: francisChar.evidence.filter((e) => e.counted).map(toLine),
  criteria: data.sources
    .filter((s) => s.role === 'voting' && s.criteria_caveat)
    .map((s) => ({ name: s.name, text: s.criteria_caveat as string })),
  notCounted: francisChar.evidence
    .filter((e) => !e.counted)
    .map((e) => ({ ...toLine(e), reason: NOT_COUNTED_REASON[e.source] ?? 'Not counted' })),
  modes: `Conquest ${francisChar.modes.conquest.tier} · Rift ${francisChar.modes.rift.tier} (Pocket Gamer only)`,
};
