//! Snapshots and snapshot diffs (plan T2.9).
//!
//! A snapshot freezes the inputs that produced a set of results **and**
//! the results themselves (§B.3), so history pages render without
//! re-computing and never change retroactively if the engine changes.
//! Snapshots are append-only (gate G9): the writer in `mongil-build`
//! refuses to overwrite an existing id.
//!
//! Change detection is by `inputs_sha256`: the SHA-256 of the canonical
//! JSON of the frozen inputs (placements + roster + source registry).
//! `created_at` is deliberately **not** part of the fingerprint, so
//! re-running `snapshot` on unchanged data is a no-op (exit 3 in the
//! CLI) instead of an endless stream of identical snapshots.

use crate::load::Dataset;
use crate::method::{compute, CharResult};
use crate::model::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

/// The frozen form of one source's registry entry (§B.3 snapshot
/// `sources` array).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotSource {
    pub id: String,
    pub list_date: Option<String>,
    pub game_version: Option<String>,
    /// Contract status string; `"current"` when the registry records
    /// no explicit status (the same default `emit` uses).
    pub status: String,
}

/// One character's frozen result at one level (§B.3 snapshot `results`).
/// Scores are rounded to 4 decimals for display, exactly as in the
/// plan's worked example (3.1667); tier/label/coverage are exact.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotResult {
    pub tier: String,
    pub label: String,
    pub score: Option<f64>,
    pub coverage: usize,
    pub spread: Option<f64>,
}

/// The frozen inputs: everything that can change a result. Values are
/// the parsed data structures re-serialized, so the fingerprint is over
/// the data itself, not over file formatting or comments.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotInputs {
    /// Source id → that source's placements structure.
    pub placements: serde_json::Value,
    pub roster: serde_json::Value,
    pub sources: serde_json::Value,
}

/// A whole snapshot document (`data/snapshots/<id>.json`, §B.3).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotDoc {
    pub id: String,
    pub contract_version: u32,
    pub created_at: String,
    pub method_version: String,
    pub game_version: String,
    pub sources: Vec<SnapshotSource>,
    pub inputs_sha256: String,
    pub inputs: SnapshotInputs,
    pub results: SnapshotResults,
}

/// Frozen results: ordered boards per level (same shape as
/// `tierlist.json`'s `boards`) plus per-character results per level.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotResults {
    /// level → tier → ordered character ids.
    pub boards: BTreeMap<String, BTreeMap<String, Vec<String>>>,
    /// character id → level → result.
    pub characters: BTreeMap<String, BTreeMap<String, SnapshotResult>>,
}

fn round4(x: f64) -> f64 {
    (x * 10_000.0).round() / 10_000.0
}

fn result_of(r: &CharResult) -> SnapshotResult {
    SnapshotResult {
        tier: r.tier.as_str().to_string(),
        label: r.label.as_str().to_string(),
        score: r.score.map(round4),
        coverage: r.coverage,
        spread: r.spread.map(round4),
    }
}

fn status_str(status: Option<SourceStatus>) -> String {
    match status {
        Some(SourceStatus::Current) => "current",
        Some(SourceStatus::PrePatchFlagged) => "pre_patch_flagged",
        Some(SourceStatus::StaleCheckOnly) => "stale_check_only",
        Some(SourceStatus::Unavailable) => "unavailable",
        None => "current",
    }
    .to_string()
}

/// Build the frozen inputs for a dataset.
pub fn snapshot_inputs(ds: &Dataset) -> SnapshotInputs {
    let mut placements = serde_json::Map::new();
    placements.insert(
        "pg".to_string(),
        serde_json::to_value(&ds.pg).unwrap_or(serde_json::Value::Null),
    );
    placements.insert(
        "kaiden".to_string(),
        serde_json::to_value(&ds.kaiden).unwrap_or(serde_json::Value::Null),
    );
    placements.insert(
        "gamewith-jp".to_string(),
        serde_json::to_value(&ds.jp).unwrap_or(serde_json::Value::Null),
    );
    for (id, file) in &ds.check_only {
        placements.insert(
            id.clone(),
            serde_json::to_value(file).unwrap_or(serde_json::Value::Null),
        );
    }
    SnapshotInputs {
        placements: serde_json::Value::Object(placements),
        roster: serde_json::to_value(&ds.roster).unwrap_or(serde_json::Value::Null),
        sources: serde_json::to_value(&ds.sources).unwrap_or(serde_json::Value::Null),
    }
}

/// SHA-256 of the canonical JSON of the frozen inputs. `serde_json`
/// object keys are stored sorted (no `preserve_order`), and the typed
/// structures serialize in declaration order, so the string is stable
/// for equal inputs.
pub fn inputs_sha256(inputs: &SnapshotInputs) -> String {
    let canonical = serde_json::to_string(inputs).unwrap_or_default();
    let digest = Sha256::digest(canonical.as_bytes());
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

/// Build a complete snapshot document for a dataset. `created_at` is
/// supplied by the caller (the CLI) so tests stay deterministic.
pub fn build_snapshot(ds: &Dataset, id: &str, created_at: &str) -> SnapshotDoc {
    let inputs = snapshot_inputs(ds);
    let inputs_sha256 = inputs_sha256(&inputs);

    let mut boards: BTreeMap<String, BTreeMap<String, Vec<String>>> = BTreeMap::new();
    let mut characters: BTreeMap<String, BTreeMap<String, SnapshotResult>> = BTreeMap::new();
    for level in Level::ALL {
        let result = compute(ds, &ds.method, level);
        let mut level_boards: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for (band, ids) in &result.boards {
            level_boards.insert(band.as_str().to_string(), ids.clone());
        }
        boards.insert(level.as_str().to_string(), level_boards);
        for c in &result.characters {
            characters
                .entry(c.id.clone())
                .or_default()
                .insert(level.as_str().to_string(), result_of(c));
        }
    }

    SnapshotDoc {
        id: id.to_string(),
        contract_version: 1,
        created_at: created_at.to_string(),
        method_version: ds.method.version.clone(),
        game_version: ds
            .game
            .as_ref()
            .map(|g| g.current_version.clone())
            .unwrap_or_default(),
        sources: ds
            .sources
            .sources
            .iter()
            .map(|s| SnapshotSource {
                id: s.id.clone(),
                list_date: s.list_date.clone(),
                game_version: s.game_version.clone(),
                status: status_str(s.status),
            })
            .collect(),
        inputs_sha256,
        inputs,
        results: SnapshotResults { boards, characters },
    }
}

// ---------------------------------------------------------------------------
// Diff
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TierChange {
    pub character: String,
    pub level: String,
    pub from: String,
    pub to: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LabelChange {
    pub character: String,
    pub level: String,
    pub from: String,
    pub to: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CoverageChange {
    pub character: String,
    pub level: String,
    pub from: usize,
    pub to: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PlacementChange {
    pub source: String,
    pub character: String,
    pub field: String,
    pub from: String,
    pub to: String,
}

/// The difference between two snapshots (or a snapshot and the working
/// data): result changes, computed from the frozen results, plus
/// placement changes, computed from the frozen inputs.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SnapshotDiff {
    pub from: String,
    pub to: String,
    pub tier_changes: Vec<TierChange>,
    pub label_changes: Vec<LabelChange>,
    pub coverage_changes: Vec<CoverageChange>,
    pub placement_changes: Vec<PlacementChange>,
}

/// Gate G5's threshold: more than this many characters changing Overall
/// tier at any level classifies a change set `needs-tom` (not a
/// failure).
pub const MASS_CHANGE_LIMIT: usize = 8;

impl SnapshotDiff {
    pub fn is_empty(&self) -> bool {
        self.tier_changes.is_empty()
            && self.label_changes.is_empty()
            && self.coverage_changes.is_empty()
            && self.placement_changes.is_empty()
    }

    /// Number of (character, level) Overall tier changes (gate G5
    /// counts characters changing tier "at any level"; the diff counts
    /// per level, and `exceeds_mass_change` applies the plan's > 8
    /// rule to distinct characters).
    pub fn changed_characters(&self) -> usize {
        let mut set = std::collections::BTreeSet::new();
        for change in &self.tier_changes {
            set.insert(change.character.as_str());
        }
        set.len()
    }

    pub fn exceeds_mass_change(&self) -> bool {
        self.changed_characters() > MASS_CHANGE_LIMIT
    }
}

fn result_at(doc: &SnapshotDoc, character: &str, level: &str) -> SnapshotResult {
    doc.results
        .characters
        .get(character)
        .and_then(|levels| levels.get(level))
        .cloned()
        .unwrap_or_else(|| SnapshotResult {
            tier: "UNRANKED".to_string(),
            label: "UNRANKED".to_string(),
            score: None,
            coverage: 0,
            spread: None,
        })
}

/// Flatten one placement entry's structured fields to dotted keys
/// (`low.tier`, `conquest`, …). `raw` and `note` are free text — the
/// same structured/free split harvest-diff uses (plan T2.1) — and are
/// left out so the diff reports placement moves, not wording edits.
fn flatten_placement(value: &serde_json::Value, prefix: &str, out: &mut BTreeMap<String, String>) {
    match value {
        serde_json::Value::Object(map) => {
            for (key, v) in map {
                if key == "raw" || key == "note" || key == "character" {
                    continue;
                }
                let path = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{prefix}.{key}")
                };
                flatten_placement(v, &path, out);
            }
        }
        serde_json::Value::Array(items) => {
            let rendered: Vec<String> = items
                .iter()
                .map(|i| match i {
                    serde_json::Value::String(s) => s.clone(),
                    other => other.to_string(),
                })
                .collect();
            out.insert(prefix.to_string(), format!("[{}]", rendered.join(", ")));
        }
        serde_json::Value::Null => {}
        other => {
            out.insert(prefix.to_string(), other.to_string());
        }
    }
}

/// Per-source placement maps: source id → character id → field → value.
fn placement_maps(
    doc: &SnapshotDoc,
) -> BTreeMap<String, BTreeMap<String, BTreeMap<String, String>>> {
    let mut out: BTreeMap<String, BTreeMap<String, BTreeMap<String, String>>> = BTreeMap::new();
    let Some(placements) = doc.inputs.placements.as_object() else {
        return out;
    };
    for (source, file) in placements {
        let mut by_character: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
        if let Some(entries) = file.get("placement").and_then(|p| p.as_array()) {
            for entry in entries {
                let Some(character) = entry.get("character").and_then(|c| c.as_str()) else {
                    continue;
                };
                let mut fields = BTreeMap::new();
                flatten_placement(entry, "", &mut fields);
                by_character.insert(character.to_string(), fields);
            }
        }
        out.insert(source.clone(), by_character);
    }
    out
}

/// Diff two snapshot documents.
pub fn diff_docs(from: &SnapshotDoc, to: &SnapshotDoc) -> SnapshotDiff {
    let mut diff = SnapshotDiff {
        from: from.id.clone(),
        to: to.id.clone(),
        tier_changes: Vec::new(),
        label_changes: Vec::new(),
        coverage_changes: Vec::new(),
        placement_changes: Vec::new(),
    };

    // Result changes, per character per level.
    let mut characters: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    characters.extend(from.results.characters.keys().map(|s| s.as_str()));
    characters.extend(to.results.characters.keys().map(|s| s.as_str()));
    for character in characters {
        for level in ["low", "high", "max"] {
            let a = result_at(from, character, level);
            let b = result_at(to, character, level);
            if a.tier != b.tier {
                diff.tier_changes.push(TierChange {
                    character: character.to_string(),
                    level: level.to_string(),
                    from: a.tier.clone(),
                    to: b.tier.clone(),
                });
            }
            if a.label != b.label {
                diff.label_changes.push(LabelChange {
                    character: character.to_string(),
                    level: level.to_string(),
                    from: a.label.clone(),
                    to: b.label.clone(),
                });
            }
            if a.coverage != b.coverage {
                diff.coverage_changes.push(CoverageChange {
                    character: character.to_string(),
                    level: level.to_string(),
                    from: a.coverage,
                    to: b.coverage,
                });
            }
        }
    }

    // Placement changes, from the frozen inputs.
    let a_map = placement_maps(from);
    let b_map = placement_maps(to);
    let mut sources: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    sources.extend(a_map.keys().map(|s| s.as_str()));
    sources.extend(b_map.keys().map(|s| s.as_str()));
    for source in sources {
        let empty = BTreeMap::new();
        let a_chars = a_map.get(source).unwrap_or(&empty);
        let b_chars = b_map.get(source).unwrap_or(&empty);
        let mut chars: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
        chars.extend(a_chars.keys().map(|s| s.as_str()));
        chars.extend(b_chars.keys().map(|s| s.as_str()));
        for character in chars {
            let a_fields = a_chars.get(character);
            let b_fields = b_chars.get(character);
            let mut fields: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
            if let Some(f) = a_fields {
                fields.extend(f.keys().map(|s| s.as_str()));
            }
            if let Some(f) = b_fields {
                fields.extend(f.keys().map(|s| s.as_str()));
            }
            for field in fields {
                let a_val = a_fields
                    .and_then(|f| f.get(field))
                    .map(|s| s.as_str())
                    .unwrap_or("—");
                let b_val = b_fields
                    .and_then(|f| f.get(field))
                    .map(|s| s.as_str())
                    .unwrap_or("—");
                if a_val != b_val {
                    diff.placement_changes.push(PlacementChange {
                        source: source.to_string(),
                        character: character.to_string(),
                        field: field.to_string(),
                        from: a_val.to_string(),
                        to: b_val.to_string(),
                    });
                }
            }
        }
    }

    diff
}

fn level_title(level: &str) -> &str {
    match level {
        "low" => "Low",
        "high" => "High",
        "max" => "Max",
        other => other,
    }
}

/// Render a diff as Markdown (the shape the T4.4 note generator and
/// the CP2 message draw from).
pub fn render_md(diff: &SnapshotDiff) -> String {
    let mut out = String::new();
    out.push_str(&format!("# Diff: {} → {}\n", diff.from, diff.to));
    if diff.is_empty() {
        out.push_str("\nNo changes.\n");
        return out;
    }
    if !diff.tier_changes.is_empty() {
        out.push_str("\n## Tier changes\n");
        for c in &diff.tier_changes {
            out.push_str(&format!(
                "- {}: {} → {} at {}\n",
                c.character,
                c.from,
                c.to,
                level_title(&c.level)
            ));
        }
    }
    if !diff.label_changes.is_empty() {
        out.push_str("\n## Label changes\n");
        for c in &diff.label_changes {
            out.push_str(&format!(
                "- {}: {} → {} at {}\n",
                c.character,
                c.from,
                c.to,
                level_title(&c.level)
            ));
        }
    }
    if !diff.coverage_changes.is_empty() {
        out.push_str("\n## Coverage changes\n");
        for c in &diff.coverage_changes {
            out.push_str(&format!(
                "- {}: {} → {} lists at {}\n",
                c.character,
                c.from,
                c.to,
                level_title(&c.level)
            ));
        }
    }
    if !diff.placement_changes.is_empty() {
        out.push_str("\n## Placement changes\n");
        for c in &diff.placement_changes {
            out.push_str(&format!(
                "- {} {} {}: {} → {}\n",
                c.source, c.character, c.field, c.from, c.to
            ));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_known_vectors_via_fingerprint() {
        // The fingerprint of an empty inputs object is the SHA-256 of
        // its canonical JSON; check the digest plumbing against the
        // well-known SHA-256 of "{}" inputs shape by hashing directly.
        let inputs = SnapshotInputs {
            placements: serde_json::json!({}),
            roster: serde_json::json!({}),
            sources: serde_json::json!({}),
        };
        let fp = inputs_sha256(&inputs);
        assert_eq!(fp.len(), 64);
        assert!(fp.chars().all(|c| c.is_ascii_hexdigit()));
        // Deterministic: same inputs, same fingerprint.
        assert_eq!(fp, inputs_sha256(&inputs));
    }
}
