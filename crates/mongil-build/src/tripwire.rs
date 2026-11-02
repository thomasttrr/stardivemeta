//! `mongil-build tripwire` (plan T4.5, D-11): the accuracy tripwire
//! over `data/errata.toml`. It exits 1 — which closes the auto lane
//! (T4.6) — in either case:
//!
//! - any `severity = "major"` erratum dated within the last 7 days
//!   without `review_done = true` (a major published error forces an
//!   immediate review before anything auto-deploys again);
//! - 3 or more errata of any severity dated within the month-one
//!   window, the 30 days from `launch_date` in
//!   `data/ops/autodeploy.toml` (the launch guardrail: three
//!   published errors of any kind in month 1 pause auto-deploy to
//!   Tom-approval). Before launch there is no `launch_date`, so this
//!   rule has no window and cannot trip.
//!
//! Dates compare by UTC day. An erratum's `found_at` may be a bare
//! date or a timestamp; its first 10 chars are the date. A `found_at`
//! or `launch_date` that is not a real date is an error (exit 2), never
//! a silent skip: the lane classifier treats that error as closed.

use mongil_core::model::{Errata, ErratumSeverity};
use serde::Deserialize;
use std::path::Path;
use std::process::ExitCode;

/// `data/ops/autodeploy.toml` (plan §B.3). Every field is optional on
/// disk; a missing file means the defaults (not paused, not launched).
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Autodeploy {
    #[serde(default)]
    pub paused: bool,
    pub launch_date: Option<String>,
    #[serde(default)]
    pub pause_reason: String,
}

pub fn load_autodeploy(data_dir: &Path) -> Result<Autodeploy, String> {
    let path = data_dir.join("ops/autodeploy.toml");
    if !path.exists() {
        return Ok(Autodeploy::default());
    }
    let text = std::fs::read_to_string(&path)
        .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    toml::from_str(&text).map_err(|e| format!("cannot parse {}: {e}", path.display()))
}

fn load_errata(data_dir: &Path) -> Result<Errata, String> {
    let path = data_dir.join("errata.toml");
    if !path.exists() {
        return Ok(Errata { errata: Vec::new() });
    }
    let text = std::fs::read_to_string(&path)
        .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    toml::from_str(&text).map_err(|e| format!("cannot parse {}: {e}", path.display()))
}

/// A date-or-timestamp string as a UTC day ordinal, or `None` when the
/// first 10 chars are not a valid `YYYY-MM-DD` date.
fn day_ordinal(stamp: &str) -> Option<i64> {
    let date = stamp.get(..10)?;
    crate::validate::parse_rfc3339(date).map(|epoch| epoch.div_euclid(86_400))
}

/// The tripwire verdict: the reasons it tripped (empty = clear).
#[derive(Clone, Debug, PartialEq)]
pub struct TripwireOutcome {
    pub reasons: Vec<String>,
}

impl TripwireOutcome {
    pub fn tripped(&self) -> bool {
        !self.reasons.is_empty()
    }
}

/// Evaluate the tripwire over a dataset directory at a given "today"
/// (a `YYYY-MM-DD` date; the CLI passes the real UTC date).
pub fn evaluate(data_dir: &Path, today: &str) -> Result<TripwireOutcome, String> {
    let errata = load_errata(data_dir)?;
    let ops = load_autodeploy(data_dir)?;
    let today_ord =
        day_ordinal(today).ok_or_else(|| format!("today `{today}` is not a YYYY-MM-DD date"))?;

    // Every date must parse before any rule runs: a corrupt record must
    // not read as "clear".
    let mut found: Vec<i64> = Vec::with_capacity(errata.errata.len());
    for e in &errata.errata {
        found.push(day_ordinal(&e.found_at).ok_or_else(|| {
            format!(
                "erratum {} has found_at `{}`, which is not a real YYYY-MM-DD date",
                e.id, e.found_at
            )
        })?);
    }
    let launch = match ops.launch_date.as_deref() {
        Some(date) => Some(day_ordinal(date).ok_or_else(|| {
            format!(
                "launch_date `{date}` in data/ops/autodeploy.toml is not a real YYYY-MM-DD date"
            )
        })?),
        None => None,
    };

    let mut reasons = Vec::new();

    // Rule 1: an unreviewed major erratum from the last 7 days.
    for (e, &found) in errata.errata.iter().zip(&found) {
        if e.severity != ErratumSeverity::Major || e.review_done {
            continue;
        }
        if (0..=7).contains(&(today_ord - found)) {
            reasons.push(format!(
                "major erratum {} dated {} is unreviewed (within the last 7 days)",
                e.id, e.found_at
            ));
        }
    }

    // Rule 2: three or more errata inside the month-one window.
    if let Some(launch) = launch {
        let in_window = found
            .iter()
            .filter(|&&found| found >= launch && found < launch + 30)
            .count();
        if in_window >= 3 {
            reasons.push(format!(
                "{in_window} errata dated within 30 days of launch_date {} (month-one limit is 3)",
                ops.launch_date.as_deref().unwrap_or_default()
            ));
        }
    }

    Ok(TripwireOutcome { reasons })
}

/// Today's UTC date as `YYYY-MM-DD` (civil-from-days, the inverse of
/// the parser in `validate`).
pub fn today_utc() -> String {
    let days = crate::validate::now_epoch().div_euclid(86_400);
    let (y, m, d) = civil_from_days(days);
    format!("{y:04}-{m:02}-{d:02}")
}

fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

pub fn run(data_dir: &Path, today: Option<&str>) -> ExitCode {
    let today = today.map(str::to_string).unwrap_or_else(today_utc);
    match evaluate(data_dir, &today) {
        Ok(outcome) => {
            if outcome.tripped() {
                for reason in &outcome.reasons {
                    println!("tripwire: TRIPPED — {reason}");
                }
                println!("tripwire: the auto lane is closed (plan T4.5)");
                ExitCode::from(1)
            } else {
                println!("tripwire: clear");
                ExitCode::SUCCESS
            }
        }
        Err(e) => {
            eprintln!("tripwire: {e}");
            ExitCode::from(2)
        }
    }
}
