# StarDiveMeta — Update Workflow (LOCKED, 2026-10-29)
> **Status note (Tom, 2026-10-29 ~19:09 ET):** decision record, NOT a permanent lock. Input to the Opus 5.5 planning pass (see `stardivemeta/00-MASTER-HANDOFF-for-Opus.md`); may be revised there with reasons stated.

Status: LOCKED (Kit drafted, Grok 4.7 reviewed and kept it as drafted, Tom
relayed the lock). Built on the locked Method v1.0 rules and the
cadence patterns Tom already runs on WuWa Today.

## 1. Source harvest (tier data)
- Sweep TWICE DAILY — 09:00 ET and 23:45 ET (Tom's amendment 2026-10-29
  ~18:41 ET, replacing the original weekly Wednesday sweep): a worker
  re-checks all three voting sources (Pocket Gamer, Kaiden guide pages,
  GameWith JP) plus the check-only sources. Silent when nothing changed.
- Patch trigger: when a new game version or balance patch lands, an extra
  sweep runs within 24 hours regardless of the twice-daily rhythm. Banners:
  the new character's evidence page goes up with first placements, and the
  twice-daily sweeps cover the re-checks until at least 2 voting sources
  rank them.
- Verification: the harvest worker extracts placements; a second worker
  independently re-reads the source and diffs the transcription. A mismatch
  blocks the update. Nothing is ever harvested from memory or guessed.
- Publishing: data-only updates (placements, timestamps) auto-commit and
  deploy when the build + data validation pass — same standing rule as
  Grace's zero-visual-change optimizations on WuWa Today. Anything touching
  code, design, or the method goes to a branch for Tom's approval.

## 2. Codes (daily)
- Daily scout ~09:00 ET, same pattern as the WuWa code scout.
- Two-source rule: a code lists as Active only when two independent sources
  agree. Conflicts go to the Unconfirmed section. A code expires when two
  sources agree it is dead or its stated expiry passes.
- The page stamp always shows the last check date. New confirmed codes are
  reported to Tom the same day; no-change days are silent.

## 3. "What moved" note (weekly)
- Every snapshot diff is computed automatically: tier changes, label changes,
  new/changed source placements.
- A worker drafts the note with a source citation for every move. Tom
  approves before it publishes (same pattern as the Lily guides).
- Graduation rule: after 4 consecutive notes Tom approves without edits, Kit
  proposes switching the note to auto-publish. Tom's call at that time.

## 4. Stale / paywall / dead source playbook
- Follows the locked Method v1.0 staleness rules (by game version, not days):
  a source that misses a patch is flagged on its evidence entry; a source
  that misses a balance patch automatically drops to check-only and coverage
  is recalculated. Tom is told the same day.
- A paywalled or vanished source is treated as unavailable and follows the
  same path.
- Replacing a voting source is a METHOD change: Kit brings Tom candidate
  sources with evidence; the swap ships only with his approval as a method
  version bump (v1.1+), and old snapshots keep their original method label.
- If voting sources ever drop below 3, harvesting mongil.gg's image tier
  lists (human/visual read) becomes the top priority task.

## 5. Health + breakage
- Daily health check: live site responds, latest deploy matches the repo,
  codes stamp is current. Silent when green; a problem is reported with the
  project name, WuWa-style.
- Harvest breakage (a source changes its page format — Kaiden's guide-page
  placements are the fragile one): the update stops, nothing is guessed,
  Tom gets a plain report of what broke and what is still current.
