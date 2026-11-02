//! The change lane (plan T4.6; path core landed with the T2.10 gates).
//!
//! `mongil-build lane --base origin/main` classifies the change set
//! against a base ref. The first output line is `data-only` — the
//! only class the auto-merge script may merge (gate G13) — only when
//! **all** of the plan's conditions hold:
//!
//! 1. every changed path is on the data-only allowlist;
//! 2. `data/roster.toml` changes are additions only (no character is
//!    edited or removed);
//! 3. `data/snapshots/` changes are new files only (a committed
//!    snapshot is never modified or deleted);
//! 4. changed `data/notes/*.md` files all carry `status: auto`
//!    frontmatter (a drafted note waits for Tom);
//! 5. the accuracy tripwire (T4.5) is clear;
//! 6. auto-deploy is not paused in `data/ops/autodeploy.toml`;
//! 7. fewer than 9 characters change Overall tier versus the newest
//!    snapshot at the base ref (the G5 mass-change rule).
//!
//! Anything else prints `needs-tom` with one line per failed
//! condition. The conditions fail closed: a check that cannot run
//! (unreadable snapshot, unparsable roster) is a failed condition,
//! except where there is plainly nothing to check (no snapshots at
//! the base ref yet, no errata file yet).

use std::path::Path;
use std::process::ExitCode;

/// Data-only allowlist (plan T4.6). Anything outside these paths is a
/// code/design/method change and waits for Tom.
pub fn is_data_path(path: &str) -> bool {
    const EXACT: [&str; 3] = ["data/codes.toml", "data/banners.toml", "data/roster.toml"];
    const PREFIXES: [&str; 5] = [
        "data/placements/",
        "data/harvests/",
        "data/state/",
        "data/snapshots/",
        "data/notes/",
    ];
    EXACT.contains(&path) || PREFIXES.iter().any(|p| path.starts_with(p))
}

fn git(args: &[&str]) -> Result<String, String> {
    let out = std::process::Command::new("git")
        .args(args)
        .output()
        .map_err(|e| format!("cannot run git: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

/// One changed path with its git status letter (`A`, `M`, `D`, `R`,
/// …; `?` for untracked files).
#[derive(Clone, Debug, PartialEq)]
pub struct Change {
    pub status: String,
    pub path: String,
}

/// Changed paths versus `base`, from git, with statuses. Includes
/// untracked files (a sweep that adds a harvest file must not slip
/// past the lane).
pub fn changed_entries(base: &str) -> Result<Vec<Change>, String> {
    let diff = git(&["diff", "--name-status", base])?;
    let mut entries: Vec<Change> = Vec::new();
    for line in diff.lines() {
        let mut parts = line.split('\t');
        let Some(status) = parts.next() else { continue };
        let path = parts.next_back().unwrap_or_default().to_string();
        if !path.is_empty() {
            entries.push(Change {
                status: status.chars().next().unwrap_or('?').to_string(),
                path,
            });
        }
    }
    let untracked = git(&["ls-files", "--others", "--exclude-standard"])?;
    for line in untracked.lines() {
        if !line.is_empty() {
            entries.push(Change {
                status: "?".to_string(),
                path: line.to_string(),
            });
        }
    }
    entries.sort_by(|a, b| a.path.cmp(&b.path));
    entries.dedup_by(|a, b| a.path == b.path);
    Ok(entries)
}

/// The `status:` value in a note file's frontmatter, if present.
fn note_status(text: &str) -> Option<String> {
    let mut in_frontmatter = false;
    for line in text.lines() {
        if line.trim() == "---" {
            if in_frontmatter {
                break;
            }
            in_frontmatter = true;
            continue;
        }
        if in_frontmatter && line.starts_with("status:") {
            return Some(line["status:".len()..].trim().to_string());
        }
    }
    None
}

/// Condition 2: roster changes are additions only — every character
/// at the base ref is still present, byte-equal as a parsed entry.
fn roster_additions_only(base: &str) -> Result<bool, String> {
    let base_text = match git(&["show", &format!("{base}:data/roster.toml")]) {
        Ok(text) => text,
        // No roster at the base ref: the file itself is an addition.
        Err(_) => return Ok(true),
    };
    let base_roster: mongil_core::model::Roster =
        toml::from_str(&base_text).map_err(|e| format!("base roster does not parse: {e}"))?;
    let working_text = std::fs::read_to_string("data/roster.toml")
        .map_err(|e| format!("cannot read data/roster.toml: {e}"))?;
    let working_roster: mongil_core::model::Roster =
        toml::from_str(&working_text).map_err(|e| format!("working roster does not parse: {e}"))?;
    Ok(base_roster.characters.iter().all(|base_char| {
        working_roster
            .characters
            .iter()
            .any(|c| c.id == base_char.id && c == base_char)
    }))
}

/// Condition 7 (G5): fewer than 9 characters change Overall tier
/// versus the newest snapshot committed at the base ref.
fn mass_change_ok(base: &str, data_dir: &Path) -> Result<bool, String> {
    let listing = match git(&["ls-tree", "--name-only", base, "data/snapshots/"]) {
        Ok(text) => text,
        Err(_) => return Ok(true),
    };
    let mut ids: Vec<String> = listing
        .lines()
        .filter_map(|p| p.strip_prefix("data/snapshots/"))
        .filter_map(|f| f.strip_suffix(".json"))
        .map(str::to_string)
        .collect();
    ids.sort_by_cached_key(|id| crate::snapshots::sweep_order_key(id));
    let Some(latest) = ids.last() else {
        // No snapshot at the base ref yet: nothing to measure against.
        return Ok(true);
    };
    let text = git(&["show", &format!("{base}:data/snapshots/{latest}.json")])?;
    let base_doc: mongil_core::snapshot::SnapshotDoc =
        serde_json::from_str(&text).map_err(|e| format!("base snapshot does not parse: {e}"))?;
    let ds = mongil_core::load_dataset(data_dir)
        .map_err(|e| format!("dataset does not load for the G5 check: {e}"))?;
    let working = mongil_core::snapshot::build_snapshot(&ds, "working", "");
    let diff = mongil_core::snapshot::diff_docs(&base_doc, &working);
    Ok(!diff.exceeds_mass_change())
}

/// The full classifier: returns the list of failed conditions (empty
/// = `data-only`).
pub fn classify_change_set(base: &str, data_dir: &Path) -> Result<Vec<String>, String> {
    let entries = changed_entries(base)?;
    let mut failures: Vec<String> = Vec::new();

    // 1. Path allowlist.
    for entry in &entries {
        if !is_data_path(&entry.path) {
            failures.push(format!("outside the data-only allowlist: {}", entry.path));
        }
    }

    // 2. Roster additions only.
    if entries.iter().any(|e| e.path == "data/roster.toml") {
        match roster_additions_only(base) {
            Ok(true) => {}
            Ok(false) => failures.push("data/roster.toml edits or removes a character".into()),
            Err(e) => failures.push(format!("roster check failed: {e}")),
        }
    }

    // 3. Snapshots: new files only.
    for entry in entries
        .iter()
        .filter(|e| e.path.starts_with("data/snapshots/"))
    {
        if entry.status != "A" && entry.status != "?" {
            failures.push(format!(
                "snapshot change is not a new file ({}): {}",
                entry.status, entry.path
            ));
        }
    }

    // 4. Notes: status auto only.
    for entry in entries
        .iter()
        .filter(|e| e.path.starts_with("data/notes/") && e.path.ends_with(".md"))
    {
        if entry.status == "D" {
            failures.push(format!("note deleted: {}", entry.path));
            continue;
        }
        match std::fs::read_to_string(&entry.path) {
            Ok(text) if note_status(&text).as_deref() == Some("auto") => {}
            Ok(_) => failures.push(format!("note is not status auto: {}", entry.path)),
            Err(e) => failures.push(format!("cannot read {}: {e}", entry.path)),
        }
    }

    // 5. Tripwire clear.
    match crate::tripwire::evaluate(data_dir, &crate::tripwire::today_utc()) {
        Ok(outcome) => {
            for reason in outcome.reasons {
                failures.push(format!("tripwire: {reason}"));
            }
        }
        Err(e) => failures.push(format!("tripwire check failed: {e}")),
    }

    // 6. Auto-deploy not paused.
    match crate::tripwire::load_autodeploy(data_dir) {
        Ok(ops) if ops.paused => {
            failures.push(format!(
                "auto-deploy is paused in data/ops/autodeploy.toml ({})",
                ops.pause_reason
            ));
        }
        Ok(_) => {}
        Err(e) => failures.push(format!("autodeploy state unreadable: {e}")),
    }

    // 7. G5 mass change.
    match mass_change_ok(base, data_dir) {
        Ok(true) => {}
        Ok(false) => failures.push(format!(
            "G5 mass change: more than {} characters change Overall tier vs the base snapshot",
            mongil_core::snapshot::MASS_CHANGE_LIMIT
        )),
        Err(e) => failures.push(format!("mass-change check failed: {e}")),
    }

    Ok(failures)
}

pub fn run(base: &str, data_dir: &Path) -> ExitCode {
    match classify_change_set(base, data_dir) {
        Ok(failures) if failures.is_empty() => {
            println!("data-only");
            ExitCode::SUCCESS
        }
        Ok(failures) => {
            println!("needs-tom");
            for failure in failures {
                println!("  {failure}");
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("lane: {e}");
            ExitCode::from(2)
        }
    }
}
