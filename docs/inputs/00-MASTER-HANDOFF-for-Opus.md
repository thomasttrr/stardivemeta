# StarDiveMeta — Master Handoff for the Planning Pass (Opus 5.5)

**What this document is.** The complete record of a planning session held
2026-10-29 between Tom (owner), Kit (his agent), and Grok 4.7 (reviewer),
with Jev (decision aid) consulted at specific points. It contains: the
product idea, the verified data situation, the specification, the decisions
reached area by area (with reasoning), and a phased plan sketch.

**How to use it — read this first.**
- These decisions are **strong defaults, NOT permanent locks**. Tom's
  instruction for this planning pass: build the best plan you can from
  this material. If you see a better approach in any area, propose it —
  with the trade-off stated plainly — rather than following a weaker
  decision out of politeness. Where you disagree, say so and say why.
- A few items are marked **HARD CONSTRAINT**. Those come from outside the
  project (publisher fan-content rules, the owner's no-spending-without-
  approval rule, honesty rules about data). Do not plan around them;
  flag conflicts instead.
- Everything else — the math constants, the stack details, the cadence,
  the scope lines — is open to improvement if you can argue it.
- **Audiences and tone:** the site is for MONGIL: STAR DIVE players
  (a small community). The owner reviews on a phone, prefers plain
  language, short scannable writing, and options he can tap. Any
  user-facing copy you draft should match: plain words, no hype.
- **Evidence rule (owner's standing rule):** claims in the plan must be
  grounded in the data summarized in §3 or flagged as unverified.
  Never invent placements, numbers, dates, or competitor features.

---

## 1. The product in one paragraph

StarDiveMeta is a fan-made consensus tier-list site for the gacha game
MONGIL: STAR DIVE. Every existing tier list is one site's opinion.
StarDiveMeta instead combines the placements of several trusted,
independent lists into one computed consensus — and, crucially, **shows
its work**: every character carries an evidence view (each source's
placement, its date, its criteria caveats), a confidence label
(Solid / Contested / Speculation) derived from source agreement, splits
by game mode (Conquest / Rift) instead of one blended ranking, a
re-computation by player investment level (Low / High / Max), and
historical snapshots so readers can see what changed and when. Research
on 2026-10-29 verified that no major gacha guide site currently offers
the combination of visible per-source evidence, computed confidence,
mode splits, investment splits, and snapshots. That transparency is the
product's edge — the site is an evidence brand, not an opinion blog.

**Name:** StarDiveMeta — stardivemeta.com (see §8).
**One-line positioning:** "Every list disagrees. We show you by how much."

---

## 2. HARD CONSTRAINTS (do not plan around these)

1. **Netmarble fan-content rules.** Non-commercial at launch (website
   display ads are a gray zone — assume no ads in v1). Screenshots and
   crops only: no ripped game files/art, no datamined or unreleased
   content. A visible "not an official site" disclaimer is required.
   Animated characters or 3D models of game characters may only ever use
   original or properly licensed art — never extracted game assets.
2. **No spending without Tom's explicit approval.** The domain purchase
   (~$10–15/yr) is pending and is Tom's action. Hosting is free-tier.
   No paid services, no paid licenses in the plan without flagging them
   as approval items.
3. **Honesty rules for data.** A fact shown on the site needs two
   independent sources or an explicit "unconfirmed" label. Harvested
   data is never guessed, reconstructed from memory, or filled in from
   an aggregator that would double-count its own inputs.
4. **Tom approves:** anything touching code/design/method that departs
   from the approved plan, the weekly movers note (until it graduates,
   see §7), method-version changes, and any external contact or
   publication beyond the site itself.
5. **No social-media integration requirement** — distribution is manual
   community posting; do not design share-button/social-embed systems
   into v1 (owner's standing preference across his sites).

---

## 3. The data situation (verified 2026-10-29)

Full dataset: `mongil-tier-list-consensus-2026-10-29.md` (source of
truth; the per-character placements live there, not in this document).

- **Roster: 29 characters.** Every character now has **at least two
  independent, current sources**. The dataset is complete enough to
  build a full list at launch — this was the gating research question
  and it passed.
- **Voting sources (3):**
  - **Pocket Gamer (PG)** — per-mode lists (Conquest / Rift), game
    v1.04, Oct 5 2026. Covers 23 of 29 in its endgame modes; the six
    newest characters have no per-mode ranking anywhere (known gap).
  - **Kaiden (K)** — overall, split by investment (Low / High / Max),
    harvested from per-character guide pages (their tier page itself
    exposes no usable text). 28 of 29 ranked; Chloe is unranked there
    ("review coming soon").
  - **GameWith JP (JP)** — overall, all 29, Oct 21 2026. Criteria
    caveat, printed in its evidence entry: limited 5★ rated at A0,
    permanent 5★ up to A2, 4★ at A6. Its placements for six characters
    (Sangun, Ellie, Bonnie, Leeho, Verna base, Daisy) do not appear in
    its text tiers — treated as **no vote**, never as a low vote.
- **Check-only sources (shown in evidence, weight 0):** gacha.gg
  (a real aggregate — median of 9 lists, 24 chars — but it aggregates
  Kaiden and mongil.gg, so counting it would double-count), urgametips
  (Conquest list near-identical to PG's), GameWith EN (launch-era,
  superseded by JP).
- **Excluded:** Pocket Tactics (~Aug 2026) and Driffle (~Jun 2026) —
  stale, pre-patch; snapshot history only. **mongil.gg** — the only
  other per-mode source; its placements are images, unread by machine;
  eligible as a voting source the day a human reads them.
- **Codes (for the codes page):** 8+ codes reported active across the
  web with heavy search demand, BUT statuses conflict between sites
  (e.g., MONGILDISCORD: active per Pocket Gamer, expired per
  SuperCheats). Hence the two-source rule in §6.
- **Known live disagreements the method must handle honestly:**
  Francis (base) — PG B/B vs Kaiden T0.5 vs JP SS-at-A6 (investment
  framing split; widest on the roster). Sangun — PG A/S vs Kaiden T4.
  Yeonhwa — PG C/C vs Kaiden T0.5 vs JP S. Flare — JP's A is the low
  outlier; Kaiden shows she's investment-dependent. Nagi — Kaiden harsh
  at Low, JP rates S at A0.

---

## 4. AREA SUMMARY — Consensus method ("Method v1.0")

Full text: `mongil-consensus-math-proposal-v1.md`.
Decided 2026-10-29 after a full 29-character simulation (below).

- **Weights:** equal (1.0) per voting source. No reputation weighting —
  equal is the only weighting defensible on a public rules page.
- **Conversion to a shared 0–4 scale** (assumption printed on the site:
  one tier step = one point on every list):
  PG S=4 A=3 B=2 C=1 · JP SS=4 S=3 A=2 (absent from JP text tiers =
  NO vote, never zero) · Kaiden T0=4, T0.5=3.5, T1=3, T2=2, T3=1, T4=0.
  Missing is never zero; zero comes only from a real T4.
- **Overall score** = mean of available votes. PG's Overall vote =
  average of its Conquest and Rift scores (when both exist).
- **Tier bands:** S ≥ 3.5 · A ≥ 3.0 · B ≥ 2.0 · C ≥ 1.0 · D < 1.0.
- **Confidence label** from two axes — coverage (how many voting
  sources placed the character) and spread (highest − lowest vote):
  coverage 0 → Unranked · coverage 1 → Speculation · coverage ≥ 2 and
  spread ≤ 1.0 → Solid · spread > 1.0 → Contested.
- **Mode views** (Conquest / Rift) show PG's placements only, labeled
  as single-source — the UI must not dress them up as consensus.
  **Mode-split override:** if PG's Conquest vs Rift scores differ by
  ≥ 2.0, no Overall tier is shown; the character is listed as
  Mode-split, per-mode only. (Live example: Angel, Conquest 1 vs
  Rift 4.)
- **Investment toggle re-runs all the math** at Low / High / Max.
  Only Kaiden's vote changes between levels; PG and JP publish fixed
  lists. This "fixed-vote rule" is printed on the page, as is the
  conversion assumption.
- **Staleness is by game version / balance patch, not calendar days.**
  A source predating a balance patch drops to check-only (weight 0)
  until it updates; coverage is recalculated.
- **Simulation results (run before deciding; file
  `mongil-consensus-simulation-v1.txt`):** clean run, no absurd
  placements. S tier: 4 at Low (Narae, Chloe, Isabella, Brisshell) →
  7 at High/Max (+ Mina, Esther, Francis: Summer Dive!). D = Daisy
  only, at every level. Angel mode-split at every level. Contested:
  12 at Low → 4 at High/Max (sources mostly disagree about
  low-investment value — a finding the site can say out loud).
  No Speculation labels at launch (minimum coverage is 2).
  Watch items, none blocking: Sangun (spread 3.5), Francis (JP's
  maxed-based SS vote appears in the Low view — explained by the
  printed fixed-vote rule; he lands A/Contested), Chloe (thinnest S —
  coverage 2, no Kaiden vote).
- **Versioning:** the method is published as "Method v1.0". If the
  rules ever change, old snapshots keep their old method label —
  history is never rewritten under new rules.
- **Open for the planning pass:** are equal weights and the five
  constants (S ≥ 3.5, Solid ≤ 1.0, T4 = 0, split ≥ 2.0) the best
  choices, or is there a demonstrably better scheme? Grok 4.7 reviewed
  the draft and kept all five unchanged; a stronger argument either
  way is welcome, but any change must be re-simulated on the real
  dataset before adoption.

---

## 5. AREA SUMMARY — Technology stack ("Stack v2")

Full text: `mongil-stack-v2-proposal.md`. Decided in two steps:
originally a Rust-only front end; **revised the same day at Tom's
request** for complete future-proofing insurance (rich animation,
possible 3D, floating animated characters, no outdated-stack regret).

- **Layer 1 — Rust core (data + method).** A Rust CLI (`mongil-build`)
  with a shared core crate: Serde-typed data model (characters, sources,
  placements, timestamps, snapshots), the Method v1.0 computation,
  validation (a malformed harvest fails the build, loudly), snapshot
  writing, and JSON emission for the front end. Rationale: the heavy
  correctness risk in this project is data-shape and math errors at
  build time; types + tests are the mitigation. This layer is
  framework-independent and is expected to outlive any front end.
- **Layer 2 — Astro site shell.** Astro + TypeScript, static output,
  near-zero JavaScript by default. Pages: home/tier list, per-mode
  views, codes, rules/method, beginner/reroll, banner evidence,
  movers-note archive, table view. Deployed to Cloudflare Pages (§8).
- **Layer 3 — React islands.** Interactive features mount as React +
  TypeScript islands fed by the Rust build's JSON: the investment
  toggle + tier board, the evidence drawer, filters (later), the
  future tier-list maker. Vanilla TypeScript is fine for tiny widgets.
  Any island can be replaced without touching the data layer or other
  islands (Astro hosts multiple frameworks; we standardize on React).
- **Styling:** Tailwind CSS + CSS custom-property design tokens, with
  a project DESIGN.md created before UI work (owner's rule: one locked,
  non-generic visual direction per project; his reference sources are
  Designeer finds, getdesign.md analyses, and AI UX Playground
  prompts — adapt patterns, never copy assets or code wholesale).
- **Animation:** CSS first; GSAP (fully free incl. all plugins since
  Apr 2025) for choreography; Motion for React component animation.
  `prefers-reduced-motion` respected. Per-page JavaScript budget;
  heavy features only on their own routes.
- **Future 3D (post-v1, own route only):** Three.js, WebGPU renderer
  with automatic WebGL2 fallback, glTF/GLB + Draco/KTX2 compression,
  LOD + on-demand rendering for mobile. Owner has prior PMX→GLB
  pipeline experience incl. mobile z-fighting lessons (delete coplanar
  invisible shells; offset face decals along normals) — reuse them.
- **Future animated 2D characters (post-v1):** Rive (original mascot,
  interactive state machines, tiny files) preferred; Lottie for motion
  spots; Live2D via the official Cubism Web SDK only if properly
  sourced models ever exist — gated by §2 constraint 1 and a Live2D
  license recheck at the time.
- **Future-proofing logic:** durability is layered, not picked —
  data/method in Rust + JSON (decades), web standards (HTML/CSS,
  WebGL/WebGPU, glTF, WASM) over vendor bets, and a replaceable shell.
  Versions pinned; dependency-freshness checked on a schedule;
  upgrade on branches with preview verification (owner's standing
  "newest stack" rule).
- **Explicitly NOT in the stack:** no server, no database, no user
  accounts in v1 (all state is build-time or browser-local), no
  analytics beyond Cloudflare's cookieless Web Analytics.
- **Open for the planning pass:** exact repo/workspace layout, the
  JSON contract between Rust and Astro, build pipeline steps
  (GitHub Actions → Cloudflare Pages), test strategy for the method
  math (the simulation must become a regression test), and whether
  the official PNG is generated by the Rust side (resvg) or the
  Astro side.

---

## 6. AREA SUMMARY — V1 scope ("Package B")

Full text: `mongil-v1-scope-locked.md`.

**IN v1:**
- Tier list with Overall / Conquest / Rift views, Method v1.0,
  evidence view per character, confidence labels, snapshots/history.
- Investment toggle (Low default — §9), re-running the math.
- Beginner / reroll view (derived from the same computed data).
- Published rules page (method, conversion assumption, fixed-vote
  rule, source criteria caveats).
- **Codes page** — only codes confirmed by two independent sources
  list as Active; a clearly separated Unconfirmed section for the
  rest; a visible "last checked" stamp updated daily.
- **Banner evidence page** — for a newly released character: first
  placements + coverage as sources weigh in. **No pull verdict, no
  recommendation** (see OUT list for why).
- Table view of the full list (same data, accessibility + skimmability).
- Official list PNG, generated at build time (the shareable artifact
  for community posting).
- Weekly "what moved" note (see §7 for its approval workflow).

**OUT of v1 (deliberately, with reasons):**
- **Pull advice / pull-value verdicts** — a new banner character
  launches at coverage 1 (Speculation), so verdicts would be weakest
  exactly when demand peaks, and the method hasn't proven itself in
  public yet. Revisit for v1.1 once coverage matures.
- **Filters** (mechanic/element tags) — tag data is not yet harvested
  or verified; unverified tags would damage an evidence brand.
- **User PNG export** — arrives with the tier-list maker.
- **Tier-list maker, My Roster mode, per-character build pages** —
  post-v1 features; the maker is planned on dnd-kit + the island
  architecture when it comes.
- **Accounts, comments, any server feature** — out by design (§5).

**Open for the planning pass:** page-by-page content spec (what each
page shows, in what order, with what copy), the evidence drawer's exact
contents, snapshot browsing UX, and the beginner view's derivation
rules — all currently at summary level, not build level.

---

## 7. AREA SUMMARY — Update workflow

Full text: `mongil-update-workflow-proposal.md`.

- **Source sweep TWICE DAILY, 09:00 ET and 23:45 ET** (owner's
  amendment — originally a weekly sweep): a worker re-checks all three
  voting sources + check-only sources; silent when nothing changed.
  Plus a **patch-triggered extra sweep within 24h** of any game
  version / balance patch.
- **Banner characters:** evidence page goes up with first placements
  immediately; the twice-daily sweeps re-check until ≥ 2 voting
  sources rank the character.
- **Double-read rule:** every harvest is extracted by one worker and
  independently re-read by a second; a mismatch blocks the update.
  Nothing is ever harvested from memory or guessed.
- **Auto-deploy lane:** data-only changes (placements, timestamps,
  codes) auto-commit and deploy when the build + validation pass.
  Anything touching code, design, or the method goes to a branch for
  Tom's approval first.
- **Codes scout daily ~09:00 ET**, under the two-source rule; a code
  expires on two-source agreement or its stated expiry; the page
  stamp always shows the last check; new confirmed codes are reported
  to Tom the same day; quiet days are silent.
- **Weekly movers note:** auto-diffed from snapshots, worker-drafted
  with a citation per move, **Tom approves before publish**.
  Graduation rule: after 4 consecutive notes approved without edits,
  switching to auto-publish may be proposed (Tom's call then).
- **Stale/paywall playbook:** follows §4 staleness rules — missed
  patch → flagged on the evidence entry; missed balance patch →
  drops to check-only automatically, Tom told the same day. Replacing
  a voting source is a method change (version bump, Tom approves,
  old snapshots keep their label). Below 3 voting sources, reading
  mongil.gg's image lists becomes the top-priority task.
- **Health check daily:** site live, latest deploy matches repo,
  codes stamp current. Silent when green; failures reported with the
  project name.
- **Harvest breakage** (a source changes its page format — Kaiden's
  guide pages are the fragile one): the update stops, nothing is
  guessed, Tom gets a plain report of what broke and what is current.

**Open for the planning pass:** the concrete automation design —
which jobs run where (GitHub Actions vs the owner's agent workers),
data-file layout for placements/snapshots, the double-read's diff
format, and failure alerting. Sized to stay inside Tom's ~30 min/week
attention budget (§8 guardrails).

---

## 8. AREA SUMMARY — Name, hosting, and launch guardrails

Full text: `mongil-launch-guardrails-proposal.md`.

- **Name: StarDiveMeta.** Registry checks 2026-10-29 (Verisign RDAP +
  rdap.org, repeated): stardivemeta.com and stardivemeta.gg both
  returned "not registered" — they look available, but only
  registering proves it. **Not yet bought** — purchase is Tom's action
  (§2 constraint 2). Buy via Cloudflare Registrar so domain + DNS +
  hosting live in one place. (.gg is an optional defensive extra.)
- **Hosting: Cloudflare Pages** (free). Deciding factors: unlimited
  free bandwidth with no pause-at-cap (Vercel Hobby pauses at
  100 GB / 1M requests — worst exactly during a traffic spike), no
  commercial-use restriction on the free tier (Vercel Hobby is
  non-commercial only), open preview links (Vercel previews are
  SSO-gated — known friction for this owner), free cookieless
  analytics. ~40 builds/month expected vs 500 free. Automation is
  host-independent (GitHub Actions + workers); the host only serves
  the static folder.
- **Accuracy outranks growth** (the guardrail everything else hangs
  on): any published error is fixed the same day it's found, with the
  fix noted on the page/snapshot.
- **Severity-split tripwire:** errors caught pre-publish by the
  double-read never count. ONE major published error (a wrong tier
  presented as fact; a code listed Active that never worked) →
  immediate review. THREE published errors of any kind in month 1 →
  data auto-deploy pauses; updates return to Tom-approval until the
  cause is fixed.
- **Traffic judgment:** month-1 numbers are EARLY SIGNAL ONLY (a new
  domain can take weeks to index). Bands: 1,000+ visits = working;
  300–1,000 = distribution problem (push the PNG + movers note into
  the game's Discord/community spaces within their self-promo rules;
  check indexing); under 300 = flat. **Real judgment happens at the
  two-patch-cycle review (~6–8 weeks live).** Flat traffic defaults
  to **passive running** at near-zero cost — shutdown only by Tom's
  active decision, never automatic.
- **Source-health trigger:** below 3 voting sources for over one
  patch cycle → mongil.gg image harvest + a replacement-source
  proposal (a Method v1.1 decision for Tom).
- **Budgets:** running cost $0/month hosting + the yearly domain.
  Tom's time ~30 minutes/week; if approvals exceed that, automate
  more or trim cadence — the site must not become a second job.

---

## 9. Small decisions worth not losing

- **Investment default: Low** for first-time visitors. The island
  remembers a returning visitor's last choice in browser-local
  storage. Copy line under the toggle: "Low = what a typical account
  has. Watch who climbs as investment rises." Rationale: most players
  are low-investment; S is honestly sparse at Low (4 vs 7), and the
  climb is the site's argument in one tap.
- **Evidence before verdicts, everywhere.** Where the site cannot
  support a claim, it shows the gap (Unranked, Speculation,
  Unconfirmed) instead of filling it. This is the brand.
- **Snapshots are immutable and method-labeled** (§4).
- **Silence is a feature of the workflow** — workers and checks report
  changes and problems, not routine success (owner's standing rule
  across his projects).
- **The owner is a solo developer.** No team/enterprise assumptions
  anywhere in the plan (tooling, pricing tiers, review processes).

---

## 10. Plan sketch (phases — for you to improve, not obey)

1. **Foundations:** register domain (Tom), repo + workspace layout,
   DESIGN.md (direction lock), Rust core skeleton with the data model,
   the simulation converted into regression tests.
2. **Data pipeline:** placement/snapshot file formats, the harvest +
   double-read workflow (initially manual/agent-run), validation gates.
3. **Site v1:** Astro shell + tier board island + evidence drawer +
   rules page + table view; build-time PNG; Cloudflare Pages deploy
   via GitHub Actions.
4. **Codes + banner pages:** codes page with two-source workflow and
   daily scout; banner evidence page template.
5. **Cadence live:** twice-daily sweeps, daily health check, weekly
   movers note with approval loop.
6. **Launch:** final data check, disclaimer + fan-rule compliance pass,
   community introduction posts (manual, within self-promo rules).
7. **Post-launch:** two-patch-cycle review against §8 guardrails;
   v1.1 candidates in order of current thinking: pull verdicts (once
   coverage matures), filters (once tag data is verified), tier-list
   maker + user PNG export, My Roster, mongil.gg as a 4th voting
   source (once read), 3D/animated-character experiments on their own
   routes.

## 11. What we need from this planning pass

1. A build-ready plan: phases, task order, dependencies, and what
   "done" means for each task.
2. The repo/workspace layout and the Rust↔Astro JSON contract.
3. A per-page content spec for every v1 page in §6.
4. A risk pass: where is this plan most likely to break (data
   fragility, method credibility, discovery/traffic, owner time) and
   what mitigations are cheap now?
5. Challenges, where earned: any decision in §4–§9 you would change,
   with the argument and the trade-off, flagged clearly as a proposed
   change for Tom — not silently baked in.
6. Estimates in owner-time and calendar time per phase, assuming a
   solo owner + AI-agent workers doing most implementation.

*Source files (same folder / workspace): the dataset
(`mongil-tier-list-consensus-2026-10-29.md`), Method v1.0
(`mongil-consensus-math-proposal-v1.md`), the simulation output
(`mongil-consensus-simulation-v1.txt`), scope (`mongil-v1-scope-locked.md`),
stack (`mongil-stack-v2-proposal.md`), workflow
(`mongil-update-workflow-proposal.md`), guardrails
(`mongil-launch-guardrails-proposal.md`). Where this summary and a
source file disagree, the source file wins and the disagreement should
be flagged.*
