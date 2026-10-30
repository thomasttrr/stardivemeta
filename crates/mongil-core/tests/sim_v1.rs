//! Regression tests from the 2026-10-29 simulation (plan T1.5, §B.6).
//! The fixture dataset + Method v1.0 must reproduce the simulation's 87
//! results exactly, forever. Expected values live in
//! `tests/fixtures/sim-v1-expected.json` (the §B.6 table, machine-readable;
//! Angel's label is MODE_SPLIT per the plan's X10 fix).

use mongil_core::derive;
use mongil_core::load::load_dataset;
use mongil_core::method::{compute, compute_modes, CharResult, LevelResult, ModeTier, TierBand};
use mongil_core::model::{Level, PlacementStatus};
use mongil_core::Dataset;
use serde::Deserialize;
use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;
use std::sync::OnceLock;

fn dataset() -> &'static Dataset {
    static DS: OnceLock<Dataset> = OnceLock::new();
    DS.get_or_init(|| {
        let dir =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/dataset-2026-10-29");
        load_dataset(&dir).expect("fixture dataset loads")
    })
}

fn levels() -> (LevelResult, LevelResult, LevelResult) {
    let ds = dataset();
    (
        compute(ds, &ds.method, Level::Low),
        compute(ds, &ds.method, Level::High),
        compute(ds, &ds.method, Level::Max),
    )
}

#[derive(Deserialize)]
struct Expected {
    characters: BTreeMap<String, ExpectedChar>,
}

#[derive(Deserialize)]
struct ExpectedChar {
    name: String,
    low: ExpectedResult,
    high: ExpectedResult,
    max: ExpectedResult,
}

#[derive(Deserialize)]
struct ExpectedResult {
    tier: String,
    label: String,
    score: f64,
    spread: f64,
    coverage: usize,
}

fn expected() -> &'static Expected {
    static EXP: OnceLock<Expected> = OnceLock::new();
    EXP.get_or_init(|| {
        serde_json::from_str(include_str!("fixtures/sim-v1-expected.json"))
            .expect("expected-results JSON parses")
    })
}

fn result_of<'a>(result: &'a LevelResult, id: &str) -> &'a CharResult {
    result
        .characters
        .iter()
        .find(|c| c.id == id)
        .unwrap_or_else(|| panic!("no result for {id}"))
}

fn name_of(result: &LevelResult, id: &str) -> String {
    result_of(result, id).name.clone()
}

fn check_character(id: &str) {
    let (low, high, max) = levels();
    let exp = &expected().characters[id];
    for (result, e) in [(&low, &exp.low), (&high, &exp.high), (&max, &exp.max)] {
        let r = result_of(result, id);
        let ctx = format!("{id} at {:?}", result.level);
        assert_eq!(r.name, exp.name, "{ctx}: name");
        assert_eq!(r.tier.as_str(), e.tier, "{ctx}: tier");
        assert_eq!(r.label.as_str(), e.label, "{ctx}: label");
        let score = r.score.expect("ranked character has a score");
        assert!(
            (score - e.score).abs() <= 0.005,
            "{ctx}: score {score} vs expected {}",
            e.score
        );
        assert_eq!(r.spread, Some(e.spread), "{ctx}: spread");
        assert_eq!(r.coverage, e.coverage, "{ctx}: coverage");
    }
}

macro_rules! char_test {
    ($name:ident, $id:expr) => {
        #[test]
        fn $name() {
            check_character($id);
        }
    };
}

char_test!(sim_v1_angel, "angel");
char_test!(sim_v1_benjamin, "benjamin");
char_test!(sim_v1_bonnie, "bonnie");
char_test!(sim_v1_brisshell, "brisshell");
char_test!(sim_v1_chloe, "chloe");
char_test!(sim_v1_cloud, "cloud");
char_test!(sim_v1_daisy, "daisy");
char_test!(sim_v1_ellie, "ellie");
char_test!(sim_v1_esther, "esther");
char_test!(sim_v1_flare, "flare");
char_test!(sim_v1_francis, "francis");
char_test!(sim_v1_francis_summer_dive, "francis-summer-dive");
char_test!(sim_v1_gabi, "gabi");
char_test!(sim_v1_isabella, "isabella");
char_test!(sim_v1_jiwon, "jiwon");
char_test!(sim_v1_leeho, "leeho");
char_test!(sim_v1_mabel, "mabel");
char_test!(sim_v1_mina, "mina");
char_test!(sim_v1_nagi, "nagi");
char_test!(sim_v1_narae, "narae");
char_test!(sim_v1_ophelia, "ophelia");
char_test!(sim_v1_penny, "penny");
char_test!(sim_v1_reina, "reina");
char_test!(sim_v1_sangun, "sangun");
char_test!(sim_v1_sera, "sera");
char_test!(sim_v1_verna, "verna");
char_test!(sim_v1_verna_summer_dive, "verna-summer-dive");
char_test!(sim_v1_vivian, "vivian");
char_test!(sim_v1_yeonhwa, "yeonhwa");

// ---------------------------------------------------------------------------
// Aggregate tests (§B.6 items 1–12)
// ---------------------------------------------------------------------------

fn count_by<T: PartialEq + Copy>(
    result: &LevelResult,
    pick: impl Fn(&CharResult) -> T,
    value: T,
) -> usize {
    result
        .characters
        .iter()
        .filter(|c| pick(c) == value)
        .count()
}

#[test]
fn sim_v1_band_counts_low() {
    let (low, _, _) = levels();
    assert_eq!(count_by(&low, |c| c.tier, TierBand::S), 4);
    assert_eq!(count_by(&low, |c| c.tier, TierBand::A), 6);
    assert_eq!(count_by(&low, |c| c.tier, TierBand::B), 11);
    assert_eq!(count_by(&low, |c| c.tier, TierBand::C), 6);
    assert_eq!(count_by(&low, |c| c.tier, TierBand::D), 1);
    assert_eq!(count_by(&low, |c| c.tier, TierBand::ModeSplit), 1);
    assert_eq!(count_by(&low, |c| c.tier, TierBand::Unranked), 0);
}

#[test]
fn sim_v1_band_counts_high() {
    let (_, high, _) = levels();
    assert_eq!(count_by(&high, |c| c.tier, TierBand::S), 7);
    assert_eq!(count_by(&high, |c| c.tier, TierBand::A), 6);
    assert_eq!(count_by(&high, |c| c.tier, TierBand::B), 8);
    assert_eq!(count_by(&high, |c| c.tier, TierBand::C), 6);
    assert_eq!(count_by(&high, |c| c.tier, TierBand::D), 1);
    assert_eq!(count_by(&high, |c| c.tier, TierBand::ModeSplit), 1);
    assert_eq!(count_by(&high, |c| c.tier, TierBand::Unranked), 0);
}

#[test]
fn sim_v1_band_counts_max() {
    let (_, _, max) = levels();
    assert_eq!(count_by(&max, |c| c.tier, TierBand::S), 7);
    assert_eq!(count_by(&max, |c| c.tier, TierBand::A), 7);
    assert_eq!(count_by(&max, |c| c.tier, TierBand::B), 8);
    assert_eq!(count_by(&max, |c| c.tier, TierBand::C), 5);
    assert_eq!(count_by(&max, |c| c.tier, TierBand::D), 1);
    assert_eq!(count_by(&max, |c| c.tier, TierBand::ModeSplit), 1);
    assert_eq!(count_by(&max, |c| c.tier, TierBand::Unranked), 0);
}

#[test]
fn sim_v1_label_counts() {
    use mongil_core::method::Label;
    let (low, high, max) = levels();
    // Low — the sim file's Solid 17 becomes 16 + 1 ModeSplit (X10).
    assert_eq!(count_by(&low, |c| c.label, Label::Solid), 16);
    assert_eq!(count_by(&low, |c| c.label, Label::Contested), 12);
    assert_eq!(count_by(&low, |c| c.label, Label::ModeSplit), 1);
    assert_eq!(count_by(&low, |c| c.label, Label::Speculation), 0);
    assert_eq!(count_by(&low, |c| c.label, Label::Unranked), 0);
    for result in [&high, &max] {
        assert_eq!(count_by(result, |c| c.label, Label::Solid), 24);
        assert_eq!(count_by(result, |c| c.label, Label::Contested), 4);
        assert_eq!(count_by(result, |c| c.label, Label::ModeSplit), 1);
    }
}

#[test]
fn sim_v1_contested_sets() {
    use mongil_core::method::Label;
    let (low, high, max) = levels();
    let contested = |result: &LevelResult| -> HashSet<String> {
        result
            .characters
            .iter()
            .filter(|c| c.label == Label::Contested)
            .map(|c| c.name.clone())
            .collect()
    };
    let low_expected: HashSet<String> = [
        "Esther",
        "Flare",
        "Francis",
        "Francis: Summer Dive!",
        "Gabi",
        "Jiwon",
        "Mina",
        "Nagi",
        "Ophelia",
        "Penny",
        "Sangun",
        "Yeonhwa",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    assert_eq!(contested(&low), low_expected);
    let high_expected: HashSet<String> = ["Flare", "Francis", "Sangun", "Yeonhwa"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(contested(&high), high_expected);
    assert_eq!(contested(&max), high_expected);
}

#[test]
fn sim_v1_board_order_low() {
    let (low, _, _) = levels();
    let board_names = |band: TierBand| -> Vec<String> {
        low.boards[&band]
            .iter()
            .map(|id| name_of(&low, id))
            .collect()
    };
    assert_eq!(
        board_names(TierBand::S),
        vec!["Narae", "Chloe", "Isabella", "Brisshell"]
    );
    assert_eq!(
        board_names(TierBand::A),
        vec![
            "Mina",
            "Verna: Summer Dive!",
            "Esther",
            "Francis",
            "Ophelia",
            "Francis: Summer Dive!"
        ]
    );
}

#[test]
fn sim_v1_d_is_daisy_only() {
    let (low, high, max) = levels();
    for result in [&low, &high, &max] {
        let d: Vec<String> = result.boards[&TierBand::D]
            .iter()
            .map(|id| name_of(result, id))
            .collect();
        assert_eq!(d, vec!["Daisy"]);
    }
}

#[test]
fn sim_v1_widest_spreads() {
    let (low, _, max) = levels();
    let sorted_spreads = |result: &LevelResult| -> Vec<(String, f64)> {
        let mut v: Vec<(String, f64)> = result
            .characters
            .iter()
            .map(|c| (c.name.clone(), c.spread.expect("spread")))
            .collect();
        v.sort_by(|a, b| {
            b.1.partial_cmp(&a.1)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.0.cmp(&b.0))
        });
        v
    };
    let low_spreads = sorted_spreads(&low);
    assert_eq!(low_spreads[0], ("Sangun".to_string(), 3.5));
    let next3: HashSet<(&str, i32)> = low_spreads[1..4]
        .iter()
        .map(|(n, s)| (n.as_str(), (*s * 2.0) as i32))
        .collect();
    assert_eq!(
        next3,
        HashSet::from([("Flare", 5), ("Jiwon", 5), ("Yeonhwa", 5)]),
        "Low: Flare / Jiwon / Yeonhwa at spread 2.5"
    );
    let max_spreads = sorted_spreads(&max);
    assert_eq!(max_spreads[0].1, 2.5);
    assert_eq!(max_spreads[1].1, 2.5);
    let top2: HashSet<&str> = max_spreads[..2].iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(top2, HashSet::from(["Sangun", "Yeonhwa"]));
}

#[test]
fn sim_v1_mode_conquest() {
    let modes = compute_modes(dataset());
    let names = |band: ModeTier| -> HashSet<String> {
        modes.conquest[&band]
            .iter()
            .map(|id| {
                dataset()
                    .roster
                    .characters
                    .iter()
                    .find(|c| &c.id == id)
                    .unwrap()
                    .name
                    .clone()
            })
            .collect()
    };
    assert_eq!(
        names(ModeTier::S),
        HashSet::from(["Chloe", "Esther", "Flare", "Mina", "Narae"].map(String::from))
    );
    assert_eq!(
        names(ModeTier::A),
        HashSet::from(
            ["Ellie", "Gabi", "Isabella", "Jiwon", "Ophelia", "Sangun"].map(String::from)
        )
    );
    assert_eq!(
        names(ModeTier::B),
        HashSet::from(["Benjamin", "Cloud", "Francis", "Penny", "Reina", "Sera"].map(String::from))
    );
    assert_eq!(
        names(ModeTier::C),
        HashSet::from(["Angel", "Bonnie", "Daisy", "Leeho", "Verna", "Yeonhwa"].map(String::from))
    );
    assert_eq!(
        names(ModeTier::Unranked),
        HashSet::from(
            [
                "Brisshell",
                "Francis: Summer Dive!",
                "Mabel",
                "Nagi",
                "Verna: Summer Dive!",
                "Vivian"
            ]
            .map(String::from)
        )
    );
}

#[test]
fn sim_v1_mode_rift() {
    let modes = compute_modes(dataset());
    let names = |band: ModeTier| -> HashSet<String> {
        modes.rift[&band]
            .iter()
            .map(|id| {
                dataset()
                    .roster
                    .characters
                    .iter()
                    .find(|c| &c.id == id)
                    .unwrap()
                    .name
                    .clone()
            })
            .collect()
    };
    assert_eq!(
        names(ModeTier::S),
        HashSet::from(
            ["Angel", "Chloe", "Isabella", "Jiwon", "Mina", "Narae", "Sangun"].map(String::from)
        )
    );
    assert_eq!(
        names(ModeTier::A),
        HashSet::from(["Benjamin", "Esther", "Flare", "Gabi", "Ophelia"].map(String::from))
    );
    assert_eq!(
        names(ModeTier::B),
        HashSet::from(["Cloud", "Ellie", "Francis", "Penny"].map(String::from))
    );
    assert_eq!(
        names(ModeTier::C),
        HashSet::from(
            ["Bonnie", "Daisy", "Leeho", "Reina", "Sera", "Verna", "Yeonhwa"].map(String::from)
        )
    );
    assert_eq!(
        names(ModeTier::Unranked),
        HashSet::from(
            [
                "Brisshell",
                "Francis: Summer Dive!",
                "Mabel",
                "Nagi",
                "Verna: Summer Dive!",
                "Vivian"
            ]
            .map(String::from)
        )
    );
}

#[test]
fn sim_v1_only_kaiden_moves() {
    let (low, high, max) = levels();
    let vote_of = |result: &LevelResult, id: &str, source: &str| -> Option<u8> {
        result_of(result, id)
            .votes
            .iter()
            .find(|v| v.source == source)
            .map(|v| v.value_h)
    };
    for character in &dataset().roster.characters {
        for source in ["pg", "gamewith-jp"] {
            let values = [
                vote_of(&low, &character.id, source),
                vote_of(&high, &character.id, source),
                vote_of(&max, &character.id, source),
            ];
            assert_eq!(
                values[0], values[1],
                "{}: {source} vote changed Low→High",
                character.id
            );
            assert_eq!(
                values[1], values[2],
                "{}: {source} vote changed High→Max",
                character.id
            );
        }
    }
}

#[test]
fn sim_v1_score_display() {
    let (low, high, _) = levels();
    assert_eq!(
        result_of(&low, "francis").score_display.as_deref(),
        Some("3.2")
    );
    assert_eq!(
        result_of(&low, "sangun").score_display.as_deref(),
        Some("1.8")
    );
    assert_eq!(
        result_of(&low, "ellie").score_display.as_deref(),
        Some("2.3")
    );
    assert_eq!(
        result_of(&high, "brisshell").score_display.as_deref(),
        Some("3.8")
    );
    assert_eq!(
        result_of(&low, "angel").score_display.as_deref(),
        Some("2.2")
    );
}

// ---------------------------------------------------------------------------
// Plus: the missing-is-not-zero proof and the T3.7 beginner lists (§B.6)
// ---------------------------------------------------------------------------

#[test]
fn sim_v1_missing_is_not_zero() {
    let ds = dataset();
    // Chloe has no Kaiden placement in the fixture (status, no cells).
    let chloe_kaiden = ds
        .kaiden
        .placements
        .iter()
        .find(|p| p.character == "chloe")
        .expect("chloe kaiden entry");
    assert_eq!(chloe_kaiden.status, Some(PlacementStatus::NoPlacement));
    assert!(chloe_kaiden.low.is_none());

    let (low, _, _) = levels();
    let chloe = result_of(&low, "chloe");
    assert_eq!(chloe.coverage, 2);
    assert!(chloe.votes.iter().all(|v| v.source != "kaiden"));

    // Giving her a Kaiden T4 instead changes the score to 8/3 ≈ 2.67 —
    // proof that "missing" and "zero" are different cases.
    let mut modified = ds.clone();
    let entry = modified
        .kaiden
        .placements
        .iter_mut()
        .find(|p| p.character == "chloe")
        .unwrap();
    let cell = mongil_core::model::KaidenLevelCell {
        tier: mongil_core::model::KaidenTier::T4,
        raw: "T4".to_string(),
        awakening: None,
        flags: vec![],
    };
    entry.low = Some(cell.clone());
    entry.high = Some(cell.clone());
    entry.max = Some(cell);
    entry.status = None;
    let low2 = compute(&modified, &modified.method, Level::Low);
    let chloe2 = result_of(&low2, "chloe");
    assert_eq!(chloe2.coverage, 3);
    let score = chloe2.score.unwrap();
    assert!(
        (score - 2.67).abs() <= 0.005,
        "with a T4 vote Chloe scores {score}, expected ≈2.67"
    );
}

#[test]
fn derive_beginner_v1() {
    let ds = dataset();
    let lists = derive::beginner(ds, &ds.method);
    let names = |ids: &[String]| -> Vec<String> {
        ids.iter()
            .map(|id| {
                ds.roster
                    .characters
                    .iter()
                    .find(|c| &c.id == id)
                    .unwrap()
                    .name
                    .clone()
            })
            .collect()
    };
    // Agreed at Low, in board order (T3.7).
    assert_eq!(
        names(&lists.agreed_low),
        vec![
            "Narae",
            "Chloe",
            "Isabella",
            "Brisshell",
            "Verna: Summer Dive!"
        ]
    );
    // Rise with investment (T3.7 lists no order rule; compared as a set).
    let rise_names: HashSet<String> = lists
        .rise
        .iter()
        .map(|r| name_of(&levels().0, &r.id))
        .collect();
    assert_eq!(
        rise_names,
        HashSet::from(
            [
                "Francis: Summer Dive!",
                "Nagi",
                "Vivian",
                "Jiwon",
                "Mabel",
                "Mina",
                "Sangun",
                "Esther"
            ]
            .map(String::from)
        )
    );
    // Disagree at Low = the 12 Contested (§B.6 contested set).
    let contested_names: HashSet<String> = names(&lists.contested_low).into_iter().collect();
    assert_eq!(contested_names.len(), 12);
    assert!(contested_names.contains("Sangun") && contested_names.contains("Mina"));
    // No pool data in the fixture → no reroll section (plan D-3).
    assert_eq!(lists.reroll, None);
}
