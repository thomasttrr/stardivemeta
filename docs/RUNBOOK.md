# StarDiveMeta — Runbook

How the site stays current (plan P5 / T5.1–T5.3). The split of labour
is the plan's guardrail G9:

- **X's workspace (scheduled jobs):** sweeps, codes, notes, digest,
  merges.
- **GitHub Actions:** CI (`ci.yml`), deploy (`deploy.yml`), health
  (`health.yml`).

Everything runs against **staging** until launch: `deploy.yml` sends
every merge on `main` to the `staging` Cloudflare Pages branch while
the repo variable `LAUNCHED` is not `"true"` (D-16). No job ever sets
`LAUNCHED`, touches the production branch, or deploys by hand.

## The jobs

All times America/New_York — the scheduler stores the IANA zone, so
DST is handled by the zone, never by a fixed UTC offset (A.0 rule 7).

| Job | When (ET) | What it does | Output |
|---|---|---|---|
| `stardivemeta-sweep-am` | Tue–Sun 09:00 | Sweep procedure, **am mode** (stamp always) + codes-scout + patch-watch | PR `data/<sweep-id>` (always) |
| `stardivemeta-sweep-audit` | Mon 09:00 | Sweep procedure, **audit mode**: full A **and** B read of every source even when A shows no change; replaces sweep-am on Mondays; includes codes-scout + stamp | PR `data/<sweep-id>` |
| `stardivemeta-sweep-pm` | daily 23:45 | Sweep procedure, **pm mode**: no stamp-only commits; also commits if a code's `expires_at` passed since the last build | PR only if something changed |
| `stardivemeta-notes-draft` | Mon 09:30 | `mongil-build note` from the last published note's snapshot to the latest snapshot | zero changes → `auto` note in a data PR; changes → PR `notes/<date>` labelled `needs-tom` |
| `stardivemeta-digest` | Mon 10:00 | One message to Tom (format below), only if there is something to approve or report | message, or silence |
| health workflow | daily 15:00 UTC (10:00 EST / 11:00 EDT) | The five checks in `scripts/health-check.sh` against the staging URL (repo variable `HEALTH_SITE_URL` after launch) | GitHub issue labelled `health` on failure; closes it on green |

The **merge step** belongs to whichever job opened the PR (see
"Sweep procedure", step 10).

## Politeness (every fetch, every job)

- At most **1 request per 5 s per host**.
- Fixed User-Agent `StarDiveMetaBot/1.0 (+https://stardivemeta.com/about/)`
  where the fetch tool allows setting one; if a host blocks the bot,
  read it through the browser as a normal visitor at the same rate.
- `robots.txt` honoured. If both routes fail, the source is
  *unavailable* — follow the harvest guide's playbook
  (`docs/harvest-guide.md`): no update for that source this sweep;
  `status = "unavailable"` after 2 consecutive failed sweeps; a
  same-day report to Tom.

## Sweep procedure (shared by sweep-am, sweep-pm, sweep-audit)

Sweep id: `YYYY-MM-DDTHHMM-sweep-am` / `-sweep-pm` using the job's
scheduled ET time (e.g. `2026-11-03T0900-sweep-am`). The audit uses
`-sweep-am` (it *is* Monday's am sweep, with full double reads).

1. **Start clean.** `git checkout main`, pull, confirm a clean tree.
   Create the branch `data/<sweep-id>` — but only once there is
   something to commit (step 8); reads happen on `main`'s checkout
   and are written under `data/harvests/<sweep-id>/`.
2. **Read A** of every voting + check-only source, exactly per
   `docs/harvest-guide.md` (rendered-text read; mongil.gg is the
   visual-read exception). Transcription only — never interpret,
   never fill a gap from another list.
3. **Patch-watch** (inside every sweep): check the official
   Netmarble channels for a game version newer than
   `data/game.toml`'s `current_version`. If one exists: update
   `game.toml`, re-evaluate source staleness per Method §5, run an
   extra full A+B sweep within 24 h, and send Tom a same-day report.
4. **Compare** each A read against `data/placements/`. Unchanged →
   nothing more for that source (am/pm modes).
5. **Read B** (screenshot read, independent of A) for every changed
   source — and for **every** source in audit mode. Run
   `mongil-build harvest-diff` on each A/B pair. Any blocking
   mismatch stops that source's update; settle it by a third read,
   never by editing a file to something neither read said.
6. **Merge** clean pairs into `data/placements/` from A's read
   exactly (the merge rule). Keep the A/B pair under
   `data/harvests/<sweep-id>/`.
7. **Codes-scout** (am + audit modes; pm checks only the expiry
   rule): observe every source in `data/code-sources.toml`, update
   observations in `data/codes.toml` (status as stated, the page's
   own updated date as `claim_date`, reward wording as stated).
   Double-read applies to codes too (B from screenshot when A's
   observations changed). In **pm mode**, commit only if a code's
   stated `expires_at` passed since the last build.
8. **Stamp + snapshot.** Always (am/audit): set
   `data/state/checks.json` — `sources_checked_at` (and
   `codes_checked_at` when codes were observed) to this sweep,
   `last_sweep` to the sweep id. If placements changed, run
   `mongil-build snapshot`. Commit on branch `data/<sweep-id>`.
   In pm mode, skip the branch entirely when nothing changed.
9. **Validate.** `cargo run -p mongil-build -- validate --data data
   --site site` must pass; `cargo test --workspace` must stay green.
   A red gate = no PR: draft a breakage report instead (below) and
   leave `main` untouched.
10. **PR + merge step.** Push the branch and open PR
    `data/<sweep-id>` (GitHub connector). When CI is green, run
    `mongil-build lane --base origin/main` on the branch:
    - `data-only` → merge the PR. `deploy.yml` ships it to staging.
    - `needs-tom` (or red CI, or `data/ops/autodeploy.toml` paused,
      or the tripwire tripped) → label the PR `needs-tom`, leave it
      for the digest, and do not merge.
11. **Report** only what the report rules below require. A clean
    no-change am sweep that merged its stamp PR is silent.

Push command (deploy key + egress proxy, per AGENTS.md):

```sh
GIT_SSH_COMMAND="ssh -i /home/hatch/.ssh/stardivemeta-deploy -o IdentitiesOnly=yes -o StrictHostKeyChecking=no -o ProxyCommand=nc\ -X\ connect\ -x\ 198.19.0.1:3128\ %h\ %p" git push origin <branch>
```

## Notes-draft rules (Mondays 09:30)

- `mongil-build note --from <last published note's snapshot> --to
  <latest snapshot> --date <today>`.
- Zero changes → the note has `status: auto` and publishes without
  approval (D-12) via a normal data PR (merge step as above).
- Any change → the note stays `status: draft` (D-13) on PR
  `notes/<date>`, labelled `needs-tom`. Never merge it; Tom approves
  by merging the PR himself.
- If last week's draft is still unapproved after 7 days, close its
  PR; the new note covers both weeks (guardrail G11).

## Report formats (T5.3 — the only messages Tom gets)

**Same-day report** (new confirmed code, patch, source drop,
breakage, health failure, major error). Max 5 lines:

```text
StarDiveMeta — <what happened>. Live site: <still correct / changed>. Action for you: <none / tap link>.
```

**Monday digest.** Max 10 lines: the movers-note PR link (if any),
the `needs-tom` PRs, any open `health` issue, and an estimate of the
Tom-minutes needed this week. **Skipped when empty.**

**Breakage report**:

```text
What broke: <source/page>. What's live: data from <date>, still correct as of that date. Nothing was guessed. Fix: <plan>.
```

Health relay: the digest — and any sweep that notices one — relays
an open `health`-labelled GitHub issue to Tom as a same-day report
with the project name. There are no messages on green.

## Health workflow (T5.2)

- `.github/workflows/health.yml`, cron `0 15 * * *` UTC (the plan
  accepts the 10:00/11:00 ET seasonal drift), plus manual dispatch.
- Checks live in `scripts/health-check.sh` so they can be run by
  hand: build.json 200; build commit == main HEAD (30-minute grace);
  `codes_checked_at` < 30 h; `sources_checked_at` < 30 h (the sweep
  heartbeat — a dead X workspace gets noticed); `/`, `/codes/`,
  `/c/narae/` 200 + disclaimer.
- Failure → open or comment on a GitHub issue titled
  `StarDiveMeta health: <check> failed`, labelled `health`.
  All-pass → close any open `health` issue.
- Acceptance test: dispatch the workflow manually with
  `codes_checked_at_override` set two days in the past → a
  `codes check` issue opens; dispatch again clean → it closes.

## Dry run (T5.4) — CLOSED Oct 31, 2026

Tom moved the launch up to Sat Oct 31, 2026, ending the dry run
early (it ran Oct 29–31 on staging). The end-of-week acceptance
numbers, the breakage drill, and the CP4 ending that were defined
here no longer apply, and nothing waits for a Nov 5-6 launch
window. The sweeps above continue unchanged as the steady-state
cadence. The sweep record is the `data/<sweep-id>` PRs plus
`checks.json` — routine sweeps do not touch `docs/BUILD-LOG.md`
(it is outside the lane allowlist).
