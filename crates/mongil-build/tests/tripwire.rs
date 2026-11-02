//! T4.5 acceptance: the accuracy tripwire (D-11). Each test pins
//! "today" with `--today` so the 7-day and month-one windows are
//! deterministic.

mod common;

use common::{run_bin, temp_dir, write_files};
use std::path::{Path, PathBuf};

fn dataset(name: &str, errata: &str, autodeploy: Option<&str>) -> PathBuf {
    let dir = temp_dir(name);
    let mut files = vec![("errata.toml", errata)];
    if let Some(ops) = autodeploy {
        files.push(("ops/autodeploy.toml", ops));
    }
    write_files(&dir, &files);
    dir
}

fn tripwire(dir: &Path, today: &str) -> common::BinOutput {
    run_bin(
        &[
            "tripwire",
            "--data",
            dir.to_str().unwrap(),
            "--today",
            today,
        ],
        None,
    )
}

/// Plan-named test: a major erratum inside the last 7 days without a
/// completed review trips the wire (exit 1).
#[test]
fn tripwire_major_blocks() {
    let dir = dataset(
        "tw-major",
        r#"
[[erratum]]
id = "e1"
found_at = "2026-11-18"
severity = "major"
what = "Isabella was shown in the wrong tier."
characters = ["isabella"]
snapshots = []
review_done = false
"#,
        None,
    );
    let out = tripwire(&dir, "2026-11-20");
    assert_eq!(out.code, Some(1), "output: {}", out.combined());
    assert!(out.stdout.contains("TRIPPED"), "output: {}", out.combined());
}

/// Plan-named test: three errata of any severity inside the 30 days
/// after launch trip the wire, even when all are reviewed.
#[test]
fn tripwire_three_in_month_one_blocks() {
    let dir = dataset(
        "tw-month-one",
        r#"
[[erratum]]
id = "e1"
found_at = "2026-11-03"
severity = "minor"
what = "Typo in a label."
characters = []
snapshots = []
review_done = true
[[erratum]]
id = "e2"
found_at = "2026-11-12"
severity = "minor"
what = "Wrong list date."
characters = []
snapshots = []
review_done = true
[[erratum]]
id = "e3"
found_at = "2026-11-29"
severity = "minor"
what = "Stale code shown."
characters = []
snapshots = []
review_done = true
"#,
        Some("paused = false\nlaunch_date = \"2026-11-01\"\n"),
    );
    let out = tripwire(&dir, "2026-12-15");
    assert_eq!(out.code, Some(1), "output: {}", out.combined());
    assert!(out.stdout.contains("TRIPPED"), "output: {}", out.combined());
}

/// Plan-named test: a reviewed old major and an erratum outside the
/// month-one window leave the wire clear (exit 0).
#[test]
fn tripwire_clear_passes() {
    let dir = dataset(
        "tw-clear",
        r#"
[[erratum]]
id = "e1"
found_at = "2026-08-15"
severity = "major"
what = "A major error, reviewed and fixed long ago."
characters = []
snapshots = []
review_done = true
[[erratum]]
id = "e2"
found_at = "2026-08-20"
severity = "minor"
what = "Before launch; outside any month-one window."
characters = []
snapshots = []
review_done = true
"#,
        Some("paused = false\nlaunch_date = \"2026-11-01\"\n"),
    );
    let out = tripwire(&dir, "2026-11-20");
    assert_eq!(out.code, Some(0), "output: {}", out.combined());
    assert!(out.stdout.contains("clear"), "output: {}", out.combined());
}

/// A missing errata file (the pre-launch state) is clear, not an
/// error.
#[test]
fn tripwire_no_errata_file_is_clear() {
    let dir = temp_dir("tw-empty");
    let out = tripwire(&dir, "2026-11-20");
    assert_eq!(out.code, Some(0), "output: {}", out.combined());
}

/// An erratum date that does not parse is an error (exit 2), never a
/// silent "clear" (bug hunt #21).
#[test]
fn tripwire_unparseable_found_at_errors() {
    for bad in ["garbage!!", "2026-02-30", "2026-13-01"] {
        let errata = format!(
            r#"
[[erratum]]
id = "e1"
found_at = "{bad}"
severity = "major"
what = "Probe."
characters = []
snapshots = []
review_done = false
"#
        );
        let dir = dataset("tw-bad-date", &errata, None);
        let out = tripwire(&dir, "2026-03-01");
        assert_eq!(out.code, Some(2), "found_at {bad}: {}", out.combined());
        assert!(
            out.combined().contains("not a real YYYY-MM-DD date"),
            "found_at {bad}: {}",
            out.combined()
        );
    }
}

/// An unparseable launch_date is an error too, not a disabled rule.
#[test]
fn tripwire_unparseable_launch_date_errors() {
    let dir = dataset(
        "tw-bad-launch",
        "",
        Some("paused = false\npause_reason = \"\"\nlaunch_date = \"soon\"\n"),
    );
    let out = tripwire(&dir, "2026-11-20");
    assert_eq!(out.code, Some(2), "output: {}", out.combined());
    assert!(
        out.combined().contains("launch_date"),
        "output: {}",
        out.combined()
    );
}
