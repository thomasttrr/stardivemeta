# A-read report — sweep 2026-10-29T2105-sweep-pm (worker A, rendered text)

Read window: 2026-10-29 ~21:03–21:08 ET. All reads are fresh from the live
pages; nothing was copied from `data/`. Files written to
`harvest/2026-10-29T2105-sweep-pm/a/`. `data/` untouched; merge waits for
the B-read diff.

## Coverage per source

### Pocket Gamer (`pg.toml`) — https://www.pocketgamer.com/mongil-star-dive/tier-list/
- Page states: "Updated on October 5th, 2026 — Version: 1.04.00 — Latest
  Addition: Isabella, Chloe". `page_updated = 2026-10-05`.
- **23 characters with both Conquest and Rift tiers** (Conquest S5/A6/B6/C6;
  Rift S7/A5/B4/C7).
- **6 characters in neither endgame list** → `status = "not_listed"`:
  nagi, brisshell, mabel, verna-summer-dive, francis-summer-dive, vivian.
  (They appear only in the World/Story list, which the guide excludes —
  and that list doesn't label the Summer variants anyway.)
- Angel is the extreme mode split: Conquest C, Rift S.

### Kaiden (`kaiden.toml`) — 29 guide pages, https://www.kaiden.gg/mongil/characters/<slug>/
- **28 characters with Low/High/Max placements** (84 level cells).
- **Chloe: `status = "no_placement"`** — her page shows "Review coming
  soon" and has no Tier Position section. Kaiden's tier-list page likewise
  shows 28 characters.
- **No guide page showed its own "updated" date** in the rendered text, so
  `page_updated` is omitted for all 29 (the UI will say "date not shown").
  The character index states "Last updated: October 19, 2026".
- Raw reconstruction: rendered text flattens "(A#)"/"(P)"/"(E)" markers
  into comma tokens; `raw` in kaiden.toml uses the §B.3 style. The exact
  flattened Tier Position line for every character is listed below so the
  B-read and the merge can audit the reconstruction.

### GameWith JP (`gamewith-jp.toml`) — https://gamewith.jp/stardive/552850
- Page updated **2026-10-21** (page metadata: 2026-10-21T11:41:21+09:00).
- **23 characters in the text tiers: SS 9, S 9, A 5.**
  - SS: francis, narae, francis-summer-dive, mina, brisshell, ophelia,
    chloe, isabella, esther
  - S: penny, jiwon, nagi, verna-summer-dive, mabel, vivian, cloud,
    yeonhwa, benjamin
  - A: flare, gabi, angel, sera, reina
- **6 absent from all text tiers** → `status = "not_in_text_tiers"` (no
  vote): sangun, verna, daisy, leeho, ellie, bonnie. (Ellie is mentioned
  in criteria prose but placed in no tier section.)
- Criteria as printed: limited 5★ at A0, permanent 5★ up to A2, 4★ up to
  A6. Headline names fully-awakened Francis (完凸フランシス) among the
  strongest — noted on Francis's row.
- The visual tier table was **not** read (that is T2.5's separate read).

## T2.3 — game version (detail in `game-version-note.md`)
Official string is **"1.4"** (Netmarble press release, 2026-09-30).
"1.04.00" is the App Store build string of the same release; current
build 1.04.10 (2026-10-28) is still version 1.4. `data/game.toml` not
edited — for the merge step.

## T2.4 — Kaiden level definitions (detail in `kaiden-level-definitions.md`)
**Found.** Kaiden's tier-list page defines: Low = "accessible awakenings
and limited signature weapon investment"; High = "higher awakenings and
stronger signature weapon investment"; Max = "full ascension, at A6, on
every character ranked". Verbatim quotes + URL in the definitions file.

## Anything unusual
- **Sera (Kaiden)** is the only conditional-by-equipment line: High is T2
  *with signature* / T3 without; Max is T2 at A6 *with signature* ("S" =
  Signature Weapon per Kaiden's legend — Ophelia's signature, per her
  guide text). Recorded with first-listed tier + note.
- **Penny** carries the (E) flag at all three levels; **Jiwon** carries
  (P) at Low and on her High second entry; **Ellie** carries (E) at High.
  Kaiden's legend: E = "Expert: mechanically demanding", P = "Partner:
  needs a specific teammate" (the harvest guide's glosses differ slightly;
  flags were recorded as printed).
- PG covers only 23/29 in the endgame modes (see above) — expected per
  the guide, not a format change.
- No source was unreachable; no page format had changed in a way that
  hid the required fields. One naming note: Kaiden's roster order/spelling
  matched the 29-character roster exactly.

## Kaiden Tier Position lines — verbatim as rendered (audit trail)
- mina: `Low Investment, T2, High Investment, T0.5, A4, High Investment, T1, A2, Max Investment, T0.5, A6`
- penny: `Low Investment, T3, E, High Investment, T2, A1 E, Max Investment, T1, A6 E`
- narae: `Low Investment, T0, High Investment, T0, Max Investment, T0, A6`
- sangun: `Low Investment, T4, High Investment, T4, Max Investment, T3, A6`
- jiwon: `Low Investment, T3, P, High Investment, T1, A4, High Investment, T2, A2 P, Max Investment, T1, A6`
- gabi: `Low Investment, T3, High Investment, T1, A4, High Investment, T2, A2, Max Investment, T1, A6`
- isabella: `Low Investment, T1, High Investment, T0.5, A2, Max Investment, T0.5, A6`
- flare: `Low Investment, T3, High Investment, T2, A2, Max Investment, T1, A6`
- chloe: *(no Tier Position section — "Review coming soon")*
- ophelia: `Low Investment, T2, High Investment, T1, A2, Max Investment, T1, A6`
- esther: `Low Investment, T2, High Investment, T1, Max Investment, T1, A6`
- nagi: `Low Investment, T3, High Investment, T2, A2, Max Investment, T1, A6`
- brisshell: `Low Investment, T1, High Investment, T0.5, A1, Max Investment, T0.5, A6`
- mabel: `Low Investment, T2, High Investment, T1, A4, Max Investment, T1, A6`
- verna-summer-dive: `Low Investment, T0.5, High Investment, T0.5, Max Investment, T0.5, A6`
- francis-summer-dive: `Low Investment, T2, High Investment, T0.5, A2, High Investment, T1, A1, Max Investment, T0, A6`
- vivian: `Low Investment, T2, High Investment, T0.5, A4, High Investment, T1, A2, Max Investment, T0.5, A6`
- verna: `Low Investment, T3, High Investment, T3, Max Investment, T3, A6`
- cloud: `Low Investment, T2, High Investment, T2, Max Investment, T2, A6`
- francis: `Low Investment, T0.5, High Investment, T0.5, Max Investment, T0.5, A6`
- yeonhwa: `Low Investment, T0.5, High Investment, T0.5, Max Investment, T0.5, A6`
- daisy: `Low Investment, T4, High Investment, T4, Max Investment, T4, A6`
- leeho: `Low Investment, T3, High Investment, T3, Max Investment, T3, A6`
- benjamin: `Low Investment, T1, High Investment, T1, Max Investment, T1, A6`
- sera: `Low Investment, T3, High Investment, T2, S, with signature, High Investment, T3, Max Investment, T2, A6 S, with signature`
- ellie: `Low Investment, T2, High Investment, T2, E, Max Investment, T2, A6`
- reina: `Low Investment, T2, High Investment, T2, Max Investment, T2, A6`
- bonnie: `Low Investment, T3, High Investment, T3, Max Investment, T3, A6`
- angel: `Low Investment, T2, High Investment, T2, Max Investment, T2, A6`

## Scope note
This A-read covers the three **voting** sources plus T2.3/T2.4, per the
task. Codes, roster/banners and the check-only refresh for the same sweep
were staged by a parallel worker in this same directory (see
`codes-candidates.md`, `banners-note.md`, `checkonly-refresh.md`). All
merges into `data/` still wait for the B-read diffs.
