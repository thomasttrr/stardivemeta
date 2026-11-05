# Patch-watch — 2026-11-05T0900-sweep-am

Checked: Thu 2026-11-05 ~09:00 ET (14:00 UTC), sweep `2026-11-05T0900-sweep-am`
Baseline: `data/game.toml` `current_version = "1.4"` (released 2026-09-30)
Question: has a version newer than 1.4 actually been RELEASED (live / downloadable,
patch notes with a release date ≤ today)? A preview, teaser, roadmap, or scheduled
date does not count as a release.

## Verdict

**No new version released — still 1.4**

Version 1.5 has only been previewed / scheduled. Every primary channel still
lists 1.4 (store build 1.04.10) as current, and the strongest dated source
places the 1.5 update in the future, on 2026-11-10.

Strongest evidence: Kaiden, "Mongil Developer Lounge Talk: November 5 Before 1.5"
(published 2026-10-30) — https://www.kaiden.gg/blog/mongil-november-5-developer-lounge-talk-notice/

> "The next Developer Lounge Talk for Mongil: Star Dive streams on
> November 5, 2026 at 10:00 (UTC), five days before the Version 1.5 update on
> November 10 (Tue). It previews the update's new content, improvements and
> new characters, and ends with a live Q&A."

`data/game.toml` was NOT edited.

## Channels checked

### 1. Netmarble forum — index (UNREADABLE via text fetch)

- URL: https://forum.netmarble.com/stardive_gl
- Attempt: `browser.open`, crawled 2026-11-05 14:00 UTC.
- Result — failure recorded exactly: the fetch returned only 2 lines,
  `# MONGIL: STAR DIVE` and `Recent Search History Delete All History`,
  with no notice list, no titles, and no dates. The forum index is
  JavaScript-rendered and does not expose its notice list to the text
  reader. No data was guessed from it.
- Workaround used: individual forum posts (below) and search-indexed
  forum content, plus the aggregators that link each official notice.

### 2. Netmarble forum — latest indexed official post

- URL: http://forum.netmarble.com/stardive_gl/view/4/719
- Title: "10/14 (Wed) Developer Lounge Talk at a Glance! - MONGIL: STAR DIVE"
- Read successfully via `browser.open` (crawled 2026-11-05 14:00 UTC).
- What it said (dates quoted from the post): "On 10/14 (Wed) at 10:00 (UTC),
  the <Developer Lounge Talk> took place!" It covers the Chloe / 10/20 update
  within 1.4, a "[v1.4 Update Survey Results Notice]", and, on 1.5, only a
  future plan: "Development is currently complete, and it is scheduled to be
  added in the Version 1.5 update." (walking mode Q&A).
- No forum post found — in search index or via this post — announcing a
  1.5 release, 1.5 maintenance, or 1.5 patch notes with a date ≤ 2026-11-05.

### 3. Netmarble official news / Newsroom

- Searched: Netmarble Newsroom (`ch.netmarble.com`) and Netmarble press
  releases for MONGIL: STAR DIVE, October–November 2026.
- Latest release announcements found:
  - 2026-09-30 — Version 1.4 launch (Isabella / Episode 7), e.g. as
    reproduced at
    https://www.capsulecomputers.com.au/2026/10/mongil-star-dive-launches-version-1-4-update-featuring-new-character-isabella/
    ("LOS ANGELES – SEPTEMBER 30, 2026 – Netmarble … has released the
    Version 1.4 update").
  - 2026-10-21 — Chloe update within 1.4 (no new version number), e.g.
    Inven Global / Capsule Computers press-release reproductions dated
    2026-10-21.
- No Netmarble press release or Newsroom item dated ≤ 2026-11-05 announces
  Version 1.5 as released. All 1.5 mentions in this channel are roadmap /
  teaser wording ("will introduce", "scheduled", "teased").

### 4. Official X / social announcements (via web search)

- Official account linked from both app stores: https://x.com/Stardive_EN
  (also YouTube https://www.youtube.com/@Stardive_EN).
- Searched for official X announcements of a 1.5 release. No post was found
  announcing 1.5 as live / released.
- What was found instead: the Nov 5 Developer Lounge Talk announcement on X,
  cited as a source by Kaiden's Oct 30 article ("Developer Lounge Talk
  announcement on X") — i.e. an announcement of the preview stream itself:
  November 5, 2026, 10:00 UTC, "Version 1.5 Preview". A preview-stream
  announcement is not a release announcement.
- Note on timing: this sweep ran at ~14:00 UTC on Nov 5, after the
  10:00 UTC preview stream's scheduled start. Even post-stream, no channel
  below shows a released build, release patch notes, or a "now live" notice.

### 5. Kaiden — news page

- URL: https://www.kaiden.gg/blog/category/mongil/
- Read via `browser.open`, crawled 2026-11-05 13:59 UTC.
- Latest items, with the page's own dates:
  - Oct 30, 2026 — "Mongil Nightmarch: Esther's Halloween Story and Events"
    (event runs to November 17; still 1.4-era content).
  - Oct 30, 2026 — "Mongil Developer Lounge Talk: November 5 Before 1.5":
    "Mongil: Star Dive's Developer Lounge Talk streams November 5 at
    10:00 (UTC) with Version 1.5 content, new characters and a live Q&A".
  - Oct 21, 2026 — Chloe / 10/20 update items.
  - Sep 29, 2026 — "Mongil 1.4: Isabella, Sorin and a Free 5★ for Everyone".
- No Kaiden news item reports a 1.5 release. The dedicated Nov 5 article
  (URL in Verdict above, published 2026-10-30) explicitly dates the
  Version 1.5 update to "November 10 (Tue)" — five days after the preview,
  and five days after this sweep.

### 6. App Store — version history

- URL: https://apps.apple.com/us/app/mongil-star-dive/id6483252725
- Read via `browser.open`, crawled 2026-11-05 13:59 UTC.
- Current version shown: "Version 1.04.10 Oct 28" (the App Store build
  number for the 1.4 line, per `data/game.toml`'s note).
- Featured event / What's New still reads: "1.4 Version Major Update —
  The Half Anniversary Festival Begins! — New Region: Gaba Archipelago
  Added — New Characters: Isabella and Chloe — Legendary Monster: Sorin —
  New Content: Elemental Sanctum Added — Character Balance Adjustments",
  and the description still leads with "Main Story Episode 7 is here!"
- No 1.05.x build and no 1.5 changelog appear.

### 7. Google Play — version / What's New

- URL: https://play.google.com/store/apps/details?id=com.netmarble.monster2
- Search-indexed listing (crawl 1 day before this sweep) states:
  "Updated on, Oct 27, 2026" and What's New = "1.4 Version Major Update"
  with the same 1.4 bullet list as the App Store.
- No 1.5 build or changelog.

### 8. Supporting aggregators (corroboration only, not primary)

- Prydwen banners — https://www.prydwen.gg/other-games/mongil-star-dive/banners
  — "Last updated: 02/November/2026". "Current Character Banners:
  Version 1.4, Oct 21, 2026 – Nov 10, 2026". Lou and M.A.D are listed
  under "Upcoming Banners" as "of Version 1.5" with "More information …
  as soon as it is announced" — i.e. upcoming, not released.
- gacha.gg news — https://www.gacha.gg/mongil-star-dive/news — latest
  items: 27 Oct "PC patch 1.04.10 fixes intermittent crashes" (client
  "from v1.04.00 to v1.04.10"), 21 Oct Chloe update, 30 Sep Version 1.4.
  Nothing newer, nothing labelled 1.5.
- Uptodown / QooApp / TapTap listings all show 1.04.10 (Oct 28–29, 2026)
  as the latest version, with older builds 1.04.00 (Oct 28), 1.03.00,
  1.02.00, etc. No 1.05.x anywhere.

## Released vs previewed — summary

- RELEASED ≤ 2026-11-05: Version 1.4 only (2026-09-30), plus in-version
  updates: Chloe update 2026-10-20/21 and the 1.04.10 hotfix build
  2026-10-27/28. None carries a new version number above 1.4.
- PREVIEWED / SCHEDULED only: Version 1.5 — Developer Lounge Talk preview
  stream Nov 5, 2026, 10:00 UTC; update itself dated Nov 10, 2026 by
  Kaiden (Oct 30 article) and consistent with Prydwen's "Version 1.4 …
  Oct 21 – Nov 10" current-banner window and Tom's Nov 10 launch target.
- Therefore: no patch-watch trigger. No `game.toml` update, no staleness
  re-evaluation, and no extra A+B sweep is owed from this check.
