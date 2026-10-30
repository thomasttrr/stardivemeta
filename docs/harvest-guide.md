# Harvest Guide — how each source is read

This is the rulebook for a sweep: how to turn each tier-list source
into a harvest file, how the two independent reads are compared, and
what happens when a source can't be read. It expands plan §B.4; the
plan's Method v1.0 decides what the harvested data *means* — this
guide only decides how it is *copied*.

Rules of the road, for every source:

- **Copy, never interpret.** If a list doesn't place a character, the
  harvest has no placement for them. Absence is data; never guess a
  tier from a neighbouring list, a portrait's position, or memory.
- **Record the page's own dates.** Each source's `list_date` in
  `data/sources.toml` is the date the *list* states, and staleness is
  judged against balance patches (Method §5), never against the
  harvest date.
- **Politeness.** At most 1 request per 5 s per host, a fixed
  User-Agent `StarDiveMetaBot/1.0 (+https://stardivemeta.com/about/)`,
  and `robots.txt` honoured. If a host blocks the bot, read it through
  the browser as a normal visitor at the same rate. If both fail, the
  source is *unavailable* (playbook below).

## The double read (independence rule)

Every sweep reads each source **twice, independently**:

- **Worker A** reads the page as rendered text (browser
  `get_page_text`) and writes `data/harvests/<sweep-id>/<source>.a.toml`.
- **Worker B** reads the same section from a **screenshot** (visual
  read) and writes `data/harvests/<sweep-id>/<source>.b.toml`, with no
  access to A's file.

The two reads take different paths, so they rarely make the same
mistake. A harvest file is the placements schema of §B.3 plus a
`[meta]` block: `source`, `harvest` (the sweep id), `worker` (`"A"` or
`"B"`), `read_mode` (`"text"` or `"screenshot"`), `read_at`.

After both reads, run:

```sh
mongil-build harvest-diff data/harvests/<sweep-id>/<source>.a.toml \
                          data/harvests/<sweep-id>/<source>.b.toml
```

- Structured fields are **blocking**: character set, `conquest`,
  `rift`, `tier`, `low/high/max.tier`, `awakening`, `flags`, `status`,
  `page_updated`. Any difference prints a `MISMATCH` (or `MISSING` for
  a character one read has and the other hasn't) and the command
  exits 1 — the update is blocked.
- `raw` and `note` are free text: they are compared after whitespace
  and punctuation are normalized, and a difference prints a `WARN`.
  Warnings never block.
- Meta fields that differ by design (worker, read mode, read time)
  are not compared.

Example output:

```text
harvest-diff kaiden 2026-11-03-am
MISMATCH mina.high.tier        A="T0.5"  B="T1"
MISMATCH jiwon.low.flags       A=["P"]   B=[]
MISSING  chloe                 A=present B=absent
WARN     narae.low.raw         A="T0"    B="T0 "   (normalised equal)
3 blocking, 1 warning → update blocked
```

**Merge rule.** A source's placements file (`data/placements/<file>`)
may only be updated from a clean diff (0 blocking). The merged file's
structured fields must equal worker A's read exactly — B's role is to
catch A's mistakes, not to be averaged in. If the two reads disagree,
a human (or X, by re-reading the page a third way) settles which read
is right; the wrong file is corrected and the diff re-run. Never edit
a merged file to something neither read said. Gate G6 re-runs this
check in `validate` and CI.

## Per-source extraction rules

### Pocket Gamer (`pg`, voting, per-mode)

- **Where:** the tier-list page in `data/sources.toml`, the
  **Conquest** and **Dimensional Rift** sections only.
- **Read:** each row's character names per tier (S / A / B / C).
- **Record:** `conquest` and `rift` per character, the page date, and
  the game version the page states (into `data/sources.toml`).
- **Ignore the World/Story list entirely** — it doesn't label the
  Summer variants and is not part of Method v1.0.
- A character in one mode list but not the other gets only that
  mode's field. A character in neither gets a placement entry only if
  there's something to say (`status`/`note`); otherwise no entry.

### Kaiden (`kaiden`, voting, overall by investment)

- **Where:** one guide page per character,
  `kaiden.gg/mongil/characters/<slug>/`, the "Tier Position" section.
- **Read:** the Low / High / Max lines **verbatim** into each cell's
  `raw`.
- **Record per level:** `tier` = the **first-listed** tier on the line
  (the method's `kaiden_conditional_rule`); `awakening` from an
  `(A#)` marker on that level's part of the line; `flags` from `(P)`
  (provisional) and `(E)` (exceptional) markers.
- **Record per character:** `page_url`, and `page_updated` = the
  page's own "updated" date if the page shows one; if it doesn't,
  omit `page_updated` (the site then says "date not shown").
- A character whose page says a review is coming (Chloe at the
  2026-10-29 harvest) gets `status = "no_placement"` and a `note`
  quoting the page's wording — never an invented tier.

### GameWith JP (`gamewith-jp`, voting, overall)

- **Where:** `gamewith.jp/stardive/552850`, the SS / S / A **text**
  sections.
- **Read:** character names per tier.
- **Record:** `tier` and the line as printed in `raw`; the page date
  and the criteria caveat (limited 5★ at A0, permanent 5★ up to A2,
  4★ at A6) live in `data/sources.toml`.
- **A character absent from the text tiers gets NO vote** — never a
  zero, never an inferred tier. The visual tier table on the same
  page is read only under T2.5's separate double visual read, and
  only enters the data if Tom approves the method extension at CP2.

### urgametips (check-only, Conquest)

- **Where:** its Conquest list. **Read:** rows per tier.
  **Record:** `conquest` only, in
  `data/placements/urgametips.toml` with `[meta] kind = "modes_conquest_only"`.
- Never counted: it is a near-copy of Pocket Gamer's Conquest list.
  Shown in evidence for context only.

### gacha.gg (check-only, overall)

- **Where:** its tier list. **Read:** rows per tier (S–D).
  **Record:** `tier` per character.
- Never counted: it is a median of other lists including Kaiden and
  mongil.gg — counting it would count those twice.

### GameWith EN (check-only, overall)

- **Where:** `gamewith.net/mongil-stardive/73714`, SS / S / A
  sections. **Read:** names per tier. **Record:** `tier`.
- Never counted: a launch-era list, kept as a historical reference.

### mongil.gg (check-only, per-mode — T2.6, conditional)

- **Where:** `mongil.gg/tierlist` **images** (World / Conquest /
  Dimensional Rift). Read by eye only — the placements are not in
  the page text.
- **Read:** both workers read the images independently (the B read is
  the second visual read here; there is no text read). If the images
  carry name text, read the names. If they show portraits only, match
  each portrait against Kaiden's character-page portraits by eye —
  both workers must agree on **every** identity.
- **Record:** `world`, `conquest`, `rift` in
  `data/placements/mongil-gg.toml`. Shown in evidence under "Not
  counted" with the note "Read from images — not counted until a
  method update".
- Tom spot-checks 5 random placements against cropped screenshots.
  Any unresolved A/B identity mismatch, or any "wrong" from Tom,
  abandons the read: mongil.gg stays unread and nothing is shown.

## Codes harvest (summary — full rules in plan T2.7)

Codes are harvested fresh each day into `data/codes.toml` with one
observation per code per source (`source`, `status`, `seen_at`, URL,
and the reward text **as that source states it**). Sources are
registered in `data/code-sources.toml` with an `independence_group`;
two sites share a group when their code lists **and** reward wording
match exactly, and the evidence for grouping is recorded. A code is
only ever shown as Active on two **independent** groups' say-so.
Double-read applies to codes too: A from text, B from screenshot.

## Unavailable / stale / breakage playbook

- **A page 404s, is paywalled, or its format changed so the guide's
  fields can't be found** → that source's file is not updated in this
  sweep. After **2 consecutive failed sweeps (24 h)** its `status`
  becomes `unavailable`; from then on its votes are excluded and
  coverage is recalculated (the status is data — Rust reads it, so
  this is a data change, not a method change). A same-day report goes
  to Tom.
- **Balance patch** → any source whose `list_date` is before the
  patch date and that hasn't updated becomes `stale_check_only`. A
  same-day report goes to Tom.
- **Voting sources drop below 3** → the mongil.gg read (T2.6) starts
  straight away. If voting sources are still below 3 after one patch
  cycle, a replacement-source proposal (Method v1.1) goes to Tom.
- **A read looks wrong in bulk** (a whole tier missing, half the
  roster absent) → treat it as a format change, not as data: stop,
  do not merge, and report the same day. Gate G4 will refuse an
  unexplained coverage drop anyway.
