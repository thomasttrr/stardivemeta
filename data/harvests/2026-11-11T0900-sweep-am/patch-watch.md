# Patch-watch — 2026-11-11T0900-sweep-am

Checked: Wed 2026-11-11 ~09:00 ET (2026-11-11 13:58 UTC), sweep `2026-11-11T0900-sweep-am`
Baseline: `data/game.toml` on main `current_version = "1.4"` (released 2026-09-30, store build 1.04.10). Version 1.5 was found RELEASED 2026-11-10 by the PM sweep `2026-11-10T2345-sweep-pm` (store build 1.05.00, new character M.A.D., `balance_changes = false`); the game.toml update to 1.5 is already proposed on PR #22 (branch `patch/2026-11-10-v15-game-version`, labelled needs-tom) and is NOT redone here.
Question: has a version newer than 1.5 actually been RELEASED (live /
downloadable, patch notes with a release date ≤ today 2026-11-11 ET),
has official 1.5 patch-notes / press material appeared that changes
the `balance_changes = false` record, and has Lou (second announced
1.5 character) released? A preview, teaser, roadmap, or scheduled
date does not count as a release.

## Verdict

**NO NEWER VERSION — current version remains 1.5, store build 1.05.00. Lou NOT released. One balance nuance flagged below.**

The App Store page, crawled fresh 2026-11-11 13:58 UTC, still shows
"Version 1.05.00 14h ago" with the identical 1.5 What's New text and
the "New Char. 「M.A.D.」 is Here!" header (channel 1) — no newer
build since the PM sweep. No announcement, store listing, or press
article for any version newer than 1.5 was found on any channel
checked; Version 1.6 exists only as the old September roadmap tease
(festival / New Year story, character epilogues) with no new
announcement, date, or release (channel 3).

NEW since the PM sweep: an official Netmarble press-release article
reproduced by Inven Global, published 2026-11-11 05:57 UTC,
announcing the 1.5 release (channel 3). It lists **no character
balance adjustments** — no per-character buffs/nerfs, and no
"Character Balance Adjustments" line like the 1.4 changelog carried.
It does add one combat-system detail not in the store changelog:
"the final damage calculation method has been improved" (plus the
'Link Chain Cat' level restriction removed). That is a system-wide
formula improvement, not a character balance-adjustment list, so it
does **not** overturn `balance_changes = false` — but it is flagged
here loudly for the main sweep / Tom, because the PM record rested
on the store changelog alone and this is the first official
release text beyond it. If Tom / the method counts a damage-formula
change as a balance change, the staleness re-evaluation must be
re-run.

Lou has NOT released: Prydwen's banner page, updated 11/November/2026,
still lists Lou under Upcoming Banners for Version 1.5 ("More
information on Lou will be added as soon as it is announced"), and
the current 1.5 banner window (Nov 11 – Dec 1, 2026) is M.A.D.'s
(channel 4).

`data/game.toml` and `data/sources.toml` were NOT edited
(patch-watch-only assignment).

## Channels checked

### 1. App Store — version history (PRIMARY EVIDENCE, store state confirmed)
- URL: https://apps.apple.com/us/app/mongil-star-dive/id6483252725
- Read via `browser.open`, crawled 2026-11-11 13:58 UTC
  (2026-11-11 08:58 ET).
- Current version shown: "Version 1.05.00 14h ago" — consistent
  with the 2026-11-10 ET release found by the PM sweep; no newer
  build. App size still 532.7 MB.
- Page header still reads "New Char. 「M.A.D.」 is Here!" and the
  Events / What's New block is unchanged from the PM sweep:

  > "1.5 Version Update — New Character: M.A.D. — Main Story:
  > Episode 「Light From a Fallen Star」 Added — Special Event Story:
  > 「A Flame Chasing Freedom」 Added — New Content: Honor Ranking,
  > Special Conquest, and Archive Added — Friends Feature Improved"

- The What's New text still lists **no character balance
  adjustments**.

### 2. Kaiden — news page / hub (still no 1.5 release coverage)
- URL: https://www.kaiden.gg/mongil/ (read via `browser.open`,
  crawled 2026-11-11 13:58 UTC).
- Hub stamp still "Last updated: October 19, 2026". Latest News is
  unchanged from both Nov 10 sweeps: Nov 3, 2026 "Mongil Bonus
  Time and Break the Ice Run to November 10"; Oct 30, 2026
  "Mongil Nightmarch: Esther's Halloween Story and Events";
  Oct 30, 2026 "Mongil Developer Lounge Talk: November 5 Before
  1.5". No Kaiden news item reports the 1.5 release yet — Kaiden
  is still lagging the stores, not contradicting them.

### 3. Official channels — Netmarble press release (NEW), forum, 1.6 search
- Inven Global (Netmarble press release, reproduced) —
  https://www.invenglobal.com/articles/28456/mongil-star-dive-adds-new-5-star-character-in-version-15-update
  — published 2026-11-11T05:57:50+00:00, read via `browser.open`,
  crawled 2026-11-11 13:58 UTC. NEW since the PM sweep, which
  found no press-release article. Text: Netmarble announced on
  November 11 (KST) that it has released the Version 1.5 update,
  introducing 5-star Fire-element assassin 'M.A.D.' (real name
  Miadrin); event story 'Flames Pursuing Freedom' runs November
  11 – December 2; main story episode 'Light from a Fallen Star';
  M.A.D. pick-up recruitment; 'Honor Ranking' seasonal system;
  'Race' revamped so playable characters participate directly;
  friend system gains a Friend Ranking tab and key-claiming by
  recent login; "the final damage calculation method has been
  improved, and the level restriction on 'Link Chain Cat' has
  been removed"; 'Archive' adds Character, Artifact, and
  Decoration tabs; detailed information pointed to the official
  website and forums. **No character balance adjustments are
  mentioned anywhere in the article** — see the Verdict flag on
  the damage-calculation line.
- Official forum: searches 2026-11-11 ~09:00 ET for a Netmarble
  1.5 patch-notes / notice text, including
  `site:forum.netmarble.com` for a 1.5 notice, returned no forum
  patch-notes text (the Inven article above was the top hit).
  Do not treat the forum as read — the search returned no
  results, and no data was guessed from it.
- Newer-than-1.5 search: a search for a Version 1.6 announcement
  found no new announcement, date, or release. The only 1.6
  mention remains the September roadmap tease reproduced
  2026-09-10 — "Version 1.6 will expand the game with a special
  festival and New Year story, character epilogues, and new
  intimacy and interaction content"
  (https://thisisgamesea.com/news/pr-news/mongil-star-dive-vivian-latest-update/).
  A roadmap tease is not a release.
- Balance-specific search: a dedicated search for 1.5 balance /
  character-balance adjustments surfaced no 1.5 balance article —
  results were the 1.4 balance record (store changelog "Character
  Balance Adjustments", seven characters rebalanced) and general
  half-anniversary balance interviews. No source states that 1.5
  adjusted any character.

### 4. Lou — second announced 1.5 character (NOT released)
- Prydwen banners — https://www.prydwen.gg/other-games/mongil-star-dive/banners
  (search-index copy, last updated 11/November/2026, crawled
  2026-11-11 ~13:00 UTC): Current Character Banners = Version 1.5,
  Nov 11, 2026 – Dec 1, 2026 (M.A.D.); Upcoming Banners: "Lou is
  the upcoming new limited 5-stars of Version 1.5. More
  information on Lou will be added as soon as it is announced."
- The Inven Global 1.5 release article (channel 3) names only
  M.A.D.; Lou is not in the 1.05.00 store changelog (channel 1)
  either. On the 1.4 pattern (Isabella at release, Chloe weeks
  later), Lou remains expected later within 1.5. No Lou release
  evidence was found in this sweep.

## Released vs previewed — summary
- RELEASED ≤ 2026-11-11 ET: **Version 1.5** — store build
  **1.05.00**, released **2026-11-10** (confirmed live again this
  sweep: App Store "Version 1.05.00 14h ago" at the 2026-11-11
  08:58 ET crawl; Netmarble press release via Inven Global,
  published 2026-11-11, announcing the release). Content: new
  character M.A.D., main story episode "Light From a Fallen
  Star", event story "A Flame Chasing Freedom" (Nov 11 – Dec 2),
  Honor Ranking / Special Conquest / Archive, Friends Feature
  Improved, Race revamp, final-damage-calculation improvement,
  'Link Chain Cat' level restriction removed. **No character
  balance adjustments in any official 1.5 text found.**
  Previously released: Version 1.4 only (2026-09-30), plus
  in-version updates (Chloe, Oct 20/21).
- NOTHING NEWER THAN 1.5: no 1.6 (or later) announcement, date,
  store build, or release found on any channel checked; 1.6
  remains a September roadmap tease only.
- PREVIEWED / NOT YET RELEASED: Lou — still an upcoming Version
  1.5 banner per Prydwen (updated 11/November/2026); not in the
  released 1.5 changelog or press release. Kaiden still shows
  pre-release / 1.4 state in Latest News and is lagging the
  stores, not contradicting them.
- Next steps (main sweep, not this patch-watch): PR #22 already
  carries the game.toml update to 1.5 for Tom — do not duplicate
  it. Flag for Tom / the main sweep: the Inven press release's
  "final damage calculation method has been improved" line — the
  only combat-affecting change in official 1.5 text beyond the
  store changelog; `balance_changes = false` still stands on the
  character-balance reading, but the call is Tom's if the method
  counts a formula change as a balance change.
