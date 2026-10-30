//! `mongil-build note` (plan T4.4, D-13): the weekly "what moved"
//! note, generated entirely from a snapshot diff — a fixed template,
//! never free-written. Writes `data/notes/<date>.md` with frontmatter
//! `{from, to, status}`:
//!
//! - a diff with changes → `status: draft` (Tom approves, T5.5);
//! - an empty diff → `status: auto` with the D-12 zero-change body,
//!   which publishes without approval;
//! - `--from none` → the launch-baseline note: every placement in the
//!   `to` snapshot is a new placement, every voting source is newly
//!   counted. Also a draft — it makes claims, so Tom approves it.
//!
//! Only `approved` or `auto` notes render on the site (check-dist
//! verifies no draft is built). An existing **approved** note is
//! never overwritten by regeneration.

use mongil_core::load::Dataset;
use mongil_core::snapshot::{diff_docs, SnapshotDiff, SnapshotDoc};
use std::collections::BTreeMap;
use std::path::Path;
use std::process::ExitCode;

const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/// "2026-10-05" → "Oct 5, 2026" (input may be a longer timestamp).
pub fn fmt_date(stamp: &str) -> String {
    let date = &stamp[..stamp.len().min(10)];
    let mut parts = date.split('-');
    let (Some(y), Some(m), Some(d)) = (parts.next(), parts.next(), parts.next()) else {
        return stamp.to_string();
    };
    let Ok(month) = m.parse::<usize>() else {
        return stamp.to_string();
    };
    let Ok(day) = d.parse::<u32>() else {
        return stamp.to_string();
    };
    match MONTHS.get(month.wrapping_sub(1)) {
        Some(name) => format!("{name} {day}, {y}"),
        None => stamp.to_string(),
    }
}

fn label_word(label: &str) -> &str {
    match label {
        "SOLID" => "Solid",
        "CONTESTED" => "Contested",
        "SPECULATION" => "Speculation",
        "MODE_SPLIT" => "Mode-split",
        "UNRANKED" => "Unranked",
        other => other,
    }
}

fn level_title(level: &str) -> &str {
    match level {
        "low" => "Low",
        "high" => "High",
        "max" => "Max",
        other => other,
    }
}

/// The display date of a snapshot: its id's date prefix when the id
/// starts with one, else its creation date.
fn snapshot_date(doc: &SnapshotDoc) -> String {
    let candidate = if doc.id.len() >= 10 {
        &doc.id[..10]
    } else {
        ""
    };
    if candidate.len() == 10 && crate::validate::parse_rfc3339(candidate).is_some() {
        fmt_date(candidate)
    } else {
        fmt_date(&doc.created_at)
    }
}

/// Source facts for citations: display name + page URL from the
/// registry, list date from the snapshot being cited.
struct SourceInfo {
    name: String,
    url: String,
}

struct Ctx {
    char_names: BTreeMap<String, String>,
    sources: BTreeMap<String, SourceInfo>,
    /// List dates as recorded in the TO snapshot.
    list_dates: BTreeMap<String, Option<String>>,
    /// The voting source ids (the baseline note counts only these).
    voting: std::collections::BTreeSet<String>,
}

impl Ctx {
    fn source_name(&self, id: &str) -> String {
        self.sources
            .get(id)
            .map(|s| s.name.clone())
            .unwrap_or_else(|| id.to_string())
    }

    /// " (Oct 5, 2026, [Pocket Gamer](https://…))" — the citation that
    /// ends a move line. The date is omitted when the snapshot records
    /// none for the source.
    fn citation(&self, source: &str) -> String {
        let name = self.source_name(source);
        let url = self
            .sources
            .get(source)
            .map(|s| s.url.clone())
            .unwrap_or_default();
        let link = if url.is_empty() {
            name.clone()
        } else {
            format!("[{name}]({url})")
        };
        match self.list_dates.get(source).cloned().flatten() {
            Some(date) => format!(" ({}, {link})", fmt_date(&date)),
            None => format!(" ({link})"),
        }
    }
}

fn is_tier_field(field: &str) -> bool {
    field == "tier" || field.ends_with(".tier") || field == "conquest" || field == "rift"
}

/// Render the note body for a non-empty diff (or a baseline).
fn render_body(
    ctx: &Ctx,
    diff: &SnapshotDiff,
    from_doc: &SnapshotDoc,
    to_doc: &SnapshotDoc,
    baseline: bool,
) -> String {
    let mut sections: Vec<String> = Vec::new();
    let mut count = 0usize;

    if !baseline && !diff.tier_changes.is_empty() {
        let mut lines = vec!["## Tier changes".to_string()];
        for c in &diff.tier_changes {
            let name = ctx.char_names.get(&c.character).unwrap_or(&c.character);
            let mut line = format!(
                "- {name}: {} → {} at {}.",
                c.from,
                c.to,
                level_title(&c.level)
            );
            let candidates: Vec<_> = diff
                .placement_changes
                .iter()
                .filter(|p| p.character == c.character)
                .collect();
            let pick = candidates
                .iter()
                .find(|p| is_tier_field(&p.field))
                .or_else(|| candidates.first());
            if let Some(p) = pick {
                line.push_str(&format!(
                    " Why: {} changed {} → {}{}.",
                    ctx.source_name(&p.source),
                    p.from,
                    p.to,
                    ctx.citation(&p.source)
                ));
            }
            lines.push(line);
            count += 1;
        }
        sections.push(lines.join("\n"));
    }

    if !baseline && !diff.label_changes.is_empty() {
        let mut lines = vec!["## Label changes".to_string()];
        for c in &diff.label_changes {
            let name = ctx.char_names.get(&c.character).unwrap_or(&c.character);
            let reason = if let Some(t) = diff
                .tier_changes
                .iter()
                .find(|t| t.character == c.character && t.level == c.level)
            {
                format!("Its tier moved {} → {}.", t.from, t.to)
            } else if let Some(v) = diff
                .coverage_changes
                .iter()
                .find(|v| v.character == c.character && v.level == c.level)
            {
                format!("Coverage changed {} → {} lists.", v.from, v.to)
            } else if let Some(p) = diff
                .placement_changes
                .iter()
                .find(|p| p.character == c.character)
            {
                format!(
                    "{} changed {} {} → {}.",
                    ctx.source_name(&p.source),
                    p.field,
                    p.from,
                    p.to
                )
            } else {
                "The lists' spread changed.".to_string()
            };
            lines.push(format!(
                "- {name}: {} → {} at {}. {reason}",
                label_word(&c.from),
                label_word(&c.to),
                level_title(&c.level)
            ));
            count += 1;
        }
        sections.push(lines.join("\n"));
    }

    if !baseline {
        // Coverage changes with no tier or label change at the same
        // character + level (the template has no section for them;
        // dropping them would lose a real change).
        let covered: Vec<_> = diff
            .coverage_changes
            .iter()
            .filter(|v| {
                !diff
                    .tier_changes
                    .iter()
                    .any(|t| t.character == v.character && t.level == v.level)
                    && !diff
                        .label_changes
                        .iter()
                        .any(|l| l.character == v.character && l.level == v.level)
            })
            .collect();
        if !covered.is_empty() {
            let mut lines = vec!["## Coverage changes".to_string()];
            for v in covered {
                let name = ctx.char_names.get(&v.character).unwrap_or(&v.character);
                lines.push(format!(
                    "- {name}: {} → {} lists at {}.",
                    v.from,
                    v.to,
                    level_title(&v.level)
                ));
                count += 1;
            }
            sections.push(lines.join("\n"));
        }
    }

    // New placements (and removals), grouped per source + character.
    {
        let mut groups: BTreeMap<(&str, &str), Vec<&mongil_core::snapshot::PlacementChange>> =
            BTreeMap::new();
        for p in &diff.placement_changes {
            groups
                .entry((p.source.as_str(), p.character.as_str()))
                .or_default()
                .push(p);
        }
        let mut lines: Vec<String> = Vec::new();
        for ((source, character), changes) in &groups {
            if baseline && !ctx.voting.contains(*source) {
                // In the baseline note only the voting lists' census
                // is launch news; check-only placements stay frozen
                // in the snapshot without headlining the note.
                continue;
            }
            let all_new = changes.iter().all(|p| p.from == "—");
            let all_gone = changes.iter().all(|p| p.to == "—");
            if !all_new && !all_gone {
                continue;
            }
            let fallback = character.to_string();
            let name = ctx.char_names.get(*character).unwrap_or(&fallback);
            let mut parts: Vec<String> = Vec::new();
            for p in changes {
                if p.field == "status" {
                    continue;
                }
                let value = if all_new { &p.to } else { &p.from };
                let part = if p.field == "tier" {
                    value.clone()
                } else if let Some(level) = p.field.strip_suffix(".tier") {
                    format!("{} {value}", level_title(level))
                } else if p.field == "conquest" {
                    format!("Conquest {value}")
                } else if p.field == "rift" {
                    format!("Rift {value}")
                } else {
                    format!("{} {value}", p.field)
                };
                parts.push(part);
            }
            if parts.is_empty() {
                continue;
            }
            if all_new {
                lines.push(format!(
                    "- {} placed {name} at {}{}.",
                    ctx.source_name(source),
                    parts.join(" · "),
                    ctx.citation(source)
                ));
            } else {
                lines.push(format!(
                    "- {} no longer places {name} (was {}).",
                    ctx.source_name(source),
                    parts.join(" · ")
                ));
            }
            count += 1;
        }
        if !lines.is_empty() {
            let mut section = vec!["## New placements".to_string()];
            section.extend(lines);
            sections.push(section.join("\n"));
        }
    }

    // Sources: status / list-date changes between the two snapshots.
    {
        let mut lines: Vec<String> = Vec::new();
        let from_sources: BTreeMap<&str, &mongil_core::snapshot::SnapshotSource> = from_doc
            .sources
            .iter()
            .map(|s| (s.id.as_str(), s))
            .collect();
        let to_sources: BTreeMap<&str, &mongil_core::snapshot::SnapshotSource> =
            to_doc.sources.iter().map(|s| (s.id.as_str(), s)).collect();
        let mut ids: Vec<&str> = from_sources
            .keys()
            .chain(to_sources.keys())
            .copied()
            .collect();
        ids.sort();
        ids.dedup();
        for id in ids {
            let name = ctx.source_name(id);
            match (from_sources.get(id), to_sources.get(id)) {
                (None, Some(to)) => {
                    if baseline && !ctx.voting.contains(id) {
                        // In the baseline note, only the voting lists
                        // are "counted"; the rest are context.
                        continue;
                    }
                    lines.push(format!("- {name} is now counted ({}).", to.status));
                    count += 1;
                }
                (Some(_), None) => {
                    lines.push(format!("- {name} is no longer counted."));
                    count += 1;
                }
                (Some(a), Some(b)) => {
                    if a.status != b.status {
                        lines.push(format!("- {name} is now {} (was {}).", b.status, a.status));
                        count += 1;
                    } else if a.list_date != b.list_date {
                        lines.push(format!(
                            "- {name} updated its list ({} → {}).",
                            a.list_date
                                .as_deref()
                                .map(fmt_date)
                                .unwrap_or_else(|| "—".into()),
                            b.list_date
                                .as_deref()
                                .map(fmt_date)
                                .unwrap_or_else(|| "—".into())
                        ));
                        count += 1;
                    }
                }
                (None, None) => {}
            }
        }
        if !lines.is_empty() {
            let mut section = vec!["## Sources".to_string()];
            section.extend(lines);
            sections.push(section.join("\n"));
        }
    }

    let from_label = if baseline {
        "launch".to_string()
    } else {
        snapshot_date(from_doc)
    };
    let mut out = format!(
        "# What moved: {} → {}\n{} changes. Each one links to the list that caused it.\n",
        from_label,
        snapshot_date(to_doc),
        count
    );
    for section in sections {
        out.push('\n');
        out.push_str(&section);
        out.push('\n');
    }
    out
}

fn empty_snapshot_doc() -> SnapshotDoc {
    SnapshotDoc {
        id: "none".to_string(),
        contract_version: 1,
        created_at: String::new(),
        method_version: String::new(),
        game_version: String::new(),
        sources: Vec::new(),
        inputs_sha256: String::new(),
        inputs: mongil_core::snapshot::SnapshotInputs {
            placements: serde_json::Value::Object(serde_json::Map::new()),
            roster: serde_json::Value::Null,
            sources: serde_json::Value::Null,
        },
        results: mongil_core::snapshot::SnapshotResults {
            boards: BTreeMap::new(),
            characters: BTreeMap::new(),
        },
    }
}

/// The status line of an existing note file, if it has frontmatter.
fn existing_status(text: &str) -> Option<String> {
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

pub fn run(data_dir: &Path, from: &str, to: &str, date: &str) -> ExitCode {
    match generate(data_dir, from, to, date) {
        Ok(message) => {
            println!("note: {message}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("note: {e}");
            ExitCode::from(1)
        }
    }
}

fn generate(data_dir: &Path, from: &str, to: &str, date: &str) -> Result<String, String> {
    if date.len() != 10 || crate::validate::parse_rfc3339(date).is_none() {
        return Err(format!("--date `{date}` is not a YYYY-MM-DD date"));
    }
    let ds: Dataset = mongil_core::load_dataset(data_dir).map_err(|e| e.to_string())?;
    let to_doc = crate::snapshots::read_snapshot(data_dir, to)?;
    let baseline = from == "none";
    let from_doc = if baseline {
        empty_snapshot_doc()
    } else {
        crate::snapshots::read_snapshot(data_dir, from)?
    };

    let ctx = Ctx {
        char_names: ds
            .roster
            .characters
            .iter()
            .map(|c| (c.id.clone(), c.name.clone()))
            .collect(),
        sources: ds
            .sources
            .sources
            .iter()
            .map(|s| {
                (
                    s.id.clone(),
                    SourceInfo {
                        name: s.name.clone(),
                        url: s.url.clone(),
                    },
                )
            })
            .collect(),
        list_dates: to_doc
            .sources
            .iter()
            .map(|s| (s.id.clone(), s.list_date.clone()))
            .collect(),
        voting: ds
            .sources
            .sources
            .iter()
            .filter(|s| s.role == mongil_core::model::SourceRole::Voting)
            .map(|s| s.id.clone())
            .collect(),
    };

    let diff = diff_docs(&from_doc, &to_doc);
    let (status, body) = if !baseline && diff.is_empty() {
        let checked = ds
            .checks
            .as_ref()
            .map(|c| fmt_date(&c.sources_checked_at))
            .unwrap_or_else(|| snapshot_date(&to_doc));
        let body = format!(
            "# What moved: {} → {}\nNothing moved this week. Lists last checked {}.\n",
            snapshot_date(&from_doc),
            snapshot_date(&to_doc),
            checked
        );
        ("auto", body)
    } else {
        (
            "draft",
            render_body(&ctx, &diff, &from_doc, &to_doc, baseline),
        )
    };

    let notes_dir = data_dir.join("notes");
    std::fs::create_dir_all(&notes_dir)
        .map_err(|e| format!("cannot create {}: {e}", notes_dir.display()))?;
    let path = notes_dir.join(format!("{date}.md"));
    if path.exists() {
        let existing = std::fs::read_to_string(&path)
            .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        if existing_status(&existing).as_deref() == Some("approved") {
            return Err(format!(
                "{} is an approved note — refusing to overwrite it",
                path.display()
            ));
        }
    }
    let text = format!("---\nfrom: {from}\nto: {to}\nstatus: {status}\n---\n\n{body}");
    std::fs::write(&path, text).map_err(|e| format!("cannot write {}: {e}", path.display()))?;
    Ok(format!(
        "wrote {} (status: {status}, {} → {})",
        path.display(),
        from,
        to
    ))
}
