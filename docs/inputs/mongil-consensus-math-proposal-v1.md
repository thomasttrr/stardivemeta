# MONGIL: STAR DIVE — Consensus Math — Method v1.0 (LOCKED, 2026-10-29)
> **Status note (Tom, 2026-10-29 ~19:09 ET):** decision record, NOT a permanent lock. Input to the Opus 5.5 planning pass (see `stardivemeta/00-MASTER-HANDOFF-for-Opus.md`); may be revised there with reasons stated.

Status: LOCKED. The five open numbers were answered (Grok 4.7 review, accepted):
equal weights; Kaiden T4 = 0; Solid at spread ≤ 1.0; S band at ≥ 3.5;
mode-split override at ≥ 2.0. A full 29-character simulation at Low/High/Max
was then run from the saved dataset (mongil-consensus-simulation-v1.txt) and
checked clean — no absurd placements; watch items are Sangun (widest spread,
3.5, PG vs Kaiden), Francis (JP's maxed-based SS vote sits in the Low view;
handled by the Contested label + the printed fixed-vote rule), and Chloe
(thinnest S, coverage 2, no Kaiden vote; coverage is shown in evidence).
Two publishing rules added: print the conversion assumption on the page, and
print the fixed-vote rule (PG and JP votes do not change with the investment
toggle; only Kaiden's does).
Method name when published: Method v1.0. If the rules ever change, old snapshots
keep their old method label — history is never rewritten under new rules.

## 1. Source eligibility (who gets a vote)

Voting sources (independent, primary, current) — exactly 3 at launch:
- Pocket Gamer (PG) — per-mode (Conquest / Rift), v1.04, Oct 5, 2026.
- Kaiden (K) — overall, split by investment (Low / High / Max), guide pages.
- GameWith JP (JP) — overall, full 29 roster, Oct 21, 2026.

Check-only sources (shown in evidence, weight 0, never in the math):
- gacha.gg — aggregator; counting it would double-count Kaiden / mongil.gg.
- urgametips — Conquest list is near-identical to PG's; corroboration, not an
  independent method.
- GameWith EN — launch-era (21 chars), superseded by GameWith JP.

Excluded entirely from current math:
- Pocket Tactics (~Aug 2026) and Driffle (~Jun 2026) — pre-patch, stale.
  They stay in snapshot history only.
- mongil.gg — placements are image-only and unread. Eligible the day a human
  reads them; until then it is not a vote and not a check.

## 2. Conversion table (one shared scale, 0–4)

Assumption, printed on the rules page: one tier step = one point on every list.

- Pocket Gamer: S=4, A=3, B=2, C=1
- GameWith JP: SS=4, S=3, A=2 (JP publishes no lower text tier; a character
  absent from JP's text tiers is MISSING, not low)
- Kaiden: T0=4, T0.5=3.5, T1=3, T2=2, T3=1, T4=0
  (Kaiden splits the top tier in two — T0/T0.5 — and adds a bottom tier, T4,
  below everyone else's C.)

Missing is never zero. Zero only ever comes from a real T4 placement.

## 3. How a tier is calculated

- Equal weights: every voting source counts 1.0. No reputation weighting in
  v1 — equal is the only weighting we can defend on the rules page.
- PG's Overall vote = average of its Conquest and Rift scores (when both exist).
- Overall score = mean of the available votes. Score shown to 1 decimal in the
  evidence view.
- Tier bands: S ≥ 3.5 · A = 3.0–3.49 · B = 2.0–2.99 · C = 1.0–1.99 · D < 1.0
- The investment toggle re-runs the whole math three times (Low / High / Max).
  Kaiden's vote changes per toggle; PG and JP publish one list, so their votes
  stay constant — JP's own mixed assumptions (limited 5★ at A0, permanent 5★
  to A2, 4★ at A6) are printed in its evidence entry.
- Mode views are NOT faked consensus: only PG publishes per-mode placements,
  so a mode tier = PG's placement, displayed next to the Overall consensus and
  its confidence label, until a second per-mode source (mongil.gg) is read.
- Mode-split override: if a character's PG Conquest vs Rift scores differ by
  ≥ 2.0 points, no Overall tier is shown — the character is listed as
  Mode-split and shown per-mode only. (Angel: Conquest 1 vs Rift 4.)

## 4. Confidence (two axes, one label)

- Coverage = number of independent voting sources with a placement for that
  character in that view.
- Agreement = spread = highest vote − lowest vote (converted scale).
- Label rules:
  - Coverage 0 → Unranked (no tier shown)
  - Coverage 1 → Speculation (tier shown, marked provisional)
  - Coverage ≥ 2 and spread ≤ 1.0 → Solid
  - Coverage ≥ 2 and spread > 1.0 → Contested
- Speculation means low coverage. Contested means good coverage, real
  disagreement. The two axes are also shown separately in the evidence view.

## 5. Staleness (by game version, not calendar days)

- Current: the source's list is on the latest game version, or was updated
  after the last balance patch.
- Pre-patch, no balance changes in that patch: vote still counts, flagged.
- Predates a balance patch or a roster-changing release: vote drops to
  check-only (weight 0) until the source updates. Coverage is recalculated.
- A new character release never lowers other characters' scores: sources that
  have not ranked the new character simply have no vote for them.
- Every snapshot records the game version and each source's version + date.

## 6. Worked examples (Low investment, today's data)

- Narae: PG 4.0, K 4.0, JP 4.0 → score 4.0 → S, spread 0 → Solid.
- Francis (base): PG 2.0, K 3.5, JP 4.0 → score 3.2 → A, spread 2.0 →
  Contested. (JP's SS is for maxed Francis — the evidence view says so.)
- Sangun: PG 3.5, K 0.0, JP no vote → score 1.8 → C, spread 3.5 → Contested.
  Kaiden's T4 is the outlier and the evidence view shows it.
- Yeonhwa: PG 1.0, K 3.5, JP 3.0 → score 2.5 → B, spread 2.5 → Contested.
- Chloe: PG 4.0, JP 4.0, K no vote → score 4.0 → S, spread 0, coverage 2 →
  Solid. (Her old Speculation tag drops — two sources now agree.)
- Angel: PG Conquest 1 vs Rift 4 → mode-split override; no Overall tier.

## 7. The five numbers — LOCKED as proposed (kept below for the record)

1. Equal weights for all 3 voting sources — or should PG count more on mode
   views / Kaiden more at Max investment?
2. Kaiden T4 = 0 (full 0–4 scale) — or T4 = 0.5 to keep every list's floor at C?
3. Solid cutoff at spread ≤ 1.0 — stricter (≤ 0.5) or looser?
4. S band at ≥ 3.5 — with only 3 sources, one A vote (3.0) pulls a character
   out of S. Intended, or should S start at 3.3?
5. Mode-split override at ≥ 2.0 points difference — right threshold?
