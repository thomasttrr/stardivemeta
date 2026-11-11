# Patch-watch — 2026-11-11T1200-sweep-am

Checked: Wed 2026-11-11 ~12:00 ET (2026-11-11 17:04 UTC), sweep `2026-11-11T1200-sweep-am`
Baseline: `data/game.toml` on main `current_version = "1.5"` (released 2026-11-10, store build 1.05.00, new character M.A.D., `balance_changes = false`; merged this morning from the record found by sweep `2026-11-10T2345-sweep-pm`).
Question: has a version newer than 1.5 actually been RELEASED (live /
downloadable, patch notes with a release date ≤ today 2026-11-11 ET),
has official 1.5 patch-notes / press material appeared since this
morning's sweep that changes the `balance_changes = false` record,
and has Lou (second announced 1.5 character) released? A preview,
teaser, roadmap, or scheduled date does not count as a release.

## Verdict

**NO NEWER VERSION — current version remains 1.5, store build 1.05.00. Lou NOT released. No new balance news — this morning's damage-formula flag stands unchanged.**

The App Store page, crawled fresh 2026-11-11 17:04 UTC, still shows
"Version 1.05.00 17h ago" with the identical 1.5 What's New text and
the "New Char. 「M.A.D.」 is Here!" header — no newer build since
this morning's sweep (which read "14h ago"). No announcement, store
listing, or press article for any version newer than 1.5 was found
on any channel checked; Version 1.6 exists only as the old
September roadmap tease (festival / New Year story, character
epilogues) with no new announcement, date, or release (channel 3).

Balance picture is UNCHANGED since this morning: the only official
1.5 release text beyond the store changelog is still the Inven
Global Netmarble press release published 2026-11-11 05:57 UTC. It
lists **no character balance adjustments** — no per-character
buffs/nerfs, and no "Character Balance Adjustments" line like the
1.4 changelog carried. Its one combat-system detail — "the final
damage calculation method has been improved" (plus the 'Link
Chain Cat' level restriction removed) — is unchanged and remains
flagged exactly as this morning: a system-wide formula
improvement, not a character balance-adjustment list, so it does
**not** overturn `balance_changes = false`. No newer official
text found in this sweep changes that picture.

Lou has NOT released: Prydwen's banner page, updated
11/November/2026 and read directly this sweep, still lists Lou
under Upcoming Banners for Version 1.5 ("More information on Lou
will be added as soon as it is announced"), and the current 1.5
banner window (Nov 11 – Dec 1, 2026) is M.A.D.'s (channel 4).

`data/game.toml` and `data/sources.toml` were NOT edited
(patch-watch-only assignment).

## Channels checked

### 1. App Store — version history (PRIMARY EVIDENCE, store state confirmed)
- URL: https://apps.apple.com/us/app/mongil-star-dive/id6483252725
- Read via `browser.open`, crawled 2026-11-11 17:04 UTC
  (2026-11-11 12:04 ET).
- Current version shown: "Version 1.05.00 17h ago" — consistent
  with the 2026-11-10 ET release; no newer build. App size still
  532.7 MB.
- Page header still reads "New Char. 「M.A.D.」 is Here!" and the
  Events / What's New block is unchanged:

  > "1.5 Version Update — New Character: M.A.D. — Main Story:
  > Episode 「Light From a Fallen Star」 Added — Special Event Story:
  > 「A Flame Chasing Freedom」 Added — New Content: Honor Ranking,
  > Special Conquest, and Archive Added — Friends Feature Improved"

- The What's New text still lists **no character balance
  adjustments**.

### 2. Kaiden — news page / hub (still no 1.5 release coverage)
- URL: https://www.kaiden.gg/mongil/ (read via `browser.open`
  during this sweep's codes-scout, crawled 2026-11-11 17:03 UTC).
- Hub stamp still "Last updated: October 19, 2026". Latest News is
  unchanged: Nov 3, 2026 "Mongil Bonus Time and Break the Ice Run
  to November 10"; Oct 30, 2026 "Mongil Nightmarch: Esther's
  Halloween Story and Events"; Oct 30, 2026 "Mongil Developer
  Lounge Talk: November 5 Before 1.5". No Kaiden news item reports
  the 1.5 release yet — Kaiden is still lagging the stores, not
  contradicting them.

### 3. Official channels — searches for 1.6, post-1.5 patch notes, and balance news
- Newer-than-1.5 search (2026-11-11 ~12:00 ET, "MONGIL STAR DIVE
  version 1.6 announcement release"): no new 1.6 announcement,
  date, or release. The only 1.6 mention remains the September
  roadmap tease reproduced 2026-09-10 — "Version 1.6 will expand
  the game with a special festival and New Year story, character
  epilogues, and new intimacy and interaction content"
  (https://thisisgamesea.com/news/pr-news/mongil-star-dive-vivian-latest-update/).
  A roadmap tease is not a release.
- Post-1.5 patch-notes search ("MONGIL STAR DIVE patch notes
  after version 1.5 update November 2026"): top game hits were
  the same Inven Global 1.5 release article (published
  2026-11-11T05:57:50+00:00 — this morning's find, not a new one),
  the Prydwen banner page, and official social graphics for an
  "Upcoming Patch Notice" dated 2026-11-11 with no version number
  or release text in the search copy. No patch notes for any
  version newer than 1.5, and no post-1.5 patch release, were
  found.
- Balance-specific search ("MONGIL STAR DIVE 1.5 character
  balance adjustments final damage calculation"): the only 1.5
  hit is again the same Inven Global article, carrying the same
  "final damage calculation method has been improved" line and
  no character balance list. Remaining hits were the 1.4 balance
  record (store / Google Play changelog "Character Balance
  Adjustments") and general half-anniversary balance interviews.
  No source states that 1.5 adjusted any character, and nothing
  newer than this morning's article changes the picture.
- Inven Global article (for the record, unchanged):
  https://www.invenglobal.com/articles/28456/mongil-star-dive-adds-new-5-star-character-in-version-15-update
  — Netmarble announced on November 11 (KST) that it has released
  the Version 1.5 update, introducing 5-star Fire-element assassin
  'M.A.D.' (real name Miadrin); event story 'Flames Pursuing
  Freedom' runs November 11 – December 2; main story episode
  'Light from a Fallen Star'; Honor Ranking; Race revamp; Friend
  Ranking tab; final-damage-calculation improvement; 'Link Chain
  Cat' level restriction removed; Archive tabs. **No character
  balance adjustments are mentioned anywhere in the article.**

### 4. Lou — second announced 1.5 character (NOT released)
- Prydwen banners — https://www.prydwen.gg/other-games/mongil-star-dive/banners
  (read directly via `browser.open`, crawled 2026-11-11 16:32 UTC;
  page stamp "Last updated: 11/November/2026"): Current Character
  Banners = Version 1.5, Nov 11, 2026 – Dec 1, 2026 (M.A.D.);
  Upcoming Banners: "Lou is the upcoming new limited 5-stars of
  Version 1.5. More information on Lou will be added as soon as
  it is announced."
- The Inven Global 1.5 release article (channel 3) names only
  M.A.D.; Lou is not in the 1.05.00 store changelog (channel 1)
  either. On the 1.4 pattern (Isabella at release, Chloe weeks
  later), Lou remains expected later within 1.5. No Lou release
  evidence was found in this sweep.

## Released vs previewed — summary
- RELEASED ≤ 2026-11-11 ET: **Version 1.5** — store build
  **1.05.00**, released **2026-11-10** (confirmed live again this
  sweep: App Store "Version 1.05.00 17h ago" at the 2026-11-11
  12:04 ET crawl; Netmarble press release via Inven Global,
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
  1.5 banner per Prydwen (updated 11/November/2026, read directly
  this sweep); not in the released 1.5 changelog or press
  release. Kaiden still shows pre-release / 1.4 state in Latest
  News and is lagging the stores, not contradicting them.
- Next steps (main sweep, not this patch-watch): the game.toml
  update to 1.5 was already merged this morning — do not
  duplicate it. The Inven press release's "final damage
  calculation method has been improved" line remains flagged for
  Tom / the main sweep exactly as this morning; `balance_changes
  = false` still stands on the character-balance reading.
