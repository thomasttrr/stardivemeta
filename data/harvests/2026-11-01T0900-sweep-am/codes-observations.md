# Codes observations — sweep 2026-11-01T0900-sweep-am (codes-scout)

Read 2026-11-01 ~08:57–08:58 ET via browser.open, sequential, one read per
source in data/code-sources.toml. Compared against data/codes.toml
(last merged sweep 2026-10-29T2105-sweep-pm + GameWith-EN 2026-10-31 rows).

## Verdict

- **NEW codes not in codes.toml: NONE.**
- **Status changes vs codes.toml: NONE.** Every source's active/expired
  split matches its recorded observations.
- **claim_date changes: one** — gamewith-en advances 2026-10-31 → 2026-11-01
  (page metadata "Updated: 2026-11-01"); its code list content is unchanged.
  All other claim_dates are identical to the previous sweep.

## Per-source observations (claim_date = page's own stated updated date)

### official — https://forum.netmarble.com/ — claim_date: 2026-10-14
Forum landing lists games only (no code list). Official code evidence is the
10/14 Developer Lounge Talk forum write-up
(http://forum.netmarble.com/stardive_gl/view/4/719, re-read this sweep):
HAPPYHALF1014 — active, "Code Validity Period (UTC): After the livestream -
11/10 (Tue) 23:30 (UTC)", rewards Compass of Destiny x3 / Compass of Memory
x3 / Compass of Promise x3 / Special Gold Upgrade Stone x1. Matches
codes.toml. No new official code found.

### pocket-gamer — claim_date: 2026-10-25 (unchanged)
"Updated on October 25th, 2026 - checked for codes"
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

### gamewith-en — claim_date: 2026-11-01 (CHANGED from 2026-10-31)
Metadata "Updated: 2026-11-01T02:49:02-05:00". Content unchanged.
Available Codes list: FINDSTAR10 (entry itself states "Until the end of
07/May/2026" — recorded expired in codes.toml), JOINMONGIL, PLAYMONGIL,
PLAY, LETS, GSBGMONGIL, MONGILREPORT, DEVLIVE0410, DINDINISDINDIN,
MONGILPRESENT, LOVEMONGIL, MONGILDISCORD. No new codes, no status change.

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

### gamsgo — claim_date: 2026-09-30 (unchanged)
Active: REGINULAUPDATE, LETSPLAYEP6, LOVEMONGIL, GSBGMONGIL, LETS, PLAY,
PLAYMONGIL, JOINMONGIL.
Reported active (conflicting): MEMOLMONGIL, MONGILDISCORD, MONGILPRESENT,
DINDINISDINDIN, MONGILREPORT.
Text states SUMMERDIVE2026, NAGIUPDATE, WINDSOFFATE have expired.
Matches codes.toml.

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
2026-11-01. Latest store build is 1.04.10 (Oct 28, 2026), whose changelog
is the "1.4 Version Major Update". Version 1.5 is upcoming only: previewed
at the Nov 5, 2026 Developer Lounge Talk (10:00 UTC) and expected with the
Nov 10 update per Kaiden's event schedule. See parent sweep report for
evidence URLs.
