# StarDiveMeta — bug-hunt confirmation pass

Verified against main at **e6e37712b08879a48665d15fa64ecdf721d955ba** ("Pick 5: OG image redrawn in the Starfield look"), pulled 2026-11-01. The hunt ran at 629b039; 16 commits have landed since.

Method: I read the current code for every finding and re-ran each probe the hunt described. Every probe ran on a copy outside the repo (the session scratch folder, which stands in for /tmp on this Windows PC) or loaded data into memory read-only.

- Binary: the debug `mongil-build` built at this HEAD.
- Findings 1–3: a scratch Rust crate that depends on `mongil-core`, run in release mode.
- Findings 19 and 23: the site built at this HEAD, after re-emitting into the gitignored `site/src/generated`, as CI does.
- Nothing was fixed, committed, branched or pushed. The tracked tree is clean apart from this file.

None of the files behind the Rust and CI findings changed after 629b039: `method.rs`, `load.rs`, `snapshot.rs`, `snapshots.rs`, `validate.rs`, `dist_check.rs`, `harvest_diff.rs`, `tripwire.rs`, `png.rs`, `check-dist.mjs` and `.github/workflows/`. The site and render files did change, and each was re-read.

## 1. Confirmed — still real

- **1. Half-point rounding.** A conversion value that isn't a whole or half point is silently rounded in release builds. Release probe: pg S = 3.7 → 7 half-points (3.5); pg S = 3.25 → 7 (3.5). There is no error. The guard is only a `debug_assert!`, at `crates/mongil-core/src/method.rs:23-31`. G3 checks only that tier keys exist (`validate.rs:443`).
- **2. Registry `role` ignored.** A source marked excluded or check-only, or missing from the registry, still votes. Release probe on live data:
  - Control (voting/current): 23/29 characters have a pg vote, total coverage 74.
  - `role=excluded`, `role=check_only`, and pg deleted from the registry: all 23/29, coverage 74, identical to the control.
  - `status=stale_check_only`: 0/29, coverage 51. Only `status` is consulted.
  - Code: `method.rs:434-443`. `validate.rs:488-501` counts by `role`, but only as a NOTE.
- **3. Method file picked by string order.** The loader picks `v1.9.toml` over `v1.10.toml`. Fixture copy with both files: `load_dataset` loaded version "1.9". `load.rs:139-156` uses `versions.sort()` and takes the last.
- **4. Codes with no evidence published.** NEWEPISODE7 and DIVEWITHISABELLA have zero observations, yet appear on /codes/ as "Unconfirmed".
  - Fresh emit: both are `unconfirmed`, `obs=0`, `reward.text=""`.
  - Built page: `<ul class="sightings"></ul>` is empty for both.
  - It is now slightly worse: since Pick 3, each card also has a **Copy** button (`CodeList.astro:79-85`).
  - The data comment says "never publish on this evidence" (`data/codes.toml:735-745`). The header still says "Only one site lists these, or sites disagree" (`codes.astro:62`).
  - G10 has no ≥1-observation check (`validate.rs:504-536`). The emit test `code_status_no_observations_is_unconfirmed` (`emit.rs:1312`) still bakes it in.
- **5. Wrong current banner from Nov 11.** Isabella will show as "Current banner" from Nov 11 onward.
  - The page's rule re-run over fresh `banners.json` gives: 11-01 chloe, 11-10 chloe, 11-11 **isabella**, 12-01 **isabella** (`banner.astro:29-31`).
  - No v1.5 banner entry exists yet; it appears only as an `[[upcoming]]` event on 11-10.
  - The new "What's next" panel goes empty at the same time.
- **6. Dead name lookup in the "(via …)" label.** This is about code, not the wording, which stands as Tom picked it. `sourceNames` is keyed by source id but looked up with a banner URL, so it never matches (`banner.astro:19,35-39`).
  - Emitted source ids are pg, kaiden, gamewith-jp, urgametips, gacha-gg, gamewith-en and mongil-gg. No banner source equals any of them.
  - The built page shows "(via prydwen.gg)" from the hostname fallback.
- **7. Download link always gets Low.** "Download this list (PNG)" is hard-coded to `tierlist-low.png` (`index.astro:90`).
  - Nothing rewrites it: `LevelToggle.astro:32-35` only sets `data-level` and localStorage.
  - The three PNGs differ: sha prefixes f3aa…/4b1f…/f53f…; sizes 260,186 / 261,852 / 259,819 bytes.
- **8. 404 page names itself canonical.** `dist/404.html` has `canonical` and `og:url` = `https://stardivemeta.com/404.html` (`404.astro:6`). No robots or noindex meta exists anywhere in `site/src` or `site/public`.
- **9. One description on every page.** All 46 built pages carry the same meta description, the tagline (`Base.astro:14,26,71,74`). No page passes `description=`.
- **10. Method changes don't count as a snapshot change.** Fixture copy:
  - Snapshot 2026-11-03-am written (sha 6fa503028245…).
  - Then `method.toml` alone was changed: version 1.0→1.1, s_min 3.5→1.0.
  - `diff … --to working` reports 63 tier changes.
  - But `snapshot --id 2026-11-03-pm` exits 3 ("inputs unchanged since 2026-11-03-am") and writes nothing.
  - The method is absent from `snapshot_inputs` (`snapshot.rs:112-137`).
- **11. "Latest" snapshot found by string order.** Fixture copy:
  - patch2 written; Mina's conquest changed S→C; patch10 written (0df20fac5e30…).
  - Reverting the change and running patch11 exits 3: "unchanged since 2026-11-03-patch2", no file written.
  - Yet `diff --from 2026-11-03-patch10 --to working` shows 3 tier changes and 1 placement change.
  - Cause: a plain string sort plus `.last()` (`snapshots.rs:94,123`).
- **12. Bad sweep ids accepted, some crash.**
  - `2026-99-99-am`, `2026-02-30-pm`, `2026-11-03T9999-sweep-am` and `0000-00-00-x1` all exit 0 and write snapshot files.
  - `€€€€-am` exits 101: "panicked at crates\mongil-build\src\snapshots.rs:30:19".
  - Control: `ééééé-am` exits 2 cleanly.
  - Code: `snapshots.rs:25-62`.
- **13. Image allowlist gate turns off when the file is missing.** A fixture `img/own/ripped.png` with no `ALLOWLIST.txt`:
  - `validate --site` prints "PASS G14", OK.
  - `check-dist.mjs` prints "OK … G14 images clean", exit 0.
  - With an allowlist that omits the file, both FAIL (node exits 1).
  - Code: `check-dist.mjs:147-162`, `dist_check.rs:171-217`.
- **14. G11 doesn't range-check stamps.** On a copy of live data, both stamps set to `2026-02-30T25:61:61Z` still give "PASS G11 stamps" and "validate: OK". `validate.rs:54-76` checks only the shape plus a string-based future check.
- **15. G9 is a no-op on push to main.** Git simulation in scratch: a committed snapshot was edited and committed, with `origin/main` = HEAD.
  - The CI command `git diff --diff-filter=MDR --name-only origin/main -- data/snapshots/` prints nothing.
  - The same diff against `HEAD~1` prints the file.
  - `ci.yml:10-12` runs on push to main and on pull requests; `ci.yml:56,66` diff against `origin/main`. `deploy.yml` has no G9 step.
- **16. Launch detection is a raw substring grep.** A fixture `autodeploy.toml` with only the comment `# launch_date is set at launch time, not yet`:
  - The CI condition reports the gate ARMED.
  - `tomllib` keys are only `['paused', 'pause_reason']`.
  - Code: `ci.yml:65`. The live file has 0 occurrences.
- **17. Character pages skip from h1 to h3.** All 29 `dist/c/*` pages have the heading sequence [1, 3, 3, 3, 2].
  - The name is h1 (`EvidencePanel.astro:117`).
  - The section labels are fixed h3 (`:199,242,255,281`).
  - Pick 2 added "Tier history" as h2 *after* those h3s (`TierHistory.astro:49`), so the outline now also goes back up out of order.
- **18. No skip link.** On /, /codes/, /c/mina/ and /table/ there are 9 focusable links before `<main>`.
  - There is no skip link, and `<main>` has no id (`Base.astro:130`).
- **19. An expired code moved client-side still looks active.** Headless browser on the built /codes/ with the clock set to 2026-11-11T12:00Z:
  - HAPPYHALF1014 moves into the Expired list but keeps class `code-card code-active`.
  - It keeps the "Reward: …", "Expires Nov 10, 2026" and "Confirmed by …" lines, plus the **Copy button**.
  - A real expired card has `code-expired`, only "Last seen Oct 30, 2026", and no Copy button.
  - Code: `codes-expiry.ts:6-16`, which only calls `append`.
- **20. Double-read ignores `world`.** A real mongil-gg harvest (`harvest/2026-10-29T2105-sweep-pm/t26-mongil-gg.toml`) against a copy that differs only in Mina's `world` (S→C):
  - `harvest-diff` prints "0 blocking, 0 warnings → clean", exit 0.
  - Control with `conquest` changed instead: MISMATCH, exit 1.
  - The field list has no `world` (`harvest_diff.rs:99`). G6 reuses `compare` (`validate.rs:690-731`).
- **21. Tripwire fails open on bad dates.** Copy of live data with one major unreviewed erratum:
  - `found_at="garbage!!"`: "tripwire: clear", exit 0.
  - Control `2026-10-30` at today 11-01: TRIPPED, exit 1.
  - `2026-02-30`: clear on 03-01, TRIPPED on 03-08, so it was treated as Mar 2.
  - Code: `tripwire.rs:57-60,89` (`if let Some`) and `:100` for launch_date.
  - The `day_ordinal` doc says invalid dates return `None`; they don't.
- **22. PNGs silently drop chips past two lines.** Copy of live data with every threshold set to 0.0:
  - `png` exits 0 and writes all 4 PNGs.
  - Replaying the renderer's `flow`/`chip_width`, the S row has 28 chips across 4 lines, and **16 are dropped** (Yeonhwa … Daisy) on each level.
  - Live data: every row fits in 2 lines or fewer.
  - Still `take(2)` in `row_svg` (`mongil-render/src/lib.rs:220`), with fixed `ROW_H`.
  - Pick 5's new `og_row_svg` copied the same `take(2)` (`:406`).
  - `png.rs:199-202` has no completeness check.

## 2. Already fixed or no longer applies

- None. The 16 commits since 629b039 changed none of the 22. Two of them made existing findings slightly worse:
  - Pick 3 (`17f822f`, tap-to-copy) added Copy buttons to #4's evidence-free codes and to #19's moved card.
  - Pick 5 (`e6e3771`) repeated #22's truncation in the new OG renderer.

## 3. Cannot reproduce

- None.

## 4. New candidates

- **23. `check-dist` passes an empty build.** A dist with **zero pages** prints "check-dist: OK — 0 pages … G14 images clean" and exits 0. If the build ever emits no HTML, the gate can't tell, since there is no minimum page count. This hits operators and CI. Evidence: running `site/scripts/check-dist.mjs` on a scratch dist with no HTML at all (one image under `img/own/`, no allowlist) printed "check-dist: OK — 0 pages", exit 0. The G14 part of that pass is #13; the "0 pages" OK is a separate gap.
- **24. Banner and upcoming dates are never validated, then compared as strings.** `validate.rs:375-377` checks only banner source URLs. Nothing checks the `start`/`end` shape on `[[banner]]` or `date` on the new `[[upcoming]]` (Pick 8). `banner.astro:30,48` compares them as plain strings. So a typo like `2026-11-5` passes validate and sorts after `2026-11-10`, which keeps a "What's next" entry or banner on screen for the wrong days. This hits /banner/ readers after a data typo. There is no live effect: all current dates are well-formed.

---

HEAD verified: **e6e37712b08879a48665d15fa64ecdf721d955ba**

Counts: **22 confirmed** / **0 fixed or moot** / **0 cannot reproduce** / **2 new candidates**
