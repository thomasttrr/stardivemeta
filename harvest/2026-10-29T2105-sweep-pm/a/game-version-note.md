# T2.3 — Game version string (A-read, sweep 2026-10-29T2105-sweep-pm)

**Answer: the official version string is "1.4".** Pocket Gamer's "1.04"
and Kaiden's "1.4" are the same version written two ways. `data/game.toml`
was NOT edited by this read; the merge step should set
`current_version = "1.4"` and record the equivalence.

## Evidence

1. **Netmarble's own press release, dated September 30, 2026** (Los
   Angeles), reproduced by Capsule Computers and Inven Global:
   "Netmarble … has released the **Version 1.4** update for its
   monster-taming action RPG MONGIL: STAR DIVE" — Episode 7, new region,
   new character Isabella, Half Anniversary events.
   - https://www.capsulecomputers.com.au/2026/10/mongil-star-dive-launches-version-1-4-update-featuring-new-character-isabella/
   - https://www.invenglobal.com/articles/26667/mongil-star-dive-version-14-update-adds-isabella-and-new-region

2. **Store build numbers are the zero-padded form of the same version.**
   The App Store build for the 1.4 release is version **1.04.00** (store
   update dated September 29, 2026, per AppBrain's App Store record), and
   the Android build reached **1.04.10** on October 28, 2026 (APKFab).
   The 1.04.10 store changelog itself reads: "**1.4 Version Major
   Update** … Character Balance Adjustments".
   - https://www.appbrain.com/appstore/mongil-star-dive/ios-6483252725
   - https://apkfab.com/mongil-star-dive/com.netmarble.monster2

3. **Pocket Gamer's "1.04.00"** (tier-list page header, updated October 5,
   2026) is the App Store build string of the 1.4 release — which is why
   PG writes "1.04" where Netmarble's announcement writes "1.4".

## Release facts for the `game.toml` release entry (for the merge step)

- Version **1.4**, released **2026-09-30** (Netmarble press release date;
  store builds rolled out 2026-09-29).
- `balance_changes = true` — the official store changelog for the 1.4
  builds lists "Character Balance Adjustments".
- Roster change: Isabella (5★ Ice) added; Chloe followed in the October
  20, 2026 update **within** 1.4 (Netmarble's October 21 press coverage
  announces no new version number for it).
- Current build as of this sweep: 1.04.10 (October 28, 2026) — still
  content version 1.4.
