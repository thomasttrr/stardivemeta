# Patch-watch — 2026-11-07T0900-sweep-am

Checked: Sat 2026-11-07 ~09:00 ET (14:00 UTC), sweep `2026-11-07T0900-sweep-am`
Baseline: `data/game.toml` `current_version = "1.4"` (released 2026-09-30)
Question: has a version newer than 1.4 actually been RELEASED (live /
downloadable, patch notes with a release date ≤ today)? A preview, teaser,
roadmap, or scheduled date does not count as a release.

## Verdict

**No new version released — still 1.4**

Version 1.5 has only been previewed / scheduled for 2026-11-10. Every
primary channel still lists 1.4 (store build 1.04.10) as current.

Strongest evidence: Kaiden, "Mongil Developer Lounge Talk: November 5
Before 1.5" (published 2026-10-30) —
https://www.kaiden.gg/blog/mongil-november-5-developer-lounge-talk-notice/

> "The next Developer Lounge Talk for Mongil: Star Dive streams on
> November 5, 2026 at 10:00 (UTC), five days before the Version 1.5
> update on November 10 (Tue)."

`data/game.toml` was NOT edited.

## Channels checked

### 1. App Store — version history
- URL: https://apps.apple.com/us/app/mongil-star-dive/id6483252725
- Read via `browser.open`, crawled 2026-11-07 13:57 UTC.
- Current version shown: "Version 1.04.10 Oct 28" (the App Store build
  number for the 1.4 line). What's New / featured event still reads
  "1.4 Version Major Update — The Half Anniversary Festival Begins! —
  New Characters: Isabella and Chloe — Character Balance Adjustments".
- No 1.05.x build and no 1.5 changelog appear.

### 2. Google Play — version / What's New (via search index)
- Listing states "Updated on Oct 27, 2026" and What's New = "1.4
  Version Major Update" with the same 1.4 bullet list. TapTap and
  Uptodown likewise show 1.04.10 (Oct 28–29, 2026) as latest.
  APKFab and QooApp likewise show 1.04.10 (Oct 28). No 1.5 build
  or changelog.

### 3. Kaiden — news page / hub
- URL: https://www.kaiden.gg/mongil/ (read 2026-11-07 ~14:00 UTC).
- Latest items: Oct 30, 2026 "Mongil Nightmarch: Esther's Halloween
  Story and Events"; Oct 30, 2026 "Mongil Developer Lounge Talk:
  November 5 Before 1.5"; Oct 21, 2026 Chloe items. No Kaiden news
  item reports a 1.5 release; the Nov 5 article dates the Version
  1.5 update to November 10.

### 4. Netmarble forum — index (UNREADABLE for content)
- URL: https://forum.netmarble.com/
- `browser.open` returned only the forum game-selector index
  (crawl dated 2026-11-05) with no notice list. No data was guessed
  from it; the aggregators above link each official notice and none
  announces a 1.5 release.

### 5. Supporting aggregators (corroboration only)
- Prydwen banners — "Last updated: 02/November/2026". "Current
  Character Banners: Version 1.4, Oct 21, 2026 – Nov 10, 2026".
  Lou and M.A.D listed under Upcoming Banners as Version 1.5 —
  upcoming, not released.

## Released vs previewed — summary
- RELEASED ≤ 2026-11-07: Version 1.4 only (2026-09-30), plus
  in-version updates: Chloe update 2026-10-20/21 and the 1.04.10
  hotfix build 2026-10-27/28. None carries a new version number
  above 1.4.
- PREVIEWED / SCHEDULED only: Version 1.5 — preview stream Nov 5,
  2026, 10:00 UTC (held); update itself dated Nov 10, 2026.
- Therefore: no patch-watch trigger. No `game.toml` update, no
  staleness re-evaluation, and no extra A+B sweep is owed.
