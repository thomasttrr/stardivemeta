//! `mongil-build emit` (plan T1.6): assemble the §B.2 contract JSON from
//! a loaded dataset and write `tierlist.json`, `codes.json`,
//! `history.json`, `meta.json`.
//!
//! Determinism (plan T1.6 step): struct field order is fixed, all maps
//! are `BTreeMap`s, and no wall-clock value enters any file except
//! `meta.json`'s `generated_at`. Running emit twice on unchanged data
//! gives a byte-identical `tierlist.json` (acceptance a).

use mongil_core::contract::{
    Beginner, Boards, Character, CodeEntry, CodeObservation as ContractObservation, CodeReward,
    CodeStatus, CodesFile, Derived, EvidencePlacement, EvidenceRow, History, KaidenFlag, Label,
    LevelCell, LevelDefinitions, Meta, MethodInfo, ModeBoard, ModeBoards, ModePlacement, ModeSplit,
    ModeTier, Modes, NoneReason, RerollEntry, Result as ContractResult, Results, Rise,
    SnapshotEntry, Source as ContractSource, SourceKind, SourceRole, SourceStatus, Tier, TierBoard,
    TierList, Vote, Why,
};
use mongil_core::derive;
use mongil_core::load::Dataset;
use mongil_core::method::{
    compute, compute_modes, pg_overall_h, CharResult, LevelResult, ModeTier as EngineModeTier,
    TierBand,
};
use mongil_core::model::{
    CheckOnlyKind, Code, CodeObservationStatus, KaidenFlag as ModelFlag, Level, PlacementStatus,
    Source as ModelSource, SourceKind as ModelKind, SourceRole as ModelRole,
    SourceStatus as ModelStatus,
};
use std::fs;
use std::path::Path;

// ---------------------------------------------------------------------------
// Enum mappers (engine/model string forms are the contract strings)
// ---------------------------------------------------------------------------

fn tier_of(band: TierBand) -> Tier {
    match band {
        TierBand::S => Tier::S,
        TierBand::A => Tier::A,
        TierBand::B => Tier::B,
        TierBand::C => Tier::C,
        TierBand::D => Tier::D,
        TierBand::ModeSplit => Tier::ModeSplit,
        TierBand::Unranked => Tier::Unranked,
    }
}

fn label_of(label: mongil_core::method::Label) -> Label {
    match label {
        mongil_core::method::Label::Solid => Label::Solid,
        mongil_core::method::Label::Contested => Label::Contested,
        mongil_core::method::Label::Speculation => Label::Speculation,
        mongil_core::method::Label::ModeSplit => Label::ModeSplit,
        mongil_core::method::Label::Unranked => Label::Unranked,
    }
}

fn mode_tier_of(tier: EngineModeTier) -> ModeTier {
    match tier {
        EngineModeTier::S => ModeTier::S,
        EngineModeTier::A => ModeTier::A,
        EngineModeTier::B => ModeTier::B,
        EngineModeTier::C => ModeTier::C,
        EngineModeTier::Unranked => ModeTier::Unranked,
    }
}

fn role_of(role: ModelRole) -> SourceRole {
    match role {
        ModelRole::Voting => SourceRole::Voting,
        // Excluded sources are filtered out before this is called.
        ModelRole::CheckOnly | ModelRole::Excluded => SourceRole::CheckOnly,
    }
}

fn kind_of(kind: ModelKind) -> SourceKind {
    match kind {
        ModelKind::PerMode => SourceKind::PerMode,
        ModelKind::Overall => SourceKind::Overall,
        ModelKind::OverallByInvestment => SourceKind::OverallByInvestment,
    }
}

fn status_of(status: ModelStatus) -> SourceStatus {
    match status {
        ModelStatus::Current => SourceStatus::Current,
        ModelStatus::PrePatchFlagged => SourceStatus::PrePatchFlagged,
        ModelStatus::StaleCheckOnly => SourceStatus::StaleCheckOnly,
        ModelStatus::Unavailable => SourceStatus::Unavailable,
    }
}

fn reason_of(status: PlacementStatus) -> NoneReason {
    match status {
        PlacementStatus::NotListed => NoneReason::NotListed,
        PlacementStatus::NoPlacement => NoneReason::NoPlacement,
        PlacementStatus::NotInTextTiers => NoneReason::NotInTextTiers,
    }
}

/// Mirror of the method engine's casting rule (Method §5): a source with
/// no recorded status counts; `stale_check_only` / `unavailable` do not.
fn source_casts(source: &ModelSource) -> bool {
    !matches!(
        source.status,
        Some(ModelStatus::StaleCheckOnly) | Some(ModelStatus::Unavailable)
    )
}

// ---------------------------------------------------------------------------
// tierlist.json assembly
// ---------------------------------------------------------------------------

fn result_of(result: &CharResult) -> ContractResult {
    let votes: Vec<Vote> = result
        .votes
        .iter()
        .map(|v| Vote {
            source: v.source.clone(),
            value: v.value(),
            raw: v.raw.clone(),
        })
        .collect();
    let why = match result.mode_split {
        Some((conquest_h, rift_h)) => Why {
            high: None,
            low: None,
            mode_split: Some(ModeSplit {
                conquest: conquest_h as f64 / 2.0,
                rift: rift_h as f64 / 2.0,
            }),
        },
        None => {
            let high = result.votes.iter().max_by_key(|v| v.value_h).map(|v| Vote {
                source: v.source.clone(),
                value: v.value(),
                raw: v.raw.clone(),
            });
            let low = result.votes.iter().min_by_key(|v| v.value_h).map(|v| Vote {
                source: v.source.clone(),
                value: v.value(),
                raw: v.raw.clone(),
            });
            Why {
                high,
                low,
                mode_split: None,
            }
        }
    };
    ContractResult {
        tier: tier_of(result.tier),
        label: label_of(result.label),
        score: result.score,
        score_display: result.score_display.clone(),
        coverage: result.coverage as u32,
        spread: result.spread,
        votes,
        why,
    }
}

fn board_of(result: &LevelResult) -> TierBoard {
    let get = |band: TierBand| result.boards.get(&band).cloned().unwrap_or_default();
    TierBoard {
        s: get(TierBand::S),
        a: get(TierBand::A),
        b: get(TierBand::B),
        c: get(TierBand::C),
        d: get(TierBand::D),
        mode_split: get(TierBand::ModeSplit),
        unranked: get(TierBand::Unranked),
    }
}

fn flag_of(flag: ModelFlag) -> KaidenFlag {
    match flag {
        ModelFlag::P => KaidenFlag::P,
        ModelFlag::E => KaidenFlag::E,
    }
}

fn evidence_for(ds: &Dataset, character_id: &str) -> Vec<EvidenceRow> {
    let method = &ds.method;
    let mut rows = Vec::new();
    for source in &ds.sources.sources {
        if source.role == ModelRole::Excluded {
            continue; // kept for the record, never shown (plan §B.3)
        }
        let role = role_of(source.role);
        let casts = source_casts(source);
        let row = match source.id.as_str() {
            "pg" => {
                let p = ds
                    .pg
                    .placements
                    .iter()
                    .find(|p| p.character == character_id);
                let (placement, counted) = match p {
                    Some(p) => {
                        if let Some(status) = p.status {
                            (
                                EvidencePlacement::None {
                                    reason: reason_of(status),
                                },
                                false,
                            )
                        } else if p.conquest.is_some() || p.rift.is_some() {
                            let overall = pg_overall_h(p.conquest, p.rift, method);
                            (
                                EvidencePlacement::Modes {
                                    conquest: p.conquest.map(|t| t.as_str().to_string()),
                                    rift: p.rift.map(|t| t.as_str().to_string()),
                                    converted_overall: overall.map(|h| h as f64 / 2.0),
                                },
                                casts && overall.is_some(),
                            )
                        } else {
                            (
                                EvidencePlacement::None {
                                    reason: NoneReason::NotListed,
                                },
                                false,
                            )
                        }
                    }
                    None => (
                        EvidencePlacement::None {
                            reason: NoneReason::NotListed,
                        },
                        false,
                    ),
                };
                EvidenceRow {
                    source: source.id.clone(),
                    role,
                    counted,
                    placement,
                    page_url: source.url.clone(),
                    page_date: source.list_date.clone(),
                    note: p.and_then(|p| p.note.clone()),
                }
            }
            "kaiden" => {
                let k = ds
                    .kaiden
                    .placements
                    .iter()
                    .find(|k| k.character == character_id);
                let cell = |c: &mongil_core::model::KaidenLevelCell| LevelCell {
                    tier: c.tier.as_str().to_string(),
                    raw: c.raw.clone(),
                    awakening: c.awakening.clone(),
                    flags: c.flags.iter().copied().map(flag_of).collect(),
                    converted: mongil_core::method::kaiden_tier_h(c.tier, method) as f64 / 2.0,
                };
                let (placement, counted) = match k {
                    Some(k) => {
                        if let Some(status) = k.status {
                            (
                                EvidencePlacement::None {
                                    reason: reason_of(status),
                                },
                                false,
                            )
                        } else if let (Some(low), Some(high), Some(max)) =
                            (k.cell(Level::Low), k.cell(Level::High), k.cell(Level::Max))
                        {
                            (
                                EvidencePlacement::Levels {
                                    low: cell(low),
                                    high: cell(high),
                                    max: cell(max),
                                },
                                casts,
                            )
                        } else {
                            // A level with no cell at all. Not present in
                            // the harvested data shapes (a placement is
                            // all three levels or a status); if it ever
                            // appears, surface the levels that exist with
                            // an empty cell rather than dropping the row.
                            let empty = LevelCell {
                                tier: String::new(),
                                raw: String::new(),
                                awakening: None,
                                flags: Vec::new(),
                                converted: 0.0,
                            };
                            (
                                EvidencePlacement::Levels {
                                    low: k
                                        .cell(Level::Low)
                                        .map(cell)
                                        .unwrap_or_else(|| empty.clone()),
                                    high: k
                                        .cell(Level::High)
                                        .map(cell)
                                        .unwrap_or_else(|| empty.clone()),
                                    max: k.cell(Level::Max).map(cell).unwrap_or(empty),
                                },
                                casts && k.cell(Level::Low).is_some(),
                            )
                        }
                    }
                    None => (
                        EvidencePlacement::None {
                            reason: NoneReason::NotListed,
                        },
                        false,
                    ),
                };
                EvidenceRow {
                    source: source.id.clone(),
                    role,
                    counted,
                    placement,
                    page_url: k
                        .and_then(|k| k.page_url.clone())
                        .unwrap_or_else(|| source.url.clone()),
                    page_date: k.and_then(|k| k.page_updated.clone()),
                    note: k.and_then(|k| k.note.clone()),
                }
            }
            "gamewith-jp" => {
                let j = ds
                    .jp
                    .placements
                    .iter()
                    .find(|j| j.character == character_id);
                let (placement, counted) = match j {
                    Some(j) => {
                        if let Some(status) = j.status {
                            (
                                EvidencePlacement::None {
                                    reason: reason_of(status),
                                },
                                false,
                            )
                        } else if let Some(tier) = j.tier {
                            (
                                EvidencePlacement::Single {
                                    raw: j.raw.clone().unwrap_or_else(|| tier.as_str().to_string()),
                                    converted: Some(
                                        mongil_core::method::jp_tier_h(tier, method) as f64 / 2.0,
                                    ),
                                },
                                casts,
                            )
                        } else {
                            (
                                EvidencePlacement::None {
                                    reason: NoneReason::NotListed,
                                },
                                false,
                            )
                        }
                    }
                    None => (
                        EvidencePlacement::None {
                            reason: NoneReason::NotListed,
                        },
                        false,
                    ),
                };
                EvidenceRow {
                    source: source.id.clone(),
                    role,
                    counted,
                    placement,
                    page_url: source.url.clone(),
                    page_date: source.list_date.clone(),
                    note: j.and_then(|j| j.note.clone()),
                }
            }
            _ => {
                // Check-only sources: a row exists only once the source
                // has a placements file (mongil.gg has none until T2.6).
                let Some(file) = ds.check_only.get(&source.id) else {
                    continue;
                };
                let entry = file.placements.iter().find(|p| p.character == character_id);
                let placement = match entry {
                    Some(e) => match file.kind {
                        CheckOnlyKind::SingleTier => match &e.tier {
                            Some(tier) => EvidencePlacement::Single {
                                raw: tier.clone(),
                                converted: None,
                            },
                            None => EvidencePlacement::None {
                                reason: NoneReason::NotListed,
                            },
                        },
                        CheckOnlyKind::ModesConquestOnly => match e.conquest {
                            Some(tier) => EvidencePlacement::Modes {
                                conquest: Some(tier.as_str().to_string()),
                                rift: None,
                                converted_overall: None,
                            },
                            None => EvidencePlacement::None {
                                reason: NoneReason::NotListed,
                            },
                        },
                        // Per-mode reference rows (T2.6): the two modes
                        // the method knows are shown; `world` has no
                        // contract slot (the method ignores World lists
                        // too) and stays in the data file only.
                        CheckOnlyKind::PerMode => {
                            if e.conquest.is_none() && e.rift.is_none() {
                                EvidencePlacement::None {
                                    reason: NoneReason::NotListed,
                                }
                            } else {
                                EvidencePlacement::Modes {
                                    conquest: e.conquest.map(|t| t.as_str().to_string()),
                                    rift: e.rift.map(|t| t.as_str().to_string()),
                                    converted_overall: None,
                                }
                            }
                        }
                    },
                    None => EvidencePlacement::None {
                        reason: NoneReason::NotListed,
                    },
                };
                EvidenceRow {
                    source: source.id.clone(),
                    role,
                    counted: false,
                    placement,
                    page_url: source.url.clone(),
                    page_date: source.list_date.clone(),
                    note: entry.and_then(|e| e.note.clone()),
                }
            }
        };
        rows.push(row);
    }
    rows
}

pub fn build_tierlist(ds: &Dataset) -> TierList {
    let method = &ds.method;
    let levels: Vec<LevelResult> = Level::ALL
        .iter()
        .map(|level| compute(ds, method, *level))
        .collect();
    let modes = compute_modes(ds);
    let beginner = derive::beginner(ds, method);

    let sources: Vec<ContractSource> = ds
        .sources
        .sources
        .iter()
        .filter(|s| s.role != ModelRole::Excluded)
        .map(|s| ContractSource {
            id: s.id.clone(),
            name: s.name.clone(),
            role: role_of(s.role),
            kind: kind_of(s.kind),
            url: s.url.clone(),
            list_date: s.list_date.clone(),
            game_version: s.game_version.clone(),
            // Sources whose freshness T2.3 has not assigned yet emit as
            // current; the fresh harvest records real statuses.
            status: status_of(s.status.unwrap_or(ModelStatus::Current)),
            criteria_caveat: s.criteria_caveat.clone(),
            level_definitions: s.level_definitions.as_ref().map(|d| LevelDefinitions {
                low: d.low.clone(),
                high: d.high.clone(),
                max: d.max.clone(),
            }),
        })
        .collect();

    let result_at = |level_idx: usize, id: &str| {
        levels[level_idx]
            .characters
            .iter()
            .find(|c| c.id == id)
            .expect("result exists for every roster character")
    };

    let characters: Vec<Character> = ds
        .roster
        .characters
        .iter()
        .map(|character| {
            let mode_pair = modes
                .per_character
                .iter()
                .find(|(id, _, _)| id == &character.id)
                .expect("mode result exists for every roster character");
            let pg = ds
                .pg
                .placements
                .iter()
                .find(|p| p.character == character.id);
            let mode_placement = |which: EngineModeTier, raw: Option<String>| ModePlacement {
                tier: mode_tier_of(which),
                raw,
            };
            Character {
                id: character.id.clone(),
                name: character.name.clone(),
                released_on: character.released_on.clone(),
                // The 45-day "new" rule is T4.2; until release dates are
                // harvested (T2.8) nothing is marked new.
                is_new: false,
                results: Results {
                    low: result_of(result_at(0, &character.id)),
                    high: result_of(result_at(1, &character.id)),
                    max: result_of(result_at(2, &character.id)),
                },
                modes: Modes {
                    conquest: mode_placement(
                        mode_pair.1,
                        pg.and_then(|p| p.conquest).map(|t| t.as_str().to_string()),
                    ),
                    rift: mode_placement(
                        mode_pair.2,
                        pg.and_then(|p| p.rift).map(|t| t.as_str().to_string()),
                    ),
                },
                evidence: evidence_for(ds, &character.id),
                errata: ds
                    .errata
                    .as_ref()
                    .map(|e| {
                        e.errata
                            .iter()
                            .filter(|erratum| erratum.characters.iter().any(|c| c == &character.id))
                            .map(|erratum| erratum.id.clone())
                            .collect()
                    })
                    .unwrap_or_default(),
            }
        })
        .collect();

    let mode_board = |board: &std::collections::BTreeMap<EngineModeTier, Vec<String>>| {
        let get = |tier: EngineModeTier| board.get(&tier).cloned().unwrap_or_default();
        ModeBoard {
            s: get(EngineModeTier::S),
            a: get(EngineModeTier::A),
            b: get(EngineModeTier::B),
            c: get(EngineModeTier::C),
            unranked: get(EngineModeTier::Unranked),
        }
    };

    TierList {
        contract_version: 1,
        method: MethodInfo {
            version: method.version.clone(),
            s_min: method.s_min,
            a_min: method.a_min,
            b_min: method.b_min,
            c_min: method.c_min,
            solid_max_spread: method.solid_max_spread,
            mode_split_min: method.mode_split_min,
            conversion: conversion_tables(ds),
        },
        game_version: ds
            .game
            .as_ref()
            .map(|g| g.current_version.clone())
            .unwrap_or_default(),
        snapshot_id: snapshot_id(ds),
        sources,
        characters,
        boards: Boards {
            low: board_of(&levels[0]),
            high: board_of(&levels[1]),
            max: board_of(&levels[2]),
        },
        mode_boards: ModeBoards {
            conquest: mode_board(&modes.conquest),
            rift: mode_board(&modes.rift),
        },
        derived: Derived {
            beginner: Beginner {
                agreed_low: beginner.agreed_low.clone(),
                rise: beginner
                    .rise
                    .iter()
                    .map(|r| Rise {
                        id: r.id.clone(),
                        low: tier_of(r.low),
                        max: tier_of(r.max),
                        delta: r.delta,
                    })
                    .collect(),
                contested_low: beginner.contested_low.clone(),
                reroll: beginner.reroll.as_ref().map(|ids| {
                    ids.iter()
                        .map(|id| RerollEntry { id: id.clone() })
                        .collect()
                }),
            },
        },
    }
}

/// The method block carries every source's conversion table, keyed by
/// source id as in `data/method/v1.0.toml`.
fn conversion_tables(
    ds: &Dataset,
) -> std::collections::BTreeMap<String, std::collections::BTreeMap<String, f64>> {
    let c = &ds.method.conversion;
    let mut out = std::collections::BTreeMap::new();
    out.insert("pg".to_string(), c.pg.clone());
    out.insert("gamewith-jp".to_string(), c.gamewith_jp.clone());
    out.insert("kaiden".to_string(), c.kaiden.clone());
    out
}

fn snapshot_id(ds: &Dataset) -> String {
    ds.checks
        .as_ref()
        .map(|c| c.last_sweep.clone())
        .unwrap_or_else(|| "none".to_string())
}

// ---------------------------------------------------------------------------
// codes.json / history.json / meta.json
// ---------------------------------------------------------------------------

/// One independence group's current say on a code: the status of its
/// freshest observation (by `seen_at`), and when that was seen.
struct GroupSay {
    status: CodeObservationStatus,
    seen_epoch: i64,
}

/// Compute a code's status from its observations, per §B.3's rules
/// (the T2.7 codes engine). Observations are grouped by independence
/// group; each source's — and then each group's — freshest observation
/// is the one that speaks.
///
/// 1. `expired` if the stated `expires_at` is before build time, or at
///    least 2 distinct groups say `expired`.
/// 2. `active` if at least 2 distinct groups say `active`, each of
///    those sightings within 7 days of build time, and no group says
///    `expired`.
/// 3. otherwise `unconfirmed`.
fn compute_code_status(
    code: &Code,
    groups: &std::collections::BTreeMap<String, String>,
    now: i64,
) -> CodeStatus {
    // Freshest observation per group.
    let mut says: std::collections::BTreeMap<String, GroupSay> = std::collections::BTreeMap::new();
    for obs in &code.observations {
        let seen_epoch = crate::validate::parse_rfc3339(&obs.seen_at).unwrap_or(0);
        let group = groups
            .get(&obs.source)
            .cloned()
            .unwrap_or_else(|| obs.source.clone());
        match says.get(&group) {
            Some(existing) if existing.seen_epoch > seen_epoch => {}
            _ => {
                says.insert(
                    group,
                    GroupSay {
                        status: obs.status,
                        seen_epoch,
                    },
                );
            }
        }
    }

    if let Some(expires_at) = &code.expires_at {
        if let Some(expiry) = crate::validate::parse_rfc3339(expires_at) {
            if expiry < now {
                return CodeStatus::Expired;
            }
        }
    }
    let expired_groups = says
        .values()
        .filter(|s| s.status == CodeObservationStatus::Expired)
        .count();
    if expired_groups >= 2 {
        return CodeStatus::Expired;
    }
    let fresh_active_groups = says
        .values()
        .filter(|s| s.status == CodeObservationStatus::Active && s.seen_epoch >= now - 7 * 86_400)
        .count();
    if fresh_active_groups >= 2 && expired_groups == 0 {
        return CodeStatus::Active;
    }
    CodeStatus::Unconfirmed
}

fn code_status_str(status: CodeObservationStatus) -> &'static str {
    match status {
        CodeObservationStatus::Active => "active",
        CodeObservationStatus::Expired => "expired",
        CodeObservationStatus::NotListed => "not_listed",
    }
}

pub fn build_codes(ds: &Dataset) -> Result<CodesFile, String> {
    let now = crate::validate::now_epoch();
    let groups: std::collections::BTreeMap<String, String> = ds
        .code_sources
        .as_ref()
        .map(|registry| {
            registry
                .sources
                .iter()
                .map(|s| (s.id.clone(), s.group.clone()))
                .collect()
        })
        .unwrap_or_default();

    let mut entries: Vec<CodeEntry> = Vec::new();
    if let Some(codes) = &ds.codes {
        for code in &codes.codes {
            let group_of = |source: &str| {
                groups
                    .get(source)
                    .cloned()
                    .unwrap_or_else(|| source.to_string())
            };
            let first_seen = code
                .observations
                .iter()
                .min_by_key(|o| crate::validate::parse_rfc3339(&o.seen_at).unwrap_or(i64::MAX))
                .map(|o| o.seen_at.clone());
            entries.push(CodeEntry {
                code: code.code.clone(),
                status: compute_code_status(code, &groups, now),
                reward: CodeReward {
                    text: code.reward.clone(),
                    source: code.reward_source.clone(),
                },
                expires_at: code.expires_at.clone(),
                first_seen,
                observations: code
                    .observations
                    .iter()
                    .map(|o| ContractObservation {
                        source: o.source.clone(),
                        group: group_of(&o.source),
                        status: code_status_str(o.status).to_string(),
                        seen_at: o.seen_at.clone(),
                        url: o.url.clone(),
                    })
                    .collect(),
            });
        }
    }
    Ok(CodesFile {
        checked_at: ds.checks.as_ref().map(|c| c.codes_checked_at.clone()),
        codes: entries,
    })
}

pub fn build_history(data_dir: &Path) -> Result<History, String> {
    let dir = data_dir.join("snapshots");
    let mut snapshots = Vec::new();
    if dir.is_dir() {
        let mut paths: Vec<_> = fs::read_dir(&dir)
            .map_err(|e| format!("cannot read {}: {e}", dir.display()))?
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|x| x == "json"))
            .collect();
        paths.sort();
        for path in paths {
            let text = fs::read_to_string(&path)
                .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
            let value: serde_json::Value = serde_json::from_str(&text)
                .map_err(|e| format!("cannot parse {}: {e}", path.display()))?;
            let get_str = |key: &str| value.get(key).and_then(|v| v.as_str()).map(str::to_string);
            snapshots.push(SnapshotEntry {
                id: get_str("id").unwrap_or_default(),
                contract_version: value
                    .get("contract_version")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32,
                created_at: get_str("created_at").unwrap_or_default(),
                method_version: get_str("method_version").unwrap_or_default(),
                game_version: get_str("game_version").unwrap_or_default(),
            });
        }
    }
    snapshots.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(History { snapshots })
}

pub fn build_meta(ds: &Dataset) -> Meta {
    Meta {
        contract_version: 1,
        commit: commit_sha(),
        generated_at: now_iso_utc(),
        method_version: ds.method.version.clone(),
        snapshot_id: snapshot_id(ds),
        sources_checked_at: ds.checks.as_ref().map(|c| c.sources_checked_at.clone()),
        codes_checked_at: ds.checks.as_ref().map(|c| c.codes_checked_at.clone()),
    }
}

fn commit_sha() -> String {
    if let Ok(sha) = std::env::var("SDM_COMMIT") {
        if !sha.is_empty() {
            return sha;
        }
    }
    if let Ok(sha) = std::env::var("GITHUB_SHA") {
        if !sha.is_empty() {
            return sha;
        }
    }
    std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".to_string())
}

/// Current UTC time as `YYYY-MM-DDTHH:MM:SSZ` (no external crates).
pub fn now_iso_utc() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    iso_utc(secs)
}

fn iso_utc(secs: u64) -> String {
    let days = (secs / 86_400) as i64;
    let rem = secs % 86_400;
    let (year, month, day) = civil_from_days(days);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        rem / 3600,
        (rem % 3600) / 60,
        rem % 60
    )
}

/// Howard Hinnant's civil-from-days algorithm.
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

// ---------------------------------------------------------------------------
// The emit run itself
// ---------------------------------------------------------------------------

fn write_json<T: serde::Serialize>(path: &Path, value: &T) -> Result<(), String> {
    let text = serde_json::to_string_pretty(value)
        .map_err(|e| format!("cannot serialize {}: {e}", path.display()))?;
    fs::write(path, format!("{text}\n"))
        .map_err(|e| format!("cannot write {}: {e}", path.display()))
}

pub fn run(data_dir: &Path, out_dir: &Path) -> Result<(), String> {
    let ds = mongil_core::load_dataset(data_dir).map_err(|e| e.to_string())?;

    let tierlist = build_tierlist(&ds);
    let codes = build_codes(&ds)?;
    let history = build_history(data_dir)?;
    let meta = build_meta(&ds);

    fs::create_dir_all(out_dir).map_err(|e| format!("cannot create {}: {e}", out_dir.display()))?;
    write_json(&out_dir.join("tierlist.json"), &tierlist)?;
    write_json(&out_dir.join("codes.json"), &codes)?;
    write_json(&out_dir.join("history.json"), &history)?;
    write_json(&out_dir.join("meta.json"), &meta)?;
    println!(
        "emit: wrote tierlist.json ({} characters), codes.json, history.json, meta.json to {}",
        tierlist.characters.len(),
        out_dir.display()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mongil_core::model::CodeObservation;

    #[test]
    fn iso_utc_formats_known_instants() {
        assert_eq!(iso_utc(0), "1970-01-01T00:00:00Z");
        assert_eq!(iso_utc(1_761_825_600), "2025-10-30T12:00:00Z");
        assert_eq!(iso_utc(951_782_400), "2000-02-29T00:00:00Z");
    }

    // -- T2.7 codes engine (§B.3 status rules) --------------------------

    /// "Now" for the engine tests: 2026-10-30T01:00:00Z.
    fn now() -> i64 {
        crate::validate::parse_rfc3339("2026-10-30T01:00:00Z").unwrap()
    }

    fn groups() -> std::collections::BTreeMap<String, String> {
        [
            ("pg", "pg-syndicated"),
            ("ldshop", "pg-syndicated"),
            ("dexerto", "dexerto"),
            ("cosm", "cosm-game"),
            ("kaiden", "kaiden"),
        ]
        .iter()
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect()
    }

    fn obs(source: &str, status: CodeObservationStatus, seen_at: &str) -> CodeObservation {
        CodeObservation {
            source: source.to_string(),
            status,
            seen_at: seen_at.to_string(),
            url: "https://example.com/codes".to_string(),
        }
    }

    fn code(observations: Vec<CodeObservation>) -> Code {
        Code {
            code: "TESTCODE".to_string(),
            reward: "Stuff".to_string(),
            reward_source: "pg".to_string(),
            expires_at: None,
            expires_source: None,
            observations,
        }
    }

    const FRESH: &str = "2026-10-29T21:15:00-04:00"; // ~4 h before `now`

    #[test]
    fn code_status_two_independent_groups_active() {
        let c = code(vec![
            obs("pg", CodeObservationStatus::Active, FRESH),
            obs("cosm", CodeObservationStatus::Active, FRESH),
        ]);
        assert_eq!(
            compute_code_status(&c, &groups(), now()),
            CodeStatus::Active
        );
    }

    #[test]
    fn code_status_same_group_twice_is_one_witness() {
        // PG and LDShop share the pg-syndicated group (LDShop copies PG).
        let c = code(vec![
            obs("pg", CodeObservationStatus::Active, FRESH),
            obs("ldshop", CodeObservationStatus::Active, FRESH),
        ]);
        assert_eq!(
            compute_code_status(&c, &groups(), now()),
            CodeStatus::Unconfirmed
        );
    }

    #[test]
    fn code_status_one_expired_group_blocks_active() {
        let c = code(vec![
            obs("pg", CodeObservationStatus::Active, FRESH),
            obs("cosm", CodeObservationStatus::Active, FRESH),
            obs("kaiden", CodeObservationStatus::Expired, FRESH),
        ]);
        assert_eq!(
            compute_code_status(&c, &groups(), now()),
            CodeStatus::Unconfirmed
        );
    }

    #[test]
    fn code_status_two_groups_expired_wins_over_actives() {
        let c = code(vec![
            obs("pg", CodeObservationStatus::Active, FRESH),
            obs("cosm", CodeObservationStatus::Active, FRESH),
            obs("kaiden", CodeObservationStatus::Expired, FRESH),
            obs("dexerto", CodeObservationStatus::Expired, FRESH),
        ]);
        assert_eq!(
            compute_code_status(&c, &groups(), now()),
            CodeStatus::Expired
        );
    }

    #[test]
    fn code_status_past_stated_expiry_is_expired() {
        let mut c = code(vec![
            obs("pg", CodeObservationStatus::Active, FRESH),
            obs("cosm", CodeObservationStatus::Active, FRESH),
        ]);
        c.expires_at = Some("2026-10-01T00:00:00Z".to_string());
        assert_eq!(
            compute_code_status(&c, &groups(), now()),
            CodeStatus::Expired
        );
    }

    #[test]
    fn code_status_stale_actives_decay_to_unconfirmed() {
        // Both sightings are 9+ days old at build time: the 7-day
        // freshness rule (§B.3 rule 2) stops them counting.
        let stale = "2026-10-20T21:15:00-04:00";
        let c = code(vec![
            obs("pg", CodeObservationStatus::Active, stale),
            obs("cosm", CodeObservationStatus::Active, stale),
        ]);
        assert_eq!(
            compute_code_status(&c, &groups(), now()),
            CodeStatus::Unconfirmed
        );
    }

    #[test]
    fn code_status_single_expired_witness_is_unconfirmed() {
        let c = code(vec![obs("kaiden", CodeObservationStatus::Expired, FRESH)]);
        assert_eq!(
            compute_code_status(&c, &groups(), now()),
            CodeStatus::Unconfirmed
        );
    }

    #[test]
    fn code_status_group_say_is_its_freshest_observation() {
        // The pg-syndicated group said expired in September but active
        // in the fresh sweep: the fresh say is the one that counts.
        let c = code(vec![
            obs("pg", CodeObservationStatus::Expired, "2026-09-01T00:00:00Z"),
            obs("ldshop", CodeObservationStatus::Active, FRESH),
            obs("cosm", CodeObservationStatus::Active, FRESH),
        ]);
        assert_eq!(
            compute_code_status(&c, &groups(), now()),
            CodeStatus::Active
        );
    }

    #[test]
    fn code_status_no_observations_is_unconfirmed() {
        let c = code(vec![]);
        assert_eq!(
            compute_code_status(&c, &groups(), now()),
            CodeStatus::Unconfirmed
        );
    }
}
