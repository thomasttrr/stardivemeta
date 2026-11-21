# Patch-watch — 2026-11-21T0900-sweep-am

Checked: Sat 2026-11-21 ~09:00 ET (2026-11-21 13:58–13:59 UTC), sweep `2026-11-21T0900-sweep-am`
Baseline: `data/game.toml` in this worktree `current_version = "1.5"` (released 2026-11-10, store build 1.05.00, new character M.A.D., `balance_changes = false`).
Question: has a version newer than 1.5 actually been RELEASED (live / downloadable, release date ≤ today 2026-11-21 ET), and has Lou (second announced 1.5 character, also listed as Rue) released? A preview, teaser, roadmap, or scheduled date does not count as a release.

## Verdict

**NO NEWER VERSION — current version remains 1.5, store build 1.05.00. Lou NOT released.**

The App Store page, crawled fresh 2026-11-21 13:58 UTC, still shows "Version 1.05.00 Nov 10" with the identical 1.5 What's New text and the "New Char. 「M.A.D.」 is Here!" header — no newer build since the 2026-11-10 release. Google Play corroborates: its What's New is the same 1.5 text, and third-party mirrors (APKFab, Uptodown, TapTap) all list 1.05.00 as latest. No announcement, store listing, or press article for any version newer than 1.5 was found on any channel checked; Netmarble's own news list still leads with the Nov 12, 2026 Version 1.5 / M.A.D. release.

Follow-up on the 2026-11-19 notice: the official Facebook / Instagram "Upcoming Patch Notice" post dated 2026-11-19 (patch scheduled for later on 11/19) has NOT landed as a store build by 2026-11-21 — both stores still list only build 1.05.00 (Nov 10) with unchanged What's New text, and no newer version number, date, or changelog has appeared. Whatever that patch was, it produced no new store version ≤ 2026-11-21.

Lou has NOT released: no Lou/Rue release announcement, banner start, or store evidence found in this sweep's searches. Freshest corroboration is Prydwen's banners page (last updated 18/November/2026, crawled this sweep): current Version 1.5 window is Nov 11 – Dec 1, 2026 (M.A.D.), and under Upcoming Banners — "Rue is the upcoming new limited 5-stars of Version 1.5. More information on Rue will be added as soon as it is announced." GamsGo's tier list (updated Nov 11) likewise still says M.A.D and Rue "aren't on the grid yet". That is anticipation, not a release. The only newer official social item is a Developer Lounge Talk promo posted 2026-11-20 for a livestream scheduled 2026-11-25 — not a version or character release.

`data/game.toml` was NOT edited (patch-watch-only assignment).

## Channels checked

### 1. App Store — version history (PRIMARY EVIDENCE)
- URL: https://apps.apple.com/us/app/mongil-star-dive/id6483252725
- Read via `browser.open`, crawled 2026-11-21 13:58 UTC (2026-11-21 08:58 ET).
- Current version shown: "Version 1.05.00 Nov 10" — consistent with the 2026-11-10 release; no newer build. App size still 532.7 MB.
- Page header still reads "New Char. 「M.A.D.」 is Here!" and the Events / What's New block is unchanged, verbatim:

  > "1.5 Version Update — New Character: M.A.D. — Main Story: Episode 「Light From a Fallen Star」 Added — Special Event Story: 「A Flame Chasing Freedom」 Added — New Content: Honor Ranking, Special Conquest, and Archive Added — Friends Feature Improved"

- No character balance adjustments listed (unchanged from the 1.5 record).

### 2. Google Play — listing
- URL: https://play.google.com/store/apps/details?id=com.netmarble.monster2
- Read via `browser.open`, crawled 2026-11-21 13:59 UTC; version/date fields were not exposed in the fetched text, so the listing was corroborated via search-index copy of the same Play page plus mirrors.
- Search-index copy of the Play listing: "Updated on, Nov 6, 2026" with What's New identical to the App Store 1.5 text above (M.A.D. / Light From a Fallen Star / A Flame Chasing Freedom / Honor Ranking, Special Conquest, and Archive / Friends Feature Improved). No newer What's New.
- Mirrors agree on latest build: APKFab lists 1.05.00 as latest (updated November 13, 2026); Uptodown lists 1.05.00 (date Nov 16, 2026) with older versions 1.04.10 / 1.04.00 below it; TapTap lists 1.05.00 (updated 2026-11-11). No build newer than 1.05.00 anywhere.

### 3. Official Netmarble news / forum — version announcements
- Netmarble company news list (https://mcompany.netmarble.com/en/news/list?pageNo=1, crawled this sweep): latest MONGIL item is still Nov 12, 2026 — "MONGIL: STAR DIVE ADDS NEW 5-STAR CHARACTER M.A.D. IN VERSION 1.5 UPDATE" (release dated November 11, 2026). No version newer than 1.5 announced.
- Official forum search (`site:forum.netmarble.com MONGIL STAR DIVE update November 2026`): no results found — no forum announcement for a newer version surfaced.
- News-vertical search for a 1.6 / newer version: no release hit beyond known 1.5 coverage (Inven Global Nov 10, thisisgamesea / wespeakmedia2 Nov 12 republications of the 1.5 press release) and the older Oct 21 Chloe update. No 1.6 announcement, date, or release.

### 4. Searches — Nov 19 patch follow-up, Lou/Rue
- Patch search ("MONGIL STAR DIVE patch notice November 19 2026 update", news vertical): the only Mongil hit is the official Facebook "Upcoming Patch Notice" post dated 2026-11-19 announcing a patch scheduled for later on 11/19 (mirrored on Instagram the same day). Two days later neither store shows a newer build, version, or What's New — see Verdict. No patch notes for any version newer than 1.5.
- Lou search ("MONGIL STAR DIVE Lou release banner November 2026" and "\"MONGIL: STAR DIVE\" Lou Rue banner starts released"): no Lou/Rue release announcement or banner start found. Prydwen banners (updated 18/November/2026) still lists Rue under Upcoming Banners for Version 1.5, current window Nov 11 – Dec 1, 2026 = M.A.D. GamsGo (updated Nov 11) still describes Rue (Lou) as the second-half 1.5 banner and notes she is not yet on the grid.

## Released vs previewed — summary
- RELEASED ≤ 2026-11-21 ET: **Version 1.5** — store build **1.05.00**, released **2026-11-10** (confirmed live again this sweep: App Store "Version 1.05.00 Nov 10" at the 2026-11-21 08:58 ET crawl; Google Play What's New is the same 1.5 text). Content: M.A.D., "Light From a Fallen Star", "A Flame Chasing Freedom" (Nov 11 – Dec 2), Honor Ranking / Special Conquest / Archive, Friends Feature Improved. Previously: Version 1.4 (2026-09-30) plus in-version Chloe (Oct 20/21).
- NOTHING NEWER THAN 1.5: no 1.6 (or later) announcement, date, store build, or release found on the App Store, Google Play, Netmarble news, the official forum search, or official social. The Nov 19 "Upcoming Patch Notice" has not produced a new store build as of Nov 21.
- PREVIEWED / NOT YET RELEASED: Lou (Rue) — still an upcoming Version 1.5 banner per Prydwen (updated 18/November/2026); not in the released 1.5 changelog or press release, and no release announcement found. A Developer Lounge Talk livestream is scheduled for 2026-11-25 (promo posted 2026-11-20) — future event, not a release.
