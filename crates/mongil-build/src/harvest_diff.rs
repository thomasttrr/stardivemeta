//! `mongil-build harvest-diff` (plan T2.1): compare the two independent
//! reads (A = rendered text, B = screenshot) of one source harvest.
//!
//! Structured fields are **blocking**: character set, `world`, `conquest`,
//! `rift`, `tier`, `low/high/max.tier`, `awakening`, `flags`, `status`,
//! `page_updated`. `raw` and `note` are free text: they are compared
//! after whitespace/punctuation normalisation and a difference there is
//! a **warning**, never a block. Meta fields that differ by design
//! (worker, read mode, read time) are not compared.
//!
//! Output follows §B.4:
//! ```text
//! harvest-diff kaiden 2026-11-03-am
//! MISMATCH mina.high.tier        A="T0.5"  B="T1"
//! MISSING  chloe                 A=present B=absent
//! WARN     narae.low.raw         A="T0"    B="T0 "   (normalised equal)
//! 3 blocking, 1 warning → update blocked
//! ```
//! Exit 1 when any blocking difference exists, 0 otherwise (warnings
//! included), 2 when a file cannot be read or parsed.

use std::collections::BTreeMap;
use std::path::Path;
use std::process::ExitCode;

/// One parsed harvest file, reduced to the fields the diff compares.
pub struct HarvestFile {
    pub source: String,
    pub sweep: Option<String>,
    /// character → field → formatted value (blocking comparisons).
    pub blocking: BTreeMap<String, BTreeMap<String, String>>,
    /// character → field → raw text (warning comparisons).
    pub warnings: BTreeMap<String, BTreeMap<String, String>>,
}

fn fmt_value(v: &toml::Value) -> String {
    match v {
        toml::Value::String(s) => format!("\"{s}\""),
        toml::Value::Array(items) => {
            let inner: Vec<String> = items.iter().map(fmt_value).collect();
            format!("[{}]", inner.join(", "))
        }
        other => other.to_string(),
    }
}

fn get_str<'a>(table: &'a toml::value::Table, key: &str) -> Option<&'a str> {
    table.get(key).and_then(|v| v.as_str())
}

/// Parse one harvest/placements file into its comparison maps. The
/// source is taken from `[meta] source`, falling back to the file stem.
pub fn parse_harvest(path: &Path) -> Result<HarvestFile, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    let value: toml::Value =
        toml::from_str(&text).map_err(|e| format!("cannot parse {}: {e}", path.display()))?;
    let root = value
        .as_table()
        .ok_or_else(|| format!("{}: top level is not a table", path.display()))?;

    let meta = root.get("meta").and_then(|m| m.as_table());
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        // Harvest files are named `<source>.a.toml` / `<source>.b.toml`.
        .trim_end_matches(".a")
        .trim_end_matches(".b")
        .to_string();
    let source = meta
        .and_then(|m| get_str(m, "source").map(|s| s.to_string()))
        .unwrap_or(stem);
    let sweep = meta.and_then(|m| get_str(m, "harvest").map(|s| s.to_string()));

    let mut out = HarvestFile {
        source,
        sweep,
        blocking: BTreeMap::new(),
        warnings: BTreeMap::new(),
    };

    let placements = root
        .get("placement")
        .and_then(|p| p.as_array())
        .cloned()
        .unwrap_or_default();
    for entry in &placements {
        let Some(entry) = entry.as_table() else {
            continue;
        };
        let Some(character) = get_str(entry, "character") else {
            continue;
        };
        let blocking = out.blocking.entry(character.to_string()).or_default();
        let warnings = out.warnings.entry(character.to_string()).or_default();

        // Fields shared by every source shape.
        for key in [
            "world",
            "conquest",
            "rift",
            "tier",
            "status",
            "page_updated",
        ] {
            if let Some(v) = entry.get(key) {
                blocking.insert(key.to_string(), fmt_value(v));
            }
        }
        if let Some(v) = entry.get("note") {
            warnings.insert("note".to_string(), raw_text(v));
        }
        if let Some(v) = entry.get("raw") {
            warnings.insert("raw".to_string(), raw_text(v));
        }
        // Kaiden level cells.
        for level in ["low", "high", "max"] {
            if let Some(cell) = entry.get(level).and_then(|c| c.as_table()) {
                if let Some(v) = cell.get("tier") {
                    blocking.insert(format!("{level}.tier"), fmt_value(v));
                }
                if let Some(v) = cell.get("awakening") {
                    blocking.insert(format!("{level}.awakening"), fmt_value(v));
                }
                // Flags default to [] when the key is absent
                // (skip_serializing in the data model), so an absent
                // key and an empty list are the same read.
                let flags = cell
                    .get("flags")
                    .map(fmt_value)
                    .unwrap_or_else(|| "[]".to_string());
                blocking.insert(format!("{level}.flags"), flags);
                if let Some(v) = cell.get("raw") {
                    warnings.insert(format!("{level}.raw"), raw_text(v));
                }
            }
        }
    }
    Ok(out)
}

fn raw_text(v: &toml::Value) -> String {
    match v {
        toml::Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

/// Whitespace/punctuation normalisation for free-text comparison:
/// lowercase, keep alphanumerics, collapse everything else to single
/// spaces, trim.
pub fn normalize(text: &str) -> String {
    let mut out = String::new();
    let mut last_was_space = true; // trims leading separators
    for ch in text.chars() {
        if ch.is_alphanumeric() {
            out.extend(ch.to_lowercase());
            last_was_space = false;
        } else if !last_was_space {
            out.push(' ');
            last_was_space = true;
        }
    }
    out.trim_end().to_string()
}

pub struct DiffOutcome {
    pub blocking: usize,
    pub report: String,
}

/// Compare two parsed harvest files (A vs B).
pub fn compare(a: &HarvestFile, b: &HarvestFile) -> DiffOutcome {
    let mut lines: Vec<String> = Vec::new();
    let mut blocking_count = 0usize;
    let mut warning_count = 0usize;

    // (kind, field, rendered) so widths can be computed before printing.
    let mut rows: Vec<(&str, String, String)> = Vec::new();

    if a.source != b.source {
        rows.push((
            "MISMATCH",
            "source".to_string(),
            format!("A=\"{}\"  B=\"{}\"", a.source, b.source),
        ));
        blocking_count += 1;
    }

    // Character set differences are blocking MISSING rows.
    let mut characters: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    characters.extend(a.blocking.keys().map(|s| s.as_str()));
    characters.extend(b.blocking.keys().map(|s| s.as_str()));
    characters.extend(a.warnings.keys().map(|s| s.as_str()));
    characters.extend(b.warnings.keys().map(|s| s.as_str()));
    for character in &characters {
        let in_a = a.blocking.contains_key(*character) || a.warnings.contains_key(*character);
        let in_b = b.blocking.contains_key(*character) || b.warnings.contains_key(*character);
        if in_a && !in_b {
            rows.push((
                "MISSING",
                character.to_string(),
                "A=present B=absent".to_string(),
            ));
            blocking_count += 1;
            continue;
        }
        if in_b && !in_a {
            rows.push((
                "MISSING",
                character.to_string(),
                "A=absent B=present".to_string(),
            ));
            blocking_count += 1;
            continue;
        }
        // Structured fields.
        let a_fields = a.blocking.get(*character).cloned().unwrap_or_default();
        let b_fields = b.blocking.get(*character).cloned().unwrap_or_default();
        let mut fields: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
        fields.extend(a_fields.keys().map(|s| s.as_str()));
        fields.extend(b_fields.keys().map(|s| s.as_str()));
        for field in fields {
            let a_val = a_fields.get(field).map(|s| s.as_str());
            let b_val = b_fields.get(field).map(|s| s.as_str());
            if a_val != b_val {
                rows.push((
                    "MISMATCH",
                    format!("{character}.{field}"),
                    format!(
                        "A={}  B={}",
                        a_val.unwrap_or("<absent>"),
                        b_val.unwrap_or("<absent>")
                    ),
                ));
                blocking_count += 1;
            }
        }
        // Free-text fields: warnings only.
        let a_text = a.warnings.get(*character).cloned().unwrap_or_default();
        let b_text = b.warnings.get(*character).cloned().unwrap_or_default();
        let mut text_fields: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
        text_fields.extend(a_text.keys().map(|s| s.as_str()));
        text_fields.extend(b_text.keys().map(|s| s.as_str()));
        for field in text_fields {
            let a_val = a_text.get(field).map(|s| s.as_str()).unwrap_or("");
            let b_val = b_text.get(field).map(|s| s.as_str()).unwrap_or("");
            if a_val != b_val {
                let note = if normalize(a_val) == normalize(b_val) {
                    "(normalised equal)"
                } else {
                    "(normalised differs)"
                };
                rows.push((
                    "WARN",
                    format!("{character}.{field}"),
                    format!("A=\"{a_val}\"  B=\"{b_val}\"   {note}"),
                ));
                warning_count += 1;
            }
        }
    }

    let width = rows
        .iter()
        .map(|(_, field, _)| field.len())
        .max()
        .unwrap_or(0)
        + 4;
    for (kind, field, rendered) in &rows {
        lines.push(format!("{kind:<9}{field:<width$}{rendered}"));
    }

    let source = if a.source == b.source {
        a.source.clone()
    } else {
        format!("{} vs {}", a.source, b.source)
    };
    let sweep = a
        .sweep
        .clone()
        .or_else(|| b.sweep.clone())
        .unwrap_or_else(|| "—".to_string());
    let mut report = format!("harvest-diff {source} {sweep}\n");
    for line in &lines {
        report.push_str(line);
        report.push('\n');
    }
    let verdict = if blocking_count > 0 {
        "update blocked"
    } else {
        "clean"
    };
    let warning_word = if warning_count == 1 {
        "warning"
    } else {
        "warnings"
    };
    report.push_str(&format!(
        "{blocking_count} blocking, {warning_count} {warning_word} → {verdict}\n"
    ));

    DiffOutcome {
        blocking: blocking_count,
        report,
    }
}

pub fn run(a_path: &Path, b_path: &Path) -> ExitCode {
    let a = match parse_harvest(a_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("harvest-diff: {e}");
            return ExitCode::from(2);
        }
    };
    let b = match parse_harvest(b_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("harvest-diff: {e}");
            return ExitCode::from(2);
        }
    };
    let outcome = compare(&a, &b);
    print!("{}", outcome.report);
    if outcome.blocking > 0 {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
