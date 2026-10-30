//! `mongil-build png` (plan T3.9): render the official per-level PNGs
//! and the OG image from the dataset, via mongil-render.
//!
//! Boards come from the same engine path as `emit` (`load_dataset` +
//! `method::compute`), so the images can never disagree with the site.
//! Output goes to `site/public/img/generated/` in the pipeline (the
//! directory is gitignored; Astro copies it into `dist/`).

use mongil_core::load::load_dataset;
use mongil_core::method::{compute, Label, TierBand};
use mongil_core::model::Level;
use mongil_render::{BoardView, ChipView, RowView};
use std::collections::BTreeMap;
use std::path::Path;
use std::process::ExitCode;

fn label_str(label: Label) -> &'static str {
    match label {
        Label::Solid => "SOLID",
        Label::Contested => "CONTESTED",
        Label::Speculation => "SPECULATION",
        Label::ModeSplit => "MODE_SPLIT",
        Label::Unranked => "UNRANKED",
    }
}

fn glyph(label: Label) -> &'static str {
    match label {
        Label::Solid => "●",
        Label::Contested => "◐",
        Label::Speculation => "○",
        Label::ModeSplit => "⇄",
        Label::Unranked => "—",
    }
}

fn tier_key(band: TierBand) -> &'static str {
    match band {
        TierBand::S => "S",
        TierBand::A => "A",
        TierBand::B => "B",
        TierBand::C => "C",
        TierBand::D => "D",
        TierBand::ModeSplit => "MODE_SPLIT",
        TierBand::Unranked => "UNRANKED",
    }
}

const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/// Civil date (Howard Hinnant's algorithm) from days since the epoch.
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = if m > 2 { m - 3 } else { m + 9 } as i64;
    let doy = (153 * mp + 2) / 5 + d as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// Day of week for a days-since-epoch value (0 = Sunday).
fn weekday(days: i64) -> i64 {
    (days + 4).rem_euclid(7) // 1970-01-01 was a Thursday
}

/// Epoch second when US Eastern daylight time starts in `year`
/// (second Sunday of March, 07:00 UTC) / ends (first Sunday of
/// November, 06:00 UTC).
fn dst_bounds_utc(year: i64) -> (i64, i64) {
    let nth_sunday = |month: u32, n: i64| -> i64 {
        let first = days_from_civil(year, month, 1);
        let delta = (7 - weekday(first)) % 7 + 7 * (n - 1);
        first + delta
    };
    let start = nth_sunday(3, 2) * 86_400 + 7 * 3600;
    let end = nth_sunday(11, 1) * 86_400 + 6 * 3600;
    (start, end)
}

/// "2026-10-30T01:20:00Z" -> "Oct 29, 2026": the check date in
/// America/New_York, matching the site's own stamp (data.ts).
fn date_text(iso: &str) -> String {
    let Some(epoch) = crate::validate::parse_rfc3339(iso) else {
        return iso.get(..10).unwrap_or(iso).to_string();
    };
    let (utc_year, _, _) = civil_from_days(epoch.div_euclid(86_400));
    let (start, end) = dst_bounds_utc(utc_year);
    let offset = if epoch >= start && epoch < end {
        -4 * 3600 // EDT
    } else {
        -5 * 3600 // EST
    };
    let (y, m, d) = civil_from_days((epoch + offset).div_euclid(86_400));
    let month = MONTHS[(m - 1) as usize];
    format!("{month} {d}, {y}")
}

pub fn run(data_dir: &Path, out_dir: &Path) -> ExitCode {
    let ds = match load_dataset(data_dir) {
        Ok(ds) => ds,
        Err(e) => {
            eprintln!("png: loading {}: {e}", data_dir.display());
            return ExitCode::FAILURE;
        }
    };
    let checked = ds
        .checks
        .as_ref()
        .map(|c| c.sources_checked_at.as_str())
        .unwrap_or("");
    let date = date_text(checked);

    let mut views: BTreeMap<Level, BoardView> = BTreeMap::new();
    for level in Level::ALL {
        let result = compute(&ds, &ds.method, level);
        let by_id: BTreeMap<&str, &mongil_core::method::CharResult> = result
            .characters
            .iter()
            .map(|c| (c.id.as_str(), c))
            .collect();
        let mut rows = Vec::new();
        for band in [
            TierBand::S,
            TierBand::A,
            TierBand::B,
            TierBand::C,
            TierBand::D,
            TierBand::ModeSplit,
            TierBand::Unranked,
        ] {
            let ids = result.boards.get(&band).cloned().unwrap_or_default();
            if ids.is_empty() {
                continue;
            }
            let chips = ids
                .iter()
                .map(|id| {
                    let c = by_id[id.as_str()];
                    ChipView {
                        name: c.name.clone(),
                        label: label_str(c.label).to_string(),
                        glyph: glyph(c.label).to_string(),
                    }
                })
                .collect();
            rows.push(RowView {
                tier: tier_key(band).to_string(),
                chips,
            });
        }
        views.insert(
            level,
            BoardView {
                level_name: match level {
                    Level::Low => "Low",
                    Level::High => "High",
                    Level::Max => "Max",
                }
                .to_string(),
                method_version: ds.method.version.clone(),
                date_text: date.clone(),
                rows,
            },
        );
    }

    if let Err(e) = std::fs::create_dir_all(out_dir) {
        eprintln!("png: creating {}: {e}", out_dir.display());
        return ExitCode::FAILURE;
    }
    let write = |name: &str, bytes: &[u8]| -> Result<(), String> {
        let path = out_dir.join(name);
        std::fs::write(&path, bytes).map_err(|e| format!("writing {}: {e}", path.display()))?;
        println!("png: {} ({} bytes)", path.display(), bytes.len());
        Ok(())
    };
    let outcome = (|| -> Result<(), String> {
        for (level, file) in [
            (Level::Low, "tierlist-low.png"),
            (Level::High, "tierlist-high.png"),
            (Level::Max, "tierlist-max.png"),
        ] {
            let png = mongil_render::render_board_png(&views[&level]).map_err(|e| e.to_string())?;
            write(file, &png)?;
        }
        let og = mongil_render::render_og_png(&views[&Level::Low]).map_err(|e| e.to_string())?;
        write("og.png", &og)?;
        Ok(())
    })();
    match outcome {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("png: {e}");
            ExitCode::FAILURE
        }
    }
}
