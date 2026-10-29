# MONGIL: STAR DIVE tier-list site — V1 Scope (LOCKED, 2026-10-29)
> **Status note (Tom, 2026-10-29 ~19:09 ET):** decision record, NOT a permanent lock. Input to the Opus 5.5 planning pass (see `stardivemeta/00-MASTER-HANDOFF-for-Opus.md`); may be revised there with reasons stated.

Locked by consensus: Tom, Kit, Grok 4.7, and Jev (advisory) all converged on
Package B. Architecture and consensus math (Method v1.0) were locked earlier
the same day. STACK AMENDED 2026-10-29 ~18:50 ET (Tom's tap, his future-
proofing / 3D / animated-character insurance requirement): Stack v2 —
Rust core (data, method, validation, JSON output) + Astro static shell +
React/TypeScript islands; Tailwind + design tokens; GSAP for animation;
Three.js (WebGPU/WebGL) reserved for a future 3D route; Rive/Lottie/
Live2D available for animated characters (art sourcing + licensing gates
apply). Full text: mongil-stack-v2-proposal.md. This supersedes the
Askama + Leptos-island front end; all other locks stand.

## Name — LOCKED 2026-10-29 (Tom's pick)
**StarDiveMeta** — stardivemeta.com.
Registry checks (2026-10-29, Verisign RDAP + rdap.org): stardivemeta.com and
stardivemeta.gg both returned "not registered" on repeat checks — they look
available, but only registering proves it. Domain not yet bought (buying is
Tom's call / a spending step).

## Hosting — LOCKED 2026-10-29 (Tom's pick)
**Cloudflare Pages.** Deciding factor: unlimited free bandwidth with no
pause-at-cap risk (Vercel Hobby pauses the site at 100 GB / 1M requests,
which would bite exactly during a traffic spike on a non-commercial launch),
and no commercial-use restriction if monetization ever opens. Automation is
identical on either host — it lives in GitHub Actions + Kit's workers; the
host only serves the built static folder. Buying stardivemeta.com via
Cloudflare Registrar (at-cost) keeps domain + DNS + hosting in one place.

## IN v1
- Tier list: Overall + Conquest + Rift views, Method v1.0 math, evidence view,
  Solid / Contested / Speculation labels, history snapshots.
- Investment toggle (Low / High / Max) — DEFAULT LOCKED 2026-10-29 (Tom's
  tap): Low is the first-visit default. The island remembers a returning
  visitor's last choice locally and restores it. Copy line under the toggle:
  "Low = what a typical account has. Watch who climbs as investment rises."
  (Rationale: most players are low-investment; S is honestly sparse at Low —
  4 vs 7 at High/Max — and the climb is the site's argument in one tap.)
- Beginner / reroll view.
- Published rules page (conversion assumption + fixed-vote rule printed).
- Codes page — with conditions: only two-source-confirmed codes list as
  Active; a clearly marked Unconfirmed section for the rest; a checked-today
  stamp. (Verified 2026-10-29: 8+ codes reported active, heavy search demand,
  but statuses conflict between sites — the two-source rule is the safeguard.)
- Banner evidence page — a new character's first placements + coverage,
  updated as sources weigh in. NO pull verdict, no recommendation.
- Table view of the tier list (same computed data, accessibility).
- Official list PNG (generated at build time, resvg).
- Weekly "what moved" note.

## OUT of v1 (later)
- Pull advice / pull-value verdicts — out because a new banner character
  launches at coverage 1 (Speculation), so verdicts would be weakest exactly
  when demand peaks, and the method has not yet proven itself in public.
  Revisit for v1.1 once a character's coverage matures.
- Filters (mechanic / element tags) — tag data not yet harvested or verified;
  unverified tags would hurt an evidence brand.
- User PNG export — arrives with the tier-list maker (WASM canvas).
- My Roster mode, tier-list maker, per-character build pages (as planned).

## Rules that still bind v1
- Netmarble fan content: non-commercial at launch, screenshots/crops only,
  no ripped art, no leaked/unreleased content, "not an official site" note.
