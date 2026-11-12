# Codes observations — sweep 2026-11-12T0900-sweep-am (codes-scout)

Read 2026-11-12 ~09:00 ET (crawled 2026-11-12 13:57 UTC) via browser.open, one read per source in
data/code-sources.toml. Compared against data/codes.toml
(last merged sweep 2026-11-10T0900-sweep-am) and yesterday's
sweep 2026-11-11T1200-sweep-am codes-observations.md.

## Verdict

- **NEW codes not in codes.toml: NONE.** No source lists any code
  string that is not already in codes.toml. MAKEANDDESTROY is still
  listed by NO second source this sweep — it stays single-source
  (official) / Unconfirmed. No source mentions a version 1.5
  launch code: the only 1.5 mention found is GamsGo's events section
  noting the November 5 Developer Lounge Talk: Version 1.5 Preview,
  which states "No new coupon or viewing reward is guaranteed" and
  names no code string — identical to yesterday.
- **Status changes: NONE in source listings.** Every readable
  source's active/expired split matches its recorded observations
  in codes.toml exactly, and is identical to yesterday.
  Post-expiry note (unchanged from yesterday): HAPPYHALF1014 and
  MAKEANDDESTROY both expired 2026-11-10T23:30Z, which is now past,
  but NO source has yet moved either code to an expired list.
  Per-source flip status: pocket-gamer — neither code listed at all
  (no flip); ldshop — neither listed (no flip); dexerto — neither
  listed (no flip); gamewith-en — neither listed (no flip); kaiden —
  neither listed (no flip); supercheats — neither listed (no
  flip); gamsgo — HAPPYHALF1014 still ACTIVE ("Available during
  the redemption period", expires November 10, 2026 23:30 UTC —
  NOT flipped), MAKEANDDESTROY not listed; cosm-game —
  HAPPYHALF1014 still in the active table as its newest row
  (released Oct 14, 2026, expires Nov 10, 2026 — NOT flipped),
  MAKEANDDESTROY not listed; gamesradar — neither listed (no
  flip); pocket-tactics — neither listed (no flip);
  gamingonphone — neither listed (no flip); official —
  unreadable (no flip readable). Per the sweep instructions,
  statuses recompute at emit from expires_at in codes.toml — a
  source still listing HAPPYHALF1014 active is NOT a new code
  and NOT a status change to merge.
- **claim_date changes: ONE — gamewith-en only.**
  gamewith-en crawl metadata today shows
  "Updated: 2026-11-11T22:47:11-05:00" — a new stamp since
  yesterday (2026-11-10T20:53:18-05:00), so its claim_date moves
  2026-11-10 → 2026-11-11. The code list content is identical
  (same 12 codes, same rewards, FINDSTAR10 entry still states
  "Until the end of 07/May/2026").
  Every other source's page-own stated updated date is identical
  to the claim_date already recorded in codes.toml and to yesterday:
  official unreadable (last recorded 2026-10-14 / 2026-11-05),
  pocket-gamer 2026-11-08, ldshop 2026-09-02, dexerto 2026-09-20,
  kaiden 2026-10-19, supercheats 2026-10-13, gamsgo 2026-11-02,
  cosm-game 2026-10-27, gamesradar none, pocket-tactics none,
  gamingonphone 2026-08-03.
  Metadata notes (neither changes the stated claim):
  (1) dexerto crawl metadata again shows
  "Updated: 2026-11-09T16:51:40.000Z" while its body claim — the
  claim codes.toml records — still reads "Updated September 20,
  2026", unchanged; (2) ldshop crawl metadata shows Published
  2026-09-02T15:41:53.000Z / Updated 2026-09-02T23:41:54.000Z,
  page prints no human-readable updated date, list identical.
- **Unreadable source: official.** browser.open on
  https://forum.netmarble.com/ returned only the game-selector
  index with no extractable code content this sweep — same
  failure pattern as the 2026-11-04/05/06/07/09/10/11 sweeps.
  No new official code was found via any other source read today,
  and the last recorded official observations
  (HAPPYHALF1014 claim_date 2026-10-14; MAKEANDDESTROY
  claim_date 2026-11-05) stand unchanged. Kaiden fallback check:
  hub Latest News still leads with the Nov 3, 2026 item ("Mongil
  Bonus Time and Break the Ice Run to November 10") and mentions
  no new official code.
- codes.toml updates needed from this scout: claim_date refresh
  for gamewith-en observations only (2026-11-10 → 2026-11-11) —
  no new code, no source status change. (Parent sweep still
  re-stamps checks.json per RUNBOOK. This scout did not edit
  codes.toml.)

## Per-source observations (claim_date = page's own stated updated date)

### official — https://forum.netmarble.com/ — claim_date: UNREADABLE this sweep (last recorded 2026-10-14 / 2026-11-05)
browser.open returned only the game-selector index — no
code list could be read. Last recorded official evidence stands:
HAPPYHALF1014 (Oct 14 Developer Lounge Talk; valid until Nov. 10,
2026, 23:30 UTC — now past) and MAKEANDDESTROY (Nov 5 Dev Lounge
Talk stream, expires Nov 10, 2026 23:30 UTC — now past). No new
official code found via any other source read today.

### pocket-gamer — claim_date: 2026-11-08 (unchanged)
"Updated on November 8th, 2026 - checked for codes"
Active: REGINULAUPDATE — 10 Premium Salmon Puree, 1 Purple Upgrade Stone, 3 Compass of Promise; LETSPLAYEP6 — 10 Premium Salmon Puree, 1 Purple Upgrade Stone, 3 Compass of Promise; MEMOLMONGIL — Green Cappy Papa, 25 Keys, 10,000 Gold; LOVEMONGIL — 1 Overflowing Love; MONGILPRESENT — 1 Brown Cappy, 1 Spark Slime, 1 Forkmugger; MONGILDISCORD — 30 Starlight Crystal, 3 Glimmering Stardust, 300k Gold; DINDINISDINDIN — 1 Green Cappy, 10k Gold; DEVLIVE0410 — 1 Spark Slime, 1 Scar, 1 White Wolf Warrior; MONGILREPORT — 1 White Wolf Warrior.
Expired: SPECIALGIFT0430 — 1 Blue Upgrade Stone; FINDSTAR10 — 10 Compass of Promise. No HAPPYHALF1014, no MAKEANDDESTROY, no new code. Matches codes.toml.

### ldshop — claim_date: 2026-09-02 (unchanged as stated; see metadata note in Verdict)
Active (same 9, same order as Pocket Gamer, pg-syndicated group):
REGINULAUPDATE ("new!") — 10 Premium Salmon Puree, 1 Purple Upgrade Stone, 3 Compass of Promise; LETSPLAYEP6 — 10 Premium Salmon Puree, 1 Purple Upgrade Stone, 3 Compass of Promise; MEMOLMONGIL — Green Cappy Papa, 25 Keys, 10,000 Gold; LOVEMONGIL — 1 Overflowing Love; MONGILPRESENT — 1 Brown Cappy, 1 Spark Slime, 1 Forkmugger; MONGILDISCORD — 30 Starlight Crystal, 3 Glimmering Stardust, 300k Gold; DINDINISDINDIN — 1 Green Cappy, 10k Gold; DEVLIVE0410 — 1 Spark Slime, 1 Scar, 1 White Wolf Warrior; MONGILREPORT — 1 White Wolf Warrior. No expired section. Crawl metadata: Published 2026-09-02T15:41:53.000Z, Updated 2026-09-02T23:41:54.000Z. Matches codes.toml.

### dexerto — claim_date: 2026-09-20 (unchanged)
"Updated September 20, 2026 … No new codes found."
Active (9): REGINULAUPDATE — 10 Premium Salmon Puree, 1 Purple Upgrade Stone, 3 Compass of Promise; LETSPLAYEP6 — 10 Premium Salmon Puree, 3 Compass of Promise, 1 Purple Upgrade Stone; MEMOLMONGIL — x25 Keys, 10,000 Gold; MONGILDISCORD — 30 Free Diamond, 30 Glimmering Stardust, 300,000 Gold; LOVEMONGIL — Overflowing Love Artifact; DEVLIVE0410 — Scar Monsterling, White Wolf Warrior Monsterling, Spark Slime Monsterling; MONGILPRESENT — Brown Cappy Monsterling, Spark Slime Monsterling, Forkmugger Monsterling; DINDINISDINDIN — Green Cappy Monsterling, 10,000 Gold; MONGILREPORT — White Wolf Warrior Monsterling.
Expired: LETS, PLAY. Matches codes.toml. (Crawl metadata "Updated: 2026-11-09T16:51:40.000Z" — body claim unchanged; see Verdict note.)

### gamewith-en — claim_date: 2026-11-11 (CHANGED from 2026-11-10; metadata 2026-11-11T22:47:11-05:00, new stamp since yesterday)
Content unchanged. Available Codes list: FINDSTAR10 (entry states "Until the end of 07/May/2026" — recorded expired in codes.toml) — Compass of Promise x10; JOINMONGIL — Leafymander Monsterling x1; PLAYMONGIL — Ring Slime Monsterling x1; PLAY — Swellymander Monsterling x1; LETS — Goblin Recruit Monsterling x1; GSBGMONGIL — Uncle Cappy Monsterling x1; MONGILREPORT — White Wolf Warrior Monsterling x1; DEVLIVE0410 — White Wolf Warrior Monsterling x1, Scar Monsterling x1, Spark Slime Monsterling x1; DINDINISDINDIN — Green Cappy Monsterling x1; MONGILPRESENT — Brown Cappy Monsterling x1, Spark Slime Monsterling x1, Forkmugger Monsterling x1; LOVEMONGIL — 4★ Overflowing Love Artifact x1; MONGILDISCORD — Starlight Crystal x30, Glimmering Stardust x3, Gold x300,000. No expired section separate from the FINDSTAR10 entry note. No new codes, no status change — only the claim_date stamp moved.

### kaiden — claim_date: 2026-10-19 (unchanged)
"Last updated: October 19, 2026". Active Redeem Codes section shows no readable active entries in text; Expired Codes (5): BUNNYSHOW — Key x150, Compass of Promise x3, Premium Salmon Puree x10; WELCOMENARAE — Premium Salmon Puree x10, Purple Upgrade Stone x1, Compass of Promise x3; SUMMERDIVE2026 — Key x150, Premium Salmon Purée x10, Compass of Promise x3; WINDSOFFATE — Key x100, Conquest Entry Ticket x3, Compass of Promise x3; NAGIUPDATE — Key x100, Conquest Entry Ticket x3, Compass of Promise x3. Matches codes.toml.
Latest News (fallback check for official): Nov 3, 2026 "Mongil Bonus Time and Break the Ice Run to November 10"; Oct 30, 2026 "Mongil Nightmarch: Esther's Halloween Story and Events"; Oct 30, 2026 "Mongil Developer Lounge Talk: November 5 Before 1.5". No newer news item and no new official code.

### supercheats — claim_date: 2026-10-13 (unchanged; metadata Updated 2026-10-13T05:11:52-07:00)
Working: MEMOLMONGIL — 100 Keys, 3 Conquest Entry Tickets, 3 Compass of Promise (reward wording differs from majority, as previously noted); LOVEMONGIL — Overflowing Love (Artifact); LETSPLAYEP6 — 10 Premium Salmon Puree, 3 Compass of Promise, 1 Purple Upgrade Stone; REGINULAUPDATE — 10 Premium Salmon Puree, 5 Compass of Promise, 1 Purple Upgrade Stone (wording differs, as previously noted); SUMMERDIVE2026 — 150 Keys, 10 Premium Salmon Puree, 3 Compass of Promise; WINDSOFFATE — 100 Keys, 3 Conquest Entry Tickets, 3 Compass of Promise; PLAY — Swellymander Monsterling; LETS — Goblin Recruit Monsterling.
Expired: NAGIUPDATE, WELCOMENARAE, SPECIALGIFT0430, MONGILDISCORD, DEVLIVE0410, MONGILPRESENT, DINDINISDINDIN, MONGILREPORT. Matches codes.toml.

### gamsgo — claim_date: 2026-11-02 (unchanged; metadata Updated 2026-11-02T00:00:00+00:00)
Page title "(November 2026)".
Active / reported: HAPPYHALF1014 — Compass of Destiny ×3, Compass of Memory ×3, Compass of Promise ×3, Special Gold Upgrade Stone ×1, "Available during the redemption period", expires November 10, 2026 23:30 UTC (deadline now past, source has NOT moved it to expired — see Verdict); REGINULAUPDATE — Premium Salmon Puree ×10, Purple Upgrade Stone ×1, Compass of Promise ×3, "Recently reported"; LETSPLAYEP6 — same reward, "Recently reported"; MEMOLMONGIL — Green Cappy Papa Monsterling ×1, Keys ×25, Gold ×10,000, "Reported active", conflicting community reports; LOVEMONGIL — Overflowing Love Artifact ×1, "Recently reported"; GSBGMONGIL — Uncle Cappy Monsterling ×1, Gold ×10,000, "Recently reported"; LETS — Goblin Recruit Monsterling ×1, "Recently reported"; PLAY — Swellymander Monsterling ×1, "Recently reported"; JOINMONGIL — Leafymander Monsterling ×1, "Recently reported".
Expired: MONGILDISCORD — Starlight Crystal ×30, Glimmering Stardust ×3, Gold ×300,000; MONGILPRESENT — Brown Cappy Monsterling ×1, Forkmugger Monsterling ×1, Spark Slime Monsterling ×1; DINDINISDINDIN — Green Cappy Monsterling ×1, Gold ×10,000; MONGILREPORT — White Wolf Warrior Monsterling ×1; PLAYMONGIL — Ring Slime Monsterling ×1; SUMMERDIVE2026 — Keys ×150, Premium Salmon Puree ×10, Compass of Promise ×3, expired July 27, 2026; NAGIUPDATE — Keys ×100, Conquest Entry Ticket ×3, Compass of Promise ×3, expired July 6, 2026; WINDSOFFATE — Keys ×100, Conquest Entry Ticket ×3, Compass of Promise ×3, expired July 6, 2026. Matches codes.toml. Events section mentions the November 5 Developer Lounge Talk: Version 1.5 Preview and states no new coupon or viewing reward is guaranteed — no 1.5 launch code named. No MAKEANDDESTROY.

### cosm-game — claim_date: 2026-10-27 (unchanged, "Last checked Oct 27, 2026"; page also "Updated Oct 27, 2026")
"14 active codes Verified Oct 27", 13 expired archived. Code strings in the tables are JS-hidden in text (as in prior sweeps); the tag cloud names HAPPYHALF1014, DEVLIVE0410, REGINULAUPDATE, NEWEPISODE7, DIVEWITHISABELLA, and the reward/date rows map 1:1 onto the codes already in codes.toml (newest active row: 3 Compasses of Destiny + 3 Compasses of Memory + 3 Compasses of Promise + 1 Special Gold Upgrade Stone, released Oct 14, 2026, expires Nov 10, 2026 — HAPPYHALF1014, still in the active table, NOT moved to the expired table despite the deadline now being past). No new strings, no MAKEANDDESTROY. Matches codes.toml.

### gamesradar — claim_date: none (no page-level updated date, as before)
Claimable now: MEMOLMONGIL — Green Cappy Papa (Monsterling), 25x Keys, 10,000 Gold; JOINMONGIL — Leafymander (Monsterling); PLAYMONGIL — Ring Slime (Monsterling); LETS — Goblin Recruit (Monsterling); PLAY — Swellymander (Monsterling); GSBGMONGIL — Uncle Cappy (Monsterling), 10,000 Gold; MONGILDISCORD — 30x Starlight Crystal, 3x Glimmering Stardust, 300,000 Gold; LOVEMONGIL — Overflowing Love (Artifact); MONGILPRESENT — Brown Cappy (Monsterling), Forkmugger (Monsterling), Spark Slime (Monsterling); DINDINISDINDIN — Green Cappy (Monsterling), 10,000 Gold; MONGILREPORT — White Wolf Warrior (Monsterling).
Expired: DEVLIVE0410 — Scar (Monsterling), Spark Slime (Monsterling), White Wolf Warrior (Monsterling) [expired May 13, 2026]; SPECIALGIFT0430 — Blue Upgrade Stone [expired May 7, 2026]; FINDSTAR10 — 10x Compass of Promise [expired May 7, 2026]. Matches codes.toml. (Crawl copy dated 2026-09-02, same as prior sweeps.)

### pocket-tactics — claim_date: none (no page-level updated date, as before)
Listed as new/current: NAGIUPDATE — three conquest entry tickets, three compasses of promise, and 100 keys; REGINULAUPDATE — ten premium salmon puree, one purple upgrade stone, and three compasses of promise; MEMOLMONGIL — rewards (as printed); MONGILDISCORD — 30 diamonds, three glimmering stardust, and 300k gold; DEVLIVE0410 — one Scar monsterling, one White Wolf Warrior monsterling, and one Spark Slime; MONGILREPORT — one White Wolf Warrior monsterling; LOVEMONGIL — one Overflowing Love artifact; MONGILPRESENT — one Brown Cappy monsterling, one Forkmugger monsterling, and one Spark Slime monsterling; DINDINISDINDIN — 10k gold and one Green Cappy monsterling.
Expired: SPECIALGIFT0430 — rewards; FINDSTAR — rewards (as printed — not FINDSTAR10). Matches codes.toml. (Crawl copy dated 2026-09-04, same as prior sweeps.)

### gamingonphone — claim_date: 2026-08-03 (unchanged)
Active table (Date Added/Checked August 3, 2026): NAGIUPDATE — 100x Keys, 3x Conquest Entry Ticket, and 3x Compass of Promise, availability TBA; MEMOLMONGIL — 25x Green Cappy (Monsterling) [as printed], 10,000x Gold, and 25x Keys, availability TBA.
Expired: MONGILPRESENT, DINDINISDINDIN, MONGILREPORT, DEVLIVE0410, MONGILDISCORD, LOVEMONGIL. Matches codes.toml.
