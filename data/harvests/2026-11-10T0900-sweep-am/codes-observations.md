# Codes observations — sweep 2026-11-10T0900-sweep-am (codes-scout)

Read 2026-11-10 ~09:00 ET via browser.open, one read per source in
data/code-sources.toml. Kaiden was read by the Kaiden worker this
sweep (see its section below). Compared against data/codes.toml
(last merged sweep 2026-11-09T0900-sweep-am).

## Verdict

- **NEW codes not in codes.toml: NONE.** MAKEANDDESTROY is still
  listed by NO second source this sweep — it stays single-source
  (official) / Unconfirmed. No source mentions a version 1.5
  launch code: the only 1.5 mention found is GamsGo's events
  section noting the November 5 Developer Lounge Talk: Version
  1.5 Preview, which states "No new coupon or viewing reward is
  guaranteed" and names no code string.
- **Status changes: NONE.** Every readable source's active/expired
  split matches its recorded observations in codes.toml exactly.
  HAPPYHALF1014 is still shown active — NOT prematurely expired
  by any source — (GamsGo: "Available during the redemption
  period", expires November 10, 2026 23:30 UTC; COSM-GAME newest
  active row, released Oct 14, 2026, expires Nov 10, 2026). It
  expires 2026-11-10T23:30Z, still in the future at sweep time
  (09:00 ET = 14:00 UTC). MAKEANDDESTROY likewise expires
  2026-11-10T23:30Z and is listed expired by no source (it is
  listed by no second source at all).
- **claim_date changes: one confirmed** — gamewith-en advances
  2026-11-08 → 2026-11-10 (page metadata
  "Updated: 2026-11-10T00:20:00-05:00"; code list content
  identical). This is a date-only re-stamp with zero content
  change — it reveals no code that a read of the list content
  alone had missed (audit D-5 check). All other sources' stated
  claim_dates are identical to yesterday's observations.
  Two metadata notes for the parent sweep to weigh before
  re-stamping anything (neither changes the stated claim used
  in codes.toml): (1) ldshop's crawl metadata today shows
  "Updated: 2026-09-03T06:41:54.000Z" where yesterday's file
  recorded Updated 2026-09-02 — the page prints no human-readable
  updated date, its Published date is still 2026-09-02, and its
  code list is identical; (2) dexerto's crawl metadata today
  shows "Updated: 2026-11-09T16:51:40.000Z" while its body claim
  — the claim codes.toml records — still reads "Updated
  September 20, 2026", unchanged.
- **Unreadable source: official.** browser.open on
  https://forum.netmarble.com/ returned only the game-selector
  index with no extractable code content this sweep — same
  failure pattern as the 2026-11-04/05/06/07/09 sweeps. No new
  official code was found via any other source read today, and
  the last recorded official observations (HAPPYHALF1014
  claim_date 2026-10-14; MAKEANDDESTROY claim_date 2026-11-05)
  stand unchanged. Kaiden fallback check (Kaiden worker): hub
  Latest News still leads with the Nov 3, 2026 item ("Mongil
  Bonus Time and Break the Ice Run to November 10") and
  mentions no new official code.
- codes.toml updates made by this sweep: all gamewith-en
  observations (12) re-stamped (seen_at 2026-11-10,
  claim_date 2026-11-10) — no other source needs any change,
  and no computed code status changes.

## Per-source observations (claim_date = page's own stated updated date)

### official — https://forum.netmarble.com/ — claim_date: UNREADABLE this sweep (last recorded 2026-10-14 / 2026-11-05)
browser.open returned only the game-selector index — no
code list could be read. Last recorded official evidence stands:
HAPPYHALF1014 (Oct 14 Developer Lounge Talk; valid until Nov. 10,
2026, 23:30 UTC) and MAKEANDDESTROY (Nov 5 Dev Lounge Talk
stream, expires Nov 10, 2026 23:30 UTC). No new official code
found via any other source read today.

### pocket-gamer — claim_date: 2026-11-08 (unchanged)
"Updated on November 8th, 2026 - checked for codes"
Active: REGINULAUPDATE, LETSPLAYEP6, MEMOLMONGIL, LOVEMONGIL,
MONGILPRESENT, MONGILDISCORD, DINDINISDINDIN, DEVLIVE0410,
MONGILREPORT (rewards as recorded).
Expired: SPECIALGIFT0430, FINDSTAR10. Matches codes.toml.

### ldshop — claim_date: 2026-09-02 (unchanged as stated; see metadata note in Verdict)
Active (same 9, same order as Pocket Gamer, pg-syndicated group):
REGINULAUPDATE ("new!"), LETSPLAYEP6, MEMOLMONGIL, LOVEMONGIL,
MONGILPRESENT, MONGILDISCORD, DINDINISDINDIN, DEVLIVE0410,
MONGILREPORT. No expired section. Crawl metadata today:
Published 2026-09-02T15:41:53.000Z, Updated
2026-09-03T06:41:54.000Z. Matches codes.toml.

### dexerto — claim_date: 2026-09-20 (unchanged)
"Updated September 20, 2026 … No new codes found."
Active (9): REGINULAUPDATE, LETSPLAYEP6, MEMOLMONGIL,
MONGILDISCORD, LOVEMONGIL, DEVLIVE0410, MONGILPRESENT,
DINDINISDINDIN, MONGILREPORT.
Expired: LETS, PLAY. Matches codes.toml. (Crawl metadata
"Updated: 2026-11-09T16:51:40.000Z" — body claim unchanged;
see Verdict note.)

### gamewith-en — claim_date: 2026-11-10 (CHANGED from 2026-11-08; metadata 2026-11-10T00:20:00-05:00)
Content unchanged. Available Codes list: FINDSTAR10 (entry
states "Until the end of 07/May/2026" — recorded expired in
codes.toml), JOINMONGIL, PLAYMONGIL, PLAY, LETS, GSBGMONGIL,
MONGILREPORT, DEVLIVE0410, DINDINISDINDIN, MONGILPRESENT,
LOVEMONGIL, MONGILDISCORD. No new codes, no status change.

### kaiden — claim_date: 2026-10-19 (unchanged; read by the Kaiden worker)
"Last updated: October 19, 2026". Active Redeem Codes section
shows no readable active entries in text; Expired Codes (5):
BUNNYSHOW, WELCOMENARAE, SUMMERDIVE2026, WINDSOFFATE,
NAGIUPDATE. Matches codes.toml.
Latest News (fallback check for official): Nov 3, 2026
"Mongil Bonus Time and Break the Ice Run to November 10";
Oct 30, 2026 "Mongil Nightmarch: Esther's Halloween Story
and Events"; Oct 30, 2026 "Mongil Developer Lounge Talk:
November 5 Before 1.5". No newer news item and no new
official code.

### supercheats — claim_date: 2026-10-13 (unchanged)
Working: MEMOLMONGIL, LOVEMONGIL, LETSPLAYEP6, REGINULAUPDATE,
SUMMERDIVE2026, WINDSOFFATE, PLAY, LETS.
Expired: NAGIUPDATE, WELCOMENARAE, SPECIALGIFT0430,
MONGILDISCORD, DEVLIVE0410, MONGILPRESENT, DINDINISDINDIN,
MONGILREPORT. Matches codes.toml.

### gamsgo — claim_date: 2026-11-02 (unchanged)
Page "Updated: 2026-11-02T00:00:00+00:00", title "(November 2026)".
Active / reported: HAPPYHALF1014 ("Available during the
redemption period", expires November 10, 2026 23:30 UTC),
REGINULAUPDATE, LETSPLAYEP6, LOVEMONGIL, GSBGMONGIL, LETS,
PLAY, JOINMONGIL, MEMOLMONGIL ("Reported active", conflicting
community reports).
Expired: MONGILDISCORD, MONGILPRESENT, DINDINISDINDIN,
MONGILREPORT, PLAYMONGIL, SUMMERDIVE2026, NAGIUPDATE,
WINDSOFFATE. Matches codes.toml. Events section mentions the
November 5 Developer Lounge Talk: Version 1.5 Preview and
states no new coupon or viewing reward is guaranteed — no 1.5
launch code named.

### cosm-game — claim_date: 2026-10-27 (unchanged, "Last checked Oct 27, 2026")
"14 active codes Verified Oct 27", 13 expired archived. Code
strings in the tables are JS-hidden in text (as in prior
sweeps); the tag cloud names HAPPYHALF1014, DEVLIVE0410,
REGINULAUPDATE, NEWEPISODE7, DIVEWITHISABELLA, and the
reward/date rows map 1:1 onto the codes already in codes.toml
(newest active row: released Oct 14, 2026, expires Nov 10,
2026 — HAPPYHALF1014, still in the active table, not the
expired table). No new strings. Matches codes.toml.

### gamesradar — claim_date: none (no page-level updated date, as before)
Claimable now: MEMOLMONGIL, JOINMONGIL, PLAYMONGIL, LETS,
PLAY, GSBGMONGIL, MONGILDISCORD, LOVEMONGIL, MONGILPRESENT,
DINDINISDINDIN, MONGILREPORT.
Expired: DEVLIVE0410 [expired May 13, 2026], SPECIALGIFT0430,
FINDSTAR10. Matches codes.toml. (Crawl copy dated 2026-09-02,
same as prior sweeps.)

### pocket-tactics — claim_date: none (no page-level updated date, as before)
Listed as new/current: NAGIUPDATE, REGINULAUPDATE,
MEMOLMONGIL, MONGILDISCORD, DEVLIVE0410, MONGILREPORT,
LOVEMONGIL, MONGILPRESENT, DINDINISDINDIN.
Expired: SPECIALGIFT0430, FINDSTAR (as printed — not
FINDSTAR10). Matches codes.toml. (Crawl copy dated 2026-09-04,
same as prior sweeps.)

### gamingonphone — claim_date: 2026-08-03 (unchanged)
Active table (Date Added/Checked August 3, 2026): NAGIUPDATE,
MEMOLMONGIL.
Expired: MONGILPRESENT, DINDINISDINDIN, MONGILREPORT,
DEVLIVE0410, MONGILDISCORD, LOVEMONGIL. Matches codes.toml.
