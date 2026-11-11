# Codes expiry check — sweep 2026-11-10T2345-sweep-pm (pm mode)

PM mode checks only the expiry rule (RUNBOOK step 7): no codes-scout,
no new observations. Checked 2026-11-10 23:45 ET (2026-11-11 04:50 UTC)
against the last build's stamps (2026-11-10T14:10:26Z, sweep
2026-11-10T0900-sweep-am).

## Expiry passed since the last build

Two codes carry a stated `expires_at` of `2026-11-10T23:30:00Z`,
which passed between the AM build and this sweep:

- `HAPPYHALF1014` — expires_at 2026-11-10T23:30:00Z (official)
- `MAKEANDDESTROY` — expires_at 2026-11-10T23:30:00Z (official)

No other code in `data/codes.toml` has a stated `expires_at`.

## Action

Statuses are computed by the emit engine (§B.3 rule 1: a stated
expiry before build time → `expired`) and are never stored, so
`data/codes.toml` itself is not edited: adding an `active`
observation at/after the expiry would fail gate G10, and no source
was re-read for codes in pm mode. This sweep's merge to main
triggers the rebuild in which both codes recompute to `expired`.
The codes page's client-side expiry script already moves both
cards out of Active in the meantime, so the live site does not
show them as active after 23:30 UTC.

`codes_checked_at` is stamped for this sweep on the strength of
this expiry check only.
