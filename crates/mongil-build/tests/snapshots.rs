//! Acceptance tests for `mongil-build snapshot` / `diff` (plan T2.9).

mod common;

use common::{run_bin, write_files};
use std::path::{Path, PathBuf};

const ROSTER4: &str = r#"
[[character]]
id = "mina"
name = "Mina"
[[character]]
id = "narae"
name = "Narae"
[[character]]
id = "chloe"
name = "Chloe"
[[character]]
id = "daisy"
name = "Daisy"
"#;

const PG4: &str = r#"
[[placement]]
character = "mina"
conquest = "S"
rift = "S"
[[placement]]
character = "narae"
conquest = "S"
rift = "A"
[[placement]]
character = "chloe"
conquest = "A"
rift = "A"
[[placement]]
character = "daisy"
conquest = "C"
rift = "C"
"#;

const KAIDEN4: &str = r#"
[[placement]]
character = "mina"
low = { tier = "T2", raw = "T2" }
high = { tier = "T0.5", raw = "T0.5" }
max = { tier = "T0.5", raw = "T0.5" }
[[placement]]
character = "narae"
low = { tier = "T0", raw = "T0" }
high = { tier = "T0", raw = "T0" }
max = { tier = "T0", raw = "T0" }
[[placement]]
character = "daisy"
low = { tier = "T4", raw = "T4" }
high = { tier = "T4", raw = "T4" }
max = { tier = "T4", raw = "T4" }
"#;

const JP3: &str = r#"
[[placement]]
character = "mina"
tier = "SS"
raw = "SS"
[[placement]]
character = "narae"
tier = "SS"
raw = "SS"
[[placement]]
character = "daisy"
tier = "A"
raw = "A"
"#;

fn dataset4(name: &str) -> PathBuf {
    let dir = common::temp_dir(name);
    write_files(
        &dir,
        &[
            ("roster.toml", ROSTER4),
            ("sources.toml", common::SOURCES),
            ("method.toml", common::METHOD),
            ("placements/pg.toml", PG4),
            ("placements/kaiden.toml", KAIDEN4),
            ("placements/gamewith-jp.toml", JP3),
        ],
    );
    dir
}

fn snapshot(dir: &Path, id: &str) -> common::BinOutput {
    run_bin(
        &["snapshot", "--data", dir.to_str().unwrap(), "--id", id],
        None,
    )
}

/// Re-running `snapshot` on unchanged inputs writes nothing and exits 3.
#[test]
fn snapshot_unchanged_exits_3() {
    let dir = dataset4("unchanged");
    let first = snapshot(&dir, "2026-11-03-am");
    assert_eq!(first.code, Some(0), "output: {}", first.combined());
    let snap_path = dir.join("snapshots/2026-11-03-am.json");
    assert!(snap_path.is_file());

    let second = snapshot(&dir, "2026-11-03-pm");
    assert_eq!(second.code, Some(3), "output: {}", second.combined());
    assert!(second.stdout.contains("unchanged"));
    assert!(!dir.join("snapshots/2026-11-03-pm.json").exists());
}

/// A snapshot's frozen results equal a fresh computation on the same
/// data, character for character, level for level, boards included.
#[test]
fn snapshot_roundtrip_renders_identical_results() {
    let dir = dataset4("roundtrip");
    let out = snapshot(&dir, "2026-11-03-am");
    assert_eq!(out.code, Some(0), "output: {}", out.combined());

    let text = std::fs::read_to_string(dir.join("snapshots/2026-11-03-am.json")).unwrap();
    let doc: mongil_core::snapshot::SnapshotDoc = serde_json::from_str(&text).unwrap();
    assert_eq!(doc.id, "2026-11-03-am");
    assert_eq!(doc.contract_version, 1);
    assert_eq!(doc.inputs_sha256.len(), 64);

    let ds = mongil_core::load_dataset(&dir).unwrap();
    for level in mongil_core::model::Level::ALL {
        let fresh = mongil_core::method::compute(&ds, &ds.method, level);
        // Boards.
        let frozen_boards = &doc.results.boards[level.as_str()];
        for (band, ids) in &fresh.boards {
            assert_eq!(
                frozen_boards.get(band.as_str()),
                Some(ids),
                "board {} at {}",
                band.as_str(),
                level.as_str()
            );
        }
        // Per-character results.
        for c in &fresh.characters {
            let frozen = &doc.results.characters[&c.id][level.as_str()];
            assert_eq!(frozen.tier, c.tier.as_str(), "{} tier", c.id);
            assert_eq!(frozen.label, c.label.as_str(), "{} label", c.id);
            assert_eq!(frozen.coverage, c.coverage, "{} coverage", c.id);
            match (frozen.score, c.score) {
                (Some(a), Some(b)) => {
                    assert!((a - (b * 10_000.0).round() / 10_000.0).abs() < 1e-9)
                }
                (None, None) => {}
                other => panic!("score mismatch for {}: {other:?}", c.id),
            }
        }
    }
}

/// A diff reports tier, label, coverage and placement changes.
#[test]
fn diff_reports_tier_label_coverage_and_placement_changes() {
    let dir = dataset4("diff");
    let out = snapshot(&dir, "2026-11-03-am");
    assert_eq!(out.code, Some(0), "output: {}", out.combined());

    // v2 changes:
    // - mina: Kaiden Low T2 → T4 (placement change; Low score 3.33 → 2.67, tier A → B)
    // - narae: JP SS → A (placement change; score 3.83 → 3.17, tier S → A,
    //   spread 0.5 → 2.0, label Solid → Contested)
    // - chloe: gains a JP placement (placement change; coverage 1 → 2,
    //   label Speculation → Solid)
    let kaiden_v2 = KAIDEN4.replace(
        "character = \"mina\"\nlow = { tier = \"T2\", raw = \"T2\" }",
        "character = \"mina\"\nlow = { tier = \"T4\", raw = \"T4\" }",
    );
    assert_ne!(kaiden_v2, KAIDEN4);
    let jp_v2 = format!(
        "{}\n[[placement]]\ncharacter = \"chloe\"\ntier = \"S\"\nraw = \"S\"\n",
        JP3.replace(
            "character = \"narae\"\ntier = \"SS\"",
            "character = \"narae\"\ntier = \"A\""
        )
    );
    write_files(
        &dir,
        &[
            ("placements/kaiden.toml", &kaiden_v2),
            ("placements/gamewith-jp.toml", &jp_v2),
        ],
    );
    let out = snapshot(&dir, "2026-11-03-pm");
    assert_eq!(out.code, Some(0), "output: {}", out.combined());

    let out = run_bin(
        &[
            "diff",
            "--data",
            dir.to_str().unwrap(),
            "--from",
            "2026-11-03-am",
            "--to",
            "2026-11-03-pm",
            "--format",
            "json",
        ],
        None,
    );
    assert_eq!(out.code, Some(0), "output: {}", out.combined());
    let diff: serde_json::Value = serde_json::from_str(&out.stdout).unwrap();

    let tier_changes = diff["tier_changes"].as_array().unwrap();
    assert!(
        tier_changes.iter().any(|c| c["character"] == "mina"
            && c["level"] == "low"
            && c["from"] == "A"
            && c["to"] == "B"),
        "tier changes: {tier_changes:?}"
    );
    assert!(
        tier_changes
            .iter()
            .any(|c| c["character"] == "narae" && c["from"] == "S" && c["to"] == "A"),
        "tier changes: {tier_changes:?}"
    );

    let label_changes = diff["label_changes"].as_array().unwrap();
    assert!(
        label_changes
            .iter()
            .any(|c| c["character"] == "narae" && c["from"] == "SOLID" && c["to"] == "CONTESTED"),
        "label changes: {label_changes:?}"
    );
    assert!(
        label_changes
            .iter()
            .any(|c| c["character"] == "chloe" && c["from"] == "SPECULATION" && c["to"] == "SOLID"),
        "label changes: {label_changes:?}"
    );

    let coverage_changes = diff["coverage_changes"].as_array().unwrap();
    assert!(
        coverage_changes
            .iter()
            .any(|c| c["character"] == "chloe" && c["from"] == 1 && c["to"] == 2),
        "coverage changes: {coverage_changes:?}"
    );

    let placement_changes = diff["placement_changes"].as_array().unwrap();
    assert!(
        placement_changes.iter().any(|c| c["source"] == "kaiden"
            && c["character"] == "mina"
            && c["field"] == "low.tier"),
        "placement changes: {placement_changes:?}"
    );
    assert!(
        placement_changes
            .iter()
            .any(|c| c["source"] == "gamewith-jp" && c["character"] == "chloe"),
        "placement changes: {placement_changes:?}"
    );

    // The Markdown form renders the same story.
    let out = run_bin(
        &[
            "diff",
            "--data",
            dir.to_str().unwrap(),
            "--from",
            "2026-11-03-am",
            "--to",
            "2026-11-03-pm",
        ],
        None,
    );
    assert_eq!(out.code, Some(0), "output: {}", out.combined());
    assert!(out.stdout.contains("# Diff: 2026-11-03-am → 2026-11-03-pm"));
    assert!(out.stdout.contains("## Tier changes"));
    assert!(out.stdout.contains("## List-count changes"));
    assert!(out.stdout.contains("## Placement changes"));
}
