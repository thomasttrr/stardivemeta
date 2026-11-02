# Codes observations — sweep 2026-11-02T0900-sweep-am (codes-scout, audit)

Read 2026-11-02 ~08:58–09:05 ET via browser.open, sequential, one read per
source in data/code-sources.toml. Compared against data/codes.toml
(last merged sweep 2026-10-29T2105-sweep-pm + GameWith-EN 2026-11-01 rows).

## Verdict

- **NEW codes not in codes.toml: NONE.** One new observation:
  GamsGo now lists HAPPYHALF1014 as active (it did not list it before).
- **Status changes: GamsGo only.** On its page updated 2026-11-02,
  GamsGo moved MONGILDISCORD, MONGILPRESENT, DINDINISDINDIN,
  MONGILREPORT and PLAYMONGIL from active/reported-active to its
  Expired table. Every other source's active/expired split matches
  its recorded observations.
- **claim_date changes: two** — pocket-gamer advances
  2026-10-25 → 2026-11-01 ("Updated on November 01st, 2026 - checked
  for codes"; code list content unchanged), and gamsgo advances
  2026-09-30 → 2026-11-02 (page "Updated: 2026-11-02"). All other
  claim_dates are identical to the previous sweep. GameWith EN's
  codes page metadata moved to 2026-11-01T22:26:23-05:00, but its
  claim date stays 2026-11-01 and its list content is unchanged.
- codes.toml updates in this sweep: all pocket-gamer observations
  re-stamped (seen_at 2026-11-02, claim_date 2026-11-01); all gamsgo
  observations re-stamped (seen_at 2026-11-02, claim_date
  2026-11-02) with the five status flips above and the new
  HAPPYHALF1014 observation added. No computed code status changes:
  the flipped codes are still listed active by fresher-or-equal
  independent sources (Pocket Gamer, claim 2026-11-01), and
  HAPPYHALF1014 was already confirmed active by official + COSM.

## Per-source observations (claim_date = page's own stated updated date)

### official — https://forum.netmarble.com/ — claim_date: 2026-10-14
Forum landing lists games only (no code list). Official code evidence is the
10/14 Developer Lounge Talk forum write-up (re-read this sweep via Kaiden's
recap, https://www.kaiden.gg/blog/mongil-october-14-developer-lounge-talk-recap/):
HAPPYHALF1014 — active, "Valid until Nov. 10, 2026, 23:30 (UTC)", rewards
Compass of Destiny x3 / Compass of Memory x3 / Compass of Promise x3 /
Special Gold Upgrade Stone x1. Matches codes.toml. No new official code found.

### pocket-gamer — claim_date: 2026-11-01 (CHANGED from 2026-10-25)
"Updated on November 01st, 2026 - checked for codes"
Active: REGINULAUPDATE, LETSPLAYEP6, MEMOLMONGIL, LOVEMONGIL,
MONGILPRESENT, MONGILDISCORD, DINDINISDINDIN, DEVLIVE0410, MONGILREPORT.
Expired: SPECIALGIFT0430, FINDSTAR10. Matches codes.toml.

### ldshop — claim_date: 2026-09-02 (unchanged; metadata Updated 2026-09-03)
Active (same 9, same order as Pocket Gamer, pg-syndicated group):
REGINULAUPDATE ("new!"), LETSPLAYEP6, MEMOLMONGIL, LOVEMONGIL,
MONGILPRESENT, MONGILDISCORD, DINDINISDINDIN, DEVLIVE0410, MONGILREPORT.
No expired section. Matches codes.toml.

### dexerto — claim_date: 2026-09-20 (unchanged)
"Updated September 20, 2026 … No new codes found."
Active (9): REGINULAUPDATE, LETSPLAYEP6, MEMOLMONGIL, MONGILDISCORD,
LOVEMONGIL, DEVLIVE0410, MONGILPRESENT, DINDINISDINDIN, MONGILREPORT.
Expired: LETS, PLAY. Matches codes.toml.

### gamewith-en — claim_date: 2026-11-01 (unchanged date; metadata 2026-11-01T22:26:23-05:00)
Content unchanged. Available Codes list: FINDSTAR10 (entry itself states
"Until the end of 07/May/2026" — recorded expired in codes.toml),
JOINMONGIL, PLAYMONGIL, PLAY, LETS, GSBGMONGIL, MONGILREPORT,
DEVLIVE0410, DINDINISDINDIN, MONGILPRESENT, LOVEMONGIL, MONGILDISCORD.
No new codes, no status change.

### kaiden — claim_date: 2026-10-19 (unchanged)
"Last updated: October 19, 2026". Active Redeem Codes section shows no
readable active entries in text; Expired Codes (5): BUNNYSHOW,
WELCOMENARAE, SUMMERDIVE2026, WINDSOFFATE, NAGIUPDATE. Matches codes.toml.

### supercheats — claim_date: 2026-10-13 (unchanged)
Working: MEMOLMONGIL, LOVEMONGIL, LETSPLAYEP6, REGINULAUPDATE,
SUMMERDIVE2026, WINDSOFFATE, PLAY, LETS.
Expired: NAGIUPDATE, WELCOMENARAE, SPECIALGIFT0430, MONGILDISCORD,
DEVLIVE0410, MONGILPRESENT, DINDINISDINDIN, MONGILREPORT.
Matches codes.toml (incl. its SUMMERDIVE2026/WINDSOFFATE "working" outlier).

### gamsgo — claim_date: 2026-11-02 (CHANGED from 2026-09-30)
Page "Updated: 2026-11-02T00:00:00+00:00", title now "(November 2026)".
Active / reported: HAPPYHALF1014 ("Available during the redemption
period", expires November 10, 2026 23:30 UTC — NEW observation for
this source), REGINULAUPDATE, LETSPLAYEP6, LOVEMONGIL, GSBGMONGIL,
LETS, PLAY, JOINMONGIL ("Recently reported"), MEMOLMONGIL
("Reported active", conflicting community reports).
Expired (page's Expired table): MONGILDISCORD, MONGILPRESENT,
DINDINISDINDIN, MONGILREPORT, PLAYMONGIL — all five CHANGED from
active/reported-active in the previous read — plus SUMMERDIVE2026,
NAGIUPDATE, WINDSOFFATE (expired before, unchanged).

### cosm-game — claim_date: 2026-10-27 (unchanged, "Last checked Oct 27, 2026")
"14 active codes Verified Oct 27", 13 expired archived. Code strings in the
tables are JS-hidden in text (as in prior sweeps); the tag cloud names
HAPPYHALF1014, DEVLIVE0410, REGINULAUPDATE, NEWEPISODE7,
DIVEWITHISABELLA, and the reward/date rows map 1:1 onto the codes already
in codes.toml (newest: HAPPYHALF1014, released Oct 14, 2026, expires
Nov 10, 2026). No new strings. Matches codes.toml.

### gamesradar — claim_date: none (no page-level updated date, as before)
Claimable now: MEMOLMONGIL, JOINMONGIL, PLAYMONGIL, LETS, PLAY,
GSBGMONGIL, MONGILDISCORD, LOVEMONGIL, MONGILPRESENT, DINDINISDINDIN,
MONGILREPORT.
Expired: DEVLIVE0410 [expired May 13, 2026], SPECIALGIFT0430,
FINDSTAR10. Matches codes.toml.

### pocket-tactics — claim_date: none (no page-level updated date, as before)
Listed as new/current: NAGIUPDATE, REGINULAUPDATE, MEMOLMONGIL,
MONGILDISCORD, DEVLIVE0410, MONGILREPORT, LOVEMONGIL, MONGILPRESENT,
DINDINISDINDIN.
Expired: SPECIALGIFT0430, FINDSTAR (as printed — not FINDSTAR10).
Matches codes.toml.

### gamingonphone — claim_date: 2026-08-03 (unchanged)
Active table (Date Added/Checked August 3, 2026): NAGIUPDATE,
MEMOLMONGIL.
Expired: MONGILPRESENT, DINDINISDINDIN, MONGILREPORT, DEVLIVE0410,
MONGILDISCORD, LOVEMONGIL. Matches codes.toml.

## Patch-watch note (same sweep)
No game version newer than 1.4 (data/game.toml) has been released as of
2026-11-02. Latest store build is still 1.04.10 (Oct 28/29, 2026), whose
changelog is the "1.4 Version Major Update". Version 1.5 is upcoming
only: previewed at the Nov 5, 2026 Developer Lounge Talk (10:00 UTC)
and expected with the Nov 10 update per Kaiden's event schedule /
GamsGo's November events table. No game.toml change, no staleness
re-evaluation.
