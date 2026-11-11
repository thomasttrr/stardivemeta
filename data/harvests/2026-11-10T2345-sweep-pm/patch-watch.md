# Patch-watch — 2026-11-10T2345-sweep-pm

Checked: Tue 2026-11-10 ~23:45 ET (2026-11-11 04:43–04:45 UTC), sweep `2026-11-10T2345-sweep-pm`
Baseline: `data/game.toml` `current_version = "1.4"` (released 2026-09-30, store build 1.04.10)
Question: has a version newer than 1.4 actually been RELEASED (live /
downloadable, patch notes with a release date ≤ today 2026-11-10 ET)?
A preview, teaser, roadmap, or scheduled date does not count as a release.

## Verdict

**RELEASED — Version 1.5, store build 1.05.00, on 2026-11-10**

Version 1.5 went live between the AM sweep (09:00 ET, still 1.4) and
this PM sweep. The App Store page, crawled fresh 2026-11-11 04:43 UTC,
now shows "Version 1.05.00 5h ago" with the full 1.5 changelog and a
"New Char. 「M.A.D.」 is Here!" header (see channel 1). APKFab's
downloadable 1.05.00 build is likewise dated November 10, 2026
(channel 5). That is a live, downloadable build with a release date
of 2026-11-10 ET — not a preview or a scheduled date.

`data/game.toml` was NOT edited (patch-watch-only assignment). The
patch procedure now triggers for the main sweep: game.toml update to
1.5, staleness re-evaluation, extra full A+B sweep within 24 h, and a
same-day report.

## Channels checked

### 1. App Store — version history (PRIMARY EVIDENCE)
- URL: https://apps.apple.com/us/app/mongil-star-dive/id6483252725
- Read via `browser.open`, crawled 2026-11-11 04:43 UTC
  (2026-11-10 23:43 ET).
- Current version shown: "Version 1.05.00 5h ago" — at crawl time
  that puts the release on 2026-11-10 ET, after the Nov 10–11
  maintenance window announced 2026-11-09. App size is now 532.7 MB
  (was 526.4 MB on the 1.04.10 listing).
- Page header now reads "New Char. 「M.A.D.」 is Here!" and the
  Events / What's New block reads:

  > "1.5 Version Update — New Character: M.A.D. — Main Story:
  > Episode 「Light From a Fallen Star」 Added — Special Event Story:
  > 「A Flame Chasing Freedom」 Added — New Content: Honor Ranking,
  > Special Conquest, and Archive Added — Friends Feature Improved"

- Contrast with the AM sweep, same URL crawled 2026-11-10 14:01 UTC:
  "Version 1.04.10 Oct 28" with the 1.4 What's New text. The store
  build has changed since AM — this is the release event.

### 2. Kaiden — news page / hub (no release news yet — lagging)
- URL: https://www.kaiden.gg/mongil/ (read 2026-11-11 04:45 UTC).
- Hub stamp still "Last updated: October 19, 2026". Latest News is
  unchanged from AM: Nov 3, 2026 "Mongil Bonus Time and Break the
  Ice Run to November 10"; Oct 30, 2026 "Mongil Nightmarch:
  Esther's Halloween Story and Events"; Oct 30, 2026 "Mongil
  Developer Lounge Talk: November 5 Before 1.5". No Kaiden news
  item reports a 1.5 release yet.
- The Oct 30 article still on the hub —
  https://www.kaiden.gg/blog/mongil-november-5-developer-lounge-talk-notice/
  — dates "the Version 1.5 update on November 10 (Tue)", which is
  the scheduled date the release has now met.

### 3. Official channels — Netmarble forum, Facebook, Instagram, X
- Searches 2026-11-10 ~23:45 ET for the 1.5 release / patch notes /
  maintenance completion, for a Netmarble "launches Version 1.5"
  press release, and `site:forum.netmarble.com` for a 1.5 notice:
  no official forum patch-notes text and no press-release article
  found as of this sweep (unlike 1.4, which had same-day Capsule
  Computers / Inven Global release articles on 2026-09-30). Do not
  treat the forum as read — the search returned no results, and no
  data was guessed from it.
- Official Instagram (@stardive_en): a post advertising the
  "1.5 Version Update" titled "Light From a Fallen Star", indexed
  as posted 2026-11-11 (KST go-live timing, = Nov 10 ET evening):
  https://www.instagram.com/p/DfldJvTOWIe/
- Official Facebook: an "Events" post indexed as posted 2026-11-11
  with event content dated 2026-11-10:
  https://www.facebook.com/permalink.php?story_fbid=pfbid031RM6NADBg7pL7eadeqHF2mCFCQcU6nB7tc2Wmkcnn5UqQz6vL6YGjRfwC26NWPdXl&id=61574474293883
- The 2026-11-09 maintenance notices for the Nov 10–11 window
  (recorded in the AM patch-watch) are now in the past; no separate
  "maintenance completed" post was found in search, but the live
  store build in channel 1 supersedes that check.

### 4. Google Play — What's New (corroboration via search index)
- URL: https://play.google.com/store/apps/details?id=com.netmarble.monster2
- Direct `browser.open` (crawled 2026-11-11 04:45 UTC) returned the
  store description only — no What's New / Updated-on fields in the
  extracted text, so Google Play is NOT primary evidence here.
- The search-index copy of the same listing does show What's New =
  the identical 1.5 text ("1.5 Version Update — New Character:
  M.A.D. …") with "Updated on, Nov 6, 2026" — consistent with a
  staged store changelog going live for the Nov 10 release.

### 5. Supporting aggregators
- APKFab — https://apkfab.com/mongil-star-dive/com.netmarble.monster2
  (crawled 2026-11-11 04:45 UTC): the version listing gives
  "MONGIL: STAR DIVE 1.05.00 XAPK — November 10, 2026 — 292.77 MB"
  with the same 1.5 What's New text, directly above 1.04.10
  (October 28, 2026). A downloadable 1.05.00 build dated
  2026-11-10. Note: APKFab's header line "latest version …
  was updated on October 27, 2026" conflicts with its own version
  listing and is treated as a stale header; the dated version
  listing is the operative record.
- Prydwen banners — https://www.prydwen.gg/other-games/mongil-star-dive/banners
  (crawled 2026-11-11 03:18 UTC): "Last updated: 10/November/2026",
  still lists Lou and M.A.D. under Upcoming Banners as Version 1.5
  — a pre-release snapshot, not yet updated for the release.
- Pocket Gamer tier list — https://www.pocketgamer.com/mongil-star-dive/tier-list/
  (crawled 2026-11-11 04:45 UTC): header still reads "Updated on
  November 7th, 2026 — Version: 1.04.10 — Latest Addition:
  Isabella, Chloe". Not yet updated for 1.5.
- Uptodown mirror (search index) still lists latest = 1.04.10
  (Oct 29, 2026). Mirror lag only.

## Released vs previewed — summary
- RELEASED ≤ 2026-11-10 ET: **Version 1.5** — store build
  **1.05.00**, released **2026-11-10** (App Store "Version 1.05.00
  5h ago" at the 2026-11-10 23:43 ET crawl; APKFab 1.05.00 build
  dated November 10, 2026). Content in the released changelog:
  new character M.A.D., main story episode "Light From a Fallen
  Star", event story "A Flame Chasing Freedom", Honor Ranking /
  Special Conquest / Archive, Friends Feature Improved.
  Previously released: Version 1.4 only (2026-09-30), plus
  in-version updates (Chloe, Oct 20/21).
- PREVIEWED / NOT YET EVIDENCED AS RELEASED: Lou — named as a
  second new Version 1.5 character by Prydwen and the Kaiden
  roadmap coverage, but Lou is NOT in the 1.05.00 changelog.
  On the 1.4 pattern (Isabella at release, Chloe weeks later),
  Lou is expected later within 1.5; no Lou release evidence was
  found in this sweep. No official 1.5 patch-notes text or press
  release was found yet either — Kaiden, Pocket Gamer, and
  Prydwen all still show pre-release / 1.4 state and are lagging
  the stores, not contradicting them.
- Next steps (main sweep, not this patch-watch): update
  `data/game.toml` to 1.5 with the 2026-11-10 release date and
  the M.A.D. roster change, re-evaluate staleness, run the extra
  full A+B sweep within 24 h, and send the same-day patch report.
