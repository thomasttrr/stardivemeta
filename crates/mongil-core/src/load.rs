//! Dataset loading (plan T1.2 step 4). Bad data fails loudly: every error
//! names the file, and parse errors carry the line/field from the parser.

use crate::model::*;
use serde::de::DeserializeOwned;
use std::collections::{BTreeMap, HashSet};
use std::fmt;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

/// A fully parsed dataset: everything the method engine and the emit
/// pipeline read. Optional files are `None` when absent.
#[derive(Clone, Debug, PartialEq)]
pub struct Dataset {
    pub roster: Roster,
    pub sources: SourceRegistry,
    pub method: MethodConfig,
    pub pg: PgPlacements,
    pub kaiden: KaidenPlacements,
    pub jp: JpPlacements,
    /// Check-only placements by source id. Never used by the method engine.
    pub check_only: BTreeMap<String, CheckOnlyPlacements>,
    pub game: Option<GameInfo>,
    pub codes: Option<Codes>,
    pub code_sources: Option<CodeSources>,
    pub banners: Option<Banners>,
    pub errata: Option<Errata>,
    pub checks: Option<Checks>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LoadError {
    Missing {
        path: PathBuf,
    },
    Io {
        path: PathBuf,
        message: String,
    },
    /// Parse failure; `message` carries the parser's line/column and the
    /// offending field path where the format provides one.
    Parse {
        path: PathBuf,
        message: String,
    },
    DuplicateCharacter {
        id: String,
    },
    DuplicateSource {
        id: String,
    },
    DuplicatePlacement {
        file: String,
        character: String,
    },
    UnknownCharacterRef {
        file: String,
        character: String,
    },
    SourceMismatch {
        file: String,
        declared: String,
    },
    /// A method conversion value the engine cannot represent exactly
    /// (conversion tables hold whole or half points only).
    InvalidConversion {
        path: PathBuf,
        table: String,
        tier: String,
        value: f64,
    },
    /// A voting source's placement file has no entry in sources.toml,
    /// so whether it may vote is unknown.
    MissingVotingSource {
        id: String,
    },
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::Missing { path } => write!(f, "missing data file: {}", path.display()),
            LoadError::Io { path, message } => {
                write!(f, "cannot read {}: {message}", path.display())
            }
            LoadError::Parse { path, message } => {
                write!(f, "cannot parse {}: {message}", path.display())
            }
            LoadError::DuplicateCharacter { id } => {
                write!(f, "roster.toml: duplicate character id \"{id}\"")
            }
            LoadError::DuplicateSource { id } => {
                write!(f, "sources.toml: duplicate source id \"{id}\"")
            }
            LoadError::DuplicatePlacement { file, character } => {
                write!(
                    f,
                    "{file}: duplicate placement for character \"{character}\""
                )
            }
            LoadError::UnknownCharacterRef { file, character } => {
                write!(
                    f,
                    "{file}: placement references unknown character \"{character}\""
                )
            }
            LoadError::SourceMismatch { file, declared } => {
                write!(
                    f,
                    "{file}: [meta] source is \"{declared}\" but the file is for a different source"
                )
            }
            LoadError::InvalidConversion {
                path,
                table,
                tier,
                value,
            } => write!(
                f,
                "{}: conversion.{table}.{tier} = {value} is not a whole or half point from 0 to 4",
                path.display()
            ),
            LoadError::MissingVotingSource { id } => write!(
                f,
                "sources.toml: voting source \"{id}\" has placements but no registry entry"
            ),
        }
    }
}

impl std::error::Error for LoadError {}

fn read_file(path: &Path) -> Result<String, LoadError> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(text),
        Err(e) if e.kind() == ErrorKind::NotFound => Err(LoadError::Missing {
            path: path.to_path_buf(),
        }),
        Err(e) => Err(LoadError::Io {
            path: path.to_path_buf(),
            message: e.to_string(),
        }),
    }
}

fn parse_toml<T: DeserializeOwned>(path: &Path) -> Result<T, LoadError> {
    let text = read_file(path)?;
    toml::from_str(&text).map_err(|e| LoadError::Parse {
        path: path.to_path_buf(),
        message: e.to_string(),
    })
}

fn parse_toml_opt<T: DeserializeOwned>(path: &Path) -> Result<Option<T>, LoadError> {
    if path.exists() {
        parse_toml(path).map(Some)
    } else {
        Ok(None)
    }
}

/// The method file lives at `method.toml` in a self-contained dataset dir
/// (the test fixture) or at `method/v<version>.toml` in the live `data/`
/// tree (plan §B.1). With several versioned files, the highest version
/// wins, compared numerically (v1.10 is newer than v1.9).
fn method_path(dir: &Path) -> PathBuf {
    let flat = dir.join("method.toml");
    if flat.exists() {
        return flat;
    }
    let sub = dir.join("method");
    let mut versions: Vec<PathBuf> = Vec::new();
    if let Ok(entries) = fs::read_dir(&sub) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "toml") {
                versions.push(path);
            }
        }
    }
    versions.sort_by_key(|path| method_version_key(path));
    versions.pop().unwrap_or(flat)
}

/// Sort key for `v<major>.<minor>[.<patch>…].toml`: the numeric parts,
/// then the file name. Names that do not parse sort first.
fn method_version_key(path: &Path) -> (Option<Vec<u64>>, String) {
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default()
        .to_string();
    let numbers = path
        .file_stem()
        .and_then(|s| s.to_str())
        .and_then(|s| s.strip_prefix('v'))
        .and_then(|v| {
            v.split('.')
                .map(|part| part.parse::<u64>().ok())
                .collect::<Option<Vec<u64>>>()
        });
    (numbers, name)
}

/// Every conversion value must be a whole or half point in 0..=4: the
/// engine works in integer half-points, so anything else would be
/// silently rounded.
fn check_conversions(path: &Path, method: &MethodConfig) -> Result<(), LoadError> {
    let tables = [
        ("pg", &method.conversion.pg),
        ("gamewith-jp", &method.conversion.gamewith_jp),
        ("kaiden", &method.conversion.kaiden),
    ];
    for (table, values) in tables {
        for (tier, &value) in values {
            let doubled = value * 2.0;
            let exact = value.is_finite() && (doubled - doubled.round()).abs() < 1e-9;
            if !exact || !(0.0..=4.0).contains(&value) {
                return Err(LoadError::InvalidConversion {
                    path: path.to_path_buf(),
                    table: table.to_string(),
                    tier: tier.clone(),
                    value,
                });
            }
        }
    }
    Ok(())
}

fn check_meta_source(
    file: &Path,
    stem: &str,
    meta: Option<&PlacementMeta>,
) -> Result<(), LoadError> {
    if let Some(meta) = meta {
        if meta.source != stem {
            return Err(LoadError::SourceMismatch {
                file: file.display().to_string(),
                declared: meta.source.clone(),
            });
        }
    }
    Ok(())
}

fn check_unique<'a>(
    file: &Path,
    chars: impl Iterator<Item = &'a str>,
    known: &HashSet<&str>,
) -> Result<(), LoadError> {
    let mut seen: HashSet<&str> = HashSet::new();
    for character in chars {
        if !seen.insert(character) {
            return Err(LoadError::DuplicatePlacement {
                file: file.display().to_string(),
                character: character.to_string(),
            });
        }
        if !known.contains(character) {
            return Err(LoadError::UnknownCharacterRef {
                file: file.display().to_string(),
                character: character.to_string(),
            });
        }
    }
    Ok(())
}

/// Load a complete dataset from a directory with the layout:
///
/// ```text
/// <dir>/roster.toml  <dir>/sources.toml  <dir>/method.toml (or method/v*.toml)
/// <dir>/placements/{pg,kaiden,gamewith-jp,<check-only sources>}.toml
/// <dir>/game.toml  <dir>/codes.toml  <dir>/code-sources.toml   (optional)
/// <dir>/banners.toml  <dir>/errata.toml                        (optional)
/// <dir>/state/checks.json                                      (optional)
/// ```
pub fn load_dataset(dir: &Path) -> Result<Dataset, LoadError> {
    let roster: Roster = parse_toml(&dir.join("roster.toml"))?;
    let sources: SourceRegistry = parse_toml(&dir.join("sources.toml"))?;
    let method_file = method_path(dir);
    let method: MethodConfig = parse_toml(&method_file)?;
    check_conversions(&method_file, &method)?;

    // Roster / registry integrity.
    let mut roster_ids: HashSet<&str> = HashSet::new();
    for character in &roster.characters {
        if !roster_ids.insert(character.id.as_str()) {
            return Err(LoadError::DuplicateCharacter {
                id: character.id.clone(),
            });
        }
    }
    let mut source_ids: HashSet<&str> = HashSet::new();
    for source in &sources.sources {
        if !source_ids.insert(source.id.as_str()) {
            return Err(LoadError::DuplicateSource {
                id: source.id.clone(),
            });
        }
    }
    for character in &roster.characters {
        if let Some(base) = &character.base_of {
            if !roster_ids.contains(base.as_str()) {
                return Err(LoadError::UnknownCharacterRef {
                    file: dir.join("roster.toml").display().to_string(),
                    character: base.clone(),
                });
            }
        }
    }

    // Placements, dispatched by file stem.
    let pdir = dir.join("placements");
    let mut pg: Option<PgPlacements> = None;
    let mut kaiden: Option<KaidenPlacements> = None;
    let mut jp: Option<JpPlacements> = None;
    let mut check_only: BTreeMap<String, CheckOnlyPlacements> = BTreeMap::new();
    let mut stems: Vec<(String, PathBuf)> = Vec::new();
    if pdir.is_dir() {
        if let Ok(entries) = fs::read_dir(&pdir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|ext| ext == "toml") {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        stems.push((stem.to_string(), path));
                    }
                }
            }
        }
    }
    stems.sort();
    for (stem, path) in &stems {
        match stem.as_str() {
            "pg" => {
                let parsed: PgPlacements = parse_toml(path)?;
                check_meta_source(path, stem, parsed.meta.as_ref())?;
                check_unique(
                    path,
                    parsed.placements.iter().map(|p| p.character.as_str()),
                    &roster_ids,
                )?;
                pg = Some(parsed);
            }
            "kaiden" => {
                let parsed: KaidenPlacements = parse_toml(path)?;
                check_meta_source(path, stem, parsed.meta.as_ref())?;
                check_unique(
                    path,
                    parsed.placements.iter().map(|p| p.character.as_str()),
                    &roster_ids,
                )?;
                kaiden = Some(parsed);
            }
            "gamewith-jp" => {
                let parsed: JpPlacements = parse_toml(path)?;
                check_meta_source(path, stem, parsed.meta.as_ref())?;
                check_unique(
                    path,
                    parsed.placements.iter().map(|p| p.character.as_str()),
                    &roster_ids,
                )?;
                jp = Some(parsed);
            }
            _ => {
                let parsed: CheckOnlyPlacements = parse_toml(path)?;
                check_meta_source(path, stem, Some(&parsed.meta))?;
                check_unique(
                    path,
                    parsed.placements.iter().map(|p| p.character.as_str()),
                    &roster_ids,
                )?;
                check_only.insert(stem.clone(), parsed);
            }
        }
    }
    let pg = pg.ok_or_else(|| LoadError::Missing {
        path: pdir.join("pg.toml"),
    })?;
    let kaiden = kaiden.ok_or_else(|| LoadError::Missing {
        path: pdir.join("kaiden.toml"),
    })?;
    let jp = jp.ok_or_else(|| LoadError::Missing {
        path: pdir.join("gamewith-jp.toml"),
    })?;
    // The voting placement files above always load, so each voting
    // source needs a registry entry: its role and status decide whether
    // it casts votes (method.rs `source_counts`).
    for id in ["pg", "kaiden", "gamewith-jp"] {
        if !source_ids.contains(id) {
            return Err(LoadError::MissingVotingSource { id: id.to_string() });
        }
    }

    let game: Option<GameInfo> = parse_toml_opt(&dir.join("game.toml"))?;
    let codes: Option<Codes> = parse_toml_opt(&dir.join("codes.toml"))?;
    let code_sources: Option<CodeSources> = parse_toml_opt(&dir.join("code-sources.toml"))?;
    let banners: Option<Banners> = parse_toml_opt(&dir.join("banners.toml"))?;
    let errata: Option<Errata> = parse_toml_opt(&dir.join("errata.toml"))?;

    if let Some(banners) = &banners {
        check_unique(
            &dir.join("banners.toml"),
            banners.banners.iter().map(|b| b.character.as_str()),
            &roster_ids,
        )?;
    }

    let checks_path = dir.join("state").join("checks.json");
    let checks: Option<Checks> = if checks_path.exists() {
        let text = read_file(&checks_path)?;
        Some(serde_json::from_str(&text).map_err(|e| LoadError::Parse {
            path: checks_path.clone(),
            message: e.to_string(),
        })?)
    } else {
        None
    };

    Ok(Dataset {
        roster,
        sources,
        method,
        pg,
        kaiden,
        jp,
        check_only,
        game,
        codes,
        code_sources,
        banners,
        errata,
        checks,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    const METHOD: &str = r#"
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

    const ROSTER: &str = r#"
[[character]]
id = "mina"
name = "Mina"
[[character]]
id = "narae"
name = "Narae"
"#;

    const SOURCES: &str = r#"
[[source]]
id = "pg"
name = "Pocket Gamer"
role = "voting"
kind = "per_mode"
url = "https://www.pocketgamer.com/mongil-star-dive/tier-list/"
[[source]]
id = "kaiden"
name = "Kaiden"
role = "voting"
kind = "overall_by_investment"
url = "https://www.kaiden.gg/mongil/characters/"
[[source]]
id = "gamewith-jp"
name = "GameWith JP"
role = "voting"
kind = "overall"
url = "https://gamewith.jp/stardive/552850"
"#;

    const PG: &str = r#"
[[placement]]
character = "mina"
conquest = "S"
rift = "S"
"#;

    const KAIDEN: &str = r#"
[[placement]]
character = "mina"
low = { tier = "T2", raw = "T2" }
high = { tier = "T0.5", raw = "T0.5" }
max = { tier = "T0.5", raw = "T0.5" }
"#;

    const JP: &str = r#"
[[placement]]
character = "mina"
tier = "SS"
raw = "SS"
"#;

    fn base_files() -> Vec<(String, String)> {
        vec![
            ("roster.toml".into(), ROSTER.into()),
            ("sources.toml".into(), SOURCES.into()),
            ("method.toml".into(), METHOD.into()),
            ("placements/pg.toml".into(), PG.into()),
            ("placements/kaiden.toml".into(), KAIDEN.into()),
            ("placements/gamewith-jp.toml".into(), JP.into()),
        ]
    }

    fn write_dataset(name: &str, files: &[(String, String)]) -> PathBuf {
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!(
            "mongil-core-load-test-{}-{n}-{name}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        for (rel, content) in files {
            let path = dir.join(rel);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(&path, content).unwrap();
        }
        dir
    }

    fn with_file(name: &str, rel: &str, content: &str) -> PathBuf {
        let mut files = base_files();
        for entry in files.iter_mut() {
            if entry.0 == rel {
                entry.1 = content.to_string();
            }
        }
        write_dataset(name, &files)
    }

    #[test]
    fn load_minimal_dataset_ok() {
        let dir = write_dataset("ok", &base_files());
        let ds = load_dataset(&dir).expect("minimal dataset loads");
        assert_eq!(ds.roster.characters.len(), 2);
        assert_eq!(ds.pg.placements.len(), 1);
        assert_eq!(ds.method.s_min, 3.5);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn load_rejects_unknown_field() {
        let roster = format!("{ROSTER}\nbogus = true\n");
        // The unknown key must sit inside a [[character]] table to be
        // rejected by deny_unknown_fields on Character.
        let roster = roster.replace("name = \"Mina\"", "name = \"Mina\"\nbogus_field = 1");
        let dir = with_file("unknown-field", "roster.toml", &roster);
        let err = load_dataset(&dir).expect_err("unknown field must fail");
        assert!(
            matches!(err, LoadError::Parse { .. }),
            "expected Parse error, got {err:?}"
        );
        assert!(
            err.to_string().contains("roster.toml"),
            "error must name the file: {err}"
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn load_rejects_bad_tier() {
        let kaiden = KAIDEN.replace("\"T2\"", "\"T5\"");
        let dir = with_file("bad-tier", "placements/kaiden.toml", &kaiden);
        let err = load_dataset(&dir).expect_err("tier T5 must fail");
        assert!(
            matches!(err, LoadError::Parse { .. }),
            "expected Parse error, got {err:?}"
        );
        assert!(
            err.to_string().contains("kaiden.toml"),
            "error must name the file: {err}"
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn load_rejects_duplicate_character() {
        let roster = format!("{ROSTER}\n[[character]]\nid = \"mina\"\nname = \"Mina Again\"\n");
        let dir = with_file("dup-char", "roster.toml", &roster);
        let err = load_dataset(&dir).expect_err("duplicate id must fail");
        assert_eq!(
            err,
            LoadError::DuplicateCharacter {
                id: "mina".to_string()
            }
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn load_rejects_unknown_character_ref() {
        let pg = PG.replace("\"mina\"", "\"ghost\"");
        let dir = with_file("unknown-ref", "placements/pg.toml", &pg);
        let err = load_dataset(&dir).expect_err("unknown character must fail");
        assert_eq!(
            err,
            LoadError::UnknownCharacterRef {
                // Joined per component so the separator matches the
                // loader's on every platform.
                file: dir.join("placements").join("pg.toml").display().to_string(),
                character: "ghost".to_string(),
            }
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn load_rejects_off_half_point_conversion() {
        for bad in ["3.7", "3.25", "4.5", "-1.0"] {
            let method = METHOD.replace(
                "S = 4.0
A = 3.0",
                &format!(
                    "S = {bad}
A = 3.0"
                ),
            );
            assert_ne!(method, METHOD, "replacement must apply");
            let dir = with_file("bad-conversion", "method.toml", &method);
            let err = load_dataset(&dir).expect_err("off-half-point conversion must fail");
            assert!(
                matches!(&err, LoadError::InvalidConversion { table, tier, .. } if table == "pg" && tier == "S"),
                "expected InvalidConversion for pg S = {bad}, got {err:?}"
            );
            let _ = fs::remove_dir_all(&dir);
        }
    }

    #[test]
    fn load_rejects_voting_source_missing_from_registry() {
        let sources = SOURCES.replace("id = \"pg\"", "id = \"pg-typo\"");
        let dir = with_file("missing-voting", "sources.toml", &sources);
        let err = load_dataset(&dir).expect_err("unregistered voting source must fail");
        assert_eq!(
            err,
            LoadError::MissingVotingSource {
                id: "pg".to_string()
            }
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn method_versions_compare_numerically() {
        let mut files: Vec<(String, String)> = base_files()
            .into_iter()
            .filter(|(rel, _)| rel != "method.toml")
            .collect();
        for v in ["1.0", "1.9", "1.10", "1.2"] {
            files.push((
                format!("method/v{v}.toml"),
                METHOD.replace("version = \"1.0\"", &format!("version = \"{v}\"")),
            ));
        }
        let dir = write_dataset("method-versions", &files);
        let ds = load_dataset(&dir).expect("dataset loads");
        assert_eq!(ds.method.version, "1.10");
        let _ = fs::remove_dir_all(&dir);
    }
}
