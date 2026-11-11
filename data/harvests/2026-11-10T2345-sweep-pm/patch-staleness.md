# Patch staleness re-evaluation — v1.5 (sweep 2026-11-10T2345-sweep-pm)

Patch-watch (see patch-watch.md in this harvest) found Version 1.5
released 2026-11-10 (store build 1.05.00, new character M.A.D.).
RUNBOOK step 3 / Method §5 re-evaluation:

## Balance changes?

The released 1.05.00 changelog (App Store What's New, read
2026-11-11 04:43 UTC) lists: new character M.A.D., main story
episode "Light From a Fallen Star", event story "A Flame Chasing
Freedom", Honor Ranking / Special Conquest / Archive, Friends
Feature Improved. It lists **no character balance adjustments** —
unlike the 1.4 changelog, which explicitly listed "Character
Balance Adjustments" and was recorded with
`balance_changes = true`. No official patch-notes text announcing
balance changes for 1.5 was found on any channel checked.

Recorded for the 1.5 release entry: `balance_changes = false`,
on that evidence. If official 1.5 patch notes later show balance
changes, this re-evaluation must be re-run.

## Source staleness (Method §5)

The stale_check_only rule applies to a **balance patch**: a source
whose list predates the patch and hasn't updated stops counting.
With no balance changes in 1.5, no source becomes stale:

- pg — list_date 2026-11-07, states version 1.04.10: stays
  `current`. It has not ranked M.A.D. yet; that is absence of a
  placement, not staleness.
- kaiden — index "Last updated: October 19, 2026", no M.A.D. page
  yet: stays `current`, same reasoning.
- gamewith-jp — list_date 2026-10-21: stays `current`, same
  reasoning.
- Check-only sources unchanged in role.

No `data/sources.toml` edit is made by this sweep. Voting coverage
stays at 3. All seven A reads in this sweep were taken after the
release and are identical to the pre-release lists, because none
of the lists has republished yet.

## Follow-ups triggered

- `data/game.toml` update to 1.5 is proposed on a separate
  needs-tom PR (game.toml is outside the data-only lane
  allowlist): current_version 1.5, release entry dated 2026-11-10,
  balance_changes = false, roster_change = ["m-a-d"], source =
  the App Store listing.
- Extra full A+B sweep within 24 h (by 2026-11-11 23:45 ET).
- Roster/banner facts for M.A.D. already exist pre-staged on
  branch `kit/v15-prestage` (unmerged, Tom's call); this sweep
  does not merge it.
- Same-day patch report sent with this sweep's result.
