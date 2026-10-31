// Character portraits for the medallions (display only — no data, no
// ranking). Each file is a 320×320 face crop under public/img/own/art/,
// cut from the official art archived in design-assets/ (see
// docs/BUILD-LOG.md for the per-character source). A character with no
// file here gets the letter fallback, so a newly emitted id never breaks
// the build.
const ART = new Set([
  'angel',
  'benjamin',
  'bonnie',
  'brisshell',
  'chloe',
  'cloud',
  'daisy',
  'ellie',
  'esther',
  'flare',
  'francis',
  'francis-summer-dive',
  'gabi',
  'isabella',
  'jiwon',
  'leeho',
  'mabel',
  'mina',
  'nagi',
  'narae',
  'ophelia',
  'penny',
  'reina',
  'sangun',
  'sera',
  'verna',
  'verna-summer-dive',
  'vivian',
  'yeonhwa',
]);

/** Public path of a character's portrait, or null when there is none. */
export function portraitSrc(id: string): string | null {
  return ART.has(id) ? `/img/own/art/${id}.webp` : null;
}
