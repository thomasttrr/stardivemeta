//! `mongil-build validate` (plan T1.6 + §B.7 gates; cores completed
//! in T2.10).
//!
//! Implemented: G1 (schema — the loader parses every file with
//! `deny_unknown_fields` and resolves references), G2 (slugs, URL
//! hygiene, roster-fact sources, game-version mapping), G3 (raw tiers
//! belong to the method file's conversion tables), G4 core (coverage
//! drop vs the latest snapshot on disk — the full form diffs against
//! `origin/main`, which only exists once the repo has a remote), G5
//! (reported, never a failure: the count feeds the lane), G6 core
//! (double-read enforcement whenever a sweep's harvest directory is on
//! disk), G10 (structural half + the expiry half of the status check;
//! the computed-status half is the T2.7 engine in `emit`),
//! G11 (stamp shape + not-in-the-future), G12 core + G14 (run when
//! `--dist` / `--site` are passed; CI passes `--site`).
//!
//! Deferred, and printed as notes so the gap is visible rather than
//! silent: G5 + G13's remaining lane conditions (T4.5/T4.6), G7 + G8
//! (`cargo test` in the CI `rust` job), G9 (git check in the CI `data`
//! job; the method-file half applies only once `launch_date` exists,
//! T4.6), the computed-status half of G10 (codes engine, T2.7), the
//! monotonic half of G11 (git history, CI), and G12's wiring into the
//! site build (T3.10).

use mongil_core::load::Dataset;
use mongil_core::model::{CodeObservationStatus, SourceStatus};
use mongil_core::snapshot::{build_snapshot, diff_docs, SnapshotDoc};
use std::collections::BTreeSet;
use std::path::Path;

pub struct Outcome {
    pub passed: Vec<String>,
    pub notes: Vec<String>,
    pub errors: Vec<String>,
}

fn is_slug(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

fn check_url(url: &str, errors: &mut Vec<String>, where_: &str) {
    if url.is_empty() || url.contains('<') || url.contains('…') {
        errors.push(format!(
            "G2: {where_} has a placeholder or empty URL: {url:?}"
        ));
    }
}

/// Parse the `YYYY-MM-DDTHH:MM:SSZ` stamp shape (the only one the
/// pipeline writes). Returns the string itself for lexicographic
/// comparison when the shape is right.
fn check_stamp(value: &str, errors: &mut Vec<String>, where_: &str) {
    let bytes = value.as_bytes();
    let shape_ok = value.len() == 20
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[10] == b'T'
        && bytes[13] == b':'
        && bytes[16] == b':'
        && bytes[19] == b'Z'
        && value
            .chars()
            .enumerate()
            .all(|(i, c)| matches!(i, 4 | 7 | 10 | 13 | 16 | 19) || c.is_ascii_digit());
    if !shape_ok {
        errors.push(format!(
            "G11: {where_} is not a YYYY-MM-DDTHH:MM:SSZ stamp: {value:?}"
        ));
        return;
    }
    if parse_rfc3339(value).is_none() {
        errors.push(format!(
            "G11: {where_} is not a real date and time: {value:?}"
        ));
        return;
    }
    let now = crate::emit::now_iso_utc();
    if value > now.as_str() {
        errors.push(format!("G11: {where_} is in the future: {value} > {now}"));
    }
}

/// A `YYYY-MM-DD` day that must exist; pushes a G11 error and returns
/// `None` otherwise. Returns the epoch seconds of midnight UTC.
fn check_day(value: &str, errors: &mut Vec<String>, where_: &str) -> Option<i64> {
    let parsed = if value.len() == 10 {
        parse_rfc3339(value)
    } else {
        None
    };
    if parsed.is_none() {
        errors.push(format!(
            "G11: {where_} is not a real YYYY-MM-DD date: {value:?}"
        ));
    }
    parsed
}

/// Whether `year-month-day` is a real Gregorian calendar date.
pub(crate) fn valid_civil_date(year: i64, month: i64, day: i64) -> bool {
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => return false,
    };
    (1..=days).contains(&day)
}

/// Days since 1970-01-01 for a civil date (Howard Hinnant's algorithm).
fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// Parse an RFC 3339 timestamp (or bare `YYYY-MM-DD`, treated as
/// midnight UTC) to epoch seconds. Used by the G10 expiry core, where
/// offsets matter (a JST expiry is not a UTC expiry), and by the T2.7
/// codes engine in `emit`.
pub(crate) fn parse_rfc3339(s: &str) -> Option<i64> {
    let s = s.trim();
    let (date, time, offset_secs) = if s.len() == 10 {
        (s, "00:00:00", 0i64)
    } else {
        if s.len() < 19 || !matches!(s.as_bytes().get(10), Some(b'T') | Some(b' ')) {
            return None;
        }
        let date = s.get(..10)?;
        let rest = &s[11..]; // HH:MM:SS[.fff](Z|±HH:MM)
        let (time, offset) = if let Some(t) = rest.strip_suffix('Z') {
            (t, 0i64)
        } else if let Some(idx) = rest.rfind(['+', '-']) {
            if idx == 0 {
                return None;
            }
            let sign = &rest[idx..idx + 1];
            let mut parts = rest[idx + 1..].split(':');
            let oh: i64 = parts.next()?.parse().ok()?;
            let om: i64 = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
            let magnitude = oh * 3600 + om * 60;
            (
                &rest[..idx],
                if sign == "+" { magnitude } else { -magnitude },
            )
        } else {
            (rest, 0i64)
        };
        (date, time.split('.').next()?, offset)
    };
    let mut dparts = date.split('-');
    let year: i64 = dparts.next()?.parse().ok()?;
    let month: i64 = dparts.next()?.parse().ok()?;
    let day: i64 = dparts.next()?.parse().ok()?;
    let mut tparts = time.split(':');
    let hour: i64 = tparts.next()?.parse().ok()?;
    let minute: i64 = tparts.next()?.parse().ok()?;
    let second: i64 = tparts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
    // A date or time that does not exist (Feb 30, hour 25) is not a
    // timestamp: callers get `None` instead of a silently rolled-over day.
    if !valid_civil_date(year, month, day)
        || !(0..24).contains(&hour)
        || !(0..60).contains(&minute)
        || !(0..60).contains(&second)
        || offset_secs.abs() >= 24 * 3600
    {
        return None;
    }
    Some(
        days_from_civil(year, month, day) * 86_400 + hour * 3600 + minute * 60 + second
            - offset_secs,
    )
}

pub(crate) fn now_epoch() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// Characters a source currently places (casts a vote or a mode
/// placement for), per source id.
fn placed_sets(ds: &Dataset) -> std::collections::BTreeMap<String, BTreeSet<String>> {
    let mut out: std::collections::BTreeMap<String, BTreeSet<String>> =
        std::collections::BTreeMap::new();
    out.insert(
        "pg".to_string(),
        ds.pg
            .placements
            .iter()
            .filter(|p| p.conquest.is_some() || p.rift.is_some())
            .map(|p| p.character.clone())
            .collect(),
    );
    out.insert(
        "kaiden".to_string(),
        ds.kaiden
            .placements
            .iter()
            .filter(|p| {
                mongil_core::model::Level::ALL
                    .iter()
                    .any(|level| p.cell(*level).is_some())
            })
            .map(|p| p.character.clone())
            .collect(),
    );
    out.insert(
        "gamewith-jp".to_string(),
        ds.jp
            .placements
            .iter()
            .filter(|p| p.tier.is_some())
            .map(|p| p.character.clone())
            .collect(),
    );
    for (id, file) in &ds.check_only {
        out.insert(
            id.clone(),
            file.placements
                .iter()
                .map(|p| p.character.clone())
                .collect(),
        );
    }
    out
}

/// Placed sets frozen inside a snapshot's inputs.
fn snapshot_placed_sets(doc: &SnapshotDoc) -> std::collections::BTreeMap<String, BTreeSet<String>> {
    let mut out = std::collections::BTreeMap::new();
    let Some(placements) = doc.inputs.placements.as_object() else {
        return out;
    };
    for (id, value) in placements {
        let mut set = BTreeSet::new();
        match id.as_str() {
            "pg" => {
                if let Ok(file) =
                    serde_json::from_value::<mongil_core::model::PgPlacements>(value.clone())
                {
                    set = file
                        .placements
                        .iter()
                        .filter(|p| p.conquest.is_some() || p.rift.is_some())
                        .map(|p| p.character.clone())
                        .collect();
                }
            }
            "kaiden" => {
                if let Ok(file) =
                    serde_json::from_value::<mongil_core::model::KaidenPlacements>(value.clone())
                {
                    set = file
                        .placements
                        .iter()
                        .filter(|p| {
                            mongil_core::model::Level::ALL
                                .iter()
                                .any(|level| p.cell(*level).is_some())
                        })
                        .map(|p| p.character.clone())
                        .collect();
                }
            }
            "gamewith-jp" => {
                if let Ok(file) =
                    serde_json::from_value::<mongil_core::model::JpPlacements>(value.clone())
                {
                    set = file
                        .placements
                        .iter()
                        .filter(|p| p.tier.is_some())
                        .map(|p| p.character.clone())
                        .collect();
                }
            }
            _ => {
                if let Ok(file) =
                    serde_json::from_value::<mongil_core::model::CheckOnlyPlacements>(value.clone())
                {
                    set = file
                        .placements
                        .iter()
                        .map(|p| p.character.clone())
                        .collect();
                }
            }
        }
        out.insert(id.clone(), set);
    }
    out
}

/// Whether a dropped character carries the status + note G4 requires
/// in the current placements.
fn has_status_note(ds: &Dataset, source: &str, character: &str) -> bool {
    fn noted(status: &Option<mongil_core::model::PlacementStatus>, note: &Option<String>) -> bool {
        status.is_some() && note.as_deref().is_some_and(|n| !n.trim().is_empty())
    }
    match source {
        "pg" => ds
            .pg
            .placements
            .iter()
            .find(|p| p.character == character)
            .is_some_and(|p| noted(&p.status, &p.note)),
        "kaiden" => ds
            .kaiden
            .placements
            .iter()
            .find(|p| p.character == character)
            .is_some_and(|p| noted(&p.status, &p.note)),
        "gamewith-jp" => ds
            .jp
            .placements
            .iter()
            .find(|p| p.character == character)
            .is_some_and(|p| noted(&p.status, &p.note)),
        // Check-only files have no status/note concept: a drop there
        // can never be excused by G4.
        _ => false,
    }
}

pub fn run(data_dir: &Path, site_dir: Option<&Path>, dist_dir: Option<&Path>) -> Outcome {
    let mut out = Outcome {
        passed: Vec::new(),
        notes: Vec::new(),
        errors: Vec::new(),
    };

    let ds: Dataset = match mongil_core::load_dataset(data_dir) {
        Ok(ds) => ds,
        Err(e) => {
            out.errors.push(format!("G1: dataset does not load: {e}"));
            return out;
        }
    };
    out.passed.push(
        "G1 schema: every data file parses (deny_unknown_fields); roster/source/placement \
         references resolve"
            .to_string(),
    );

    // -- G2: slugs ------------------------------------------------------
    let mut slug_errors = 0;
    for character in &ds.roster.characters {
        if !is_slug(&character.id) {
            out.errors.push(format!(
                "G2: character id is not a slug: {:?}",
                character.id
            ));
            slug_errors += 1;
        }
    }
    for source in &ds.sources.sources {
        if !is_slug(&source.id) {
            out.errors
                .push(format!("G2: source id is not a slug: {:?}", source.id));
            slug_errors += 1;
        }
    }
    if let Some(code_sources) = &ds.code_sources {
        for source in &code_sources.sources {
            if !is_slug(&source.id) {
                out.errors
                    .push(format!("G2: code-source id is not a slug: {:?}", source.id));
                slug_errors += 1;
            }
        }
    }
    if slug_errors == 0 {
        out.passed.push(
            "G2 slugs: every character / source / code-source id matches ^[a-z0-9-]+$".to_string(),
        );
    }

    // -- G2: URL hygiene -------------------------------------------------
    let before = out.errors.len();
    for source in &ds.sources.sources {
        check_url(&source.url, &mut out.errors, "sources.toml url");
    }
    for character in &ds.roster.characters {
        for url in character
            .released_on_sources
            .iter()
            .chain(&character.rarity_sources)
            .chain(&character.pool_sources)
        {
            check_url(url, &mut out.errors, "roster.toml fact source");
        }
    }
    for placement in &ds.kaiden.placements {
        if let Some(url) = &placement.page_url {
            check_url(url, &mut out.errors, "kaiden page_url");
        }
    }
    if let Some(code_sources) = &ds.code_sources {
        for source in &code_sources.sources {
            check_url(&source.url, &mut out.errors, "code-sources.toml url");
        }
    }
    if let Some(codes) = &ds.codes {
        for code in &codes.codes {
            for obs in &code.observations {
                check_url(&obs.url, &mut out.errors, "codes.toml observation url");
            }
        }
    }
    if let Some(banners) = &ds.banners {
        for banner in &banners.banners {
            check_url(&banner.source, &mut out.errors, "banners.toml source");
        }
    }
    if let Some(game) = &ds.game {
        for release in &game.releases {
            check_url(&release.source, &mut out.errors, "game.toml release source");
        }
    }
    if out.errors.len() == before {
        out.passed
            .push("G2 URLs: no placeholder (`<`, `…`) or empty URLs anywhere".to_string());
    }

    // -- G2: every optional roster fact carries its sources -------------
    let before = out.errors.len();
    for character in &ds.roster.characters {
        if character.released_on.is_some() && character.released_on_sources.is_empty() {
            out.errors.push(format!(
                "G2: {} has released_on but no released_on_sources",
                character.id
            ));
        }
        if character.rarity.is_some() && character.rarity_sources.is_empty() {
            out.errors.push(format!(
                "G2: {} has rarity but no rarity_sources",
                character.id
            ));
        }
        if character.pool.is_some() && character.pool_sources.is_empty() {
            out.errors
                .push(format!("G2: {} has pool but no pool_sources", character.id));
        }
    }
    if out.errors.len() == before {
        out.passed
            .push("G2 facts: every optional roster fact carries its sources".to_string());
    }

    // -- G2: game versions map to game.toml ------------------------------
    match &ds.game {
        Some(game) => {
            let before = out.errors.len();
            for source in &ds.sources.sources {
                if let Some(version) = &source.game_version {
                    let known = version == &game.current_version
                        || game.releases.iter().any(|r| &r.version == version);
                    if !known {
                        out.errors.push(format!(
                            "G2: source {} states game_version {version:?}, which is neither \
                             game.toml's current_version nor a recorded release",
                            source.id
                        ));
                    }
                }
            }
            if out.errors.len() == before {
                out.passed.push(
                    "G2 versions: every stated source game_version maps to game.toml".to_string(),
                );
            }
        }
        None => out
            .notes
            .push("G2 versions: game.toml absent — version mapping skipped until T2.3".to_string()),
    }

    // -- G3: raw tiers belong to the conversion tables --------------------
    let before = out.errors.len();
    let conversion = &ds.method.conversion;
    for p in &ds.pg.placements {
        for tier in [p.conquest, p.rift].into_iter().flatten() {
            if !conversion.pg.contains_key(tier.as_str()) {
                out.errors.push(format!(
                    "G3: pg tier {:?} for {} is not in the method conversion table",
                    tier.as_str(),
                    p.character
                ));
            }
        }
    }
    for p in &ds.kaiden.placements {
        for level in mongil_core::model::Level::ALL {
            if let Some(cell) = p.cell(level) {
                if !conversion.kaiden.contains_key(cell.tier.as_str()) {
                    out.errors.push(format!(
                        "G3: kaiden tier {:?} for {} is not in the method conversion table",
                        cell.tier.as_str(),
                        p.character
                    ));
                }
            }
        }
    }
    for p in &ds.jp.placements {
        if let Some(tier) = p.tier {
            if !conversion.gamewith_jp.contains_key(tier.as_str()) {
                out.errors.push(format!(
                    "G3: gamewith-jp tier {:?} for {} is not in the method conversion table",
                    tier.as_str(),
                    p.character
                ));
            }
        }
    }
    if out.errors.len() == before {
        out.passed.push(
            "G3 enums: every raw tier belongs to the method file's conversion tables".to_string(),
        );
    }

    // -- Source status sanity (Method §5 inputs) ---------------------------
    let voting_live = ds
        .sources
        .sources
        .iter()
        .filter(|s| s.role == mongil_core::model::SourceRole::Voting)
        .filter(|s| {
            !matches!(
                s.status,
                Some(SourceStatus::StaleCheckOnly) | Some(SourceStatus::Unavailable)
            )
        })
        .count();
    out.notes.push(format!(
        "voting sources currently casting votes: {voting_live} (staleness is data, Method §5)"
    ));

    // -- G10 (structural half): codes --------------------------------------
    if let Some(codes) = &ds.codes {
        let before = out.errors.len();
        let mut seen = std::collections::HashSet::new();
        for code in &codes.codes {
            if !seen.insert(code.code.as_str()) {
                out.errors
                    .push(format!("G10: duplicate code string {:?}", code.code));
            }
        }
        if let Some(code_sources) = &ds.code_sources {
            let known: std::collections::HashSet<&str> =
                code_sources.sources.iter().map(|s| s.id.as_str()).collect();
            for code in &codes.codes {
                for obs in &code.observations {
                    if !known.contains(obs.source.as_str()) {
                        out.errors.push(format!(
                            "G10: code {:?} has an observation from unknown source {:?}",
                            code.code, obs.source
                        ));
                    }
                }
                if !known.contains(code.reward_source.as_str()) {
                    out.errors.push(format!(
                        "G10: code {:?} has unknown reward_source {:?}",
                        code.code, code.reward_source
                    ));
                }
            }
        }
        if out.errors.len() == before {
            out.passed.push(
                "G10 codes (structural): no duplicate code strings; observation sources exist"
                    .to_string(),
            );
        }

        // G10 status core (T2.10): a code whose stated expiry is
        // already past cannot show an `active` observation at or after
        // that expiry. The full computed-status check (freshest
        // observation per source, region handling) lands with the
        // T2.7 codes engine.
        let before = out.errors.len();
        let now = now_epoch();
        for code in &codes.codes {
            if let Some(expires_at) = &code.expires_at {
                if let Some(expiry) = parse_rfc3339(expires_at) {
                    if expiry < now {
                        for obs in &code.observations {
                            if obs.status == CodeObservationStatus::Active {
                                if let Some(seen) = parse_rfc3339(&obs.seen_at) {
                                    if seen >= expiry {
                                        out.errors.push(format!(
                                            "G10: code {:?} shows active (source {}, seen {}) \
                                             at/after its stated expiry {}",
                                            code.code, obs.source, obs.seen_at, expires_at
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if out.errors.len() == before {
            out.passed.push(
                "G10 codes (expiry core): no code shows active at/after a past stated expiry"
                    .to_string(),
            );
        }
        out.notes.push(
            "G10 codes (computed-status half): statuses are computed by the T2.7 engine in \
             `emit` (build_codes, §B.3 rules, unit-tested there); validate checks the \
             structural + expiry cores above"
                .to_string(),
        );
    }

    // -- G11 (partial): check stamps ---------------------------------------
    if let Some(checks) = &ds.checks {
        let before = out.errors.len();
        check_stamp(
            &checks.sources_checked_at,
            &mut out.errors,
            "sources_checked_at",
        );
        check_stamp(
            &checks.codes_checked_at,
            &mut out.errors,
            "codes_checked_at",
        );
        if out.errors.len() == before {
            out.passed
                .push("G11 stamps: check stamps parse and are not in the future".to_string());
        }
        out.notes.push(
            "G11 stamps (monotonic half): compared against git history in CI from P2 on"
                .to_string(),
        );
    }

    // -- G11 (dates): banner, upcoming-event and errata dates -------------
    // The banner page compares these as strings and the tripwire reads
    // errata dates, so each must be a real date in the canonical shape.
    {
        let before = out.errors.len();
        if let Some(banners) = &ds.banners {
            for banner in &banners.banners {
                let where_ = format!("banners.toml {} ", banner.character);
                let start = check_day(&banner.start, &mut out.errors, &format!("{where_}start"));
                if let Some(end) = &banner.end {
                    let end_day = check_day(end, &mut out.errors, &format!("{where_}end"));
                    if let (Some(s), Some(e)) = (start, end_day) {
                        if e < s {
                            out.errors.push(format!(
                                "G11: {where_}ends ({end}) before it starts ({})",
                                banner.start
                            ));
                        }
                    }
                }
            }
            for event in &banners.upcoming {
                check_day(
                    &event.date,
                    &mut out.errors,
                    &format!("banners.toml upcoming {:?} date", event.title),
                );
            }
        }
        if let Some(errata) = &ds.errata {
            for e in &errata.errata {
                if parse_rfc3339(&e.found_at).is_none() {
                    out.errors.push(format!(
                        "G11: errata.toml {} found_at is not a real date: {:?}",
                        e.id, e.found_at
                    ));
                }
                if let Some(fixed) = &e.fixed_at {
                    if parse_rfc3339(fixed).is_none() {
                        out.errors.push(format!(
                            "G11: errata.toml {} fixed_at is not a real date: {fixed:?}",
                            e.id
                        ));
                    }
                }
            }
        }
        if out.errors.len() == before {
            out.passed.push(
                "G11 dates: banner, upcoming and errata dates are real YYYY-MM-DD dates"
                    .to_string(),
            );
        }
    }

    // -- G4 core + G5 report: versus the latest snapshot on disk ---------
    // (Full G4 diffs against origin/main; until the repo has a remote
    // the latest snapshot is the strongest baseline available, and it
    // is the baseline a sweep actually works from.)
    let snapshots = crate::snapshots::list_snapshots(data_dir);
    match snapshots.last() {
        None => {
            out.notes.push(
                "G4: no snapshot on disk yet — the coverage-drop comparison starts with the \
                 first snapshot (T2.9)"
                    .to_string(),
            );
            out.notes
                .push("G5: no snapshot on disk yet — no mass-change count to report".to_string());
        }
        Some((latest_id, latest)) => {
            let before = out.errors.len();
            let base_sets = snapshot_placed_sets(latest);
            let current_sets = placed_sets(&ds);
            for (source, base) in &base_sets {
                let current = current_sets.get(source);
                let current_count = current.map(|s| s.len()).unwrap_or(0);
                if base.len() > current_count && base.len() - current_count > 2 {
                    let dropped: Vec<&String> = base
                        .iter()
                        .filter(|c| !current.is_some_and(|s| s.contains(*c)))
                        .collect();
                    let unexplained: Vec<&&String> = dropped
                        .iter()
                        .filter(|c| !has_status_note(&ds, source, c))
                        .collect();
                    if !unexplained.is_empty() {
                        out.errors.push(format!(
                            "G4: source {source} placed count fell {} → {} vs snapshot \
                             {latest_id} (>2); dropped without a status + note: {}",
                            base.len(),
                            current_count,
                            unexplained
                                .iter()
                                .map(|c| c.as_str())
                                .collect::<Vec<_>>()
                                .join(", ")
                        ));
                    }
                }
            }
            if out.errors.len() == before {
                out.passed.push(format!(
                    "G4 coverage (core): no source's placed count fell by >2 vs snapshot \
                     {latest_id} without status + note explanations"
                ));
            }

            let working = build_snapshot(&ds, "working", "");
            let diff = diff_docs(latest, &working);
            let changed = diff.changed_characters();
            if diff.exceeds_mass_change() {
                out.notes.push(format!(
                    "G5 mass change: {changed} characters change Overall tier vs snapshot \
                     {latest_id} (> {}): this change set's lane is needs-tom (classification, \
                     not a failure)",
                    mongil_core::snapshot::MASS_CHANGE_LIMIT
                ));
            } else {
                out.notes.push(format!(
                    "G5 mass change: {changed} character(s) change Overall tier vs snapshot \
                     {latest_id} (lane threshold: > {})",
                    mongil_core::snapshot::MASS_CHANGE_LIMIT
                ));
            }
        }
    }

    // -- G6 core: double-read enforcement --------------------------------
    {
        let mut enforced = 0usize;
        let placements_dir = data_dir.join("placements");
        if let Ok(entries) = std::fs::read_dir(&placements_dir) {
            let mut files: Vec<_> = entries.flatten().map(|e| e.path()).collect();
            files.sort();
            for file in files {
                if file.extension().and_then(|e| e.to_str()) != Some("toml") {
                    continue;
                }
                let Ok(merged) = crate::harvest_diff::parse_harvest(&file) else {
                    continue; // G1 already reported an unloadable dataset
                };
                let Some(sweep) = merged.sweep.clone() else {
                    continue;
                };
                let sweep_dir = data_dir.join("harvests").join(&sweep);
                if !sweep_dir.is_dir() {
                    continue; // legacy transcription; enforcement starts
                              // with the first recorded sweep (T2.2)
                }
                enforced += 1;
                let a_path = sweep_dir.join(format!("{}.a.toml", merged.source));
                let b_path = sweep_dir.join(format!("{}.b.toml", merged.source));
                if !a_path.is_file() || !b_path.is_file() {
                    out.errors.push(format!(
                        "G6: placements for {} carry sweep {sweep} but data/harvests/{sweep}/ \
                         lacks the .a.toml/.b.toml pair",
                        merged.source
                    ));
                    continue;
                }
                let (Ok(a), Ok(b)) = (
                    crate::harvest_diff::parse_harvest(&a_path),
                    crate::harvest_diff::parse_harvest(&b_path),
                ) else {
                    out.errors.push(format!(
                        "G6: harvest pair for {} in sweep {sweep} does not parse",
                        merged.source
                    ));
                    continue;
                };
                let ab = crate::harvest_diff::compare(&a, &b);
                if ab.blocking > 0 {
                    out.errors.push(format!(
                        "G6: harvest-diff for {} in sweep {sweep} has {} blocking \
                         difference(s) — the two reads do not agree",
                        merged.source, ab.blocking
                    ));
                    continue;
                }
                let merged_vs_a = crate::harvest_diff::compare(&merged, &a);
                if merged_vs_a.blocking > 0 {
                    out.errors.push(format!(
                        "G6: merged placements for {} differ from worker A's structured \
                         fields in sweep {sweep} ({} blocking)",
                        merged.source, merged_vs_a.blocking
                    ));
                }
            }
        }
        if enforced > 0 && out.errors.is_empty() {
            out.passed.push(format!(
                "G6 double-read (core): {enforced} source(s) with a recorded sweep have a \
                 clean A/B harvest-diff and merged == A"
            ));
        }
        if enforced == 0 {
            out.notes.push(
                "G6 double-read (core): no recorded sweep on disk for the current placements \
                 — enforcement starts with the first recorded sweep (T2.2)"
                    .to_string(),
            );
        }
    }

    // -- G13: the lane classifier exists and is exercised by tests ----
    out.notes.push(
        "G13 lane: `lane` classifies change sets with all of T4.6's conditions (path \
         allowlist, roster additions only, snapshots new-files only, auto-status notes, \
         tripwire clear, not paused, G5 mass change) and is covered by the gate and lane \
         tests; the PR-result posting lands with the cadence workflows (T5.1)"
            .to_string(),
    );

    // -- G12 core / G14: only when the caller points at the outputs ------
    match dist_dir {
        Some(dist) => {
            let dist_errors = crate::dist_check::check_dist(data_dir, dist);
            if dist_errors.is_empty() {
                out.passed.push(
                    "G12 dist (core): every page carries the disclaimer, no draft note is \
                     rendered, internal links resolve, per-page JS ≤ 6 KB gzip; G14 dist \
                     images are inside img/own/ (allowlisted) or img/generated/"
                        .to_string(),
                );
            } else {
                out.errors.extend(dist_errors);
            }
        }
        None => out.notes.push(
            "G12 dist (core): pass --dist <dir> to check a built site (wired into the site \
             build in T3.10)"
                .to_string(),
        ),
    }
    match site_dir {
        Some(site) => {
            let image_errors = crate::dist_check::check_site_images(site);
            if image_errors.is_empty() {
                out.passed.push(
                    "G14 images: site/public/img holds only img/own/ (allowlisted) and \
                     img/generated/ content"
                        .to_string(),
                );
            } else {
                out.errors.extend(image_errors);
            }
        }
        None => out.notes.push(
            "G14 images: pass --site <dir> to check the image allowlist (CI does)".to_string(),
        ),
    }

    out.notes
        .push("G7/G8: method regression + invariants run as `cargo test` in CI".to_string());
    out.notes.push(
        "G9: snapshot immutability is a git check in the CI data job (method-file half applies \
         once launch_date exists)"
            .to_string(),
    );
    out.notes.push(
        "G4 (full form): diffing against origin/main replaces the latest-snapshot baseline \
         once the repo has its remote (T0.3)"
            .to_string(),
    );

    out
}

pub fn print_and_exit(out: &Outcome) -> std::process::ExitCode {
    for line in &out.passed {
        println!("PASS  {line}");
    }
    for line in &out.notes {
        println!("NOTE  {line}");
    }
    for line in &out.errors {
        eprintln!("FAIL  {line}");
    }
    if out.errors.is_empty() {
        println!("validate: OK ({} checks passed)", out.passed.len());
        std::process::ExitCode::SUCCESS
    } else {
        eprintln!("validate: {} check(s) failed", out.errors.len());
        std::process::ExitCode::FAILURE
    }
}
