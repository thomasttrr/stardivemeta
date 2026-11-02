//! Method v1.0 engine (plan T1.4). No floating-point maths in any
//! decision: votes are integer half-points and thresholds are compared in
//! integer milli-points (1 point = 1000). With the v1.0 constants the
//! milli-point comparisons reduce exactly to the plan's step-3 formulas
//! (`sum_h >= 7n` for S, `>= 6n` for A, `>= 4n` for B, `>= 2n` for C, and
//! `spread_h <= 2` for Solid); the general form keeps the tests honest if
//! a constant is ever edited (see the plan's T1.5 "tests bite" check).
//!
//! All constants are read from `MethodConfig` (plan T1.4 step 9).

use crate::load::Dataset;
use crate::model::*;
use serde::Serialize;
use std::collections::BTreeMap;

/// Convert a points value from the method file to milli-points.
fn milli(points: f64) -> i64 {
    (points * 1000.0).round() as i64
}

/// Points → integer half-points. Conversion tables only ever hold whole
/// or half points (0.0, 0.5, …, 4.0); `load_dataset` rejects anything
/// else, and this assert holds in release builds too.
fn half_points(points: f64) -> u8 {
    let doubled = points * 2.0;
    assert!(
        (doubled - doubled.round()).abs() < 1e-9,
        "conversion value {points} is not a whole number of half-points"
    );
    doubled.round() as u8
}

fn conversion_h(table: &BTreeMap<String, f64>, key: &str) -> u8 {
    let points = table
        .get(key)
        .unwrap_or_else(|| panic!("method conversion table is missing \"{key}\""));
    half_points(*points)
}

pub fn pg_tier_h(tier: PgTier, method: &MethodConfig) -> u8 {
    conversion_h(&method.conversion.pg, tier.as_str())
}

pub fn jp_tier_h(tier: JpTier, method: &MethodConfig) -> u8 {
    conversion_h(&method.conversion.gamewith_jp, tier.as_str())
}

pub fn kaiden_tier_h(tier: KaidenTier, method: &MethodConfig) -> u8 {
    conversion_h(&method.conversion.kaiden, tier.as_str())
}

// ---------------------------------------------------------------------------
// Output types
// ---------------------------------------------------------------------------

/// Overall tier band (plus the two non-band outcomes). String form matches
/// the contract's `Tier` type (§B.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub enum TierBand {
    S,
    A,
    B,
    C,
    D,
    ModeSplit,
    Unranked,
}

impl TierBand {
    pub fn as_str(&self) -> &'static str {
        match self {
            TierBand::S => "S",
            TierBand::A => "A",
            TierBand::B => "B",
            TierBand::C => "C",
            TierBand::D => "D",
            TierBand::ModeSplit => "MODE_SPLIT",
            TierBand::Unranked => "UNRANKED",
        }
    }
}

/// Confidence label. String form matches the contract's `Label` type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub enum Label {
    Solid,
    Contested,
    Speculation,
    ModeSplit,
    Unranked,
}

impl Label {
    pub fn as_str(&self) -> &'static str {
        match self {
            Label::Solid => "SOLID",
            Label::Contested => "CONTESTED",
            Label::Speculation => "SPECULATION",
            Label::ModeSplit => "MODE_SPLIT",
            Label::Unranked => "UNRANKED",
        }
    }
}

/// One counted vote, in integer half-points. `raw` is what the source
/// published, for the evidence view.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct VoteOut {
    pub source: String,
    pub value_h: u8,
    pub raw: String,
}

impl VoteOut {
    /// Display-only points value (never used in a decision).
    pub fn value(&self) -> f64 {
        self.value_h as f64 / 2.0
    }
}

/// A character's computed result at one investment level.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CharResult {
    pub id: String,
    pub name: String,
    pub tier: TierBand,
    pub label: Label,
    /// Mean of the votes, for display only. `None` when unranked.
    pub score: Option<f64>,
    pub score_display: Option<String>,
    pub coverage: usize,
    /// Highest vote − lowest vote, in points. `None` when unranked.
    pub spread: Option<f64>,
    pub votes: Vec<VoteOut>,
    /// `(conquest_h, rift_h)` when the mode-split override fired.
    pub mode_split: Option<(u8, u8)>,
}

/// Everything computed for one investment level: per-character results in
/// roster order, plus the ordered boards (plan T1.4 step 6).
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LevelResult {
    pub level: Level,
    pub characters: Vec<CharResult>,
    pub boards: BTreeMap<TierBand, Vec<String>>,
}

/// A tier on a per-mode board (PG publishes S–C; no D).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub enum ModeTier {
    S,
    A,
    B,
    C,
    Unranked,
}

impl ModeTier {
    pub fn as_str(&self) -> &'static str {
        match self {
            ModeTier::S => "S",
            ModeTier::A => "A",
            ModeTier::B => "B",
            ModeTier::C => "C",
            ModeTier::Unranked => "UNRANKED",
        }
    }
}

/// Per-mode results (plan T1.4 step 8): PG's placements only — the mode
/// views are not a consensus and are never dressed up as one.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ModeResults {
    pub conquest: BTreeMap<ModeTier, Vec<String>>,
    pub rift: BTreeMap<ModeTier, Vec<String>>,
    /// `(character id, conquest tier, rift tier)` in roster order.
    pub per_character: Vec<(String, ModeTier, ModeTier)>,
}

// ---------------------------------------------------------------------------
// Rule functions (plan T1.4 steps 1–8; one small function per rule)
// ---------------------------------------------------------------------------

/// Step 1: PG's Overall vote from its two mode tiers. PG tiers are whole
/// points, so in half-points each value is even and the average is exact.
/// With only one mode, that mode is the vote.
pub fn pg_overall_h(
    conquest: Option<PgTier>,
    rift: Option<PgTier>,
    method: &MethodConfig,
) -> Option<u8> {
    match (conquest, rift) {
        (Some(c), Some(r)) => {
            let sum = pg_tier_h(c, method) as u16 + pg_tier_h(r, method) as u16;
            debug_assert!(
                sum.is_multiple_of(2),
                "PG half-point sum {sum} must be even"
            );
            Some((sum / 2) as u8)
        }
        (Some(c), None) => Some(pg_tier_h(c, method)),
        (None, Some(r)) => Some(pg_tier_h(r, method)),
        (None, None) => None,
    }
}

/// Step 5 predicate: the two PG mode scores differ by at least the
/// method's mode-split threshold (2.0 points in v1.0).
pub fn is_mode_split(conquest_h: u8, rift_h: u8, method: &MethodConfig) -> bool {
    let diff_points_milli = (conquest_h as i64 - rift_h as i64).abs() * 500;
    diff_points_milli >= milli(method.mode_split_min)
}

/// Step 3: the band for a vote sum, decided in integers.
/// `score = sum_h / (2n)`; the comparison `score >= min` is evaluated as
/// `sum_h * 500 >= n * milli(min)`.
pub fn band_for(sum_h: u32, n: usize, method: &MethodConfig) -> TierBand {
    debug_assert!(n > 0);
    let lhs = sum_h as i64 * 500;
    let n = n as i64;
    if lhs >= n * milli(method.s_min) {
        TierBand::S
    } else if lhs >= n * milli(method.a_min) {
        TierBand::A
    } else if lhs >= n * milli(method.b_min) {
        TierBand::B
    } else if lhs >= n * milli(method.c_min) {
        TierBand::C
    } else {
        TierBand::D
    }
}

/// Step 4: the confidence label from coverage and spread.
pub fn label_for(coverage: usize, spread_h: u8, method: &MethodConfig) -> Label {
    match coverage {
        0 => Label::Unranked,
        1 => Label::Speculation,
        _ => {
            if spread_h as i64 * 500 <= milli(method.solid_max_spread) {
                Label::Solid
            } else {
                Label::Contested
            }
        }
    }
}

/// Step 7: display rounding — half away from zero, 1 decimal.
pub fn score_display(score: f64) -> String {
    format!("{:.1}", (score * 10.0).round() / 10.0)
}

fn pg_raw(conquest: Option<PgTier>, rift: Option<PgTier>) -> String {
    match (conquest, rift) {
        (Some(c), Some(r)) => format!("Conquest {} · Rift {}", c.as_str(), r.as_str()),
        (Some(c), None) => format!("Conquest {}", c.as_str()),
        (None, Some(r)) => format!("Rift {}", r.as_str()),
        (None, None) => String::new(),
    }
}

/// Assemble one character's result from its counted votes (steps 2–5).
fn assemble(
    id: &str,
    name: &str,
    votes: Vec<VoteOut>,
    mode_split: Option<(u8, u8)>,
    method: &MethodConfig,
) -> CharResult {
    let coverage = votes.len();
    if coverage == 0 {
        return CharResult {
            id: id.to_string(),
            name: name.to_string(),
            tier: TierBand::Unranked,
            label: Label::Unranked,
            score: None,
            score_display: None,
            coverage: 0,
            spread: None,
            votes,
            mode_split: None,
        };
    }
    let sum_h: u32 = votes.iter().map(|v| v.value_h as u32).sum();
    let max_h = votes.iter().map(|v| v.value_h).max().unwrap();
    let min_h = votes.iter().map(|v| v.value_h).min().unwrap();
    let spread_h = max_h - min_h;
    let score = sum_h as f64 / (2.0 * coverage as f64);
    let (tier, label) = if mode_split.is_some() {
        (TierBand::ModeSplit, Label::ModeSplit)
    } else {
        (
            band_for(sum_h, coverage, method),
            label_for(coverage, spread_h, method),
        )
    };
    CharResult {
        id: id.to_string(),
        name: name.to_string(),
        tier,
        label,
        score: Some(score),
        score_display: Some(score_display(score)),
        coverage,
        spread: Some(spread_h as f64 / 2.0),
        votes,
        mode_split,
    }
}

/// Step 6: boards. Inside S–D: score desc, then coverage desc, then
/// display name asc (Unicode code-point order). The Mode-split and
/// Unranked rows are name asc.
fn build_boards(characters: &[CharResult]) -> BTreeMap<TierBand, Vec<String>> {
    let mut boards: BTreeMap<TierBand, Vec<String>> = BTreeMap::new();
    for band in [
        TierBand::S,
        TierBand::A,
        TierBand::B,
        TierBand::C,
        TierBand::D,
        TierBand::ModeSplit,
        TierBand::Unranked,
    ] {
        let mut members: Vec<&CharResult> = characters.iter().filter(|c| c.tier == band).collect();
        match band {
            TierBand::ModeSplit | TierBand::Unranked => {
                members.sort_by(|a, b| a.name.cmp(&b.name));
            }
            _ => {
                members.sort_by(|a, b| {
                    b.score
                        .unwrap_or(0.0)
                        .partial_cmp(&a.score.unwrap_or(0.0))
                        .unwrap_or(std::cmp::Ordering::Equal)
                        .then_with(|| b.coverage.cmp(&a.coverage))
                        .then_with(|| a.name.cmp(&b.name))
                });
            }
        }
        boards.insert(band, members.iter().map(|c| c.id.clone()).collect());
    }
    boards
}

/// Compute the Overall consensus for one investment level (steps 1–6).
/// Only the three voting sources ever contribute a vote (step 2); a
/// source whose registry status is `stale_check_only` or `unavailable`
/// has dropped out under the staleness rules (Method §5) and casts none.
pub fn compute(ds: &Dataset, method: &MethodConfig, level: Level) -> LevelResult {
    let mut characters: Vec<CharResult> = Vec::with_capacity(ds.roster.characters.len());
    for character in &ds.roster.characters {
        let mut votes: Vec<VoteOut> = Vec::new();
        let mut mode_split: Option<(u8, u8)> = None;

        if source_counts(ds, "pg") {
            if let Some(p) = ds
                .pg
                .placements
                .iter()
                .find(|p| p.character == character.id)
            {
                if let (Some(c), Some(r)) = (p.conquest, p.rift) {
                    let c_h = pg_tier_h(c, method);
                    let r_h = pg_tier_h(r, method);
                    if is_mode_split(c_h, r_h, method) {
                        mode_split = Some((c_h, r_h));
                    }
                }
                if let Some(h) = pg_overall_h(p.conquest, p.rift, method) {
                    votes.push(VoteOut {
                        source: "pg".to_string(),
                        value_h: h,
                        raw: pg_raw(p.conquest, p.rift),
                    });
                }
            }
        }

        if source_counts(ds, "kaiden") {
            if let Some(k) = ds
                .kaiden
                .placements
                .iter()
                .find(|k| k.character == character.id)
            {
                if let Some(cell) = k.cell(level) {
                    votes.push(VoteOut {
                        source: "kaiden".to_string(),
                        value_h: kaiden_tier_h(cell.tier, method),
                        raw: cell.raw.clone(),
                    });
                }
            }
        }

        if source_counts(ds, "gamewith-jp") {
            if let Some(j) = ds
                .jp
                .placements
                .iter()
                .find(|j| j.character == character.id)
            {
                if let Some(tier) = j.tier {
                    votes.push(VoteOut {
                        source: "gamewith-jp".to_string(),
                        value_h: jp_tier_h(tier, method),
                        raw: j.raw.clone().unwrap_or_else(|| tier.as_str().to_string()),
                    });
                }
            }
        }

        characters.push(assemble(
            &character.id,
            &character.name,
            votes,
            mode_split,
            method,
        ));
    }
    let boards = build_boards(&characters);
    LevelResult {
        level,
        characters,
        boards,
    }
}

/// Whether a voting source currently casts votes: its registry role must
/// be `voting`, and its status must not be `stale_check_only` or
/// `unavailable` (no recorded status counts as current). A source with
/// no registry entry never votes; `load_dataset` refuses that case.
fn source_counts(ds: &Dataset, id: &str) -> bool {
    match ds.sources.sources.iter().find(|s| s.id == id) {
        Some(source) => {
            source.role == SourceRole::Voting
                && !matches!(
                    source.status,
                    Some(SourceStatus::StaleCheckOnly) | Some(SourceStatus::Unavailable)
                )
        }
        None => false,
    }
}

fn mode_tier(tier: Option<PgTier>) -> ModeTier {
    match tier {
        Some(PgTier::S) => ModeTier::S,
        Some(PgTier::A) => ModeTier::A,
        Some(PgTier::B) => ModeTier::B,
        Some(PgTier::C) => ModeTier::C,
        None => ModeTier::Unranked,
    }
}

/// Step 8: the per-mode boards — PG's tiers as published, name asc inside
/// each tier; characters PG has not placed are Unranked in that mode.
pub fn compute_modes(ds: &Dataset) -> ModeResults {
    let mut conquest: BTreeMap<ModeTier, Vec<String>> = BTreeMap::new();
    let mut rift: BTreeMap<ModeTier, Vec<String>> = BTreeMap::new();
    let mut per_character: Vec<(String, ModeTier, ModeTier)> = Vec::new();
    for band in [
        ModeTier::S,
        ModeTier::A,
        ModeTier::B,
        ModeTier::C,
        ModeTier::Unranked,
    ] {
        conquest.insert(band, Vec::new());
        rift.insert(band, Vec::new());
    }
    // Name asc inside a tier: walk the roster ordered by display name.
    let mut by_name: Vec<&Character> = ds.roster.characters.iter().collect();
    by_name.sort_by(|a, b| a.name.cmp(&b.name));
    for character in by_name {
        let placement = ds
            .pg
            .placements
            .iter()
            .find(|p| p.character == character.id);
        let (c, r) = match placement {
            Some(p) => (mode_tier(p.conquest), mode_tier(p.rift)),
            None => (ModeTier::Unranked, ModeTier::Unranked),
        };
        conquest.get_mut(&c).unwrap().push(character.id.clone());
        rift.get_mut(&r).unwrap().push(character.id.clone());
    }
    for character in &ds.roster.characters {
        let placement = ds
            .pg
            .placements
            .iter()
            .find(|p| p.character == character.id);
        let (c, r) = match placement {
            Some(p) => (mode_tier(p.conquest), mode_tier(p.rift)),
            None => (ModeTier::Unranked, ModeTier::Unranked),
        };
        per_character.push((character.id.clone(), c, r));
    }
    ModeResults {
        conquest,
        rift,
        per_character,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_method() -> MethodConfig {
        toml::from_str(
            r#"
version = "1.0"
weights = "equal"
s_min = 3.5
a_min = 3.0
b_min = 2.0
c_min = 1.0
solid_max_spread = 1.0
mode_split_min = 2.0
kaiden_conditional_rule = "first_listed"
[conversion.pg]
S = 4.0
A = 3.0
B = 2.0
C = 1.0
[conversion.gamewith-jp]
SS = 4.0
S = 3.0
A = 2.0
[conversion.kaiden]
T0 = 4.0
"T0.5" = 3.5
T1 = 3.0
T2 = 2.0
T3 = 1.0
T4 = 0.0
"#,
        )
        .expect("test method parses")
    }

    fn vote(source: &str, value_h: u8) -> VoteOut {
        VoteOut {
            source: source.to_string(),
            value_h,
            raw: String::new(),
        }
    }

    // Step 1
    #[test]
    fn pg_overall_averages_both_modes() {
        let m = test_method();
        assert_eq!(
            pg_overall_h(Some(PgTier::S), Some(PgTier::A), &m),
            Some(7) // (4.0 + 3.0) / 2 = 3.5
        );
        assert_eq!(
            pg_overall_h(Some(PgTier::C), Some(PgTier::S), &m),
            Some(5) // (1.0 + 4.0) / 2 = 2.5
        );
    }

    #[test]
    fn pg_overall_single_mode_is_used() {
        let m = test_method();
        assert_eq!(pg_overall_h(Some(PgTier::B), None, &m), Some(4));
        assert_eq!(pg_overall_h(None, Some(PgTier::A), &m), Some(6));
        assert_eq!(pg_overall_h(None, None, &m), None);
    }

    // Step 3
    #[test]
    fn band_thresholds_are_exact() {
        let m = test_method();
        // n = 2, so S needs sum_h >= 14, A >= 12, B >= 8, C >= 4.
        assert_eq!(band_for(14, 2, &m), TierBand::S);
        assert_eq!(band_for(13, 2, &m), TierBand::A);
        assert_eq!(band_for(12, 2, &m), TierBand::A);
        assert_eq!(band_for(11, 2, &m), TierBand::B);
        assert_eq!(band_for(8, 2, &m), TierBand::B);
        assert_eq!(band_for(7, 2, &m), TierBand::C);
        assert_eq!(band_for(4, 2, &m), TierBand::C);
        assert_eq!(band_for(3, 2, &m), TierBand::D);
        assert_eq!(band_for(0, 2, &m), TierBand::D);
    }

    #[test]
    fn band_thresholds_come_from_config() {
        // The T1.5 "tests bite" property at unit level: with s_min = 3.3 a
        // 10/3 score (Mina at Low) reaches S; with 3.5 it stays A.
        let mut m = test_method();
        assert_eq!(band_for(20, 3, &m), TierBand::A);
        m.s_min = 3.3;
        assert_eq!(band_for(20, 3, &m), TierBand::S);
    }

    // Step 4
    #[test]
    fn label_rules() {
        let m = test_method();
        assert_eq!(label_for(0, 0, &m), Label::Unranked);
        assert_eq!(label_for(1, 0, &m), Label::Speculation);
        assert_eq!(label_for(2, 2, &m), Label::Solid); // spread exactly 1.0
        assert_eq!(label_for(2, 3, &m), Label::Contested); // spread 1.5
        assert_eq!(label_for(3, 0, &m), Label::Solid);
        assert_eq!(label_for(3, 7, &m), Label::Contested); // spread 3.5
    }

    // Step 5
    #[test]
    fn mode_split_threshold() {
        let m = test_method();
        assert!(is_mode_split(2, 8, &m)); // C vs S: 3.0 apart
        assert!(is_mode_split(4, 8, &m)); // exactly 2.0 apart
        assert!(!is_mode_split(6, 8, &m)); // 1.0 apart
        assert!(!is_mode_split(8, 8, &m));
    }

    #[test]
    fn mode_split_overrides_tier_and_label() {
        let m = test_method();
        let result = assemble(
            "angel",
            "Angel",
            vec![vote("pg", 5), vote("kaiden", 4), vote("gamewith-jp", 4)],
            Some((2, 8)),
            &m,
        );
        assert_eq!(result.tier, TierBand::ModeSplit);
        assert_eq!(result.label, Label::ModeSplit);
        // Score, spread and coverage are still output for evidence.
        assert_eq!(result.coverage, 3);
        assert_eq!(result.spread, Some(0.5));
        assert_eq!(result.score_display.as_deref(), Some("2.2"));
    }

    #[test]
    fn unranked_when_no_votes() {
        let m = test_method();
        let result = assemble("ghost", "Ghost", vec![], None, &m);
        assert_eq!(result.tier, TierBand::Unranked);
        assert_eq!(result.label, Label::Unranked);
        assert_eq!(result.score, None);
        assert_eq!(result.spread, None);
    }

    // Step 7
    #[test]
    fn score_display_rounds_half_away_from_zero() {
        assert_eq!(score_display(3.1666666666666665), "3.2");
        assert_eq!(score_display(1.75), "1.8");
        assert_eq!(score_display(2.25), "2.3");
        assert_eq!(score_display(3.75), "3.8");
        assert_eq!(score_display(2.1666666666666665), "2.2");
        assert_eq!(score_display(4.0), "4.0");
        assert_eq!(score_display(0.5), "0.5");
    }

    // Step 6
    #[test]
    fn board_order_score_then_coverage_then_name() {
        let m = test_method();
        let mk = |id: &str, name: &str, votes: Vec<VoteOut>| assemble(id, name, votes, None, &m);
        let chars = vec![
            mk("b", "Beta", vec![vote("pg", 8), vote("kaiden", 6)]), // 3.5, cov 2
            mk(
                "a",
                "Alpha",
                vec![vote("pg", 8), vote("kaiden", 6), vote("gamewith-jp", 7)],
            ), // 3.5, cov 3
            mk("c", "Cara", vec![vote("pg", 8), vote("kaiden", 8)]), // 4.0, cov 2
        ];
        let boards = build_boards(&chars);
        // Cara (4.0) first; Alpha beats Beta on coverage at equal score.
        assert_eq!(boards[&TierBand::S], vec!["c", "a", "b"]);
    }
}
