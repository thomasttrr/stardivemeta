// Typed access to the emitted contract JSON (site/src/generated).
// Pages and components read ONLY these files (plan P3 rule) — never data/.
import tierlistJson from '../generated/tierlist.json';
import metaJson from '../generated/meta.json';
import historyJson from '../generated/history.json';
import errataJson from '../generated/errata.json';
import codesJson from '../generated/codes.json';
import bannersJson from '../generated/banners.json';
import type { Character } from '../types/contract/Character';
import type { CodesFile } from '../types/contract/CodesFile';
import type { Derived } from '../types/contract/Derived';
import type { EvidenceRow } from '../types/contract/EvidenceRow';
import type { History } from '../types/contract/History';
import type { Label } from '../types/contract/Label';
import type { Level } from '../types/contract/Level';
import type { MethodInfo } from '../types/contract/MethodInfo';
import type { Source } from '../types/contract/Source';
import type { Tier } from '../types/contract/Tier';
import type { TierBoard } from '../types/contract/TierBoard';
import type { ModeBoard } from '../types/contract/ModeBoard';

export interface TierListData {
  contract_version: number;
  method: MethodInfo;
  game_version: string;
  snapshot_id: string;
  sources: Source[];
  characters: Character[];
  boards: Record<Level, TierBoard>;
  mode_boards: { conquest: ModeBoard; rift: ModeBoard };
  derived: Derived;
}

export const history = historyJson as unknown as History;

/** One corrections record from errata.json (emitted from
 * data/errata.toml; plan T3.8). */
export interface ErrataEntry {
  id: string;
  found_at: string;
  fixed_at: string | null;
  severity: 'minor' | 'major';
  what: string;
  characters: string[];
  snapshots: string[];
}

export const errata = (errataJson as unknown as { errata: ErrataEntry[] })
  .errata;

export const errataById = new Map<string, ErrataEntry>(
  errata.map((e) => [e.id, e]),
);

/** codes.json (plan T4.1): statuses computed in Rust; the page only
 * renders them. */
export const codesFile = codesJson as unknown as CodesFile;

/** One banner record from banners.json (emitted from
 * data/banners.toml; plan T4.2). `end` is null when no end date is
 * recorded. */
export interface BannerEntry {
  character: string;
  name: string;
  start: string;
  end: string | null;
  source: string;
}

export const banners = (bannersJson as unknown as { banners: BannerEntry[] })
  .banners;

export const data = tierlistJson as unknown as TierListData;
export const meta = metaJson as unknown as {
  commit: string;
  generated_at: string;
  method_version: string;
  snapshot_id: string;
  sources_checked_at: string;
  codes_checked_at: string;
};

export const LEVELS: Level[] = ['low', 'high', 'max'];
export const LEVEL_NAME: Record<Level, string> = {
  low: 'Low',
  high: 'High',
  max: 'Max',
};

export const LABEL_WORD: Record<Label, string> = {
  SOLID: 'Solid',
  CONTESTED: 'Contested',
  SPECULATION: 'One list',
  MODE_SPLIT: 'Mode-split',
  UNRANKED: 'Unranked',
};

export const LABEL_GLYPH: Record<Label, string> = {
  SOLID: '●',
  CONTESTED: '◐',
  SPECULATION: '○',
  MODE_SPLIT: '⇄',
  UNRANKED: '—',
};

export function tierDisplay(tier: Tier): string {
  if (tier === 'MODE_SPLIT') return '⇄';
  if (tier === 'UNRANKED') return '—';
  return tier;
}

export const sourcesById = new Map<string, Source>(
  data.sources.map((s) => [s.id, s]),
);
export const charactersById = new Map<string, Character>(
  data.characters.map((c) => [c.id, c]),
);

export function sourceName(id: string): string {
  return sourcesById.get(id)?.name ?? id;
}

/** Why a check-only source is not counted (§B.5 Character page, item 7). */
export const NOT_COUNTED_REASON: Record<string, string> = {
  'gacha-gg':
    'Combines other lists, including Kaiden — counting it would count Kaiden twice',
  urgametips: "Near-copy of Pocket Gamer's Conquest list",
  'gamewith-en': 'Old list from launch — replaced by GameWith JP',
  'mongil-gg': 'Its lists are images, not text — not counted yet',
};

/** Canonical text for a missing vote (§B.5 Character page, item 5). */
export const NONE_REASON_TEXT: Record<string, string> = {
  no_placement: 'No placement — review coming soon',
  not_in_text_tiers: 'Not in its text lists — not counted',
  not_listed: 'Not listed',
};

/** Short display versions of each voting source's criteria caveat
 * (CP3 debloat, Tom 2026-10-29). Display-only: the full caveat text
 * stays in the data and is printed in full on the Rules page. */
export const CRITERIA_SHORT: Record<string, string> = {
  pg: 'Conquest = boss fights; Rift = dungeons, many enemies. Story list not used.',
  kaiden: 'Rates every character at Low, High and Max investment.',
  'gamewith-jp': 'Limited 5★ at A0, permanent 5★ to A2, 4★ at A6.',
};

/** Short display versions of long per-row evidence notes, keyed by the
 * exact note text in the data (CP3 debloat, Tom 2026-10-29).
 * Display-only: the full note stays in the data untouched. */
export const NOTE_SHORT: Record<string, string> = {
  'Headline names 完凸フランシス (fully-awakened Francis) among the strongest; Francis is ★4, and the page criteria evaluate 4★ up to A6.':
    'Headline names fully-awakened Francis among the strongest; rated as 4★ up to A6.',
  "High and Max tiers carry the page's 'S' marker: the tier-list page legend defines S = Signature Weapon. High reads T2 with signature (Ophelia's signature, per the guide text), T3 without; Max reads T2 at A6 with signature.":
    "Kaiden's 'S' = Signature Weapon. High: T2 with, T3 without. Max: T2 at A6.",
};

/** Kaiden's own legend wording for its (E)/(P) markers (read from the
 * site's RANKING CRITERIA panel during the T2.2 harvest). */
export const FLAG_TEXT: Record<string, string> = {
  E: 'Expert: hard to play well',
  P: 'Partner: needs a specific teammate',
};

/** "4.0" / "3.5" — converted vote values always show one decimal. */
export function fmtVal(v: number): string {
  return v.toFixed(1);
}

/** Spread/gap values: "0", "0.5", "1", "2" — no trailing ".0". */
export function fmtSpread(v: number | null): string {
  if (v === null) return '—';
  return String(parseFloat(v.toFixed(2)));
}

/** "2026-10-05" → "Oct 5, 2026". */
export function fmtDate(iso: string | null): string {
  if (!iso) return '—';
  const d = new Date(`${iso}T12:00:00Z`);
  return d.toLocaleDateString('en-US', {
    month: 'short',
    day: 'numeric',
    year: 'numeric',
    timeZone: 'UTC',
  });
}

const checkedFmt = new Intl.DateTimeFormat('en-US', {
  month: 'short',
  day: 'numeric',
  year: 'numeric',
  hour: 'numeric',
  minute: '2-digit',
  timeZone: 'America/New_York',
});

/** "Lists last checked {date, time ET}" stamp text (§B.5 Home, item 7). */
export function stampChecked(): string {
  return `${checkedFmt.format(new Date(meta.sources_checked_at))} ET`;
}

/** The same ET stamp for an arbitrary instant (codes page, T4.1). */
export function stampFrom(iso: string): string {
  return `${checkedFmt.format(new Date(iso))} ET`;
}

/** "{Month YYYY}" for page titles, from the last check date (ET). */
export function titleMonth(): string {
  const parts = new Intl.DateTimeFormat('en-US', {
    month: 'long',
    year: 'numeric',
    timeZone: 'America/New_York',
  }).format(new Date(meta.sources_checked_at));
  return parts;
}

/** The "Why {label}" sentence for one character at one level (§B.5 #4). */
export function whyText(c: Character, level: Level): string {
  const r = c.results[level];
  switch (r.label) {
    case 'SOLID':
      return `${r.coverage} lists placed ${c.name}. Their placements are within ${fmtSpread(r.spread)} tier steps of each other.`;
    case 'CONTESTED': {
      const hi = r.why.high;
      const lo = r.why.low;
      if (hi && lo) {
        return `${sourceName(hi.source)} says ${hi.raw} (${fmtVal(hi.value)}). ${sourceName(lo.source)} says ${lo.raw} (${fmtVal(lo.value)}). That gap is more than 1 tier step.`;
      }
      return `The lists are more than 1 tier step apart on ${c.name}.`;
    }
    case 'SPECULATION': {
      const only = r.votes[0];
      return only
        ? `Only ${sourceName(only.source)} has placed ${c.name} so far.`
        : `Only one list has placed ${c.name} so far.`;
    }
    case 'MODE_SPLIT':
      return `Pocket Gamer has ${c.name} at ${c.modes.conquest.tier} in Conquest and ${c.modes.rift.tier} in Rift. That gap is too big for one Overall tier.`;
    case 'UNRANKED':
      return `None of the lists we count has placed ${c.name} yet.`;
  }
}

/** Flag summary line for a Kaiden "levels" placement, or null. */
export function flagSummary(row: EvidenceRow): string | null {
  if (row.placement.kind !== 'levels') return null;
  const parts: string[] = [];
  for (const flag of ['E', 'P'] as const) {
    const at = LEVELS.filter((lv) => row.placement.kind === 'levels' && row.placement[lv].flags.includes(flag));
    if (at.length > 0) {
      parts.push(
        `${flag} (${FLAG_TEXT[flag]}) at ${at.map((lv) => LEVEL_NAME[lv]).join(', ')}`,
      );
    }
  }
  return parts.length > 0 ? parts.join(' · ') : null;
}
