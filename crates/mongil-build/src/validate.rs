//! `mongil-build validate` (plan T1.6 + §B.7 gates).
//!
//! Implemented now: G1 (schema — the loader parses every file with
//! `deny_unknown_fields` and resolves references), G2 (slugs, URL
//! hygiene, roster-fact sources, game-version mapping), G3 (raw tiers
//! belong to the method file's conversion tables), and the structural
//! halves of G10/G11 (code duplicates and observation sources; check
//! stamps parse and are not in the future).
//!
//! Deferred, and printed as notes so the gap is visible rather than
//! silent: G4 + G6 (need harvest-diff / origin-main, T2.1 + P2),
//! G5 + G13 (the `lane` subcommand, T4.6), G7 + G8 (`cargo test` in the
//! CI `rust` job), G9 (git check in the CI `data` job; the method-file
//! half applies only once `launch_date` exists, T4.6), the status half
//! of G10 (codes engine, T2.7), and G12 + G14 (dist checks, T3.10).

use mongil_core::load::Dataset;
use mongil_core::model::SourceStatus;
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
    let now = crate::emit::now_iso_utc();
    if value > now.as_str() {
        errors.push(format!("G11: {where_} is in the future: {value} > {now}"));
    }
}

pub fn run(data_dir: &Path) -> Outcome {
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
        out.notes.push(
            "G10 codes (status half): computed statuses are checked once the T2.7 codes engine \
             lands"
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

    out.notes.push(
        "G4/G6: coverage-drop and double-read gates land with harvest-diff in T2.1".to_string(),
    );
    out.notes.push(
        "G5/G13: mass-change and lane checks land with the lane subcommand in T4.6".to_string(),
    );
    out.notes
        .push("G7/G8: method regression + invariants run as `cargo test` in CI".to_string());
    out.notes.push(
        "G9: snapshot immutability is a git check in the CI data job (method-file half applies \
         once launch_date exists)"
            .to_string(),
    );
    out.notes
        .push("G12/G14: dist content + image allowlist checks land in T3.10".to_string());

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
