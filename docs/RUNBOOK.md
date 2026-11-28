# StarDiveMeta — Runbook

How the site stays current. Corrected 2026-11-27 to match the
2026-11-24 direction change; the previous version of this runbook
described the retired consensus-sweep system as if it were still
running.

## What the site is now

Since 2026-11-24 the site is a clean editorial tier-list guide in our
own words: one board, plain tiers, and a short "why" for every
character (the `why` field in `data/roster.toml`). There are no source
names, no Solid / Contested / Speculation labels, no investment
toggle, and no method, notes, history, mode or table pages on the
public site. The consensus engine and the source data remain in the
repo underneath, unseen.

- **Tier changes are editorial — Tom's call.** They are never
  sweep-driven. A change is made by editing the roster data and
  merging to `main` like any other change.
- **Codes and banner pages** are kept, with source attributions
  stripped.

## The jobs

| Job | When (ET) | Status |
|---|---|---|
| `stardivemeta-sweep-pm` | daily 23:45 | **Rewritten (2026-11-24)** as a lean codes + patch watch: codes ship when confirmed; patch / version news is reported to Tom. Tiers are never touched. |
| `stardivemeta-sweep-am` | — | **Disabled 2026-11-24.** |
| `stardivemeta-sweep-audit` | — | **Disabled 2026-11-24.** |
| `stardivemeta-notes-draft` | — | **Disabled 2026-11-24.** |
| `stardivemeta-digest` | — | **Disabled 2026-11-24.** |
| health workflow | daily 15:00 UTC | GitHub Actions (`.github/workflows/health.yml`), checks in `scripts/health-check.sh`, run against the staging URL. Failure opens (or comments on) a GitHub issue labelled `health`; all-pass closes it. |

The old sweep procedure, the double-read harvest rules and the
notes-draft rules are retired. `docs/harvest-guide.md` is kept as the
historical record of how the consensus-era sweeps worked.

## Deploy and launch state

- Everything runs against **staging**: `deploy.yml` sends every merge
  on `main` to the `staging` Cloudflare Pages branch while the repo
  variable `LAUNCHED` is not `"true"`.
- Production is gated (`LAUNCHED=false`). **Tom launches manually** —
  no job sets `LAUNCHED` or touches the production branch.
- The domain `stardivemeta.com` is **not registered**. Until Tom buys
  it, the site lives on its Cloudflare Pages staging address, and no
  document should present stardivemeta.com as a live address.

## Report format

Same-day reports to Tom stay short (max 5 lines):

```text
StarDiveMeta — <what happened>. Live site: <still correct / changed>. Action for you: <none / tap link>.
```

Health failures are relayed to Tom the same day, with the project
name. There are no messages on green.
