//! Shared helpers for the mongil-build integration tests: temporary
//! datasets, the binary runner, and small git helpers.

#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn temp_dir(name: &str) -> PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "mongil-build-test-{}-{n}-{name}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

pub fn write_files(dir: &Path, files: &[(&str, &str)]) {
    for (rel, content) in files {
        let path = dir.join(rel);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, content).unwrap();
    }
}

pub struct BinOutput {
    pub code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

impl BinOutput {
    pub fn combined(&self) -> String {
        format!("{}\n{}", self.stdout, self.stderr)
    }
}

/// Run the built `mongil-build` binary.
pub fn run_bin(args: &[&str], cwd: Option<&Path>) -> BinOutput {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_mongil-build"));
    cmd.args(args);
    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }
    let out = cmd.output().unwrap();
    BinOutput {
        code: out.status.code(),
        stdout: String::from_utf8_lossy(&out.stdout).to_string(),
        stderr: String::from_utf8_lossy(&out.stderr).to_string(),
    }
}

pub fn git(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).to_string()
}

pub fn git_init_repo(dir: &Path) {
    git(dir, &["init", "-q"]);
    git(dir, &["config", "user.email", "test@example.com"]);
    git(dir, &["config", "user.name", "Test"]);
    git(dir, &["config", "commit.gpgsign", "false"]);
}

// ---------------------------------------------------------------------------
// A minimal valid dataset (mina + narae), shaped like the loader tests'.
// ---------------------------------------------------------------------------

pub const METHOD: &str = r#"
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

pub const ROSTER: &str = r#"
[[character]]
id = "mina"
name = "Mina"
[[character]]
id = "narae"
name = "Narae"
"#;

pub const SOURCES: &str = r#"
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

pub const PG: &str = r#"
[[placement]]
character = "mina"
conquest = "S"
rift = "S"
[[placement]]
character = "narae"
conquest = "S"
rift = "A"
"#;

pub const KAIDEN: &str = r#"
[[placement]]
character = "mina"
low = { tier = "T2", raw = "T2" }
high = { tier = "T0.5", raw = "T0.5" }
max = { tier = "T0.5", raw = "T0.5" }
[[placement]]
character = "narae"
low = { tier = "T0", raw = "T0" }
high = { tier = "T0", raw = "T0" }
max = { tier = "T0", raw = "T0" }
"#;

pub const JP: &str = r#"
[[placement]]
character = "mina"
tier = "SS"
raw = "SS"
[[placement]]
character = "narae"
tier = "SS"
raw = "SS"
"#;

pub fn base_files() -> Vec<(String, String)> {
    vec![
        ("roster.toml".into(), ROSTER.into()),
        ("sources.toml".into(), SOURCES.into()),
        ("method.toml".into(), METHOD.into()),
        ("placements/pg.toml".into(), PG.into()),
        ("placements/kaiden.toml".into(), KAIDEN.into()),
        ("placements/gamewith-jp.toml".into(), JP.into()),
    ]
}

/// Write a dataset from `files` (Vec form) into a fresh temp dir.
pub fn write_dataset_vec(name: &str, files: &[(String, String)]) -> PathBuf {
    let dir = temp_dir(name);
    let refs: Vec<(&str, &str)> = files
        .iter()
        .map(|(a, b)| (a.as_str(), b.as_str()))
        .collect();
    write_files(&dir, &refs);
    dir
}

/// The base dataset with one file replaced.
pub fn dataset_with(name: &str, rel: &str, content: &str) -> PathBuf {
    let mut files = base_files();
    for entry in files.iter_mut() {
        if entry.0 == rel {
            entry.1 = content.to_string();
        }
    }
    write_dataset_vec(name, &files)
}

/// The base dataset with one extra file added.
pub fn dataset_plus(name: &str, rel: &str, content: &str) -> PathBuf {
    let mut files = base_files();
    files.push((rel.to_string(), content.to_string()));
    write_dataset_vec(name, &files)
}
