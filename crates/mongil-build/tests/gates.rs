//! Gate tests (plan §B.7, T2.10): one failing-fixture test per gate,
//! each proving the gate rejects bad input. Run with:
//! `cargo test -p mongil-build gates`.

mod common;

use common::{
    base_files, dataset_plus, dataset_with, git, git_init_repo, run_bin, temp_dir,
    write_dataset_vec, write_files, KAIDEN, METHOD, PG, ROSTER, SOURCES,
};
use std::path::PathBuf;

fn validate(dir: &std::path::Path, extra: &[&str]) -> common::BinOutput {
    let mut args = vec!["validate", "--data", dir.to_str().unwrap()];
    args.extend(extra);
    run_bin(&args, None)
}

fn expect_gate_failure(name: &str, out: &common::BinOutput, gate: &str) {
    assert_eq!(
        out.code,
        Some(1),
        "{name}: expected validate to fail; output: {}",
        out.combined()
    );
    assert!(
        out.combined().contains(gate),
        "{name}: expected a {gate} failure; output: {}",
        out.combined()
    );
}

/// G1 — schema: an unknown field anywhere fails the load.
#[test]
fn gates_g1_schema_rejects_unknown_field() {
    let roster = format!("{ROSTER}\nbogus_top_level = true\n");
    let dir = dataset_with("g1", "roster.toml", &roster);
    let out = validate(&dir, &[]);
    expect_gate_failure("G1", &out, "G1");
}

/// G2 — references & hygiene: a placeholder URL is rejected.
#[test]
fn gates_g2_rejects_placeholder_url() {
    let sources = SOURCES.replace(
        "https://www.pocketgamer.com/mongil-star-dive/tier-list/",
        "https://<todo>/",
    );
    let dir = dataset_with("g2", "sources.toml", &sources);
    let out = validate(&dir, &[]);
    expect_gate_failure("G2", &out, "G2");
}

/// G3 — enums: a raw tier missing from the method's conversion table
/// is rejected (here the data uses C but the method file drops it).
#[test]
fn gates_g3_rejects_tier_missing_from_conversion() {
    let method = METHOD.replace(
        "B = 2.0\nC = 1.0\n[conversion.gamewith-jp]",
        "B = 2.0\n[conversion.gamewith-jp]",
    );
    assert_ne!(method, METHOD);
    let pg = PG.replace(
        "conquest = \"S\"\nrift = \"S\"",
        "conquest = \"C\"\nrift = \"C\"",
    );
    let mut files = base_files();
    for entry in files.iter_mut() {
        if entry.0 == "method.toml" {
            entry.1 = method.clone();
        }
        if entry.0 == "placements/pg.toml" {
            entry.1 = pg.clone();
        }
    }
    let dir = write_dataset_vec("g3", &files);
    let out = validate(&dir, &[]);
    expect_gate_failure("G3", &out, "G3");
}

/// G4 — coverage drop: a source placing 5 characters in the baseline
/// snapshot and 2 in the working data, with no status+note on the
/// dropped entries, is rejected.
#[test]
fn gates_g4_rejects_unexplained_coverage_drop() {
    let roster5 = r#"
[[character]]
id = "aa"
name = "Aa"
[[character]]
id = "bb"
name = "Bb"
[[character]]
id = "cc"
name = "Cc"
[[character]]
id = "dd"
name = "Dd"
[[character]]
id = "ee"
name = "Ee"
"#;
    let place_all = |chars: &[&str]| -> String {
        let mut s = String::new();
        for c in chars {
            s.push_str(&format!(
                "[[placement]]\ncharacter = \"{c}\"\nconquest = \"S\"\nrift = \"S\"\n"
            ));
        }
        s
    };
    let kaiden_aa = r#"
[[placement]]
character = "aa"
low = { tier = "T0", raw = "T0" }
high = { tier = "T0", raw = "T0" }
max = { tier = "T0", raw = "T0" }
"#;
    let jp_aa = r#"
[[placement]]
character = "aa"
tier = "SS"
raw = "SS"
"#;
    let dir = common::temp_dir("g4");
    write_files(
        &dir,
        &[
            ("roster.toml", roster5),
            ("sources.toml", SOURCES),
            ("method.toml", METHOD),
            (
                "placements/pg.toml",
                &place_all(&["aa", "bb", "cc", "dd", "ee"]),
            ),
            ("placements/kaiden.toml", kaiden_aa),
            ("placements/gamewith-jp.toml", jp_aa),
        ],
    );
    let snap = run_bin(
        &[
            "snapshot",
            "--data",
            dir.to_str().unwrap(),
            "--id",
            "2026-11-03-am",
        ],
        None,
    );
    assert_eq!(snap.code, Some(0), "snapshot: {}", snap.combined());

    // Drop three of five PG placements with no explanation.
    write_files(&dir, &[("placements/pg.toml", &place_all(&["aa", "bb"]))]);
    let out = validate(&dir, &[]);
    expect_gate_failure("G4", &out, "G4");
}

/// G5 — mass change: a diff where > 8 characters change tier is
/// classified over the limit (the lane's needs-tom trigger).
#[test]
fn gates_g5_mass_change_threshold() {
    fn dataset_n(name: &str, strong: bool) -> mongil_core::load::Dataset {
        let n = 10;
        let mut roster = String::new();
        let mut pg = String::new();
        let mut kaiden = String::new();
        let mut jp = String::new();
        for i in 1..=n {
            let id = format!("c{i:02}");
            roster.push_str(&format!("[[character]]\nid = \"{id}\"\nname = \"C{i}\"\n"));
            if strong {
                pg.push_str(&format!(
                    "[[placement]]\ncharacter = \"{id}\"\nconquest = \"S\"\nrift = \"S\"\n"
                ));
                kaiden.push_str(&format!(
                    "[[placement]]\ncharacter = \"{id}\"\nlow = {{ tier = \"T0\", raw = \"T0\" }}\nhigh = {{ tier = \"T0\", raw = \"T0\" }}\nmax = {{ tier = \"T0\", raw = \"T0\" }}\n"
                ));
                jp.push_str(&format!(
                    "[[placement]]\ncharacter = \"{id}\"\ntier = \"SS\"\nraw = \"SS\"\n"
                ));
            } else {
                pg.push_str(&format!(
                    "[[placement]]\ncharacter = \"{id}\"\nconquest = \"C\"\nrift = \"C\"\n"
                ));
                kaiden.push_str(&format!(
                    "[[placement]]\ncharacter = \"{id}\"\nlow = {{ tier = \"T4\", raw = \"T4\" }}\nhigh = {{ tier = \"T4\", raw = \"T4\" }}\nmax = {{ tier = \"T4\", raw = \"T4\" }}\n"
                ));
                jp.push_str(&format!(
                    "[[placement]]\ncharacter = \"{id}\"\ntier = \"A\"\nraw = \"A\"\n"
                ));
            }
        }
        let dir = common::temp_dir(name);
        write_files(
            &dir,
            &[
                ("roster.toml", &roster),
                ("sources.toml", SOURCES),
                ("method.toml", METHOD),
                ("placements/pg.toml", &pg),
                ("placements/kaiden.toml", &kaiden),
                ("placements/gamewith-jp.toml", &jp),
            ],
        );
        mongil_core::load_dataset(&dir).unwrap()
    }

    let a = mongil_core::snapshot::build_snapshot(&dataset_n("g5-a", true), "2026-11-03-am", "");
    let b = mongil_core::snapshot::build_snapshot(&dataset_n("g5-b", false), "2026-11-03-pm", "");
    let diff = mongil_core::snapshot::diff_docs(&a, &b);
    assert_eq!(diff.changed_characters(), 10);
    assert!(
        diff.exceeds_mass_change(),
        "10 changed characters must exceed the G5 limit of {}",
        mongil_core::snapshot::MASS_CHANGE_LIMIT
    );
    // A diff of a snapshot against itself changes nothing.
    let same = mongil_core::snapshot::diff_docs(&a, &a);
    assert_eq!(same.changed_characters(), 0);
    assert!(!same.exceeds_mass_change());
}

/// G6 — double read: a recorded sweep whose A and B reads disagree
/// blocks validation.
#[test]
fn gates_g6_rejects_disagreeing_double_read() {
    let meta =
        |source: &str| format!("[meta]\nsource = \"{source}\"\nharvest = \"2026-11-03-am\"\n");
    let pg = format!("{}{PG}", meta("pg"));
    let kaiden = format!("{}{KAIDEN}", meta("kaiden"));
    let jp = format!("{}{}", meta("gamewith-jp"), common::JP);
    let mut files = base_files();
    for entry in files.iter_mut() {
        match entry.0.as_str() {
            "placements/pg.toml" => entry.1 = pg.clone(),
            "placements/kaiden.toml" => entry.1 = kaiden.clone(),
            "placements/gamewith-jp.toml" => entry.1 = jp.clone(),
            _ => {}
        }
    }
    let dir = write_dataset_vec("g6", &files);

    // Harvest pairs: pg and jp agree; kaiden's B read has one tier off.
    let kaiden_b = kaiden.replace(
        "high = { tier = \"T0.5\", raw = \"T0.5\" }",
        "high = { tier = \"T1\", raw = \"T0.5\" }",
    );
    assert_ne!(kaiden_b, kaiden);
    write_files(
        &dir,
        &[
            ("harvests/2026-11-03-am/pg.a.toml", &pg),
            ("harvests/2026-11-03-am/pg.b.toml", &pg),
            ("harvests/2026-11-03-am/kaiden.a.toml", &kaiden),
            ("harvests/2026-11-03-am/kaiden.b.toml", &kaiden_b),
            ("harvests/2026-11-03-am/gamewith-jp.a.toml", &jp),
            ("harvests/2026-11-03-am/gamewith-jp.b.toml", &jp),
        ],
    );
    let out = validate(&dir, &[]);
    expect_gate_failure("G6", &out, "G6");
}

/// Compare computed results against a B.6-style expected table.
/// Returns the number of mismatches.
fn regression_mismatches(expected: &serde_json::Value, tamper: bool) -> usize {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../mongil-core/tests/fixtures/dataset-2026-10-29");
    let ds = mongil_core::load_dataset(&fixture).unwrap();
    let mut expected = expected.clone();
    if tamper {
        expected["characters"]["mina"]["low"]["tier"] = serde_json::Value::String("D".to_string());
    }
    let mut mismatches = 0;
    for level in mongil_core::model::Level::ALL {
        let result = mongil_core::method::compute(&ds, &ds.method, level);
        for c in &result.characters {
            let e = &expected["characters"][&c.id][level.as_str()];
            let expected_tier = e["tier"].as_str().unwrap();
            let expected_label = e["label"].as_str().unwrap();
            if expected_tier == "MODE_SPLIT" {
                if c.label.as_str() != "MODE_SPLIT" {
                    mismatches += 1;
                }
            } else if c.tier.as_str() != expected_tier || c.label.as_str() != expected_label {
                mismatches += 1;
            }
        }
    }
    mismatches
}

/// G7 — method regression: the frozen B.6 expected table matches the
/// engine exactly, and tampering with an expectation is detected.
#[test]
fn gates_g7_method_regression_bites() {
    let expected_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../mongil-core/tests/fixtures/sim-v1-expected.json");
    let expected: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&expected_path).unwrap()).unwrap();
    assert_eq!(
        regression_mismatches(&expected, false),
        0,
        "the B.6 regression table must match the engine"
    );
    assert!(
        regression_mismatches(&expected, true) > 0,
        "a tampered expectation must be detected"
    );
}

/// G8 — invariants: result invariants hold on real output and are
/// violated by tampered results.
#[test]
fn gates_g8_result_invariants_bite() {
    fn violations(
        r: &mongil_core::method::CharResult,
        method: &mongil_core::MethodConfig,
    ) -> Vec<String> {
        let mut v = Vec::new();
        if r.coverage != r.votes.len() {
            v.push("coverage != votes.len()".into());
        }
        match r.score {
            None => {
                if r.coverage > 0 {
                    v.push("ranked character without a score".into());
                }
                if r.label.as_str() != "UNRANKED" {
                    v.push("unranked character with a label".into());
                }
            }
            Some(score) => {
                if !(0.0..=4.0).contains(&score) {
                    v.push(format!("score {score} outside [0, 4]"));
                }
                if r.mode_split.is_none() {
                    let band = if score >= method.s_min {
                        "S"
                    } else if score >= method.a_min {
                        "A"
                    } else if score >= method.b_min {
                        "B"
                    } else if score >= method.c_min {
                        "C"
                    } else {
                        "D"
                    };
                    if r.tier.as_str() != band {
                        v.push(format!(
                            "tier {} inconsistent with score {score}",
                            r.tier.as_str()
                        ));
                    }
                }
                let expected_label = if r.mode_split.is_some() {
                    "MODE_SPLIT"
                } else if r.coverage == 1 {
                    "SPECULATION"
                } else if r
                    .spread
                    .map(|s| s <= method.solid_max_spread + 1e-9)
                    .unwrap_or(false)
                {
                    "SOLID"
                } else {
                    "CONTESTED"
                };
                if r.label.as_str() != expected_label {
                    v.push(format!(
                        "label {} inconsistent (expected {expected_label})",
                        r.label.as_str()
                    ));
                }
            }
        }
        v
    }

    let dir = dataset_with("g8", "roster.toml", common::ROSTER);
    let ds = mongil_core::load_dataset(&dir).unwrap();
    let result = mongil_core::method::compute(&ds, &ds.method, mongil_core::model::Level::Low);
    let mina = result.characters.iter().find(|c| c.id == "mina").unwrap();
    assert!(
        violations(mina, &ds.method).is_empty(),
        "real engine output must satisfy the §B.6 invariants"
    );

    let mut bad_score = mina.clone();
    bad_score.score = Some(4.5);
    assert!(!violations(&bad_score, &ds.method).is_empty());

    let mut bad_coverage = mina.clone();
    bad_coverage.coverage = 99;
    assert!(!violations(&bad_coverage, &ds.method).is_empty());

    let mut bad_tier = mina.clone();
    bad_tier.tier = mongil_core::method::TierBand::D;
    assert!(!violations(&bad_tier, &ds.method).is_empty());
}

/// G9 — immutability: the CI git check detects a modified snapshot.
#[test]
fn gates_g9_snapshot_immutability_detected() {
    let dir = temp_dir("g9");
    git_init_repo(&dir);
    write_files(&dir, &[("data/snapshots/2026-11-03-am.json", "{}\n")]);
    git(&dir, &["add", "-A"]);
    git(&dir, &["commit", "-qm", "baseline"]);

    let check = |dir: &PathBuf| {
        git(
            dir,
            &[
                "diff",
                "--diff-filter=MDR",
                "--name-only",
                "HEAD",
                "--",
                "data/snapshots/",
            ],
        )
    };
    assert!(check(&dir).trim().is_empty(), "clean tree must pass G9");
    write_files(
        &dir,
        &[("data/snapshots/2026-11-03-am.json", "{\"x\":1}\n")],
    );
    let detected = check(&dir);
    assert!(
        detected.contains("2026-11-03-am.json"),
        "a modified snapshot must be detected, got: {detected}"
    );
}

/// G10 — codes: a code showing `active` after its stated expiry is
/// rejected.
#[test]
fn gates_g10_rejects_active_after_expiry() {
    let code_sources = r#"
[[code_source]]
id = "official-x"
name = "Official X"
group = "official"
url = "https://example.com/codes"
"#;
    let codes = r#"
[[code]]
code = "TESTCODE"
reward = "100 Astrite"
reward_source = "official-x"
expires_at = "2020-01-01T00:00:00Z"
expires_source = "official-x"
[[code.observation]]
source = "official-x"
status = "active"
seen_at = "2020-01-02T00:00:00Z"
url = "https://example.com/codes"
"#;
    let mut files = base_files();
    files.push(("code-sources.toml".into(), code_sources.into()));
    files.push(("codes.toml".into(), codes.into()));
    let dir = write_dataset_vec("g10", &files);
    let out = validate(&dir, &[]);
    expect_gate_failure("G10", &out, "G10");
}

/// G11 — stamps: a check stamp in the future is rejected.
#[test]
fn gates_g11_rejects_future_stamp() {
    let checks = r#"{"codes_checked_at":"2999-01-01T00:00:00Z","last_sweep":"2026-11-03-am","sources_checked_at":"2026-11-03T00:00:00Z"}"#;
    let dir = dataset_plus("g11", "state/checks.json", checks);
    let out = validate(&dir, &[]);
    expect_gate_failure("G11", &out, "G11");
}

/// G12 — dist content (core): a built page without the disclaimer is
/// rejected.
#[test]
fn gates_g12_rejects_page_without_disclaimer() {
    let dir = dataset_with("g12", "roster.toml", common::ROSTER);
    let dist = temp_dir("g12-dist");
    write_files(
        &dist,
        &[(
            "index.html",
            "<html><body><p>No disclaimer here.</p></body></html>",
        )],
    );
    let out = validate(&dir, &["--dist", dist.to_str().unwrap()]);
    expect_gate_failure("G12", &out, "G12");
}

/// G13 — lane: the classifier calls a code change `needs-tom` and a
/// data-only change `data-only` (only `data-only` may auto-merge).
#[test]
fn gates_g13_lane_classifies_change_sets() {
    let dir = temp_dir("g13");
    git_init_repo(&dir);
    write_files(
        &dir,
        &[
            ("crates/mongil-build/src/lib.rs", "// code\n"),
            ("data/placements/pg.toml", "# data\n"),
        ],
    );
    git(&dir, &["add", "-A"]);
    git(&dir, &["commit", "-qm", "baseline"]);

    write_files(&dir, &[("crates/mongil-build/src/lib.rs", "// changed\n")]);
    let out = run_bin(&["lane", "--base", "HEAD"], Some(&dir));
    assert_eq!(out.code, Some(0), "lane output: {}", out.combined());
    assert_eq!(out.stdout.lines().next(), Some("needs-tom"));
    git(&dir, &["checkout", "--", "crates"]);

    write_files(&dir, &[("data/placements/pg.toml", "# changed\n")]);
    let out = run_bin(&["lane", "--base", "HEAD"], Some(&dir));
    assert_eq!(out.code, Some(0), "lane output: {}", out.combined());
    assert_eq!(out.stdout.lines().next(), Some("data-only"));
}

/// G14 — images: an image outside img/own/ and img/generated/ is
/// rejected.
#[test]
fn gates_g14_rejects_stray_image() {
    let dir = dataset_with("g14", "roster.toml", common::ROSTER);
    let site = temp_dir("g14-site");
    write_files(&site, &[("public/img/stray.png", "not-really-a-png")]);
    let out = validate(&dir, &["--site", site.to_str().unwrap()]);
    expect_gate_failure("G14", &out, "G14");
}

/// G11 — stamps: a stamp of the right shape for a date/time that does
/// not exist is rejected (bug hunt #14).
#[test]
fn gates_g11_rejects_impossible_stamp() {
    let checks = r#"{"codes_checked_at":"2026-02-30T25:61:61Z","last_sweep":"2026-11-03-am","sources_checked_at":"2026-02-30T25:61:61Z"}"#;
    let dir = dataset_plus("g11-impossible", "state/checks.json", checks);
    let out = validate(&dir, &[]);
    expect_gate_failure("G11", &out, "is not a real date and time");
}

/// G11 — dates: a banner or upcoming-event date that is not a real
/// YYYY-MM-DD date is rejected (bug hunt #24).
#[test]
fn gates_g11_rejects_bad_banner_dates() {
    for banners in [
        "[[banner]]\ncharacter = \"mina\"\nstart = \"2026-11-5\"\nsource = \"https://example.com/b\"\n",
        "[[banner]]\ncharacter = \"mina\"\nstart = \"2026-11-05\"\nend = \"2026-02-30\"\nsource = \"https://example.com/b\"\n",
        "[[banner]]\ncharacter = \"mina\"\nstart = \"2026-11-05\"\nend = \"2026-11-01\"\nsource = \"https://example.com/b\"\n",
        "[[upcoming]]\ntitle = \"Stream\"\ndate = \"Nov 10\"\nnote = \"n\"\nsource = \"Official\"\n",
    ] {
        let dir = dataset_plus("g11-banner", "banners.toml", banners);
        let out = validate(&dir, &[]);
        expect_gate_failure("G11", &out, "G11: banners.toml");
    }
}

/// G12 — completeness: a dist with no pages (or missing the required
/// home, 404 and character pages) is rejected (bug hunt #23).
#[test]
fn gates_g12_rejects_empty_dist() {
    let dir = dataset_with("g12-empty", "roster.toml", common::ROSTER);
    let dist = temp_dir("g12-empty-dist");
    write_files(&dist, &[("img/generated/og.png", "png")]);
    let out = validate(&dir, &["--dist", dist.to_str().unwrap()]);
    expect_gate_failure("G12", &out, "contains no HTML pages");
    assert!(
        out.combined()
            .contains("missing the required page c/mina/index.html"),
        "output: {}",
        out.combined()
    );
}

/// G14 — images: with ALLOWLIST.txt missing, an img/own/ image fails
/// instead of passing unchecked (bug hunt #13).
#[test]
fn gates_g14_missing_allowlist_fails_closed() {
    let dir = dataset_with("g14-closed", "roster.toml", common::ROSTER);
    let site = temp_dir("g14-closed-site");
    write_files(&site, &[("public/img/own/ripped.png", "not-really-a-png")]);
    let out = validate(&dir, &["--site", site.to_str().unwrap()]);
    expect_gate_failure("G14", &out, "ALLOWLIST.txt is missing");
}
