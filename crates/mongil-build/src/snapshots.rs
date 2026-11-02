//! `mongil-build snapshot` and `mongil-build diff` (plan T2.9).
//!
//! `snapshot` writes `data/snapshots/<sweep-id>.json` **only** when
//! something changed versus the latest existing snapshot — the frozen
//! inputs, the method version, or the results the method produces (so a
//! method-only change is recorded too); when nothing changed it writes
//! nothing and exits **3** (the sweep playbook records "no change" and
//! moves on). "Latest" is by sweep chronology (`sweep_order_key`), not
//! by raw id string. A snapshot id that
//! already exists is never overwritten (gate G9 immutability is
//! enforced by git in CI; the writer refuses on principle too).
//!
//! `diff --from <id> --to <id|working>` prints the change set between
//! two snapshots, or between a snapshot and the current working data,
//! as Markdown (default) or JSON.

use crate::emit::now_iso_utc;
use mongil_core::load::load_dataset;
use mongil_core::snapshot::{build_snapshot, diff_docs, render_md, SnapshotDoc};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// Sweep ids look like `2026-11-03-am`, `2026-11-03-pm`,
/// `2026-11-06-patch1`, `2026-11-03-x2` — or the timestamped form the
/// cadence uses, `2026-10-29T2105-sweep-pm` (date + HHMM + `-sweep-am|pm`),
/// which is what `data/state/checks.json` records as `last_sweep` and
/// what the harvest directories are named. The date must be a real
/// calendar date and HHMM a real time of day.
pub fn valid_sweep_id(id: &str) -> bool {
    parse_sweep_id(id).is_some()
}

/// A parsed sweep id: (date, minute of day, kind rank, counter).
type SweepKey = (String, u32, u8, u64);

/// Parse a sweep id into its chronological sort key, or `None` when it
/// is not a valid id. Within a date: the scheduled sweeps by time of
/// day (bare `am` counts as 00:00 and bare `pm` as 12:00; the
/// timestamped form uses its HHMM), then `patchN`, then `xN`, each in
/// numeric counter order (`patch2` before `patch10`).
fn parse_sweep_id(id: &str) -> Option<SweepKey> {
    // ASCII only, so every byte index below is a char boundary.
    if !id.is_ascii() || id.len() < 13 {
        return None;
    }
    let date = &id[..10];
    let digits = |s: &str| !s.is_empty() && s.chars().all(|c| c.is_ascii_digit());
    let b = date.as_bytes();
    if b[4] != b'-'
        || b[7] != b'-'
        || !digits(&date[..4])
        || !digits(&date[5..7])
        || !digits(&date[8..])
    {
        return None;
    }
    let year: i64 = date[..4].parse().ok()?;
    let month: i64 = date[5..7].parse().ok()?;
    let day: i64 = date[8..].parse().ok()?;
    if !crate::validate::valid_civil_date(year, month, day) {
        return None;
    }
    let key = |minute: u32, rank: u8, counter: u64| Some((date.to_string(), minute, rank, counter));
    // Timestamped form: `YYYY-MM-DDTHHMM-sweep-am|pm`.
    if id.as_bytes()[10] == b'T' {
        if id.len() != 24 || !digits(&id[11..15]) || &id[15..22] != "-sweep-" {
            return None;
        }
        if &id[22..] != "am" && &id[22..] != "pm" {
            return None;
        }
        let hour: u32 = id[11..13].parse().ok()?;
        let minute: u32 = id[13..15].parse().ok()?;
        if hour >= 24 || minute >= 60 {
            return None;
        }
        return key(hour * 60 + minute, 0, 0);
    }
    if id.as_bytes()[10] != b'-' {
        return None;
    }
    match &id[11..] {
        "am" => return key(0, 0, 0),
        "pm" => return key(12 * 60, 0, 0),
        _ => {}
    }
    for (prefix, rank) in [("patch", 1u8), ("x", 2u8)] {
        if let Some(rest) = id[11..].strip_prefix(prefix) {
            if !digits(rest) {
                return None;
            }
            return key(24 * 60, rank, rest.parse().ok()?);
        }
    }
    None
}

/// Chronological order for sweep ids (see `parse_sweep_id`). Ids that
/// do not parse sort first, by raw string, so they never count as the
/// latest snapshot.
pub fn sweep_order_key(id: &str) -> (Option<SweepKey>, String) {
    (parse_sweep_id(id), id.to_string())
}

fn snapshot_path(data_dir: &Path, id: &str) -> PathBuf {
    data_dir.join("snapshots").join(format!("{id}.json"))
}

pub(crate) fn read_snapshot(data_dir: &Path, id: &str) -> Result<SnapshotDoc, String> {
    let path = snapshot_path(data_dir, id);
    let text = std::fs::read_to_string(&path)
        .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("cannot parse {}: {e}", path.display()))
}

/// All snapshots on disk, oldest first by sweep chronology.
pub fn list_snapshots(data_dir: &Path) -> Vec<(String, SnapshotDoc)> {
    let dir = data_dir.join("snapshots");
    let mut out = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let Some(id) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            if let Ok(text) = std::fs::read_to_string(&path) {
                if let Ok(doc) = serde_json::from_str::<SnapshotDoc>(&text) {
                    out.push((id.to_string(), doc));
                }
            }
        }
    }
    out.sort_by_cached_key(|(id, _)| sweep_order_key(id));
    out
}

pub fn run_snapshot(data_dir: &Path, id: Option<&str>) -> ExitCode {
    let ds = match load_dataset(data_dir) {
        Ok(ds) => ds,
        Err(e) => {
            eprintln!("snapshot: dataset does not load: {e}");
            return ExitCode::FAILURE;
        }
    };
    let id = match id {
        Some(id) => id.to_string(),
        None => match &ds.checks {
            Some(checks) if !checks.last_sweep.is_empty() => checks.last_sweep.clone(),
            _ => {
                eprintln!("snapshot: no --id given and data/state/checks.json has no last_sweep");
                return ExitCode::from(2);
            }
        },
    };
    if !valid_sweep_id(&id) {
        eprintln!("snapshot: id `{id}` is not a sweep id (YYYY-MM-DD-am|pm|patchN|xN)");
        return ExitCode::from(2);
    }

    let doc = build_snapshot(&ds, &id, &now_iso_utc());

    if let Some((latest_id, latest)) = list_snapshots(data_dir).into_iter().last() {
        // The inputs fingerprint does not cover the method file, so the
        // method version and the computed results are compared too: a
        // method change alone still gets its own snapshot.
        if latest.inputs_sha256 == doc.inputs_sha256
            && latest.method_version == doc.method_version
            && latest.results == doc.results
        {
            println!(
                "snapshot: inputs, method and results unchanged since {latest_id} — nothing written"
            );
            return ExitCode::from(3);
        }
    }
    let path = snapshot_path(data_dir, &id);
    if path.exists() {
        eprintln!(
            "snapshot: {} already exists and snapshots are immutable — refusing to overwrite",
            path.display()
        );
        return ExitCode::FAILURE;
    }
    if let Some(parent) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            eprintln!("snapshot: cannot create {}: {e}", parent.display());
            return ExitCode::FAILURE;
        }
    }
    let mut text = match serde_json::to_string_pretty(&doc) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("snapshot: cannot serialize: {e}");
            return ExitCode::FAILURE;
        }
    };
    text.push('\n');
    match std::fs::write(&path, text) {
        Ok(()) => {
            println!(
                "snapshot: wrote {} (inputs_sha256 {}…)",
                path.display(),
                &doc.inputs_sha256[..12]
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("snapshot: cannot write {}: {e}", path.display());
            ExitCode::FAILURE
        }
    }
}

pub fn run_diff(data_dir: &Path, from: &str, to: &str, format: &str) -> ExitCode {
    let from_doc = match read_snapshot(data_dir, from) {
        Ok(doc) => doc,
        Err(e) => {
            eprintln!("diff: {e}");
            return ExitCode::FAILURE;
        }
    };
    let to_doc = if to == "working" {
        let ds = match load_dataset(data_dir) {
            Ok(ds) => ds,
            Err(e) => {
                eprintln!("diff: dataset does not load: {e}");
                return ExitCode::FAILURE;
            }
        };
        build_snapshot(&ds, "working", &now_iso_utc())
    } else {
        match read_snapshot(data_dir, to) {
            Ok(doc) => doc,
            Err(e) => {
                eprintln!("diff: {e}");
                return ExitCode::FAILURE;
            }
        }
    };
    let diff = diff_docs(&from_doc, &to_doc);
    match format {
        "json" => match serde_json::to_string_pretty(&diff) {
            Ok(text) => println!("{text}"),
            Err(e) => {
                eprintln!("diff: cannot serialize: {e}");
                return ExitCode::FAILURE;
            }
        },
        _ => print!("{}", render_md(&diff)),
    }
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::valid_sweep_id;

    #[test]
    fn sweep_id_forms() {
        for ok in [
            "2026-11-03-am",
            "2026-11-03-pm",
            "2026-11-06-patch1",
            "2026-11-03-x2",
            "2026-10-29T2105-sweep-pm",
            "2026-10-29T0900-sweep-am",
        ] {
            assert!(valid_sweep_id(ok), "{ok} should be a valid sweep id");
        }
        for bad in [
            "2026-10-29",
            "2026-10-29-sweep-pm",
            "2026-10-29T2105-pm",
            "2026-10-29T2105-sweep-xx",
            "2026-1-03-am",
            "working",
            "",
        ] {
            assert!(
                !valid_sweep_id(bad),
                "{bad:?} should not be a valid sweep id"
            );
        }
    }
}
