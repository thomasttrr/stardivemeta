//! Derived lists (plan T3.7, computed here so the Beginner page is pure
//! rendering). Everything is derived from `method::compute` output — no
//! second source of truth.

use crate::load::Dataset;
use crate::method::{compute, CharResult, Label, LevelResult, TierBand};
use crate::model::{Level, MethodConfig};
use serde::Serialize;

/// One "rises with investment" entry.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RiseEntry {
    pub id: String,
    pub low: TierBand,
    pub max: TierBand,
    /// Max score − Low score.
    pub delta: f64,
}

/// The Beginner page's three lists (plus the reroll list once pool data
/// exists for the whole roster).
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BeginnerLists {
    /// Low tier S or A **and** Solid, in Low board order (ids).
    pub agreed_low: Vec<String>,
    /// Max tier strictly higher than Low tier, in Max board order (ids).
    pub rise: Vec<RiseEntry>,
    /// The Contested characters at Low, in Low board order (ids).
    pub contested_low: Vec<String>,
    /// `Some` only when every character has two-source `pool` data:
    /// permanent-pool characters at Low tier S or A. Otherwise `None`
    /// and the section does not render (plan D-3).
    pub reroll: Option<Vec<String>>,
}

fn board_order(result: &LevelResult) -> Vec<String> {
    let mut ids = Vec::new();
    for band in [
        TierBand::S,
        TierBand::A,
        TierBand::B,
        TierBand::C,
        TierBand::D,
        TierBand::ModeSplit,
        TierBand::Unranked,
    ] {
        if let Some(members) = result.boards.get(&band) {
            ids.extend(members.iter().cloned());
        }
    }
    ids
}

fn tier_rank(tier: TierBand) -> Option<u8> {
    match tier {
        TierBand::S => Some(5),
        TierBand::A => Some(4),
        TierBand::B => Some(3),
        TierBand::C => Some(2),
        TierBand::D => Some(1),
        TierBand::ModeSplit | TierBand::Unranked => None,
    }
}

fn result_of<'a>(result: &'a LevelResult, id: &str) -> &'a CharResult {
    result
        .characters
        .iter()
        .find(|c| c.id == id)
        .expect("result exists for every roster character")
}

pub fn beginner(ds: &Dataset, method: &MethodConfig) -> BeginnerLists {
    let low = compute(ds, method, Level::Low);
    let max = compute(ds, method, Level::Max);

    let low_order = board_order(&low);
    let max_order = board_order(&max);

    let agreed_low: Vec<String> = low_order
        .iter()
        .filter(|id| {
            let r = result_of(&low, id);
            matches!(r.tier, TierBand::S | TierBand::A) && r.label == Label::Solid
        })
        .cloned()
        .collect();

    let contested_low: Vec<String> = low_order
        .iter()
        .filter(|id| result_of(&low, id).label == Label::Contested)
        .cloned()
        .collect();

    let rise: Vec<RiseEntry> = max_order
        .iter()
        .filter_map(|id| {
            let low_r = result_of(&low, id);
            let max_r = result_of(&max, id);
            match (tier_rank(low_r.tier), tier_rank(max_r.tier)) {
                (Some(l), Some(m)) if m > l => Some(RiseEntry {
                    id: id.clone(),
                    low: low_r.tier,
                    max: max_r.tier,
                    delta: max_r.score.unwrap_or(0.0) - low_r.score.unwrap_or(0.0),
                }),
                _ => None,
            }
        })
        .collect();

    let reroll = if ds
        .roster
        .characters
        .iter()
        .all(|c| c.pool.is_some() && !c.pool_sources.is_empty())
    {
        Some(
            low_order
                .iter()
                .filter(|id| {
                    let character = ds
                        .roster
                        .characters
                        .iter()
                        .find(|c| &c.id == *id)
                        .expect("roster character");
                    character.pool.as_deref() == Some("permanent")
                        && matches!(result_of(&low, id).tier, TierBand::S | TierBand::A)
                })
                .cloned()
                .collect(),
        )
    } else {
        None
    };

    BeginnerLists {
        agreed_low,
        rise,
        contested_low,
        reroll,
    }
}
