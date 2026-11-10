# Patch-watch — 2026-11-10T0900-sweep-am

Checked: Tue 2026-11-10 ~09:00 ET (14:00 UTC), sweep `2026-11-10T0900-sweep-am`
Baseline: `data/game.toml` `current_version = "1.4"` (released 2026-09-30)
Question: has a version newer than 1.4 actually been RELEASED (live /
downloadable, patch notes with a release date ≤ today)? A preview, teaser,
roadmap, or scheduled date does not count as a release.

## Verdict

**No new version released — still 1.4**

Version 1.5 has NOT yet been released as of this sweep. It remains
scheduled only: Kaiden dates the update to November 10 (Tue), it has
also been described as November 11 KST, and the official social
channels posted a maintenance notice on 2026-11-09 for scheduled
downtime on November 10–11, 2026 — a future maintenance window, not a
release. Every primary channel still lists 1.4 (store build 1.04.10)
as current, and no 1.5 patch notes or 1.05.x build appear anywhere.

Strongest evidence: App Store page, crawled fresh 2026-11-10 14:01 UTC,
still shows "Version 1.04.10 Oct 28" with the 1.4 What's New text (see
channel 1 below).

`data/game.toml` was NOT edited.

## Channels checked

### 1. App Store — version history
- URL: https://apps.apple.com/us/app/mongil-star-dive/id6483252725
- Read via `browser.open`, crawled 2026-11-10 14:01 UTC.
- Current version shown: "Version 1.04.10 Oct 28" (the App Store build
  number for the 1.4 line). What's New / featured event still reads
  "1.4 Version Major Update — The Half Anniversary Festival Begins! —
  New Characters: Isabella and Chloe — Character Balance Adjustments".
- No 1.05.x build and no 1.5 changelog appear.

### 2. Kaiden — news page / hub
- URL: https://www.kaiden.gg/mongil/ (read 2026-11-10 ~14:01 UTC).
- Latest News items are unchanged: Nov 3, 2026 "Mongil Bonus Time and
  Break the Ice Run to November 10"; Oct 30, 2026 "Mongil Nightmarch:
  Esther's Halloween Story and Events"; Oct 30, 2026 "Mongil Developer
  Lounge Talk: November 5 Before 1.5". No Kaiden news item reports a
  1.5 release. The Oct 30 article (published 2026-10-30) —
  https://www.kaiden.gg/blog/mongil-november-5-developer-lounge-talk-notice/

  > "The next Developer Lounge Talk for Mongil: Star Dive streams on
  > November 5, 2026 at 10:00 (UTC), five days before the Version 1.5
  > update on November 10 (Tue)."

  That is a scheduled date, quoted before the fact — not a release.

### 3. Official social — maintenance notice (scheduled, not released)
- Search 2026-11-10 for "Mongil Star Dive version 1.5 update November
  2026 released maintenance" and for the 1.5 maintenance / patch-notes
  notice found no official Netmarble 1.5 patch-notes or release notice
  with today's date, and no new-character (M.A.D / Lou) release post.
- What it did find: official Facebook and Instagram "Maintenance
  Notice" posts, both posted 2026-11-09 ~15:00, announcing scheduled
  server downtime on November 10–11, 2026:
  - https://www.facebook.com/permalink.php?story_fbid=pfbid023Kef2qkcvQDQueXQHtVpKBYyCNVJrjozTPMriQU4vzWDQxZA6Ca3Hyphx5HHewCal&id=61574474293883
  - https://www.instagram.com/p/DfidhysDKdq/
  The posts are image notices (no patch-notes text was readable), and
  they announce a *future* maintenance window — consistent with a
  Nov 11 KST rollout that has not happened yet at 09:00 ET Nov 10.

### 4. Netmarble forum — index (UNREADABLE for content)
- URL: https://forum.netmarble.com/
- `browser.open` returned only the forum game-selector index
  (crawl dated 2026-11-05) with no notice list. No data was guessed
  from it; the aggregators below link each official notice and none
  announces a 1.5 release.

### 5. Supporting aggregators (corroboration only)
- Prydwen banners — "Last updated: 02/November/2026". "Current
  Character Banners: Version 1.4, Oct 21, 2026 – Nov 10, 2026".
  Lou and M.A.D listed under Upcoming Banners as Version 1.5 —
  upcoming, not released.
- Pocket Gamer tier list now prints "Version: 1.04.10" (page updated
  Nov 7, 2026) — still the 1.4 line.
- Uptodown mirror lists latest = 1.04.10 (Oct 29, 2026). No 1.5 build.

## Released vs previewed — summary
- RELEASED ≤ 2026-11-10 (09:00 ET): Version 1.4 only (2026-09-30),
  plus in-version updates (Chloe, Oct 20/21).
- PREVIEWED / SCHEDULED ONLY: Version 1.5 — update dated
  November 10, 2026 per Kaiden (also described as Nov 11 KST), with
  an official maintenance window announced for Nov 10–11. Not
  released as of this sweep: no new store build, no patch notes,
  no release report on any channel checked.
- The next sweep must re-check: once 1.5 actually goes live (store
  build changes / patch notes publish), the patch procedure triggers
  (game.toml update, staleness re-evaluation, extra full A+B sweep
  within 24 h, same-day report).
