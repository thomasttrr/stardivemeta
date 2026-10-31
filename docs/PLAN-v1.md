# StarDiveMeta v1 — Build Plan (for Tom's approval, then X executes)

Planner: Opus 5.5 · Written 2026-10-29 · Inputs: the 8 files in
`stardivemeta_plan_data/` (all read in full).

---

## Summary for Tom (phone read, 1 minute)

- **What you approve:** this whole document. X builds it exactly as written.
- **Your time before launch:** about 105 minutes, spread over 6 short checkpoints
  across ~4 weeks. **After launch:** about 10–15 minutes a week.
- **Spending:** only the domain (stardivemeta.com, about $10–11 a year at
  Cloudflare). Everything else is free tier. No other money.
- **The method stays Method v1.0.** I tested other numbers on the real data;
  none of them did better (Part D, D-0). I'm keeping all five constants.
- **Biggest changes I made (all in Part D, each one you can veto):**
  1. One evidence page per character (`/c/narae/`), so search finds them and you can link to them.
  2. No React in v1. Plain HTML plus a tiny script. Faster and fewer bugs. React gets added with the tier-list maker.
  3. The "reroll" part waits until we have checked data on which banner each character is on. Until then: a Beginner page with no reroll claims.
  4. The toggle copy loses "what a typical account has". We have no data behind that claim.
  5. Sweeps read each source once. A second, independent read happens only when something changed, plus a full double-read every Monday. That's cheaper, and nothing gets published without two reads.
  6. Two image-only lists (GameWith JP's lower tiers, mongil.gg) get read by X's agents from screenshots, with a 5-minute spot-check by you. They don't count toward scores until you approve.
- **4 questions for you** (Part E), each with a default you can accept in one tap.

---

## Input disagreements I found (source file wins, as instructed)

| # | Handoff says | Source file says | What this plan does |
|---|---|---|---|
| X1 | Codes: "MONGILDISCORD active per Pocket Gamer, expired per SuperCheats" | No source file has any code data. Scope only says "8+ codes reported active … statuses conflict" | We treat the codes dataset as **empty**. Codes are harvested fresh in T2.7. The MONGILDISCORD example isn't used anywhere. |
| X2 | "the six newest characters have no per-mode ranking anywhere" | Dataset names 5 (Brisshell, Mabel, Nagi, both Summer variants) and says PG covers 23 of 29. The sixth is Vivian (second pass). Nothing establishes that they're the "newest" | The six are named explicitly: Brisshell, Mabel, Nagi, Verna: Summer Dive!, Francis: Summer Dive!, Vivian. |
| X3 | PNG "Rust side (resvg) or Astro side" is open | Scope: "Official list PNG (generated at build time, resvg)" | resvg on the Rust side (crate `mongil-render`). |
| X4 | Twice-daily sweeps (§7) | Guardrails §4: "stays inside the existing weekly sweep + daily codes scout" | Twice-daily wins. Workflow §1 records it as Tom's later amendment (18:41 ET). The guardrails line is out of date. |
| X5 | §7: "Below 3 voting sources → mongil.gg top priority" (straight away). §8: "for over one patch cycle" | Workflow §4: straight away. Guardrails §3: after one patch cycle | Both, in two steps: the mongil.gg read starts **straight away**, and the replacement-source proposal comes **after one patch cycle**. |
| X6 | §2.4: Tom approves code/design/method that "**departs from the approved plan**" | Workflow §1: "Anything touching code, design, or the method goes to a branch for Tom's approval" | During the build (Phases 1–6), X merges in-plan code once CI is green, and Tom reviews at checkpoints. After launch, the workflow rule applies: every code/design/method change waits for Tom. |
| X7 | Game "v1.04" (PG) | Dataset: PG "version 1.04"; Kaiden says Vivian "released Sep 30, 2026 with version 1.4" | Unresolved in the data. T2.3 sets the official version string. Staleness logic waits on it. |
| X8 | §1: research "verified that no major gacha guide site offers the combination…" | No source file holds that research | Not used in any user-facing copy. |
| X9 | Agent named "Kit" | Tom's brief: executor is "X" | X = executor everywhere in this plan. |
| X10 | Simulation prints Angel as "MODE-SPLIT/Solid" | Method §3: mode-split shows no Overall tier | Angel's displayed label is **Mode-split**. The spread is still computed and shown in evidence. Label counts in the tests are adjusted (Low: 16 Solid, not the sim's 17). |
| X11 | Maker "on dnd-kit + islands" | Scope: user PNG export "arrives with the tier-list maker (WASM canvas)" | Post-v1. No action. Logged only. |
| X12 | — | Dataset first pass has a "Draft consensus" (PG-only S/A lists) | Superseded by Method v1.0. **Never used** anywhere. |

---

# PART A — Build plan

## A.0 Ground rules for X (apply to every task)

1. **Astro does no math.** Every tier, label, score, order, derived list and
   code status is computed in Rust and emitted as JSON. Astro only renders.
2. **Harvest = transcription.** Workers copy what a source publishes into
   structured fields. Rust converts the values. Workers never convert, average
   or guess.
3. **Missing is never zero.** It's enforced in types (`Option<Vote>`) and in tests.
4. **Snapshots are append-only.** CI fails if any file already in
   `data/snapshots/` on `main` is modified or deleted.
5. **Silence is the default.** Reports to Tom go out only for changes,
   failures and approvals.
6. **Task done = its acceptance criteria pass in CI or on a preview URL.**
   X records each pass in `docs/BUILD-LOG.md` (one line: task id, date,
   commit, how it was checked).
7. **Times are America/New_York (ET)** unless marked UTC. US DST ends
   Sun 2026-11-01. X's scheduler must use the `America/New_York` zone, not
   a fixed UTC offset.

## A.1 Phases at a glance

| Phase | Calendar (agent-built) | Tom-minutes | Ends with |
|---|---|---|---|
| P0 Approval + setup | day 0–1 | 25 | CP0: plan approved, domain bought |
| P1 Foundations (Rust core, tests, CI, design pick) | days 1–4 | 10 | CP1: design direction picked |
| P2 Data pipeline + fresh harvest | days 3–8 (overlaps P1) | 15 | CP2: data checkpoint |
| P3 Site v1 (board, evidence, rules, table, PNG, deploy) | days 7–13 | 20 | CP3: phone review of preview |
| P4 Codes, banners, history, notes, errata | days 12–15 | 0 | — |
| P5 Cadence dry run (7 days, staging only) | days 15–22 | 10 | CP4: first movers note approved |
| P6 Launch | days 22–24 | 25 | CP5: go/no-go, search-console sign-ins, Tom's community posts |
| **Total to launch** | **~24 days** | **~105** | |
| P7 Post-launch (steady state) | weekly; review at 6–8 weeks | 10–15/week; 30 at review | Two-patch-cycle review |

The critical path is P1 → P3 → P5 → P6. P2 runs alongside P1, and P4 alongside the end of P3.

---

## P0 — Approval + setup

**T0.1 Tom approves the plan** · Tom 10 min
- Deliverable: Tom's "approved" reply, plus his answers to Part E (or "defaults").
- Steps: X copies this file to `docs/PLAN-v1.md` in the repo (T1.1) unchanged, then records the approval date in `docs/BUILD-LOG.md`.
- Accept: `docs/PLAN-v1.md` matches this file byte for byte (sha256 recorded in the log).

**T0.2 Domain** · Tom 10 min · *spending item, see Part E Q1*
- Deliverable: `stardivemeta.com` registered at Cloudflare Registrar in Tom's Cloudflare account. No `.gg`: the inputs mark it optional, and I don't think Cloudflare Registrar sells .gg (X confirms at purchase). If Tom wants it anyway, that's a separate spending tap.
- Accept: `whois stardivemeta.com` (or RDAP) shows the registration. The zone appears in Cloudflare DNS.
- Blocks: T6.4 only. All earlier work runs on `*.pages.dev`.

**T0.3 Access check** · Tom 5 min (only if X lacks access)
- Deliverable: a private GitHub repo `stardivemeta` that X can push to. A Cloudflare Pages project `stardivemeta` (fallback name `stardivemeta-site`) created via direct upload. GitHub Actions secrets `CLOUDFLARE_API_TOKEN` (scope: Account → Cloudflare Pages → Edit) and `CLOUDFLARE_ACCOUNT_ID`. If X already has the project's GitHub/Cloudflare setup, X does all of this and Tom does nothing.
- Accept: a test workflow runs `wrangler pages project list` and it succeeds.

---

## P1 — Foundations

**T1.1 Repo scaffold**
- Goal: a workspace that builds on Windows (Tom's machine) and Ubuntu (CI).
- Deliverable: the tree in Part B §B.1, with empty crates that compile.
- Inputs: Part B §B.1.
- Steps:
  1. `Cargo.toml` workspace with members `crates/mongil-core`, `crates/mongil-render`, `crates/mongil-build`. `rust-toolchain.toml` pinned to the stable release current on the day (expected `1.99.x`; X records the exact version).
  2. `site/`: Astro (latest stable major at start, exact version pinned in `pnpm-lock.yaml`), TypeScript strict, Tailwind (latest stable major), `@astrojs/sitemap`. Node 24 LTS (`.nvmrc` = `24`), pnpm 10 (`packageManager` field).
  3. `.gitignore`: `target/`, `site/node_modules/`, `site/dist/`, `site/src/generated/`, `site/public/img/generated/`.
  4. Copy the 8 input files into `docs/inputs/` (read-only record).
- Accept: `cargo build --workspace` and `pnpm -C site build` both exit 0 on CI ubuntu-latest and on Windows.

**T1.2 Data model + loaders (`mongil-core`)**
- Goal: every data file parses into strict types, so bad data fails loudly.
- Deliverable: `crates/mongil-core/src/{model.rs, load.rs}`.
- Inputs: file formats in Part B §B.3.
- Steps:
  1. Serde structs for `Roster`, `Character`, `SourceRegistry`, `Source`, `PgPlacements`, `KaidenPlacements`, `JpPlacements`, `CheckOnlyPlacements`, `GameInfo`, `Codes`, `CodeSources`, `Banners`, `Errata`, `MethodConfig`, `Checks`. Every struct uses `#[serde(deny_unknown_fields)]`.
  2. Tier enums per source: `PgTier {S,A,B,C}`, `JpTier {SS,S,A}` (B/C added only if T2.5 approves), `KaidenTier {T0,T0_5,T1,T2,T3,T4}` (serialized as `"T0"`, `"T0.5"`, …).
  3. Internally, votes are integer **half-points** (`u8`, 0..=8). Example: T0.5 = 7 half-points = 3.5.
  4. `load::load_dataset(dir: &Path) -> Result<Dataset, LoadError>`. Errors name the file, line and field.
- Accept: unit tests `load_rejects_unknown_field`, `load_rejects_bad_tier("T5")`, `load_rejects_duplicate_character`, `load_rejects_unknown_character_ref` all pass.

**T1.3 Fixture: the 2026-10-29 dataset**
- Goal: freeze the exact dataset the simulation used, as test input.
- Deliverable: `crates/mongil-core/tests/fixtures/dataset-2026-10-29/` with `roster.toml`, `sources.toml`, `method.toml` (copy of `data/method/v1.0.toml`), `placements/pg.toml`, `placements/kaiden.toml`, `placements/gamewith-jp.toml`, plus the check-only files.
- Inputs: `mongil-tier-list-consensus-2026-10-29.md` (PG Conquest/Rift lines 24–34; Kaiden lines 129–159; JP lines 195–204; urgametips 180–183; gacha.gg 216–220; GameWith EN 46–48).
- Steps: transcribe by hand into the formats in §B.3. Kaiden's `raw` field gets the dataset's full line (e.g. `"T0.5 (A4), T1 at A2"`). For a conditional line, `tier` is the **first-listed** tier (the rule the simulation used, codified in D-G1). Chloe gets `status = "no_placement"`. JP's six absentees get `status = "not_in_text_tiers"`.
- Accept: a second worker transcribes the same lines independently, and `mongil-build harvest-diff` (T2.1) on the two copies reports 0 mismatches. Until T2.1 exists, a plain `diff` of the two TOML files must be empty.

**T1.4 Method engine (`mongil-core/src/method.rs`)**
- Goal: Method v1.0 exactly as written, with no floating-point maths in any decision.
- Deliverable: `pub fn compute(ds: &Dataset, method: &MethodConfig, level: Level) -> LevelResult` and `pub fn compute_modes(ds) -> ModeResults`.
- Steps (each rule is one small function with its own unit test):
  1. `pg_overall_h(c, r) = (c_h + r_h) / 2`. PG tiers are whole points, so the average is always a whole number of half-points. A `debug_assert!` checks this. If only one mode exists, use it.
  2. Votes per character: PG overall (if any), Kaiden at `level` (if `placed`), JP (if `placed`). Check-only sources are never included.
  3. `coverage = n = votes.len()`. `spread_h = max − min`. `score = sum_h / (2n)`, as `f64` for display only. Decisions compare integers: S if `sum_h >= 7n` (3.5), A if `>= 6n`, B if `>= 4n`, C if `>= 2n`, else D.
  4. Label: coverage 0 → Unranked; 1 → Speculation; ≥2 and `spread_h <= 2` (1.0) → Solid; else Contested.
  5. Mode-split: if PG has both modes and `|c_h − r_h| >= 4` (2.0) → tier `ModeSplit`, label `ModeSplit`. Score, spread and coverage are still output for evidence.
  6. Order inside each tier: score desc, then coverage desc, then display name asc (Unicode code-point order).
  7. `score_display`: round half away from zero to 1 decimal (`format!("{:.1}", (score*10.0).round()/10.0)`).
  8. Mode views: tier = PG's tier in that mode. Characters without a PG mode placement → `Unranked` in that mode. Order inside a tier: display name asc.
  9. All constants are read from `MethodConfig` (`data/method/v1.0.toml`). None are hard-coded.
- Accept: T1.5 passes.

**T1.5 Regression tests from the simulation + invariants**
- Goal: the 29-character simulation becomes a test suite that fails on any drift.
- Deliverable: `crates/mongil-core/tests/sim_v1.rs`, `tests/invariants.rs`, and `tests/fixtures/sim-v1-expected.json` (the table in Part B §B.6, machine-readable).
- Steps: write 29 named per-character tests plus 12 aggregate tests, exactly as listed in §B.6. Score comparisons use tolerance ±0.005 against the 2-decimal expected value. Tier, label, coverage and spread must match exactly. Invariants use `proptest` with 2,000 cases each (listed in §B.6).
- Accept: `cargo test -p mongil-core` shows 41 sim tests + 8 invariant tests passing. Deliberately changing `s_min` to 3.3 in a scratch branch makes `sim_v1_band_counts_low` and `sim_v1_mina` fail (proof the tests bite; X records this once in BUILD-LOG, then discards the branch).

**T1.6 CLI + JSON contract**
- Goal: one binary does everything the pipeline needs.
- Deliverable: `crates/mongil-build` (clap) with the subcommands `validate`, `emit`, `snapshot`, `diff`, `harvest-diff`, `png`, `lane`, `tripwire`, `note`. In this task `validate` and `emit` are complete; the others are stubbed and finished in later tasks. Contract types live in `crates/mongil-core/src/contract.rs` with `#[derive(Serialize, ts_rs::TS)]`, and `cargo test` exports them to `site/src/types/contract/*.ts` (committed).
- Inputs: Part B §B.2.
- Steps: `mongil-build emit --data data --out site/src/generated` writes `tierlist.json`, `codes.json`, `history.json`, `meta.json`. Output is deterministic: sorted keys, no timestamps except `meta.json.generated_at`.
- Accept: (a) running `emit` twice on unchanged data gives byte-identical `tierlist.json`. (b) CI step `git diff --exit-code site/src/types/contract` passes, so types can't drift from Rust. (c) `astro check` passes with the generated types.

**T1.7 CI (Rust + data)**
- Deliverable: `.github/workflows/ci.yml` with jobs `rust` (fmt --check, clippy -D warnings, test) and `data` (`mongil-build validate --data data`, the snapshot immutability check G9).
- Accept: a PR that changes a test expectation goes red; a clean PR goes green. Runtime ≤ 6 min with `Swatinem/rust-cache`.

**T1.8 DESIGN.md — direction pick (CP1)** · Tom 10 min
- Goal: one locked, non-generic visual direction before any UI work (Tom's rule).
- Deliverable: three static mock pages, `site/src/pages/_design/a.astro`, `b.astro`, `c.astro`, each showing the Low board (from the T1.3 fixture output), one evidence panel and the legend. They're deployed as a preview. X sends Tom one message with three phone screenshots and the preview URL: "Pick A, B or C."
- Rules for the directions: patterns adapted from Tom's reference sources (Designeer, getdesign.md, AI UX Playground), with no copied assets or code. No game art. Labels are never shown by colour alone (text + shape). WCAG AA contrast. One direction should be "data-sheet" (dense, tabular, mono numerals). The other two are X's choice and must differ in layout, not just colour.
- After the pick: `docs/DESIGN.md` locks the tokens (colour, type scale, spacing, radius, tier colours, label glyphs: Solid ●, Contested ◐, Speculation ○, Mode-split ⇄, Unranked —), the font (an OFL-licensed family; its file is bundled for the PNG too), and the motion rules (CSS only, ≤200 ms, off under `prefers-reduced-motion`). The `_design/` pages are deleted.
- Accept: `docs/DESIGN.md` exists with every token named, and `site/src/styles/tokens.css` defines exactly those tokens.

---

## P2 — Data pipeline + fresh harvest

**T2.1 Harvest format, guide and double-read diff**
- Deliverable: `docs/harvest-guide.md` (per-source extraction rules, §B.4). Harvest files at `data/harvests/<sweep-id>/<source>.a.toml` and `.b.toml`. `mongil-build harvest-diff <a> <b>` is complete.
- Diff rule: compare **structured fields only** (character set, `conquest`, `rift`, `tier`, `low/high/max.tier`, `awakening`, `flags`, `status`, `page_updated`). `raw` and `note` are compared after whitespace and punctuation are normalized, and a raw-only difference is a **warning**, not a block. Output format is in §B.4. Exit 1 on any mismatch.
- **Independence rule:** worker A reads the page as rendered text (browser `get_page_text`). Worker B reads the same section from a **screenshot** (visual read), with no access to A's file. The two reads take different paths, so they rarely make the same mistake.
- Accept: tests `harvest_diff_identical_ok`, `harvest_diff_tier_mismatch_blocks`, `harvest_diff_missing_character_blocks`, `harvest_diff_raw_only_warns` pass.

**T2.2 Fresh full harvest of all sources**
- Goal: the launch data is freshly double-read, not inherited from the 10-29 notes.
- Inputs: `docs/harvest-guide.md`, and the URLs in `data/sources.toml` (§B.3).
- Steps: run A and B reads for `pg`, `kaiden` (29 guide pages; record each page's own "updated" date if shown, else `page_updated` is omitted and the UI says "date not shown"), `gamewith-jp`, `urgametips`, `gacha-gg`, `gamewith-en`. Merge into `data/placements/*.toml` once each diff is clean.
- Accept: `mongil-build validate` passes. Every `data/placements/*.toml` has a matching clean A/B pair under `data/harvests/<sweep-id>/`. X writes `docs/harvest-reports/<sweep-id>.md` listing every difference from the T1.3 fixture (expected: none, or real source updates since 10-29, each with its URL).

**T2.3 Game version + balance-patch table**
- Deliverable: `data/game.toml` (§B.3), with `current_version`, plus `[[release]]` entries giving version, date, `balance_changes` (bool) and the official source URL.
- Steps: read the official MONGIL: STAR DIVE patch notes/news (Netmarble's official site or official channels). Settle the **1.04 vs 1.4** question (X7) by quoting the official version string. Mark each voting source `current`, `pre_patch_flagged` or `stale_check_only` per Method §5.
- Accept: every source in `sources.toml` has a `game_version` that matches an entry in `game.toml`, or is marked `version_unstated`. Validate gate G2 checks this.

**T2.4 Kaiden level definitions**
- Deliverable: `data/sources.toml` → `kaiden.level_definitions = { low = "...", high = "...", max = "..." }`, paraphrased from Kaiden's own text with its URL. Paraphrase, not a quote over 15 words.
- Accept: if Kaiden publishes no definitions, the field is absent and the toggle uses the fallback copy (§B.5 Home). Either way, X records the finding in the harvest report.

**T2.5 GameWith JP visual-table read (conditional)** — *method-touching, Tom taps at CP2*
- Goal: settle whether JP places Sangun, Ellie, Bonnie, Leeho, Verna (base) and Daisy below A on its visual tier table.
- Steps: A and B visual reads of the table image(s) on `https://gamewith.jp/stardive/552850`. Record the tier for each of the six, or `not_on_table`.
- Branches:
  - Not on the table, or unreadable → no change. They stay "no vote".
  - On the table at named tiers below A → propose the extension `JP B = 1, JP C = 0` (one step per tier, the printed assumption). X runs `mongil-build diff` with and without it and puts the before/after table in the CP2 message. If Tom taps yes, `JpTier` gains `B`/`C`, the method file is updated, and since nothing is published yet it stays **Method v1.0**. The T1.5 fixture tests stay unchanged (they test the 10-29 dataset); new tests `jp_ext_<slug>` are added for the six.
  - Hypothetical effect if all six are JP B (from my re-run on the real data): Ellie B→C at all levels (2.25 → 1.83, label Solid→Contested). Sangun at Max goes B→C (2.25 → 1.83). Bonnie, Leeho and Verna stay C/Solid. Daisy stays D/Solid (0.67). Nothing else moves.
- Accept: the CP2 message contains the before/after table and Tom's tap is recorded in BUILD-LOG.

**T2.6 mongil.gg read as check-only (conditional)**
- Goal: a second per-mode source in the evidence, without touching the math.
- Steps: A and B visual reads of `https://mongil.gg/tierlist` (World / Conquest / Dimensional Rift).
  - If the images carry name text, read them.
  - If they show portraits only, match each portrait against Kaiden's character-page portraits by eye. Both workers must agree on every identity.
  - Tom spot-check: X sends 5 random placements, each with a cropped screenshot. Tom taps "match" or "wrong" (5 min).
- Result: `data/placements/mongil-gg.toml`, `role = "check_only"`. Shown in evidence under "Not counted", with the note "Read from images. Not counted until a method update."
- Branches: any A/B identity mismatch that can't be resolved, or any "wrong" from Tom → abandon. mongil.gg stays unread and nothing is shown. Promotion to a voting source is a **Method v1.1** decision after launch (T7.3), simulated on this real data first.
- Accept: validate passes, the evidence panel shows mongil.gg rows, and every score is unchanged (`mongil-build diff` reports 0 result changes).

**T2.7 Codes harvest + independence registry**
- Deliverable: `data/code-sources.toml` (each site with an `independence_group`; official channels = group `official`) and `data/codes.toml` (§B.3).
- Steps: check at least 4 code-listing sites plus official channels. Record one observation per code per source, with `status`, `seen_at`, URL, and the reward text as that source states it. Two sites go in the same `independence_group` if their code lists **and** reward wording match exactly; X records the evidence. Double-read applies: A from text, B from screenshot.
- Accept: `mongil-build validate` passes G10. `emit` shows each code's computed status (rules in §B.3 codes).

**T2.8 Roster facts: release dates + banners**
- Deliverable: `released_on` per character in `data/roster.toml` (only where an official source or two independent sources give a date; otherwise omitted), and `data/banners.toml` (current and recent banners, from official news).
- Optional, same task: `rarity` and `pool` (`permanent` / `limited`) per character, two-source rule. If it can't be confirmed for all 29, the fields stay omitted and the Beginner page's reroll section doesn't render (D-3).
- Accept: validate gate G2 passes. Every set field has `*_sources = [url, url]` (or one official URL).

**T2.9 Snapshots + diff**
- Deliverable: `mongil-build snapshot --id <sweep-id>` and `mongil-build diff --from <id> --to <id|working> [--format md|json]`.
- Rules:
  - A snapshot is written only if any placement, source metadata or roster field changed since the latest snapshot (exit code 3 = unchanged, no file).
  - It stores the frozen inputs **and** frozen results, so history renders without re-computing.
  - Snapshot id = sweep id (`YYYY-MM-DD-am`, `-pm`, `-patchN`, or `-xN` for manual).
- Accept: tests `snapshot_unchanged_exits_3`, `snapshot_roundtrip_renders_identical_results`, `diff_reports_tier_label_coverage_and_placement_changes` pass.

**T2.10 Validation gates G1–G14 wired into `validate` and CI**
- Deliverable: every gate in Part B §B.7, each with a failing-fixture test in `crates/mongil-build/tests/gates.rs`.
- Accept: `cargo test -p mongil-build gates` has one passing test per gate (14 tests), and each one proves its gate rejects bad input.

**CP2 — Data checkpoint** · Tom 15 min
- X sends one phone message containing:
  - the harvest report summary (changes vs 10-29, maximum 10 lines);
  - the version answer (X7);
  - the T2.5 before/after table and a yes/no tap;
  - the T2.6 spot-check (5 taps);
  - the codes count by status;
  - the Low S tier as it now stands.
- Accept: Tom's taps are recorded in BUILD-LOG.

---

## P3 — Site v1

**T3.1 Shell, tokens, layout, disclaimer**
- Deliverable: `site/src/layouts/Base.astro` (head meta, canonical to `https://stardivemeta.com<path>`, OG tags, the inline level-restore script, the header nav, and a footer carrying `Disclaimer.astro`). Also `site/src/styles/tokens.css` and `global.css` from DESIGN.md.
- Nav (in this order): Tier list · Conquest · Rift · Table · Beginner · Codes · New characters · Rules. The footer adds History, Notes, Corrections, Data and About.
- Accept: every page in `dist/` contains the exact disclaimer string (gate G12). HTML validates with `html-validate` at 0 errors.

**T3.2 Home tier board + investment toggle** (`index.astro`, `TierBoard.astro`, `CharacterChip.astro`, `LevelToggle.astro`, `Legend.astro`, `Stamp.astro`, `src/scripts/level.ts`)
- How it works: all three level boards are rendered as static HTML. `<html data-level="low|high|max">` decides which one shows, via CSS (`[data-level=low] .lvl:not(.lvl-low){display:none}`). The toggle is 3 radio buttons. `level.ts` sets the attribute and saves it to `localStorage` key `sdm.level` (wrapped in try/catch). An inline `<head>` script restores it before first paint, so there's no flash. Without JS, Low shows and the toggle becomes 3 links (`/?level=high` is handled by the same script; with no JS at all, the links point to `/table/#high`).
- Chip: name, label glyph + label text (screen-reader text), and `data-slug`. Tapping opens the evidence `<dialog>` (T3.3) and also works as a link to `/c/<slug>/`.
- Accept:
  - (a) In the preview, toggling changes the visible S row from 4 chips (Low) to 7 (High). X checks this with the browser tool and records it.
  - (b) With JS disabled, the Low board shows and is complete.
  - (c) The page's total JS is ≤ 6 KB gzip (gate G12).
  - (d) Reloading after choosing High shows High.

**T3.3 Evidence panel + character pages** (`EvidencePanel.astro`, `src/pages/c/[slug].astro`, `src/scripts/dialog.ts`)
- One component, two places it appears: inside a `<dialog>` on boards (pre-rendered per character) and as the body of `/c/<slug>/` (29 pages). Contents and order: §B.5 Character page.
- Accept: 29 pages exist in `dist/c/`. Francis's page shows "Contested" at Low with PG 2.0, Kaiden 3.5, JP 4.0 and JP's criteria caveat. Chloe's shows Kaiden "No placement — review coming soon". Angel's shows "Mode-split" with Conquest C / Rift S. The dialog closes on Esc and on the backdrop, and focus returns to the chip (checked with the browser tool).

**T3.4 Mode pages** (`conquest.astro`, `rift.astro`, `ModeBoard.astro`)
- Accept: the Conquest S row = Chloe, Esther, Flare, Mina, Narae (name order). The Rift S row = Angel, Chloe, Isabella, Jiwon, Mina, Narae, Sangun. The "Not ranked by mode" row has the six named in X2. The page header contains the single-source sentence (§B.5).

**T3.5 Table page** (`table.astro`, `TierTable.astro`)
- Accept: 29 rows. Columns as in §B.5. Default sort is Low score desc. The row ids `#low/#high/#max` anchor the column headers. There's a `<caption>`, `scope` attributes and no JS.

**T3.6 Rules page** (`rules.astro`)
- Every number on the page is read from `tierlist.json.method` (generated from `data/method/v1.0.toml`). Nothing is typed by hand.
- Accept: changing `s_min` in a scratch branch changes the rules page text (proof it isn't hard-coded); the branch is discarded. The copy matches §B.5.

**T3.7 Beginner page** (`beginner.astro`; lists computed in `mongil-core/src/derive.rs`)
- Accept (on the fixture, test `derive_beginner_v1`):
  - Agreed at Low = Narae, Chloe, Isabella, Brisshell, Verna: Summer Dive!
  - Rise with investment = Francis: Summer Dive!, Nagi, Vivian, Jiwon, Mabel, Mina, Sangun, Esther
  - Disagree at Low = the 12 Contested, in board order
  - With no `pool` data, the reroll section is absent from `dist/beginner/index.html`.

**T3.8 About, Corrections, Data, 404**
- Accept: the pages exist with §B.5 copy. `/data/` links to `/data/tierlist.json` and `/data/codes.json` (copied into `dist/data/` at build time). If Part E Q4's answer is "none", the corrections address line is absent.

**T3.9 Official PNG + OG image** (`crates/mongil-render`)
- Deliverable: `mongil-build png --out site/public/img/generated/` writes `tierlist-low.png`, `tierlist-high.png`, `tierlist-max.png` (1200 px wide; height = 220 + 120 × rows, capped at 2000) and `og.png` (1200×630).
- How: an SVG string is built in Rust, then usvg/resvg render it with the DESIGN.md font loaded from `crates/mongil-render/fonts/` (OFL). Names only, no game art. Contents: §B.5 PNG.
- Accept: test `png_low_snapshot`. The PNG is decoded and its dimensions checked, and rendering twice gives identical sha256. X looks at the image once in BUILD-LOG. Every PNG is ≤ 300 KB.

**T3.10 Dist checks, accessibility, preview deploys**
- Deliverable: `site/scripts/check-dist.mjs` (gates G12 + G14), `pa11y-ci` config covering `/`, `/c/francis/`, `/codes/`, `/rules/`, `/table/`, and `ci.yml` jobs `site` + `preview`. The `preview` job runs `wrangler pages deploy site/dist --project-name stardivemeta --branch <pr-branch>` and posts the URL as a PR comment.
- `deploy.yml` (on push to `main`): the full pipeline, then deploy to branch `staging` **until launch**. The production branch deploy runs only when repo variable `LAUNCHED == "true"` (set in T6.4). It writes `dist/build.json = {commit, built_at, method_version, snapshot_id, sources_checked_at, codes_checked_at}`.
- Accept: a PR gets a working preview URL. pa11y reports 0 errors. Lighthouse (run by X via the browser tool on the preview, mobile) scores ≥ 95 on Performance and Accessibility for `/` and `/c/francis/`, recorded in BUILD-LOG.

**CP3 — Phone review** · Tom 20 min
- X sends the staging URL and a checklist of 8 taps: home at Low/High, one evidence dialog, Francis's page, Conquest, Table, Rules, Beginner, the PNG.
- Tom replies with "ok" or numbered fixes. Fixes inside the plan are done without another checkpoint; fixes outside the plan go to Part D-style notes in BUILD-LOG.

---

## P4 — Codes, banners, history, notes, errata

**T4.1 Codes page** (`codes.astro`, `CodeList.astro`, `src/scripts/codes-expiry.ts`)
- Status is computed in Rust (§B.3 rules). A ≤400-byte script moves any Active code whose `expires_at` has passed into Expired on the client, so a code never shows as Active after its stated expiry, even between builds.
- Accept: fixture `codes-fixture.toml` with 6 example codes (`EXAMPLE-ACTIVE`, `EXAMPLE-ONE-SOURCE`, `EXAMPLE-CONFLICT`, `EXAMPLE-SAME-GROUP`, `EXAMPLE-EXPIRED-DATE`, `EXAMPLE-EXPIRED-2SRC`) → statuses active / unconfirmed / unconfirmed / unconfirmed / expired / expired (test `codes_status_rules`). The page shows the "Last checked" stamp from `checks.json`.

**T4.2 New characters (banner) page** (`banner.astro`)
- Lists characters where `released_on` is within 45 days of the build date, **or** whose Overall coverage at Low is < 2. Each one links to `/c/<slug>/`, which shows "New" and the "x of 3 lists have placed them" line.
- Accept: the fixture with `released_on` for Vivian = 2026-09-30 and build date 2026-11-10 → Vivian is listed. Build date 2026-11-20 → not listed (coverage 2). With no qualifying characters, the empty-state copy shows.

**T4.3 History pages** (`history/index.astro`, `history/[id].astro`)
- Each snapshot page renders the frozen results with the same `TierBoard` component, through `adapters/snapshot-v1.ts` (one adapter per snapshot `contract_version`). It also shows "Changes from the previous snapshot" (from `diff`) and any errata linked to it.
- Accept: with 2 test snapshots, both pages render and the diff list is correct. Editing a committed snapshot fails CI (G9).

**T4.4 Movers note generator + pages** (`mongil-build note`, `notes/index.astro`, `notes/[date].astro`)
- `mongil-build note --from <last-note-snapshot> --to <latest> --date <YYYY-MM-DD>` writes `data/notes/<date>.md`. The frontmatter is `{from, to, status: draft}`. The body is generated entirely from the diff (template in §B.5 Notes), with one citation (source name, list date, URL) per move.
- Weeks with zero changes: the generator writes `status: auto` with the body "Nothing moved this week. Lists last checked {date}." This publishes without approval (D-12).
- Accept: test `note_from_diff_golden` (fixture diff → expected markdown byte for byte). Only `status: approved` or `status: auto` notes render (check-dist verifies no `draft` is built).

**T4.5 Errata + tripwire**
- `data/errata.toml` (§B.3) is rendered on the affected character page, the snapshot page and `/corrections/`. `mongil-build tripwire` exits 1 in either case:
  - any `severity = "major"` erratum dated within the last 7 days, without `review_done = true`;
  - 3 or more errata of any severity within 30 days of `launch_date` (from `data/ops/autodeploy.toml`).
- Exit 1 sets the auto lane to closed (T4.6).
- Accept: tests `tripwire_major_blocks`, `tripwire_three_in_month_one_blocks`, `tripwire_clear_passes`.

**T4.6 Lane classifier (auto-deploy guard)**
- `mongil-build lane --base origin/main` prints `data-only` or `needs-tom`. It prints `data-only` only if **all** of these hold:
  - every changed path is under `data/placements/`, `data/harvests/`, `data/codes.toml`, `data/state/`, `data/banners.toml`, `data/roster.toml` (additions only), or `data/snapshots/` (new files only), or is `data/notes/*.md` with `status: auto`;
  - `tripwire` passes;
  - `data/ops/autodeploy.toml` has `paused = false`;
  - fewer than 9 characters change Overall tier at any level (gate G5).
- X's merge script merges a PR only when CI is green **and** lane = `data-only`. Otherwise it labels the PR `needs-tom` and adds it to Tom's digest.
- Accept: 6 tests, one per condition, each flipping the result.

---

## P5 — Cadence dry run (7 days, staging only)

**T5.1 X's scheduled jobs** (in X's workspace; documented in `docs/RUNBOOK.md`)

| Job | When (ET) | What | Output |
|---|---|---|---|
| `sweep-am` | daily 09:00 | Read A of all voting + check-only sources. Compare with `data/placements`. If they differ: read B, harvest-diff, merge, snapshot. Always update `data/state/checks.json` (sources_checked_at). | PR `data/<sweep-id>` (always, because of the stamp) |
| `codes-scout` | daily 09:00 (same PR as sweep-am) | Observe all code sources. Update `data/codes.toml` observations and `checks.json` (codes_checked_at). | same PR |
| `sweep-pm` | daily 23:45 | Same as sweep-am but **no** stamp-only commits. Also commits if a code's `expires_at` passed since the last build. | PR only if something changed |
| `sweep-audit` | Mondays 09:00 (replaces sweep-am that day) | Full A **and** B read of every source, even when A shows no change. | PR (stamp + any changes) |
| `patch-watch` | inside every sweep | Check official news for a new version. If found: update `game.toml`, re-evaluate staleness, run an extra full A+B sweep within 24 h, report to Tom the same day. | PR + report |
| `notes-draft` | Mondays 09:30 | `mongil-build note` from the last published note's snapshot to the latest. Zero changes → `auto` note in the data PR. Changes → `notes/<date>` PR labelled `needs-tom`. | PR |
| `merge` | after each PR's CI | Merge if lane = `data-only` and CI is green. | — |
| `digest` | Mondays 10:00 | One message to Tom (format in T5.3), only if there's something to approve or report. | message |

Request politeness for every fetch: ≤ 1 request per 5 s per host, a fixed User-Agent `StarDiveMetaBot/1.0 (+https://stardivemeta.com/about/)`, and `robots.txt` honoured. If a host blocks the bot, the browser tool reads it as a normal visitor at the same rate. If both fail, the source is "unavailable" (playbook §B.4).

**T5.2 Health workflow** (`.github/workflows/health.yml`, cron `0 15 * * *` UTC = 10:00 or 11:00 ET)
- Checks:
  - (1) `https://<site>/build.json` returns 200;
  - (2) `build.json.commit` equals `main` HEAD, or HEAD is younger than 30 min;
  - (3) `codes_checked_at` is less than 30 h old;
  - (4) `sources_checked_at` is less than 30 h old (the sweep heartbeat, so a dead X workspace gets noticed);
  - (5) `/`, `/codes/` and `/c/narae/` return 200 and contain the disclaimer.
- Failure: open or update a GitHub issue titled `StarDiveMeta health: <check> failed`, labelled `health`. X's next job relays it to Tom with the project name. When all checks pass, the workflow closes any open `health` issue. There are no messages on green.
- Before launch, `<site>` = the staging URL. In T6.4 it switches to `stardivemeta.com`.
- Accept: a manual run with `codes_checked_at` faked to 2 days old opens an issue. A clean run closes it.

**T5.3 Report formats for Tom** (in RUNBOOK)
- **Same-day report** (new confirmed code, patch, source drop, breakage, health failure, major error). Max 5 lines:
  `StarDiveMeta — <what happened>. Live site: <still correct / changed>. Action for you: <none / tap link>.`
- **Monday digest.** Max 10 lines: the movers note PR link (if any), `needs-tom` PRs, and an estimate of Tom-minutes needed this week. It's skipped when empty.
- **Breakage report** (workflow §5): "What broke: <source/page>. What's live: data from <date>, still correct as of that date. Nothing was guessed. Fix: <plan>."

**T5.4 7-day dry run**
- All jobs run against staging for 7 consecutive days.
- Accept:
  - 14 sweeps logged;
  - 7 codes checks;
  - 1 sweep-audit;
  - 0 unexplained harvest-diff mismatches;
  - 0 health issues left open at the end;
  - X's logged effort for Tom ≤ 10 min for the week.
- X also runs a **breakage drill** once: feed the pipeline a Kaiden harvest with 5 characters missing. Expected: G4 fails, nothing merges, and a breakage report is drafted (not sent).

**T5.5 First movers note (CP4)** · Tom 10 min
- X generates a note for the dry-run week (staging snapshots). Tom approves or edits the PR on his phone. X also sends the staging codes page.
- Accept: Tom's approval is recorded. If Tom edited it, the edit is reflected in the template if it was generic (`docs/BUILD-LOG.md` notes which). The graduation counter starts at launch, not here.

---

## P6 — Launch

**T6.1 Final fresh harvest**: a sweep-audit (full A+B) within 24 h before launch. Accept: clean diffs, validate green.

**T6.2 Compliance pass**
- Deliverable: `docs/compliance.md`.
- Steps:
  - X re-reads Netmarble's current fan-content policy and records its URL, the date read and a short paraphrase of each rule. If no policy page is found, the input rules (handoff §2.1) apply and that fact is recorded.
  - Checklist, each item marked pass with evidence: disclaimer on every page (G12); no image files outside the allowlist (G14); no game art, logos or datamined content; no ads, affiliate links or donation links; non-commercial; no unreleased content; codes only from public sources.
- Accept: all items pass.

**T6.3 Search setup**
- `@astrojs/sitemap` → `/sitemap-index.xml`. `robots.txt` allows all and points to the sitemap. A canonical tag on every page. Titles in the format given in §B.5.
- Google Search Console and Bing Webmaster: X adds the DNS TXT verification records in Cloudflare, and Tom signs in once to each (free; 5 min each, inside CP5).
- Accept: both consoles show the property verified and the sitemap submitted.

**T6.4 Production switch**
- Steps:
  1. Attach `stardivemeta.com` and `www` (redirecting to the apex) to the Pages project.
  2. Set repo variable `LAUNCHED=true`.
  3. Set `launch_date` in `data/ops/autodeploy.toml`.
  4. Switch the health check target.
  5. Turn on Cloudflare Web Analytics (cookieless) for the domain.
- Accept: `https://stardivemeta.com/build.json` shows main HEAD. The health check passes on production. The `*.pages.dev` pages carry a canonical tag pointing to `.com`.

**T6.5 Launch snapshot**: snapshot id `<date>-launch`, method v1.0. Accept: `/history/<date>-launch/` renders.
- As run (2026-10-31, Tom's call): the launch-day sweep snapshot `2026-10-31T0900-sweep-am` serves as the launch snapshot, since the tool accepts only sweep ids and writes nothing when inputs are unchanged (BUILD-LOG). Accept: `/history/2026-10-31T0900-sweep-am/` renders.

**T6.6 Community posts (CP5)** · Tom 15 min (+10 for the search consoles)
- X reads the self-promotion rules of each target space (the official Discord's rules channel, r/MongilStarDive if it exists, any other community Tom names) and records them in `docs/launch-posts.md`, along with **draft** posts (copy in §B.5 Launch post) and the Low PNG.
- Tom posts them himself, since external publication is his call. X never posts.
- Accept: Tom's "posted" reply is logged with the place and date.

---

## P7 — Post-launch

- **Weekly:** the T5.1 jobs. Notes start at `status: draft` → Tom approves. After 4 approvals in a row **with no edits** (X compares the merged file to the drafted commit and keeps a count in `data/ops/notes-approval.toml`), X proposes auto-publish. Tom decides.
- **Monthly:** Dependabot (`.github/dependabot.yml`: cargo, npm and github-actions, monthly, grouped into one PR per ecosystem). X checks the preview and batches them into one `needs-tom` PR in the digest (Tom 3 min).
- **T7.1 Month-1 signal report** (day 30): visits, top pages and return-visit share from Cloudflare Web Analytics, against the bands (1,000+ / 300–1,000 / <300), labelled "early signal only". Tom 5 min.
- **T7.2 Two-patch-cycle review** (~6–8 weeks): accuracy record (errata), source health, traffic, and Tom's actual minutes per week. Defaults from the guardrails: flat traffic → passive running. Tom 30 min.
- **T7.3 v1.1 candidates, in this order:**
  1. mongil.gg as a 4th voting source (if T2.6 succeeded): re-simulate on real data, show the diff, Method v1.1.
  2. Reroll section (once T2.8 pool data is complete).
  3. Pull verdicts (once coverage matures).
  4. Filters (once tag data is verified).
  5. Tier-list maker + user PNG export (adds React + dnd-kit then).
  6. My Roster.
  7. 3D/animated experiments on their own routes.

# PART B — Specs

## B.1 Repo / workspace layout

```
stardivemeta/
├─ Cargo.toml                    # workspace
├─ rust-toolchain.toml
├─ crates/
│  ├─ mongil-core/               # model, method, derive, codes, snapshot, diff, contract types
│  │  ├─ src/{lib,model,load,method,derive,codes,snapshot,diff,validate,contract}.rs
│  │  └─ tests/
│  │     ├─ sim_v1.rs            # 41 golden tests (B.6)
│  │     ├─ invariants.rs        # 8 proptest invariants (B.6)
│  │     └─ fixtures/
│  │        ├─ dataset-2026-10-29/{roster,sources,method}.toml + placements/*.toml
│  │        ├─ sim-v1-expected.json
│  │        └─ codes-fixture.toml
│  ├─ mongil-render/             # SVG → PNG via resvg
│  │  ├─ src/lib.rs
│  │  └─ fonts/<OFL font files + OFL.txt>
│  └─ mongil-build/              # CLI (clap): validate emit snapshot diff harvest-diff png lane tripwire note
│     ├─ src/main.rs
│     └─ tests/gates.rs          # one test per gate G1–G14
├─ data/
│  ├─ method/v1.0.toml           # constants + conversion tables (immutable once launched)
│  ├─ roster.toml
│  ├─ sources.toml
│  ├─ game.toml
│  ├─ placements/{pg,kaiden,gamewith-jp,urgametips,gacha-gg,gamewith-en,mongil-gg}.toml
│  ├─ code-sources.toml
│  ├─ codes.toml
│  ├─ banners.toml
│  ├─ errata.toml
│  ├─ state/checks.json
│  ├─ harvests/<sweep-id>/<source>.{a,b}.toml
│  ├─ snapshots/<snapshot-id>.json          # append-only
│  ├─ notes/<YYYY-MM-DD>.md
│  └─ ops/{autodeploy.toml, notes-approval.toml}
├─ site/
│  ├─ package.json  pnpm-lock.yaml  astro.config.mjs  tsconfig.json  .nvmrc
│  ├─ src/
│  │  ├─ generated/              # gitignored: tierlist.json codes.json history.json meta.json
│  │  ├─ types/contract/*.ts     # ts-rs output, committed, CI-checked
│  │  ├─ adapters/snapshot-v1.ts
│  │  ├─ layouts/Base.astro
│  │  ├─ components/{TierBoard,CharacterChip,EvidencePanel,LevelToggle,Legend,Stamp,
│  │  │              Disclaimer,ModeBoard,TierTable,CodeList,NoteBody,ErrataList}.astro
│  │  ├─ scripts/{level.ts, dialog.ts, codes-expiry.ts}
│  │  ├─ styles/{tokens.css, global.css}
│  │  └─ pages/
│  │     ├─ index.astro  conquest.astro  rift.astro  table.astro  beginner.astro
│  │     ├─ codes.astro  banner.astro  rules.astro  about.astro  corrections.astro
│  │     ├─ data.astro  404.astro
│  │     ├─ c/[slug].astro
│  │     ├─ history/index.astro  history/[id].astro
│  │     └─ notes/index.astro  notes/[date].astro
│  ├─ public/{robots.txt, fonts/, img/own/}   # img/own = allowlisted original assets only
│  └─ scripts/check-dist.mjs
├─ docs/{PLAN-v1.md, DESIGN.md, RUNBOOK.md, harvest-guide.md, compliance.md,
│        launch-posts.md, BUILD-LOG.md, harvest-reports/, inputs/}
└─ .github/
   ├─ workflows/{ci.yml, deploy.yml, health.yml}
   └─ dependabot.yml
```

**Build order (CI and local):**
1. `cargo test --workspace`
2. `mongil-build validate --data data`
3. `mongil-build emit --data data --out site/src/generated`
4. `mongil-build png --data data --out site/public/img/generated`
5. `pnpm -C site astro check`
6. `pnpm -C site build`
7. `node site/scripts/check-dist.mjs site/dist`
8. `pa11y-ci`
9. deploy

## B.2 Rust → Astro JSON contract (`contract_version: 1`)

Files in `site/src/generated/`, also copied to `dist/data/`:

| File | Contents |
|---|---|
| `tierlist.json` | method, sources, characters with per-level results + evidence, level boards, mode boards, derived lists |
| `codes.json` | codes with computed status + observations |
| `history.json` | snapshot index + per-snapshot frozen results |
| `meta.json` | build info, check stamps |

Types (the TypeScript generated by ts-rs; Rust names are the same in snake_case):

```ts
type Level = "low" | "high" | "max";
type Tier = "S" | "A" | "B" | "C" | "D" | "MODE_SPLIT" | "UNRANKED";
type Label = "SOLID" | "CONTESTED" | "SPECULATION" | "MODE_SPLIT" | "UNRANKED";
type SourceRole = "voting" | "check_only";
type SourceStatus = "current" | "pre_patch_flagged" | "stale_check_only" | "unavailable";

interface TierList {
  contract_version: 1;
  method: { version: "1.0"; s_min: 3.5; a_min: 3.0; b_min: 2.0; c_min: 1.0;
            solid_max_spread: 1.0; mode_split_min: 2.0;
            conversion: Record<string, Record<string, number>>; };
  game_version: string;
  snapshot_id: string;
  sources: Source[];
  characters: Character[];
  boards: Record<Level, Record<Tier, string[]>>;          // ordered character ids
  mode_boards: Record<"conquest" | "rift", Record<"S"|"A"|"B"|"C"|"UNRANKED", string[]>>;
  derived: { beginner: { agreed_low: string[]; rise: { id: string; low: Tier; max: Tier; delta: number }[];
                         contested_low: string[]; reroll: null | { id: string }[] } };
}
interface Source { id: string; name: string; role: SourceRole; kind: "per_mode" | "overall" | "overall_by_investment";
  url: string; list_date: string | null; game_version: string | null; status: SourceStatus;
  criteria_caveat: string | null; level_definitions: Record<Level, string> | null; }
interface Character { id: string; name: string; released_on: string | null; is_new: boolean;
  results: Record<Level, Result>; modes: Record<"conquest" | "rift", { tier: "S"|"A"|"B"|"C"|"UNRANKED"; raw: string | null }>;
  evidence: EvidenceRow[]; errata: string[]; }
interface Result { tier: Tier; label: Label; score: number | null; score_display: string | null;
  coverage: number; spread: number | null; votes: Vote[];
  why: { high: Vote | null; low: Vote | null; mode_split: { conquest: number; rift: number } | null }; }
interface Vote { source: string; value: number; raw: string; }
interface EvidenceRow { source: string; role: SourceRole; counted: boolean;
  placement: { kind: "single"; raw: string; converted: number | null }
           | { kind: "modes"; conquest: string | null; rift: string | null; converted_overall: number | null }
           | { kind: "levels"; low: LevelCell; high: LevelCell; max: LevelCell }
           | { kind: "none"; reason: "no_placement" | "not_in_text_tiers" | "not_listed" };
  page_url: string; page_date: string | null; note: string | null; }
interface LevelCell { tier: string; raw: string; awakening: string | null; flags: ("P" | "E")[]; converted: number; }
```

**Example payload: Francis (base) from the 10-29 fixture (trimmed to Low + evidence):**

```json
{
  "id": "francis",
  "name": "Francis",
  "released_on": null,
  "is_new": false,
  "results": {
    "low": {
      "tier": "A", "label": "CONTESTED",
      "score": 3.1666666666666665, "score_display": "3.2",
      "coverage": 3, "spread": 2.0,
      "votes": [
        { "source": "pg", "value": 2.0, "raw": "Conquest B · Rift B" },
        { "source": "kaiden", "value": 3.5, "raw": "T0.5" },
        { "source": "gamewith-jp", "value": 4.0, "raw": "SS (at A6)" }
      ],
      "why": { "high": { "source": "gamewith-jp", "value": 4.0, "raw": "SS (at A6)" },
               "low":  { "source": "pg", "value": 2.0, "raw": "Conquest B · Rift B" },
               "mode_split": null }
    }
  },
  "modes": { "conquest": { "tier": "B", "raw": "B" }, "rift": { "tier": "B", "raw": "B" } },
  "evidence": [
    { "source": "pg", "role": "voting", "counted": true,
      "placement": { "kind": "modes", "conquest": "B", "rift": "B", "converted_overall": 2.0 },
      "page_url": "https://www.pocketgamer.com/mongil-star-dive/tier-list/", "page_date": "2026-10-05", "note": null },
    { "source": "kaiden", "role": "voting", "counted": true,
      "placement": { "kind": "levels",
        "low":  { "tier": "T0.5", "raw": "T0.5", "awakening": null, "flags": [], "converted": 3.5 },
        "high": { "tier": "T0.5", "raw": "T0.5", "awakening": null, "flags": [], "converted": 3.5 },
        "max":  { "tier": "T0.5", "raw": "T0.5", "awakening": null, "flags": [], "converted": 3.5 } },
      "page_url": "https://www.kaiden.gg/mongil/characters/<slug>/", "page_date": null, "note": null },
    { "source": "gamewith-jp", "role": "voting", "counted": true,
      "placement": { "kind": "single", "raw": "SS (at A6)", "converted": 4.0 },
      "page_url": "https://gamewith.jp/stardive/552850", "page_date": "2026-10-21",
      "note": "Rated at A6 (4★ criteria). The page names maxed Francis co-#1." },
    { "source": "urgametips", "role": "check_only", "counted": false,
      "placement": { "kind": "modes", "conquest": "B", "rift": null, "converted_overall": null },
      "page_url": "https://urgametips.com/mongil-star-dive-conquest-character-tier-list/", "page_date": "2026-09-07", "note": null },
    { "source": "gacha-gg", "role": "check_only", "counted": false,
      "placement": { "kind": "single", "raw": "A", "converted": null },
      "page_url": "https://www.gacha.gg/mongil-star-dive/tier-list", "page_date": "2026-10-29",
      "note": "Aggregate of 9 lists incl. Kaiden. Not counted, so nothing is counted twice." },
    { "source": "gamewith-en", "role": "check_only", "counted": false,
      "placement": { "kind": "single", "raw": "S", "converted": null },
      "page_url": "https://gamewith.net/mongil-stardive/73714", "page_date": null, "note": "Launch-era list (21 characters)." }
  ],
  "errata": []
}
```
(`<slug>` is Kaiden's real slug, recorded by T2.2. Placeholders like this are never emitted; G2 rejects any URL containing `<`.)

**Angel, Low (mode-split):**
`{"tier":"MODE_SPLIT","label":"MODE_SPLIT","score":2.1666666666666665,"score_display":"2.2","coverage":3,"spread":0.5,"why":{"high":null,"low":null,"mode_split":{"conquest":1.0,"rift":4.0}}}`

**`meta.json`:** `{"contract_version":1,"commit":"<sha>","built_at":"2026-11-20T14:04:10Z","method_version":"1.0","snapshot_id":"2026-11-20-am","sources_checked_at":"2026-11-20T14:00:00Z","codes_checked_at":"2026-11-20T14:00:00Z"}`

**`codes.json`** (made-up example codes, shown so the shape is clear):
```json
{ "checked_at": "2026-11-20T14:00:00Z",
  "codes": [
    { "code": "EXAMPLE-ACTIVE", "status": "active", "reward": { "text": "<as stated by source>", "source": "site-a" },
      "expires_at": null, "first_seen": "2026-11-18",
      "observations": [
        { "source": "site-a", "group": "site-a", "status": "active", "seen_at": "2026-11-20T14:00:00Z", "url": "https://…" },
        { "source": "official-discord", "group": "official", "status": "active", "seen_at": "2026-11-20T14:01:00Z", "url": "https://…" } ] } ] }
```

## B.3 Data file formats (with examples)

**`data/method/v1.0.toml`**
```toml
version = "1.0"
weights = "equal"
s_min = 3.5
a_min = 3.0
b_min = 2.0
c_min = 1.0
solid_max_spread = 1.0
mode_split_min = 2.0
kaiden_conditional_rule = "first_listed"   # D-G1
[conversion.pg]          S = 4.0; A = 3.0; B = 2.0; C = 1.0
[conversion.gamewith-jp] SS = 4.0; S = 3.0; A = 2.0
[conversion.kaiden]      T0 = 4.0; "T0.5" = 3.5; T1 = 3.0; T2 = 2.0; T3 = 1.0; T4 = 0.0
```
(Written one key per line in the real file. Compacted here.)

**`data/roster.toml`**
```toml
[[character]]
id = "francis-summer-dive"
name = "Francis: Summer Dive!"
base_of = "francis"                      # optional, variant link
released_on = "2026-08-xx"               # only if sourced; else omit
released_on_sources = ["https://…"]
# rarity / pool optional (T2.8), each with *_sources
```
Ids for all 29: `mina, penny, narae, sangun, jiwon, gabi, isabella, flare, chloe, ophelia, esther, nagi, brisshell, mabel, verna-summer-dive, francis-summer-dive, vivian, verna, cloud, francis, yeonhwa, daisy, leeho, benjamin, sera, ellie, reina, bonnie, angel`.

**`data/sources.toml`**
```toml
[[source]]
id = "pg"
name = "Pocket Gamer"
role = "voting"
kind = "per_mode"
url = "https://www.pocketgamer.com/mongil-star-dive/tier-list/"
list_date = "2026-10-05"
game_version = "1.04"            # as stated by the source; T2.3 maps to official
criteria_caveat = "Conquest = boss, single target. Rift = dungeon, AoE. World/Story list not used (Summer variants unlabeled)."

[[source]]
id = "kaiden"
name = "Kaiden"
role = "voting"
kind = "overall_by_investment"
url = "https://www.kaiden.gg/mongil/characters/"
criteria_caveat = "Rates each character at Low, High and Max investment. Where a level has two tiers, we count the first one listed."
level_definitions = { low = "…", high = "…", max = "…" }   # T2.4, optional

[[source]]
id = "gamewith-jp"
name = "GameWith JP"
role = "voting"
kind = "overall"
url = "https://gamewith.jp/stardive/552850"
list_date = "2026-10-21"
criteria_caveat = "Rates limited 5★ at A0, permanent 5★ up to A2, 4★ at A6. Its text lists show SS, S and A only."
# check-only entries: urgametips, gacha-gg, gamewith-en, mongil-gg — same fields, role = "check_only"
# excluded entries kept for the record (never shown): pocket-tactics, driffle — role = "excluded"
```

**`data/placements/pg.toml`**
```toml
[meta]
source = "pg"
harvest = "2026-11-03-am"
[[placement]]
character = "angel"
conquest = "C"
rift = "S"
[[placement]]
character = "brisshell"
status = "not_listed"          # in no Conquest or Rift row
```

**`data/placements/kaiden.toml`**
```toml
[meta]
source = "kaiden"
harvest = "2026-11-03-am"
[[placement]]
character = "mina"
page_url = "https://www.kaiden.gg/mongil/characters/mina/"
page_updated = "2026-10-19"     # omit if the page shows no date
low  = { tier = "T2",   raw = "T2" }
high = { tier = "T0.5", raw = "T0.5 (A4), T1 at A2", awakening = "A4" }
max  = { tier = "T0.5", raw = "T0.5" }
[[placement]]
character = "jiwon"
low  = { tier = "T3", raw = "T3 (P)", flags = ["P"] }
high = { tier = "T1", raw = "T1 (A4), T2 at A2 (P)", awakening = "A4", flags = ["P"] }
max  = { tier = "T1", raw = "T1" }
[[placement]]
character = "chloe"
status = "no_placement"
note = "Guide page says review coming soon. No Tier Position section."
```

**`data/placements/gamewith-jp.toml`**
```toml
[[placement]]
character = "francis"
tier = "SS"
raw = "SS (at A6)"
note = "Rated at A6 (4★ criteria). The page names maxed Francis co-#1."
[[placement]]
character = "sangun"
status = "not_in_text_tiers"
```

**`data/harvests/2026-11-03-am/kaiden.a.toml`**: the same schema as `placements/kaiden.toml`, plus `[meta] worker = "A"`, `read_mode = "text"`, `read_at = "…"`. The `.b.toml` has `worker = "B"`, `read_mode = "screenshot"`.

**`data/game.toml`**
```toml
current_version = "<official string>"
[[release]]
version = "<official string>"
date = "2026-09-30"
balance_changes = true
roster_change = ["vivian"]
source = "https://…official…"
```

**`data/code-sources.toml`**
```toml
[[code_source]]
id = "official-discord"
name = "MONGIL: STAR DIVE official Discord"
group = "official"
url = "https://…"
[[code_source]]
id = "site-a"
name = "…"
group = "site-a"         # same group = not independent
url = "https://…"
```

**`data/codes.toml`**
```toml
[[code]]
code = "EXAMPLE-ACTIVE"
reward = "<text as the source states it>"
reward_source = "site-a"
expires_at = "2026-12-01T00:00:00+09:00"   # only if a source states it
expires_source = "site-a"
[[code.observation]]
source = "site-a"
status = "active"           # active | expired | not_listed
seen_at = "2026-11-20T09:02:00-05:00"
url = "https://…"
```
**Code status rules (Rust, `codes.rs`)**, using each source's **latest** observation:
1. `expired` if `expires_at` < build time, **or** at least 2 distinct groups say `expired`.
2. `active` if at least 2 distinct groups say `active`, each seen within 7 days, **and** no group says `expired`.
3. otherwise `unconfirmed`.

**`data/banners.toml`**: `[[banner]] character = "vivian"`, `start = "2026-09-30"`, `end = "…"`, `source = "https://…"`.

**`data/errata.toml`**
```toml
[[erratum]]
id = "e-2026-12-02-1"
found_at = "2026-12-02"
fixed_at = "2026-12-02"
severity = "minor"           # major = wrong tier shown as fact, or an Active code that never worked
what = "Kaiden's High tier for Mabel was shown as T2. Kaiden says T1 (A4)."
characters = ["mabel"]
snapshots = ["2026-12-01-pm"]
review_done = false          # major only: true after the immediate review
```

**`data/state/checks.json`**: `{"sources_checked_at":"…","codes_checked_at":"…","last_sweep":"2026-11-20-am"}`

**`data/ops/autodeploy.toml`**: `paused = false`, `launch_date = "2026-11-24"`, `pause_reason = ""`.

**Snapshot `data/snapshots/2026-11-20-am.json`**
```json
{ "id": "2026-11-20-am", "contract_version": 1, "created_at": "2026-11-20T14:03:00Z",
  "method_version": "1.0", "game_version": "<official>",
  "sources": [ { "id": "pg", "list_date": "2026-10-05", "game_version": "1.04", "status": "current" } ],
  "inputs_sha256": "<sha of the canonicalised data/ inputs>",
  "inputs": { "placements": { "...": "frozen copy" }, "roster": [ "..." ] },
  "results": { "boards": { "...": "same shape as tierlist.json boards" },
               "characters": { "francis": { "low": { "tier": "A", "label": "CONTESTED", "score": 3.1667, "coverage": 3, "spread": 2.0 } } } } }
```

## B.4 Harvest guide (summary of `docs/harvest-guide.md`)

| Source | Where | Read | Record |
|---|---|---|---|
| Pocket Gamer | tier-list page, Conquest + Dimensional Rift sections | each row's names | `conquest`, `rift`, page date, stated version. **Ignore the World/Story list.** |
| Kaiden | each guide page `kaiden.gg/mongil/characters/<slug>/`, "Tier Position" | Low / High / Max lines verbatim | `tier` = first-listed tier per level; `awakening` from "(A#)"; `flags` from "(P)"/"(E)"; full line in `raw` |
| GameWith JP | `gamewith.jp/stardive/552850` SS/S/A text sections + (T2.5) visual table | names per tier | `tier`; caveat text; page date |
| urgametips | Conquest list | rows | `conquest` only |
| gacha.gg | tier list | rows | `tier` (S–D) |
| GameWith EN | `gamewith.net/mongil-stardive/73714` | SS/S/A | `tier` |
| mongil.gg | `/tierlist` images (T2.6) | per mode | `world`, `conquest`, `rift` |

**harvest-diff output** (stdout; exit 1 when blocked):
```
harvest-diff kaiden 2026-11-03-am
MISMATCH mina.high.tier        A="T0.5"  B="T1"
MISMATCH jiwon.low.flags       A=["P"]   B=[]
MISSING  chloe                 A=present B=absent
WARN     narae.low.raw         A="T0"    B="T0 "   (normalised equal)
3 blocking, 1 warning → update blocked
```

**Unavailable / stale / breakage playbook:**
- **A page 404s, is paywalled, or its format changed so the guide's fields can't be found** → that source's file is not updated. Its `status` becomes `unavailable` after 2 consecutive failed sweeps (24 h). From then on its votes are excluded and coverage is recalculated: Rust reads the status, so this is a data change, not a method change. A same-day report goes to Tom.
- **Balance patch** → any source whose `list_date` is before the patch date and that hasn't updated becomes `stale_check_only`. A same-day report goes to Tom.
- **Voting sources drop below 3** → the T2.6 mongil.gg read starts straight away. After one patch cycle still below 3 → a replacement-source proposal (Method v1.1) goes to Tom.

## B.5 Page content specs (v1)

Copy rules: plain words, short lines, no hype. `{…}` = filled from JSON. Every page ends with the footer:
> Fan-made site. Not an official MONGIL: STAR DIVE or Netmarble site.
> Game names and terms belong to their owners.

**Page titles** (`<title>`):
- Home: "MONGIL: STAR DIVE Tier List ({Month YYYY}) – StarDiveMeta"
- Codes: "MONGIL: STAR DIVE Codes ({Month YYYY}) – StarDiveMeta"
- Character: "{Name} Tier – MONGIL: STAR DIVE – StarDiveMeta"
- Others: "{Page} – StarDiveMeta"

### Home `/`
1. H1 "MONGIL: STAR DIVE tier list".
2. Sub: "Every list disagrees. We show you by how much."
3. Line: "Built from 3 tier lists: Pocket Gamer, Kaiden and GameWith JP. Tap a character to see what each list says."
4. Toggle "Investment: Low · High · Max".
   - Under it: "Only Kaiden rates by investment. The other two lists stay the same at every level."
   - Second line: "Watch who climbs as investment rises."
   - If T2.4 found definitions, a third line: "Kaiden's levels: {low def} / {high def} / {max def}."
5. Board rows in order: S, A, B, C, D, Mode-split, Unranked (rows with no characters are hidden).
   - Each chip: name + label glyph. Speculation chips get a dashed outline and "1 list".
6. Legend:
   - "● Solid — the lists agree within 1 tier."
   - "◐ Contested — the lists are more than 1 tier apart."
   - "○ Speculation — only 1 list has placed them so far."
   - "⇄ Mode-split — strong in one mode, weak in the other. See Conquest and Rift."
7. Stamp: "Lists last checked {date, time ET} · Method v1.0 · Game version {v}".
8. Links: "See the full table" · "How we score" · "What moved this week" · "Download this list (PNG)".
- Edge case, a balance patch with stale sources: a banner above the board: "A balance patch landed on {date}. {n} of 3 lists haven't updated yet, so some tiers rest on fewer lists."

### Character page `/c/{slug}/` (also the evidence dialog)
1. H1 "{Name}". If `is_new`: tag "New".
2. Summary line per level, with the current level highlighted:
   "Low: {tier} · {label}   High: …   Max: …"
3. Numbers (current level): "Score {score_display} of 4 · {coverage} lists · gap {spread}".
4. "Why {label}":
   - Solid: "{n} lists placed {Name}. Their calls are within {spread} of each other."
   - Contested: "{high source} says {high raw} ({high value}). {low source} says {low raw} ({low value}). That gap is more than 1.0."
   - Speculation: "Only {source} has placed {Name} so far."
   - Mode-split: "Pocket Gamer has {Name} at {c} in Conquest and {r} in Rift. That gap is too big for one Overall tier."
   - Unranked: "None of the lists we count has placed {Name} yet."
5. "What each list says" (counted), one row per source:
   - source name · what it published (raw) · our number · list date · link;
   - Kaiden's row shows Low / High / Max with the current level bold, plus P/E flags spelled out ("provisional", "exceptional");
   - a missing vote shows the reason: "No placement — review coming soon" / "Not in its text lists — not counted".
6. Criteria notes (each counted source's caveat, one line each).
7. "Not counted (shown for context)": the check-only rows, each with its reason ("Aggregate that includes Kaiden — counting it would count Kaiden twice", "Near-copy of Pocket Gamer's Conquest list", "Launch-era list", "Read from images — not counted until a method update").
8. Modes: "Conquest {tier} · Rift {tier} (Pocket Gamer only)".
9. Corrections for this character (if any): "{date}: {what}".
10. "How we score →" link.

### Conquest `/conquest/` and Rift `/rift/`
1. H1 "Conquest tier list" / "Dimensional Rift tier list".
2. "Only Pocket Gamer ranks characters by mode. These are its calls, not a consensus."
3. "Conquest = boss fights, single target." / "Rift = dungeons, many enemies."
4. Rows S, A, B, C, then "Not ranked by mode", sorted by name.
   - Each chip carries a small Overall label at Low (e.g. "Overall A ◐").
5. Stamp: "Pocket Gamer list dated {date}".

### Table `/table/`
1. H1 "Full table".
2. "Same data as the tier list, in one table."
3. Columns: Character · Low · High · Max (each "tier label score") · Conquest · Rift · Lists (coverage at Low).
   - Mode-split shows "⇄".
   - Sorted by Low score; mode-split rows last.
4. Stamp.

### Beginner `/beginner/`
1. H1 "Starting out".
2. "This is not pull advice. It's what the lists say about low investment."
3. "Lists agree these are strong at Low" → `agreed_low` (Low tier S or A **and** Solid).
4. "Get better with investment" → `rise` (Max tier higher than Low tier), each with "{Low tier} → {Max tier}".
5. "Lists disagree on these at Low" → `contested_low`, each linking to its evidence.
6. *(Only when `reroll` isn't null)* "On the permanent banner and strong at Low" → the list.
7. Link to Rules.

### Codes `/codes/`
1. H1 "MONGIL: STAR DIVE codes".
2. Stamp: "Last checked {date, time ET}. We check every day."
3. "Active" → "Two separate sites say these work."
   - Each: the code (monospace, copy-friendly), "Reward: {text} (per {source})", "Expires {date}" if stated, "Confirmed by {source 1}, {source 2}".
4. "Unconfirmed" → "Only one site lists these, or sites disagree. They may not work."
   - Each: the code + per-source status line ("{source}: active, seen {date}").
5. "Expired (last 30 days)" → collapsed `<details>`.
6. "How to redeem" → shown **only** if two sources give the same steps (T2.7). Otherwise omitted.
7. Empty state: "No codes are confirmed right now. Last checked {date}."

### New characters `/banner/`
1. H1 "New characters".
2. "We add a new character's placements as each list ranks them. No verdicts here — just what the lists say so far."
3. Per character: name · "{coverage} of 3 lists have placed them" · their current Low tier + label, or "Not placed yet" · link "See the evidence".
4. Empty state: "No new characters right now."

### Rules `/rules/` — draft copy (numbers come from the method file)
- **What we count.** "We count 3 tier lists: Pocket Gamer, Kaiden and GameWith JP. Each counts the same. We don't weight one list over another."
- **What we don't count.**
  - "gacha.gg combines other lists, including Kaiden, so counting it would count Kaiden twice."
  - "urgametips' Conquest list is close to a copy of Pocket Gamer's."
  - "GameWith's English list is from launch."
  - "We show these on each character's page, but they don't change scores."
  - "Pocket Tactics and Driffle are from before recent patches, so we leave them out."
  - "mongil.gg's lists are images. {If T2.6 passed: We read them and show them, but don't count them yet. / Else: We haven't read them yet.}"
- **One scale.** "Every list's tiers go on a 0–4 scale. We assume one tier step is worth one point on every list."
  - Then a table: Pocket Gamer S 4 · A 3 · B 2 · C 1. GameWith JP SS 4 · S 3 · A 2. Kaiden T0 4 · T0.5 3.5 · T1 3 · T2 2 · T3 1 · T4 0.
  - "If a list hasn't placed a character, that's no vote, not a zero."
- **Score and tier.**
  - "Score = the average of the votes."
  - "Pocket Gamer's vote is the average of its Conquest and Rift tiers."
  - "S is 3.5 and up · A 3.0 · B 2.0 · C 1.0 · D below 1.0."
- **Labels.**
  - "Solid: at least 2 lists, all within 1.0 of each other."
  - "Contested: at least 2 lists, more than 1.0 apart."
  - "Speculation: only 1 list."
  - "Unranked: no list yet."
- **Modes.**
  - "Only Pocket Gamer ranks by mode, so the Conquest and Rift pages show its calls alone."
  - "If its Conquest and Rift tiers are 2.0 or more apart, we don't give one Overall tier. We mark the character Mode-split."
- **Investment.**
  - "Kaiden rates each character at Low, High and Max. When you switch levels, only Kaiden's vote changes. Pocket Gamer and GameWith JP publish one list, so their votes stay the same."
  - "Where Kaiden gives two tiers for one level, we count the first one it lists."
  - "GameWith JP rates limited 5★ at A0, permanent 5★ up to A2, and 4★ at A6."
- **Known limits.**
  - "At Low, many characters are Contested. Part of that is real disagreement. Part of it comes from the two fixed lists not rating at Low."
  - "Mode views rest on one list."
- **When a list goes out of date.**
  - "We go by game version, not by calendar age."
  - "If a list predates a balance patch, it stops counting until it updates."
- **Checks.**
  - "We check the lists twice a day. Each change is read twice, by two separate checks, before it goes live."
  - "Codes need two separate sites."
- **Changes.**
  - "This is Method v1.0. If the rules change, the version number changes. Old snapshots keep the rules they were made with."

### History `/history/` and `/history/{id}/`
- **Index:**
  1. H1 "History".
  2. "Each time the data changes, we save a snapshot. We never edit old snapshots."
  3. List, newest first: "{date} · Method v{v} · {n} changes".
- **Snapshot:**
  1. H1 "Snapshot {date}".
  2. "Method v{v} · Game version {v}".
  3. The board with the toggle.
  4. "Changes from the previous snapshot" list.
  5. Corrections linked to this snapshot.

### Notes `/notes/` and `/notes/{date}/` (generated template)
```
# What moved: {from date} → {to date}
{n} changes. Each one links to the list that caused it.

## Tier changes
- {Name}: {old} → {new} at {Level}. Why: {source} changed {old raw} → {new raw} ({list date}, link).
## Label changes
- {Name}: {old label} → {new label} at {Level}. {one-line reason from diff}.
## New placements
- {source} placed {Name} at {raw} ({date}, link).
## Sources
- {source} is now {status} ({reason}).
```
Zero-change week: "Nothing moved this week. Lists last checked {date}."

### Corrections `/corrections/`
1. H1 "Corrections".
2. "When we get something wrong, we fix it the same day and list it here."
3. Entries: "{date found} · {what} · fixed {date}".
4. Empty state: "No corrections yet."

### Data `/data/`
1. H1 "Data".
2. "Everything on this site comes from these files. They are rebuilt with every update."
3. Links to `tierlist.json` and `codes.json`.
4. "Source lists belong to their publishers. We link to every one."

### About `/about/`
1. H1 "About".
2. "StarDiveMeta is a fan project by one player."
3. "It combines public tier lists and shows where they agree and where they don't."
4. "No ads. No accounts."
5. *(If Q4 is answered)* "Spotted a mistake? Email {address}."
6. Disclaimer, repeated in full.

### 404
"That page doesn't exist." plus a link to the tier list.

### Official PNG (per level) and OG image
- **Per-level PNG:**
  - Header: "MONGIL: STAR DIVE — consensus tier list".
  - Sub: "Investment: {Level} · Method v1.0 · 3 lists · {date}".
  - Rows S–D + Mode-split, with name chips + label glyphs.
  - Legend line.
  - Footer: "stardivemeta.com · Fan-made. Not affiliated with Netmarble."
- **OG image:** S and A rows at Low + the site name + the positioning line.

### Launch post draft (Tom posts; X adapts length per space's rules)
> I made a fan site that combines 3 MONGIL tier lists (Pocket Gamer, Kaiden, GameWith JP) into one.
> It shows where they agree, where they don't, and why — per character.
> There's a Low / High / Max investment switch and a codes page that only lists codes two sites confirm.
> stardivemeta.com — no ads, no sign-up. Corrections welcome.

## B.6 Regression tests (from `mongil-consensus-simulation-v1.txt`, verified by hand against the dataset)

Vote order in the simulation is [PG, Kaiden, JP]. I re-derived all 87 results from the dataset; every one matches the simulation file.

**Per-character tests** in `tests/sim_v1.rs`. Each test asserts tier / label / score (±0.005) / spread / coverage at Low | High | Max:

| Test | Low | High | Max |
|---|---|---|---|
| `sim_v1_angel` | MODE_SPLIT/MODE_SPLIT 2.17 s0.5 c3 | same | same |
| `sim_v1_benjamin` | B/Solid 2.83 s0.5 c3 | same | same |
| `sim_v1_bonnie` | C/Solid 1.00 s0 c2 | same | same |
| `sim_v1_brisshell` | S/Solid 3.50 s1 c2 | S/Solid 3.75 s0.5 c2 | S/Solid 3.75 s0.5 c2 |
| `sim_v1_chloe` | S/Solid 4.00 s0 c2 | same | same |
| `sim_v1_cloud` | B/Solid 2.33 s1 c3 | same | same |
| `sim_v1_daisy` | D/Solid 0.50 s1 c2 | same | same |
| `sim_v1_ellie` | B/Solid 2.25 s0.5 c2 | same | same |
| `sim_v1_esther` | A/Contested 3.17 s2 c3 | S/Solid 3.50 s1 c3 | S/Solid 3.50 s1 c3 |
| `sim_v1_flare` | B/Contested 2.17 s2.5 c3 | B/Contested 2.50 s1.5 c3 | B/Contested 2.83 s1.5 c3 |
| `sim_v1_francis` | A/Contested 3.17 s2 c3 | same | same |
| `sim_v1_francis_summer_dive` | A/Contested 3.00 s2 c2 | S/Solid 3.75 s0.5 c2 | S/Solid 4.00 s0 c2 |
| `sim_v1_gabi` | B/Contested 2.00 s2 c3 | B/Solid 2.67 s1 c3 | B/Solid 2.67 s1 c3 |
| `sim_v1_isabella` | S/Solid 3.50 s1 c3 | S/Solid 3.67 s0.5 c3 | S/Solid 3.67 s0.5 c3 |
| `sim_v1_jiwon` | B/Contested 2.50 s2.5 c3 | A/Solid 3.17 s0.5 c3 | A/Solid 3.17 s0.5 c3 |
| `sim_v1_leeho` | C/Solid 1.00 s0 c2 | same | same |
| `sim_v1_mabel` | B/Solid 2.50 s1 c2 | A/Solid 3.00 s0 c2 | A/Solid 3.00 s0 c2 |
| `sim_v1_mina` | A/Contested 3.33 s2 c3 | S/Solid 3.83 s0.5 c3 | S/Solid 3.83 s0.5 c3 |
| `sim_v1_nagi` | B/Contested 2.00 s2 c2 | B/Solid 2.50 s1 c2 | A/Solid 3.00 s0 c2 |
| `sim_v1_narae` | S/Solid 4.00 s0 c3 | same | same |
| `sim_v1_ophelia` | A/Contested 3.00 s2 c3 | A/Solid 3.33 s1 c3 | A/Solid 3.33 s1 c3 |
| `sim_v1_penny` | B/Contested 2.00 s2 c3 | B/Solid 2.33 s1 c3 | B/Solid 2.67 s1 c3 |
| `sim_v1_reina` | C/Solid 1.83 s0.5 c3 | same | same |
| `sim_v1_sangun` | C/Contested 1.75 s3.5 c2 | C/Contested 1.75 s3.5 c2 | B/Contested 2.25 s2.5 c2 |
| `sim_v1_sera` | C/Solid 1.50 s1 c3 | C/Solid 1.83 s0.5 c3 | C/Solid 1.83 s0.5 c3 |
| `sim_v1_verna` | C/Solid 1.00 s0 c2 | same | same |
| `sim_v1_verna_summer_dive` | A/Solid 3.25 s0.5 c2 | same | same |
| `sim_v1_vivian` | B/Solid 2.50 s1 c2 | A/Solid 3.25 s0.5 c2 | A/Solid 3.25 s0.5 c2 |
| `sim_v1_yeonhwa` | B/Contested 2.50 s2.5 c3 | same | same |

**Aggregate tests (12):**
1. `sim_v1_band_counts_low`: S4 A6 B11 C6 D1 ModeSplit1 Unranked0.
2. `sim_v1_band_counts_high`: S7 A6 B8 C6 D1 MS1.
3. `sim_v1_band_counts_max`: S7 A7 B8 C5 D1 MS1.
4. `sim_v1_label_counts`:
   - Low: Solid 16 · Contested 12 · ModeSplit 1 · Speculation 0 · Unranked 0.
   - High and Max: Solid 24 · Contested 4 · ModeSplit 1.
   - (The sim file's 17/25 counted Angel as Solid, see X10.)
5. `sim_v1_contested_sets`:
   - Low = {Esther, Flare, Francis, Francis: Summer Dive!, Gabi, Jiwon, Mina, Nagi, Ophelia, Penny, Sangun, Yeonhwa}.
   - High = Max = {Flare, Francis, Sangun, Yeonhwa}.
6. `sim_v1_board_order_low`:
   - S = [Narae, Chloe, Isabella, Brisshell].
   - A = [Mina, Verna: Summer Dive!, Esther, Francis, Ophelia, Francis: Summer Dive!].
7. `sim_v1_d_is_daisy_only`: D = [Daisy] at all 3 levels.
8. `sim_v1_widest_spreads`:
   - Low top 4: Sangun 3.5, then Flare / Jiwon / Yeonhwa 2.5.
   - Max top 2: Sangun 2.5, Yeonhwa 2.5.
9. `sim_v1_mode_conquest`:
   - S {Chloe, Esther, Flare, Mina, Narae}.
   - A {Ellie, Gabi, Isabella, Jiwon, Ophelia, Sangun}.
   - B {Benjamin, Cloud, Francis, Penny, Reina, Sera}.
   - C {Angel, Bonnie, Daisy, Leeho, Verna, Yeonhwa}.
   - Unranked {Brisshell, Francis: Summer Dive!, Mabel, Nagi, Verna: Summer Dive!, Vivian}.
10. `sim_v1_mode_rift`:
    - S {Angel, Chloe, Isabella, Jiwon, Mina, Narae, Sangun}.
    - A {Benjamin, Esther, Flare, Gabi, Ophelia}.
    - B {Cloud, Ellie, Francis, Penny}.
    - C {Bonnie, Daisy, Leeho, Reina, Sera, Verna, Yeonhwa}.
    - Unranked = the same 6.
11. `sim_v1_only_kaiden_moves`: for every character, PG and JP vote values are identical across the 3 levels.
12. `sim_v1_score_display`:
    - Francis Low "3.2", Sangun Low "1.8", Ellie "2.3", Brisshell High "3.8", Angel "2.2".
    - These match the Method §6 worked examples (3.2, 1.8).

Plus `derive_beginner_v1` (T3.7) and `sim_v1_missing_is_not_zero`: Chloe's coverage = 2 and her Kaiden evidence = `none/no_placement`; replacing it with T4 changes her score to 2.67 (proof the two cases differ).

**Invariants (`tests/invariants.rs`, proptest, 2,000 cases each):**
1. Score ∈ [0, 4].
2. The tier matches the band thresholds exactly, computed in integers.
3. Coverage = number of non-missing votes.
4. Label follows the rules from coverage and spread.
5. `score_display`, read back as a number, never lands in a different band than the score. Example: a value displayed "3.5" is never tier A. Tested for 1–6 voting sources with half-point votes. It holds up to 9 sources. At 10 a mean of 3.45 becomes possible, so adding sources beyond 6 requires revisiting this rule (noted in the rules-change checklist).
6. Mode-split occurs **only if** PG has both modes and |c − r| ≥ 2.0.
7. Removing a check-only source never changes any result.
8. Adding a new character never changes another character's result.

## B.7 Validation gates (all run in `mongil-build validate` + CI; any failure = no merge, no deploy)

| Gate | Rule |
|---|---|
| G1 Schema | Every data file parses with `deny_unknown_fields`. |
| G2 References | Every character / source / code-source id exists. Slugs are unique and match `^[a-z0-9-]+$`. No URL contains `<` or `…`. Every `game_version` maps to `game.toml` or is `version_unstated`. Every optional roster fact has its sources. |
| G3 Enums | Raw tiers belong to the source's allowed set (from `method/v1.0.toml` conversion keys). |
| G4 Coverage drop | A source's placed-character count may not fall by more than 2 vs `main`, unless each dropped character has a `status` with a `note`. (This catches page-format breakage.) |
| G5 Mass change | More than 8 characters changing Overall tier at any level in one PR → lane `needs-tom` (not a failure). |
| G6 Double-read | Every changed `data/placements/<s>.toml` has a matching `harvests/<sweep>/<s>.a.toml` + `.b.toml`. CI re-runs `harvest-diff` and requires 0 blocking mismatches. The merged file must equal A's structured fields. |
| G7 Method regression | All B.6 tests pass. |
| G8 Invariants | All B.6 invariants pass. |
| G9 Snapshot immutability | `git diff --diff-filter=MDR origin/main -- data/snapshots/` is empty. Same rule for `data/method/v*.toml` once `launch_date` is set. |
| G10 Codes | No duplicate code strings. Every observation's source exists. A code can't be `active` with `expires_at` in the past. `codes_checked_at` is not in the future. |
| G11 Stamps | `checks.json` times parse, aren't in the future, and aren't older than the previous value. |
| G12 Dist content | Every HTML page contains the disclaimer string. No `status: draft` note is rendered. JS per page ≤ 6 KB gzip. Every internal link resolves. |
| G13 Lane | `mongil-build lane` result is posted on the PR. Only `data-only` may be merged by X's script. |
| G14 Images | Every file under `site/public/img/` and `dist/**/*.{png,jpg,jpeg,webp,gif,svg,avif}` is in `site/public/img/own/ALLOWLIST.txt` or under `img/generated/`. This is the fan-rule guard against game art slipping in. |

# PART C — Risk register

| # | Risk | Where it breaks | Likelihood / impact | Cheap mitigation (built into this plan) |
|---|---|---|---|---|
| C1 | Kaiden changes its guide-page layout (29 pages, the fragile source) | P2 harvest, every sweep | High / Medium | G4 coverage-drop gate stops the update. The live site keeps the last good data, with its true stamp. A breakage report in plain words. The harvest guide gives the "Tier Position" anchor text, so the fix is one guide edit. The Monday full double-read catches quiet drift. |
| C2 | Both reads make the same mistake | the double-read | Medium / High (a published error) | Reader A uses page text and reader B uses a screenshot, so their errors are unlikely to match. G5 sends large changes to Tom. The same-day errata path. The tripwire is enforced in CI (T4.5), not by memory. |
| C3 | An Active code that never worked (a major error) | codes page | Medium / High | The two-group rule (copy-sites share a group). Observations must be within 7 days. Any "expired" report → Unconfirmed. The client script hides codes past their stated expiry. Default to Unconfirmed when in doubt. |
| C4 | Method credibility: "your Low view is broken" (12 of 29 Contested at Low) | public reaction | Medium / Medium | Toggle copy names the fixed-vote rule. The rules page has a "Known limits" paragraph that says the Low Contested count is partly from the method. Each character page shows the exact votes. |
| C5 | Outlier fights (Sangun gap 3.5, Francis 2.0) | public reaction | High / Low | That's the product: the evidence page shows the outlier and its source. No editorial override exists to argue about. |
| C6 | A balance patch makes 1–2 voting sources stale at once | staleness rules | Medium / Medium | Patch-watch inside every sweep, plus a full sweep within 24 h. The home banner explains thinner coverage. Below 3 sources, the mongil.gg read starts straight away (X5). |
| C7 | 1.04 vs 1.4 version confusion causes wrong staleness calls | T2.3 | Medium / Medium | Canonical version table from official notes before launch. G2 ties every source version to it. |
| C8 | Little traffic (new domain, small community) | discovery | High / Medium | 29 character pages + a dated codes title target real searches. Sitemap + Search Console/Bing at launch. OG card so links unfurl in Discord. Tom posts the PNG in community spaces within their rules (X checks the rules first). Bands stay "early signal only". Passive running is the default. |
| C9 | Tom's time goes past 30 min/week | approvals | Medium / High | A single Monday digest. Zero-change notes auto-publish. Dependency PRs batched monthly. The movers note is generated, so there's little to edit. X logs estimated Tom-minutes. Over 30 for 2 weeks → X proposes a cadence trim (e.g. drop sweep-pm). |
| C10 | X's workspace stops, so sweeps stop silently | ops | Medium / Medium | The GitHub-side health check tests the sweep heartbeat (`sources_checked_at` < 30 h) on its own, without X. |
| C11 | Game art slips in (fan-rule breach) | site assets | Low / High | No character images in v1. G14 image allowlist. Compliance pass T6.2. |
| C12 | A source blocks automated reads or objects (ToS) | harvest | Low–Medium / Medium | Polite rate (≤1 request/5 s/host), an honest bot UA with an about-page link, robots.txt honoured. The browser tool is the fallback. If a publisher objects, we stop reading them and follow the "unavailable" path. |
| C13 | Correlated "independent" sources (urgametips ≈ PG; gacha.gg includes Kaiden) | method | Known / Low | Already handled. Both are check-only, and the reason is printed. |
| C14 | Cloudflare Pages product changes | hosting | Low / Low | Static `dist/` + `wrangler` direct upload. Moving to Workers static assets is a one-step deploy change. |
| C15 | Astro/Tailwind major upgrades break the build | deps | Medium / Low | Exact pins. Monthly batched upgrade on a branch with a preview, so no surprise upgrades. |
| C16 | Single per-mode source makes the mode pages thin | method | Known / Medium | Labelled "Pocket Gamer only". The T2.6 mongil.gg read adds a check-only second view now and a v1.1 path to counting it. |

---

# PART D — Decision changes (every departure from the input)

Items marked **$** involve spending. Items marked **⚖** carry legal or publisher-rule exposure. Each item can be vetoed on its own; vetoing one leaves the rest of the plan intact.

## D-0 What I kept, and why (re-run on the real data)

The handoff invited a challenge to the five constants. I tested each alternative on the 10-29 dataset. **None is better, so all five are kept, and the method stays v1.0.**

| Alternative | Effect on the real data | Verdict |
|---|---|---|
| S band ≥ 3.3 (instead of 3.5) | S grows Low 4→5 (adds Mina 3.33, Contested), High 7→8 (adds Ophelia 3.33), Max 7→8 | Worse: it pulls Contested characters into S. **Keep 3.5.** |
| Solid ≤ 0.5 (instead of 1.0) | Contested goes Low 12→19, High 4→11, Max 4→10. Every one-tier gap between lists with different tier steps becomes "Contested" | Worse: the label stops meaning anything. **Keep 1.0.** |
| Kaiden T4 = 0.5 (instead of 0) | Sangun Low/High C→B (1.75→2.0). Daisy stays D (0.75) | Worse: it breaks "one tier step = one point" (T3 = 1). **Keep 0.** |
| Mode-split ≥ 1.5 or ≥ 3.0 | No change. Angel's gap is 3.0; the next largest PG gap is 1.0 | Same result for anything from 1.5 to 3.0. **Keep 2.0.** |
| Median instead of mean | Low S goes 4→7 (adds Mina, Esther, Francis), so Low ≈ High and the toggle barely moves anything. It throws away Kaiden's investment signal, the only vote that varies | Worse for this product. **Keep mean.** |
| Reputation weights | No published basis we could print and defend | **Keep equal.** |

## D-1 Changes

**D-1. Per-character evidence pages `/c/{slug}/` (29 pages).**
- Input: evidence lives in a drawer; per-character *build* pages are out of v1.
- Plan: the same evidence also renders as a page per character. No build or guide content.
- Why: search ("narae tier mongil") and linkable evidence (notes and corrections link to it). It costs nothing extra: it's one component.
- Risk: none for scope. "Thin page" SEO risk is low, because each page has unique evidence.
- Right if: Search Console shows impressions on `/c/` pages by the review. Checkable at T7.2.

**D-2. No React in v1 (Astro + about 2 KB of plain TypeScript).**
- Input: React islands for the toggle, board and drawer.
- Plan: all three levels are rendered as static HTML. A `data-level` attribute plus CSS switches between them. The evidence uses native `<dialog>`. React is added with the tier-list maker (v1.1+). `astro add react` is a one-line change, and the data layer is unaffected.
- Why: the pages work without JS, there's nothing to hydrate, there are fewer moving parts for agents to get wrong, and they're faster on phones.
- Risk: none to the future-proofing goal, because the island architecture stays available.
- Right if: v1 interactions stay at toggle + dialog. G12 enforces the ≤ 6 KB JS budget.

**D-3. Beginner view ships without "reroll".**
- Input: "Beginner / reroll view."
- Plan: the Beginner page is built only from computed data. The reroll section renders only once `rarity` and `pool` are two-source verified (T2.8).
- Why: reroll needs banner-pool facts that aren't in the dataset. Guessing them would break the evidence rule, and it leans toward pull advice, which is out of v1.
- Risk: less search traffic for "reroll" until the data exists.

**D-4. Toggle copy.**
- Input (Tom's tap): "Low = what a typical account has. Watch who climbs as investment rises."
- Plan: "Only Kaiden rates by investment. The other two lists stay the same at every level. / Watch who climbs as investment rises." Plus Kaiden's own level definitions if T2.4 finds them.
- Why: "typical account" is a claim with no data behind it. The new line also states the fixed-vote rule where readers need it.
- Risk: Tom preferred the original line. He can veto.

**D-5. Sweep design: read once, read twice only on change, plus a weekly full double-read.**
- Input: every harvest is extracted by one worker and re-read by a second.
- Plan: every sweep does read A. Read B runs whenever A differs from the current data. Nothing changes without two matching reads. A full A+B read runs every Monday.
- Why: about 70% less agent work, and the weekly audit catches cases where A misses a change.
- Risk: a change A misses could go unseen for up to 12 h (the next sweep) or, worst case, 7 days (the audit). Nothing wrong gets published.
- Right if: the dry run (T5.4) shows no audit-only catches. If an audit catches something A missed, X reports it and B becomes every-sweep for that source.

**D-6. Image-only lists read by agents. ⚖ (low)**
- Input: mongil.gg is "eligible the day a human reads them". JP's six absentees are "no vote".
- Plan: X's workers read both from screenshots (two independent reads, plus Tom's 5-tap spot-check for mongil.gg). JP's lower tiers count only after Tom's CP2 tap. mongil.gg shows as check-only until a v1.1 decision.
- Why: more evidence, and a possible second per-mode view.
- Risk: misidentifying a character from a portrait. Mitigated by the abandon rule in T2.6.

**D-7. Open Graph tags + OG image.**
- Input: no share buttons or social embeds.
- Plan: no buttons or embeds. Only `<meta property="og:*">` and `og.png`, so links Tom posts show a preview card.
- Why: manual community posting is the whole distribution plan, and a link with a card gets more clicks.
- Risk: none. Nothing loads from social platforms.

**D-8. A machine-readable data download (`/data/`). ⚖**
- Input: silent.
- Plan: publish `tierlist.json` and `codes.json`.
- Why: "shows its work" at full strength.
- Risk: it redistributes third-party placements in bulk (the same facts the pages already show, with links). Veto → remove `/data/` (T3.8). Nothing else depends on it.

**D-9. Corrections email via Cloudflare Email Routing (free). ⚖ (privacy, inbound only)**
- Input: no contact channel. External contact needs Tom's approval.
- Plan: `corrections@stardivemeta.com` forwards to the inbox Tom names (Part E Q4).
- Why: an evidence brand needs a way for readers to report errors.
- Risk: spam, and Tom-time if the forward goes to Tom. Veto → the line is omitted.

**D-10. Snapshots freeze their results; errata live in a separate file.**
- Input: snapshots are immutable; fixes are noted on the page/snapshot.
- Plan: a snapshot stores its inputs **and** its results. Fixes are new `errata.toml` entries shown next to the snapshot, never edits to it.
- Why: history stays exactly as it was published, even if code changes later.

**D-11. The tripwire is enforced by CI.**
- Input: a policy rule.
- Plan: `mongil-build tripwire` closes the auto lane on its own (T4.5).
- Why: it can't be forgotten.

**D-12. Zero-change weeks auto-publish "Nothing moved".**
- Input: Tom approves every note.
- Plan: notes that contain changes still need Tom. Empty-diff notes publish automatically.
- Why: saves Tom a tap on quiet weeks, and the note carries no claims.
- Risk: none. Veto → empty weeks publish no note at all.

**D-13. The movers note is generated from the diff, not written by a worker.**
- Input: "worker-drafted".
- Plan: a fixed template filled from `diff`, with one citation per line (T4.4).
- Why: no room for invented wording or hype, and it graduates to auto-publish more easily.

**D-14. Deploy by GitHub Actions direct upload (`wrangler pages deploy`).**
- Input: "GitHub Actions → Cloudflare Pages".
- Plan: build and every gate run in Actions. Cloudflare only receives the finished `dist/`. No Cloudflare Git build.
- Why: the gates live in one place, we don't depend on Cloudflare's build quota or build image, and moving host is simpler.
- Cost: free. Private-repo Actions use about 500 of the 2,000 free minutes a month (about 60 runs × ~6 min + 30 health checks × 1 min). **$ only if minutes exceed the free 2,000** (not expected). The fallback is making the repo public (unlimited minutes), which is Tom's call.

**D-15. Source-health timing (resolves X5).** The mongil.gg read starts straight away below 3 sources. The replacement proposal comes after one patch cycle.

**D-16. Staging-only until launch.** Production deploys are gated by `LAUNCHED=true`, so nothing half-built is indexed.

## D-G Gaps the input left open (filled, not departures)

| # | Gap | Filled with |
|---|---|---|
| G1 | Kaiden lines with two tiers for one level ("T0.5 (A2), T1 at A1"; Sera "T2 with Ophelia's signature, T3 without") | Count the **first-listed** tier; keep the rest in evidence. This is the rule the simulation used (it reproduces all 87 results). It's printed on the rules page. |
| G2 | Kaiden (P)/(E) flags | Votes count as normal. The flag is spelled out in evidence. |
| G3 | Order inside a tier | Score desc → coverage desc → name asc. Mode pages: name asc. |
| G4 | Display rounding | 1 decimal, half away from zero. Bands are decided on exact integers (invariant 5). |
| G5 | Angel's label | `MODE_SPLIT` (X10). |
| G6 | Code independence and freshness | `independence_group`; observations within 7 days; any "expired" → Unconfirmed. |
| G7 | Kaiden page dates | Recorded per character, since each guide page can be updated at a different time. |
| G8 | Snapshot trigger | Any change to placements, source metadata or roster. Stamp-only changes don't create one. |
| G9 | Which jobs run where | X's workspace: sweeps, codes, notes, digest, merges. GitHub Actions: CI, deploy, health. |
| G10 | "New character" for the banner page | `released_on` ≤ 45 days ago, or coverage < 2. |
| G11 | Note day and approval window | Draft Monday 09:30 ET. If it's not approved within 7 days, the next week's note covers both weeks and the old PR is closed. |
| G12 | Search consoles | Google + Bing, free. Tom signs in once (CP5). |
| G13 | Tom's approval mechanism | GitHub PRs with preview links, merged from the GitHub mobile app (Part E Q2). |

---

# PART E — Open questions for Tom (each blocks one task; tap the default to accept)

**Q1. Buy stardivemeta.com now? (blocks T6.4 launch) $**
- Default: **Yes**, buy the .com only at Cloudflare Registrar (~$10–11/yr at cost). Skip .gg (it's optional, and Cloudflare likely doesn't sell it).
- Do it at CP0 so DNS is ready. The site isn't public until launch.

**Q2. Approve things by merging GitHub PRs on your phone? (blocks T5.5 and every later approval)**
- Default: **Yes**. You get a PR link with a preview URL and tap "Merge" in the GitHub app. You need to be a collaborator on the repo.
- The alternative is X asking you in chat and merging for you; that's slower to audit.

**Q3. Who posts the launch messages in community spaces? (blocks T6.6)**
- Default: **You post X's drafts from your own accounts**. X never posts (handoff §2.4).
- Answer "skip" to launch with search traffic only.

**Q4. Where should corrections@ forward to? (blocks the T3.8 About copy; veto D-9 here)**
- Default: **none in v1**. The About page has no email line, and corrections come only from X's own checks.
- Name an inbox to turn it on.

---

## How X verifies the whole build end to end (summary)

1. `cargo test --workspace`: 41 sim + 8 invariant + gate tests are green.
2. `mongil-build validate` + `emit` + `png` on live `data/`: green, and output is deterministic (emit twice, compare sha256).
3. `pnpm -C site build` + `check-dist.mjs` + `pa11y-ci`: green.
4. Browser tool on the staging URL, recorded in BUILD-LOG:
   - Low S = 4 chips → High = 7;
   - Francis's evidence shows PG 2.0 / Kaiden 3.5 / JP 4.0;
   - Angel is under Mode-split;
   - with JS off, the Low board is complete;
   - Lighthouse mobile ≥ 95 for Performance and Accessibility.
5. 7-day dry run meets every T5.4 number, including the breakage drill.
6. After launch: `https://stardivemeta.com/build.json` commit = `main` HEAD, and the health workflow is green.
