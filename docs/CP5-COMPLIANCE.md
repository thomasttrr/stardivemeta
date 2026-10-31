# CP5 — Compliance re-read: Netmarble fan-content policy vs StarDiveMeta

Read and written: 2026-10-30 (overnight job 2, launch gate CP5).
Research + findings only; no site code or copy was changed. Fixes, if any
are wanted, are Tom's call.

## Policy sources read (official Netmarble pages only)

1. **Netmarble Content Creator Guidelines** — dated **Mar 21, 2025** on the
   page; read 2026-10-30.
   https://help.netmarble.com/en/terms/ch_creator_guidelines_en
   This is Netmarble's fan-content policy. It governs use of "Netmarble game
   images" (play video or still screens, e.g. screenshots).
2. **Netmarble Terms of Service** — last updated **25 June 2025**; §10
   (Intellectual property) and §2.1 read 2026-10-30.
   https://help.netmarble.com/terms/terms_of_service_en
3. **MONGIL: STAR DIVE official site** — checked 2026-10-30 for a
   game-specific fan policy (the Guide says a per-game policy takes
   precedence over it if one exists).
   https://stardive.netmarble.com/en/
   **No separate MONGIL: STAR DIVE fan-content policy was found.** The
   game's store listings point only to the general Terms of Service and
   Privacy Policy. So the general Content Creator Guidelines govern.

## Verdict

**The site's current posture fits the policy.** Every point below passes.
Two notes are recorded at the end: one wording-tightening option and one
standing residual risk that no fan site can remove. Neither blocks launch.

## Point-by-point fit

1. **Not-official disclaimer — PASS.**
   Guide: "Please clearly indicate the contents are not official contents
   of Netmarble." Every page of the site carries the footer disclaimer
   (build gate G12 checks the exact string on every page):
   "Fan-made site. Not an official MONGIL: STAR DIVE or Netmarble site.
   Game names and terms belong to their owners." The About page also opens
   with "StarDiveMeta is a fan project by one player." Nothing on the site
   claims or implies a partnership, sponsorship, or official status.

2. **Non-commercial use — PASS (stricter than required).**
   Guide: "Netmarble game images should be used for non-profit purposes
   only." (Its platform ad-revenue/donation exception applies to video
   platforms; the site doesn't use it.) The site has no ads, no affiliate
   links, no donation links, no paywall, no accounts — the About page says
   so in plain words: "No ads. No accounts."

3. **Game images — PASS (official portraits only, credited).**
   The Guide licenses screenshots/stills for introduction, review, and
   promotion, for non-profit use. Since the Starfield theme (2026-10-31,
   Tom's call), the site uses **official character portraits**:
   face crops of art Netmarble itself published (the official site and
   forum profile cards), plus five bust portraits from Kaiden.gg under
   Tom's recorded exception where no Netmarble square portrait exists.
   Every page's footer credits them: "Character portraits are official
   MONGIL: STAR DIVE art © Netmarble, used non-commercially." No logos,
   no screenshots on pages. Build gate G14 restricts every image file
   to an allowlist; the allowlist holds the portraits under
   `img/own/art/` and the site's own generated PNGs (the tier-board
   images and og.png, rendered from the site's own text/data). No ripped
   game files, no datamined assets — those are banned site-wide by the
   plan's hard constraints.

4. **No modification or falsification of game material — PASS.**
   Guide: "You should not illegally modify, remake, or falsify Netmarble's
   games or Netmarble's game images." The site falsifies no game
   material. The only change to published art is cropping the portraits
   to the face for the round frames — nothing redrawn, recoloured, or
   misrepresented. Its boards and PNGs are original renders of its own
   computed combined data. Animated/3D experiments, if ever added, are constrained by the
   plan to original or properly licensed art — never extracted assets.

5. **No unreleased or unpublicized content — PASS.**
   Guide: "You should not acquire or publicize in an abnormal manner
   Netmarble games that have not yet been publicized." The site covers
   only the released game (v1.4) from publicly published tier lists and
   publicly posted redeem codes. No leaks, no datamines, no embargoed
   material.

6. **No harm to Netmarble's interests or reputation — PASS.**
   Guide: content should not "unfairly harm the legitimate interests of
   Netmarble" or damage the games' reputation/brand image. The site is a
   neutral aggregator: it shows three public lists' placements with
   sources, dates, and a corrections page; it publishes no attacks, no
   cheats, no exploits, no political/religious/hate content (the Guide's
   other listed prohibitions — none apply to anything the site does).

7. **Region scope — PASS.**
   The Guide "only applies to regions where Netmarble games are
   officially published." MONGIL: STAR DIVE is officially published in the
   United States (the site's home region) and globally.

8. **Names and trademarks — PASS.**
   Terms of Service §10.1–10.2: service IP, names, and trademarks belong
   to Netmarble/licensors. The site uses the game's name only to identify
   what it covers (nominative use), never Netmarble's logo, and the
   footer states "Game names and terms belong to their owners." The
   site's own name, StarDiveMeta, and its domain contain no Netmarble
   trademark. Source lists are credited by name with links and dates on
   every evidence page (those are third-party sites' names, used the
   same nominative way).

9. **Original text — PASS.**
   All site copy is written for this site. Harvested material is factual
   data (placements, codes, dates) re-expressed in the site's own words
   and formats, with the source linked — not copied articles, guides, or
   artwork from Pocket Gamer, Kaiden, GameWith, or anyone else.

## Notes (not breaches)

- **Disclaimer wording option.** The Guide gives a recommended sentence:
  'This content is not the official material of Netmarble and contains
  trademarks or works for which Netmarble holds the rights to.' It is
  offered as an example ("such as including the following statement"),
  not a mandated string. The site's footer covers both required elements
  — not official, trademarks belong to their owners — in shorter words.
  Verdict: fits as-is. If Tom ever wants belt-and-braces, swapping the
  second footer line for Netmarble's sentence is a one-line change;
  it would also require updating the G12 gate string to match.

- **Standing residual risk (true of every fan site).** The general Terms
  of Service (§10.2) reserve all rights and bar derivative works absent
  written agreement; the Creator Guidelines are Netmarble's published
  permission framework, but they are written around game *images* and
  don't expressly discuss text/data fan sites like this one. The site's
  actual IP footprint is minimal (the game's name, game terms, factual
  data) and it follows the Guide's rules wherever they apply. Still:
  Guide §2 reserves Netmarble's right to request deletion of
  non-compliant postings, the Guide "may be updated from time to time,"
  and Guide §2 also says Netmarble may reuse fan content for promotion
  unless the creator asks it to stop. Practical consequence: re-run this
  re-read at each launch gate (the T6.2 checklist) and after any Guide
  amendment, and treat any future Netmarble notice as act-first,
  argue-never.
