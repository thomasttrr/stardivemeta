//! The change lane (plan T4.6, core landed with the T2.10 gates).
//!
//! `mongil-build lane --base origin/main` classifies the paths changed
//! against a base ref: if every changed path is on the data-only
//! allowlist, the first output line is `data-only` (the only class the
//! auto-merge script may merge, gate G13); anything else prints
//! `needs-tom`.
//!
//! This is the path-classification core only. The plan's other lane
//! conditions land with their tasks and are printed as NOTEs so the
//! deferral is visible: the mass-change rule reads the sweep diff
//! (classification wired in `validate` as gate G5's note), tripwire
//! state is T4.5, and the "note files only with status `auto`" content
//! condition is T4.6.

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

/// True when every changed path is data-only.
pub fn classify(paths: &[String]) -> bool {
    paths.iter().all(|p| is_data_path(p))
}

/// Changed paths versus `base`, from git. Includes untracked files
/// (a sweep that adds a harvest file must not slip past the lane).
pub fn changed_paths(base: &str) -> Result<Vec<String>, String> {
    let diff = std::process::Command::new("git")
        .args(["diff", "--name-only", base])
        .output()
        .map_err(|e| format!("cannot run git: {e}"))?;
    if !diff.status.success() {
        return Err(format!(
            "git diff --name-only {base} failed: {}",
            String::from_utf8_lossy(&diff.stderr).trim()
        ));
    }
    let untracked = std::process::Command::new("git")
        .args(["ls-files", "--others", "--exclude-standard"])
        .output()
        .map_err(|e| format!("cannot run git: {e}"))?;
    let mut paths: Vec<String> = String::from_utf8_lossy(&diff.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect();
    paths.extend(
        String::from_utf8_lossy(&untracked.stdout)
            .lines()
            .map(|s| s.to_string()),
    );
    paths.sort();
    paths.dedup();
    Ok(paths)
}

pub fn run(base: &str) -> ExitCode {
    let paths = match changed_paths(base) {
        Ok(paths) => paths,
        Err(e) => {
            eprintln!("lane: {e}");
            return ExitCode::from(2);
        }
    };
    if classify(&paths) {
        println!("data-only");
    } else {
        println!("needs-tom");
        for path in &paths {
            if !is_data_path(path) {
                println!("  outside the data-only allowlist: {path}");
            }
        }
    }
    println!(
        "NOTE: lane core = path classification only (T2.10). The remaining lane conditions land later: tripwire state (T4.5), note-status and full auto-merge conditions (T4.6); the >8-changes mass-change rule is reported by `validate` (gate G5)."
    );
    ExitCode::SUCCESS
}
