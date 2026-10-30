//! T4.4 acceptance: `mongil-build note` generates the movers note
//! from the snapshot diff alone (D-13) — byte-for-byte against a
//! golden file — plus the D-12 zero-change note and the
//! launch-baseline note.

mod common;

use common::{dataset_with, run_bin, write_files};
use std::path::PathBuf;

fn snapshot(data: &PathBuf, id: &str) {
    let out = run_bin(
        &["snapshot", "--data", data.to_str().unwrap(), "--id", id],
        None,
    );
    assert_eq!(out.code, Some(0), "snapshot {id}: {}", out.combined());
}

fn note(data: &PathBuf, from: &str, to: &str, date: &str) -> common::BinOutput {
    run_bin(
        &[
            "note",
            "--data",
            data.to_str().unwrap(),
            "--from",
            from,
            "--to",
            to,
            "--date",
            date,
        ],
        None,
    )
}

/// Pocket Gamer drops Narae S/A → C/C between the two snapshots:
/// her Overall falls S → A at every level and her label flips
/// Solid → Contested. The generated note must match the golden text
/// exactly — every line comes from the diff, nothing is free-written.
#[test]
fn note_from_diff_golden() {
    let data = dataset_with("note-golden", "placements/pg.toml", common::PG);
    snapshot(&data, "2026-11-03-am");
    let flipped = common::PG.replace(
        "character = \"narae\"\nconquest = \"S\"\nrift = \"A\"",
        "character = \"narae\"\nconquest = \"C\"\nrift = \"C\"",
    );
    write_files(&data, &[("placements/pg.toml", &flipped)]);
    snapshot(&data, "2026-11-10-am");

    let out = note(&data, "2026-11-03-am", "2026-11-10-am", "2026-11-10");
    assert_eq!(out.code, Some(0), "note: {}", out.combined());
    let text = std::fs::read_to_string(data.join("notes/2026-11-10.md")).unwrap();
    let expected = r#"---
from: 2026-11-03-am
to: 2026-11-10-am
status: draft
---

# What moved: Nov 3, 2026 → Nov 10, 2026
6 changes. Each one links to the list that caused it.

## Tier changes
- Narae: S → A at Low. Why: Pocket Gamer changed S → C ([Pocket Gamer](https://www.pocketgamer.com/mongil-star-dive/tier-list/)).
- Narae: S → A at High. Why: Pocket Gamer changed S → C ([Pocket Gamer](https://www.pocketgamer.com/mongil-star-dive/tier-list/)).
- Narae: S → A at Max. Why: Pocket Gamer changed S → C ([Pocket Gamer](https://www.pocketgamer.com/mongil-star-dive/tier-list/)).

## Label changes
- Narae: Solid → Contested at Low. Its tier moved S → A.
- Narae: Solid → Contested at High. Its tier moved S → A.
- Narae: Solid → Contested at Max. Its tier moved S → A.
"#;
    assert_eq!(text, expected);
}

/// D-12: an empty diff produces the zero-change note with
/// `status: auto` — the only note kind that publishes unapproved.
#[test]
fn note_zero_change_is_auto() {
    let data = dataset_with("note-zero", "placements/pg.toml", common::PG);
    snapshot(&data, "2026-11-03-am");
    let out = note(&data, "2026-11-03-am", "2026-11-03-am", "2026-11-10");
    assert_eq!(out.code, Some(0), "note: {}", out.combined());
    let text = std::fs::read_to_string(data.join("notes/2026-11-10.md")).unwrap();
    assert!(text.contains("status: auto"), "{text}");
    assert!(
        text.contains("Nothing moved this week. Lists last checked Nov 3, 2026."),
        "{text}"
    );
}

/// The launch baseline (`--from none`): every placement in the first
/// snapshot is a new placement, and the note is a draft — it makes
/// claims, so Tom approves it (CP4).
#[test]
fn note_baseline_from_none() {
    let data = dataset_with("note-baseline", "placements/pg.toml", common::PG);
    snapshot(&data, "2026-11-03-am");
    let out = note(&data, "none", "2026-11-03-am", "2026-11-03");
    assert_eq!(out.code, Some(0), "note: {}", out.combined());
    let text = std::fs::read_to_string(data.join("notes/2026-11-03.md")).unwrap();
    assert!(text.contains("status: draft"), "{text}");
    assert!(text.contains("# What moved: launch → Nov 3, 2026"), "{text}");
    assert!(text.contains("## New placements"), "{text}");
    assert!(
        text.contains("Pocket Gamer placed Narae at Conquest S · Rift A"),
        "{text}"
    );
    assert!(text.contains("## Sources"), "{text}");
    assert!(text.contains("Pocket Gamer is now counted"), "{text}");
}

/// Regeneration never overwrites a note Tom has approved.
#[test]
fn note_refuses_to_overwrite_approved() {
    let data = dataset_with("note-approved", "placements/pg.toml", common::PG);
    snapshot(&data, "2026-11-03-am");
    let out = note(&data, "none", "2026-11-03-am", "2026-11-03");
    assert_eq!(out.code, Some(0), "note: {}", out.combined());
    let path = data.join("notes/2026-11-03.md");
    let approved = std::fs::read_to_string(&path)
        .unwrap()
        .replace("status: draft", "status: approved");
    std::fs::write(&path, &approved).unwrap();
    let out = note(&data, "none", "2026-11-03-am", "2026-11-03");
    assert_eq!(out.code, Some(1), "note: {}", out.combined());
    assert_eq!(std::fs::read_to_string(&path).unwrap(), approved);
}
