//! JSON contract types (plan §B.2, `contract_version: 1`).
//!
//! These are the wire types between the Rust pipeline and the Astro site.
//! They are deliberately separate from the data-file model (`model.rs`):
//! the TOML files are the input shape, these structs are the output shape.
//! `mongil-build emit` assembles them from a loaded `Dataset`; Astro only
//! renders them (plan A.0 rule 1: Astro does no math).
//!
//! Every type derives `ts_rs::TS` and is exported to
//! `site/src/types/contract/*.ts` by the export test at the bottom of this
//! file, so the TypeScript side can never drift from Rust (plan T1.6
//! acceptance b). Field names in the generated TS follow the serde
//! renames (ts-rs `serde-compat`).

use serde::Serialize;
use std::collections::BTreeMap;
use ts_rs::TS;


/// Investment level (the site's toggle).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, TS)]
#[serde(rename_all = "lowercase")]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub enum Level {
    Low,
    High,
    Max,
}

/// Overall tier band, plus the mode-split and unranked outcomes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub enum Tier {
    S,
    A,
    B,
    C,
    D,
    ModeSplit,
    Unranked,
}

/// Confidence label.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub enum Label {
    Solid,
    Contested,
    Speculation,
    ModeSplit,
    Unranked,
}

/// A tier on a per-mode board (PG publishes S–C; no D).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub enum ModeTier {
    S,
    A,
    B,
    C,
    Unranked,
}

/// A source's role in the contract. Excluded sources are never emitted.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub enum SourceRole {
    Voting,
    CheckOnly,
}

/// How a source publishes its list.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub enum SourceKind {
    PerMode,
    Overall,
    OverallByInvestment,
}

/// Freshness of a source's list (Method §5).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub enum SourceStatus {
    Current,
    PrePatchFlagged,
    StaleCheckOnly,
    Unavailable,
}

/// Kaiden's exceptionality / provisional flags (plan D-G2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub enum KaidenFlag {
    P,
    E,
}

/// The method block at the top of `tierlist.json`. Constants are the
/// values from `data/method/v1.0.toml`, never hard-coded (plan T1.4.9).
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub struct MethodInfo {
    #[ts(type = "\"1.0\"")]
    pub version: String,
    pub s_min: f64,
    pub a_min: f64,
    pub b_min: f64,
    pub c_min: f64,
    pub solid_max_spread: f64,
    pub mode_split_min: f64,
    pub conversion: BTreeMap<String, BTreeMap<String, f64>>,
}

/// Kaiden's own Low/High/Max definitions (T2.4), when published.
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub struct LevelDefinitions {
    pub low: String,
    pub high: String,
    pub max: String,
}

/// One source as the site sees it.
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub struct Source {
    pub id: String,
    pub name: String,
    pub role: SourceRole,
    pub kind: SourceKind,
    pub url: String,
    pub list_date: Option<String>,
    pub game_version: Option<String>,
    pub status: SourceStatus,
    pub criteria_caveat: Option<String>,
    pub level_definitions: Option<LevelDefinitions>,
}

/// One counted vote.
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub struct Vote {
    pub source: String,
    pub value: f64,
    pub raw: String,
}

/// The mode-split detail inside `Result.why`.
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub struct ModeSplit {
    pub conquest: f64,
    pub rift: f64,
}

/// Why a result carries its label: the highest and lowest votes, or the
/// mode split that overrode the Overall tier.
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub struct Why {
    pub high: Option<Vote>,
    pub low: Option<Vote>,
    pub mode_split: Option<ModeSplit>,
}

/// A character's computed result at one investment level.
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub struct Result {
    pub tier: Tier,
    pub label: Label,
    /// Mean of the votes, display only. `None` when unranked.
    pub score: Option<f64>,
    pub score_display: Option<String>,
    pub coverage: u32,
    /// Highest vote − lowest vote, in points. `None` when unranked.
    pub spread: Option<f64>,
    pub votes: Vec<Vote>,
    pub why: Why,
}

/// Per-level results for one character (the contract's
/// `Record<Level, Result>`, as named fields so the shape is exact).
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub struct Results {
    pub low: Result,
    pub high: Result,
    pub max: Result,
}

/// One mode placement for one character.
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub struct ModePlacement {
    pub tier: ModeTier,
    pub raw: Option<String>,
}

/// A character's two mode placements (the contract's
/// `Record<"conquest" | "rift", …>`, as named fields).
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub struct Modes {
    pub conquest: ModePlacement,
    pub rift: ModePlacement,
}

/// One Kaiden level inside an evidence row.
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub struct LevelCell {
    pub tier: String,
    pub raw: String,
    pub awakening: Option<String>,
    pub flags: Vec<KaidenFlag>,
    pub converted: f64,
}

/// Why an evidence row carries no placement.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub enum NoneReason {
    NoPlacement,
    NotInTextTiers,
    NotListed,
}

/// What a source published for a character, in the source's own shape.
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub enum EvidencePlacement {
    Single {
        raw: String,
        converted: Option<f64>,
    },
    Modes {
        conquest: Option<String>,
        rift: Option<String>,
        converted_overall: Option<f64>,
    },
    Levels {
        low: LevelCell,
        high: LevelCell,
        max: LevelCell,
    },
    None {
        reason: NoneReason,
    },
}

/// One row of a character's evidence view.
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub struct EvidenceRow {
    pub source: String,
    pub role: SourceRole,
    /// Whether this source's vote is in the math for this character.
    pub counted: bool,
    pub placement: EvidencePlacement,
    pub page_url: String,
    pub page_date: Option<String>,
    pub note: Option<String>,
}

/// One character with per-level results and full evidence.
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub struct Character {
    pub id: String,
    pub name: String,
    pub released_on: Option<String>,
    pub is_new: bool,
    pub results: Results,
    pub modes: Modes,
    pub evidence: Vec<EvidenceRow>,
    /// Ids of errata (`data/errata.toml`) that mention this character.
    pub errata: Vec<String>,
}

/// The board for one level: ordered character ids per tier.
#[derive(Clone, Debug, Default, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub struct TierBoard {
    #[serde(rename = "S")]
    pub s: Vec<String>,
    #[serde(rename = "A")]
    pub a: Vec<String>,
    #[serde(rename = "B")]
    pub b: Vec<String>,
    #[serde(rename = "C")]
    pub c: Vec<String>,
    #[serde(rename = "D")]
    pub d: Vec<String>,
    #[serde(rename = "MODE_SPLIT")]
    pub mode_split: Vec<String>,
    #[serde(rename = "UNRANKED")]
    pub unranked: Vec<String>,
}

/// The boards for all three levels (the contract's
/// `Record<Level, Record<Tier, string[]>>`, as named fields).
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub struct Boards {
    pub low: TierBoard,
    pub high: TierBoard,
    pub max: TierBoard,
}

/// One per-mode board (PG's tiers only).
#[derive(Clone, Debug, Default, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub struct ModeBoard {
    #[serde(rename = "S")]
    pub s: Vec<String>,
    #[serde(rename = "A")]
    pub a: Vec<String>,
    #[serde(rename = "B")]
    pub b: Vec<String>,
    #[serde(rename = "C")]
    pub c: Vec<String>,
    #[serde(rename = "UNRANKED")]
    pub unranked: Vec<String>,
}

/// The per-mode boards (the contract's
/// `Record<"conquest" | "rift", …>`, as named fields).
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub struct ModeBoards {
    pub conquest: ModeBoard,
    pub rift: ModeBoard,
}

/// One "rises with investment" entry (Beginner page).
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub struct Rise {
    pub id: String,
    pub low: Tier,
    pub max: Tier,
    /// Max score − Low score.
    pub delta: f64,
}

/// One reroll entry (Beginner page, once pool data exists — plan D-3).
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub struct RerollEntry {
    pub id: String,
}

/// The Beginner page's derived lists.
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub struct Beginner {
    pub agreed_low: Vec<String>,
    pub rise: Vec<Rise>,
    pub contested_low: Vec<String>,
    pub reroll: Option<Vec<RerollEntry>>,
}

/// Derived lists emitted alongside the boards.
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub struct Derived {
    pub beginner: Beginner,
}

/// `tierlist.json` — the whole computed site dataset for the tier pages.
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub struct TierList {
    #[ts(type = "1")]
    pub contract_version: u32,
    pub method: MethodInfo,
    pub game_version: String,
    pub snapshot_id: String,
    pub sources: Vec<Source>,
    pub characters: Vec<Character>,
    pub boards: Boards,
    pub mode_boards: ModeBoards,
    pub derived: Derived,
}

/// A code's computed status (rules land with the T2.7 codes engine;
/// the type is part of the contract from the start).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub enum CodeStatus {
    Active,
    Unconfirmed,
    Expired,
}

/// Where a code's reward text came from.
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub struct CodeReward {
    pub text: String,
    pub source: String,
}

/// One sighting of a code by one source.
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub struct CodeObservation {
    pub source: String,
    /// The source's independence group (plan D-G6).
    pub group: String,
    pub status: String,
    pub seen_at: String,
    pub url: String,
}

/// One code with its computed status and all observations.
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub struct CodeEntry {
    pub code: String,
    pub status: CodeStatus,
    pub reward: CodeReward,
    pub expires_at: Option<String>,
    pub first_seen: Option<String>,
    pub observations: Vec<CodeObservation>,
}

/// `codes.json`.
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub struct CodesFile {
    pub checked_at: Option<String>,
    pub codes: Vec<CodeEntry>,
}

/// One snapshot's index entry in `history.json`. The frozen results
/// themselves live in `data/snapshots/<id>.json` (plan §B.3) and are
/// rendered through the site adapter; the snapshot writer lands in T2.9.
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub struct SnapshotEntry {
    pub id: String,
    pub contract_version: u32,
    pub created_at: String,
    pub method_version: String,
    pub game_version: String,
}

/// `history.json` — the snapshot index.
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub struct History {
    pub snapshots: Vec<SnapshotEntry>,
}

/// `meta.json` — build info and check stamps. This is the only emitted
/// file allowed a build timestamp (plan T1.6: `generated_at`).
#[derive(Clone, Debug, PartialEq, Serialize, TS)]
#[ts(export, export_to = "../../../site/src/types/contract/")]
pub struct Meta {
    #[ts(type = "1")]
    pub contract_version: u32,
    pub commit: String,
    pub generated_at: String,
    pub method_version: String,
    pub snapshot_id: String,
    pub sources_checked_at: Option<String>,
    pub codes_checked_at: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Plan T1.6: `cargo test` exports the contract types to
    /// `site/src/types/contract/*.ts` (committed; CI diffs them).
    #[test]
    fn export_contract_types() {
        TierList::export_all().expect("export TierList");
        CodesFile::export_all().expect("export CodesFile");
        History::export_all().expect("export History");
        Meta::export_all().expect("export Meta");
    }
}
