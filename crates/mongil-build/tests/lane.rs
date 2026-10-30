//! T4.6 acceptance: the full lane classifier — one test per lane
//! condition, each flipping the verdict. Repos are real temp git
//! repos; the classifier runs as the built binary, like CI will.

mod common;

use common::{git, git_init_repo, run_bin, temp_dir, write_dataset_vec, write_files};
use std::path::{Path, PathBuf};

/// A repo whose `data/` holds the given dataset files, committed.
fn repo_with_dataset(name: &str, extra: &[(&str, &str)]) -> PathBuf {
    let dir = temp_dir(name);
    git_init_repo(&dir);
    let mut files: Vec<(String, String)> = common::base_files()
        .into_iter()
        .map(|(rel, content)| (format!("data/{rel}"), content))
        .collect();
    for (rel, content) in extra {
        files.push((rel.to_string(), content.to_string()));
    }
    let refs: Vec<(&str, &str)> = files
        .iter()
        .map(|(a, b)| (a.as_str(), b.as_str()))
        .collect();
    write_files(&dir, &refs);
    git(&dir, &["add", "-A"]);
    git(&dir, &["commit", "-qm", "baseline"]);
    dir
}

fn lane(dir: &Path) -> common::BinOutput {
    run_bin(&["lane", "--base", "HEAD", "--data", "data"], Some(dir))
}

fn first_line(out: &common::BinOutput) -> Option<&str> {
    out.stdout.lines().next()
}

/// The happy path: a placements-only change with a clear tripwire
/// stays in the auto lane.
#[test]
fn lane_data_only_passes() {
    let dir = repo_with_dataset("lane-ok", &[]);
    write_files(
        &dir,
        &[(
            "data/placements/pg.toml",
            &format!("{}\n# touched\n", common::PG),
        )],
    );
    let out = lane(&dir);
    assert_eq!(out.code, Some(0), "output: {}", out.combined());
    assert_eq!(
        first_line(&out),
        Some("data-only"),
        "output: {}",
        out.combined()
    );
}

/// Condition 1: a code path in the change set needs Tom.
#[test]
fn lane_code_path_needs_tom() {
    let dir = repo_with_dataset("lane-code", &[]);
    write_files(&dir, &[("crates/mongil-build/src/lib.rs", "// code\n")]);
    let out = lane(&dir);
    assert_eq!(
        first_line(&out),
        Some("needs-tom"),
        "output: {}",
        out.combined()
    );
}

/// Condition 2: editing an existing roster character needs Tom
/// (additions only are allowed in the auto lane).
#[test]
fn lane_roster_edit_needs_tom() {
    let dir = repo_with_dataset("lane-roster", &[]);
    let edited = common::ROSTER.replace("name = \"Mina\"", "name = \"Mina Renamed\"");
    write_files(&dir, &[("data/roster.toml", &edited)]);
    let out = lane(&dir);
    assert_eq!(
        first_line(&out),
        Some("needs-tom"),
        "output: {}",
        out.combined()
    );
}

/// Condition 3: modifying a committed snapshot needs Tom (snapshots
/// enter the repo as new files only).
#[test]
fn lane_snapshot_edit_needs_tom() {
    let dir = repo_with_dataset(
        "lane-snap",
        &[("data/snapshots/2026-01-01-base.json", "{}\n")],
    );
    write_files(
        &dir,
        &[("data/snapshots/2026-01-01-base.json", "{\"edited\":true}\n")],
    );
    let out = lane(&dir);
    assert_eq!(
        first_line(&out),
        Some("needs-tom"),
        "output: {}",
        out.combined()
    );
}

/// Condition 4: a note that is not `status: auto` needs Tom.
#[test]
fn lane_draft_note_needs_tom() {
    let dir = repo_with_dataset("lane-note", &[]);
    write_files(
        &dir,
        &[(
            "data/notes/2026-11-10.md",
            "---\nfrom: a\nto: b\nstatus: draft\n---\n\n# What moved\n",
        )],
    );
    let out = lane(&dir);
    assert_eq!(
        first_line(&out),
        Some("needs-tom"),
        "output: {}",
        out.combined()
    );
}

/// Condition 5: a tripped tripwire needs Tom. Tripped via the
/// month-one rule so the test does not depend on the run date.
#[test]
fn lane_tripwire_needs_tom() {
    let errata = r#"
[[erratum]]
id = "e1"
found_at = "2026-01-05"
severity = "minor"
what = "One."
characters = []
snapshots = []
review_done = true
[[erratum]]
id = "e2"
found_at = "2026-01-10"
severity = "minor"
what = "Two."
characters = []
snapshots = []
review_done = true
[[erratum]]
id = "e3"
found_at = "2026-01-15"
severity = "minor"
what = "Three."
characters = []
snapshots = []
review_done = true
"#;
    let dir = repo_with_dataset(
        "lane-tripwire",
        &[
            ("data/errata.toml", errata),
            (
                "data/ops/autodeploy.toml",
                "paused = false\nlaunch_date = \"2026-01-01\"\n",
            ),
        ],
    );
    write_files(
        &dir,
        &[(
            "data/placements/pg.toml",
            &format!("{}\n# touched\n", common::PG),
        )],
    );
    let out = lane(&dir);
    assert_eq!(
        first_line(&out),
        Some("needs-tom"),
        "output: {}",
        out.combined()
    );
    assert!(
        out.stdout.contains("tripwire"),
        "output: {}",
        out.combined()
    );
}

/// Condition 6: a paused auto-deploy needs Tom for everything.
#[test]
fn lane_paused_needs_tom() {
    let dir = repo_with_dataset(
        "lane-paused",
        &[(
            "data/ops/autodeploy.toml",
            "paused = true\npause_reason = \"testing\"\n",
        )],
    );
    write_files(
        &dir,
        &[(
            "data/placements/pg.toml",
            &format!("{}\n# touched\n", common::PG),
        )],
    );
    let out = lane(&dir);
    assert_eq!(
        first_line(&out),
        Some("needs-tom"),
        "output: {}",
        out.combined()
    );
}

/// Condition 7 (G5): more than 8 characters changing Overall tier
/// versus the base snapshot needs Tom.
#[test]
fn lane_mass_change_needs_tom() {
    // A 12-character dataset where Pocket Gamer is the only list
    // placing anyone: flipping every placement S → C moves all 12.
    let mut roster = String::new();
    let mut pg = String::new();
    for i in 1..=12 {
        roster.push_str(&format!(
            "[[character]]\nid = \"c{i:02}\"\nname = \"Char {i:02}\"\n"
        ));
        pg.push_str(&format!(
            "[[placement]]\ncharacter = \"c{i:02}\"\nconquest = \"S\"\nrift = \"S\"\n"
        ));
    }
    let data_dir = write_dataset_vec(
        "lane-mass-data",
        &[
            ("roster.toml".into(), roster),
            ("sources.toml".into(), common::SOURCES.into()),
            ("method.toml".into(), common::METHOD.into()),
            ("placements/pg.toml".into(), pg.clone()),
            // The loader requires a file per voting source; these
            // place one character each, at values that do not change
            // the flip math (c01 still changes tier when PG flips).
            (
                "placements/kaiden.toml".into(),
                "[[placement]]\ncharacter = \"c01\"\nlow = { tier = \"T0\", raw = \"T0\" }\nhigh = { tier = \"T0\", raw = \"T0\" }\nmax = { tier = \"T0\", raw = \"T0\" }\n".into(),
            ),
            (
                "placements/gamewith-jp.toml".into(),
                "[[placement]]\ncharacter = \"c01\"\ntier = \"SS\"\nraw = \"SS\"\n".into(),
            ),
        ],
    );
    let repo = temp_dir("lane-mass");
    git_init_repo(&repo);
    // Place the dataset at data/ inside the repo.
    let out = std::process::Command::new("cp")
        .args([
            "-r",
            data_dir.to_str().unwrap(),
            repo.join("data").to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(out.status.success());
    let snap = run_bin(
        &[
            "snapshot",
            "--data",
            repo.join("data").to_str().unwrap(),
            "--id",
            "2026-01-01-am",
        ],
        None,
    );
    assert_eq!(snap.code, Some(0), "snapshot: {}", snap.combined());
    git(&repo, &["add", "-A"]);
    git(&repo, &["commit", "-qm", "baseline"]);

    let flipped = pg.replace("\"S\"", "\"C\"");
    write_files(&repo, &[("data/placements/pg.toml", &flipped)]);
    let out = lane(&repo);
    assert_eq!(
        first_line(&out),
        Some("needs-tom"),
        "output: {}",
        out.combined()
    );
    assert!(out.stdout.contains("G5"), "output: {}", out.combined());
}
