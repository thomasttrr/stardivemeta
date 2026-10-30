//! Data model for StarDiveMeta (plan T1.2, formats in plan §B.3).
//!
//! Every struct denies unknown fields, so a typo or a new field in a data
//! file fails the load loudly instead of being silently ignored (gate G1).

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// ---------------------------------------------------------------------------
// Shared enums
// ---------------------------------------------------------------------------

/// Investment level (the site's toggle). Serialized lowercase, matching the
/// JSON contract in §B.2 (`"low" | "high" | "max"`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Low,
    High,
    Max,
}

impl Level {
    pub const ALL: [Level; 3] = [Level::Low, Level::High, Level::Max];

    pub fn as_str(&self) -> &'static str {
        match self {
            Level::Low => "low",
            Level::High => "high",
            Level::Max => "max",
        }
    }
}

/// A source's role. `excluded` entries are kept for the record and never
/// shown (plan §B.3 sources.toml note).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceRole {
    Voting,
    CheckOnly,
    Excluded,
}

/// How a source publishes its list.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    PerMode,
    Overall,
    OverallByInvestment,
}

/// Freshness of a source's list (contract §B.2 `SourceStatus`). Assigned by
/// the T2.3 version table in live data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceStatus {
    Current,
    PrePatchFlagged,
    StaleCheckOnly,
    Unavailable,
}

/// Why a placement entry carries no vote. Missing is never zero (plan A.0.3).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlacementStatus {
    /// PG: the character appears in no Conquest or Rift row.
    NotListed,
    /// Kaiden: the guide page exists but has no Tier Position section.
    NoPlacement,
    /// GameWith JP: absent from the SS/S/A text tiers.
    NotInTextTiers,
}

// ---------------------------------------------------------------------------
// Per-source tier enums (plan T1.2 step 2). The string forms are the keys
// into the MethodConfig conversion tables; points come from the method
// file only, never from constants in code (plan T1.4 step 9).
// ---------------------------------------------------------------------------

/// Pocket Gamer tiers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PgTier {
    S,
    A,
    B,
    C,
}

impl PgTier {
    pub fn as_str(&self) -> &'static str {
        match self {
            PgTier::S => "S",
            PgTier::A => "A",
            PgTier::B => "B",
            PgTier::C => "C",
        }
    }
}

/// GameWith JP tiers. B/C are added only if T2.5 approves them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum JpTier {
    SS,
    S,
    A,
}

impl JpTier {
    pub fn as_str(&self) -> &'static str {
        match self {
            JpTier::SS => "SS",
            JpTier::S => "S",
            JpTier::A => "A",
        }
    }
}

/// Kaiden tiers, serialized as `"T0"`, `"T0.5"`, `"T1"`, … (plan T1.2 step 2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum KaidenTier {
    T0,
    #[serde(rename = "T0.5")]
    T0_5,
    T1,
    T2,
    T3,
    T4,
}

impl KaidenTier {
    pub fn as_str(&self) -> &'static str {
        match self {
            KaidenTier::T0 => "T0",
            KaidenTier::T0_5 => "T0.5",
            KaidenTier::T1 => "T1",
            KaidenTier::T2 => "T2",
            KaidenTier::T3 => "T3",
            KaidenTier::T4 => "T4",
        }
    }
}

/// Kaiden's own exceptionality / provisional flags (plan D-G2: votes count
/// as normal; the flag is spelled out in the evidence view).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum KaidenFlag {
    P,
    E,
}

// ---------------------------------------------------------------------------
// Roster (`data/roster.toml`)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Roster {
    #[serde(rename = "character")]
    pub characters: Vec<Character>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Character {
    pub id: String,
    pub name: String,
    /// Variant link, e.g. `francis-summer-dive` has `base_of = "francis"`.
    pub base_of: Option<String>,
    /// Only set when sourced (T2.8); each fact carries its sources.
    pub released_on: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub released_on_sources: Vec<String>,
    pub rarity: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rarity_sources: Vec<String>,
    /// `"permanent"` / `"limited"` (T2.8).
    pub pool: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pool_sources: Vec<String>,
}

// ---------------------------------------------------------------------------
// Source registry (`data/sources.toml`)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceRegistry {
    #[serde(rename = "source")]
    pub sources: Vec<Source>,
}

/// Kaiden's own Low/High/Max definitions (T2.4, optional).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LevelDefinitions {
    pub low: String,
    pub high: String,
    pub max: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub id: String,
    pub name: String,
    pub role: SourceRole,
    pub kind: SourceKind,
    pub url: String,
    pub list_date: Option<String>,
    /// The game version as stated by the source; T2.3 maps it to official.
    pub game_version: Option<String>,
    pub status: Option<SourceStatus>,
    pub criteria_caveat: Option<String>,
    pub level_definitions: Option<LevelDefinitions>,
}

// ---------------------------------------------------------------------------
// Placements (`data/placements/*.toml`, harvests share the same schemas)
// ---------------------------------------------------------------------------

/// Common `[meta]` block. Harvest files add `worker`, `read_mode`, `read_at`
/// (plan §B.3).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlacementMeta {
    pub source: String,
    pub harvest: String,
    pub worker: Option<String>,
    pub read_mode: Option<String>,
    pub read_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PgPlacements {
    pub meta: Option<PlacementMeta>,
    #[serde(rename = "placement")]
    pub placements: Vec<PgPlacement>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PgPlacement {
    pub character: String,
    pub conquest: Option<PgTier>,
    pub rift: Option<PgTier>,
    pub status: Option<PlacementStatus>,
    pub note: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KaidenPlacements {
    pub meta: Option<PlacementMeta>,
    #[serde(rename = "placement")]
    pub placements: Vec<KaidenPlacement>,
}

/// One Kaiden level cell. `tier` is the first-listed tier when the level
/// text is conditional (plan D-G1); `raw` keeps the full published line.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KaidenLevelCell {
    pub tier: KaidenTier,
    pub raw: String,
    pub awakening: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub flags: Vec<KaidenFlag>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KaidenPlacement {
    pub character: String,
    /// The character's guide-page URL (recorded by the T2.2 harvest).
    pub page_url: Option<String>,
    pub page_updated: Option<String>,
    pub low: Option<KaidenLevelCell>,
    pub high: Option<KaidenLevelCell>,
    pub max: Option<KaidenLevelCell>,
    pub status: Option<PlacementStatus>,
    pub note: Option<String>,
}

impl KaidenPlacement {
    pub fn cell(&self, level: Level) -> Option<&KaidenLevelCell> {
        match level {
            Level::Low => self.low.as_ref(),
            Level::High => self.high.as_ref(),
            Level::Max => self.max.as_ref(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JpPlacements {
    pub meta: Option<PlacementMeta>,
    #[serde(rename = "placement")]
    pub placements: Vec<JpPlacement>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JpPlacement {
    pub character: String,
    pub tier: Option<JpTier>,
    pub raw: Option<String>,
    pub status: Option<PlacementStatus>,
    pub note: Option<String>,
}

/// Which shape a check-only file uses. Check-only sources never enter the
/// method math (plan T1.4 step 2); this only records what was published.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckOnlyKind {
    /// One tier per character (`tier` field).
    SingleTier,
    /// A Conquest-only list (`conquest` field), e.g. urgametips.
    ModesConquestOnly,
    /// A per-mode list (`world` / `conquest` / `rift` fields), e.g.
    /// mongil.gg (T2.6). Shown as reference rows; never counted.
    PerMode,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckOnlyPlacements {
    pub meta: PlacementMeta,
    pub kind: CheckOnlyKind,
    #[serde(rename = "placement")]
    pub placements: Vec<CheckOnlyPlacement>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckOnlyPlacement {
    pub character: String,
    /// The tier exactly as published (check-only sources have no shared
    /// scale, so this stays a string and is never converted).
    pub tier: Option<String>,
    /// Per-mode tiers (`per_mode` kind only), on the publisher's own
    /// S/A/B/C scale — recorded, never converted or counted.
    pub world: Option<PgTier>,
    pub conquest: Option<PgTier>,
    pub rift: Option<PgTier>,
    pub note: Option<String>,
}

// ---------------------------------------------------------------------------
// Game versions (`data/game.toml`, T2.3)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameInfo {
    pub current_version: String,
    #[serde(rename = "release", default, skip_serializing_if = "Vec::is_empty")]
    pub releases: Vec<GameRelease>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameRelease {
    pub version: String,
    pub date: String,
    pub balance_changes: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roster_change: Vec<String>,
    pub source: String,
}

// ---------------------------------------------------------------------------
// Codes (`data/codes.toml`, `data/code-sources.toml`)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodeSources {
    #[serde(rename = "code_source")]
    pub sources: Vec<CodeSource>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodeSource {
    pub id: String,
    pub name: String,
    /// Sites in the same independence group are not independent (plan D-G6).
    pub group: String,
    pub url: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Codes {
    #[serde(rename = "code", default, skip_serializing_if = "Vec::is_empty")]
    pub codes: Vec<Code>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Code {
    pub code: String,
    pub reward: String,
    pub reward_source: String,
    pub expires_at: Option<String>,
    pub expires_source: Option<String>,
    #[serde(rename = "observation", default, skip_serializing_if = "Vec::is_empty")]
    pub observations: Vec<CodeObservation>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodeObservationStatus {
    Active,
    Expired,
    NotListed,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodeObservation {
    pub source: String,
    pub status: CodeObservationStatus,
    pub seen_at: String,
    /// The source page's own stated "last updated" date (`YYYY-MM-DD`)
    /// as displayed when this sweep read it — the claim's date, kept
    /// separate from `seen_at` (when we read it). One date per source
    /// per sweep; absent when the page shows no date. The CP2 freshness
    /// guard (2026-10-29) compares these, never `seen_at`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_date: Option<String>,
    pub url: String,
}

// ---------------------------------------------------------------------------
// Banners (`data/banners.toml`)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Banners {
    #[serde(rename = "banner", default, skip_serializing_if = "Vec::is_empty")]
    pub banners: Vec<Banner>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Banner {
    pub character: String,
    pub start: String,
    pub end: Option<String>,
    pub source: String,
}

// ---------------------------------------------------------------------------
// Errata (`data/errata.toml`)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Errata {
    #[serde(rename = "erratum", default, skip_serializing_if = "Vec::is_empty")]
    pub errata: Vec<Erratum>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErratumSeverity {
    Minor,
    Major,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Erratum {
    pub id: String,
    pub found_at: String,
    pub fixed_at: Option<String>,
    pub severity: ErratumSeverity,
    pub what: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub characters: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub snapshots: Vec<String>,
    /// Major errata only: true after the immediate review (plan T4.5).
    #[serde(default)]
    pub review_done: bool,
}

// ---------------------------------------------------------------------------
// Method config (`data/method/v1.0.toml`; fixture copy: `method.toml`)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MethodConfig {
    pub version: String,
    pub weights: String,
    pub s_min: f64,
    pub a_min: f64,
    pub b_min: f64,
    pub c_min: f64,
    pub solid_max_spread: f64,
    pub mode_split_min: f64,
    /// `"first_listed"` in v1.0 (plan D-G1).
    pub kaiden_conditional_rule: String,
    pub conversion: ConversionTables,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConversionTables {
    pub pg: BTreeMap<String, f64>,
    #[serde(rename = "gamewith-jp")]
    pub gamewith_jp: BTreeMap<String, f64>,
    pub kaiden: BTreeMap<String, f64>,
}

// ---------------------------------------------------------------------------
// Check stamps (`data/state/checks.json`)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Checks {
    pub sources_checked_at: String,
    pub codes_checked_at: String,
    pub last_sweep: String,
}
