# T2.5 — GameWith JP visual quick table (最強キャラ早見表)

Source: https://gamewith.jp/stardive/552850
Visual read: B-read #1, 2026-10-29. Page update shown: 2026-10-21 11:41.
Status: staging only. These cells are not in `data/` and are not counted unless Tom approves the extension at CP2.

## Full visual table, in the read's reading order

| Visual tier | Characters, with awakening (凸) shown in the cell |
|---|---|
| SS (Tier1) | Francis 6凸, Narae 0凸, Francis: Summer Dive! 0凸, Mina 0凸, Brisshell 0凸, Ophelia 2凸, Chloe 0凸, Isabella 0凸, Esther 0凸 |
| S (Tier2) | Penny 1凸, Jiwon 0凸, Nagi 0凸, Verna: Summer Dive! 0凸, Mabel 0凸, Vivian 0凸, Cloud 4凸, Yeonhwa 1凸, Benjamin 4凸 |
| A (Tier3) | Flare 2凸, Gabi 2凸, Angel 4凸, Sera 6凸, Reina 6凸 |
| B (Tier4) | Sangun 2凸, Bonnie 2凸, Ellie 4凸 |
| C (Tier5) | Verna 6凸, Leeho 6凸 |
| D (Tier6) | Daisy 6凸 |

The six text-absent cells are also staged in `t25-jp-visual.toml`: Sangun B/A2, Bonnie B/A2, Ellie B/A4, Verna C/A6, Leeho C/A6, Daisy D/A6.

## Criteria caution

GameWith JP's text-tier criteria caveat is that limited 5★ are evaluated at A0, permanent 5★ up to A2, and 4★ up to A6. The visual table does **not** present one uniform awakening per tier or rarity: its cells carry per-character counts from 0凸 through 6凸 (for example Ophelia SS at 2凸, Cloud S at 4凸, Francis SS at 6凸, and Sera A at 6凸). The visual cells therefore should not be described as if they were all produced under one awakening assumption. Any approval should be for the visual tier as published, with the individual awakening shown in evidence.

## Before / after, using the A-read dataset

Method used: a scratch data directory was populated from the sweep's A-read PG, Kaiden and GameWith JP placements (only harvest-only meta annotations were stripped so the strict loader would accept the files) and emitted with the repo's unmodified `mongil-build`. For the hypothetical, a separate scratch copy of `mongil-core` extended `JpTier` with B/C/D and scratch method files supplied the stated conversions; no repo code or `data/` file was changed.

Primary extension tested: JP SS = 4, S = 3, A = 2, **B = 1, C = 0.5, D = 0**.

| Character | Visual JP | Current Low | Current High | Current Max | With visual Low | With visual High | With visual Max |
|---|---|---:|---:|---:|---:|---:|---:|
| Sangun | B | C · Contested (1.75) | C · Contested (1.75) | B · Contested (2.25) | C · Contested (1.50) | C · Contested (1.50) | C · Contested (1.83) |
| Bonnie | B | C · Solid (1.00) | C · Solid (1.00) | C · Solid (1.00) | C · Solid (1.00) | C · Solid (1.00) | C · Solid (1.00) |
| Ellie | B | B · Solid (2.25) | B · Solid (2.25) | B · Solid (2.25) | C · Contested (1.83) | C · Contested (1.83) | C · Contested (1.83) |
| Verna | C | C · Solid (1.00) | C · Solid (1.00) | C · Solid (1.00) | D · Solid (0.83) | D · Solid (0.83) | D · Solid (0.83) |
| Leeho | C | C · Solid (1.00) | C · Solid (1.00) | C · Solid (1.00) | D · Solid (0.83) | D · Solid (0.83) | D · Solid (0.83) |
| Daisy | D | D · Solid (0.50) | D · Solid (0.50) | D · Solid (0.50) | D · Solid (0.33) | D · Solid (0.33) | D · Solid (0.33) |

Coverage rises from 2 to 3 for all six if the visual cells are counted. Bonnie's band, label and score do not change. Sangun changes only at Max (B → C). Ellie changes at every level (B/Solid → C/Contested). Verna and Leeho change at every level (C → D, still Solid). Daisy stays D/Solid with a lower score.

### Alternative C value

The alternative **B = 1, C = 0, D = 0** scale — also the extension printed in plan T2.5 — was run through the same scratch engine. It changes **no band or label** for any of the six compared with C = 0.5. The only differences are Verna and Leeho: score 0.83 → 0.67 and spread 0.5 → 1.0 at every level; they remain D · Solid.
