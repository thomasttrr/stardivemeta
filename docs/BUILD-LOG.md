# StarDiveMeta — Build Log

Format: one line per task — task id, date, commit, how it was checked (plan A.0 rule 6).

## Approval (T0.1)

- Plan: `docs/PLAN-v1.md` (stardivemeta-build-plan-v1.md, by Opus 5.5, 2026-10-29)
- Plan sha256: `7dc1ca29df373d48e78f4485c6da551550543a53dd12d472b56f560987ad1321`
- Approved by Tom: 2026-10-29 — whole plan, as written. Part D vetoes: none. Part E answers: all defaults (Q1 buy stardivemeta.com only at Cloudflare Registrar; Q2 approvals via GitHub PR merges in the mobile app; Q3 X drafts launch posts, Tom posts, X never posts; Q4 no corrections email in v1, D-9 off).
- Approval record: `docs/APPROVAL-v1.md`.

## Toolchain (installed 2026-10-29, this VM)

- rustc 1.99.1 (58cf8f9c3 2026-10-13) / cargo 1.99.1 (5f94df478 2026-08-27) — rustup, stable, under ~/.cargo + ~/.rustup. Pinned in `rust-toolchain.toml`.
- Node v24.22.1 (system binary, no bundled npm/corepack).
- pnpm 10.34.9 — via corepack 0.34.6 (fetched from registry.npmjs.org into ~/.corepack-dist; shims in ~/.local/bin; COREPACK_HOME ~/.cache/corepack).

## Task log

- T0.1 · 2026-10-29 · initial commit · `docs/PLAN-v1.md` copied byte-identical; sha256 verified equal to the approved plan hash (above). 8 input files copied to `docs/inputs/`; approval record filed as `docs/APPROVAL-v1.md`.
- T1.1 (scaffold portion) · 2026-10-29 · initial commit · Repo tree per §B.1 created; workspace with 3 member crates; `cargo build --workspace` result recorded below. Remaining T1.1 acceptance (site build on CI + Windows) is checked when the Astro site exists.
- T1.1 build check · 2026-10-29 · commit d04ddd9 · `cargo build --workspace` exit 0 — mongil-core, mongil-render, mongil-build all compiled (dev profile, 1.90s compile after the pinned 1.99.1 toolchain auto-installed on first run).
- T1.2 · 2026-10-29 · commit bc5ff13 · Data model + loaders: Serde structs for all §B.3 files (deny_unknown_fields), per-source tier enums, `load_dataset` with reference validation. Checked by the 4 named acceptance unit tests (load_rejects_unknown_field, load_rejects_bad_tier, load_rejects_duplicate_character, load_rejects_unknown_character_ref) + a minimal-dataset positive test — all pass.
- T1.3 · 2026-10-29 · commit ee457c6 · Fixture dataset transcribed from the dataset file's named line ranges (PG 24–34, Kaiden 129–159, JP 195–204, urgametips 180–183, gacha.gg 216–220, GameWith EN 46–48); D-G1 first-listed rule in Kaiden `tier` with the full line in `raw`; Chloe `no_placement`; JP's six absentees `not_in_text_tiers`. `data/method/v1.0.toml` written per §B.3; fixture `method.toml` is a byte-identical copy (verified by diff). Acceptance note: the plan's double-transcription check runs when T2.1's harvest-diff exists; the fixture is proven consistent by the T1.5 suite reproducing all 87 simulation results from it.
- T1.4 · 2026-10-29 · commit 266fffb · Method engine: integer half-point votes; band/label decisions in integer milli-points (exactly the plan's 7n/6n/4n/2n and spread ≤ 2 formulas at the v1.0 constants, and still sensitive to constant edits — see the bite check); mode-split override; board ordering; score_display; per-mode boards; constants only from MethodConfig. One unit test per rule (10 method tests) — all pass.
- T1.5 · 2026-10-29 · commit a7c829f · `cargo test -p mongil-core` fully green: 43 sim tests (29 per-character + 12 aggregate + sim_v1_missing_is_not_zero + derive_beginner_v1, driven by tests/fixtures/sim-v1-expected.json) + 8 proptest invariants × 2,000 cases + 15 unit tests. Workspace clippy `-D warnings` clean, fmt clean.
- T1.5 tests-bite check · 2026-10-29 · scratch branch (discarded) · With `s_min` set to 3.3 in the fixture method.toml, the sim suite failed in exactly the expected places: sim_v1_band_counts_low and sim_v1_mina (the two the plan names), plus sim_v1_band_counts_high, sim_v1_band_counts_max, sim_v1_board_order_low, sim_v1_ophelia and derive_beginner_v1 (the same 3.33-score promotions). Branch deleted; fixture restored to s_min 3.5 and re-verified green.
