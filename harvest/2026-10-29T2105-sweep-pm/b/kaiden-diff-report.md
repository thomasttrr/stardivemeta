# Kaiden double-read diff report — sweep 2026-10-29T2105-sweep-pm

B-read #2 (Kaiden) processing: `b/kaiden.toml` was transcribed from
`B-READ-2-kaiden.md` (browser screenshot/visual read) ONLY, following
docs/harvest-guide.md's Kaiden rules — `tier` = first-listed tier per
level (two-card High lines keep both cards in `raw`, first card is the
value), `awakening` = the (A#) marker on the first-listed card,
`flags` = (P)/(E) markers on that level's line, Sera's signature
conditionals recorded first-listed with the condition preserved in
`raw`/`note`, Chloe = `status = "no_placement"`. The A-read values were
not consulted while transcribing.

## Command

```sh
mongil-build harvest-diff harvest/2026-10-29T2105-sweep-pm/a/kaiden.toml \
                          harvest/2026-10-29T2105-sweep-pm/b/kaiden.toml
```

Exit code: **0** (clean).

## Diff output (verbatim)

```text
harvest-diff kaiden 2026-10-29T2105-sweep-pm
WARN     chloe.note                      A="Page shows 'Review coming soon' and has no Tier Position section."  B="Page shows 'Review coming soon' and no Tier Position section."   (normalised differs)
WARN     francis-summer-dive.high.raw    A="T0.5 (A2), T1 at A1"  B="T0.5 (A2), T1 (A1)"   (normalised differs)
WARN     gabi.high.raw                   A="T1 (A4), T2 at A2"  B="T1 (A4), T2 (A2)"   (normalised differs)
WARN     jiwon.high.raw                  A="T1 (A4), T2 at A2 (P)"  B="T1 (A4), T2 (A2, P)"   (normalised differs)
WARN     jiwon.note                      A=""  B="The (P) marker at High appears on the second (A2) card only."   (normalised differs)
WARN     mina.high.raw                   A="T0.5 (A4), T1 at A2"  B="T0.5 (A4), T1 (A2)"   (normalised differs)
WARN     penny.high.raw                  A="T2 (A1) (E)"  B="T2 (A1, E)"   (normalised equal)
WARN     penny.max.raw                   A="T1 (A6) (E)"  B="T1 (A6, E)"   (normalised equal)
WARN     sera.high.raw                   A="T2 (with signature), T3"  B="T2 (S, with signature), T3 (no signature)"   (normalised differs)
WARN     sera.max.raw                    A="T2 (A6, with signature)"  B="T2 (A6, S, with signature)"   (normalised differs)
WARN     sera.note                       A="High and Max tiers carry the page's 'S' marker: the tier-list page legend defines S = Signature Weapon. High reads T2 with signature (Ophelia's signature, per the guide text), T3 without; Max reads T2 at A6 with signature."  B="Only character with signature-conditional tiers. High: T2 with signature (S marker), T3 with no signature marker; Max: T2 at A6 with signature. Per the B read, the page's Investment Scores distinguish 'Ascension 6 / No Ophelia Signature' vs 'Ascension 6 / With Ophelia Signature'."   (normalised differs)
WARN     vivian.high.raw                 A="T0.5 (A4), T1 at A2"  B="T0.5 (A4), T1 (A2)"   (normalised differs)
0 blocking, 12 warnings → clean
```

## Totals

- **Blocking mismatches: 0.** Character set (29/29, Chloe
  `no_placement` in both), every `low/high/max.tier`, every
  `awakening`, every `flags`, `status`, and `page_updated` (omitted in
  both) agree between the two independent reads.
- **Warnings: 12**, all in the free-text fields (`raw` / `note`), which
  never block per the guide.

## Classification of blocking mismatches

None — there are no blocking mismatches to classify as (a) format
artifact, (b) genuine disagreement, or (c) A-read reconstruction risk.
Neither file was modified to reach this result.

## Warning analysis (for the merge step's awareness)

All 12 warnings are wording differences over identical underlying
facts — the class-(a) pattern, in non-blocking fields:

- **Second-card style (5):** mina, gabi, vivian, francis-summer-dive
  and jiwon `high.raw` — A reconstructed the flattened line as
  "T1 at A2"; B wrote "T1 (A2)". Same tiers, same awakenings.
- **Marker punctuation (2):** penny `high.raw` / `max.raw` — "(A1) (E)"
  vs "(A1, E)"; these two are even *normalised equal* and are printed
  by the tool for completeness.
- **Sera conditionals (3):** `high.raw`, `max.raw`, `note` — B spells
  out the "S" marker and the "no signature" second card; A summarised.
  Same first-listed tiers (T2/T2) and same condition.
- **Notes (2):** chloe `note` (one word differs) and jiwon `note`
  (B adds an explanatory note about the (P) sitting on the second High
  card; A left the character note empty — the marker itself is in both
  reads' `flags`).

## Verdict

Diff is **clean (0 blocking)** → per the guide's merge rule, Kaiden's
placements may be merged from worker A's read. No file was fixed or
edited during this processing.
