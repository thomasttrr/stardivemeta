# Merge summary — sweep 2026-10-29T2105-sweep-pm (T2.2 merge, for CP2)

Launch-shape results from the emitted `tierlist.json` after the merge.
Game version: **1.4** · Snapshot: `2026-10-29T2105-sweep-pm` (inputs_sha256 eb6616c67701…)

## Band counts and label counts per level

| Level | S | A | B | C | D | Mode-split | Unranked | Solid | Contested | Mode-split label | Speculation |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Low | 4 | 6 | 11 | 6 | 1 | 1 | 0 | 16 | 12 | 1 | 0 |
| High | 7 | 6 | 8 | 6 | 1 | 1 | 0 | 24 | 4 | 1 | 0 |
| Max | 7 | 7 | 8 | 5 | 1 | 1 | 0 | 24 | 4 | 1 | 0 |

## S tier by level

- **Low S (4):** Narae, Chloe, Isabella, Brisshell
- **High S (7):** Narae, Chloe, Mina, Brisshell, Francis: Summer Dive!, Isabella, Esther
- **Max S (7):** Narae, Chloe, Francis: Summer Dive!, Mina, Brisshell, Isabella, Esther

## The Low board in full

### S (4)
- Narae — SOLID, score 4.0, 3 list(s)
- Chloe — SOLID, score 4.0, 2 list(s)
- Isabella — SOLID, score 3.5, 3 list(s)
- Brisshell — SOLID, score 3.5, 2 list(s)

### A (6)
- Mina — CONTESTED, score 3.3, 3 list(s)
- Verna: Summer Dive! — SOLID, score 3.3, 2 list(s)
- Esther — CONTESTED, score 3.2, 3 list(s)
- Francis — CONTESTED, score 3.2, 3 list(s)
- Ophelia — CONTESTED, score 3.0, 3 list(s)
- Francis: Summer Dive! — CONTESTED, score 3.0, 2 list(s)

### B (11)
- Benjamin — SOLID, score 2.8, 3 list(s)
- Jiwon — CONTESTED, score 2.5, 3 list(s)
- Yeonhwa — CONTESTED, score 2.5, 3 list(s)
- Mabel — SOLID, score 2.5, 2 list(s)
- Vivian — SOLID, score 2.5, 2 list(s)
- Cloud — SOLID, score 2.3, 3 list(s)
- Ellie — SOLID, score 2.3, 2 list(s)
- Flare — CONTESTED, score 2.2, 3 list(s)
- Gabi — CONTESTED, score 2.0, 3 list(s)
- Penny — CONTESTED, score 2.0, 3 list(s)
- Nagi — CONTESTED, score 2.0, 2 list(s)

### C (6)
- Reina — SOLID, score 1.8, 3 list(s)
- Sangun — CONTESTED, score 1.8, 2 list(s)
- Sera — SOLID, score 1.5, 3 list(s)
- Bonnie — SOLID, score 1.0, 2 list(s)
- Leeho — SOLID, score 1.0, 2 list(s)
- Verna — SOLID, score 1.0, 2 list(s)

### D (1)
- Daisy — SOLID, score 0.5, 2 list(s)

### Mode-split (1)
- Angel — MODE_SPLIT, score 2.2, 3 list(s)

## Codes — computed status (§B.3 engine, emit)

Active **6** · Unconfirmed **8** · Expired **11** (of 25).

Active: HAPPYHALF1014, REGINULAUPDATE, LETSPLAYEP6, MEMOLMONGIL, GSBGMONGIL, JOINMONGIL.

**Divergence from the harvest's human classification (7 active / 12 unconfirmed / 6 expired) — read before CP2:**
- LOVEMONGIL computes **unconfirmed**: GamingOnPhone's recorded expired claim blocks §B.3 rule 2, even though the harvest judged the stale (Aug 3) dissent outweighed.
- MONGILREPORT, MONGILPRESENT, DINDINISDINDIN, DEVLIVE0410, MONGILDISCORD compute **expired**: §B.3 rule 1 fires because two independent groups (SuperCheats + GamingOnPhone) say expired, even though fresher groups say active and the harvest called them conflicted/unconfirmed.
Both follow mechanically from §B.3 as written, on faithfully recorded observations. Whether stale witnesses should carry this weight is a Method question for Tom at CP2 — the data does not need to change either way.

## Vs the inherited (T1.3) dataset

- Baseline snapshot `2026-10-29-x1` (taken from the pre-merge data in /tmp; game_version "1.04") → this sweep's snapshot (game_version "1.4").
- **Zero tier, label or coverage changes at any level** — the fresh double-read harvest reproduces the inherited dataset's consensus exactly.
- Placement-input changes are additive only: Kaiden `page_url` on all 29 entries; Kaiden `awakening` filled where the research transcription lacked it (Brisshell High A1; Max A6 for Brisshell, Flare, Francis, Gabi, Isabella, Jiwon, Mina, Narae, Sangun, Verna: Summer Dive!, Yeonhwa); the new mongil.gg check-only file (21 characters × World/Conquest/Rift, never counted).
- Outside the snapshot's frozen inputs: game version "1.04" → "1.4" (T2.3) with the 1.4 release entry; codes went from empty to 25 codes with witness observations (T2.7); released_on added for 25 characters and 11 debut banners recorded (T2.8).
- NOT merged: the T2.5 GameWith JP visual cells (Sangun B, Bonnie B, Ellie B, Verna C, Leeho C, Daisy D) — Tom's CP2 decision. Rarity/pool fields stay omitted (T2.8 verdict PARTIAL).
