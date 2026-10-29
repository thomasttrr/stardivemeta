# StarDiveMeta — Launch Guardrails (LOCKED, 2026-10-29)
> **Status note (Tom, 2026-10-29 ~19:09 ET):** decision record, NOT a permanent lock. Input to the Opus 5.5 planning pass (see `stardivemeta/00-MASTER-HANDOFF-for-Opus.md`); may be revised there with reasons stated.

Status: LOCKED with Kit's amendments (Kit drafted; Jev consulted — lukewarm
on the original tripwire, firm 0.80 on the passive-run default; Grok picked
the amended lock; Tom relayed it). Review point: after the first two patch
cycles (roughly 6–8 weeks live). MONGIL is a small community — targets are
sized for that, not for WuWa-scale traffic.

## 1. Accuracy guardrails (hard — these outrank growth)
- Zero uncorrected errors: any wrong placement, label, or code status is
  fixed the same day it is found, and the fix is noted on the page/snapshot.
- Severity-split tripwire (AMENDED): errors caught before publishing (the
  double-read doing its job) never count. ONE major published error (e.g. a
  wrong tier placement presented as fact, a code listed Active that never
  worked) triggers an immediate review. THREE published errors of any kind
  in the first month → data auto-deploy pauses and updates go back to
  Tom-approval until the cause is fixed.
- No rule exceptions for speed: two-source rule (data + codes) and the
  no-guessing harvest rule apply even on banner day.

## 2. Is it working? (month-1 signals, Cloudflare Web Analytics)
- Indexing footnote (AMENDED): a new domain can take weeks to index, so
  month-1 traffic is EARLY SIGNAL ONLY. The real judgment happens at the
  two-patch-cycle review.
- Working: 1,000+ visits in month 1, with the codes page and the weekly
  movers note visibly drawing return visits.
- Slow but alive: 300–1,000 visits → distribution is the problem, not the
  product: push the PNG card + movers note into the game's Discord and
  community spaces (within their self-promo rules), check search indexing.
- Flat: under 300 visits despite distribution → passive running is the
  DEFAULT at the review (near-zero cost; AMENDED). The review can also cut
  cadence. Shutdown happens only if Tom actively decides he no longer
  wants the site — never as an automatic consequence of low traffic.

## 3. Source health guardrail
- The site needs its 3 voting sources. Dropping to 2 for more than one
  patch cycle triggers the mongil.gg image harvest as top priority and a
  replacement-source proposal to Tom (Method v1.1 decision).

## 4. Cost + time guardrails
- Running cost target: $0/month hosting (Cloudflare free). Only fixed cost
  is the domain registration, once a year.
- Tom's time budget: ~30 minutes a week (movers-note approval + escalations).
  If approvals regularly exceed that, Kit automates more or trims cadence —
  the site must not become a second job.
- Kit's cadence cost stays inside the existing weekly sweep + daily codes
  scout; no new standing jobs beyond those without Tom's say-so.

## 5. Content guardrails (already locked, restated so they are not lost)
- No pull verdicts in v1. Banner pages show evidence only.
- Non-commercial at launch, screenshots/crops only, no ripped art, no
  leaked/unreleased content, "not an official site" disclaimer visible.
- Method changes only via a version bump with Tom's approval.
