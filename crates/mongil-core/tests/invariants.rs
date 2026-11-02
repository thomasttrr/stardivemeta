//! The 8 method invariants (plan §B.6), each as a proptest with 2,000
//! cases. Datasets are synthetic: random placements for 1–5 characters
//! across the three voting sources.

use mongil_core::load::Dataset;
use mongil_core::method::{
    band_for, compute, pg_tier_h, score_display, Label, LevelResult, TierBand,
};
use mongil_core::model::*;
use proptest::prelude::*;
use std::collections::BTreeMap;

/// The three voting sources as live data registers them (voting,
/// current): `load_dataset` requires the entries, and the engine reads
/// their role and status.
fn voting_registry() -> SourceRegistry {
    let source = |id: &str, kind: SourceKind| Source {
        id: id.to_string(),
        name: id.to_string(),
        role: SourceRole::Voting,
        kind,
        url: format!("https://example.com/{id}"),
        list_date: None,
        game_version: None,
        status: Some(SourceStatus::Current),
        criteria_caveat: None,
        level_definitions: None,
    };
    SourceRegistry {
        sources: vec![
            source("pg", SourceKind::PerMode),
            source("kaiden", SourceKind::OverallByInvestment),
            source("gamewith-jp", SourceKind::Overall),
        ],
    }
}

const METHOD_TOML: &str = r#"
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
"#;

fn method() -> MethodConfig {
    toml::from_str(METHOD_TOML).expect("method parses")
}

#[derive(Clone, Debug)]
struct Spec {
    conquest: Option<PgTier>,
    rift: Option<PgTier>,
    kaiden: [Option<KaidenTier>; 3],
    jp: Option<JpTier>,
}

fn pg_tier() -> impl Strategy<Value = PgTier> {
    prop_oneof![
        Just(PgTier::S),
        Just(PgTier::A),
        Just(PgTier::B),
        Just(PgTier::C)
    ]
}

fn jp_tier() -> impl Strategy<Value = JpTier> {
    prop_oneof![Just(JpTier::SS), Just(JpTier::S), Just(JpTier::A)]
}

fn kaiden_tier() -> impl Strategy<Value = KaidenTier> {
    prop_oneof![
        Just(KaidenTier::T0),
        Just(KaidenTier::T0_5),
        Just(KaidenTier::T1),
        Just(KaidenTier::T2),
        Just(KaidenTier::T3),
        Just(KaidenTier::T4)
    ]
}

fn spec() -> impl Strategy<Value = Spec> {
    (
        prop::option::of(pg_tier()),
        prop::option::of(pg_tier()),
        prop::option::of(kaiden_tier()),
        prop::option::of(kaiden_tier()),
        prop::option::of(kaiden_tier()),
        prop::option::of(jp_tier()),
    )
        .prop_map(|(c, r, kl, kh, km, j)| Spec {
            conquest: c,
            rift: r,
            kaiden: [kl, kh, km],
            jp: j,
        })
}

fn specs() -> impl Strategy<Value = Vec<Spec>> {
    prop::collection::vec(spec(), 1..=5)
}

fn mk_character(id: &str) -> Character {
    Character {
        id: id.to_string(),
        name: format!("Name {id}"),
        base_of: None,
        released_on: None,
        released_on_sources: vec![],
        rarity: None,
        rarity_sources: vec![],
        pool: None,
        pool_sources: vec![],
    }
}

fn build_dataset(specs: &[Spec], extra_character: bool, with_check_only: bool) -> Dataset {
    let mut chars = Vec::new();
    let mut pg = Vec::new();
    let mut kaiden = Vec::new();
    let mut jp = Vec::new();
    let cell = |t: Option<KaidenTier>| {
        t.map(|tier| KaidenLevelCell {
            tier,
            raw: tier.as_str().to_string(),
            awakening: None,
            flags: vec![],
        })
    };
    for (i, s) in specs.iter().enumerate() {
        let id = format!("char{i}");
        chars.push(mk_character(&id));
        pg.push(PgPlacement {
            character: id.clone(),
            conquest: s.conquest,
            rift: s.rift,
            status: None,
            note: None,
        });
        kaiden.push(KaidenPlacement {
            character: id.clone(),
            page_url: None,
            page_updated: None,
            low: cell(s.kaiden[0]),
            high: cell(s.kaiden[1]),
            max: cell(s.kaiden[2]),
            status: None,
            note: None,
        });
        jp.push(JpPlacement {
            character: id.clone(),
            tier: s.jp,
            raw: s.jp.map(|t| t.as_str().to_string()),
            status: None,
            note: None,
        });
    }
    if extra_character {
        let id = "extra".to_string();
        chars.push(mk_character(&id));
        pg.push(PgPlacement {
            character: id.clone(),
            conquest: Some(PgTier::S),
            rift: Some(PgTier::S),
            status: None,
            note: None,
        });
        kaiden.push(KaidenPlacement {
            character: id.clone(),
            page_url: None,
            page_updated: None,
            low: cell(Some(KaidenTier::T0)),
            high: cell(Some(KaidenTier::T0)),
            max: cell(Some(KaidenTier::T0)),
            status: None,
            note: None,
        });
        jp.push(JpPlacement {
            character: id,
            tier: Some(JpTier::SS),
            raw: Some("SS".to_string()),
            status: None,
            note: None,
        });
    }
    let mut check_only = BTreeMap::new();
    if with_check_only {
        check_only.insert(
            "urgametips".to_string(),
            CheckOnlyPlacements {
                meta: PlacementMeta {
                    source: "urgametips".to_string(),
                    harvest: "test".to_string(),
                    worker: None,
                    read_mode: None,
                    read_at: None,
                },
                kind: CheckOnlyKind::ModesConquestOnly,
                placements: specs
                    .iter()
                    .enumerate()
                    .map(|(i, _)| CheckOnlyPlacement {
                        character: format!("char{i}"),
                        tier: None,
                        world: None,
                        conquest: Some(PgTier::C),
                        rift: None,
                        note: None,
                    })
                    .collect(),
            },
        );
    }
    Dataset {
        roster: Roster { characters: chars },
        sources: voting_registry(),
        method: method(),
        pg: PgPlacements {
            meta: None,
            placements: pg,
        },
        kaiden: KaidenPlacements {
            meta: None,
            placements: kaiden,
        },
        jp: JpPlacements {
            meta: None,
            placements: jp,
        },
        check_only,
        game: None,
        codes: None,
        code_sources: None,
        banners: None,
        errata: None,
        checks: None,
    }
}

fn all_levels(ds: &Dataset) -> Vec<LevelResult> {
    Level::ALL
        .iter()
        .map(|&level| compute(ds, &ds.method, level))
        .collect()
}

fn expected_coverage(s: &Spec, level: Level) -> usize {
    let pg = (s.conquest.is_some() || s.rift.is_some()) as usize;
    let kaiden = s.kaiden[level as usize].is_some() as usize;
    let jp = s.jp.is_some() as usize;
    pg + kaiden + jp
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(2000))]

    /// Invariant 1: Score ∈ [0, 4].
    #[test]
    fn invariant_1_score_in_range(specs in specs()) {
        let ds = build_dataset(&specs, false, false);
        for result in all_levels(&ds) {
            for c in &result.characters {
                if let Some(score) = c.score {
                    prop_assert!((0.0..=4.0).contains(&score), "score {score} out of range");
                }
            }
        }
    }

    /// Invariant 2: the tier matches the band thresholds exactly, computed
    /// in integers from the emitted votes (v1.0 constants: S sum_h >= 7n,
    /// A >= 6n, B >= 4n, C >= 2n).
    #[test]
    fn invariant_2_tier_matches_integer_bands(specs in specs()) {
        let ds = build_dataset(&specs, false, false);
        for result in all_levels(&ds) {
            for c in &result.characters {
                if c.tier == TierBand::ModeSplit {
                    continue;
                }
                let n = c.votes.len();
                if n == 0 {
                    prop_assert_eq!(c.tier, TierBand::Unranked);
                    continue;
                }
                let sum_h: u32 = c.votes.iter().map(|v| v.value_h as u32).sum();
                let expected = if sum_h >= 7 * n as u32 {
                    TierBand::S
                } else if sum_h >= 6 * n as u32 {
                    TierBand::A
                } else if sum_h >= 4 * n as u32 {
                    TierBand::B
                } else if sum_h >= 2 * n as u32 {
                    TierBand::C
                } else {
                    TierBand::D
                };
                prop_assert_eq!(c.tier, expected, "sum_h={} n={}", sum_h, n);
            }
        }
    }

    /// Invariant 3: Coverage = number of non-missing votes.
    #[test]
    fn invariant_3_coverage_counts_votes(specs in specs()) {
        let ds = build_dataset(&specs, false, false);
        for result in all_levels(&ds) {
            for (i, c) in result.characters.iter().enumerate() {
                prop_assert_eq!(c.coverage, c.votes.len());
                prop_assert_eq!(c.coverage, expected_coverage(&specs[i], result.level));
            }
        }
    }

    /// Invariant 4: Label follows the rules from coverage and spread.
    #[test]
    fn invariant_4_label_follows_rules(specs in specs()) {
        let ds = build_dataset(&specs, false, false);
        for result in all_levels(&ds) {
            for c in &result.characters {
                if c.tier == TierBand::ModeSplit {
                    prop_assert_eq!(c.label, Label::ModeSplit);
                    continue;
                }
                let expected = match c.votes.len() {
                    0 => Label::Unranked,
                    1 => Label::Speculation,
                    _ => {
                        let max_h = c.votes.iter().map(|v| v.value_h).max().unwrap();
                        let min_h = c.votes.iter().map(|v| v.value_h).min().unwrap();
                        if max_h - min_h <= 2 {
                            Label::Solid
                        } else {
                            Label::Contested
                        }
                    }
                };
                prop_assert_eq!(c.label, expected);
            }
        }
    }

    /// Invariant 5: `score_display`, read back as a number, never lands in
    /// a different band than the score. Exercised at the function level
    /// for 1–6 voting sources (the plan notes it holds up to 9 sources),
    /// and on real compute() output.
    #[test]
    fn invariant_5_display_never_changes_band(
        votes in prop::collection::vec(0u8..=8, 1..=6),
        specs in specs(),
    ) {
        let m = method();
        let n = votes.len();
        let sum_h: u32 = votes.iter().map(|v| *v as u32).sum();
        let band = band_for(sum_h, n, &m);
        let score = sum_h as f64 / (2.0 * n as f64);
        let displayed: f64 = score_display(score).parse().expect("display parses");
        let display_band = if displayed >= 3.5 {
            TierBand::S
        } else if displayed >= 3.0 {
            TierBand::A
        } else if displayed >= 2.0 {
            TierBand::B
        } else if displayed >= 1.0 {
            TierBand::C
        } else {
            TierBand::D
        };
        prop_assert_eq!(display_band, band, "sum_h={} n={}", sum_h, n);

        let ds = build_dataset(&specs, false, false);
        for result in all_levels(&ds) {
            for c in &result.characters {
                if c.tier == TierBand::ModeSplit || c.tier == TierBand::Unranked {
                    continue;
                }
                let displayed: f64 = c.score_display.as_ref().unwrap().parse().unwrap();
                let db = if displayed >= 3.5 {
                    TierBand::S
                } else if displayed >= 3.0 {
                    TierBand::A
                } else if displayed >= 2.0 {
                    TierBand::B
                } else if displayed >= 1.0 {
                    TierBand::C
                } else {
                    TierBand::D
                };
                prop_assert_eq!(db, c.tier);
            }
        }
    }

    /// Invariant 6: Mode-split occurs only if PG has both modes and
    /// |c − r| ≥ 2.0.
    #[test]
    fn invariant_6_mode_split_only_when_pg_gap(specs in specs()) {
        let ds = build_dataset(&specs, false, false);
        let m = method();
        for result in all_levels(&ds) {
            for (i, c) in result.characters.iter().enumerate() {
                if c.tier == TierBand::ModeSplit {
                    let s = &specs[i];
                    let (con, rift) = (s.conquest, s.rift);
                    prop_assert!(con.is_some() && rift.is_some(), "split without both modes");
                    let diff = (pg_tier_h(con.unwrap(), &m) as i16
                        - pg_tier_h(rift.unwrap(), &m) as i16)
                        .abs();
                    prop_assert!(diff >= 4, "split with gap {diff} half-points");
                    prop_assert_eq!(c.mode_split, Some((pg_tier_h(con.unwrap(), &m), pg_tier_h(rift.unwrap(), &m))));
                }
            }
        }
    }

    /// Invariant 7: Removing a check-only source never changes any result.
    #[test]
    fn invariant_7_check_only_sources_never_count(specs in specs()) {
        let with = build_dataset(&specs, false, true);
        let without = build_dataset(&specs, false, false);
        for level in Level::ALL {
            let a = compute(&with, &with.method, level);
            let b = compute(&without, &without.method, level);
            prop_assert_eq!(a.characters, b.characters);
            prop_assert_eq!(a.boards, b.boards);
        }
    }

    /// Invariant 8: Adding a new character never changes another
    /// character's result.
    #[test]
    fn invariant_8_adding_a_character_changes_nothing(specs in specs()) {
        let base = build_dataset(&specs, false, false);
        let extended = build_dataset(&specs, true, false);
        for level in Level::ALL {
            let a = compute(&base, &base.method, level);
            let b = compute(&extended, &extended.method, level);
            for c in &a.characters {
                let other = b
                    .characters
                    .iter()
                    .find(|x| x.id == c.id)
                    .expect("character still present");
                prop_assert_eq!(other, c);
            }
        }
    }
}
