//! `mongil-build snapshot` and `mongil-build diff` (plan T2.9).
//!
//! `snapshot` writes `data/snapshots/<sweep-id>.json` **only** when the
//! frozen inputs changed versus the latest existing snapshot; when
//! nothing changed it writes nothing and exits **3** (the sweep
//! playbook records "no change" and moves on). A snapshot id that
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
/// what the harvest directories are named.
pub fn valid_sweep_id(id: &str) -> bool {
    let bytes = id.as_bytes();
    if bytes.len() < 13 {
        return false;
    }
    let date = &id[..10];
    let date_ok = date.chars().enumerate().all(|(i, c)| {
        if i == 4 || i == 7 {
            c == '-'
        } else {
            c.is_ascii_digit()
        }
    });
    if !date_ok {
        return false;
    }
    // Timestamped form: `YYYY-MM-DDTHHMM-sweep-am|pm`.
    if bytes[10] == b'T' {
        return id.len() == 24
            && id[11..15].chars().all(|c| c.is_ascii_digit())
            && &id[15..22] == "-sweep-"
            && (&id[22..] == "am" || &id[22..] == "pm");
    }
    if bytes[10] != b'-' {
        return false;
    }
    let suffix = &id[11..];
    if suffix == "am" || suffix == "pm" {
        return true;
    }
    for prefix in ["patch", "x"] {
        if let Some(rest) = suffix.strip_prefix(prefix) {
            return !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit());
        }
    }
    false
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

/// All snapshots on disk, oldest id first (ids sort chronologically).
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
    out.sort_by(|a, b| a.0.cmp(&b.0));
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
        if latest.inputs_sha256 == doc.inputs_sha256 {
            println!("snapshot: inputs unchanged since {latest_id} — nothing written");
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
