// Adapter for contract_version 1 snapshot files (plan T4.3): turns
// the frozen documents emitted to generated/snapshots/<id>.json into
// the shapes the components already render, so a history page shows
// exactly what the board looked like then — never today's data.
// One adapter per contract_version; a v2 contract gets a sibling file.
import type { Character } from '../types/contract/Character';
import type { Label } from '../types/contract/Label';
import type { Level } from '../types/contract/Level';
import type { Result } from '../types/contract/Result';
import type { Tier } from '../types/contract/Tier';
import type { TierBoard } from '../types/contract/TierBoard';

export interface FrozenResult {
  tier: Tier;
  label: Label;
  score: number;
  coverage: number;
  spread: number | null;
}

export interface FrozenSource {
  id: string;
  list_date: string | null;
  game_version: string | null;
  status: string;
}

export interface FrozenSnapshot {
  id: string;
  contract_version: number;
  created_at: string;
  method_version: string;
  game_version: string;
  sources: FrozenSource[];
  inputs: {
    roster: {
      character: { id: string; name: string; released_on: string | null }[];
    };
  };
  results: {
    boards: Record<Level, Record<string, string[]>>;
    characters: Record<string, Record<Level, FrozenResult>>;
  };
}

export interface TierChange {
  character: string;
  level: Level;
  from: string;
  to: string;
}
export interface LabelChange {
  character: string;
  level: Level;
  from: string;
  to: string;
}
export interface CoverageChange {
  character: string;
  level: Level;
  from: number;
  to: number;
}
export interface PlacementChange {
  source: string;
  character: string;
  field: string;
  from: string;
  to: string;
}

export interface SnapshotDiff {
  tier_changes: TierChange[];
  label_changes: LabelChange[];
  coverage_changes: CoverageChange[];
  placement_changes: PlacementChange[];
}

export interface SnapshotFileData {
  id: string;
  snapshot: FrozenSnapshot;
  diff: SnapshotDiff | null;
}

interface RawFile {
  snapshot: FrozenSnapshot;
  diff_from_previous: SnapshotDiff | null;
}

const modules = import.meta.glob<RawFile>('../generated/snapshots/*.json', {
  eager: true,
  import: 'default',
});

export const snapshotFiles: SnapshotFileData[] = Object.values(modules)
  .map((raw) => ({
    id: raw.snapshot.id,
    snapshot: raw.snapshot,
    diff: raw.diff_from_previous,
  }))
  .sort((a, b) => a.id.localeCompare(b.id));

const LEVELS: Level[] = ['low', 'high', 'max'];
const TIERS: (keyof TierBoard)[] = [
  'S',
  'A',
  'B',
  'C',
  'D',
  'MODE_SPLIT',
  'UNRANKED',
];

function toResult(frozen: FrozenResult): Result {
  return {
    tier: frozen.tier,
    label: frozen.label,
    score: frozen.coverage > 0 ? frozen.score : null,
    // Matches emit's display rounding (3.1667 → "3.2").
    score_display: frozen.coverage > 0 ? frozen.score.toFixed(1) : null,
    coverage: frozen.coverage,
    spread: frozen.spread,
    votes: [],
    why: { high: null, low: null, mode_split: null },
  };
}

/** The snapshot's characters in the live `Character` shape, with
 * names and release dates from the frozen roster. Evidence and modes
 * are not frozen per snapshot, so they stay empty — the history page
 * renders boards and change lists only. */
export function snapshotCharacters(
  snapshot: FrozenSnapshot,
): Map<string, Character> {
  const roster = new Map(
    snapshot.inputs.roster.character.map((c) => [c.id, c]),
  );
  const out = new Map<string, Character>();
  for (const [id, levels] of Object.entries(snapshot.results.characters)) {
    const entry = roster.get(id);
    out.set(id, {
      id,
      name: entry?.name ?? id,
      released_on: entry?.released_on ?? null,
      is_new: false,
      results: {
        low: toResult(levels.low),
        high: toResult(levels.high),
        max: toResult(levels.max),
      },
      modes: {
        conquest: { tier: 'UNRANKED', raw: null },
        rift: { tier: 'UNRANKED', raw: null },
      },
      evidence: [],
      errata: [],
    });
  }
  return out;
}

export function snapshotBoard(
  snapshot: FrozenSnapshot,
  level: Level,
): TierBoard {
  const raw = snapshot.results.boards[level] ?? {};
  const board = {} as TierBoard;
  for (const tier of TIERS) {
    board[tier] = raw[tier] ?? [];
  }
  return board;
}

export { LEVELS as SNAPSHOT_LEVELS };
